mod shader;
mod texture;

use crate::{Error, Result, Window};

use core::ptr::{self, NonNull};
use sdl3_sys::gpu::*;

pub use texture::Texture;

#[derive(Debug)]
pub struct Gpu {
    pub(crate) device: NonNull<SDL_GPUDevice>,
}

impl Gpu {
    pub fn new(window: &Window) -> Result<Self> {
        let device = unsafe {
            SDL_CreateGPUDevice(SDL_GPU_SHADERFORMAT_SPIRV, true, ptr::null())
        };

        if device != ptr::null_mut()
            && unsafe { SDL_ClaimWindowForGPUDevice(device, window.as_raw()) }
        {
            Ok(Self {
                device: unsafe { NonNull::new_unchecked(device) },
            })
        } else {
            Err(Error::from_sdl())
        }
    }
}
