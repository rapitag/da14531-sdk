// const struct cust_prf_func_callbacks *custs_get_func_callbacks(enum KE_API_ID task_id)
// {
//     int i = 0;

use rtt_target::rprintln;

//     // max number of service characteristics
//     while (cust_prf_funcs[i].task_id != TASK_ID_INVALID)
//     {
//         if(cust_prf_funcs[i].task_id == task_id)
//             return &cust_prf_funcs[i];
//         else
//             i++;
//     }
//     return NULL;
// }
use crate::{
    app_modules::KeApiId,
    bindings::{cust_prf_func_callbacks as CustPrfFuncCallbacks, KE_API_ID_TASK_ID_INVALID},
};

extern "C" {
    static cust_prf_funcs: [CustPrfFuncCallbacks; 2];
}

#[no_mangle]
pub extern "C" fn custs_get_func_callbacks(task_id: KeApiId) -> *const CustPrfFuncCallbacks {
    rprintln!("Get custs func callbacks for: 0x{:02x}", task_id);
    for pfcb in unsafe { &cust_prf_funcs } {
        if pfcb.task_id == task_id {
            rprintln!("Found custs func callbacks for: 0x{:02x}", task_id);
            return pfcb as *const _ as *const CustPrfFuncCallbacks;
        } else if pfcb.task_id == KE_API_ID_TASK_ID_INVALID {
            break;
        }
    }

    core::ptr::null()
}

#[cfg(feature = "ble_custom_server1")]
pub mod custs1;

#[cfg(feature = "ble_custom_server2")]
pub mod custs2;
