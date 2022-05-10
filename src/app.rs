pub mod user_config {
    pub use crate::bindings::{
        connection_param_configuration as ConnectionParamConfiguration, user_central_conf,
    };
}

pub use crate::bindings::CFG_MAX_CONNECTIONS;
