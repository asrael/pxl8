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
use alloc::boxed::Box;
use alloc::ffi::CString;
use core::ffi::{c_char, c_int, c_void, CStr};
use core::panic::PanicInfo;
use core::ptr::{self, NonNull};
use core::slice;
use libc::printf;

#[derive(Default)]
pub(crate) struct SDLAlloc;

unsafe impl GlobalAlloc for SDLAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size() as c_int;

        printf(c"allocating %d bytes\n".as_ptr(), size);

        SDL_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size() as c_int;

        printf(c"freeing %d bytes\n".as_ptr(), size);

        SDL_free(ptr as *mut c_void);
    }
}

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[repr(C)]
#[derive(Debug)]
pub struct Context {
    window: NonNull<SDL_Window>,
}

pub trait Game {
    fn size(&self) -> (u32, u32);
    fn title(&self) -> &str;

    fn init(&mut self, ctx: Context);
    fn event(&mut self, ctx: Context);
    fn frame(&mut self, ctx: Context);
    fn quit(&mut self, ctx: Context);
}

#[inline]
pub unsafe extern "C" fn init(
    appstate: *mut *mut c_void,
    argc: c_int,
    argv: *mut *mut c_char,
) -> SDL_AppResult {
    let window = SDL_CreateWindow(*argv, 1280, 720, SDL_WINDOW_RESIZABLE as u64);

    let argc = argc as usize;
    let args: &[*mut c_char] = slice::from_raw_parts(argv.cast_const(), argc);

    println!("{:?}", CStr::from_ptr(args[argc - 1]));

    if window != ptr::null_mut() {
        let ctx = Box::into_raw(Box::new(Context {
            window: NonNull::new_unchecked(window),
        }));

        *appstate = ctx as *mut c_void;

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
    _appstate: *mut c_void,
    event: *mut SDL_Event,
) -> SDL_AppResult {
    let event_type = (*event).type_.into();

    match event_type {
        SDL_EventType::SDL_EVENT_QUIT => SDL_AppResult::SDL_APP_SUCCESS,
        _ => SDL_AppResult::SDL_APP_CONTINUE,
    }
}

#[inline]
pub unsafe extern "C" fn quit(appstate: *mut c_void, _result: SDL_AppResult) {
    let _ = Box::from_raw(appstate as *mut Context);
}

impl core::convert::From<ffi::Uint32> for SDL_EventType {
    fn from(value: ffi::Uint32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
