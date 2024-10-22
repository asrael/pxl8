#![feature(core_intrinsics)]
#![no_std]

extern crate alloc;

pub mod ffi;

use ffi::{
    SDL_AppResult, SDL_CreateWindow, SDL_Event, SDL_EventType, SDL_LogError,
    SDL_LogInfo, SDL_Window, SDL_free, SDL_malloc, SDL_WINDOW_RESIZABLE,
};

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::ffi::CString;
use core::ffi::{c_char, c_int, c_void};
use core::intrinsics;
use core::panic::PanicInfo;
use core::ptr::{self, NonNull};

#[derive(Default)]
pub struct SDLAlloc;

unsafe impl GlobalAlloc for SDLAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        SDL_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        SDL_free(ptr as *mut c_void);
    }
}

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[repr(C)]
pub struct Context {
    window: NonNull<SDL_Window>,
}

#[inline]
pub unsafe extern "C" fn init(
    appstate: *mut *mut c_void,
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
pub unsafe extern "C" fn update(_appstate: *mut c_void) -> SDL_AppResult {
    SDL_AppResult::SDL_APP_CONTINUE
}

#[inline]
pub unsafe extern "C" fn event(
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
pub unsafe extern "C" fn quit(_appstate: *mut c_void, result: SDL_AppResult) {}

impl core::convert::From<u32> for SDL_EventType {
    fn from(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort()
}
