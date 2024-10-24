#![windows_subsystem = "console"]
#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};
use pxl8::*;

#[no_mangle]
extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    println!("hello, world...");
    pxl8::run(argc, argv);
    0
}
