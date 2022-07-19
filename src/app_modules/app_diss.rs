use rtt_target::rprintln;

use crate::{
    app_modules::get_user_prf_srv_perm,
    ble_stack::{
        host::gap::gapm::task::{KeMsgDynGapmProfileTaskAdd, GAPM_PROFILE_TASK_ADD},
        profiles::dis::diss::{
            DissDbCfg, DIS_FIRM_REV_STR_CHAR_SUP, DIS_MANUFACTURER_NAME_CHAR_SUP,
            DIS_MODEL_NB_STR_CHAR_SUP, DIS_PNP_ID_CHAR_SUP, DIS_SW_REV_STR_CHAR_SUP,
            DIS_SYSTEM_ID_CHAR_SUP,
        },
    },
    platform::core_modules::rwip::{TASK_APP, TASK_GAPM, TASK_ID_DISS},
};

pub mod task;

#[no_mangle]
pub extern "C" fn app_dis_init() {
    // rprintln!("app_dis_init");
    // Nothing to do
}

#[no_mangle]
pub extern "C" fn app_diss_create_db() {
    // rprintln!("app_diss_create_db");
    const SIZE: u16 = core::mem::size_of::<DissDbCfg>() as u16;
    let mut msg = KeMsgDynGapmProfileTaskAdd::<SIZE>::new(TASK_APP as u16, TASK_GAPM as u16);

    msg.fields().operation = GAPM_PROFILE_TASK_ADD as u8;
    msg.fields().sec_lvl = get_user_prf_srv_perm(TASK_ID_DISS) as u8;
    msg.fields().prf_task_id = TASK_ID_DISS as u16;
    msg.fields().app_task = TASK_APP as u16;
    msg.fields().start_hdl = 0;

    let db_cfg_ptr = &mut msg.fields().param as *mut _ as *mut DissDbCfg;

    let db_cfg = unsafe { db_cfg_ptr.as_mut().unwrap() };

    // TODO: Make user configurable!
    db_cfg.features = (DIS_MANUFACTURER_NAME_CHAR_SUP
        | DIS_MODEL_NB_STR_CHAR_SUP
        | DIS_SYSTEM_ID_CHAR_SUP
        | DIS_SW_REV_STR_CHAR_SUP
        | DIS_FIRM_REV_STR_CHAR_SUP
        | DIS_PNP_ID_CHAR_SUP) as u16;

    msg.send();
}
