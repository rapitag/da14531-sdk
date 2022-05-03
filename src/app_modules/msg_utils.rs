#[inline]
pub fn app_check_ble_active() -> bool {
    unsafe { crate::bindings::app_check_BLE_active() }
}
