pub mod msg {

    pub use crate::bindings::{
        ke_msg_id_t as KeMsgId, ke_msg_status_tag as KeMsgStatusTag,
        ke_task_id_t as KeTaskId,
    };

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
                pub type [<KeMsg $Msg>] = KernelMessage<$id, 0, $Msg>;
                pub type [<KeMsgDyn $Msg>]<const SIZE: u16> = KernelMessage<$id, SIZE, $Msg>;
            }
        };
        ($Prefix: ident, $Msg: ty, $id: ident) => {
            paste::paste! {
                pub type [<KeMsg $Prefix $Msg>] = KernelMessage<$id, 0, $Msg>;
                pub type [<KeMsgDyn $Prefix $Msg>]<const SIZE: u16> = KernelMessage<$id, SIZE, $Msg>;
            }
        };
        (private, $Msg: ty, $id: ident) => {
            paste::paste! {
                type [<KeMsg $Msg>] = KernelMessage<$id, 0, $Msg>;
                type [<KeMsgDyn $Msg>]<const SIZE: u16> = KernelMessage<$id, SIZE, $Msg>;
            }
        };
        (private, $Prefix: ident, $Msg: ty, $id: ident) => {
            paste::paste! {
                type [<KeMsg $Prefix $Msg>] = KernelMessage<$id, 0, $Msg>;
                type [<KeMsgDyn $Prefix $Msg>]<const SIZE: u16> = KernelMessage<$id, SIZE, $Msg>;
            }
        };
    }

    pub(crate) use kernel_msg_type;
}

pub mod timer {
    use super::msg::{KeMsgId, KeTaskId};

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
