pub mod custs {
    pub use crate::bindings::{prf_func_uint8_t, prf_func_validate_t, prf_func_void_t};

    use crate::platform::core_modules::rwip::KeApiId;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CustPrfFuncCallbacks {
        #[doc = " Profile Task ID."]
        pub task_id: KeApiId,
        #[doc = " pointer to the custom database table defined by user"]
        pub att_db: usize,
        #[doc = " max number of attributes in custom database"]
        pub max_nb_att: u8,
        #[doc = " Pointer to the custom database create function defined by user"]
        pub db_create_func: prf_func_void_t,
        #[doc = " Pointer to the custom profile enable function defined by user"]
        pub enable_func: prf_func_uint8_t,
        #[doc = " Pointer to the custom profile initialization function"]
        pub init_func: prf_func_void_t,
        #[doc = " Pointer to the validation function defined by user"]
        pub value_wr_validation_func: prf_func_validate_t,
    }

    pub mod custs1_task {
        pub use crate::bindings::{
            custs1_att_info_req as Custs1AttInfoReq, custs1_att_info_rsp as Custs1AttInfoRsp,
            custs1_val_ind_cfm as Custs1ValIndCfm, custs1_val_ntf_cfm as Custs1ValNtfCfm,
            custs1_val_write_ind as Custs1ValWriteInd, custs1_value_req_ind as Custs1ValueReqInd,
            custs1_value_req_rsp as Custs1ValueReqRsp, CUSTS1_ATT_INFO_REQ, CUSTS1_VALUE_REQ_IND,
            CUSTS1_VAL_IND_CFM, CUSTS1_VAL_NTF_CFM, CUSTS1_VAL_WRITE_IND,
        };
        use crate::{
            bindings::{CUSTS1_ATT_INFO_RSP, CUSTS1_VALUE_REQ_RSP},
            platform::core_modules::ke::msg::kernel_msg_type,
        };

        kernel_msg_type!(Custs1ValueReqRsp, CUSTS1_VALUE_REQ_RSP);
        kernel_msg_type!(Custs1AttInfoRsp, CUSTS1_ATT_INFO_RSP);
    }
}
