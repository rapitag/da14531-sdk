pub use crate::bindings::{
    prf_func_uint8_t as PrfFuncUint8, prf_func_validate_t as PrfFuncValidate,
    prf_func_void_t as PrfFuncVoid, rom_cust_prf_cfg_t as RomCustPrfCfg,
};

use crate::platform::core_modules::rwip::KeApiId;

#[cfg(feature = "profile_custom_server1")]
pub mod custs1;

unsafe impl Sync for RomCustPrfCfg {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CustPrfFuncCallbacks {
    #[doc = " Profile Task ID."]
    pub task_id: KeApiId,
    #[doc = " pointer to the custom database table defined by user"]
    pub att_db: usize,
    #[doc = " max number of attributes in custom database"]
    pub max_nb_att: u8,
    #[doc = " Pointer to the custom database create function defined by user"]
    pub db_create_func: PrfFuncVoid,
    #[doc = " Pointer to the custom profile enable function defined by user"]
    pub enable_func: PrfFuncUint8,
    #[doc = " Pointer to the custom profile initialization function"]
    pub init_func: PrfFuncVoid,
    #[doc = " Pointer to the validation function defined by user"]
    pub value_wr_validation_func: PrfFuncValidate,
}
