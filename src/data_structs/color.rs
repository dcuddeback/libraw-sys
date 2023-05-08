use libc::*;

#[repr(C)]
pub struct libraw_colordata_t {
    pub curve: [c_ushort; 65536usize],
    pub cblack: [c_uint; 4104usize],
    pub black: c_uint,
    pub data_maximum: c_uint,
    pub maximum: c_uint,
    pub linear_max: [c_long; 4usize],
    pub fmaximum: f32,
    pub fnorm: f32,
    pub white: [[c_ushort; 8usize]; 8usize],
    pub cam_mul: [f32; 4usize],
    pub pre_mul: [f32; 4usize],
    pub cmatrix: [[f32; 4usize]; 3usize],
    pub ccm: [[f32; 4usize]; 3usize],
    pub rgb_cam: [[f32; 4usize]; 3usize],
    pub cam_xyz: [[f32; 3usize]; 4usize],
    pub phase_one_data: ph1_t,
    pub flash_used: f32,
    pub canon_ev: f32,
    pub model2: [c_char; 64usize],
    pub UniqueCameraModel: [c_char; 64usize],
    pub LocalizedCameraModel: [c_char; 64usize],
    pub ImageUniqueID: [c_char; 64usize],
    pub RawDataUniqueID: [c_char; 17usize],
    pub OriginalRawFileName: [c_char; 64usize],
    pub profile: *mut c_void,
    pub profile_length: c_uint,
    pub black_stat: [c_uint; 8usize],
    pub dng_color: [libraw_dng_color_t; 2usize],
    pub dng_levels: libraw_dng_levels_t,
    pub WB_Coeffs: [[c_int; 4usize]; 256usize],
    pub WBCT_Coeffs: [[f32; 5usize]; 64usize],
    pub as_shot_wb_applied: c_int,
    pub P1_color: [libraw_P1_color_t; 2usize],
    pub raw_bps: c_uint,
    pub ExifColorSpace: c_int,
}

#[repr(C)]
pub struct libraw_dng_levels_t {
    pub parsedfields: c_uint,
    pub dng_cblack: [c_uint; 4104usize],
    pub dng_black: c_uint,
    pub dng_fcblack: [f32; 4104usize],
    pub dng_fblack: f32,
    pub dng_whitelevel: [c_uint; 4usize],
    pub default_crop: [c_ushort; 4usize],
    pub user_crop: [f32; 4usize],
    pub preview_colorspace: c_uint,
    pub analogbalance: [f32; 4usize],
    pub asshotneutral: [f32; 4usize],
    pub baseline_exposure: f32,
    pub LinearResponseLimit: f32,
}

#[repr(C)]
pub struct libraw_P1_color_t {
    pub romm_cam: [f32; 9usize],
}

#[repr(C)]
pub struct ph1_t {
    pub format: c_int,
    pub key_off: c_int,
    pub tag_21a: c_int,
    pub t_black: c_int,
    pub split_col: c_int,
    pub black_col: c_int,
    pub split_row: c_int,
    pub black_row: c_int,
    pub tag_210: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libraw_dng_color_t {
    pub parsedfields: c_uint,
    pub illuminant: c_ushort,
    pub calibration: [[f32; 4usize]; 4usize],
    pub colormatrix: [[f32; 3usize]; 4usize],
    pub forwardmatrix: [[f32; 4usize]; 3usize],
}
