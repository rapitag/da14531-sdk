pub use crate::bindings::{
    app_env, timer_callback as TimerCallback, timer_hnd as TimerHnd, EASY_TIMER_INVALID_TIMER,
    cust_prf_func_callbacks as CustPrfFuncCallbacks, app_custs1_create_db
};

use crate::{ble_stack::host::gap::{
    gapc::task::GapcConnectionReqInd, gapm::task::GapmStartAdvertiseCmd,
}, core_modules::common::{ADV_DATA_LEN, SCAN_RSP_DATA_LEN}};

pub fn app_env_get_conidx(conidx:u8) -> u8 {
    unsafe { app_env.get_unchecked(conidx as usize).conidx }
}

pub fn app_easy_gap_param_update_start(conidx: u8) {
    unsafe {
        crate::bindings::app_easy_gap_param_update_start(conidx);
    }
}

pub fn app_easy_gap_undirected_advertise_get_active() -> GapmStartAdvertiseCmd {
    unsafe { *crate::bindings::app_easy_gap_undirected_advertise_get_active() }
}

pub fn app_easy_gap_undirected_advertise_start() {
    unsafe {
        crate::bindings::app_easy_gap_undirected_advertise_start();
    }
}

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

pub fn app_easy_timer(delay: u32, callback: unsafe extern "C" fn()) -> TimerHnd {
    unsafe { crate::bindings::app_easy_timer(delay, Some(callback)) }
}

pub fn app_easy_timer_cancel(timer: TimerHnd) {
    unsafe {
        crate::bindings::app_easy_timer_cancel(timer);
    }
}

pub fn default_app_on_connection(conidx: u8, param: *const GapcConnectionReqInd) {
    unsafe {
        crate::bindings::default_app_on_connection(conidx, param);
    }
}

pub fn default_app_on_init() {
    unsafe {
        crate::bindings::default_app_on_init();
    }
}
