pub use crate::bindings::{ke_msg_id_t as KeMsgId, ke_msg_status_tag as KeMsgStatusTag};

use crate::bindings::{ke_msg_alloc, ke_msg_send};

pub struct KernelMessage<const ID: u32, const SIZE: u16, T>(*mut T);

impl<const ID: u32, const SIZE: u16, T> KernelMessage<ID, SIZE, T> {
    pub fn new(src_id: KeMsgId, dest_id: KeMsgId) -> Self {
        let msg_ptr = unsafe {
            ke_msg_alloc(
                ID as u16,
                dest_id,
                src_id,
                core::mem::size_of::<T>() as u16 + SIZE,
            ) as *mut T
        };

        Self(msg_ptr)
    }

    pub fn fields(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }

    pub fn send(self) {
        unsafe {
            ke_msg_send(self.0 as *const cty::c_void);
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

macro_rules! ke_build_id {
    ($type: expr, $index: expr) => {
        ((($index as u16) << 8) | ($type as u16))
    };
}

pub(crate) use ke_build_id;
