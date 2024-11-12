use core::ptr::{self, NonNull};

use sdl3_sys::gpu::*;
use sdl3_sys::video::SDL_Window;

use crate::{get_sdl_error, Result};

#[derive(Clone, Copy, Debug)]
pub struct Gpu {
    device: NonNull<SDL_GPUDevice>,
}

impl Gpu {
    pub unsafe fn new(window: *mut SDL_Window) -> Result<Self> {
        let device =
            SDL_CreateGPUDevice(SDL_GPU_SHADERFORMAT_SPIRV, true, ptr::null());

        if device != ptr::null_mut() && SDL_ClaimWindowForGPUDevice(device, window) {
            Ok(Self {
                device: NonNull::new_unchecked(device),
            })
        } else {
            Err(get_sdl_error())
        }
    }
}
