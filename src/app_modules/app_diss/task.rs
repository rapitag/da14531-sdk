use crate::{
    app_modules::ProcessEventResponse,
    ble_stack::profiles::dis::diss::{
        task::{DissValueReqInd, KeMsgDynDissValueCfm, DISS_VALUE_REQ_IND},
        DIS_MANUFACTURER_NAME_CHAR, DIS_MODEL_NB_STR_CHAR, DIS_PNP_ID_CHAR, DIS_SW_REV_STR_CHAR,
        DIS_SYSTEM_ID_CHAR,
    },
    platform::core_modules::ke::{
        msg::{KeMsgHandler, KeMsgId, KeMsgStatusTag, KE_MSG_CONSUMED},
        task::KeTaskId,
    },
};

const APP_DIS_MANUFACTURER_NAME: [u8; 4] = *b"Test";
const APP_DIS_MODEL_NB_STR: [u8; 4] = *b"Test";
const APP_DIS_SYSTEM_ID: [u8; 4] = *b"Test";
const APP_DIS_PNP_ID: [u8; 4] = *b"Test";
const APP_DIS_HARD_REV_STR: [u8; 4] = *b"Test";
const APP_DIS_SERIAL_NB_STR: [u8; 4] = *b"Test";
const APP_DIS_FIRM_REV_STR: [u8; 4] = *b"Test";
const APP_DIS_SW_REV_STR: [u8; 4] = *b"Test";
const APP_DIS_IEEE: [u8; 4] = *b"Test";

#[no_mangle]
pub extern "C" fn diss_value_req_ind_handler(
    _msg_id: KeMsgId,
    param: *const cty::c_void,
    dest_id: KeTaskId,
    src_id: KeTaskId,
) -> i32 {
    let param = param as *const DissValueReqInd;
    let param = unsafe { &*param };
    match param.value as u32 {
        DIS_MANUFACTURER_NAME_CHAR => {
            const len: u16 = APP_DIS_MANUFACTURER_NAME.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_MANUFACTURER_NAME)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_MODEL_NB_STR_CHAR => {
            const len: u16 = APP_DIS_MODEL_NB_STR.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_MODEL_NB_STR)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_SYSTEM_ID_CHAR => {
            const len: u16 = APP_DIS_SYSTEM_ID.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_SYSTEM_ID)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_PNP_ID_CHAR => {
            const len: u16 = APP_DIS_PNP_ID.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_PNP_ID)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_SERIAL_NB_STR_CHAR => {
            const len: u16 = APP_DIS_HARD_REV_STR.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_HARD_REV_STR)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_HARD_REV_STR_CHAR => {
            const len: u16 = APP_DIS_SERIAL_NB_STR.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_SERIAL_NB_STR)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_SW_REV_STR_CHAR => {
            const len: u16 = APP_DIS_FIRM_REV_STR.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_FIRM_REV_STR)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_FIRM_REV_STR_CHAR => {
            const len: u16 = APP_DIS_SW_REV_STR.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_SW_REV_STR)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        DIS_IEEE_CHAR => {
            const len: u16 = APP_DIS_IEEE.len() as u16;
            let mut msg = KeMsgDynDissValueCfm::<len>::new(dest_id, src_id);
            unsafe {
                msg.fields()
                    .data
                    .as_mut_slice(len as usize)
                    .copy_from_slice(&APP_DIS_IEEE)
            };
            msg.fields().length = len as u8;

            msg.fields().value = param.value;

            msg.send();
        }
        _ => {
            let mut msg = KeMsgDynDissValueCfm::<0>::new(dest_id, src_id);
            msg.fields().length = 0;

            msg.fields().value = param.value;

            msg.send();
        }
    };

    KE_MSG_CONSUMED as i32
}

static APP_DISS_PROCESS_HANDLERS: [KeMsgHandler; 1] = [KeMsgHandler {
    id: DISS_VALUE_REQ_IND as u16,
    func: Some(diss_value_req_ind_handler),
}];

#[no_mangle]
pub extern "C" fn app_diss_process_handler(
    msg_id: KeMsgId,
    param: *const cty::c_void,
    dest_id: KeTaskId,
    src_id: KeTaskId,
    msg_ret: *mut KeMsgStatusTag,
) -> ProcessEventResponse {
    return unsafe {
        crate::bindings::app_std_process_event(
            msg_id,
            param,
            src_id,
            dest_id,
            msg_ret,
            APP_DISS_PROCESS_HANDLERS.as_ptr() as *mut _,
            APP_DISS_PROCESS_HANDLERS.len() as i32,
        )
    };
}
