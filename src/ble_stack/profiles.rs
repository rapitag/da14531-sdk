use crate::platform::core_modules::ke::msg::{KeMsgId, KeTaskId};

pub fn prf_get_task_from_id(msg_id: KeMsgId) -> KeTaskId {
    unsafe { crate::bindings::prf_get_task_from_id(msg_id) }
}

pub mod custom {
    pub mod custs {
        pub mod custs1_task {
            pub use crate::bindings::{
                custs1_att_info_req as Custs1AttInfoReq,
                custs1_att_info_rsp as Custs1AttInfoRsp, custs1_val_ind_cfm as Custs1ValIndCfm,
                custs1_val_ntf_cfm as Custs1ValNtfCfm,
                custs1_val_write_ind as Custs1ValWriteInd,
                custs1_value_req_ind as Custs1ValueReqInd,
                custs1_value_req_rsp as Custs1ValueReqRsp, CUSTS1_ATT_INFO_REQ,
                CUSTS1_VALUE_REQ_IND, CUSTS1_VAL_IND_CFM, CUSTS1_VAL_NTF_CFM,
                CUSTS1_VAL_WRITE_IND,
            };
            use crate::{
                bindings::{CUSTS1_ATT_INFO_RSP, CUSTS1_VALUE_REQ_RSP},
                platform::core_modules::ke::msg::{kernel_msg_type, KernelMessage},
            };

            kernel_msg_type!(Custs1ValueReqRsp, CUSTS1_VALUE_REQ_RSP);
            kernel_msg_type!(Custs1AttInfoRsp, CUSTS1_ATT_INFO_RSP);
        }
    }
}
