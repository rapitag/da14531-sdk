use crate::{
    bindings::{
        APP_MODULES_TIMER_API_LAST_MES, APP_MODULES_TIMER_API_MES0, APP_MODULES_TIMER_MAX_NUM,
        KE_TIMER_DELAY_MAX, MSG_APP_CANCEL_TIMER, MSG_APP_CREATE_TIMER, MSG_APP_MODIFY_TIMER,
    },
    platform::{
        arch::arch_ble_force_wakeup,
        core_modules::{
            ke::{
                msg::{kernel_msg_type, KeMsgId, KeMsgStatusTag, KeTaskId, KernelMessage},
                timer::{ke_timer_clear, ke_timer_set},
            },
            rwip::TASK_APP,
        },
    },
};

use super::{msg_utils::app_check_ble_active, AppMsg, ProcessEventResponse, TimerHandle};

#[repr(C)]
struct AppTimerParams {
    handle: TimerHandle,
    delay: u32,
}

kernel_msg_type!(private, Create, AppTimerParams, MSG_APP_CREATE_TIMER);
kernel_msg_type!(private, Modify, AppTimerParams, MSG_APP_MODIFY_TIMER);
kernel_msg_type!(private, Cancel, AppTimerParams, MSG_APP_CANCEL_TIMER);

pub type TimerCallback = fn();

#[derive(PartialEq, Clone, Copy)]
enum TimerState {
    None,
    Modified,
    Canceled,
    Active(TimerCallback),
}

impl TimerState {
    pub fn take_callback(&mut self) -> Option<TimerCallback> {
        if let TimerState::Active(callback) = *self {
            *self = TimerState::None;
            Some(callback)
        } else {
            None
        }
    }
}

#[link_section = "retention_mem_area0"]
static mut TIMER_CALLBACKS: [TimerState; APP_MODULES_TIMER_MAX_NUM as usize] =
    [TimerState::None; APP_MODULES_TIMER_MAX_NUM as usize];

// #[link_section = "retention_mem_area0"]
// static mut MODIFIED_TIMER_CALLBACKS: [TimerState; APP_MODULES_TIMER_MAX_NUM as usize] =
//     [TimerState::None; APP_MODULES_TIMER_MAX_NUM as usize];

#[inline]
fn timer_handle_to_msg_id(handle: TimerHandle) -> KeMsgId {
    ((handle as i32) - 1 + (APP_MODULES_TIMER_API_MES0 as i32)) as u16
}
#[inline]
fn timer_msg_id_to_handle(msg_id: KeMsgId) -> TimerHandle {
    ((msg_id as i32) - (APP_MODULES_TIMER_API_MES0 as i32) + 1) as u8
}

#[inline]
fn timer_handle_to_index(handle: TimerHandle) -> usize {
    (handle - 1) as usize
}

#[inline]
fn timer_index_to_handle(index: usize) -> TimerHandle {
    (index + 1) as TimerHandle
}

#[inline]
fn is_timer_handle_valid(handle: TimerHandle) -> bool {
    (handle > 0) && (handle <= APP_MODULES_TIMER_MAX_NUM)
}

pub struct AppTimer(TimerHandle);

impl AppTimer {
    pub fn new(delay: u32, callback: TimerCallback) -> Option<Self> {
        assert!(delay <= KE_TIMER_DELAY_MAX);

        if let Some(handle) = register_callback(callback) {
            create_timer(delay, handle);

            Some(Self(handle))
        } else {
            None
        }
    }

    // pub fn modify(&mut self, delay: u32) {
    //     todo!();
    //     //     if APP_EASY_TIMER_HND_IS_VALID(handle)
    //     //     {
    //     //         if ((timer_callbacks[APP_EASY_TIMER_HND_TO_IDX(handle)] != NULL) &&
    //     //             (timer_callbacks[APP_EASY_TIMER_HND_TO_IDX(handle)] != timer_canceled_handler))
    //     //         {
    //     //             // Remove the timer from the timer queue
    //     //             ke_timer_clear(APP_EASY_TIMER_HND_TO_MSG_ID(handle), TASK_APP);
    //     //             timer_callbacks[APP_EASY_TIMER_HND_TO_IDX(handle)] = timer_canceled_handler;
    //     //             /*
    //     //                 Send a message to the kernel in order to clear the timer callback function and
    //     //                 free the respective position in the timers callback array.
    //     //                 The app_easy_timer_cancel() function cannot guarantee if a timer has entered
    //     //                 the message queue or not. Therefore a message must be sent to the kernel and
    //     //                 inform it about the requested cancel operation.
    //     //             */
    //     //             struct cancel_timer_struct *req = KE_MSG_ALLOC(APP_CANCEL_TIMER, TASK_APP, TASK_APP,
    //     //                                                            cancel_timer_struct);
    //     //             req->handle = handle;
    //     //             ke_msg_send(req);
    //     //         }
    //     //         else
    //     //         {
    //     //             ASSERT_WARNING(0);
    //     //         }
    //     //    }
    //     //    else
    //     //    {
    //     //        ASSERT_WARNING(0);
    //     //    }
    // }

    pub fn cancel(self) {
        // assert!(is_timer_handle_valid(self.0));

        let timer_idx = timer_handle_to_index(self.0);

        let callback = unsafe { &TIMER_CALLBACKS[timer_idx] };

        if *callback != TimerState::None && *callback != TimerState::Modified {
            // Remove the timer from the timer queue
            ke_timer_clear(timer_handle_to_msg_id(self.0), TASK_APP);

            unsafe {
                TIMER_CALLBACKS[timer_idx] = TimerState::Canceled;
            }

            /*
                Send a message to the kernel in order to clear the timer callback function and
                free the respective position in the timers callback array.
                The AppTimer::cancel() function cannot guarantee if a timer has entered
                the message queue or not. Therefore a message must be sent to the kernel and
                inform it about the requested cancel operation.
            */
            let mut msg = KeMsgCancelAppTimerParams::new(TASK_APP, TASK_APP);

            msg.fields().handle = self.0;

            msg.send();
        }
    }

    pub fn cancel_all() {
        for (timer_idx, callback) in unsafe { TIMER_CALLBACKS.iter().enumerate() } {
            if *callback != TimerState::None && *callback != TimerState::Canceled {
                let handle = timer_index_to_handle(timer_idx);
                let timer = AppTimer(handle);
                timer.cancel();
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn app_timer_api_process_handler(
    msg_id: KeMsgId,
    param: *const cty::c_void,
    _dest_id: KeTaskId,
    _src_id: KeTaskId,
    msg_ret: *mut KeMsgStatusTag,
) -> ProcessEventResponse {
    let app_msg: AppMsg = core::mem::transmute(msg_id as u32);

    let timer_params = &*(param as *const AppTimerParams);

    match app_msg {
        AppMsg::APP_CREATE_TIMER => {
            *msg_ret = create_timer_handler(timer_params.handle, timer_params.delay);
            ProcessEventResponse::PR_EVENT_HANDLED
        }
        AppMsg::APP_CANCEL_TIMER => {
            *msg_ret = cancel_timer_handler(timer_params.handle);
            ProcessEventResponse::PR_EVENT_HANDLED
        }
        AppMsg::APP_MODIFY_TIMER => {
            // *msg_ret = modify_timer_handler(timer_params.handle, timer_params.delay);
            // ProcessEventResponse::PR_EVENT_HANDLED
            ProcessEventResponse::PR_EVENT_UNHANDLED
        }
        _ => {
            let msg_id = msg_id as KeMsgId;
            if msg_id < APP_MODULES_TIMER_API_MES0 || msg_id > APP_MODULES_TIMER_API_LAST_MES {
                ProcessEventResponse::PR_EVENT_UNHANDLED
            } else {
                let handle = timer_msg_id_to_handle(msg_id);
                *msg_ret = call_timer_callback_handler(handle);
                ProcessEventResponse::PR_EVENT_HANDLED
            }
        }
    }
}

#[inline]
fn create_timer_handler(handle: TimerHandle, delay: u32) -> KeMsgStatusTag {
    ke_timer_set(timer_handle_to_msg_id(handle), TASK_APP, delay);
    KeMsgStatusTag::KE_MSG_CONSUMED
}

fn cancel_timer_handler(handle: TimerHandle) -> KeMsgStatusTag {
    assert!(is_timer_handle_valid(handle));
    let timer_idx = timer_handle_to_index(handle);
    let callback = unsafe { &mut TIMER_CALLBACKS[timer_idx] };
    if *callback == TimerState::Canceled {
        *callback = TimerState::None;
        //         modified_timer_callbacks[i] = NULL;
    } else if *callback == TimerState::Modified {
        let mut msg = KeMsgModifyAppTimerParams::new(TASK_APP, TASK_APP);
        msg.fields().handle = handle;
        //         req->delay = param->delay;
        msg.send();
    }
    KeMsgStatusTag::KE_MSG_CONSUMED
}

// fn modify_timer_handler(handle: TimerHandle, delay: u32) -> KeMsgStatusTag {
//     // if APP_EASY_TIMER_HND_IS_VALID(param->handle)
//     // {
//     //     int i = APP_EASY_TIMER_HND_TO_IDX(param->handle);
//     //     if (timer_callbacks[i] == timer_modified_handler)
//     //     {
//     //         // Restore timer callback function
//     //         timer_callbacks[i] = modified_timer_callbacks[i];
//     //         // Re-create the timer with new delay
//     //         create_timer(param->handle, param->delay);
//     //     }
//     // }
//     // else
//     // {
//     //     ASSERT_ERROR(0);
//     // }
//     KeMsgStatusTag::KE_MSG_CONSUMED
// }

fn call_timer_callback_handler(handle: TimerHandle) -> KeMsgStatusTag {
    rtt_target::rprintln!("call_timer_callback_handler(0x{:02x})", handle);
    let callback = unsafe { &mut TIMER_CALLBACKS[timer_handle_to_index(handle)] };

    if *callback != TimerState::Canceled && *callback != TimerState::Modified {
        if let Some(callback) = callback.take_callback() {
            // //                 modified_timer_callbacks[APP_EASY_TIMER_HND_TO_IDX(handle)] = NULL;
            callback();
        }
    }

    KeMsgStatusTag::KE_MSG_CONSUMED
}

fn create_timer(delay: u32, handle: TimerHandle) {
    if app_check_ble_active() {
        let msg_id = timer_handle_to_msg_id(handle);
        ke_timer_set(msg_id, TASK_APP, delay);
    } else {
        arch_ble_force_wakeup();

        let mut msg = KeMsgCreateAppTimerParams::new(TASK_APP, TASK_APP);

        msg.fields().delay = delay;
        msg.fields().handle = handle;

        msg.send();
    }
}

#[inline]
fn register_callback(callback: TimerCallback) -> Option<TimerHandle> {
    for (idx, callback_entry) in unsafe { TIMER_CALLBACKS.iter_mut().enumerate() } {
        if *callback_entry == TimerState::None {
            *callback_entry = TimerState::Active(callback);
            return Some(timer_index_to_handle(idx));
        }
    }
    None
}

// fn unregister_callback(handle: TimerHandle) {
//     let callback_entry = unsafe { TIMER_CALLBACKS.get_mut(handle as usize) };
// }
