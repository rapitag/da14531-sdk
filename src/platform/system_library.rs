#[inline]
pub fn patch_func() {
    unsafe {
        crate::bindings::patch_func();
    }
}
