pub fn rand() -> u32 {
    unsafe {
        crate::bindings::rand() as u32
    }
}