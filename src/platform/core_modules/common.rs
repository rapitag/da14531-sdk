pub use crate::bindings::{
    adv_channel_map_ADV_ALL_CHNLS_EN as ADV_ALL_CHNLS_EN,
    adv_channel_map_ADV_CHNL_37_EN as ADV_CHNL_37_EN,
    adv_channel_map_ADV_CHNL_38_EN as ADV_CHNL_38_EN,
    adv_channel_map_ADV_CHNL_39_EN as ADV_CHNL_39_EN,
    adv_filter_policy_ADV_ALLOW_SCAN_ANY_CON_ANY as ADV_ALLOW_SCAN_ANY_CON_ANY,
    adv_filter_policy_ADV_ALLOW_SCAN_ANY_CON_WLST as ADV_ALLOW_SCAN_ANY_CON_WLST,
    bd_addr as BDAddr, co_list as CoList, co_list_hdr as CoListHdr,
    hci_le_rx_test_cmd as HciLeRxTestCmd, hci_le_tx_test_cmd as HciLeTxTestCmd, ADV_DATA_LEN,
    KEY_LEN, SCAN_RSP_DATA_LEN,
};

#[inline]
pub fn co_list_init(list: &mut CoList) {
    unsafe {
        crate::bindings::co_list_init(list as *mut _);
    }
}

#[inline]
pub fn co_list_is_empty(list: &mut CoList) -> bool {
    list.first == core::ptr::null_mut()
}

#[inline]
pub fn co_list_pop_front(list: &mut CoList) -> &mut CoListHdr {
    unsafe {
        crate::bindings::co_list_pop_front(list as *mut _)
            .as_mut()
            .unwrap()
    }
}

#[inline]
pub fn co_min<T: core::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
