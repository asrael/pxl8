#![allow(internal_features)]
#![feature(lang_items)]
#![no_std]

pub extern crate alloc;

pub mod ffi;
mod macros;

use ffi::{
    SDL_AppResult, SDL_CreateWindow, SDL_EnterAppMainCallbacks, SDL_Event,
    SDL_EventType, SDL_Window, SDL_free, SDL_malloc, SDL_WINDOW_RESIZABLE,
};

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::ffi::CString;
use core::ffi::{c_char, c_int, c_void};
use core::panic::PanicInfo;
use core::ptr::{self, NonNull};
use libc::printf;

#[derive(Default)]
pub(crate) struct SDLAlloc;

unsafe impl GlobalAlloc for SDLAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        printf(
            b"allocating %d bytes\n\0".as_ptr() as *const c_char,
            layout.size() as c_int,
        );

        SDL_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        SDL_free(ptr as *mut c_void);
    }
}

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[repr(C)]
pub struct Context {
    window: NonNull<SDL_Window>,
}

pub fn run(argc: c_int, argv: *mut *mut c_char) {
    unsafe {
        SDL_EnterAppMainCallbacks(
            argc,
            argv as *mut *mut c_char,
            Some(init),
            Some(update),
            Some(event),
            Some(quit),
        );
    }
}

#[inline]
unsafe extern "C" fn init(
    _appstate: *mut *mut c_void,
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> SDL_AppResult {
    let title = CString::new("pxl8").unwrap();
    let window =
        SDL_CreateWindow(title.as_ptr(), 1280, 720, SDL_WINDOW_RESIZABLE as u64);

    if window != ptr::null_mut() {
        let _ctx = Context {
            window: NonNull::new_unchecked(window),
        };

        SDL_AppResult::SDL_APP_CONTINUE
    } else {
        SDL_AppResult::SDL_APP_FAILURE
    }
}

#[inline]
unsafe extern "C" fn update(_appstate: *mut c_void) -> SDL_AppResult {
    SDL_AppResult::SDL_APP_CONTINUE
}

#[inline]
unsafe extern "C" fn event(
    _app_state: *mut c_void,
    event: *mut SDL_Event,
) -> SDL_AppResult {
    let event_type = (*event).type_.into();

    match event_type {
        SDL_EventType::SDL_EVENT_QUIT => SDL_AppResult::SDL_APP_SUCCESS,
        _ => SDL_AppResult::SDL_APP_CONTINUE,
    }
}

#[inline]
unsafe extern "C" fn quit(_appstate: *mut c_void, _result: SDL_AppResult) {}

impl core::convert::From<u32> for SDL_EventType {
    fn from(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
