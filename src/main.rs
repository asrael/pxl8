#![windows_subsystem = "console"]
#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};
use pxl8::ffi::SDL_EnterAppMainCallbacks;

#[no_mangle]
unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    SDL_EnterAppMainCallbacks(
        argc,
        argv,
        Some(pxl8::init),
        Some(pxl8::update),
        Some(pxl8::event),
        Some(pxl8::quit),
    )
}
