#![no_std]

extern crate alloc;
extern crate pxl8_panic;

mod event;
#[macro_use]
mod log;

mod pixels;
mod res;
mod shaders;
mod sprite;
mod stbi;
mod ui;
mod vm;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::boxed::Box;
use alloc::format;
use res::palettes;
//use res::sprites;

use sdl3_sys::init::{
    SDL_Init, SDL_INIT_AUDIO, SDL_INIT_GAMEPAD, SDL_INIT_VIDEO,
};
use sdl3_sys::stdinc::{SDL_free, SDL_malloc};

pub use event::*;
pub use log::*;
pub use pixels::{Pixel, Pixels};
pub use sprite::Sprite;

pub use sdl3_main::main;

use core::ffi::c_void;

struct SDLAllocator;

unsafe impl GlobalAlloc for SDLAllocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        SDL_malloc(layout.size()) as *mut u8
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        SDL_free(ptr as *mut c_void);
    }
}

#[global_allocator]
static SDL_ALLOC: SDLAllocator = SDLAllocator;

pub struct Context {
    palette: Pixels,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            palette: Pixels::from_encoded(palettes::SWEAFT_64),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn run() {
    let user_data = Box::into_raw(Box::new(Context::new())) as *mut c_void;

    unsafe {
        if !SDL_Init(SDL_INIT_AUDIO | SDL_INIT_GAMEPAD | SDL_INIT_VIDEO) {
            panic!("SDL Init failed");
        }
    }

    info!("pxl8 running... {:p}", user_data);
}

#[cfg(not(test))]
mod lang_items {
    #[no_mangle]
    pub extern "C" fn rust_eh_personality() {}
}
