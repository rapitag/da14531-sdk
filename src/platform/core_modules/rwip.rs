pub use crate::bindings::{
    KE_API_ID as KeApiId, KE_API_ID_TASK_ID_CUSTS1 as TASK_ID_CUSTS1,
    KE_API_ID_TASK_ID_INVALID as TASK_ID_INVALID, KE_MEM_ATT_DB, KE_MEM_ENV, KE_MEM_KE_MSG,
    KE_MEM_NON_RETENTION, KE_TASK_TYPE as KeTaskType, KE_TASK_TYPE_TASK_APP as TASK_APP,
    KE_TASK_TYPE_TASK_GAPC as TASK_GAPC, KE_TASK_TYPE_TASK_GAPM as TASK_GAPM,
};

#[inline]
pub fn rwip_schedule() {
    unsafe {
        crate::bindings::rwip_schedule();
    }
}
