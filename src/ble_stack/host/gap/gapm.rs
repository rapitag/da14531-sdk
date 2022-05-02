pub mod task {
    pub use crate::bindings::{
        gapm_msg_id_GAPM_PROFILE_TASK_ADD_CMD as GAPM_PROFILE_TASK_ADD_CMD,
        gapm_profile_task_add_cmd as GapmProfileTaskAdd,
        gapm_start_advertise_cmd as GapmStartAdvertiseCmd,
    };
    use crate::platform::core_modules::ke::msg::kernel_msg_type;

    kernel_msg_type!(GapmProfileTaskAdd, GAPM_PROFILE_TASK_ADD_CMD);
}
