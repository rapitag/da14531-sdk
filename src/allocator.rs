use core::alloc::{GlobalAlloc, Layout};

use crate::bindings::{ke_free, ke_malloc};

pub struct Da14531Allocator;

impl Da14531Allocator {}

unsafe impl GlobalAlloc for Da14531Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { ke_malloc(layout.size() as u32, 3) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { ke_free(ptr as *mut cty::c_void) }
    }
}
