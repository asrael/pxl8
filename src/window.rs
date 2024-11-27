use core::ffi::c_int;
use core::ptr::{self, NonNull};

use sdl3_sys::video::{SDL_CreateWindow, SDL_Window, SDL_WINDOW_RESIZABLE};

use crate::ffi::CString;
use crate::{Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct Window {
    raw: NonNull<SDL_Window>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let title = CString::new(title).unwrap();

        let raw = unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                width as c_int,
                height as c_int,
                SDL_WINDOW_RESIZABLE as u64,
            )
        };

        if raw != ptr::null_mut() {
            Ok(Self {
                raw: unsafe { NonNull::new_unchecked(raw) },
            })
        } else {
            Err(Error::from_sdl())
        }
    }

    pub fn as_ptr(&self) -> *mut SDL_Window {
        self.raw.as_ptr()
    }
}
