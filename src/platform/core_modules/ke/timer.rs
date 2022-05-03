use super::{msg::KeMsgId, task::KeTaskId};

#[inline]
pub fn ke_timer_set(timer_id: KeMsgId, task: KeTaskId, delay: u32) {
    unsafe {
        crate::bindings::ke_timer_set(timer_id, task, delay);
    }
}

#[inline]
pub fn ke_timer_clear(timer_id: KeMsgId, task: KeTaskId) {
    unsafe {
        crate::bindings::ke_timer_clear(timer_id, task);
    }
}
