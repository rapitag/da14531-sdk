use crate::{
    app_modules::get_user_prf_srv_perm,
    ble_stack::host::gap::gapm::task::KeMsgDynGapmProfileTaskAdd,
    platform::core_modules::rwip::{TASK_APP, TASK_GAPM},
};

pub use crate::bindings::{
    custs1_db_cfg as Custs1DbCfg, gapm_operation_GAPM_PROFILE_TASK_ADD as GAPM_PROFILE_TASK_ADD,
    KE_API_ID_TASK_ID_CUSTS1,
};

#[no_mangle]
pub extern "C" fn app_custs1_init() {
    // Nothing to do
}

#[no_mangle]
pub extern "C" fn app_custs1_create_db() {
    const SIZE: u16 = core::mem::size_of::<Custs1DbCfg>() as u16;
    let mut msg = KeMsgDynGapmProfileTaskAdd::<SIZE>::new(TASK_APP as u16, TASK_GAPM as u16);

    msg.fields().operation = GAPM_PROFILE_TASK_ADD as u8;
    msg.fields().sec_lvl = get_user_prf_srv_perm(KE_API_ID_TASK_ID_CUSTS1) as u8;
    msg.fields().prf_task_id = KE_API_ID_TASK_ID_CUSTS1 as u16;
    msg.fields().app_task = TASK_APP as u16;
    msg.fields().start_hdl = 0;

    let db_cfg_ptr = &mut msg.fields().param as *mut _ as *mut Custs1DbCfg;

    let db_cfg = unsafe { db_cfg_ptr.as_mut().unwrap() };

    // Attribute table. In case the handle offset needs to be saved
    db_cfg.att_tbl = core::ptr::null_mut();
    db_cfg.cfg_flag = core::ptr::null_mut();
    db_cfg.features = 0;

    msg.send();
}
