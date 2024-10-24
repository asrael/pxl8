#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let fmt = $crate::alloc::format!($($arg)*);
        let c_string = $crate::alloc::ffi::CString::new(fmt).unwrap();

        unsafe { $crate::ffi::SDL_Log(c_string.as_ptr()); }
    }}
}
