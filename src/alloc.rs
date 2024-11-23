extern crate alloc;

#[allow(unused_imports)]
use core::ffi::{c_int, c_void};

use alloc::alloc::{GlobalAlloc, Layout};
use sdl3_sys::stdinc::{SDL_free, SDL_malloc};

pub use alloc::borrow::*;
pub use alloc::boxed::Box;
pub use alloc::ffi;
pub use alloc::format;
pub use alloc::string::String;
pub use alloc::vec::Vec;

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[derive(Default)]
struct SDLAlloc;

unsafe impl GlobalAlloc for SDLAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // libc::printf(c"allocating %d bytes\n".as_ptr(), layout.size() as c_int);

        SDL_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // libc::printf(c"freeing %d bytes\n".as_ptr(), layout.size() as c_int);

        SDL_free(ptr as *mut c_void);
    }
}
