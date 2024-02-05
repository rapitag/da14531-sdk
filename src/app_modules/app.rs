pub use crate::bindings::{app_env_tag as AppEnvTag, APP_EASY_MAX_ACTIVE_CONNECTION};

use crate::platform::core_modules::common::BDAddr;

impl Default for AppEnvTag {
    fn default() -> Self {
        zero_app_env_tag()
    }
}

pub const fn zero_app_env_tag() -> AppEnvTag {
    AppEnvTag {
        conhdl: 0,
        conidx: 0,
        connection_active: false,
        peer_addr_type: 0,
        peer_addr: BDAddr { addr: [0; 6] },
        pairing_in_progress: false,
    }
}
