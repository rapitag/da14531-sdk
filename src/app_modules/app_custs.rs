pub use crate::bindings::{
    cust_prf_func_callbacks as CustPrfFuncCallbacks, prf_func_callbacks as PrfFuncCallbacks,
};

unsafe impl Sync for CustPrfFuncCallbacks {}

#[cfg(feature = "profile_custom_server1")]
pub mod custs1;

#[cfg(feature = "profile_custom_server2")]
pub mod custs2;
