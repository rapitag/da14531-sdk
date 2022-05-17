pub use crate::bindings::{diss_value_req_ind as DissValueReqInd, DISS_VALUE_REQ_IND, diss_value_cfm as DissValueCfm, DISS_VALUE_CFM};

use crate::platform::core_modules::ke::msg::kernel_msg_type;

kernel_msg_type!(DissValueCfm, DISS_VALUE_CFM);