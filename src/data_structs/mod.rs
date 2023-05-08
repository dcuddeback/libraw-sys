use libc::*;
mod color;
mod info;
mod params;
mod thumbnail;

pub use color::*;
pub use info::*;
pub use params::*;
pub use thumbnail::*;

/// Structure libraw_data_t is a "wrapping" for data structures accessible to the user of the library.
///
/// The data in this structure appear after a file is opened through open_file (and other open_ calls),
///
/// except for the image itself (filled by unpack()) and data containing the preview information (filled by calling unpack_thumb()).
#[repr(C)]
pub struct libraw_data_t {
    pub image: *mut [c_ushort; 4usize],
    pub sizes: libraw_image_sizes_t,
    pub idata: libraw_iparams_t,
    pub lens: libraw_lensinfo_t,
    pub makernotes: libraw_makernotes_t,
    pub shootinginfo: libraw_shootinginfo_t,
    pub params: libraw_output_params_t,
    pub rawparams: libraw_raw_unpack_params_t,
    pub progress_flags: c_uint,
    pub process_warnings: c_uint,
    pub color: libraw_colordata_t,
    pub other: libraw_imgother_t,
    pub thumbnail: libraw_thumbnail_t,
    pub thumbs_list: libraw_thumbnail_list_t,
    pub rawdata: libraw_rawdata_t,
    pub parent_class: *mut c_void,
}

/// Structure libraw_image_sizes_t is a collection of all file data that describe the size of the image.
#[repr(C)]
pub struct libraw_image_sizes_t {
    pub raw_height: c_ushort,
    pub raw_width: c_ushort,
    pub height: c_ushort,
    pub width: c_ushort,
    pub top_margin: c_ushort,
    pub left_margin: c_ushort,
    pub iheight: c_ushort,
    pub iwidth: c_ushort,
    pub raw_pitch: c_uint,
    pub pixel_aspect: f64,
    pub flip: c_int,
    pub mask: [[c_int; 4usize]; 8usize],
    pub raw_aspect: c_ushort,
    pub raw_inset_crops: [libraw_raw_inset_crop_t; 2usize],
}

/// Other Parameters of the Image
#[repr(C)]
pub struct libraw_imgother_t {
    pub iso_speed: f32,
    pub shutter: f32,
    pub aperture: f32,
    pub focal_len: f32,
    pub timestamp: time_t,
    pub shot_order: c_uint,
    pub gpsdata: [c_uint; 32usize],
    pub parsed_gps: libraw_gps_info_t,
    pub desc: [c_char; 512usize],
    pub artist: [c_char; 64usize],
    pub analogbalance: [f32; 4usize],
}

/// holds unpacked RAW data
#[repr(C)]
pub struct libraw_rawdata_t {
    pub raw_alloc: *mut c_void,
    pub raw_image: *mut c_ushort,
    pub color4_image: *mut [c_ushort; 4usize],
    pub color3_image: *mut [c_ushort; 3usize],
    pub float_image: *mut f32,
    pub float3_image: *mut [f32; 3usize],
    pub float4_image: *mut [f32; 4usize],
    pub ph1_cblack: *mut [c_short; 2usize],
    pub ph1_rblack: *mut [c_short; 2usize],
    pub iparams: libraw_iparams_t,
    pub sizes: libraw_image_sizes_t,
    pub ioparams: libraw_internal_output_params_t,
    pub color: libraw_colordata_t,
}

#[repr(C)]
pub struct libraw_internal_output_params_t {
    pub mix_green: c_uint,
    pub raw_color: c_uint,
    pub zero_is_bad: c_uint,
    pub shrink: c_ushort,
    pub fuji_width: c_ushort,
}

/// result set for dcraw_make_mem_image()/dcraw_make_mem_thumb() functions
///
/// contains in-memory image of interpolated data or thumbnail.
#[repr(C)]
pub struct libraw_processed_image_t {
    pub image_type: LibRaw_image_formats,
    pub height: c_ushort,
    pub width: c_ushort,
    pub colors: c_ushort,
    pub bits: c_ushort,
    pub data_size: c_uint,
    pub data: [c_uchar; 1],
}

/// RAW decoder name and data format
#[repr(C)]
pub struct libraw_decoder_info_t {
    pub decoder_name: *const c_char,
    pub decoder_flags: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_raw_inset_crop_t {
    pub cleft: c_ushort,
    pub ctop: c_ushort,
    pub cwidth: c_ushort,
    pub cheight: c_ushort,
}

pub const LIB_RAW_IMAGE_FORMATS_LIBRAW_IMAGE_JPEG: LibRaw_image_formats = 1;
pub const LIB_RAW_IMAGE_FORMATS_LIBRAW_IMAGE_BITMAP: LibRaw_image_formats = 2;
pub type LibRaw_image_formats = c_uint;
