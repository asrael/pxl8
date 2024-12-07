use crate::alloc::{Cow, String};

use core::convert::From;
use core::ffi::CStr;
use core::fmt;
use sdl3_sys::error::SDL_GetError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: Cow<'static, str>,
}

impl Error {
    pub fn new<T: AsRef<str> + ?Sized>(s: &'static T) -> Self {
        Self {
            msg: Cow::Borrowed(s.as_ref()),
        }
    }

    pub(crate) fn from_sdl() -> Self {
        let msg_cstr = unsafe { CStr::from_ptr(SDL_GetError()) };

        Self {
            msg: msg_cstr.to_string_lossy(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::new(s)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error { msg: Cow::Owned(s) }
    }
}
