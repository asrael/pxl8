#[macro_export]
macro_rules! run {
    ($game:ident { $($field:ident: $value:expr),* $(,)? }) => {
        use core::ffi::{c_char, c_int, c_void};
        use core::ptr::{self, NonNull};
        use $crate::alloc::boxed::Box;
        use $crate::alloc::ffi::CString;
        use $crate::sdl3_sys::events::{SDL_Event, SDL_EventType};
        use $crate::sdl3_sys::init::SDL_AppResult;
        use $crate::sdl3_sys::video::{SDL_CreateWindow, SDL_WINDOW_RESIZABLE};
        use $crate::sdl3_sys::main::SDL_EnterAppMainCallbacks;

        static mut GAME: $game = $game {
            $(
                $field: $value,
            )*
        };

        #[no_mangle]
        unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
            let result = SDL_EnterAppMainCallbacks(
                argc,
                argv,
                Some(app_init),
                Some(app_iterate),
                Some(app_event),
                Some(app_quit),
            );

            if result == 0 {
                libc::EXIT_SUCCESS
            } else {
                result
            }
        }

        unsafe extern "C" fn app_init(
            appstate: *mut *mut c_void,
            argc: c_int,
            argv: *mut *mut c_char,
        ) -> SDL_AppResult {
            let width = GAME.size.0 as c_int;
            let height = GAME.size.1 as c_int;
            let cstring = CString::new(GAME.title).unwrap();
            let title = cstring.as_c_str();
            let window =
                SDL_CreateWindow(title.as_ptr(), width, height, SDL_WINDOW_RESIZABLE as u64);
            if window != ptr::null_mut() {
                let ctx = Context {
                    window: NonNull::new_unchecked(window),
                };

                GAME.init(&ctx);

                let heap_ctx = Box::into_raw(Box::new(ctx));
                *appstate = heap_ctx as *mut c_void;

                SDL_AppResult::CONTINUE
            } else {
                SDL_AppResult::FAILURE
            }
        }

        unsafe extern "C" fn app_iterate(appstate: *mut c_void) -> SDL_AppResult {
            let ctx = &*(appstate as *mut Context);
            GAME.frame(&ctx);

            SDL_AppResult::CONTINUE
        }

        unsafe extern "C" fn app_event(
            appstate: *mut c_void,
            event: *mut SDL_Event,
        ) -> SDL_AppResult {
            let event_type = core::mem::transmute((*event).r#type);

            let ctx = &*(appstate as *const Context);
            GAME.event(&ctx);

            match event_type {
                SDL_EventType::QUIT => SDL_AppResult::SUCCESS,
                _ => SDL_AppResult::CONTINUE,
            }
        }

        unsafe extern "C" fn app_quit(appstate: *mut c_void, _result: SDL_AppResult) {
            let ctx = &*(appstate as *mut Context);
            GAME.quit(&ctx);

            let _ = Box::from_raw(appstate as *mut Context);
        }
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use $crate::alloc::format;
        use $crate::alloc::ffi::CString;
        use $crate::sdl3_sys::log::SDL_Log;

        let fmt = format!($($arg)*);
        let c_string = CString::new(fmt).unwrap();

        unsafe { SDL_Log(c_string.as_ptr()); }
    }}
}
