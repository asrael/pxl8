pub type __int64_t = ::core::ffi::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __sbuf"][::core::mem::size_of::<__sbuf>() - 16usize];
    ["Alignment of __sbuf"][::core::mem::align_of::<__sbuf>() - 8usize];
    ["Offset of field: __sbuf::_base"]
        [::core::mem::offset_of!(__sbuf, _base) - 0usize];
    ["Offset of field: __sbuf::_size"]
        [::core::mem::offset_of!(__sbuf, _size) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub _read: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: *mut ::core::ffi::c_char,
            __n: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: fpos_t,
            arg3: ::core::ffi::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::core::ffi::c_void,
            arg2: *const ::core::ffi::c_char,
            __n: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3usize],
    pub _nbuf: [::core::ffi::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __sFILE"][::core::mem::size_of::<__sFILE>() - 152usize];
    ["Alignment of __sFILE"][::core::mem::align_of::<__sFILE>() - 8usize];
    ["Offset of field: __sFILE::_p"]
        [::core::mem::offset_of!(__sFILE, _p) - 0usize];
    ["Offset of field: __sFILE::_r"]
        [::core::mem::offset_of!(__sFILE, _r) - 8usize];
    ["Offset of field: __sFILE::_w"]
        [::core::mem::offset_of!(__sFILE, _w) - 12usize];
    ["Offset of field: __sFILE::_flags"]
        [::core::mem::offset_of!(__sFILE, _flags) - 16usize];
    ["Offset of field: __sFILE::_file"]
        [::core::mem::offset_of!(__sFILE, _file) - 18usize];
    ["Offset of field: __sFILE::_bf"]
        [::core::mem::offset_of!(__sFILE, _bf) - 24usize];
    ["Offset of field: __sFILE::_lbfsize"]
        [::core::mem::offset_of!(__sFILE, _lbfsize) - 40usize];
    ["Offset of field: __sFILE::_cookie"]
        [::core::mem::offset_of!(__sFILE, _cookie) - 48usize];
    ["Offset of field: __sFILE::_close"]
        [::core::mem::offset_of!(__sFILE, _close) - 56usize];
    ["Offset of field: __sFILE::_read"]
        [::core::mem::offset_of!(__sFILE, _read) - 64usize];
    ["Offset of field: __sFILE::_seek"]
        [::core::mem::offset_of!(__sFILE, _seek) - 72usize];
    ["Offset of field: __sFILE::_write"]
        [::core::mem::offset_of!(__sFILE, _write) - 80usize];
    ["Offset of field: __sFILE::_ub"]
        [::core::mem::offset_of!(__sFILE, _ub) - 88usize];
    ["Offset of field: __sFILE::_extra"]
        [::core::mem::offset_of!(__sFILE, _extra) - 104usize];
    ["Offset of field: __sFILE::_ur"]
        [::core::mem::offset_of!(__sFILE, _ur) - 112usize];
    ["Offset of field: __sFILE::_ubuf"]
        [::core::mem::offset_of!(__sFILE, _ubuf) - 116usize];
    ["Offset of field: __sFILE::_nbuf"]
        [::core::mem::offset_of!(__sFILE, _nbuf) - 119usize];
    ["Offset of field: __sFILE::_lb"]
        [::core::mem::offset_of!(__sFILE, _lb) - 120usize];
    ["Offset of field: __sFILE::_blksize"]
        [::core::mem::offset_of!(__sFILE, _blksize) - 136usize];
    ["Offset of field: __sFILE::_offset"]
        [::core::mem::offset_of!(__sFILE, _offset) - 144usize];
};
pub type FILE = __sFILE;
pub type stbi_uc = ::core::ffi::c_uchar;
pub type stbi_us = ::core::ffi::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stbi_io_callbacks {
    pub read: ::core::option::Option<
        unsafe extern "C" fn(
            user: *mut ::core::ffi::c_void,
            data: *mut ::core::ffi::c_char,
            size: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub skip: ::core::option::Option<
        unsafe extern "C" fn(
            user: *mut ::core::ffi::c_void,
            n: ::core::ffi::c_int,
        ),
    >,
    pub eof: ::core::option::Option<
        unsafe extern "C" fn(
            user: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of stbi_io_callbacks"]
        [::core::mem::size_of::<stbi_io_callbacks>() - 24usize];
    ["Alignment of stbi_io_callbacks"]
        [::core::mem::align_of::<stbi_io_callbacks>() - 8usize];
    ["Offset of field: stbi_io_callbacks::read"]
        [::core::mem::offset_of!(stbi_io_callbacks, read) - 0usize];
    ["Offset of field: stbi_io_callbacks::skip"]
        [::core::mem::offset_of!(stbi_io_callbacks, skip) - 8usize];
    ["Offset of field: stbi_io_callbacks::eof"]
        [::core::mem::offset_of!(stbi_io_callbacks, eof) - 16usize];
};
unsafe extern "C" {
    pub fn stbi_load_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_uc;
}
unsafe extern "C" {
    pub fn stbi_load_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::core::ffi::c_void,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_uc;
}
unsafe extern "C" {
    pub fn stbi_load(
        filename: *const ::core::ffi::c_char,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_uc;
}
unsafe extern "C" {
    pub fn stbi_load_from_file(
        f: *mut FILE,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_uc;
}
unsafe extern "C" {
    pub fn stbi_load_gif_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
        delays: *mut *mut ::core::ffi::c_int,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        z: *mut ::core::ffi::c_int,
        comp: *mut ::core::ffi::c_int,
        req_comp: ::core::ffi::c_int,
    ) -> *mut stbi_uc;
}
unsafe extern "C" {
    pub fn stbi_load_16_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_us;
}
unsafe extern "C" {
    pub fn stbi_load_16_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::core::ffi::c_void,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_us;
}
unsafe extern "C" {
    pub fn stbi_load_16(
        filename: *const ::core::ffi::c_char,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_us;
}
unsafe extern "C" {
    pub fn stbi_load_from_file_16(
        f: *mut FILE,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut stbi_us;
}
unsafe extern "C" {
    pub fn stbi_loadf_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut f32;
}
unsafe extern "C" {
    pub fn stbi_loadf_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::core::ffi::c_void,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut f32;
}
unsafe extern "C" {
    pub fn stbi_loadf(
        filename: *const ::core::ffi::c_char,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut f32;
}
unsafe extern "C" {
    pub fn stbi_loadf_from_file(
        f: *mut FILE,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        channels_in_file: *mut ::core::ffi::c_int,
        desired_channels: ::core::ffi::c_int,
    ) -> *mut f32;
}
unsafe extern "C" {
    pub fn stbi_hdr_to_ldr_gamma(gamma: f32);
}
unsafe extern "C" {
    pub fn stbi_hdr_to_ldr_scale(scale: f32);
}
unsafe extern "C" {
    pub fn stbi_ldr_to_hdr_gamma(gamma: f32);
}
unsafe extern "C" {
    pub fn stbi_ldr_to_hdr_scale(scale: f32);
}
unsafe extern "C" {
    pub fn stbi_is_hdr_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_hdr_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_hdr(
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_hdr_from_file(f: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_failure_reason() -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn stbi_image_free(retval_from_stbi_load: *mut ::core::ffi::c_void);
}
unsafe extern "C" {
    pub fn stbi_info_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        comp: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_info_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::core::ffi::c_void,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        comp: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_16_bit_from_memory(
        buffer: *const stbi_uc,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_16_bit_from_callbacks(
        clbk: *const stbi_io_callbacks,
        user: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_info(
        filename: *const ::core::ffi::c_char,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        comp: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_info_from_file(
        f: *mut FILE,
        x: *mut ::core::ffi::c_int,
        y: *mut ::core::ffi::c_int,
        comp: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_16_bit(
        filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_is_16_bit_from_file(f: *mut FILE) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_set_unpremultiply_on_load(
        flag_true_if_should_unpremultiply: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn stbi_convert_iphone_png_to_rgb(
        flag_true_if_should_convert: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn stbi_set_flip_vertically_on_load(
        flag_true_if_should_flip: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn stbi_set_unpremultiply_on_load_thread(
        flag_true_if_should_unpremultiply: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn stbi_convert_iphone_png_to_rgb_thread(
        flag_true_if_should_convert: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn stbi_set_flip_vertically_on_load_thread(
        flag_true_if_should_flip: ::core::ffi::c_int,
    );
}
unsafe extern "C" {
    pub fn stbi_zlib_decode_malloc_guesssize(
        buffer: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        initial_size: ::core::ffi::c_int,
        outlen: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn stbi_zlib_decode_malloc_guesssize_headerflag(
        buffer: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        initial_size: ::core::ffi::c_int,
        outlen: *mut ::core::ffi::c_int,
        parse_header: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn stbi_zlib_decode_malloc(
        buffer: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        outlen: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn stbi_zlib_decode_buffer(
        obuffer: *mut ::core::ffi::c_char,
        olen: ::core::ffi::c_int,
        ibuffer: *const ::core::ffi::c_char,
        ilen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn stbi_zlib_decode_noheader_malloc(
        buffer: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        outlen: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
unsafe extern "C" {
    pub fn stbi_zlib_decode_noheader_buffer(
        obuffer: *mut ::core::ffi::c_char,
        olen: ::core::ffi::c_int,
        ibuffer: *const ::core::ffi::c_char,
        ilen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
