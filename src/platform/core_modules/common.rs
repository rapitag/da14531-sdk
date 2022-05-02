pub use crate::bindings::{
    co_list as CoList, co_list_hdr as CoListHdr, ADV_DATA_LEN, SCAN_RSP_DATA_LEN,
};

pub fn co_list_init(list: &mut CoList) {
    unsafe {
        crate::bindings::co_list_init(list as *mut _);
    }
}

pub fn co_list_is_empty(list: &mut CoList) -> bool {
    list.first == core::ptr::null_mut()
}

pub fn co_list_pop_front(list: &mut CoList) -> &mut CoListHdr {
    unsafe {
        crate::bindings::co_list_pop_front(list as *mut _)
            .as_mut()
            .unwrap()
    }
}
