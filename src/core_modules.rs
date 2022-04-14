pub mod ke {
    pub mod api {
        pub mod msg {
            pub use crate::bindings::{ke_msg_id_t as KeMsgId, ke_task_id_t as KeTaskId};

            use crate::bindings::{ke_msg_alloc, ke_msg_send};

            pub struct KernelMessage<const ID: u32, T> {
                msg_ptr: *mut T,
            }

            impl<const ID: u32, T> KernelMessage<ID, T> {
                pub fn new(src_id: KeMsgId, dest_id: KeMsgId) -> Self {
                    let msg_ptr = unsafe {
                        ke_msg_alloc(
                            ID as u16,
                            dest_id,
                            src_id,
                            core::mem::size_of::<Self>() as u16,
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
        }
    }
}

pub mod common {
    pub use crate::bindings::{ADV_DATA_LEN, SCAN_RSP_DATA_LEN};
}


pub mod rwip {
    pub use crate::bindings::KE_API_ID as KeApiId;
}