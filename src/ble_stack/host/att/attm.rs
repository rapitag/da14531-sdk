pub use crate::{
    bindings::{
        attm_perm_mask_PERM_MASK_RD as PERM_MASK_RD, attm_perm_mask_PERM_MASK_WR as PERM_MASK_WR,
        attm_perm_mask_PERM_MASK_WRITE_COMMAND as PERM_MASK_WRITE_COMMAND,
        attm_perm_mask_PERM_MASK_WRITE_REQ as PERM_MASK_WRITE_REQ,
        attm_perm_mask_PERM_POS_RD as PERM_POS_RD, attm_perm_mask_PERM_POS_WR as PERM_POS_WR,
        attm_perm_mask_PERM_POS_WRITE_COMMAND as PERM_POS_WRITE_COMMAND,
        attm_perm_mask_PERM_POS_WRITE_REQ as PERM_POS_WRITE_REQ,
        attm_svc_perm_mask_PERM_MASK_SVC_AUTH as PERM_MASK_SVC_AUTH,
        attm_svc_perm_mask_PERM_MASK_SVC_EKS as PERM_MASK_SVC_EKS,
        attm_svc_perm_mask_PERM_MASK_SVC_MI as PERM_MASK_SVC_MI,
        attm_svc_perm_mask_PERM_MASK_SVC_PRIMARY as PERM_MASK_SVC_PRIMARY,
        attm_svc_perm_mask_PERM_POS_SVC_MI as PERM_POS_SVC_MI,
        attm_svc_perm_mask_PERM_POS_SVC_PRIMARY as PERM_POS_SVC_PRIMARY,
        attm_value_perm_mask_PERM_MASK_RI as PERM_MASK_RI,
        attm_value_perm_mask_PERM_POS_RI as PERM_POS_RI, PERM_RIGHT_AUTH, PERM_RIGHT_DISABLE,
        PERM_RIGHT_ENABLE, PERM_RIGHT_SECURE, PERM_RIGHT_UNAUTH,
    },
    ble_stack::profiles::prf::{PERM_MASK_PRF_MI, PERM_POS_PRF_MI},
};

use crate::platform::core_modules::ke::task::KeTaskId;

unsafe impl Sync for AttmDesc128 {}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttmDesc128 {
    pub uuid: *const u8,
    pub uuid_size: u8,
    pub perm: u32,
    pub max_length: u16,
    pub length: u16,
    pub value: *const u8,
}

// TODO: rustify
#[inline]
pub fn attm_svc_create_db_128(
    svc_idx: u8,
    shdl: &mut u16,
    cfg_flag: *mut u8,
    max_nb_att: u8,
    att_tbl: *mut u8,
    dest_id: KeTaskId,
    att_db: *const AttmDesc128,
    svc_perm: u8,
) -> u8 {
    unsafe {
        crate::bindings::attm_svc_create_db_128(
            svc_idx,
            shdl,
            cfg_flag,
            max_nb_att,
            att_tbl,
            dest_id,
            att_db as *const _,
            svc_perm,
        )
    }
}
