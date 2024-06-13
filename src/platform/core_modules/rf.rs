#[repr(u32)]
pub enum PaPwrLevel {
    /// -19.5 dBm
    RF_TX_PWR_LVL_MINUS_19d5 = 1,
    /// -13.5 dBm
    RF_TX_PWR_LVL_MINUS_13d5 = 2,
    /// -10 dBm
    RF_TX_PWR_LVL_MINUS_10d0 = 3,
    /// -7 dBm
    RF_TX_PWR_LVL_MINUS_7d0 = 4,
    /// -5 dBm
    RF_TX_PWR_LVL_MINUS_5d0 = 5,
    /// -3.5 dBm
    RF_TX_PWR_LVL_MINUS_3d5 = 6,
    /// -2 dBm
    RF_TX_PWR_LVL_MINUS_2d0 = 7,
    /// -1 dBm
    RF_TX_PWR_LVL_MINUS_1d0 = 8,
    /// 0 dBm
    RF_TX_PWR_LVL_0d0 = 9,
    /// +1 dBm
    RF_TX_PWR_LVL_PLUS_1d0 = 10,
    /// +1.5 dBm
    RF_TX_PWR_LVL_PLUS_1d5 = 11,
    /// +2.5 dBm
    RF_TX_PWR_LVL_PLUS_2d5 = 12,
}

pub fn rf_pa_pwr_set(level: PaPwrLevel) {
    unsafe {
        crate::bindings::rf_pa_pwr_set(level as u32);
    }
}
