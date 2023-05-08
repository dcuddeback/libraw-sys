#![allow(unused)]
use libc::*;

pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_UNKNOWN: LibRaw_thumbnail_formats = 0;
pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_JPEG: LibRaw_thumbnail_formats = 1;
pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_BITMAP: LibRaw_thumbnail_formats = 2;
pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_BITMAP16: LibRaw_thumbnail_formats = 3;
pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_LAYER: LibRaw_thumbnail_formats = 4;
pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_ROLLEI: LibRaw_thumbnail_formats = 5;
pub const LIB_RAW_THUMBNAIL_FORMATS_LIBRAW_THUMBNAIL_H265: LibRaw_thumbnail_formats = 6;
pub type LibRaw_thumbnail_formats = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_thumbnail_t {
    pub tformat: LibRaw_thumbnail_formats,
    pub twidth: c_ushort,
    pub theight: c_ushort,
    pub tlength: c_uint,
    pub tcolors: c_int,
    pub thumb: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct libraw_thumbnail_list_t {
    pub thumbcount: c_int,
    pub thumblist: [libraw_thumbnail_item_t; 8usize],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct libraw_thumbnail_item_t {
    pub tformat: LibRaw_internal_thumbnail_formats,
    pub twidth: c_ushort,
    pub theight: c_ushort,
    pub tflip: c_ushort,
    pub tlength: c_uint,
    pub tmisc: c_uint,
    pub toffset: i64,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub enum LibRaw_internal_thumbnail_formats {
    LIBRAW_INTERNAL_THUMBNAIL_UNKNOWN = 0,
    LIBRAW_INTERNAL_THUMBNAIL_KODAK_THUMB = 1,
    LIBRAW_INTERNAL_THUMBNAIL_KODAK_YCBCR = 2,
    LIBRAW_INTERNAL_THUMBNAIL_KODAK_RGB = 3,
    LIBRAW_INTERNAL_THUMBNAIL_JPEG = 4,
    LIBRAW_INTERNAL_THUMBNAIL_LAYER,
    LIBRAW_INTERNAL_THUMBNAIL_ROLLEI,
    LIBRAW_INTERNAL_THUMBNAIL_PPM,
    LIBRAW_INTERNAL_THUMBNAIL_PPM16,
    LIBRAW_INTERNAL_THUMBNAIL_X3F,
}
