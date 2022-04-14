pub mod rwble_hl {
    pub mod error {
        pub use crate::bindings::hl_err as HlErr;
    }
}

pub mod host {
    pub mod gap {
        pub use crate::bindings::{gap_ad_type as GapAdType, GAP_INVALID_CONIDX};
        pub mod gapc {
            pub mod task {
                pub use crate::bindings::{
                    gapc_connection_req_ind as GapcConnectionReqInd,
                    gapc_disconnect_ind as GapcDisconnectInd, gapc_msg_id as GapcMsgId,
                    gapc_param_updated_ind as GapcParamUpdatedInd,
                };
            }
        }
        pub mod gapm {
            pub mod task {
                pub use crate::bindings::{
                    gapm_start_advertise_cmd as GapmStartAdvertiseCmd
                };
            }
        }
    }

    pub mod att {
        pub mod attm {
            pub use crate::bindings::{
                attm_desc_128 as AttmDesc128
            };
        }
    }
}

pub mod profiles {
    pub mod custom {
        pub mod custs {
            pub mod custs1_task {
                use paste::paste;

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
                    core_modules::ke::api::msg::KernelMessage,
                };

                macro_rules! kernel_msg_type {
                    ($Msg: ty, $id: ident) => {
                        paste! {
                            pub type [<KeMsg $Msg>] = KernelMessage<$id, $Msg>;
                        }
                    };
                }

                kernel_msg_type!(Custs1ValueReqRsp, CUSTS1_VALUE_REQ_RSP);
                kernel_msg_type!(Custs1AttInfoRsp, CUSTS1_ATT_INFO_RSP);
            }
        }
    }
}
