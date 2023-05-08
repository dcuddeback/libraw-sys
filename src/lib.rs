#![allow(non_camel_case_types, non_snake_case)]

use libc::*;
pub mod data_structs;

use data_structs::*;
pub type LibRaw_constructor_flags = c_uint;
pub const LIBRAW_OPTIONS_NONE: LibRaw_constructor_flags = 0;
pub const LIBRAW_OPIONS_NO_MEMERR_CALLBACK: LibRaw_constructor_flags = 1;
pub const LIBRAW_OPIONS_NO_DATAERR_CALLBACK: LibRaw_constructor_flags = 1 << 1;

pub type LibRaw_errors = c_int;
pub const LIBRAW_SUCCESS: LibRaw_errors = 0;
pub const LIBRAW_UNSPECIFIED_ERROR: LibRaw_errors = -1;
pub const LIBRAW_FILE_UNSUPPORTED: LibRaw_errors = -2;
pub const LIBRAW_REQUEST_FOR_NONEXISTENT_IMAGE: LibRaw_errors = -3;
pub const LIBRAW_OUT_OF_ORDER_CALL: LibRaw_errors = -4;
pub const LIBRAW_NO_THUMBNAIL: LibRaw_errors = -5;
pub const LIBRAW_UNSUPPORTED_THUMBNAIL: LibRaw_errors = -6;
pub const LIBRAW_INPUT_CLOSED: LibRaw_errors = -7;
pub const LIBRAW_INSUFFICIENT_MEMORY: LibRaw_errors = -100007;
pub const LIBRAW_DATA_ERROR: LibRaw_errors = -100008;
pub const LIBRAW_IO_ERROR: LibRaw_errors = -100009;
pub const LIBRAW_CANCELLED_BY_CALLBACK: LibRaw_errors = -100010;
pub const LIBRAW_BAD_CROP: LibRaw_errors = -100011;

pub type LibRaw_progress = c_int;
pub const LIBRAW_PROGRESS_START: LibRaw_progress = 0;
pub const LIBRAW_PROGRESS_OPEN: LibRaw_progress = 1 << 0;
pub const LIBRAW_PROGRESS_IDENTIFY: LibRaw_progress = 1 << 1;
pub const LIBRAW_PROGRESS_SIZE_ADJUST: LibRaw_progress = 1 << 2;
pub const LIBRAW_PROGRESS_LOAD_RAW: LibRaw_progress = 1 << 3;
pub const LIBRAW_PROGRESS_RAW2_IMAGE: LibRaw_progress = 1 << 4;
pub const LIBRAW_PROGRESS_REMOVE_ZEROES: LibRaw_progress = 1 << 5;
pub const LIBRAW_PROGRESS_BAD_PIXELS: LibRaw_progress = 1 << 6;
pub const LIBRAW_PROGRESS_DARK_FRAME: LibRaw_progress = 1 << 7;
pub const LIBRAW_PROGRESS_FOVEON_INTERPOLATE: LibRaw_progress = 1 << 8;
pub const LIBRAW_PROGRESS_SCALE_COLORS: LibRaw_progress = 1 << 9;
pub const LIBRAW_PROGRESS_PRE_INTERPOLATE: LibRaw_progress = 1 << 10;
pub const LIBRAW_PROGRESS_INTERPOLATE: LibRaw_progress = 1 << 11;
pub const LIBRAW_PROGRESS_MIX_GREEN: LibRaw_progress = 1 << 12;
pub const LIBRAW_PROGRESS_MEDIAN_FILTER: LibRaw_progress = 1 << 13;
pub const LIBRAW_PROGRESS_HIGHLIGHTS: LibRaw_progress = 1 << 14;
pub const LIBRAW_PROGRESS_FUJI_ROTATE: LibRaw_progress = 1 << 15;
pub const LIBRAW_PROGRESS_FLIP: LibRaw_progress = 1 << 16;
pub const LIBRAW_PROGRESS_APPLY_PROFILE: LibRaw_progress = 1 << 17;
pub const LIBRAW_PROGRESS_CONVERT_RGB: LibRaw_progress = 1 << 18;
pub const LIBRAW_PROGRESS_STRETCH: LibRaw_progress = 1 << 19;
pub const LIBRAW_PROGRESS_STAGE20: LibRaw_progress = 1 << 20;
pub const LIBRAW_PROGRESS_STAGE21: LibRaw_progress = 1 << 21;
pub const LIBRAW_PROGRESS_STAGE22: LibRaw_progress = 1 << 22;
pub const LIBRAW_PROGRESS_STAGE23: LibRaw_progress = 1 << 23;
pub const LIBRAW_PROGRESS_STAGE24: LibRaw_progress = 1 << 24;
pub const LIBRAW_PROGRESS_STAGE25: LibRaw_progress = 1 << 25;
pub const LIBRAW_PROGRESS_STAGE26: LibRaw_progress = 1 << 26;
pub const LIBRAW_PROGRESS_STAGE27: LibRaw_progress = 1 << 27;
pub const LIBRAW_PROGRESS_THUMB_LOAD: LibRaw_progress = 1 << 28;
pub const LIBRAW_PROGRESS_TRESERVED1: LibRaw_progress = 1 << 29;
pub const LIBRAW_PROGRESS_TRESERVED2: LibRaw_progress = 1 << 30;
pub const LIBRAW_PROGRESS_TRESERVED3: LibRaw_progress = 1 << 31;
pub const LIBRAW_PROGRESS_THUMB_MASK: LibRaw_progress = 0x0fffffff;

pub type LibRaw_decoder_flags = c_uint;
pub const LIBRAW_DECODER_USEBAYER2: LibRaw_decoder_flags = 1 << 3;
pub const LIBRAW_DECODER_HASCURVE: LibRaw_decoder_flags = 1 << 4;
pub const LIBRAW_DECODER_SONYARW2: LibRaw_decoder_flags = 1 << 5;
pub const LIBRAW_DECODER_TRYRAWSPEED: LibRaw_decoder_flags = 1 << 6;
pub const LIBRAW_DECODER_OWNALLOC: LibRaw_decoder_flags = 1 << 7;
pub const LIBRAW_DECODER_FIXEDMAXC: LibRaw_decoder_flags = 1 << 8;
pub const LIBRAW_DECODER_NOTSET: LibRaw_decoder_flags = 1 << 15;

pub type memory_callback =
    extern "C" fn(data: *mut c_void, file: *const c_char, func: *const c_char);
pub type exif_parser_callback = extern "C" fn(
    data: *mut c_void,
    tag: c_int,
    tag_type: c_int,
    len: c_int,
    ord: c_uint,
    ifp: *mut c_void,
);
pub type data_callback = extern "C" fn(data: *mut c_void, file: *const c_char, offset: c_int);
pub type progress_callback = extern "C" fn(
    data: *mut c_void,
    stage: LibRaw_progress,
    iteration: c_int,
    expected: c_int,
) -> c_int;

extern "C" {
    pub fn libraw_version() -> *const c_char;
    pub fn libraw_versionNumber() -> c_int;
    pub fn libraw_capabilities() -> c_uint;

    pub fn libraw_cameraList() -> *const *const c_char;
    pub fn libraw_cameraCount() -> c_int;

    /// Converts progress stage code to description string (in English).
    pub fn libraw_strprogress(progress: LibRaw_progress) -> *const c_char;
    /// Analog of (the c) strerror(3) function: outputs the text descriptions of LibRaw error codes (in English).
    pub fn libraw_strerror(err: c_int) -> *const c_char;

    /// The function returns the pointer to the instance of libraw_data_t structure.  
    ///
    /// The resultant pointer should be passed as the first argument to all C API functions (except for libraw_strerror).
    ///  
    /// Returns NULL in case of error, pointer to the structure in all other cases.
    pub fn libraw_init(flags: c_uint) -> *mut libraw_data_t;
    /// Closes libraw_data_t handler and deallocates all memory.
    pub fn libraw_close(lr: *mut libraw_data_t);

    /// Returns function name of file unpacking function. Intended only for LibRaw test suite designers to use in test coverage evaluation.
    pub fn libraw_unpack_function_name(lr: *mut libraw_data_t) -> *const c_char;

    /// This call will subtract black level values from RAW data (for suitable RAW data). colordata.data_maximum and colordata.maximum and black level data (colordata.black and colordata.cblack) will be adjusted too.
    ///
    ///This call should be used if you postprocess RAW data by your own code. LibRaw postprocessing functions will call subtract_black() by oneself.
    ///
    ///The function returns an integer number in accordance with the return code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_subtract_black(lr: *mut libraw_data_t);

    /// Creates an LibRaw_file_datastream object, calls open_datastream(). If succeed, sets internal flag which signals to destroy internal datastream object on recycle(). On failure, just created file_datastream destroyed immediately.
    ///
    /// The function returns an integer number in accordance with the return code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_open_file(lr: *mut libraw_data_t, file: *const c_char) -> c_int;
    /// Creates an LibRaw_file_datastream object, calls open_datastream(). If succeed, sets internal flag which signals to destroy internal datastream object on recycle(). On failure, just created file_datastream destroyed immediately.
    ///
    /// Third parameter bigfile_size controls background I/O interface used for file operations. For files smaller than bigfile_size the LibRaw_file_datastream will be used and the LibRaw_bigfile_datastream otherwise.
    pub fn libraw_open_file_ex(lr: *mut libraw_data_t, file: *const c_char, sz: i64) -> c_int;
    /// Created an LibRaw_buffer_datastream object, calls open_datastream(). If succeed, sets internal flag which signals to destroy internal datastream object on recycle(). On failure, just created file_datastream destroyed immediately.
    ///
    /// The function returns an integer number in accordance with the return code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_open_buffer(lr: *mut libraw_data_t, buffer: *mut c_void, size: size_t) -> c_int;

    /// Parameters:
    ///
    ///data, datalen - buffer passed  
    ///
    ///_raw_width/_raw_height/*margin - image size and margins  
    ///
    ///procflags:  
    ///
    /// for 10-bit format:  
    ///     1: "4 pixels in 5 bytes" packing is used  
    ///     0: "6 pixels in 8 bytes" packing is used  
    ///
    /// for 16-bit format:  
    ///     1: Big-endian data  
    ///
    /// bayer_pattern: one of LIBRAW_OPENBAYER_RGGB,LIBRAW_OPENBAYER_BGGR, LIBRAW_OPENBAYER_GRBG,LIBRAW_OPENBAYER_GBRG  
    ///
    /// unused_bits: count of upper zero bits  
    ///
    /// otherflags:  
    ///     Bit 1 - filter (average neighbors) for pixels with values of zero  
    ///     Bits 2-4 - the orientation of the image (0=do not rotate, 3=180, 5=90CCW, 6=90CW)  
    ///
    ///black_level: file black level (it also may be specified via imgdata.params)  
    pub fn libraw_open_bayer(
        lr: *mut libraw_data_t,
        data: *mut c_uchar,
        datalen: c_uint,
        _raw_width: c_ushort,
        _raw_height: c_ushort,
        _left_margin: c_ushort,
        _top_margin: c_ushort,
        _right_margin: c_ushort,
        _bottom_margin: c_ushort,
        procflags: c_uchar,
        bayer_battern: c_uchar,
        unused_bits: c_uint,
        otherflags: c_uint,
        black_level: c_uint,
    ) -> c_int;

    /// Unpacks the RAW files of the image, calculates the black level (not for all formats). The results are placed in imgdata.image.
    ///
    /// The function returns an integer number in accordance with the return code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_unpack(lr: *mut libraw_data_t) -> c_int;
    ///  reads (or unpacks) the default (largest) image preview (thumbnail), placing the result into the imgdata.thumbnail.thumb buffer.
    pub fn libraw_unpack_thumb(lr: *mut libraw_data_t) -> c_int;
    /// See libraw_unpack_thumb
    ///
    /// additional parameter bigfile_size controls background I/O interface used for file operations. For files smaller than bigfile_size the LibRaw_file_datastream will be used and the LibRaw_bigfile_datastream otherwise.
    pub fn libraw_unpack_thumb_ex(lr: *mut libraw_data_t, ind: c_int) -> c_int;

    /// This call closes input datastream with associated data buffer and unblocks opened file.
    pub fn libraw_recycle_datastream(lr: *mut libraw_data_t);
    /// Frees the allocated data of LibRaw instance, enabling one to process the next file using the same processor. Repeated calls of recycle() are quite possible and do not conflict with anything.
    pub fn libraw_recycle(lr: *mut libraw_data_t);

    pub fn libraw_set_exifparser_handler(
        lr: *mut libraw_data_t,
        cb: exif_parser_callback,
        data: *mut c_void,
    );
    pub fn libraw_set_memerror_handler(
        lr: *mut libraw_data_t,
        cb: memory_callback,
        data: *mut c_void,
    );
    pub fn libraw_set_dataerror_handler(
        lr: *mut libraw_data_t,
        cb: data_callback,
        data: *mut c_void,
    );
    pub fn libraw_set_progress_handler(
        lr: *mut libraw_data_t,
        cb: progress_callback,
        data: *mut c_void,
    );

    ///The function calculates the correct size of the output image (imgdata.sizes.iwidth and imgdata.sizes.iheight) for the following cases:  
    ///     
    /// Files from Fuji cameras (with a 45-degree rotation)  
    ///
    /// Files from cameras with non-square pixels  
    ///
    /// Images shot by a rotated camera.  
    ///
    ///In the aforementioned cases, the function changes the fields of the image output size; note that this change cannot be repeated again.
    pub fn libraw_adjust_sizes_info_only(lr: *mut libraw_data_t) -> c_int;

    /// The function outputs the postprocessing results to a file in the PPM/PGM or TIFF format (the format is set via imgdata.params.output_tiff). The results are binary identical to those provided by dcraw.  
    ///
    /// If "-" is passed as outfile, the function will write to standard output (stdout).  
    ///
    /// The function returns an integer number in accordance with the error code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_dcraw_ppm_tiff_writer(lr: *mut libraw_data_t, filename: *const c_char) -> c_int;

    /// Writes the thumbnail to a file in the PPM format for bitmap thumbnails and in the JPEG format for JPEG thumbnails, i.e., in the format completely identical to the results provided by dcraw.  
    ///
    /// The function returns an integer number in accordance with the error code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_dcraw_thumb_writer(lr: *mut libraw_data_t, filename: *const c_char) -> c_int;

    /// The function emulates the postprocessing capabilities available in dcraw.  
    ///
    /// Called after calling LibRaw::unpack();  
    ///
    /// The entire functionality of dcraw (set via the field values in imgdata.params) is supported, except for  
    ///
    ///     Dark frame subtraction  
    ///     Work with bad pixels.  
    ///
    /// The function is intended solely for demonstration and testing purposes; it is assumed that its source code will be used in most real applications as the reference material concerning the order of RAW data processing.
    ///
    /// The function returns an integer number in accordance with the error code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_dcraw_process(lr: *mut libraw_data_t) -> c_int;
    /// Store processed image data into allocated buffer
    pub fn libraw_dcraw_make_mem_image(
        lr: *mut libraw_data_t,
        errc: *mut c_int,
    ) -> *mut libraw_processed_image_t;
    /// Store extracted thumbnail into buffer as JPEG-file image (for most cameras) or as RGB-bitmap.
    pub fn libraw_dcraw_make_mem_thumb(
        lr: *mut libraw_data_t,
        errc: *mut c_int,
    ) -> *mut libraw_processed_image_t;
    /// This function will free the memory allocated by dcraw_make_mem_image or dcraw_make_mem_thumb.  
    ///
    /// This call translates directly to free() system function, but it is better to use dcraw_clear_mem because LibRaw (DLL) may be compiled with memory manager other than in calling application.
    pub fn libraw_dcraw_clear_mem(p: *mut libraw_processed_image_t);

    /// This function allocates buffer for postprocessing (imgdata.image) and fills it with data layout compatible with LibRaw 0.13/0.14 and below. If the buffer is already allocated, it will be free()ed and allocated again.  
    ///
    /// This function should be called only if your code do postprocessing stage. If you use LibRaw's postprocessing calls (see below) you don't need to call raw2image().  
    ///
    /// The function returns an integer number in accordance with the return code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.  
    pub fn libraw_raw2image(lr: *mut libraw_data_t) -> c_int;

    /// This function releases the imgdata.image buffer allocated by raw2image();  
    ///
    /// This method should be called if current postprocessing results are not needed by the program (e.g. already copied somewhere), but new postprocessing calls (with another settings) are possible, so it is to early to call recycle().
    pub fn libraw_free_image(lr: *mut libraw_data_t) -> c_int;

    /// The function fills libraw_decoder_info_t structure by passed pointer with current raw decoder data.  
    ///
    /// The function returns an integer number in accordance with the return code convention: positive if any system call has returned an error, negative (from the LibRaw error list) if there has been an error situation within LibRaw.
    pub fn libraw_get_decoder_info(lr: *mut libraw_data_t, d: *mut libraw_decoder_info_t) -> c_int;

    /// This call returns pixel color (color component number) in bayer pattern at row,col. The returned value is in 0..3 range for 4-component Bayer (RGBG2, CMYG and so on) and in 0..2 range for 3-color data.  
    ///
    /// Color indexes returned could be used as index in imgdata.idata.cdesc string to get color 'name'.
    pub fn libraw_COLOR(lr: *mut libraw_data_t, row: c_int, col: c_int) -> c_int;

    pub fn libraw_set_demosaic(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_color(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_bps(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_gamma(lr: *mut libraw_data_t, index: c_int, value: c_float);
    pub fn libraw_set_no_auto_bright(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_bright(lr: *mut libraw_data_t, value: c_float);
    pub fn libraw_set_highlight(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_fbdd_noiserd(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_tif(lr: *mut libraw_data_t, value: c_int);

    pub fn libraw_get_raw_height(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_raw_width(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iheight(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iwidth(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iparams(lr: *mut libraw_data_t) -> *mut libraw_iparams_t;
    pub fn libraw_get_lensinfo(lr: *mut libraw_data_t) -> *mut libraw_lensinfo_t;
    pub fn libraw_get_imgother(lr: *mut libraw_data_t) -> *mut libraw_imgother_t;
    pub fn libraw_get_cam_mul(lr: *mut libraw_data_t, index: c_int) -> c_float;
    pub fn libraw_get_pre_mul(lr: *mut libraw_data_t, index: c_int) -> c_float;
    pub fn libraw_get_rgb_cam(lr: *mut libraw_data_t, index1: c_int, index2: c_int) -> c_float;
    pub fn libraw_get_color_maximum(lr: *mut libraw_data_t) -> c_int;

}
