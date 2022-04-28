pub use crate::bindings::{
    CORE_MODULES_RWIP_TASK_APP as TASK_APP, KE_API_ID as KeApiId,
    KE_TASK_TYPE as KeTaskType,
};

#[inline]
pub fn rwip_schedule() {
    unsafe {
        crate::bindings::rwip_schedule();
    }
}