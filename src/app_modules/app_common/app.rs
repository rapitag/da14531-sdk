use rtt_target::rprintln;

use crate::{
    app_modules::{
        app::{zero_app_env_tag, AppEnvTag, APP_EASY_MAX_ACTIVE_CONNECTION},
        app_cfg_addr_src, app_cfg_addr_type,
        app_common::{
            app_default_handler, app_state, AppDeviceInfo, AppDeviceName, APP_CONNECTABLE,
            APP_DISABLED, APP_IDX_MAX, APP_STATE_MAX,
        },
        app_custs::{CustPrfFuncCallbacks, PrfFuncCallbacks},
        ms_to_ble_slots, zero_app_prf_srv_sec, AdvertiseConfiguration, AppPrfSrvSec,
        GapmConfiguration, PRFS_TASK_ID_MAX,
    },
    ble_stack::{
        controller::llm::llm_le_env,
        host::gap::{
            gapm::task::{
                KeMsgGapmSetDevConfigCmd, KeMsgGapmStartAdvertiseCmd, GAPM_ADV_UNDIRECT,
                GAPM_MASK_ATT_SVC_CHG_EN, GAPM_SET_DEV_CONFIG,
            },
            GAP_AD_TYPE_COMPLETE_NAME, GAP_GEN_DISCOVERABLE, GAP_MAX_NAME_SIZE,
            GAP_NON_DISCOVERABLE, GAP_ROLE_PERIPHERAL,
        },
        // profiles::custom::custs::custs1::{CUSTS1_ATT_DB, CUSTS1_ATT_DB_LEN},
    },
    platform::core_modules::{
        common::{
            co_min, ADV_ALLOW_SCAN_ANY_CON_ANY, ADV_ALLOW_SCAN_ANY_CON_WLST, ADV_ALL_CHNLS_EN,
            ADV_DATA_LEN, KEY_LEN, SCAN_RSP_DATA_LEN,
        },
        ke::task::{ke_state_set, ke_task_create, KeTaskDesc},
        rwip::{
            KeApiId, TASK_APP, TASK_GAPM, TASK_ID_ANCC, TASK_ID_BASS, TASK_ID_BCSS, TASK_ID_BMSS,
            TASK_ID_CTSC, TASK_ID_CTSS, TASK_ID_CUSTS1, TASK_ID_DISS, TASK_ID_FINDL, TASK_ID_FINDT,
            TASK_ID_GATT_CLIENT, TASK_ID_INVALID, TASK_ID_PROXR, TASK_ID_SUOTAR, TASK_ID_UDSS,
            TASK_ID_WSSS,
        },
    },
};

#[cfg(feature = "profile_custom_server1")]
use crate::app_modules::app_custs::custs1::{app_custs1_create_db, app_custs1_init};

#[cfg(feature = "profile_custom_server2")]
use crate::app_modules::app_custs::custs2::{app_custs2_create_db, app_custs2_init};

#[cfg(feature = "address_mode_public")]
use crate::app_modules::APP_CFG_ADDR_PUB;

#[cfg(feature = "address_mode_static")]
use crate::{app_modules::APP_CFG_ADDR_STATIC, platform::core_modules::common::BDAddr};

mod advertise;

use advertise::*;

// type TimeoutCallback = fn();

// type KeMsgGapmStartConnectionCmd = KeMsgDynGapmStartConnectionCmd<
//     { core::mem::size_of::<GapBDAddr>() as u16 * CFG_MAX_CONNECTIONS as u16 },
// >;

#[link_section = "retention_mem_area0"]
#[no_mangle]
static mut app_env: [AppEnvTag; APP_EASY_MAX_ACTIVE_CONNECTION as usize] =
    [zero_app_env_tag(); APP_EASY_MAX_ACTIVE_CONNECTION as usize];

#[link_section = "retention_mem_area0"]
#[no_mangle]
static mut app_prf_srv_perm: [AppPrfSrvSec; PRFS_TASK_ID_MAX as usize] =
    [zero_app_prf_srv_sec(); PRFS_TASK_ID_MAX as usize];

#[link_section = "retention_mem_area0"]
#[no_mangle]
static mut device_info: AppDeviceInfo = AppDeviceInfo {
    dev_name: AppDeviceName {
        length: 0,
        name: [0; 32],
    },
    appearance: 0,
};

static USER_DEVICE_NAME: [u8; 5] = *b"10000";

macro_rules! prf_func_callbacks_size {
    () => {
        0
    };
    ($head: literal $($tail: literal)*) => {
        cfg!(feature = $head) as usize + prf_func_callbacks_size!($($tail)*)
    };
}


const PRF_FUNCS: [PrfFuncCallbacks;
    prf_func_callbacks_size!(
        "profile_gatt_client"
        "profile_prox_reporter"
        "profile_batt_server"
        "profile_findme_target"
        "profile_findme_locator"
        "profile_dis_server"
        "profile_bms_server"
        "profile_anc_client"
        "profile_bcs_server"
        "profile_uds_server"
        "profile_cts_server"
        "profile_cts_client"
        "profile_suota_receiver"
        "profile_wss_server"
    )] = [
    #[cfg(feature = "profile_gatt_client")]
    PrfFuncCallbacks {
        task_id: TASK_ID_GATT_CLIENT,
        db_create_func: Some(crate::bindings::app_gattc_create_task),
        enable_func: None,
    },
    #[cfg(feature = "profile_prox_reporter")]
    PrfFuncCallbacks {
        task_id: TASK_ID_PROXR,
        db_create_func: Some(crate::bindings::app_proxr_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_batt_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_BASS,
        db_create_func: Some(crate::bindings::app_bass_create_db),
        enable_func: Some(crate::bindings::app_bass_enable),
    },
    #[cfg(feature = "profile_findme_target")]
    PrfFuncCallbacks {
        task_id: TASK_ID_FINDT,
        db_create_func: Some(crate::bindings::app_findt_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_findme_locator")]
    PrfFuncCallbacks {
        task_id: TASK_ID_FINDL,
        db_create_func: Some(crate::bindings::app_findl_create_task),
        enable_func: Some(crate::bindings::app_findl_enable),
    },
    #[cfg(feature = "profile_dis_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_DISS,
        db_create_func: Some(crate::bindings::app_diss_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_bms_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_BMSS,
        db_create_func: Some(crate::bindings::app_bmss_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_anc_client")]
    PrfFuncCallbacks {
        task_id: TASK_ID_ANCC,
        db_create_func: Some(crate::bindings::app_ancc_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_bcs_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_BCSS,
        db_create_func: Some(crate::bindings::app_bcss_create_db),
        enable_func: Some(crate::bindings::app_bcss_enable),
    },
    #[cfg(feature = "profile_uds_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_UDSS,
        db_create_func: Some(crate::bindings::app_udss_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_cts_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_CTSS,
        db_create_func: Some(crate::bindings::app_ctss_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_cts_client")]
    PrfFuncCallbacks {
        task_id: TASK_ID_CTSC,
        db_create_func: Some(crate::bindings::app_ctsc_create_task),
        enable_func: Some(crate::bindings::app_ctsc_enable),
    },
    #[cfg(feature = "profile_suota_receiver")]
    PrfFuncCallbacks {
        task_id: TASK_ID_SUOTAR,
        db_create_func: Some(crate::bindings::app_suotar_create_db),
        enable_func: None,
    },
    #[cfg(feature = "profile_wss_server")]
    PrfFuncCallbacks {
        task_id: TASK_ID_WSSS,
        db_create_func: Some(crate::bindings::app_wsss_create_db),
        enable_func: Some(crate::bindings::app_wsss_enable),
    },
];

static TASK_DESC_APP: KeTaskDesc = KeTaskDesc {
    state_handler: core::ptr::null(),
    default_handler: unsafe { &app_default_handler },
    state: unsafe { &app_state as *const _ as *mut _ },
    state_max: APP_STATE_MAX as u16,
    idx_max: APP_IDX_MAX as u16,
};

// #[link_section = "retention_mem_area0"]
// static mut ADV_TIMER_ID: TimerHandle = 0;

// static void (*adv_timeout_callback)(void)                                          __SECTION_ZERO("retention_mem_area0"); // @RETENTION MEMORY

#[cfg(feature = "profile_custom_server1")]
pub extern "C" fn app_custs1_enable(conhdl: u8) {
    unsafe {
        crate::bindings::app_custs1_enable(conhdl as u16);
    }
}

#[cfg(feature = "profile_custom_server2")]
pub unsafe extern "C" fn app_custs2_enable(conhdl: u8) {
    unsafe {
        crate::bindings::app_custs2_enable(conhdl as u16);
    }
}

#[cfg(feature = "profile_custom_server")]
extern "Rust" {
    pub static CUST_PRF_FUNCS: [CustPrfFuncCallbacks;
        prf_func_callbacks_size!(
            "profile_custom_server1"
            "profile_custom_server2"
        )];
}

const USER_PRF_FUNCS: [PrfFuncCallbacks; 1] = [PrfFuncCallbacks {
    task_id: TASK_ID_INVALID,
    db_create_func: None,
    enable_func: None,
}];

const USER_ADV_CONF: AdvertiseConfiguration = AdvertiseConfiguration {
    #[cfg(feature = "address_mode_public")]
    addr_src: app_cfg_addr_src(APP_CFG_ADDR_PUB),
    #[cfg(feature = "address_mode_static")]
    addr_src: app_cfg_addr_src(APP_CFG_ADDR_STATIC),
    intv_min: ms_to_ble_slots(99),
    intv_max: ms_to_ble_slots(99),
    channel_map: ADV_ALL_CHNLS_EN as u8,
    mode: GAP_GEN_DISCOVERABLE as u8,
    adv_filt_policy: ADV_ALLOW_SCAN_ANY_CON_ANY as u8,
    peer_addr: [0x1, 0x2, 0x3, 0x4, 0x5, 0x6],
    peer_addr_type: 0,
};

const USER_GAPM_CONF: GapmConfiguration = GapmConfiguration {
    role: GAP_ROLE_PERIPHERAL,
    max_mtu: 23,
    #[cfg(feature = "address_mode_public")]
    addr_type: app_cfg_addr_type(APP_CFG_ADDR_PUB),
    #[cfg(feature = "address_mode_static")]
    addr_type: app_cfg_addr_type(APP_CFG_ADDR_STATIC),
    addr: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    renew_dur: 15000,
    irk: [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f,
    ],
    att_cfg: GAPM_MASK_ATT_SVC_CHG_EN as u8,
    gap_start_hdl: 0,
    gatt_start_hdl: 0,
    max_mps: 0,
    max_txoctets: 251,
    max_txtime: 2120,
};

#[cfg(feature = "address_mode_static")]
static mut APP_RANDOM_ADDR: BDAddr = BDAddr { addr: [0; 6] };

configure_user_adv_data!(
    {ADV_TYPE_COMPLETE_LIST_16BIT_SERVICE_IDS, 0x6b, 0xfd},
    {ADV_TYPE_MANUFACTURER_SPECIFIC_DATA, 0x98, 0x05, 0x01, 0x90, 0x01}
);

configure_user_scan_response_data!();

#[cfg(feature = "profile_custom_server")]
#[no_mangle]
pub extern "C" fn custs_get_func_callbacks(task_id: KeApiId) -> *const CustPrfFuncCallbacks {
    rprintln!("custs_get_func_callbacks({})", task_id);
    for pfcb in unsafe { CUST_PRF_FUNCS } {
        if pfcb.task_id == task_id {
            return &pfcb as *const _ as *const CustPrfFuncCallbacks;
        } else if pfcb.task_id == TASK_ID_INVALID {
            break;
        }
    }

    core::ptr::null()
}

fn update_device_info() {
    unsafe {
        let cropped_len = if USER_DEVICE_NAME.len() <= GAP_MAX_NAME_SIZE as usize {
            USER_DEVICE_NAME.len()
        } else {
            GAP_MAX_NAME_SIZE as usize
        };

        device_info.dev_name.length = cropped_len as u8;

        device_info.dev_name.name[..cropped_len].copy_from_slice(&USER_DEVICE_NAME[..cropped_len]);

        device_info.appearance = 0x0200; // Tag appearance
    }
}

fn reset_app_env() {
    let app_environment = unsafe { &mut app_env };

    app_environment.iter_mut().for_each(|env_entry| {
        *env_entry = AppEnvTag::default();
    });
}

fn app_task_in_user_app(task_id: KeApiId) -> bool {
    USER_PRF_FUNCS
        .iter()
        .find(|user_prf_func| user_prf_func.task_id == task_id)
        .is_some()
}

#[no_mangle]
pub extern "C" fn app_db_init_next() -> bool {
    rprintln!("app_db_init_next");
    let extract_cb = |user_prf_func: &PrfFuncCallbacks| user_prf_func.db_create_func;

    let callbacks = USER_PRF_FUNCS.iter().filter_map(extract_cb).chain(
        PRF_FUNCS
            .iter()
            .filter(|prf_func| !app_task_in_user_app(prf_func.task_id))
            .filter_map(extract_cb),
    );

    #[cfg(feature = "profile_custom_server")]
    let callbacks = callbacks.chain(unsafe {
        CUST_PRF_FUNCS
            .iter()
            .filter_map(|cust_prf_func| cust_prf_func.db_create_func)
    });

    callbacks.for_each(|cb| unsafe { cb() });

    true
}

#[no_mangle]
pub extern "C" fn app_prf_enable(conidx: u8) {
    rprintln!("app_prf_enable");

    let extract_cb = |user_prf_func: &PrfFuncCallbacks| user_prf_func.enable_func;

    let callbacks = USER_PRF_FUNCS.iter().filter_map(extract_cb).chain(
        PRF_FUNCS
            .iter()
            .filter(|prf_func| !app_task_in_user_app(prf_func.task_id))
            .filter_map(extract_cb),
    );

    #[cfg(feature = "profile_custom_server")]
    let callbacks = callbacks.chain(unsafe {
        CUST_PRF_FUNCS
            .iter()
            .filter_map(|cust_prf_func| cust_prf_func.enable_func)
    });

    callbacks.for_each(|cb| unsafe { cb(conidx) });
}

fn app_easy_gap_undirected_advertise_start_create_msg() -> KeMsgGapmStartAdvertiseCmd {
    assert!(USER_ADVERTISE_DATA.len() <= ADV_DATA_LEN as usize);
    assert!(USER_ADVERTISE_SCAN_RESPONSE_DATA.len() <= SCAN_RSP_DATA_LEN as usize);

    let mut cmd = KeMsgGapmStartAdvertiseCmd::new(TASK_APP as u16, TASK_GAPM as u16);

    let msg = cmd.fields();

    msg.op.code = GAPM_ADV_UNDIRECT as u8;

    msg.op.addr_src = USER_ADV_CONF.addr_src;
    msg.intv_min = USER_ADV_CONF.intv_min;
    msg.intv_max = USER_ADV_CONF.intv_max;
    msg.channel_map = USER_ADV_CONF.channel_map;
    msg.info.host.adv_filt_policy = USER_ADV_CONF.adv_filt_policy;
    if USER_ADV_CONF.adv_filt_policy == ADV_ALLOW_SCAN_ANY_CON_WLST as u8 {
        msg.info.host.mode = GAP_NON_DISCOVERABLE as u8;
    } else {
        msg.info.host.mode = USER_ADV_CONF.mode;
    }

    let host = unsafe { &mut msg.info.host };

    host.adv_data_len = USER_ADVERTISE_DATA.len() as u8;
    host.adv_data[..USER_ADVERTISE_DATA.len()].copy_from_slice(&USER_ADVERTISE_DATA);

    host.scan_rsp_data_len = USER_ADVERTISE_SCAN_RESPONSE_DATA.len() as u8;
    host.scan_rsp_data[..USER_ADVERTISE_SCAN_RESPONSE_DATA.len()]
        .copy_from_slice(&USER_ADVERTISE_SCAN_RESPONSE_DATA);

    // #if (USER_CFG_ADDRESS_MODE == APP_CFG_CNTL_PRIV_RPA_RAND)
    //         // Local Address has been added to RAL. Use this entry to advertise with RPA
    //         memcpy(cmd->info.host.peer_info.addr.addr, &(gapm_env.addr), BD_ADDR_LEN * sizeof(uint8_t));
    //         cmd->info.host.peer_info.addr_type = ((GAPM_F_GET(gapm_env.cfg_flags, ADDR_TYPE) == GAPM_CFG_ADDR_PUBLIC) ? ADDR_PUBLIC : ADDR_RAND);

    // #elif (USER_CFG_ADDRESS_MODE == APP_CFG_CNTL_PRIV_RPA_PUB)
    //         // If there is at least one bond device in Bond Database, use its address to advertise with RPA, else use Public address
    //         uint8_t bdb_size;
    //         bool bonded_dev_found = false;
    //         struct gap_ral_dev_info bonded_dev_info;

    //         // Get Bond Database size
    //         bdb_size = app_easy_security_bdb_get_size();
    //         for (uint8_t bdb_slot = 0; bdb_slot < bdb_size; bdb_slot++)
    //         {
    //             bonded_dev_found = app_easy_security_bdb_get_device_info_from_slot(bdb_slot, &bonded_dev_info);
    //             if (bonded_dev_found)
    //             {
    //                 // Add peer device identity found in advertising command.
    //                 memcpy(cmd->info.host.peer_info.addr.addr, &bonded_dev_info.addr, BD_ADDR_LEN * sizeof(uint8_t));
    //                 cmd->info.host.peer_info.addr_type = bonded_dev_info.addr_type;
    //                 break;
    //             }
    //         }
    // #endif

    if USER_DEVICE_NAME.len() > 0 {
        let total_adv_space = 3 + host.adv_data_len as u16 + 2 + USER_DEVICE_NAME.len() as u16;
        let total_scan_space = host.scan_rsp_data_len as u16 + 2 + USER_DEVICE_NAME.len() as u16;

        if total_adv_space <= ADV_DATA_LEN as u16 {
            let mut offset = host.adv_data_len as usize;

            host.adv_data[offset] = 1 + USER_DEVICE_NAME.len() as u8;
            offset += 1;
            host.adv_data[offset] = GAP_AD_TYPE_COMPLETE_NAME as u8;
            offset += 1;

            host.adv_data[offset..(offset + USER_DEVICE_NAME.len())]
                .copy_from_slice(&USER_DEVICE_NAME);

            host.adv_data_len = (offset + USER_DEVICE_NAME.len()) as u8;
        } else if total_scan_space <= SCAN_RSP_DATA_LEN as u16 {
            let mut offset = host.scan_rsp_data_len as usize;

            host.scan_rsp_data[offset] = 1 + USER_DEVICE_NAME.len() as u8;
            offset += 1;
            host.scan_rsp_data[offset] = GAP_AD_TYPE_COMPLETE_NAME as u8;
            offset += 1;

            host.scan_rsp_data[offset..(offset + USER_DEVICE_NAME.len())]
                .copy_from_slice(&USER_DEVICE_NAME);

            host.scan_rsp_data_len = (offset + USER_DEVICE_NAME.len()) as u8;
        }
    }

    cmd
}

fn app_easy_gap_dev_config_create_msg() -> KeMsgGapmSetDevConfigCmd {
    let mut cmd = KeMsgGapmSetDevConfigCmd::new(TASK_APP as u16, TASK_GAPM as u16);

    let msg = cmd.fields();

    msg.operation = GAPM_SET_DEV_CONFIG as u8;

    msg.role = USER_GAPM_CONF.role as u8;
    msg.max_mtu = USER_GAPM_CONF.max_mtu;
    msg.addr_type = USER_GAPM_CONF.addr_type;

    #[cfg(any(feature = "address_mode_public", feature = "address_mode_static"))]
    {
        msg.renew_dur = 0;
        msg.irk.key.copy_from_slice(&[0; KEY_LEN as usize]);
    }
    #[cfg(not(any(feature = "address_mode_public", feature = "address_mode_static")))]
    {
        msg.renew_dur = USER_GAPM_CONF.renew_dur;
        msg.irk.key.copy_from_slice(&USER_GAPM_CONF.irk);
    }

    #[cfg(feature = "address_mode_static")]
    {
        let tmp_addr = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

        todo!();
        //         // Check whether the user-defined Random Static address is null.
        //         if (memcmp(USER_GAPM_CONF.addr, &co_null_bdaddr, BD_ADDR_LEN) == 0)
        //         {
        //             CALLBACK_ARGS_1(user_app_callbacks.app_on_generate_static_random_addr, &app_random_addr)
        //             memcpy(msg.addr.addr, app_random_addr.addr, BD_ADDR_LEN * sizeof(uint8_t));
        //         }
        //         else
        //         {
        //             // All the (BD_ADDR_LEN - 1) * sizeof(uint8_t) least significant
        //             // bits of the user-defined Random Static address shall NOT be equal to 1.
        //             ASSERT_ERROR(memcmp(USER_GAPM_CONF.addr, tmp_addr, BD_ADDR_LEN - 1) != 0);
        //             // The two most significant bits of the user-defined Random Static
        //             // address shall be equal to 1.
        //             ASSERT_ERROR((USER_GAPM_CONF.addr[BD_ADDR_LEN - 1] & GAP_STATIC_ADDR) == GAP_STATIC_ADDR)
        //             memcpy(msg.addr.addr, USER_GAPM_CONF.addr, BD_ADDR_LEN * sizeof(uint8_t));
        //         }
    }

    msg.att_cfg = USER_GAPM_CONF.att_cfg;
    msg.gap_start_hdl = USER_GAPM_CONF.gap_start_hdl;
    msg.gatt_start_hdl = USER_GAPM_CONF.gatt_start_hdl;
    msg.max_mps = USER_GAPM_CONF.max_mps;
    unsafe {
        msg.max_txoctets = co_min(USER_GAPM_CONF.max_txoctets, llm_le_env.supportedMaxTxOctets);
        msg.max_txtime = co_min(USER_GAPM_CONF.max_txtime, llm_le_env.supportedMaxTxTime);
    }

    cmd
}

#[no_mangle]
pub extern "C" fn app_init() {
    reset_app_env();

    update_device_info();

    ke_task_create(TASK_APP as u8, &TASK_DESC_APP);

    ke_state_set(TASK_APP as u16, APP_DISABLED as u8);
}

pub fn app_easy_gap_undirected_advertise_start() {
    app_easy_gap_undirected_advertise_start_create_msg().send();
    ke_state_set(TASK_APP as u16, APP_CONNECTABLE as u8);
}

// #[inline]
// pub fn app_easy_gap_undirected_advertise_start() {
//     unsafe {
//         crate::bindings::app_easy_gap_undirected_advertise_start();
//     }
// }

#[no_mangle]
pub extern "C" fn app_easy_gap_dev_configure() {
    app_easy_gap_dev_config_create_msg().send();
}

// fn app_easy_gap_non_connectable_advertise_start_create_msg() -> KeMsgGapmStartAdvertiseCmd {
//     assert!(USER_ADVERTISE_DATA.len() <= ADV_DATA_LEN as usize);
//     assert!(USER_ADVERTISE_SCAN_RESPONSE_DATA.len() <= SCAN_RSP_DATA_LEN as usize);
//     let mut cmd = KeMsgGapmStartAdvertiseCmd::new(TASK_APP as u16, TASK_GAPM as u16);
//     cmd.fields().op.code = GAPM_ADV_NON_CONN as u8;
//     unsafe {
//         cmd.fields().op.addr_src = USER_ADV_CONF.addr_src;
//         cmd.fields().intv_min = USER_ADV_CONF.intv_min;
//         cmd.fields().intv_max = USER_ADV_CONF.intv_max;
//         cmd.fields().channel_map = USER_ADV_CONF.channel_map;
//         cmd.fields().info.host.mode = USER_ADV_CONF.mode;
//         cmd.fields().info.host.adv_filt_policy = USER_ADV_CONF.adv_filt_policy;
//     }
//     let host = unsafe { &mut cmd.fields().info.host };
//     let adv_data_len = &mut host.adv_data_len;
//     let adv_data = &mut host.adv_data;
//     *adv_data_len = USER_ADVERTISE_DATA.len() as u8;
//     adv_data.clone_from_slice(&USER_ADVERTISE_DATA);
//     let scan_rsp_data_len = &mut host.scan_rsp_data_len;
//     let scan_rsp_data = &mut host.scan_rsp_data;
//     *scan_rsp_data_len = USER_ADVERTISE_SCAN_RESPONSE_DATA.len() as u8;
//     scan_rsp_data.clone_from_slice(USER_ADVERTISE_SCAN_RESPONSE_DATA);
//     if USER_DEVICE_NAME.len() > 0 {
//         let adv_data_len = unsafe { cmd.fields().info.host.adv_data_len };
//         let scan_rsp_data_len = unsafe { cmd.fields().info.host.scan_rsp_data_len };
//         let total_adv_space = adv_data_len as u16 + 2 + USER_DEVICE_NAME.len() as u16;
//         let total_scan_space = scan_rsp_data_len as u16 + 2 + USER_DEVICE_NAME.len() as u16;
//         if total_adv_space <= ADV_DATA_LEN as u16 {
//             let host = unsafe { &mut cmd.fields().info.host };
//             let adv_data_len = &mut host.adv_data_len;
//             let adv_data = &mut host.adv_data;
//             append_device_name(
//                 adv_data_len,
//                 USER_DEVICE_NAME.len(),
//                 &mut adv_data[*adv_data_len as usize..],
//                 USER_DEVICE_NAME,
//             );
//         } else if total_scan_space <= SCAN_RSP_DATA_LEN as u16 {
//             let host = unsafe { &mut cmd.fields().info.host };
//             let scan_rsp_data_len = &mut host.scan_rsp_data_len;
//             let scan_rsp_data = &mut host.scan_rsp_data;
//             append_device_name(
//                 scan_rsp_data_len,
//                 USER_DEVICE_NAME.len(),
//                 &mut scan_rsp_data[*scan_rsp_data_len as usize..],
//                 USER_DEVICE_NAME,
//             );
//         }
//     }
//     cmd
// }

// fn app_easy_gap_directed_advertise_start_create_msg(
//     ldc_enable: bool,
// ) -> KeMsgGapmStartAdvertiseCmd {
//     let mut cmd = KeMsgGapmStartAdvertiseCmd::new(TASK_APP as u16, TASK_GAPM as u16);
//     if ldc_enable {
//         cmd.fields().op.code = GAPM_ADV_DIRECT_LDC as u8;
//         unsafe {
//             cmd.fields().intv_min = USER_ADV_CONF.intv_min;
//             cmd.fields().intv_max = USER_ADV_CONF.intv_max;
//         }
//     } else {
//         cmd.fields().op.code = GAPM_ADV_DIRECT as u8;
//         cmd.fields().intv_min = LLM_ADV_INTERVAL_MIN as u16;
//         cmd.fields().intv_max = LLM_ADV_INTERVAL_MAX as u16;
//     }
//     unsafe {
//         cmd.fields().op.addr_src = USER_ADV_CONF.addr_src;
//         cmd.fields().channel_map = USER_ADV_CONF.channel_map;
//         cmd.fields()
//             .info
//             .direct
//             .addr
//             .addr
//             .copy_from_slice(&USER_ADV_CONF.peer_addr);
//         cmd.fields().info.direct.addr_type = USER_ADV_CONF.peer_addr_type;
//     }
//     cmd
// }

// fn app_easy_gap_param_update_msg_create(conidx: u8) -> KeMsgGapcParamUpdateCmd {
//     let mut cmd =
//         KeMsgGapcParamUpdateCmd::new(TASK_APP as u16, ke_build_id!(TASK_GAPC as u8, conidx));
//     unsafe {
//         cmd.fields().intv_max = user_connection_param_conf.intv_max;
//         cmd.fields().intv_min = user_connection_param_conf.intv_min;
//         cmd.fields().latency = user_connection_param_conf.latency;
//         cmd.fields().time_out = user_connection_param_conf.time_out;
//         cmd.fields().ce_len_min = user_connection_param_conf.ce_len_min;
//         cmd.fields().ce_len_max = user_connection_param_conf.ce_len_max;
//     }
//     cmd
// }

// fn app_easy_gap_start_connection_to_msg_create() -> KeMsgGapmStartConnectionCmd {
//     let mut cmd = KeMsgDynGapmStartConnectionCmd::new(TASK_APP as u16, TASK_GAPM as u16);
//     unsafe {
//         cmd.fields().op.code = user_central_conf.code;
//         cmd.fields().op.addr_src = user_central_conf.addr_src;
//         cmd.fields().scan_interval = user_central_conf.scan_interval;
//         cmd.fields().scan_window = user_central_conf.scan_window;
//         cmd.fields().con_intv_min = user_central_conf.con_intv_min;
//         cmd.fields().con_intv_max = user_central_conf.con_intv_max;
//         cmd.fields().con_latency = user_central_conf.con_latency;
//         cmd.fields().superv_to = user_central_conf.superv_to;
//         cmd.fields().ce_len_min = user_central_conf.ce_len_min;
//         cmd.fields().ce_len_max = user_central_conf.ce_len_max;
//     }
//     if (unsafe { user_central_conf.code } == GAPM_CONNECTION_DIRECT as u8)
//         || (unsafe { user_central_conf.code } == GAPM_CONNECTION_NAME_REQUEST as u8)
//     {
//         cmd.fields().nb_peers = 1;
//     } else {
//         cmd.fields().nb_peers = CFG_MAX_CONNECTIONS as u8;
//         let peers = unsafe {
//             cmd.fields()
//                 .peers
//                 .as_mut_slice(CFG_MAX_CONNECTIONS as usize)
//         };
//         for i in 0..CFG_MAX_CONNECTIONS as usize {
//             let user_central_configuration =
//                 unsafe { &*(&user_central_conf as *const _ as *const CentralConfiguration) };
//             peers[i]
//                 .addr
//                 .addr
//                 .copy_from_slice(&user_central_configuration.peer_addresses[i].peer_addr);
//             peers[i].addr_type = user_central_configuration.peer_addresses[i].peer_addr_type;
//         }
//     }
//     cmd
// }

// pub fn app_easy_gap_advertise_stop_handler() {
//     let app_timer_id = unsafe { &mut ADV_TIMER_ID };
//     unsafe {
//         app_easy_gap_advertise_stop();
//     }
//     *app_timer_id = 0;
//     //     if(adv_timeout_callback!=NULL)
//     //     {
//     //         timeout_callback = adv_timeout_callback;
//     //         adv_timeout_callback = NULL;
//     //         timeout_callback();
//     //     }
// }

// pub fn app_easy_gap_directed_advertise_start(ldc_enable: bool) {
//     let cmd = app_easy_gap_directed_advertise_start_create_msg(ldc_enable);
//     cmd.send();
//     ke_state_set(TASK_APP as u16, APP_CONNECTABLE as u8);
// }

// pub fn app_easy_gap_non_connectable_advertise_start() {
//     let cmd = app_easy_gap_non_connectable_advertise_start_create_msg();
//     cmd.send();
//     let state = ke_state_get(TASK_APP as u16);
//     if state != APP_CONNECTED as u8 {
//         ke_state_set(TASK_APP as u16, APP_CONNECTABLE as u8);
//     }
// }

// pub fn app_easy_gap_param_update_start(conidx: u8) {
//     app_easy_gap_param_update_msg_create(conidx).send();
// }

// pub fn app_easy_gap_start_connection_to() {
//     app_easy_gap_start_connection_to_msg_create().send();
// }

// pub fn app_easy_gap_get_peer_features(conidx: u8) {
//     let mut cmd = KeMsgGapcGetInfoCmd::new(TASK_APP as u16, ke_build_id!(TASK_GAPC as u8, conidx));
//     cmd.fields().operation = GAPC_GET_PEER_FEATURES as u8;
//     cmd.send();
// }

// #[cfg(not(feature = "exclude_dlg_timer"))]
// pub fn app_easy_gap_undirected_advertise_with_timeout_start(delay: u32, callback: TimeoutCallback) {
//     todo!();
//     //     //stop the current running timer
//     //     if(adv_timer_id != EASY_TIMER_INVALID_TIMER)
//     //         app_easy_timer_cancel(adv_timer_id);
//     //     if(timeout_callback != NULL)
//     //         adv_timeout_callback = timeout_callback;
//     //     adv_timer_id = app_easy_timer(delay, app_easy_gap_advertise_stop_handler);
//     //     app_easy_gap_undirected_advertise_start();
// }

// #[cfg(not(feature = "exclude_dlg_timer"))]
// pub fn app_easy_gap_advertise_with_timeout_stop() {
//     todo!();
//     //     //stop the current running timer
//     //     if (adv_timer_id != EASY_TIMER_INVALID_TIMER)
//     //     {
//     //         app_easy_timer_cancel(adv_timer_id);
//     //     }
//     //     adv_timer_id = EASY_TIMER_INVALID_TIMER;
//     //     adv_timeout_callback = NULL;
// }

// fn app_easy_gap_non_connectable_advertise_get_active() -> &'static KeMsgGapmStartAdvertiseCmd {
//     app_easy_gap_non_connectable_advertise_start_create_msg()
// }

// fn app_easy_gap_directed_advertise_get_active(
//     ldc_enable: bool,
// ) -> &'static KeMsgGapmStartAdvertiseCmd {
//     app_easy_gap_directed_advertise_start_create_msg(ldc_enable)
// }

// fn app_easy_gap_undirected_advertise_get_active() -> &'static KeMsgGapmStartAdvertiseCmd {
//     app_easy_gap_undirected_advertise_start_create_msg()
// }

// fn app_easy_gap_param_update_get_active(conidx: u8) -> &'static KeMsgGapcParamUpdateCmd {
//     app_easy_gap_param_update_msg_create(conidx)
// }

// fn app_easy_gap_start_connection_to_get_active() -> &'static KeMsgGapmStartConnectionCmd {
//     app_easy_gap_start_connection_to_msg_create()
// }

// fn app_easy_gap_dev_config_get_active() -> &'static KeMsgGapmSetDevConfigCmd {
//     app_easy_gap_dev_config_create_msg()
// }

// fn app_easy_gap_start_connection_to_set(peer_addr_type: u8, peer_addr: *const u8, intv: u16) {
//     let mut msg = app_easy_gap_start_connection_to_msg_create();
//     msg.fields().nb_peers = 1;
//     let peers = unsafe { msg.fields().peers.as_mut_slice(1) };
//     // memcpy(&msg.fields().peers[0].addr, peer_addr, BD_ADDR_LEN);
//     // msg.fields().peers[0].addr_type = peer_addr_type;
//     peers[0].addr_type = peer_addr_type;
//     msg.fields().con_intv_max = intv;
//     msg.fields().con_intv_min = intv;
// }
