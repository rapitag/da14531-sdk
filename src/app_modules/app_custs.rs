use rtt_target::rprintln;

pub use crate::bindings::{cust_prf_func_callbacks as CustPrfFuncCallbacks, prf_func_callbacks as PrfFuncCallbacks};

use crate::{app_modules::KeApiId, platform::core_modules::rwip::TASK_ID_INVALID};

#[cfg(feature = "ble_custom_server")]
extern "C" {
    pub static cust_prf_funcs: [CustPrfFuncCallbacks; 2];
}

#[no_mangle]
pub extern "C" fn custs_get_func_callbacks(task_id: KeApiId) -> *const CustPrfFuncCallbacks {
    rprintln!("Get custs func callbacks for: 0x{:02x}", task_id);
    for pfcb in unsafe { &cust_prf_funcs } {
        if pfcb.task_id == task_id {
            rprintln!("Found custs func callbacks for: 0x{:02x}", task_id);
            return pfcb as *const _ as *const CustPrfFuncCallbacks;
        } else if pfcb.task_id == TASK_ID_INVALID {
            break;
        }
    }

    core::ptr::null()
}

#[cfg(feature = "ble_custom_server1")]
pub mod custs1;

#[cfg(feature = "ble_custom_server2")]
pub mod custs2;
