pub use crate::bindings::{
    advertise_configuration as AdvertiseConfiguration, app_env,
    app_prf_srv_perm_t as AppPrfSrvPerm, app_prf_srv_sec_t as AppPrfSrvSec, prf_func_uint8_t,
    prf_func_validate_t, prf_func_void_t, process_event_response as ProcessEventResponse,
    timer_hnd as TimerHandle, APP_MSG as AppMsg, EASY_TIMER_INVALID_TIMER, PRFS_TASK_ID_MAX,
};

use crate::{
    ble_stack::host::gap::{gapc::task::GapcConnectionReqInd, gapm::task::GapmStartAdvertiseCmd},
    platform::core_modules::{
        common::{ADV_DATA_LEN, SCAN_RSP_DATA_LEN},
        rwip::KeApiId,
    },
};

pub mod app;
pub mod app_common;
pub mod app_custs;
pub mod app_task;
pub mod msg_utils;
pub mod timer;

pub type TimerCallback = unsafe extern "C" fn();

pub const fn zero_app_prf_srv_sec() -> AppPrfSrvSec {
    AppPrfSrvSec { task_id: 0, perm: 0 }
}

#[inline]
pub fn append_device_name(
    scan_data_len: &mut u8,
    name_len: usize,
    scan_data: &mut [u8],
    name: &[u8],
) {
    unsafe {
        crate::bindings::append_device_name(
            scan_data_len,
            name_len as u8,
            scan_data.as_mut_ptr(),
            name.as_ptr() as *const cty::c_void,
        );
    }
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
