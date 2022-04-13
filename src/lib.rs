#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

macro_rules! binding_mod {
    ($i:ident) => {
        mod inner {
            include!(concat!(env!("OUT_DIR"), "/", stringify!($i), ".h.rs"));
        }
    };
}

pub mod custs1 {
    pub mod task {
        binding_mod!(custs1_task);

        use crate::ke::msg::KeTaskId;

        pub use inner::{
            custs1_att_info_rsp as Custs1AttInfoRsp, custs1_att_info_req as Custs1AttInfoReq, custs1_val_write_ind as Custs1ValWriteInd,
            CUSTS1_ATT_INFO_RSP, hl_err_ATT_ERR_WRITE_NOT_PERMITTED as ATT_ERR_WRITE_NOT_PERMITTED
        };

        #[inline]
        pub fn ke_msg_alloc(id: KeTaskId, dest_id: KeTaskId, src_id: KeTaskId) -> Custs1AttInfoRsp {
            let msg_ptr = unsafe {
                inner::ke_msg_alloc(
                    id,
                    dest_id,
                    src_id,
                    core::mem::size_of::<Custs1AttInfoRsp>() as u16,
                ) as *mut Custs1AttInfoRsp
            };

            return unsafe { *msg_ptr };
        }

        #[inline]
        pub fn ke_msg_send(msg: Custs1AttInfoRsp) {
            unsafe {
                inner::ke_msg_send(&msg as *const _  as *const cty::c_void);
            }
        }
    }
}

pub mod ke {
    pub mod msg {
        binding_mod!(ke_msg);

        pub use inner::{ke_msg_id_t as KeMsgId, ke_task_id_t as KeTaskId};
    }
}

pub mod syscntl {
    binding_mod!(syscntl);

    pub use inner::{syscntl_dcdc_level_t as SyscntlDcdcLevel, app_env};

    #[inline]
    pub fn dcdc_turn_on_in_boost(dcdc_level: SyscntlDcdcLevel) {
        unsafe {
            inner::syscntl_dcdc_turn_on_in_boost(dcdc_level);
        }
    }
}

pub mod system_library {
    binding_mod!(system_library);

    #[inline]
    pub fn patch_func() {
        unsafe {
            inner::patch_func();
        }
    }
}
