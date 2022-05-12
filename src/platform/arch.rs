pub use crate::bindings::{arch_main_loop_callbacks as ArchMainLoopCallbacks, arch_main_loop_callback_ret_t as ArchMainLoopCallbackRet, sleep_mode_t as SleepMode};
pub use da14531_sdk_macros::register_main_loop_callbacks;

#[inline]
pub fn arch_ble_force_wakeup() -> bool {
    unsafe { crate::bindings::arch_ble_force_wakeup() }
}
