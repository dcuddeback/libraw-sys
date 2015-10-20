#![allow(non_camel_case_types,non_snake_case)]

extern crate libc;

use libc::{c_void,c_char,c_uchar,c_short,c_ushort,c_int,c_uint,c_ulonglong,c_float,c_double,size_t,time_t};

#[repr(C)]
pub struct libraw_data_t {
    pub image: *mut [c_ushort; 4],
    pub sizes: libraw_image_sizes_t,
    pub idata: libraw_iparams_t,
    pub lens: libraw_lensinfo_t,
    pub params: libraw_output_params_t,
    pub progress_flags: c_uint,
    pub process_warnings: c_uint,
    pub color: libraw_colordata_t,
    pub other: libraw_imgother_t,
    pub thumbnail: libraw_thumbnail_t,
    pub rawdata: libraw_rawdata_t,
    pub parent_class: *mut c_void,
}

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
    pub pixel_aspect: c_double,
    pub flip: c_int,
    pub mask: [[c_int; 4]; 8],
}

#[repr(C)]
pub struct libraw_iparams_t {
    pub make: [c_char; 64],
    pub model: [c_char; 64],
    pub software: [c_char; 64],
    pub raw_count: c_uint,
    pub dng_version: c_uint,
    pub is_foveon: c_uint,
    pub colors: c_int,
    pub filters: c_uint,
    pub xtrans: [[c_char; 6]; 6],
    pub xtrans_abs: [[c_char; 6]; 6],
    pub cdesc: [c_char; 5],
    pub xmplen: c_uint,
    pub xmpdata: *mut c_char,
}

#[repr(C)]
pub struct libraw_lensinfo_t {
    pub MinFocal: c_float,
    pub MaxFocal: c_float,
    pub MaxAp4MinFocal: c_float,
    pub MaxAp4MaxFocal: c_float,
    pub EXIF_MaxAp: c_float,
    pub LensMake: [c_char; 128],
    pub Lens: [c_char; 128],
    pub FocalLengthIn35mmFormat: c_ushort,
    pub nikon: libraw_nikonlens_t,
    pub dng: libraw_dnglens_t,
    pub makernotes: libraw_makernotes_lens_t,
}

#[repr(C)]
pub struct libraw_nikonlens_t {
    pub NikonEffectiveMaxAp: c_float,
    pub NikonLensIDNumber: c_uchar,
    pub NikonLensFStops: c_uchar,
    pub NikonMCUVersion: c_uchar,
    pub NikonLensType: c_uchar,
}

#[repr(C)]
pub struct libraw_dnglens_t {
    pub MinFocal: c_float,
    pub MaxFocal: c_float,
    pub MaxAp4MinFocal: c_float,
    pub MaxAp4MaxFocal: c_float,
}

#[repr(C)]
pub struct libraw_makernotes_lens_t {
    pub LensID: c_ulonglong,
    pub Lens: [c_char; 128],
    pub LensFormat: c_ushort,
    pub LensMount: c_ushort,
    pub CamID: c_ulonglong,
    pub CameraFormat: c_ushort,
    pub CameraMount: c_ushort,
    pub body: [c_char; 64],
    pub FocalType: c_short,
    pub LensFeatures_pre: [c_char; 16],
    pub LensFeatures_suf: [c_char; 16],
    pub MinFocal: c_float,
    pub MaxFocal: c_float,
    pub MaxAp4MinFocal: c_float,
    pub MaxAp4MaxFocal: c_float,
    pub MinAp4MinFocal: c_float,
    pub MinAp4MaxFocal: c_float,
    pub MaxAp: c_float,
    pub MinAp: c_float,
    pub CurFocal: c_float,
    pub CurAp: c_float,
    pub MaxAp4CurFocal: c_float,
    pub MinAp4CurFocal: c_float,
    pub LensFStops: c_float,
    pub TeleconverterID: c_ulonglong,
    pub Teleconverter: [c_char; 128],
    pub AdapterID: c_ulonglong,
    pub Adapter: [c_char; 128],
    pub AttachmentID: c_ulonglong,
    pub Attachment: [c_char; 128],
    pub CanonFocalUnits: c_short,
    pub FocalLengthIn35mmFormat: c_float,
}

#[repr(C)]
pub struct libraw_output_params_t {
    pub greybox: [c_uint; 4],
    pub cropbox: [c_uint; 4],
    pub aber: [c_double; 4],
    pub gamm: [c_double; 6],
    pub user_mul: [c_float; 4],
    pub shot_select: c_uint,
    pub bright: c_float,
    pub threshold: c_float,
    pub half_size: c_int,
    pub four_color_rgb: c_int,
    pub highlight: c_int,
    pub use_auto_wb: c_int,
    pub use_camera_wb: c_int,
    pub use_camera_matrix: c_int,
    pub output_color: c_int,
    pub output_profile: *mut c_char,
    pub camera_profile: *mut c_char,
    pub bad_pixels: *mut c_char,
    pub dark_frame: *mut c_char,
    pub output_bps: c_int,
    pub output_tiff: c_int,
    pub user_flip: c_int,
    pub user_qual: c_int,
    pub user_black: c_int,
    pub user_cblack: [c_int; 4],
    pub user_sat: c_int,
    pub med_passes: c_int,
    pub auto_bright_thr: c_float,
    pub adjust_maximum_thr: c_float,
    pub no_auto_bright: c_int,
    pub use_fuji_rotate: c_int,
    pub green_matching: c_int,
    pub dcb_iterations: c_int,
    pub dcb_enhance_fl: c_int,
    pub fbdd_noiserd: c_int,
    pub eeci_refine: c_int,
    pub es_med_passes: c_int,
    pub ca_correc: c_int,
    pub cared: c_float,
    pub cablue: c_float,
    pub cfaline: c_int,
    pub linenoise: c_float,
    pub cfa_clean: c_int,
    pub lclean: c_float,
    pub cclean: c_float,
    pub cfa_green: c_int,
    pub green_thresh: c_float,
    pub exp_correc: c_int,
    pub exp_shift: c_float,
    pub exp_preser: c_float,
    pub wf_debanding: c_int,
    pub wf_deband_treshold: [c_float; 4],
    pub use_rawspeed: c_int,
    pub no_auto_scale: c_int,
    pub no_interpolation: c_int,
    pub sraw_ycc: c_int,
    pub force_foveon_x3f: c_int,
    pub x3f_flags: c_int,
    pub sony_arw2_options: c_int,
    pub sony_arw2_posterization_thr: c_int,
    pub coolscan_nef_gamma: c_float,
}

#[repr(C)]
pub struct libraw_colordata_t {
    pub curve: [c_ushort; 0x10000],
    pub cblack: [c_uint; 4102],
    pub black: c_uint,
    pub data_maximum: c_uint,
    pub maximum: c_uint,
    pub white: [[c_ushort; 8]; 8],
    pub cam_mul: [c_float; 4],
    pub pre_mul: [c_float; 4],
    pub cmatrix: [[c_float; 4]; 3],
    pub rgb_cam: [[c_float; 4]; 3],
    pub cam_xyz: [[c_float; 3]; 4],
    pub phase_one_data: ph1_t,
    pub flash_used: c_float,
    pub canon_ev: c_float,
    pub model2: [c_char; 64],
    pub profile: *mut c_void,
    pub profile_length: c_uint,
    pub black_stat: [c_uint; 8],
    pub dng_color: [libraw_dng_color_t; 2],
    pub canon_makernotes: libraw_canon_makernotes_t,
    pub baseline_exposure: c_float,
    pub OlympusSensorCalibration: [c_int; 2],
    pub FujiExpoMidPointShift: c_float,
    pub digitalBack_color: c_int,
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
    pub tag_210: c_float,
}

#[repr(C)]
pub struct libraw_dng_color_t {
    pub illuminant: c_ushort,
    pub calibration: [[c_float; 4]; 4],
    pub colormatrix: [[c_float; 3]; 4],
}

#[repr(C)]
pub struct libraw_canon_makernotes_t {
    pub CanonColorDataVer: c_int,
    pub CanonColorDataSubVer: c_int,
    pub SpecularWhiteLevel: c_int,
    pub AverageBlackLevel: c_int,
}

#[repr(C)]
pub struct libraw_imgother_t {
    pub iso_speed: c_float,
    pub shutter: c_float,
    pub aperture: c_float,
    pub focal_len: c_float,
    pub timestamp: time_t,
    pub shot_order: c_uint,
    pub gpsdata: [c_uint; 32],
    pub parsed_gps: libraw_gps_info_t,
    pub desc: [c_char; 512],
    pub artist: [c_char; 64],
}

#[repr(C)]
pub struct libraw_gps_info_t {
    pub latitude: [c_float; 3],
    pub longtitude: [c_float; 3],
    pub gpstimestamp: [c_float; 3],
    pub altitude: c_float,
    pub altref: c_char,
    pub latref: c_char,
    pub longref: c_char,
    pub gpsstatus: c_char,
    pub gpsparsed: c_char,
}

#[repr(C)]
pub struct libraw_thumbnail_t {
    pub tformat: LibRaw_thumbnail_formats,
    pub twidth: c_ushort,
    pub theight: c_ushort,
    pub tlength: c_uint,
    pub tcolors: c_int,
    pub thumb: *mut c_char,
}

#[repr(C)]
pub struct libraw_rawdata_t {
    pub raw_alloc: *mut c_void,
    pub raw_image: *mut c_ushort,
    pub color4_image: *mut [c_ushort; 4],
    pub color3_image: *mut [c_ushort; 3],
    pub ph1_cblack: *mut [c_short; 2],
    pub ph1_rblack: *mut [c_short; 2],
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

#[repr(C)]
pub struct libraw_decoder_info_t {
    pub decoder_name: *const c_char,
    pub decoder_flags: c_uint,
}

pub type LibRaw_constructor_flags = c_uint;
pub const LIBRAW_OPTIONS_NONE:               LibRaw_constructor_flags = 0;
pub const LIBRAW_OPIONS_NO_MEMERR_CALLBACK:  LibRaw_constructor_flags = 1;
pub const LIBRAW_OPIONS_NO_DATAERR_CALLBACK: LibRaw_constructor_flags = 1<<1;

pub type LibRaw_errors = c_int;
pub const LIBRAW_SUCCESS:                       LibRaw_errors = 0;
pub const LIBRAW_UNSPECIFIED_ERROR:             LibRaw_errors = -1;
pub const LIBRAW_FILE_UNSUPPORTED:              LibRaw_errors = -2;
pub const LIBRAW_REQUEST_FOR_NONEXISTENT_IMAGE: LibRaw_errors = -3;
pub const LIBRAW_OUT_OF_ORDER_CALL:             LibRaw_errors = -4;
pub const LIBRAW_NO_THUMBNAIL:                  LibRaw_errors = -5;
pub const LIBRAW_UNSUPPORTED_THUMBNAIL:         LibRaw_errors = -6;
pub const LIBRAW_INPUT_CLOSED:                  LibRaw_errors = -7;
pub const LIBRAW_INSUFFICIENT_MEMORY:           LibRaw_errors = -100007;
pub const LIBRAW_DATA_ERROR:                    LibRaw_errors = -100008;
pub const LIBRAW_IO_ERROR:                      LibRaw_errors = -100009;
pub const LIBRAW_CANCELLED_BY_CALLBACK:         LibRaw_errors = -100010;
pub const LIBRAW_BAD_CROP:                      LibRaw_errors = -100011;

pub type LibRaw_progress = c_int;
pub const LIBRAW_PROGRESS_START:              LibRaw_progress = 0;
pub const LIBRAW_PROGRESS_OPEN:               LibRaw_progress = 1<<0;
pub const LIBRAW_PROGRESS_IDENTIFY:           LibRaw_progress = 1<<1;
pub const LIBRAW_PROGRESS_SIZE_ADJUST:        LibRaw_progress = 1<<2;
pub const LIBRAW_PROGRESS_LOAD_RAW:           LibRaw_progress = 1<<3;
pub const LIBRAW_PROGRESS_RAW2_IMAGE:         LibRaw_progress = 1<<4;
pub const LIBRAW_PROGRESS_REMOVE_ZEROES:      LibRaw_progress = 1<<5;
pub const LIBRAW_PROGRESS_BAD_PIXELS:         LibRaw_progress = 1<<6;
pub const LIBRAW_PROGRESS_DARK_FRAME:         LibRaw_progress = 1<<7;
pub const LIBRAW_PROGRESS_FOVEON_INTERPOLATE: LibRaw_progress = 1<<8;
pub const LIBRAW_PROGRESS_SCALE_COLORS:       LibRaw_progress = 1<<9;
pub const LIBRAW_PROGRESS_PRE_INTERPOLATE:    LibRaw_progress = 1<<10;
pub const LIBRAW_PROGRESS_INTERPOLATE:        LibRaw_progress = 1<<11;
pub const LIBRAW_PROGRESS_MIX_GREEN:          LibRaw_progress = 1<<12;
pub const LIBRAW_PROGRESS_MEDIAN_FILTER:      LibRaw_progress = 1<<13;
pub const LIBRAW_PROGRESS_HIGHLIGHTS:         LibRaw_progress = 1<<14;
pub const LIBRAW_PROGRESS_FUJI_ROTATE:        LibRaw_progress = 1<<15;
pub const LIBRAW_PROGRESS_FLIP:               LibRaw_progress = 1<<16;
pub const LIBRAW_PROGRESS_APPLY_PROFILE:      LibRaw_progress = 1<<17;
pub const LIBRAW_PROGRESS_CONVERT_RGB:        LibRaw_progress = 1<<18;
pub const LIBRAW_PROGRESS_STRETCH:            LibRaw_progress = 1<<19;
pub const LIBRAW_PROGRESS_STAGE20:            LibRaw_progress = 1<<20;
pub const LIBRAW_PROGRESS_STAGE21:            LibRaw_progress = 1<<21;
pub const LIBRAW_PROGRESS_STAGE22:            LibRaw_progress = 1<<22;
pub const LIBRAW_PROGRESS_STAGE23:            LibRaw_progress = 1<<23;
pub const LIBRAW_PROGRESS_STAGE24:            LibRaw_progress = 1<<24;
pub const LIBRAW_PROGRESS_STAGE25:            LibRaw_progress = 1<<25;
pub const LIBRAW_PROGRESS_STAGE26:            LibRaw_progress = 1<<26;
pub const LIBRAW_PROGRESS_STAGE27:            LibRaw_progress = 1<<27;
pub const LIBRAW_PROGRESS_THUMB_LOAD:         LibRaw_progress = 1<<28;
pub const LIBRAW_PROGRESS_TRESERVED1:         LibRaw_progress = 1<<29;
pub const LIBRAW_PROGRESS_TRESERVED2:         LibRaw_progress = 1<<30;
pub const LIBRAW_PROGRESS_TRESERVED3:         LibRaw_progress = 1<<31;
pub const LIBRAW_PROGRESS_THUMB_MASK:         LibRaw_progress = 0x0fffffff;

pub type LibRaw_decoder_flags = c_uint;
pub const LIBRAW_DECODER_USEBAYER2:   LibRaw_decoder_flags = 1<<3;
pub const LIBRAW_DECODER_HASCURVE:    LibRaw_decoder_flags = 1<<4;
pub const LIBRAW_DECODER_SONYARW2:    LibRaw_decoder_flags = 1<<5;
pub const LIBRAW_DECODER_TRYRAWSPEED: LibRaw_decoder_flags = 1<<6;
pub const LIBRAW_DECODER_OWNALLOC:    LibRaw_decoder_flags = 1<<7;
pub const LIBRAW_DECODER_FIXEDMAXC:   LibRaw_decoder_flags = 1<<8;
pub const LIBRAW_DECODER_NOTSET:      LibRaw_decoder_flags = 1<<15;

#[repr(C)]
pub enum LibRaw_thumbnail_formats {
    LIBRAW_THUMBNAIL_UNKNOWN = 0,
    LIBRAW_THUMBNAIL_JPEG    = 1,
    LIBRAW_THUMBNAIL_BITMAP  = 2,
    LIBRAW_THUMBNAIL_LAYER   = 4,
    LIBRAW_THUMBNAIL_ROLLEI  = 5,
}

pub const LIBRAW_THUMBNAIL_UNKNOWN: LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_UNKNOWN;
pub const LIBRAW_THUMBNAIL_JPEG:    LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_JPEG;
pub const LIBRAW_THUMBNAIL_BITMAP:  LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_BITMAP;
pub const LIBRAW_THUMBNAIL_LAYER:   LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_LAYER;
pub const LIBRAW_THUMBNAIL_ROLLEI:  LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_ROLLEI;

#[repr(C)]
pub enum LibRaw_image_formats {
    LIBRAW_IMAGE_JPEG   = 1,
    LIBRAW_IMAGE_BITMAP = 2,
}

pub const LIBRAW_IMAGE_JPEG:   LibRaw_image_formats = LibRaw_image_formats::LIBRAW_IMAGE_JPEG;
pub const LIBRAW_IMAGE_BITMAP: LibRaw_image_formats = LibRaw_image_formats::LIBRAW_IMAGE_BITMAP;

pub type memory_callback      = extern "C" fn (data: *mut c_void, file: *const c_char, func: *const c_char);
pub type exif_parser_callback = extern "C" fn (data: *mut c_void, tag: c_int, tag_type: c_int, len: c_int, ord: c_uint, ifp: *mut c_void);
pub type data_callback        = extern "C" fn (data: *mut c_void, file: *const c_char, offset: c_int);
pub type progress_callback    = extern "C" fn (data: *mut c_void, stage: LibRaw_progress, iteration: c_int, expected: c_int) -> c_int;

extern "C" {
    pub fn libraw_version() -> *const c_char;
    pub fn libraw_versionNumber() -> c_int;

    pub fn libraw_cameraList() -> *const *const c_char;
    pub fn libraw_cameraCount() -> c_int;

    pub fn libraw_strprogress(progress: LibRaw_progress) -> *const c_char;
    pub fn libraw_strerror(err: c_int) -> *const c_char;

    pub fn libraw_init(flags: c_uint) -> *mut libraw_data_t;
    pub fn libraw_close(lr: *mut libraw_data_t);

    pub fn libraw_unpack_function_name(lr: *mut libraw_data_t) -> *const c_char;
    pub fn libraw_subtract_black(lr: *mut libraw_data_t);

    pub fn libraw_open_file(lr: *mut libraw_data_t, file: *const c_char) -> c_int;
    pub fn libraw_open_file_ex(lr: *mut libraw_data_t, file: *const c_char, sz: i64) -> c_int;

    pub fn libraw_open_buffer(lr: *mut libraw_data_t, buffer: *mut c_void, size: size_t) -> c_int;
    pub fn libraw_unpack(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_unpack_thumb(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_recycle_datastream(lr: *mut libraw_data_t);
    pub fn libraw_recycle(lr: *mut libraw_data_t);

    pub fn libraw_set_exifparser_handler(lr: *mut libraw_data_t, cb: exif_parser_callback, data: *mut c_void);
    pub fn libraw_set_memerror_handler(lr: *mut libraw_data_t, cb: memory_callback, data: *mut c_void);
    pub fn libraw_set_dataerror_handler(lr: *mut libraw_data_t, cb: data_callback, data: *mut c_void);
    pub fn libraw_set_progress_handler(lr: *mut libraw_data_t, cb: progress_callback, data: *mut c_void);

    pub fn libraw_adjust_sizes_info_only(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_dcraw_ppm_tiff_writer(lr: *mut libraw_data_t, filename: *const c_char) -> c_int;
    pub fn libraw_dcraw_thumb_writer(lr: *mut libraw_data_t, filename: *const c_char) -> c_int;
    pub fn libraw_dcraw_process(lr: *mut libraw_data_t) -> c_int;

    pub fn libraw_dcraw_make_mem_image(lr: *mut libraw_data_t, errc: *mut c_int) -> *mut libraw_processed_image_t;
    pub fn libraw_dcraw_make_mem_thumb(lr: *mut libraw_data_t, errc: *mut c_int) -> *mut libraw_processed_image_t;
    pub fn libraw_dcraw_clear_mem(p: *mut libraw_processed_image_t);

    pub fn libraw_raw2image(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_free_image(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_decoder_info(lr: *mut libraw_data_t, d: *mut libraw_decoder_info_t) -> c_int;

    pub fn libraw_COLOR(lr: *mut libraw_data_t, row: c_int, col: c_int) -> c_int;

    pub fn libraw_set_demosaic(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_color(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_bps(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_gamma(lr: *mut libraw_data_t, index: c_int, value: c_float);
    pub fn libraw_set_no_auto_bright(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_bright(lr: *mut libraw_data_t, value: c_float);
    pub fn libraw_set_highlight(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_fbdd_noiserd(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_get_raw_height(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_raw_width(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iheight(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iwidth(lr: *mut libraw_data_t) -> c_int;

    pub fn libraw_get_cam_mul(lr: *mut libraw_data_t, index: c_int) -> c_float;
    pub fn libraw_get_pre_mul(lr: *mut libraw_data_t, index: c_int) -> c_float;
    pub fn libraw_get_rgb_cam(lr: *mut libraw_data_t, index1: c_int, index2: c_int) -> c_float;
    pub fn libraw_get_color_maximum(lr: *mut libraw_data_t) -> c_int;
}
