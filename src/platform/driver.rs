pub mod syscntl {
    pub use crate::bindings::{
        syscntl_dcdc_level_t as SyscntlDcdcLevel, syscntl_dcdc_turn_on_in_boost,
    };

    #[inline]
    pub fn dcdc_turn_on_in_boost(dcdc_level: SyscntlDcdcLevel) {
        unsafe {
            syscntl_dcdc_turn_on_in_boost(dcdc_level);
        }
    }
}
