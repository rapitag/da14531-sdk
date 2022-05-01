pub use crate::bindings::{
    app_env, prf_func_uint8_t, prf_func_validate_t, prf_func_void_t,
    process_event_response as ProcessEventResponse, timer_hnd as TimerHandle, APP_MSG as AppMsg,
    EASY_TIMER_INVALID_TIMER, app_prf_srv_perm_t as AppPrfSrvPerm
};

use crate::{
    ble_stack::host::gap::{gapc::task::GapcConnectionReqInd, gapm::task::GapmStartAdvertiseCmd},
    platform::core_modules::{
        common::{ADV_DATA_LEN, SCAN_RSP_DATA_LEN},
        rwip::KeApiId,
    },
};

pub mod app_custs;
pub mod timer;

pub type TimerCallback = unsafe extern "C" fn();

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
    pub db_create_func: prf_func_void_t,
    #[doc = " Pointer to the custom profile enable function defined by user"]
    pub enable_func: prf_func_uint8_t,
    #[doc = " Pointer to the custom profile initialization function"]
    pub init_func: prf_func_void_t,
    #[doc = " Pointer to the validation function defined by user"]
    pub value_wr_validation_func: prf_func_validate_t,
}

#[inline]
pub fn app_env_get_conidx(conidx: u8) -> u8 {
    unsafe { app_env.get_unchecked(conidx as usize).conidx }
}

#[inline]
pub fn app_easy_gap_param_update_start(conidx: u8) {
    unsafe {
        crate::bindings::app_easy_gap_param_update_start(conidx);
    }
}

#[inline]
pub fn app_easy_gap_disconnect(conidx: u8) {
    unsafe {
        crate::bindings::app_easy_gap_disconnect(conidx);
    }
}

#[inline]
pub fn app_easy_gap_undirected_advertise_get_active() -> &'static mut GapmStartAdvertiseCmd {
    unsafe { &mut *crate::bindings::app_easy_gap_undirected_advertise_get_active() }
}

#[inline]
pub fn app_easy_gap_undirected_advertise_start() {
    unsafe {
        crate::bindings::app_easy_gap_undirected_advertise_start();
    }
}

#[inline]
pub fn app_easy_gap_update_adv_data(
    update_adv_data: &[u8; ADV_DATA_LEN as usize],
    update_adv_data_len: u8,
    update_scan_rsp_data: &[u8; SCAN_RSP_DATA_LEN as usize],
    update_scan_rsp_data_len: u8,
) {
    unsafe {
        let update_adv_data_ptr = update_adv_data as *const u8;
        let update_scan_rsp_data_ptr = update_scan_rsp_data as *const u8;
        crate::bindings::app_easy_gap_update_adv_data(
            update_adv_data_ptr,
            update_adv_data_len,
            update_scan_rsp_data_ptr,
            update_scan_rsp_data_len,
        );
    }
}

#[inline]
pub fn default_app_on_connection(conidx: u8, param: *const GapcConnectionReqInd) {
    unsafe {
        crate::bindings::default_app_on_connection(conidx, param);
    }
}

#[inline]
pub fn default_app_on_init() {
    unsafe {
        crate::bindings::default_app_on_init();
    }
}

#[inline]
pub fn get_user_prf_srv_perm(task_id: KeApiId) -> AppPrfSrvPerm {
    unsafe { crate::bindings::get_user_prf_srv_perm(task_id) }
}

pub mod msg_utils {
    #[inline]
    pub fn app_check_ble_active() -> bool {
        unsafe { crate::bindings::app_check_BLE_active() }
    }
}
