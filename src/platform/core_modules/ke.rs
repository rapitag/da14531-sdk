pub mod msg {

    pub use crate::bindings::{ke_msg_id_t as KeMsgId, ke_msg_status_tag as KeMsgStatusTag};

    use crate::bindings::{ke_msg_alloc, ke_msg_send};

    pub struct KernelMessage<const ID: u32, const SIZE: u16, T> {
        msg_ptr: *mut T,
    }

    impl<const ID: u32, const SIZE: u16, T> KernelMessage<ID, SIZE, T> {
        pub fn new(src_id: KeMsgId, dest_id: KeMsgId) -> Self {
            let msg_ptr = unsafe {
                ke_msg_alloc(
                    ID as u16,
                    dest_id,
                    src_id,
                    core::mem::size_of::<Self>() as u16 + SIZE,
                ) as *mut T
            };

            Self { msg_ptr }
        }

        pub fn fields(&mut self) -> &mut T {
            unsafe { &mut *self.msg_ptr }
        }

        pub fn send(self) {
            unsafe {
                ke_msg_send(self.msg_ptr as *const cty::c_void);
            }
        }
    }

    macro_rules! kernel_msg_type {
        ($Msg: ty, $id: ident) => {
            paste::paste! {
                #[allow(dead_code)]
                pub type [<KeMsg $Msg>] = $crate::platform::core_modules::ke::msg::KernelMessage<$id, 0, $Msg>;
                #[allow(dead_code)]
                pub type [<KeMsgDyn $Msg>]<const SIZE: u16> = $crate::platform::core_modules::ke::msg::KernelMessage<$id, SIZE, $Msg>;
            }
        };
        ($Prefix: ident, $Msg: ty, $id: ident) => {
            paste::paste! {
                #[allow(dead_code)]
                pub type [<KeMsg $Prefix $Msg>] = $crate::platform::core_modules::KernelMessage<$id, 0, $Msg>;
                #[allow(dead_code)]
                pub type [<KeMsgDyn $Prefix $Msg>]<const SIZE: u16> = $crate::platform::core_modules::ke::msg::KernelMessage<$id, SIZE, $Msg>;
            }
        };
        (private, $Msg: ty, $id: ident) => {
            paste::paste! {
                #[allow(dead_code)]
                type [<KeMsg $Msg>] = KernelMessage<$id, 0, $Msg>;
                #[allow(dead_code)]
                type [<KeMsgDyn $Msg>]<const SIZE: u16> = $crate::platform::core_modules::ke::msg::KernelMessage<$id, SIZE, $Msg>;
            }
        };
        (private, $Prefix: ident, $Msg: ty, $id: ident) => {
            paste::paste! {
                #[allow(dead_code)]
                type [<KeMsg $Prefix $Msg>] = KernelMessage<$id, 0, $Msg>;
                #[allow(dead_code)]
                type [<KeMsgDyn $Prefix $Msg>]<const SIZE: u16> = $crate::platform::core_modules::ke::msg::KernelMessage<$id, SIZE, $Msg>;
            }
        };
    }

    pub(crate) use kernel_msg_type;
}

pub mod timer {
    use super::{msg::KeMsgId, task::KeTaskId};

    #[inline]
    pub fn ke_timer_set(timer_id: KeMsgId, task: KeTaskId, delay: u32) {
        unsafe {
            crate::bindings::ke_timer_set(timer_id, task, delay);
        }
    }

    #[inline]
    pub fn ke_timer_clear(timer_id: KeMsgId, task: KeTaskId) {
        unsafe {
            crate::bindings::ke_timer_clear(timer_id, task);
        }
    }
}

pub mod task {
    pub use crate::bindings::{
        ke_state_handler as KeStateHandler, ke_state_t as KeState, ke_task_id_t as KeTaskId,
    };

    pub fn ke_state_set(task_id: KeTaskId, state_id: KeState) {
        unsafe {
            crate::bindings::ke_state_set(task_id, state_id);
        }
    }
}

pub mod mem {
    pub fn ke_malloc<T>(mem_type: u32) -> *mut T {
        let len = core::mem::size_of::<T>();
        unsafe { crate::bindings::ke_malloc(len as u32, mem_type as u8) as *mut T }
    }

    pub fn ke_free<T>(ptr: *mut T) {
        unsafe { crate::bindings::ke_free(ptr as *mut cty::c_void) }
    }
}
