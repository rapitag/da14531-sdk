pub mod user_config {
    pub use crate::bindings::{
        connection_param_configuration as ConnectionParamConfiguration, user_adv_conf,
        user_central_conf, user_connection_param_conf, user_gapm_conf, USER_ADVERTISE_DATA,
        USER_ADVERTISE_SCAN_RESPONSE_DATA, USER_DEVICE_NAME,
    };
}

pub use crate::bindings::CFG_MAX_CONNECTIONS;
