use crate::platform::core_modules::common::{HciLeTxTestCmd, HciLeRxTestCmd};

pub use crate::bindings::{llm_le_env, LLM_ADV_INTERVAL_MAX, LLM_ADV_INTERVAL_MIN};

#[inline]
pub fn llm_test_mode_start_tx(cmd: &HciLeTxTestCmd) -> u8 {
    unsafe { crate::bindings::llm_test_mode_start_tx(cmd) }
}
#[inline]
pub fn llm_test_mode_start_rx(cmd: &HciLeRxTestCmd) -> u8 {
    unsafe { crate::bindings::llm_test_mode_start_rx(cmd) }
}
