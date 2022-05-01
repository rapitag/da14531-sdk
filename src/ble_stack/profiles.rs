use crate::platform::core_modules::ke::msg::{KeMsgId, KeTaskId};

pub mod custom;

pub fn prf_get_task_from_id(msg_id: KeMsgId) -> KeTaskId {
    unsafe { crate::bindings::prf_get_task_from_id(msg_id) }
}
