#![windows_subsystem = "console"]
#![no_std]
#![no_main]

use core::ffi::{c_int, c_char};

#[no_mangle]
pub extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    pxl8::run(argc, argv);

    0
}
