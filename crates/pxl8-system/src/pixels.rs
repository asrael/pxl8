use crate::stbi;
use core::ptr::NonNull;

#[derive(Clone, Copy, Debug)]
pub struct Pixels {
    buffer: NonNull<u8>,
    pub width: u32,
    pub height: u32,
    pub size: usize,
}

impl Pixels {
    pub fn new(memory: &[u8]) -> Self {
        let (buffer, width, height) = stbi::load_from_memory(memory);

        Self {
            buffer,
            width,
            height,
            size: (width * height * 4) as usize,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        let ptr = NonNull::slice_from_raw_parts(self.buffer, self.size);

        unsafe { ptr.as_ref() }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pixel(pub u32);

impl From<Pixel> for [f32; 4] {
    #[inline]
    fn from(pixel: Pixel) -> [f32; 4] {
        let c = pixel.0;
        let r = ((c >> 24) & 0xFF) as f32 / 255.0;
        let g = ((c >> 16) & 0xFF) as f32 / 255.0;
        let b = ((c >> 8) & 0xFF) as f32 / 255.0;
        let a = (c & 0xFF) as f32 / 255.0;

        return [r, g, b, a];
    }
}

impl From<[f32; 4]> for Pixel {
    #[inline]
    fn from(rgba: [f32; 4]) -> Pixel {
        let r = (rgba[0] * 255.0) as u32;
        let g = (rgba[0] * 255.0) as u32;
        let b = (rgba[0] * 255.0) as u32;
        let a = (rgba[0] * 255.0) as u32;

        Pixel((r << 24) | (g << 18) | (b << 8) | a)
    }
}
