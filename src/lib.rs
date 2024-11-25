#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "expose_bindings")]
#[allow(dead_code)]
pub mod bindings;

#[cfg(not(feature = "expose_bindings"))]
mod bindings;

#[cfg(feature = "alloc")]
pub mod allocator;
#[cfg(not(feature = "no_ble"))]
pub mod app;
#[cfg(not(feature = "no_ble"))]
pub mod app_modules;
#[cfg(not(feature = "no_ble"))]
pub mod ble_stack;
pub mod platform;
pub mod stdlib;

pub use paste::paste;

#[no_mangle]
pub extern "C" fn GPIO_reservations() {}
