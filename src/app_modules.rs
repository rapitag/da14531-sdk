pub use crate::bindings::{
    advertise_configuration as AdvertiseConfiguration, app_env,
    app_prf_srv_perm_t as AppPrfSrvPerm, app_prf_srv_sec_t as AppPrfSrvSec,
    catch_rest_event_func_t as CatchRestEventFunc, default_app_operations as DefaultAppOperations,
    gapm_configuration as GapmConfiguration, prf_func_uint8_t, prf_func_validate_t,
    prf_func_void_t, process_event_response as ProcessEventResponse, timer_hnd as TimerHandle,
    APP_CFG_ADDR_PUB, APP_CFG_ADDR_STATIC, APP_MSG as AppMsg, EASY_TIMER_INVALID_TIMER,
    PRFS_TASK_ID_MAX, app_callbacks as AppCallbacks
};

pub use da14531_sdk_macros::register_app_callbacks;

use crate::{
    ble_stack::host::gap::{gapc::task::GapcConnectionReqInd, gapm::task::GapmStartAdvertiseCmd},
    platform::core_modules::{
        common::{ADV_DATA_LEN, SCAN_RSP_DATA_LEN},
        rwip::KeApiId,
    },
};

pub mod app;
pub mod app_common;
pub mod app_task;
pub mod msg_utils;
pub mod timer;

#[cfg(feature = "custom_rest_evt_cb")]
mod custom_rest_evt_cb;

#[cfg(feature = "profile_custom_server1")]
pub mod app_custs;

pub type TimerCallback = unsafe extern "C" fn();

pub const fn zero_app_prf_srv_sec() -> AppPrfSrvSec {
    AppPrfSrvSec {
        task_id: 0,
        perm: 0,
    }
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PeerAddress {
    pub peer_addr: [u8; 6usize],
    pub peer_addr_type: u8,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CentralConfiguration {
    pub code: u8,
    pub addr_src: u8,
    pub scan_interval: u16,
    pub scan_window: u16,
    pub con_intv_min: u16,
    pub con_intv_max: u16,
    pub con_latency: u16,
    pub superv_to: u16,
    pub ce_len_min: u16,
    pub ce_len_max: u16,
    pub peer_addresses: [PeerAddress; 8],
}

pub const fn app_cfg_addr_type(val: u32) -> u8 {
    (val as u8 & 0x70) >> 4
}

pub const fn app_cfg_addr_src(val: u32) -> u8 {
    (val as u8 & 0x03) >> 0
}

pub const fn ms_to_ble_slots(val: u32) -> u16 {
    ((val * 1000) / 625) as u16
}

#[macro_export]
macro_rules! register_user_operation_adv {
    ($fn: ident) => {
        #[no_mangle]
        pub extern "C" fn __app_adv_start() {
            $fn();
        }

        #[export_name = "user_default_app_operations"]
        static USER_DEFAULT_APP_OPERATIONS: $crate::app_modules::DefaultAppOperations =
            $crate::app_modules::DefaultAppOperations {
                default_operation_adv: Some(__app_adv_start),
            };
    };
}
