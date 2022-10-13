pub mod arch;
pub mod core_modules;
pub mod driver;
pub mod system_library;
pub mod utilities {
    pub mod otp_cs {

        #[inline]
        pub fn otp_cs_get_adc_trim_val() -> u16 {
            unsafe { crate::bindings::otp_cs_get_adc_trim_val() }
        }

        #[inline]
        pub fn otp_cs_get_adc_25_cal() -> u16 {
            unsafe { crate::bindings::otp_cs_get_adc_25_cal() }
        }
    }
}
