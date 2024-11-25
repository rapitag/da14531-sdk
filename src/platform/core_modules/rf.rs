#[repr(u32)]
pub enum PaPwrLevel {
    /// -19.5 dBm
    RfTxPwrLvlMinus19d5 = 1,
    /// -13.5 dBm
    RfTxPwrLvlMinus13d5 = 2,
    /// -10 dBm
    RfTxPwrLvlMinus10d0 = 3,
    /// -7 dBm
    RfTxPwrLvlMinus7d0 = 4,
    /// -5 dBm
    RfTxPwrLvlMinus5d0 = 5,
    /// -3.5 dBm
    RfTxPwrLvlMinus3d5 = 6,
    /// -2 dBm
    RfTxPwrLvlMinus2d0 = 7,
    /// -1 dBm
    RfTxPwrLvlMinus1d0 = 8,
    /// 0 dBm
    RfTxPwrLvl0d0 = 9,
    /// +1 dBm
    RfTxPwrLvlPlus1d0 = 10,
    /// +1.5 dBm
    RfTxPwrLvlPlus1d5 = 11,
    /// +2.5 dBm
    RfTxPwrLvlPlus2d5 = 12,
}

pub fn rf_pa_pwr_set(level: PaPwrLevel) {
    unsafe {
        crate::bindings::rf_pa_pwr_set(level as u32);
    }
}
