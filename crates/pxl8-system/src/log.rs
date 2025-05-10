#![allow(unused_macros)]

mod ffi {
    extern "C" {
        pub fn log_log(
            level: core::ffi::c_int,
            file: *const core::ffi::c_char,
            line: core::ffi::c_int,
            fmt: *const core::ffi::c_char,
            ...
        );
    }
}

use alloc::ffi::CString;

#[repr(i32)]
pub enum LogLevel {
    Trace = 0,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[inline]
pub fn log_log(level: LogLevel, message: &str, file: &str, line: u32) {
    let c_msg =
        CString::new(message).expect("failed to create log msg CString");
    let c_file =
        CString::new(file).expect("failed to create file name CString");

    unsafe {
        ffi::log_log(
            level as i32,
            c_file.as_ptr(),
            line as i32,
            c_msg.as_ptr(),
        );
    }
}

macro_rules! trace {
    ($($arg:tt)*) => {{
        log_log(LogLevel::Trace, &format!($($arg)*), file!(), line!());
    }};
}
macro_rules! debug {
    ($($arg:tt)*) => {{
        log_log(LogLevel::Debug, &format!($($arg)*), file!(), line!());
    }};
}
macro_rules! info {
    ($($arg:tt)*) => {{
        log_log(LogLevel::Info, &format!($($arg)*), file!(), line!());
    }};
}
macro_rules! warn {
    ($($arg:tt)*) => {{
        log_log(LogLevel::Warn, &format!($($arg)*), file!(), line!());
    }};
}
macro_rules! error {
    ($($arg:tt)*) => {{
        log_log(LogLevel::Error, &format!($($arg)*), file!(), line!());
    }};
}
macro_rules! fatal {
    ($($arg:tt)*) => {{
        log_log(LogLevel::Fatal, &format!($($arg)*), file!(), line!());
    }};
}
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        log_log($level, &format!($($arg)*), file!(), line!());
    }};
}
