pub mod task {
    pub use crate::bindings::{
        gapc_connection_req_ind as GapcConnectionReqInd, gapc_disconnect_ind as GapcDisconnectInd,
        gapc_msg_id as GapcMsgId, gapc_param_updated_ind as GapcParamUpdatedInd,
    };
}
