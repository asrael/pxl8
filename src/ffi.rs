#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(any(target_os = "linux", target_arch = "wasm32"))]
mod linux;

#[cfg(any(target_os = "linux", target_arch = "wasm32"))]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;
