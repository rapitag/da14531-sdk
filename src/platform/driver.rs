pub mod syscntl {
    pub use crate::bindings::{
        syscntl_dcdc_level_t as SyscntlDcdcLevel, syscntl_dcdc_turn_on_in_boost,
    };

    // ToDo: Port to HAL crate
    #[inline]
    pub fn dcdc_turn_on_in_boost(dcdc_level: SyscntlDcdcLevel) {
        unsafe {
            syscntl_dcdc_turn_on_in_boost(dcdc_level);
        }
    }
}

// ToDo: Port to HAL crate
pub mod gpio {
    #[inline]
    pub fn GPIO_EnableIRQ(pin: u8, irq: u8, low_input: bool, release_wait: bool, debounce: u8) {
        unsafe {
            crate::bindings::GPIO_EnableIRQ(
                0,
                pin as u32,
                irq as i32,
                low_input,
                release_wait,
                debounce,
            );
        }
    }

    #[inline]
    pub fn GPIO_RegisterCallback(irq: u8, callback: unsafe extern "C" fn()) {
        unsafe { crate::bindings::GPIO_RegisterCallback(irq as i32, Some(callback)) }
    }
}
