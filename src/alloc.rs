extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use sdl3_sys::stdinc::{SDL_free, SDL_malloc};

pub use alloc::boxed::Box;
pub use alloc::ffi;
pub use alloc::format;

#[global_allocator]
static SDL_ALLOC: SDLAlloc = SDLAlloc;

#[derive(Default)]
struct SDLAlloc;

unsafe impl GlobalAlloc for SDLAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // printf(c"allocating %d bytes\n".as_ptr(), layout.size() as c_int);
        SDL_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // printf(c"freeing %d bytes\n".as_ptr(), layout.size() as c_int);
        SDL_free(ptr as *mut c_void);
    }
}
