use core::ffi::CStr;

use sdl3_sys::error::SDL_GetError;

use crate::Box;

pub type Error = Box<dyn core::error::Error>;

pub unsafe fn get_sdl_error() -> Error {
    CStr::from_ptr(SDL_GetError()).to_string_lossy().into()
}
