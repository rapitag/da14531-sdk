pub mod task {
    pub use crate::bindings::{
        gapc_connection_req_ind as GapcConnectionReqInd, gapc_disconnect_ind as GapcDisconnectInd,
        gapc_get_info_cmd as GapcGetInfoCmd, gapc_msg_id as GapcMsgId,
        gapc_msg_id_GAPC_GET_INFO_CMD as GAPC_GET_INFO_CMD,
        gapc_msg_id_GAPC_PARAM_UPDATED_IND as GAPC_PARAM_UPDATED_IND,
        gapc_msg_id_GAPC_PARAM_UPDATE_CMD as GAPC_PARAM_UPDATE_CMD,
        gapc_operation_GAPC_GET_PEER_FEATURES as GAPC_GET_PEER_FEATURES,
        gapc_param_update_cmd as GapcParamUpdateCmd, gapc_param_updated_ind as GapcParamUpdatedInd,
    };
    use crate::platform::core_modules::ke::msg::kernel_msg_type;

    kernel_msg_type!(GapcParamUpdateCmd, GAPC_PARAM_UPDATE_CMD);
    kernel_msg_type!(GapcGetInfoCmd, GAPC_GET_INFO_CMD);
}
