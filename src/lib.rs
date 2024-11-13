#![allow(internal_features)]
#![feature(lang_items)]
#![no_std]

mod error;
//mod event;
mod gpu;
mod macros;
mod result;

use alloc::alloc::{GlobalAlloc, Layout};

use core::ffi::c_int;
use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr::NonNull;

use libc::printf;
use sdl3_sys::stdinc::{SDL_free, SDL_malloc};
use sdl3_sys::video::SDL_Window;

pub(crate) use error::get_sdl_error;

pub type Window = NonNull<SDL_Window>;

pub extern crate alloc;
pub use alloc::boxed::Box;
pub use sdl3_sys;

pub use error::Error;
pub use gpu::Gpu;
pub use result::Result;

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[derive(Default)]
struct SDLAlloc;

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

#[derive(Debug)]
pub struct Pxl8 {
    pub gpu: Gpu,
    window: Window,
}

impl Pxl8 {
    pub fn new(gpu: Gpu, window: Window) -> Self {
        Pxl8 { gpu, window }
    }
}

pub trait Game {
    fn init(&mut self, pxl8: &Pxl8);
    fn event(&mut self, pxl8: &Pxl8);
    fn frame(&mut self, pxl8: &Pxl8);
    fn quit(&mut self, pxl8: &Pxl8);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
