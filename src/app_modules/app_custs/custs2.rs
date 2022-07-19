use crate::{
    app_modules::get_user_prf_srv_perm,
    bindings::{
        custs2_db_cfg as Custs2DbCfg,
        gapm_operation_GAPM_PROFILE_TASK_ADD as GAPM_PROFILE_TASK_ADD, KE_API_ID_TASK_ID_CUSTS2,
        KE_TASK_TYPE_TASK_APP as TASK_APP, KE_TASK_TYPE_TASK_GAPM as TASK_GAPM,
    },
    ble_stack::host::gap::gapm::task::KeMsgDynGapmProfileTaskAdd,
};

#[no_mangle]
pub extern "C" fn app_custs2_create_db() {
    const SIZE: u16 = core::mem::size_of::<Custs2DbCfg>() as u16;
    let mut msg = KeMsgDynGapmProfileTaskAdd::<SIZE>::new(TASK_APP as u16, TASK_GAPM as u16);

    msg.fields().operation = GAPM_PROFILE_TASK_ADD as u8;
    msg.fields().sec_lvl = get_user_prf_srv_perm(KE_API_ID_TASK_ID_CUSTS2) as u8;
    msg.fields().prf_task_id = KE_API_ID_TASK_ID_CUSTS2 as u16;
    msg.fields().app_task = TASK_APP as u16;
    msg.fields().start_hdl = 0;

    let db_cfg_ptr = &mut msg.fields().param as *mut _ as *mut Custs2DbCfg;

    let db_cfg = unsafe { db_cfg_ptr.as_mut().unwrap() };

    // Attribute table. In case the handle offset needs to be saved
    db_cfg.att_tbl = core::ptr::null_mut();
    db_cfg.cfg_flag = core::ptr::null_mut();
    db_cfg.features = 0;

    msg.send();
}
