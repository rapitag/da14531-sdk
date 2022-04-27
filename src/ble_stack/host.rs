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
            pub use crate::bindings::gapm_start_advertise_cmd as GapmStartAdvertiseCmd;
        }
    }
}

pub mod att {
    pub mod attm {
        pub use crate::bindings::attm_desc_128 as AttmDesc128;
    }
}
