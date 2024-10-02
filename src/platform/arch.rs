pub use crate::bindings::{
    arch_main_loop_callback_ret_t as ArchMainLoopCallbackRet,
    arch_main_loop_callbacks as ArchMainLoopCallbacks, sleep_mode_t as SleepMode,
};
pub use da14531_sdk_macros::register_main_loop_callbacks;

// ToDo: Port everything to HAL crate

#[inline]
pub fn arch_ble_force_wakeup() -> bool {
    unsafe { crate::bindings::arch_ble_force_wakeup() }
}

#[inline]
pub fn arch_ble_ext_wakeup_on() {
    unsafe { crate::bindings::arch_ble_ext_wakeup_on() }
}

#[inline]
pub fn arch_ble_ext_wakeup_off() {
    unsafe { crate::bindings::arch_ble_ext_wakeup_off() }
}

#[inline]
pub fn arch_ble_ext_wakeup_get() -> bool {
    unsafe { crate::bindings::arch_ble_ext_wakeup_get() }
}

#[inline]
pub fn arch_set_sleep_mode(sleep_state: u32) {
    unsafe { crate::bindings::arch_set_sleep_mode(sleep_state) }
}

#[inline]
pub fn arch_set_extended_sleep(otp_copy: bool) {
    unsafe { crate::bindings::arch_set_extended_sleep(otp_copy) }
}

#[inline]
pub fn system_init() {
    unsafe { crate::bindings::system_init() }
}
