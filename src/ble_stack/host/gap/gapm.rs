pub mod task {
    pub use crate::bindings::{
        gapm_msg_id_GAPM_PROFILE_TASK_ADD_CMD as GAPM_PROFILE_TASK_ADD_CMD,
        gapm_msg_id_GAPM_SET_DEV_CONFIG_CMD as GAPM_SET_DEV_CONFIG_CMD,
        gapm_msg_id_GAPM_START_ADVERTISE_CMD as GAPM_START_ADVERTISE_CMD,
        gapm_msg_id_GAPM_START_CONNECTION_CMD as GAPM_START_CONNECTION_CMD,
        gapm_operation_GAPM_ADV_DIRECT as GAPM_ADV_DIRECT,
        gapm_operation_GAPM_ADV_DIRECT_LDC as GAPM_ADV_DIRECT_LDC,
        gapm_operation_GAPM_ADV_NON_CONN as GAPM_ADV_NON_CONN,
        gapm_operation_GAPM_ADV_UNDIRECT as GAPM_ADV_UNDIRECT,
        gapm_operation_GAPM_CONNECTION_DIRECT as GAPM_CONNECTION_DIRECT,
        gapm_operation_GAPM_CONNECTION_NAME_REQUEST as GAPM_CONNECTION_NAME_REQUEST,
        gapm_profile_task_add_cmd as GapmProfileTaskAdd,
        gapm_set_dev_config_cmd as GapmSetDevConfigCmd,
        gapm_start_advertise_cmd as GapmStartAdvertiseCmd,
        gapm_start_connection_cmd as GapmStartConnectionCmd,
    };

    use crate::platform::core_modules::ke::msg::kernel_msg_type;

    kernel_msg_type!(GapmProfileTaskAdd, GAPM_PROFILE_TASK_ADD_CMD);
    kernel_msg_type!(GapmStartAdvertiseCmd, GAPM_START_ADVERTISE_CMD);
    kernel_msg_type!(GapmStartConnectionCmd, GAPM_START_CONNECTION_CMD);
    kernel_msg_type!(GapmSetDevConfigCmd, GAPM_SET_DEV_CONFIG_CMD);
}
