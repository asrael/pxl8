#[macro_export]
macro_rules! load {
    ($path:literal) => {
        use core::ffi::{c_char, c_int, CStr};
        use core::slice;
        use $crate::alloc::vec::Vec;

        const CONFIG: &'static CStr = {
            let c_str = concat!(include_str!($path), "\0");
            unsafe { CStr::from_ptr(c_str.as_ptr().cast::<c_char>()) }
        };

        #[no_mangle]
        unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
            let base_args: &[*mut c_char] =
                slice::from_raw_parts(argv.cast_const(), argc as usize);

            let mut args: Vec<*mut c_char> = base_args.to_vec();

            args.push(CONFIG.as_ptr().cast_mut());

            let result = $crate::ffi::SDL_EnterAppMainCallbacks(
                argc + 1,
                args.as_mut_ptr(),
                Some($crate::init),
                Some($crate::update),
                Some($crate::event),
                Some($crate::quit),
            );

            if result == 0 {
                libc::EXIT_SUCCESS
            } else {
                result
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let fmt = $crate::alloc::format!($($arg)*);
        let c_string = $crate::alloc::ffi::CString::new(fmt).unwrap();

        unsafe { $crate::ffi::SDL_Log(c_string.as_ptr()); }
    }}
}
