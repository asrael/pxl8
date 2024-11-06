#![allow(internal_features)]
#![feature(lang_items)]
#![no_std]

pub extern crate alloc;

pub mod ffi;
mod macros;

use ffi::{SDL_EventType, SDL_Window, SDL_free, SDL_malloc};

use alloc::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_char, c_int, c_void, CStr};
use core::panic::PanicInfo;
use core::ptr::NonNull;
use libc::printf;

#[derive(Default)]
pub(crate) struct SDLAlloc;

unsafe impl GlobalAlloc for SDLAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // printf(c"allocating %d bytes\n".as_ptr(), layout.size() as c_int);
        SDL_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // printf(c"freeing %d bytes\n".as_ptr(), layout.size() as c_int);
        SDL_free(ptr as *mut c_void);
    }
}

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[derive(Debug)]
pub struct Context {
    pub window: NonNull<SDL_Window>,
}

pub trait Game {
    fn init(&mut self, ctx: &Context);
    fn event(&mut self, ctx: &Context);
    fn frame(&mut self, ctx: &Context);
    fn quit(&mut self, ctx: &Context);
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
