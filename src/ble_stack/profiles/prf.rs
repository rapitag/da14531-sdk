use crate::platform::core_modules::ke::{msg::KeMsgId, task::KeTaskId};

pub use crate::bindings::{
    prf_env as PrfEnv, prf_perm_mask_PERM_MASK_PRF_MI as PERM_MASK_PRF_MI,
    prf_perm_mask_PERM_MASK_PRF_TASK as PERM_MASK_PRF_TASK,
    prf_perm_mask_PERM_POS_PRF_MI as PERM_POS_PRF_MI,
    prf_perm_mask_PERM_POS_PRF_TASK as PERM_POS_PRF_TASK, prf_task_cbs as PrfTaskCbs,
    prf_task_env as PrfTaskEnv,
};

#[inline]
pub fn prf_get_task_from_id(msg_id: KeMsgId) -> KeTaskId {
    unsafe { crate::bindings::prf_get_task_from_id(msg_id) }
}

#[inline]
pub fn prf_src_task_get(env: &mut PrfEnv, conidx: u8) -> KeTaskId {
    unsafe { crate::bindings::prf_src_task_get(env as *mut _, conidx) }
}
