pub use crate::bindings::{
    custs1_att_info_req as Custs1AttInfoReq, custs1_att_info_rsp as Custs1AttInfoRsp,
    custs1_val_write_ind as Custs1ValWriteInd, custs1_value_req_ind as Custs1ValueReqInd,
    custs1_value_req_rsp as Custs1ValueReqRsp, CUSTS1_ATT_INFO_REQ, CUSTS1_ATT_INFO_RSP,
    CUSTS1_VALUE_REQ_IND, CUSTS1_VALUE_REQ_RSP, CUSTS1_VAL_WRITE_IND,
};

use crate::platform::core_modules::ke::msg::kernel_msg_type;

kernel_msg_type!(Custs1AttInfoRsp, CUSTS1_ATT_INFO_RSP);
kernel_msg_type!(Custs1ValueReqRsp, CUSTS1_VALUE_REQ_RSP);
