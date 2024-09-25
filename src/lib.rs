#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_fn_floating_point_arithmetic)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "expose_bindings")]
pub mod bindings;

#[cfg(not(feature = "expose_bindings"))]
mod bindings;

#[cfg(not(feature="no_ble"))]
pub mod app;
#[cfg(not(feature="no_ble"))]
pub mod app_modules;
#[cfg(not(feature="no_ble"))]
pub mod ble_stack;
pub mod platform;
pub mod stdlib;
#[cfg(feature = "alloc")]
pub mod allocator;

pub use paste::paste;

#[no_mangle]
pub extern "C" fn GPIO_reservations() {}
