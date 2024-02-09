use crate::platform::core_modules::ke::{msg::KeMsgId, task::KeTaskId};

#[cfg(feature = "custom_rest_evt_cb")]
extern "Rust" {
    fn user_catch_rest_hndl(
        msg_id: KeMsgId,
        param: *const cty::c_void,
        dest_id: KeTaskId,
        src_id: KeTaskId,
    );
}

#[cfg(feature = "custom_rest_evt_cb")]
#[no_mangle]
pub extern "C" fn app_process_catch_rest_cb(
    msg_id: KeMsgId,
    param: *const cty::c_void,
    dest_id: KeTaskId,
    src_id: KeTaskId,
) {
    rtt_target::rprintln!("app_process_catch_rest_cb");
    unsafe { user_catch_rest_hndl(msg_id, param, dest_id, src_id) };
}

#[cfg(not(feature = "custom_rest_evt_cb"))]
#[no_mangle]
pub static app_process_catch_rest_cb: usize = 0;
