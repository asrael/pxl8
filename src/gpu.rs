mod shader;
mod texture;

use core::ptr::{self, NonNull};

use sdl3_sys::gpu::*;

use crate::{Error, Result, Window};

pub use texture::Texture;

#[derive(Debug)]
pub struct Gpu {
    device: NonNull<SDL_GPUDevice>,
}

impl Gpu {
    pub fn new(window: &Window) -> Result<Self> {
        let device = unsafe {
            SDL_CreateGPUDevice(SDL_GPU_SHADERFORMAT_SPIRV, true, ptr::null())
        };

        if device != ptr::null_mut()
            && unsafe { SDL_ClaimWindowForGPUDevice(device, window.as_ptr()) }
        {
            Ok(Self {
                device: unsafe { NonNull::new_unchecked(device) },
            })
        } else {
            Err(Error::from_sdl())
        }
    }

    pub fn device_ptr(&self) -> *mut SDL_GPUDevice {
        self.device.as_ptr()
    }
}
