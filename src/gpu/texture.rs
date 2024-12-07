use core::ptr::{self, NonNull};

use sdl3_sys::gpu::{
    SDL_CreateGPUTexture, SDL_GPUTexture, SDL_GPUTextureCreateInfo,
    SDL_GPUTextureFormat, SDL_GPU_SAMPLECOUNT_1, SDL_GPU_TEXTURETYPE_2D,
    SDL_GPU_TEXTUREUSAGE_SAMPLER,
};

use crate::{Error, Gpu, Result};

#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub(crate) raw: NonNull<SDL_GPUTexture>,

    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(width: u32, height: u32, gpu: &Gpu) -> Result<Self> {
        let raw = unsafe {
            let info = SDL_GPUTextureCreateInfo {
                r#type: SDL_GPU_TEXTURETYPE_2D,
                format: SDL_GPUTextureFormat::R8G8B8A8_UNORM_SRGB,
                usage: SDL_GPU_TEXTUREUSAGE_SAMPLER,
                width,
                height,
                layer_count_or_depth: 1,
                num_levels: 1,
                sample_count: SDL_GPU_SAMPLECOUNT_1,
                props: 0,
            };

            SDL_CreateGPUTexture(gpu.device.as_ptr(), &info)
        };

        if raw != ptr::null_mut() {
            Ok(Texture {
                raw: unsafe { NonNull::new_unchecked(raw) },
                width,
                height,
            })
        } else {
            Err(Error::from_sdl())
        }
    }
}
