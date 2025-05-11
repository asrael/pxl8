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
static SYSTEM_ALLOC: SDLAllocator = SDLAllocator;

pub struct SystemState<S> {
    palette: Pixels,
    user_state: Option<S>,
}

impl<S> Default for SystemState<S> {
    fn default() -> Self {
        Self {
            palette: Pixels::new(palettes::SWEAFT_64),
            user_state: None,
        }
    }
}

impl<S> SystemState<S> {
    pub fn new(user_state: S) -> Self {
        Self::default().with_user_state(user_state)
    }

    pub fn with_user_state(mut self, user_state: S) -> Self {
        self.user_state = Some(user_state);
        self
    }
}

pub fn init<S>(user_state: S) {
    let state = SystemState::<S>::new(user_state);
    let state_ptr = Box::into_raw(Box::new(state)) as *mut c_void;

    unsafe {
        if !SDL_Init(SDL_INIT_AUDIO | SDL_INIT_GAMEPAD | SDL_INIT_VIDEO) {
            panic!("SDL Init failed");
        }
    }

    info!("pxl8 running...");
    info!("state ptr: {state_ptr:?}");
}

#[cfg(not(test))]
mod lang_items {
    #[no_mangle]
    pub extern "C" fn rust_eh_personality() {}
}
