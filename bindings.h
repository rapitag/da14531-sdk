#include "stdlib.h"

// app
#include "da1458x_config_basic.h"
#include "da1458x_config_advanced.h"
#include "user_config.h"

// app_modules
#include "app.h"
#include "app_customs.h"
#include "app_easy_gap.h"
// #include "app_easy_timer.h"
#include "app_entry_point.h"
#include "app_prf_types.h"

// ble_stack
#include "custs1_task.h"
#include "gap.h"
#include "gapc_task.h"
#include "attm_db_128.h"

// core_modules
#include "ke_msg.h"
#include "rwip_config.h"

// platform
#include "syscntl.h"
#include "system_library.h"
#include "trng.h"

const uint8_t APP_MODULES_TIMER_MAX_NUM = (APP_TIMER_API_LAST_MES - APP_TIMER_API_MES0 + 1);
const uint16_t APP_MODULES_TIMER_API_MES0 = APP_TIMER_API_MES0;
const uint16_t APP_MODULES_TIMER_API_LAST_MES = APP_TIMER_API_LAST_MES;
const uint32_t MSG_APP_CREATE_TIMER = APP_CREATE_TIMER;
const uint32_t MSG_APP_MODIFY_TIMER = APP_MODIFY_TIMER;
const uint32_t MSG_APP_CANCEL_TIMER = APP_CANCEL_TIMER;

const uint16_t CORE_MODULES_RWIP_TASK_APP = TASK_APP;