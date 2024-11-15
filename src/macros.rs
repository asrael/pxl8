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
        use $crate::{Event, Gpu, Key, KeyData};

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
            let title = CString::new(GAME.title).unwrap();
            let window = SDL_CreateWindow(
                title.as_ptr(),
                width, height,
                SDL_WINDOW_RESIZABLE as u64,
            );

            if window != ptr::null_mut() {
                let gpu = Gpu::new(window).expect("failed to create gpu");
                let pxl8 = Pxl8::new(gpu, NonNull::new_unchecked(window));

                GAME.init(&pxl8);

                *appstate = Box::into_raw(Box::new(pxl8)) as *mut c_void;

                SDL_AppResult::CONTINUE
            } else {
                SDL_AppResult::FAILURE
            }
        }

        unsafe extern "C" fn app_iterate(appstate: *mut c_void) -> SDL_AppResult {
            let pxl8 = &*(appstate as *mut Pxl8);
            GAME.frame(&pxl8);

            SDL_AppResult::CONTINUE
        }

        unsafe extern "C" fn app_event(
            appstate: *mut c_void,
            sdl_event: *mut SDL_Event,
        ) -> SDL_AppResult {
            let sdl_event = *sdl_event;
            let mut event = None;
            let event_type = core::mem::transmute(sdl_event.r#type);

            match event_type {
                SDL_EventType::KEY_DOWN => {
                    event = Some(Event::KeyDown(KeyData {
                        key: Key::from_scancode(sdl_event.key.scancode),
                        repeat: sdl_event.key.repeat,
                    }));

                }
                SDL_EventType::KEY_UP => {
                    event = Some(Event::KeyUp(KeyData {
                        key: Key::from_scancode(sdl_event.key.scancode),
                        repeat: false,
                    }));
                }

                _=> {}
            }

            if let Some(event) = event {
                let pxl8 = &*(appstate as *const Pxl8);
                GAME.event(&pxl8, event);
            }

            match event_type {
                SDL_EventType::QUIT => SDL_AppResult::SUCCESS,
                _ => SDL_AppResult::CONTINUE,
            }
        }

        unsafe extern "C" fn app_quit(appstate: *mut c_void, _result: SDL_AppResult) {
            let pxl8 = &*(appstate as *mut Pxl8);
            GAME.quit(&pxl8);

            let _ = Box::from_raw(appstate as *mut Pxl8);
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
