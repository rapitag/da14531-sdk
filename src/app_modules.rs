pub use crate::bindings::{
    app_custs1_create_db, app_env, cust_prf_func_callbacks as CustPrfFuncCallbacks,
    process_event_response as ProcessEventResponse, timer_hnd as TimerHandle, APP_MSG as AppMsg,
    EASY_TIMER_INVALID_TIMER,
};

use crate::{
    ble_stack::host::gap::{gapc::task::GapcConnectionReqInd, gapm::task::GapmStartAdvertiseCmd},
    platform::core_modules::common::{ADV_DATA_LEN, SCAN_RSP_DATA_LEN},
};

pub mod timer;

pub type TimerCallback = unsafe extern "C" fn();

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

pub mod msg_utils {
    #[inline]
    pub fn app_check_ble_active() -> bool {
        unsafe { crate::bindings::app_check_BLE_active() }
    }
}
