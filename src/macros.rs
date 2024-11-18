#[macro_export]
macro_rules! run {
    ($game:ident { $($field:ident: $value:expr),* $(,)? }) => {
        use core::ffi::{c_char, c_int, c_void};
        use core::ptr;

        use $crate::sdl3_sys::events::{SDL_Event, SDL_EventType};
        use $crate::sdl3_sys::init::SDL_AppResult;
        use $crate::sdl3_sys::main::SDL_EnterAppMainCallbacks;
        use $crate::{Box, eprintln, Pxl8};

        #[no_mangle]
        unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
            SDL_EnterAppMainCallbacks(
                argc,
                argv,
                Some(app_init),
                Some(app_iterate),
                Some(app_event),
                Some(app_quit),
            )
        }

        unsafe extern "C" fn app_init(
            appstate: *mut *mut c_void,
            argc: c_int,
            argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            let game = $game {
                $(
                    $field: $value,
                )*
            };

            match Pxl8::new(game) {
                Ok(pxl8) => {
                    pxl8.init();
                    *appstate = Box::into_raw(Box::new(pxl8)) as *mut c_void;
                    SDL_AppResult::CONTINUE
                }

                Err(err) => {
                    eprintln!("{err}");
                    SDL_AppResult::FAILURE
                }
            }
        }

        unsafe extern "C" fn app_iterate(appstate: *mut c_void) -> SDL_AppResult {
            let pxl8 = &*(appstate as *const Pxl8<$game>);

            pxl8.frame();

            SDL_AppResult::CONTINUE
        }

        unsafe extern "C" fn app_event(
            appstate: *mut c_void,
            sdl_event: *mut SDL_Event,
        ) -> SDL_AppResult {
            let pxl8 = &*(appstate as *const Pxl8<$game>);
            let sdl_event = *sdl_event;
            let sdl_event_type = core::mem::transmute(sdl_event.r#type);

            pxl8.event(sdl_event_type, sdl_event);

            match sdl_event_type {
                SDL_EventType::QUIT => SDL_AppResult::SUCCESS,
                _ => SDL_AppResult::CONTINUE,
            }
        }

        unsafe extern "C" fn app_quit(appstate: *mut c_void, _result: SDL_AppResult) {
            let pxl8 = &*(appstate as *const Pxl8<$game>);
            let _ = Box::from_raw(appstate as *mut Pxl8<$game>);

            pxl8.quit();
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use $crate::format;
        use $crate::ffi::CString;
        use $crate::sdl3_sys::log::SDL_Log;

        let fmt = format!($($arg)*);
        let c_string = CString::new(fmt).unwrap();

        unsafe { SDL_Log(c_string.as_ptr()); }
    }}
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {{
        use $crate::format;
        use $crate::ffi::CString;
        use $crate::sdl3_sys::log::{SDL_Log, SDL_LogError, SDL_LogCategory};

        let fmt = format!($($arg)*);
        let c_string = CString::new(fmt).unwrap();

        unsafe {
            let category = core::mem::transmute(SDL_LogCategory::ERROR);
            SDL_LogError(category, c_string.as_ptr());
        }
    }}
}
