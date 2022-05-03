pub fn ke_malloc<T>(mem_type: u32) -> *mut T {
    let len = core::mem::size_of::<T>();
    unsafe { crate::bindings::ke_malloc(len as u32, mem_type as u8) as *mut T }
}

pub fn ke_free<T>(ptr: *mut T) {
    unsafe { crate::bindings::ke_free(ptr as *mut cty::c_void) }
}
