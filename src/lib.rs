#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;

pub mod app;
pub mod app_modules;
pub mod ble_stack;
pub mod platform;
pub mod stdlib;

pub use paste::paste;

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "C" fn GPIO_reservations() {}
