pub use crate::bindings::{
    ke_state_handler as KeStateHandler, ke_state_t as KeState, ke_task_desc as KeTaskDesc,
    ke_task_id_t as KeTaskId,
};

unsafe impl Sync for KeTaskDesc {}

#[inline]
pub fn ke_state_set(task_id: KeTaskId, state_id: KeState) {
    unsafe {
        crate::bindings::ke_state_set(task_id, state_id);
    }
}

#[inline]
pub fn ke_state_get(task_id: KeTaskId) -> KeState {
    unsafe { crate::bindings::ke_state_get(task_id) }
}

#[inline]
pub fn ke_task_create(task_type: u8, task_desc: &KeTaskDesc) {
    unsafe {
        crate::bindings::ke_task_create(task_type, task_desc);
    }
}
