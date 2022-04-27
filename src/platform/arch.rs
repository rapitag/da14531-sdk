#[inline]
pub fn arch_ble_force_wakeup() -> bool {
    unsafe { crate::bindings::arch_ble_force_wakeup() }
}
