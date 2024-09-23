#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_fn_floating_point_arithmetic)]

extern crate alloc;

#[cfg(feature = "expose_bindings")]
pub mod bindings;

#[cfg(not(feature = "expose_bindings"))]
mod bindings;

pub mod allocator;
pub mod app;
pub mod app_modules;
pub mod ble_stack;
pub mod platform;
pub mod stdlib;

pub use paste::paste;

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "C" fn GPIO_reservations() {}
