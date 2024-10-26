#[macro_export]
macro_rules! load {
    ($game:ident { $($field:ident: $value:expr),* $(,)? }) => {
        use core::ffi::{c_char, c_int};

        #[no_mangle]
        unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
            let game_instance = $game {
                $(
                    $field: $value,
                )*
            };

            let result = $crate::ffi::SDL_EnterAppMainCallbacks(
                argc,
                argv as *mut *mut c_char,
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
