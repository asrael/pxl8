use core::ffi::c_int;
use core::ptr::NonNull;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod stb_image;

use stb_image as ffi;

pub fn load_from_memory(buffer: &[u8]) -> (NonNull<u8>, u32, u32) {
    let mut width = 0;
    let mut height = 0;
    let mut _channels = 0;
    let ptr = unsafe {
        ffi::stbi_load_from_memory(
            buffer.as_ptr(),
            buffer.len() as c_int,
            &mut width,
            &mut height,
            &mut _channels,
            4,
        )
    };

    assert!(!ptr.is_null());
    assert!((width * height * 4) > 0);

    unsafe { (NonNull::new_unchecked(ptr), width as u32, height as u32) }
}
