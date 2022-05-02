pub use crate::{
    bindings::{
        attm_desc_128 as AttmDesc128, attm_svc_perm_mask_PERM_MASK_SVC_AUTH as PERM_MASK_SVC_AUTH,
        attm_svc_perm_mask_PERM_MASK_SVC_EKS as PERM_MASK_SVC_EKS,
        attm_svc_perm_mask_PERM_MASK_SVC_MI as PERM_MASK_SVC_MI,
        attm_svc_perm_mask_PERM_MASK_SVC_PRIMARY as PERM_MASK_SVC_PRIMARY,
        attm_svc_perm_mask_PERM_POS_SVC_MI as PERM_POS_SVC_MI,
        attm_svc_perm_mask_PERM_POS_SVC_PRIMARY as PERM_POS_SVC_PRIMARY, PERM_RIGHT_AUTH,
        PERM_RIGHT_DISABLE, PERM_RIGHT_ENABLE, PERM_RIGHT_SECURE, PERM_RIGHT_UNAUTH,
    },
    ble_stack::profiles::prf::{PERM_MASK_PRF_MI, PERM_POS_PRF_MI},
};

use crate::platform::core_modules::ke::task::KeTaskId;

// TODO: rustify
#[inline]
pub fn attm_svc_create_db_128(
    svc_idx: u8,
    shdl: &mut u16,
    cfg_flag: &mut u8,
    max_nb_att: u8,
    att_tbl: &mut u8,
    dest_id: KeTaskId,
    att_db: &AttmDesc128,
    svc_perm: u8,
) -> u8 {
    unsafe {
        crate::bindings::attm_svc_create_db_128(
            svc_idx,
            shdl as *mut _,
            cfg_flag as *mut _,
            max_nb_att,
            att_tbl as *mut _,
            dest_id,
            att_db as *const _,
            svc_perm,
        )
    }
}

macro_rules! perm {
    ($access: ident, $right:ident) => {
        paste::paste! {
            (($crate::ble_stack::host::att::attm::[<PERM_RIGHT_ $right>] << $crate::ble_stack::host::att::attm::[<PERM_POS_ $access>]) & $crate::ble_stack::host::att::attm::[<PERM_MASK_ $access>]) as u8
        }
    };
}

pub(crate) use perm;

macro_rules! perm_get {
    ($perm: ident, $access: ident) => {
        paste::paste! {
            ((($perm as u32) & $crate::ble_stack::host::att::attm::[<PERM_MASK_ $access>]) >> $crate::ble_stack::host::att::attm::[<PERM_POS_ $access>])
        }
    }
}
pub(crate) use perm_get;
