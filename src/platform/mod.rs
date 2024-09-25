#[cfg(not(feature = "no_ble"))]
pub mod arch;
#[cfg(not(feature = "no_ble"))]
pub mod core_modules;
pub mod driver;
pub mod system_library;
pub mod utilities {
    #[cfg(not(feature = "no_ble"))]
    pub mod otp_cs {

        #[inline]
        pub fn otp_cs_get_adc_trim_val() -> u16 {
            unsafe { crate::bindings::otp_cs_get_adc_trim_val() }
        }

        #[inline]
        pub fn otp_cs_get_adc_single_ge() -> i16 {
            unsafe { crate::bindings::otp_cs_get_adc_single_ge() }
        }

        #[inline]
        pub fn otp_cs_get_adc_single_offset() -> i16 {
            unsafe { crate::bindings::otp_cs_get_adc_single_offset() }
        }

        #[inline]
        pub fn otp_cs_get_adc_offsh_ge() -> i16 {
            unsafe { crate::bindings::otp_cs_get_adc_offsh_ge() }
        }

        #[inline]
        pub fn otp_cs_get_adc_offsh_offset() -> i16 {
            unsafe { crate::bindings::otp_cs_get_adc_offsh_offset() }
        }
    }
}
