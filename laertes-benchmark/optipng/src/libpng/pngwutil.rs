
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn png_write_flush(png_ptr: png_structrp);
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn png_write_data(png_ptr: png_structrp, data: png_const_bytep,
                      length: png_size_t);
    #[no_mangle]
    fn png_calculate_crc(png_ptr: png_structrp, ptr: png_const_bytep,
                         length: png_size_t);
    #[no_mangle]
    fn png_reset_crc(png_ptr: png_structrp);
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
    #[no_mangle]
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t)
     -> png_voidp;
    #[no_mangle]
    fn png_calloc(png_ptr: png_const_structrp, size: png_alloc_size_t)
     -> png_voidp;
    #[no_mangle]
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    #[no_mangle]
    fn png_warning(png_ptr: png_const_structrp,
                   warning_message: png_const_charp);
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateReset(strm: z_streamp) -> std::os::raw::c_int;
    #[no_mangle]
    fn deflateInit2_(strm: z_streamp, level: std::os::raw::c_int, method: std::os::raw::c_int,
                     windowBits: std::os::raw::c_int, memLevel: std::os::raw::c_int,
                     strategy: std::os::raw::c_int, version: *const std::os::raw::c_char,
                     stream_size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn png_zstream_error(png_ptr: png_structrp, ret: std::os::raw::c_int);
    #[no_mangle]
    fn png_safecat(buffer: png_charp, bufsize: size_t, pos: size_t,
                   string: png_const_charp) -> size_t;
    #[no_mangle]
    fn png_app_warning(png_ptr: png_const_structrp, message: png_const_charp);
}
pub type size_t = std::os::raw::c_ulong;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_16 = std::os::raw::c_ushort;
pub type png_int_32 = std::os::raw::c_int;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = size_t;
pub type png_alloc_size_t = png_size_t;
pub type png_fixed_point = png_int_32;
pub type png_voidp = *mut std::os::raw::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
pub type png_const_uint_16p = *const png_uint_16;
pub type png_charp = *mut std::os::raw::c_char;
pub type png_const_charp = *const std::os::raw::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_struct_def {
    pub error_fn: png_error_ptr,
    pub warning_fn: png_error_ptr,
    pub error_ptr: png_voidp,
    pub write_data_fn: png_rw_ptr,
    pub read_data_fn: png_rw_ptr,
    pub io_ptr: png_voidp,
    pub mode: png_uint_32,
    pub flags: png_uint_32,
    pub transformations: png_uint_32,
    pub zowner: png_uint_32,
    pub zstream: z_stream,
    pub zbuffer_list: png_compression_bufferp,
    pub zbuffer_size: uInt,
    pub zlib_level: std::os::raw::c_int,
    pub zlib_method: std::os::raw::c_int,
    pub zlib_window_bits: std::os::raw::c_int,
    pub zlib_mem_level: std::os::raw::c_int,
    pub zlib_strategy: std::os::raw::c_int,
    pub zlib_set_level: std::os::raw::c_int,
    pub zlib_set_method: std::os::raw::c_int,
    pub zlib_set_window_bits: std::os::raw::c_int,
    pub zlib_set_mem_level: std::os::raw::c_int,
    pub zlib_set_strategy: std::os::raw::c_int,
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub num_rows: png_uint_32,
    pub usr_width: png_uint_32,
    pub rowbytes: png_size_t,
    pub iwidth: png_uint_32,
    pub row_number: png_uint_32,
    pub chunk_name: png_uint_32,
    pub prev_row: png_bytep,
    pub row_buf: png_bytep,
    pub try_row: png_bytep,
    pub tst_row: png_bytep,
    pub info_rowbytes: png_size_t,
    pub idat_size: png_uint_32,
    pub crc: png_uint_32,
    pub palette: png_colorp,
    pub num_palette: png_uint_16,
    pub num_palette_max: std::os::raw::c_int,
    pub num_trans: png_uint_16,
    pub compression: png_byte,
    pub filter: png_byte,
    pub interlaced: png_byte,
    pub pass: png_byte,
    pub do_filter: png_byte,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub usr_bit_depth: png_byte,
    pub pixel_depth: png_byte,
    pub channels: png_byte,
    pub usr_channels: png_byte,
    pub sig_bytes: png_byte,
    pub maximum_pixel_depth: png_byte,
    pub transformed_pixel_depth: png_byte,
    pub zstream_start: png_byte,
    pub background_gamma_type: png_byte,
    pub background_gamma: png_fixed_point,
    pub background: png_color_16,
    pub output_flush_fn: png_flush_ptr,
    pub flush_dist: png_uint_32,
    pub flush_rows: png_uint_32,
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub trans_color: png_color_16,
    pub read_row_fn: png_read_status_ptr,
    pub write_row_fn: png_write_status_ptr,
    pub free_me: png_uint_32,
    pub unknown_default: std::os::raw::c_int,
    pub num_chunk_list: std::os::raw::c_uint,
    pub chunk_list: png_bytep,
    pub big_row_buf: png_bytep,
    pub compression_type: png_byte,
    pub user_width_max: png_uint_32,
    pub user_height_max: png_uint_32,
    pub user_chunk_cache_max: png_uint_32,
    pub user_chunk_malloc_max: png_alloc_size_t,
    pub unknown_chunk: png_unknown_chunk,
    pub old_big_row_buf_size: png_size_t,
    pub read_buffer: png_bytep,
    pub read_buffer_size: png_alloc_size_t,
    pub IDAT_read_size: uInt,
    pub io_state: png_uint_32,
    pub big_prev_row: png_bytep,
    pub read_filter: [Option<unsafe extern "C" fn(_: png_row_infop,
                                                  _: png_bytep,
                                                  _: png_const_bytep)
                                 -> ()>; 4],
}
pub type png_row_infop = *mut png_row_info;
pub type png_row_info = png_row_info_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_row_info_struct {
    pub width: png_uint_32,
    pub rowbytes: png_size_t,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
}
pub type uInt = std::os::raw::c_uint;
pub type png_unknown_chunk = png_unknown_chunk_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_unknown_chunk_t {
    pub name: [png_byte; 5],
    pub data: *mut png_byte,
    pub size: png_size_t,
    pub location: png_byte,
}
pub type png_write_status_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_uint_32,
                                _: std::os::raw::c_int) -> ()>;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_read_status_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_uint_32,
                                _: std::os::raw::c_int) -> ()>;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_color_8 = png_color_8_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_flush_ptr = Option<unsafe extern "C" fn(_: png_structp) -> ()>;
pub type png_colorp = *mut png_color;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_compression_bufferp = *mut png_compression_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_compression_buffer {
    pub next: *mut png_compression_buffer,
    pub output: [png_byte; 1],
}
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const std::os::raw::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: std::os::raw::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = std::os::raw::c_ulong;
pub type voidpf = *mut std::os::raw::c_void;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type Bytef = Byte;
pub type Byte = std::os::raw::c_uchar;
pub type png_rw_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_bytep, _: png_size_t)
               -> ()>;
pub type png_error_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_const_charp) -> ()>;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_const_colorp = *const png_color;
pub type png_const_color_16p = *const png_color_16;
pub type png_const_color_8p = *const png_color_8;
pub type z_streamp = *mut z_stream;
/* pngwutil.c - utilities to write a PNG file
 *
 * Last changed in libpng 1.6.32 [August 24, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */
/* Place a 32-bit number into a buffer in PNG byte order.  We work
 * with unsigned numbers for convenience, although one supported
 * ancillary chunk uses signed (two's complement) numbers.
 */
#[no_mangle]
pub unsafe extern "C" fn png_save_uint_32(mut buf: png_bytep,
                                          mut i: png_uint_32) {
    *buf.offset(0 as std::os::raw::c_int as isize) =
        (i >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_uint) as png_byte;
    *buf.offset(1 as std::os::raw::c_int as isize) =
        (i >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_uint) as png_byte;
    *buf.offset(2 as std::os::raw::c_int as isize) =
        (i >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_uint) as png_byte;
    *buf.offset(3 as std::os::raw::c_int as isize) =
        (i & 0xff as std::os::raw::c_uint) as png_byte;
}
/* Place a 16-bit number into a buffer in PNG byte order.
 * The parameter is declared unsigned int, not png_uint_16,
 * just to avoid potential problems on pre-ANSI C compilers.
 */
#[no_mangle]
pub unsafe extern "C" fn png_save_uint_16(mut buf: png_bytep,
                                          mut i: std::os::raw::c_uint) {
    *buf.offset(0 as std::os::raw::c_int as isize) =
        (i >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_uint) as png_byte;
    *buf.offset(1 as std::os::raw::c_int as isize) =
        (i & 0xff as std::os::raw::c_uint) as png_byte;
}
/* Simple function to write the signature.  If we have already written
 * the magic bytes of the signature, or more likely, the PNG stream is
 * being embedded into another stream and doesn't need its own signature,
 * we should call png_set_sig_bytes() to tell libpng how many of the
 * bytes have already been written.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_sig(mut png_ptr: png_structrp) {
    let mut png_signature: [png_byte; 8] =
        [137 as std::os::raw::c_int as png_byte, 80 as std::os::raw::c_int as png_byte,
         78 as std::os::raw::c_int as png_byte, 71 as std::os::raw::c_int as png_byte,
         13 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte,
         26 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte];
    /* Inform the I/O callback that the signature is being written */
    (*png_ptr).io_state =
        (0x2 as std::os::raw::c_int | 0x10 as std::os::raw::c_int) as png_uint_32;
    /* Write the rest of the 8 byte signature */
    png_write_data(png_ptr,
                   &mut *png_signature.as_mut_ptr().offset((*png_ptr).sig_bytes
                                                               as isize) as
                       *mut png_byte as png_const_bytep,
                   (8 as std::os::raw::c_int - (*png_ptr).sig_bytes as std::os::raw::c_int) as
                       png_size_t);
    if ((*png_ptr).sig_bytes as std::os::raw::c_int) < 3 as std::os::raw::c_int {
        (*png_ptr).mode |= 0x1000 as std::os::raw::c_uint
    };
}
/* Write the start of a PNG chunk.  The type is the chunk type.
 * The total_length is the sum of the lengths of all the data you will be
 * passing in png_write_chunk_data().
 */
unsafe extern "C" fn png_write_chunk_header(mut png_ptr: png_structrp,
                                            mut chunk_name: png_uint_32,
                                            mut length: png_uint_32) {
    let mut buf: [png_byte; 8] = [0; 8];
    if png_ptr.is_null() { return }
    /* Inform the I/O callback that the chunk header is being written.
    * PNG_IO_CHUNK_HDR requires a single I/O call.
    */
    (*png_ptr).io_state =
        (0x2 as std::os::raw::c_int | 0x20 as std::os::raw::c_int) as png_uint_32;
    /* Write the length and the chunk name */
    png_save_uint_32(buf.as_mut_ptr(), length);
    png_save_uint_32(buf.as_mut_ptr().offset(4 as std::os::raw::c_int as isize),
                     chunk_name);
    png_write_data(png_ptr, buf.as_mut_ptr() as png_const_bytep,
                   8 as std::os::raw::c_int as png_size_t);
    /* Put the chunk name into png_ptr->chunk_name */
    (*png_ptr).chunk_name = chunk_name;
    /* Reset the crc and run it over the chunk name */
    png_reset_crc(png_ptr);
    png_calculate_crc(png_ptr,
                      buf.as_mut_ptr().offset(4 as std::os::raw::c_int as isize) as
                          png_const_bytep, 4 as std::os::raw::c_int as png_size_t);
    /* Inform the I/O callback that chunk data will (possibly) be written.
    * PNG_IO_CHUNK_DATA does NOT require a specific number of I/O calls.
    */
    (*png_ptr).io_state =
        (0x2 as std::os::raw::c_int | 0x40 as std::os::raw::c_int) as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_write_chunk_start(mut png_ptr: png_structrp,
                                               mut chunk_string:
                                                   png_const_bytep,
                                               mut length: png_uint_32) {
    png_write_chunk_header(png_ptr,
                           ((0xff as std::os::raw::c_int &
                                 *chunk_string.offset(0 as std::os::raw::c_int as
                                                          isize) as
                                     std::os::raw::c_int) as png_uint_32) <<
                               24 as std::os::raw::c_int |
                               ((0xff as std::os::raw::c_int &
                                     *chunk_string.offset(1 as std::os::raw::c_int as
                                                              isize) as
                                         std::os::raw::c_int) as png_uint_32) <<
                                   16 as std::os::raw::c_int |
                               ((0xff as std::os::raw::c_int &
                                     *chunk_string.offset(2 as std::os::raw::c_int as
                                                              isize) as
                                         std::os::raw::c_int) as png_uint_32) <<
                                   8 as std::os::raw::c_int |
                               ((0xff as std::os::raw::c_int &
                                     *chunk_string.offset(3 as std::os::raw::c_int as
                                                              isize) as
                                         std::os::raw::c_int) as png_uint_32) <<
                                   0 as std::os::raw::c_int, length);
}
/* Write the data of a PNG chunk started with png_write_chunk_header().
 * Note that multiple calls to this function are allowed, and that the
 * sum of the lengths from these calls *must* add up to the total_length
 * given to png_write_chunk_header().
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_chunk_data(mut png_ptr: png_structrp,
                                              mut data: png_const_bytep,
                                              mut length: png_size_t) {
    /* Write the data, and run the CRC over it */
    if png_ptr.is_null() { return }
    if !data.is_null() && length > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        png_write_data(png_ptr, data, length);
        /* Update the CRC after writing the data,
       * in case the user I/O routine alters it.
       */
        png_calculate_crc(png_ptr, data, length);
    };
}
/* Finish a chunk started with png_write_chunk_header(). */
#[no_mangle]
pub unsafe extern "C" fn png_write_chunk_end(mut png_ptr: png_structrp) {
    let mut buf: [png_byte; 4] = [0; 4];
    if png_ptr.is_null() { return }
    /* Inform the I/O callback that the chunk CRC is being written.
    * PNG_IO_CHUNK_CRC requires a single I/O function call.
    */
    (*png_ptr).io_state =
        (0x2 as std::os::raw::c_int | 0x80 as std::os::raw::c_int) as png_uint_32;
    /* Write the crc in a single operation */
    png_save_uint_32(buf.as_mut_ptr(), (*png_ptr).crc);
    png_write_data(png_ptr, buf.as_mut_ptr() as png_const_bytep,
                   4 as std::os::raw::c_int as png_size_t);
}
/* Write a PNG chunk all at once.  The type is an array of ASCII characters
 * representing the chunk name.  The array must be at least 4 bytes in
 * length, and does not need to be null terminated.  To be safe, pass the
 * pre-defined chunk names here, and if you need a new one, define it
 * where the others are defined.  The length is the length of the data.
 * All the data must be present.  If that is not possible, use the
 * png_write_chunk_start(), png_write_chunk_data(), and png_write_chunk_end()
 * functions instead.
 */
unsafe extern "C" fn png_write_complete_chunk(mut png_ptr: png_structrp,
                                              mut chunk_name: png_uint_32,
                                              mut data: png_const_bytep,
                                              mut length: png_size_t) {
    if png_ptr.is_null() { return }
    /* On 64-bit architectures 'length' may not fit in a png_uint_32. */
    if length > 0x7fffffff as std::os::raw::c_long as png_uint_32 as std::os::raw::c_ulong {
        png_error(png_ptr,
                  b"length exceeds PNG maximum\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    png_write_chunk_header(png_ptr, chunk_name, length as png_uint_32);
    png_write_chunk_data(png_ptr, data, length);
    png_write_chunk_end(png_ptr);
}
/* This is the API that calls the internal function above. */
#[no_mangle]
pub unsafe extern "C" fn png_write_chunk(mut png_ptr: png_structrp,
                                         mut chunk_string: png_const_bytep,
                                         mut data: png_const_bytep,
                                         mut length: png_size_t) {
    png_write_complete_chunk(png_ptr,
                             ((0xff as std::os::raw::c_int &
                                   *chunk_string.offset(0 as std::os::raw::c_int as
                                                            isize) as
                                       std::os::raw::c_int) as png_uint_32) <<
                                 24 as std::os::raw::c_int |
                                 ((0xff as std::os::raw::c_int &
                                       *chunk_string.offset(1 as std::os::raw::c_int
                                                                as isize) as
                                           std::os::raw::c_int) as png_uint_32) <<
                                     16 as std::os::raw::c_int |
                                 ((0xff as std::os::raw::c_int &
                                       *chunk_string.offset(2 as std::os::raw::c_int
                                                                as isize) as
                                           std::os::raw::c_int) as png_uint_32) <<
                                     8 as std::os::raw::c_int |
                                 ((0xff as std::os::raw::c_int &
                                       *chunk_string.offset(3 as std::os::raw::c_int
                                                                as isize) as
                                           std::os::raw::c_int) as png_uint_32) <<
                                     0 as std::os::raw::c_int, data, length);
}
/* This is used below to find the size of an image to pass to png_deflate_claim,
 * so it only needs to be accurate if the size is less than 16384 bytes (the
 * point at which a lower LZ window size can be used.)
 */
unsafe extern "C" fn png_image_size(mut png_ptr: png_structrp)
 -> png_alloc_size_t {
    /* Only return sizes up to the maximum of a png_uint_32; do this by limiting
    * the width and height used to 15 bits.
    */
    let mut h: png_uint_32 = (*png_ptr).height;
    if (*png_ptr).rowbytes < 32768 as std::os::raw::c_int as std::os::raw::c_ulong &&
           h < 32768 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int {
            /* Interlacing makes the image larger because of the replication of
          * both the filter byte and the padding to a byte boundary.
          */
            let mut w: png_uint_32 = (*png_ptr).width;
            let mut pd: std::os::raw::c_uint = (*png_ptr).pixel_depth as std::os::raw::c_uint;
            let mut cb_base: png_alloc_size_t = 0;
            let mut pass: std::os::raw::c_int = 0;
            cb_base = 0 as std::os::raw::c_int as png_alloc_size_t;
            pass = 0 as std::os::raw::c_int;
            while pass <= 6 as std::os::raw::c_int {
                let mut pw: png_uint_32 =
                    w.wrapping_add((((1 as std::os::raw::c_int) <<
                                         (if pass > 1 as std::os::raw::c_int {
                                              (7 as std::os::raw::c_int - pass) >>
                                                  1 as std::os::raw::c_int
                                          } else { 3 as std::os::raw::c_int })) -
                                        1 as std::os::raw::c_int -
                                        ((1 as std::os::raw::c_int & pass) <<
                                             3 as std::os::raw::c_int -
                                                 (pass + 1 as std::os::raw::c_int >>
                                                      1 as std::os::raw::c_int) &
                                             7 as std::os::raw::c_int)) as
                                       std::os::raw::c_uint) >>
                        (if pass > 1 as std::os::raw::c_int {
                             (7 as std::os::raw::c_int - pass) >> 1 as std::os::raw::c_int
                         } else { 3 as std::os::raw::c_int });
                if pw > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    cb_base =
                        (cb_base as
                             std::os::raw::c_ulong).wrapping_add((if pd >=
                                                                 8 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     std::os::raw::c_uint
                                                             {
                                                              (pw as
                                                                   png_size_t).wrapping_mul(pd
                                                                                                as
                                                                                                png_size_t
                                                                                                >>
                                                                                                3
                                                                                                    as
                                                                                                    std::os::raw::c_int)
                                                          } else {
                                                              ((pw as
                                                                    png_size_t).wrapping_mul(pd
                                                                                                 as
                                                                                                 png_size_t).wrapping_add(7
                                                                                                                              as
                                                                                                                              std::os::raw::c_int
                                                                                                                              as
                                                                                                                              std::os::raw::c_ulong))
                                                                  >>
                                                                  3 as
                                                                      std::os::raw::c_int
                                                          }).wrapping_add(1 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong).wrapping_mul((h.wrapping_add((((1
                                                                                                                                 as
                                                                                                                                 std::os::raw::c_int)
                                                                                                                                <<
                                                                                                                                (if pass
                                                                                                                                        >
                                                                                                                                        2
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_int
                                                                                                                                    {
                                                                                                                                     (8
                                                                                                                                          as
                                                                                                                                          std::os::raw::c_int
                                                                                                                                          -
                                                                                                                                          pass)
                                                                                                                                         >>
                                                                                                                                         1
                                                                                                                                             as
                                                                                                                                             std::os::raw::c_int
                                                                                                                                 } else {
                                                                                                                                     3
                                                                                                                                         as
                                                                                                                                         std::os::raw::c_int
                                                                                                                                 }))
                                                                                                                               -
                                                                                                                               1
                                                                                                                                   as
                                                                                                                                   std::os::raw::c_int
                                                                                                                               -
                                                                                                                               ((1
                                                                                                                                     as
                                                                                                                                     std::os::raw::c_int
                                                                                                                                     &
                                                                                                                                     !pass)
                                                                                                                                    <<
                                                                                                                                    3
                                                                                                                                        as
                                                                                                                                        std::os::raw::c_int
                                                                                                                                        -
                                                                                                                                        (pass
                                                                                                                                             >>
                                                                                                                                             1
                                                                                                                                                 as
                                                                                                                                                 std::os::raw::c_int)
                                                                                                                                    &
                                                                                                                                    7
                                                                                                                                        as
                                                                                                                                        std::os::raw::c_int))
                                                                                                                              as
                                                                                                                              std::os::raw::c_uint)
                                                                                                               >>
                                                                                                               (if pass
                                                                                                                       >
                                                                                                                       2
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                   {
                                                                                                                    (8
                                                                                                                         as
                                                                                                                         std::os::raw::c_int
                                                                                                                         -
                                                                                                                         pass)
                                                                                                                        >>
                                                                                                                        1
                                                                                                                            as
                                                                                                                            std::os::raw::c_int
                                                                                                                } else {
                                                                                                                    3
                                                                                                                        as
                                                                                                                        std::os::raw::c_int
                                                                                                                }))
                                                                                                              as
                                                                                                              std::os::raw::c_ulong))
                            as png_alloc_size_t as png_alloc_size_t
                }
                pass += 1
            }
            return cb_base
        } else {
            return (*png_ptr).rowbytes.wrapping_add(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong).wrapping_mul(h
                                                                                        as
                                                                                        std::os::raw::c_ulong)
        }
    } else { return 0xffffffff as std::os::raw::c_uint as png_alloc_size_t };
}
/* This is the code to hack the first two bytes of the deflate stream (the
    * deflate header) to correct the windowBits value to match the actual data
    * size.  Note that the second argument is the *uncompressed* size but the
    * first argument is the *compressed* data (and it must be deflate
    * compressed.)
    */
unsafe extern "C" fn optimize_cmf(mut data: png_bytep,
                                  mut data_size: png_alloc_size_t) {
    /* Optimize the CMF field in the zlib stream.  The resultant zlib stream is
    * still compliant to the stream specification.
    */
    if data_size <= 16384 as std::os::raw::c_int as std::os::raw::c_ulong {
        /* else windowBits must be 15 */
        let mut z_cmf: std::os::raw::c_uint =
            *data.offset(0 as std::os::raw::c_int as isize) as
                std::os::raw::c_uint; /* zlib compression method and flags */
        if z_cmf & 0xf as std::os::raw::c_int as std::os::raw::c_uint ==
               8 as std::os::raw::c_int as std::os::raw::c_uint &&
               z_cmf & 0xf0 as std::os::raw::c_int as std::os::raw::c_uint <=
                   0x70 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut z_cinfo: std::os::raw::c_uint = 0;
            let mut half_z_window_size: std::os::raw::c_uint = 0;
            z_cinfo = z_cmf >> 4 as std::os::raw::c_int;
            half_z_window_size =
                (1 as std::os::raw::c_uint) <<
                    z_cinfo.wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_uint);
            if data_size <= half_z_window_size as std::os::raw::c_ulong {
                /* else no change */
                let mut tmp: std::os::raw::c_uint = 0;
                loop  {
                    half_z_window_size >>= 1 as std::os::raw::c_int;
                    z_cinfo = z_cinfo.wrapping_sub(1);
                    if !(z_cinfo > 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                             data_size <= half_z_window_size as std::os::raw::c_ulong)
                       {
                        break ;
                    }
                }
                z_cmf =
                    z_cmf & 0xf as std::os::raw::c_int as std::os::raw::c_uint |
                        z_cinfo << 4 as std::os::raw::c_int;
                *data.offset(0 as std::os::raw::c_int as isize) = z_cmf as png_byte;
                tmp =
                    (*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                         0xe0 as std::os::raw::c_int) as std::os::raw::c_uint;
                tmp =
                    tmp.wrapping_add((0x1f as std::os::raw::c_int as
                                          std::os::raw::c_uint).wrapping_sub((z_cmf <<
                                                                          8 as
                                                                              std::os::raw::c_int).wrapping_add(tmp).wrapping_rem(0x1f
                                                                                                                              as
                                                                                                                              std::os::raw::c_int
                                                                                                                              as
                                                                                                                              std::os::raw::c_uint)));
                *data.offset(1 as std::os::raw::c_int as isize) = tmp as png_byte
            }
        }
    };
}
/* WRITE_OPTIMIZE_CMF */
/* Initialize the compressor for the appropriate type of compression. */
unsafe extern "C" fn png_deflate_claim(mut png_ptr: png_structrp,
                                       mut owner: png_uint_32,
                                       mut data_size: png_alloc_size_t)
 -> std::os::raw::c_int {
    if (*png_ptr).zowner != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut msg: [std::os::raw::c_char; 64] = [0; 64];
        *msg.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) =
            (owner >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(1 as std::os::raw::c_int as isize) =
            (owner >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(2 as std::os::raw::c_int as isize) =
            (owner >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(3 as std::os::raw::c_int as isize) =
            (owner & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
        msg[4 as std::os::raw::c_int as usize] = ':' as i32 as std::os::raw::c_char;
        msg[5 as std::os::raw::c_int as usize] = ' ' as i32 as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(6 as std::os::raw::c_int as
                                     isize).offset(0 as std::os::raw::c_int as isize)
            =
            ((*png_ptr).zowner >> 24 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(6 as std::os::raw::c_int as
                                     isize).offset(1 as std::os::raw::c_int as isize)
            =
            ((*png_ptr).zowner >> 16 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(6 as std::os::raw::c_int as
                                     isize).offset(2 as std::os::raw::c_int as isize)
            =
            ((*png_ptr).zowner >> 8 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
        *msg.as_mut_ptr().offset(6 as std::os::raw::c_int as
                                     isize).offset(3 as std::os::raw::c_int as isize)
            =
            ((*png_ptr).zowner & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                std::os::raw::c_char;
        /* So the message that results is "<chunk> using zstream"; this is an
       * internal error, but is very useful for debugging.  i18n requirements
       * are minimal.
       */
        png_safecat(msg.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 64]>() as
                        std::os::raw::c_ulong, 10 as std::os::raw::c_int as size_t,
                    b" using zstream\x00" as *const u8 as
                        *const std::os::raw::c_char);
        png_warning(png_ptr, msg.as_mut_ptr() as png_const_charp);
        /* Attempt sane error recovery */
        if (*png_ptr).zowner ==
               (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
                   (68 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
                   (65 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
                   (84 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
            /* don't steal from IDAT */
            (*png_ptr).zstream.msg =
                b"in use by IDAT\x00" as *const u8 as
                    *const std::os::raw::c_char; /* set below */
            return -(2 as std::os::raw::c_int)
        } /* zlib return code */
        (*png_ptr).zowner = 0 as std::os::raw::c_int as png_uint_32
    }
    let mut level: std::os::raw::c_int = (*png_ptr).zlib_level;
    let mut method: std::os::raw::c_int = (*png_ptr).zlib_method;
    let mut windowBits: std::os::raw::c_int = (*png_ptr).zlib_window_bits;
    let mut memLevel: std::os::raw::c_int = (*png_ptr).zlib_mem_level;
    let mut strategy: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    if owner ==
           (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
               (68 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
               (65 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
               (84 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
        if (*png_ptr).flags & 0x1 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            strategy = (*png_ptr).zlib_strategy
        } else if (*png_ptr).do_filter as std::os::raw::c_int != 0x8 as std::os::raw::c_int {
            strategy = 1 as std::os::raw::c_int
        } else { strategy = 0 as std::os::raw::c_int }
    } else {
        /* If customization is not supported the values all come from the
             * IDAT values except for the strategy, which is fixed to the
             * default.  (This is the pre-1.6.0 behavior too, although it was
             * implemented in a very different way.)
             */
        strategy = 0 as std::os::raw::c_int
    }
    /* Adjust 'windowBits' down if larger than 'data_size'; to stop this
       * happening just pass 32768 as the data_size parameter.  Notice that zlib
       * requires an extra 262 bytes in the window in addition to the data to be
       * able to see the whole of the data, so if data_size+262 takes us to the
       * next windowBits size we need to fix up the value later.  (Because even
       * though deflate needs the extra window, inflate does not!)
       */
    if data_size <= 16384 as std::os::raw::c_int as std::os::raw::c_ulong {
        /* IMPLEMENTATION NOTE: this 'half_window_size' stuff is only here to
          * work round a Microsoft Visual C misbehavior which, contrary to C-90,
          * widens the result of the following shift to 64-bits if (and,
          * apparently, only if) it is used in a test.
          */
        let mut half_window_size: std::os::raw::c_uint =
            (1 as std::os::raw::c_uint) << windowBits - 1 as std::os::raw::c_int;
        while data_size.wrapping_add(262 as std::os::raw::c_int as std::os::raw::c_ulong) <=
                  half_window_size as std::os::raw::c_ulong {
            half_window_size >>= 1 as std::os::raw::c_int;
            windowBits -= 1
        }
    }
    /* Check against the previous initialized values, if any. */
    if (*png_ptr).flags & 0x2 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint &&
           ((*png_ptr).zlib_set_level != level ||
                (*png_ptr).zlib_set_method != method ||
                (*png_ptr).zlib_set_window_bits != windowBits ||
                (*png_ptr).zlib_set_mem_level != memLevel ||
                (*png_ptr).zlib_set_strategy != strategy) {
        if deflateEnd(&mut (*png_ptr).zstream) != 0 as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"deflateEnd failed (ignored)\x00" as *const u8 as
                            *const std::os::raw::c_char);
        }
        (*png_ptr).flags &= !(0x2 as std::os::raw::c_uint)
    }
    /* For safety clear out the input and output pointers (currently zlib
       * doesn't use them on Init, but it might in the future).
       */
    (*png_ptr).zstream.next_in = 0 as *const Bytef;
    (*png_ptr).zstream.avail_in = 0 as std::os::raw::c_int as uInt;
    (*png_ptr).zstream.next_out = 0 as *mut Bytef;
    (*png_ptr).zstream.avail_out = 0 as std::os::raw::c_int as uInt;
    /* Now initialize if required, setting the new parameters, otherwise just
       * do a simple reset to the previous parameters.
       */
    if (*png_ptr).flags & 0x2 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        ret = deflateReset(&mut (*png_ptr).zstream)
    } else {
        ret =
            deflateInit2_(&mut (*png_ptr).zstream, level, method, windowBits,
                          memLevel, strategy,
                          b"1.2.11-optipng\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong
                              as std::os::raw::c_int);
        if ret == 0 as std::os::raw::c_int { (*png_ptr).flags |= 0x2 as std::os::raw::c_uint }
    }
    /* The return code is from either deflateReset or deflateInit2; they have
       * pretty much the same set of error codes.
       */
    if ret == 0 as std::os::raw::c_int {
        (*png_ptr).zowner = owner
    } else { png_zstream_error(png_ptr, ret); }
    return ret;
}
/* Clean up (or trim) a linked list of compression buffers. */
#[no_mangle]
pub unsafe extern "C" fn png_free_buffer_list(mut png_ptr: png_structrp,
                                              mut listp:
                                                  *mut png_compression_bufferp) {
    let mut list: png_compression_bufferp = *listp;
    if !list.is_null() {
        *listp = 0 as png_compression_bufferp;
        loop  {
            let mut next: png_compression_bufferp = (*list).next;
            png_free(png_ptr, list as png_voidp);
            list = next;
            if list.is_null() { break ; }
        }
    };
}
/* WRITE_COMPRESSED_TEXT */
/* Write the IHDR chunk, and update the png_struct with the necessary
 * information.  Note that the rest of this code depends upon this
 * information being correct.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_IHDR(mut png_ptr: png_structrp,
                                        mut width: png_uint_32,
                                        mut height: png_uint_32,
                                        mut bit_depth: std::os::raw::c_int,
                                        mut color_type: std::os::raw::c_int,
                                        mut compression_type: std::os::raw::c_int,
                                        mut filter_type: std::os::raw::c_int,
                                        mut interlace_type: std::os::raw::c_int) {
    let mut buf: [png_byte; 13] = [0; 13]; /* Buffer to store the IHDR info */
    let mut is_invalid_depth: std::os::raw::c_int = 0;
    /* Check that we have valid input data from the application info */
    match color_type {
        0 => {
            match bit_depth {
                1 | 2 | 4 | 8 | 16 => {
                    (*png_ptr).channels = 1 as std::os::raw::c_int as png_byte
                }
                _ => {
                    png_error(png_ptr,
                              b"Invalid bit depth for grayscale image\x00" as
                                  *const u8 as *const std::os::raw::c_char);
                }
            }
        }
        2 => {
            is_invalid_depth = (bit_depth != 8 as std::os::raw::c_int) as std::os::raw::c_int;
            is_invalid_depth =
                (is_invalid_depth != 0 && bit_depth != 16 as std::os::raw::c_int) as
                    std::os::raw::c_int;
            if is_invalid_depth != 0 {
                png_error(png_ptr,
                          b"Invalid bit depth for RGB image\x00" as *const u8
                              as *const std::os::raw::c_char);
            }
            (*png_ptr).channels = 3 as std::os::raw::c_int as png_byte
        }
        3 => {
            match bit_depth {
                1 | 2 | 4 | 8 => {
                    (*png_ptr).channels = 1 as std::os::raw::c_int as png_byte
                }
                _ => {
                    png_error(png_ptr,
                              b"Invalid bit depth for paletted image\x00" as
                                  *const u8 as *const std::os::raw::c_char);
                }
            }
        }
        4 => {
            is_invalid_depth = (bit_depth != 8 as std::os::raw::c_int) as std::os::raw::c_int;
            is_invalid_depth =
                (is_invalid_depth != 0 && bit_depth != 16 as std::os::raw::c_int) as
                    std::os::raw::c_int;
            if is_invalid_depth != 0 {
                png_error(png_ptr,
                          b"Invalid bit depth for grayscale+alpha image\x00"
                              as *const u8 as *const std::os::raw::c_char);
            }
            (*png_ptr).channels = 2 as std::os::raw::c_int as png_byte
        }
        6 => {
            is_invalid_depth = (bit_depth != 8 as std::os::raw::c_int) as std::os::raw::c_int;
            is_invalid_depth =
                (is_invalid_depth != 0 && bit_depth != 16 as std::os::raw::c_int) as
                    std::os::raw::c_int;
            if is_invalid_depth != 0 {
                png_error(png_ptr,
                          b"Invalid bit depth for RGBA image\x00" as *const u8
                              as *const std::os::raw::c_char);
            }
            (*png_ptr).channels = 4 as std::os::raw::c_int as png_byte
        }
        _ => {
            png_error(png_ptr,
                      b"Invalid image color type specified\x00" as *const u8
                          as *const std::os::raw::c_char);
        }
    }
    if compression_type != 0 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid compression type specified\x00" as *const u8 as
                        *const std::os::raw::c_char);
        compression_type = 0 as std::os::raw::c_int
    }
    /* Write filter_method 64 (intrapixel differencing) only if
    * 1. Libpng was compiled with PNG_MNG_FEATURES_SUPPORTED and
    * 2. Libpng did not write a PNG signature (this filter_method is only
    *    used in PNG datastreams that are embedded in MNG datastreams) and
    * 3. The application called png_permit_mng_features with a mask that
    *    included PNG_FLAG_MNG_FILTER_64 and
    * 4. The filter_method is 64 and
    * 5. The color_type is RGB or RGBA
    */
    if filter_type != 0 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid filter type specified\x00" as *const u8 as
                        *const std::os::raw::c_char);
        filter_type = 0 as std::os::raw::c_int
    }
    if interlace_type != 0 as std::os::raw::c_int &&
           interlace_type != 1 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid interlace type specified\x00" as *const u8 as
                        *const std::os::raw::c_char);
        interlace_type = 1 as std::os::raw::c_int
    }
    /* Save the relevant information */
    (*png_ptr).bit_depth = bit_depth as png_byte;
    (*png_ptr).color_type = color_type as png_byte;
    (*png_ptr).interlaced = interlace_type as png_byte;
    (*png_ptr).compression_type = compression_type as png_byte;
    (*png_ptr).width = width;
    (*png_ptr).height = height;
    (*png_ptr).pixel_depth =
        (bit_depth * (*png_ptr).channels as std::os::raw::c_int) as png_byte;
    (*png_ptr).rowbytes =
        if (*png_ptr).pixel_depth as std::os::raw::c_int >= 8 as std::os::raw::c_int {
            (width as
                 png_size_t).wrapping_mul((*png_ptr).pixel_depth as png_size_t
                                              >> 3 as std::os::raw::c_int)
        } else {
            ((width as
                  png_size_t).wrapping_mul((*png_ptr).pixel_depth as
                                               png_size_t).wrapping_add(7 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong))
                >> 3 as std::os::raw::c_int
        };
    /* Set the usr info, so any transformations can modify it */
    (*png_ptr).usr_width = (*png_ptr).width;
    (*png_ptr).usr_bit_depth = (*png_ptr).bit_depth;
    (*png_ptr).usr_channels = (*png_ptr).channels;
    /* Pack the header information into the buffer */
    png_save_uint_32(buf.as_mut_ptr(), width);
    png_save_uint_32(buf.as_mut_ptr().offset(4 as std::os::raw::c_int as isize),
                     height);
    buf[8 as std::os::raw::c_int as usize] = bit_depth as png_byte;
    buf[9 as std::os::raw::c_int as usize] = color_type as png_byte;
    buf[10 as std::os::raw::c_int as usize] = compression_type as png_byte;
    buf[11 as std::os::raw::c_int as usize] = filter_type as png_byte;
    buf[12 as std::os::raw::c_int as usize] = interlace_type as png_byte;
    /* Write the chunk */
    png_write_complete_chunk(png_ptr,
                             (73 as std::os::raw::c_int as png_uint_32) <<
                                 24 as std::os::raw::c_int |
                                 (72 as std::os::raw::c_int as png_uint_32) <<
                                     16 as std::os::raw::c_int |
                                 (68 as std::os::raw::c_int as png_uint_32) <<
                                     8 as std::os::raw::c_int |
                                 (82 as std::os::raw::c_int as png_uint_32) <<
                                     0 as std::os::raw::c_int,
                             buf.as_mut_ptr() as png_const_bytep,
                             13 as std::os::raw::c_int as png_size_t);
    if (*png_ptr).do_filter as std::os::raw::c_int == 0 as std::os::raw::c_int {
        if (*png_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int ||
               ((*png_ptr).bit_depth as std::os::raw::c_int) < 8 as std::os::raw::c_int {
            (*png_ptr).do_filter = 0x8 as std::os::raw::c_int as png_byte
        } else {
            (*png_ptr).do_filter =
                (0x8 as std::os::raw::c_int | 0x10 as std::os::raw::c_int |
                     0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                     0x80 as std::os::raw::c_int) as png_byte
        }
    }
    (*png_ptr).mode = 0x1 as std::os::raw::c_int as png_uint_32;
    /* not READY_FOR_ZTXT */
}
/* Write the palette.  We are careful not to trust png_color to be in the
 * correct order for PNG, so people can redefine it to any convenient
 * structure.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_PLTE(mut png_ptr: png_structrp,
                                        mut palette: png_const_colorp,
                                        mut num_pal: png_uint_32) {
    let mut max_palette_length: png_uint_32 = 0;
    let mut i: png_uint_32 = 0;
    let mut pal_ptr: png_const_colorp = 0 as *const png_color;
    let mut buf: [png_byte; 3] = [0; 3];
    max_palette_length =
        if (*png_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            ((1 as std::os::raw::c_int)) << (*png_ptr).bit_depth as std::os::raw::c_int
        } else { 256 as std::os::raw::c_int } as png_uint_32;
    if num_pal == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           num_pal > max_palette_length {
        if (*png_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            png_error(png_ptr,
                      b"Invalid number of colors in palette\x00" as *const u8
                          as *const std::os::raw::c_char);
        } else {
            png_warning(png_ptr,
                        b"Invalid number of colors in palette\x00" as
                            *const u8 as *const std::os::raw::c_char);
            return
        }
    }
    if (*png_ptr).color_type as std::os::raw::c_int & 2 as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Ignoring request to write a PLTE chunk in grayscale PNG\x00"
                        as *const u8 as *const std::os::raw::c_char);
        return
    }
    (*png_ptr).num_palette = num_pal as png_uint_16;
    png_write_chunk_header(png_ptr,
                           (80 as std::os::raw::c_int as png_uint_32) <<
                               24 as std::os::raw::c_int |
                               (76 as std::os::raw::c_int as png_uint_32) <<
                                   16 as std::os::raw::c_int |
                               (84 as std::os::raw::c_int as png_uint_32) <<
                                   8 as std::os::raw::c_int |
                               (69 as std::os::raw::c_int as png_uint_32) <<
                                   0 as std::os::raw::c_int,
                           num_pal.wrapping_mul(3 as std::os::raw::c_int as
                                                    std::os::raw::c_uint));
    i = 0 as std::os::raw::c_int as png_uint_32;
    pal_ptr = palette;
    while i < num_pal {
        buf[0 as std::os::raw::c_int as usize] = (*pal_ptr).red;
        buf[1 as std::os::raw::c_int as usize] = (*pal_ptr).green;
        buf[2 as std::os::raw::c_int as usize] = (*pal_ptr).blue;
        png_write_chunk_data(png_ptr, buf.as_mut_ptr() as png_const_bytep,
                             3 as std::os::raw::c_int as png_size_t);
        i = i.wrapping_add(1);
        pal_ptr = pal_ptr.offset(1)
    }
    png_write_chunk_end(png_ptr);
    (*png_ptr).mode |= 0x2 as std::os::raw::c_int as std::os::raw::c_uint;
}
/* This is similar to png_text_compress, above, except that it does not require
 * all of the data at once and, instead of buffering the compressed result,
 * writes it as IDAT chunks.  Unlike png_text_compress it *can* png_error out
 * because it calls the write interface.  As a result it does its own error
 * reporting and does not return an error code.  In the event of error it will
 * just call png_error.  The input data length may exceed 32-bits.  The 'flush'
 * parameter is exactly the same as that to deflate, with the following
 * meanings:
 *
 * Z_NO_FLUSH: normal incremental output of compressed data
 * Z_SYNC_FLUSH: do a SYNC_FLUSH, used by png_write_flush
 * Z_FINISH: this is the end of the input, do a Z_FINISH and clean up
 *
 * The routine manages the acquire and release of the png_ptr->zstream by
 * checking and (at the end) clearing png_ptr->zowner; it does some sanity
 * checks on the 'mode' flags while doing this.
 */
#[no_mangle]
pub unsafe extern "C" fn png_compress_IDAT(mut png_ptr: png_structrp,
                                           mut input: png_const_bytep,
                                           mut input_len: png_alloc_size_t,
                                           mut flush: std::os::raw::c_int) {
    if (*png_ptr).zowner !=
           (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
               (68 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
               (65 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
               (84 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
        /* First time.   Ensure we have a temporary buffer for compression and
       * trim the buffer list if it has more than one entry to free memory.
       * If 'WRITE_COMPRESSED_TEXT' is not set the list will never have been
       * created at this point, but the check here is quick and safe.
       */
        if (*png_ptr).zbuffer_list.is_null() {
            (*png_ptr).zbuffer_list =
                png_malloc(png_ptr,
                           (8 as
                                std::os::raw::c_ulong).wrapping_add((*png_ptr).zbuffer_size
                                                                as
                                                                std::os::raw::c_ulong))
                    as png_compression_bufferp;
            (*(*png_ptr).zbuffer_list).next = 0 as *mut png_compression_buffer
        } else {
            png_free_buffer_list(png_ptr,
                                 &mut (*(*png_ptr).zbuffer_list).next);
        }
        /* It is a terminal error if we can't claim the zstream. */
        if png_deflate_claim(png_ptr,
                             (73 as std::os::raw::c_int as png_uint_32) <<
                                 24 as std::os::raw::c_int |
                                 (68 as std::os::raw::c_int as png_uint_32) <<
                                     16 as std::os::raw::c_int |
                                 (65 as std::os::raw::c_int as png_uint_32) <<
                                     8 as std::os::raw::c_int |
                                 (84 as std::os::raw::c_int as png_uint_32) <<
                                     0 as std::os::raw::c_int,
                             png_image_size(png_ptr)) != 0 as std::os::raw::c_int {
            png_error(png_ptr, (*png_ptr).zstream.msg);
        }
        /* The output state is maintained in png_ptr->zstream, so it must be
       * initialized here after the claim.
       */
        (*png_ptr).zstream.next_out =
            (*(*png_ptr).zbuffer_list).output.as_mut_ptr();
        (*png_ptr).zstream.avail_out = (*png_ptr).zbuffer_size
    }
    /* Now loop reading and writing until all the input is consumed or an error
    * terminates the operation.  The _out values are maintained across calls to
    * this function, but the input must be reset each time.
    */
    (*png_ptr).zstream.next_in = input; /* set below */
    (*png_ptr).zstream.avail_in = 0 as std::os::raw::c_int as uInt;
    loop  {
        let mut ret: std::os::raw::c_int = 0;
        /* INPUT: from the row data */
        let mut avail: uInt =
            -(1 as std::os::raw::c_int) as uInt; /* safe because of the check */
        if avail as std::os::raw::c_ulong > input_len { avail = input_len as uInt }
        (*png_ptr).zstream.avail_in = avail;
        input_len =
            (input_len as std::os::raw::c_ulong).wrapping_sub(avail as std::os::raw::c_ulong)
                as png_alloc_size_t as png_alloc_size_t;
        ret =
            deflate(&mut (*png_ptr).zstream,
                    if input_len > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        0 as std::os::raw::c_int
                    } else { flush });
        /* Include as-yet unconsumed input */
        input_len =
            (input_len as
                 std::os::raw::c_ulong).wrapping_add((*png_ptr).zstream.avail_in as
                                                 std::os::raw::c_ulong) as
                png_alloc_size_t as png_alloc_size_t;
        (*png_ptr).zstream.avail_in = 0 as std::os::raw::c_int as uInt;
        /* OUTPUT: write complete IDAT chunks when avail_out drops to zero. Note
       * that these two zstream fields are preserved across the calls, therefore
       * there is no need to set these up on entry to the loop.
       */
        if (*png_ptr).zstream.avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut data: png_bytep =
                (*(*png_ptr).zbuffer_list).output.as_mut_ptr();
            let mut size: uInt = (*png_ptr).zbuffer_size;
            /* Write an IDAT containing the data then reset the buffer.  The
          * first IDAT may need deflate header optimization.
          */
            if (*png_ptr).mode & 0x4 as std::os::raw::c_uint ==
                   0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*png_ptr).compression_type as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                optimize_cmf(data, png_image_size(png_ptr));
            }
            if size > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                png_write_complete_chunk(png_ptr,
                                         (73 as std::os::raw::c_int as png_uint_32) <<
                                             24 as std::os::raw::c_int |
                                             (68 as std::os::raw::c_int as
                                                  png_uint_32) <<
                                                 16 as std::os::raw::c_int |
                                             (65 as std::os::raw::c_int as
                                                  png_uint_32) <<
                                                 8 as std::os::raw::c_int |
                                             (84 as std::os::raw::c_int as
                                                  png_uint_32) <<
                                                 0 as std::os::raw::c_int,
                                         data as png_const_bytep,
                                         size as png_size_t);
            }
            (*png_ptr).mode |= 0x4 as std::os::raw::c_uint;
            (*png_ptr).zstream.next_out = data;
            (*png_ptr).zstream.avail_out = size;
            /* For SYNC_FLUSH or FINISH it is essential to keep calling zlib with
          * the same flush parameter until it has finished output, for NO_FLUSH
          * it doesn't matter.
          */
            if ret == 0 as std::os::raw::c_int && flush != 0 as std::os::raw::c_int {
                continue ;
            }
        }
        /* The order of these checks doesn't matter much; it just affects which
       * possible error might be detected if multiple things go wrong at once.
       */
        if ret == 0 as std::os::raw::c_int {
            /* most likely return code! */
            /* If all the input has been consumed then just return.  If Z_FINISH
          * was used as the flush parameter something has gone wrong if we get
          * here.
          */
            if input_len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                if flush == 4 as std::os::raw::c_int {
                    png_error(png_ptr,
                              b"Z_OK on Z_FINISH with output space\x00" as
                                  *const u8 as *const std::os::raw::c_char);
                }
                return
            }
        } else if ret == 1 as std::os::raw::c_int && flush == 4 as std::os::raw::c_int {
            /* This is the end of the IDAT data; any pending output must be
          * flushed.  For small PNG files we may still be at the beginning.
          */
            let mut data_0: png_bytep =
                (*(*png_ptr).zbuffer_list).output.as_mut_ptr(); /* Release the stream */
            let mut size_0: uInt =
                (*png_ptr).zbuffer_size.wrapping_sub((*png_ptr).zstream.avail_out);
            if (*png_ptr).mode & 0x4 as std::os::raw::c_uint ==
                   0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*png_ptr).compression_type as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                optimize_cmf(data_0, png_image_size(png_ptr));
            }
            if size_0 > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                png_write_complete_chunk(png_ptr,
                                         (73 as std::os::raw::c_int as png_uint_32) <<
                                             24 as std::os::raw::c_int |
                                             (68 as std::os::raw::c_int as
                                                  png_uint_32) <<
                                                 16 as std::os::raw::c_int |
                                             (65 as std::os::raw::c_int as
                                                  png_uint_32) <<
                                                 8 as std::os::raw::c_int |
                                             (84 as std::os::raw::c_int as
                                                  png_uint_32) <<
                                                 0 as std::os::raw::c_int,
                                         data_0 as png_const_bytep,
                                         size_0 as png_size_t);
            }
            (*png_ptr).zstream.avail_out = 0 as std::os::raw::c_int as uInt;
            (*png_ptr).zstream.next_out = 0 as *mut Bytef;
            (*png_ptr).mode |=
                0x4 as std::os::raw::c_uint | 0x8 as std::os::raw::c_int as std::os::raw::c_uint;
            (*png_ptr).zowner = 0 as std::os::raw::c_int as png_uint_32;
            return
        } else {
            /* This is an error condition. */
            png_zstream_error(png_ptr, ret);
            png_error(png_ptr, (*png_ptr).zstream.msg);
        }
    };
}
/* Write an IEND chunk */
#[no_mangle]
pub unsafe extern "C" fn png_write_IEND(mut png_ptr: png_structrp) {
    png_write_complete_chunk(png_ptr,
                             (73 as std::os::raw::c_int as png_uint_32) <<
                                 24 as std::os::raw::c_int |
                                 (69 as std::os::raw::c_int as png_uint_32) <<
                                     16 as std::os::raw::c_int |
                                 (78 as std::os::raw::c_int as png_uint_32) <<
                                     8 as std::os::raw::c_int |
                                 (68 as std::os::raw::c_int as png_uint_32) <<
                                     0 as std::os::raw::c_int, 0 as png_const_bytep,
                             0 as std::os::raw::c_int as png_size_t);
    (*png_ptr).mode |= 0x10 as std::os::raw::c_uint;
}
/* Write the sBIT chunk */
#[no_mangle]
pub unsafe extern "C" fn png_write_sBIT(mut png_ptr: png_structrp,
                                        mut sbit: png_const_color_8p,
                                        mut color_type: std::os::raw::c_int) {
    let mut buf: [png_byte; 4] = [0; 4];
    let mut size: png_size_t = 0;
    /* Make sure we don't depend upon the order of PNG_COLOR_8 */
    if color_type & 2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        let mut maxbits: png_byte = 0;
        maxbits =
            if color_type == 2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
                8 as std::os::raw::c_int
            } else { (*png_ptr).usr_bit_depth as std::os::raw::c_int } as png_byte;
        if (*sbit).red as std::os::raw::c_int == 0 as std::os::raw::c_int ||
               (*sbit).red as std::os::raw::c_int > maxbits as std::os::raw::c_int ||
               (*sbit).green as std::os::raw::c_int == 0 as std::os::raw::c_int ||
               (*sbit).green as std::os::raw::c_int > maxbits as std::os::raw::c_int ||
               (*sbit).blue as std::os::raw::c_int == 0 as std::os::raw::c_int ||
               (*sbit).blue as std::os::raw::c_int > maxbits as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"Invalid sBIT depth specified\x00" as *const u8 as
                            *const std::os::raw::c_char);
            return
        }
        buf[0 as std::os::raw::c_int as usize] = (*sbit).red;
        buf[1 as std::os::raw::c_int as usize] = (*sbit).green;
        buf[2 as std::os::raw::c_int as usize] = (*sbit).blue;
        size = 3 as std::os::raw::c_int as png_size_t
    } else {
        if (*sbit).gray as std::os::raw::c_int == 0 as std::os::raw::c_int ||
               (*sbit).gray as std::os::raw::c_int >
                   (*png_ptr).usr_bit_depth as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"Invalid sBIT depth specified\x00" as *const u8 as
                            *const std::os::raw::c_char);
            return
        }
        buf[0 as std::os::raw::c_int as usize] = (*sbit).gray;
        size = 1 as std::os::raw::c_int as png_size_t
    }
    if color_type & 4 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if (*sbit).alpha as std::os::raw::c_int == 0 as std::os::raw::c_int ||
               (*sbit).alpha as std::os::raw::c_int >
                   (*png_ptr).usr_bit_depth as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"Invalid sBIT depth specified\x00" as *const u8 as
                            *const std::os::raw::c_char);
            return
        }
        let fresh0 = size;
        size = size.wrapping_add(1);
        buf[fresh0 as usize] = (*sbit).alpha
    }
    png_write_complete_chunk(png_ptr,
                             (115 as std::os::raw::c_int as png_uint_32) <<
                                 24 as std::os::raw::c_int |
                                 (66 as std::os::raw::c_int as png_uint_32) <<
                                     16 as std::os::raw::c_int |
                                 (73 as std::os::raw::c_int as png_uint_32) <<
                                     8 as std::os::raw::c_int |
                                 (84 as std::os::raw::c_int as png_uint_32) <<
                                     0 as std::os::raw::c_int,
                             buf.as_mut_ptr() as png_const_bytep, size);
}
/* Write the tRNS chunk */
#[no_mangle]
pub unsafe extern "C" fn png_write_tRNS(mut png_ptr: png_structrp,
                                        mut trans_alpha: png_const_bytep,
                                        mut tran: png_const_color_16p,
                                        mut num_trans: std::os::raw::c_int,
                                        mut color_type: std::os::raw::c_int) {
    let mut buf: [png_byte; 6] = [0; 6];
    if color_type == 2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        if num_trans <= 0 as std::os::raw::c_int ||
               num_trans > (*png_ptr).num_palette as std::os::raw::c_int {
            png_app_warning(png_ptr,
                            b"Invalid number of transparent colors specified\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return
        }
        /* Write the chunk out as it is */
        png_write_complete_chunk(png_ptr,
                                 (116 as std::os::raw::c_int as png_uint_32) <<
                                     24 as std::os::raw::c_int |
                                     (82 as std::os::raw::c_int as png_uint_32) <<
                                         16 as std::os::raw::c_int |
                                     (78 as std::os::raw::c_int as png_uint_32) <<
                                         8 as std::os::raw::c_int |
                                     (83 as std::os::raw::c_int as png_uint_32) <<
                                         0 as std::os::raw::c_int, trans_alpha,
                                 num_trans as png_size_t);
    } else if color_type == 0 as std::os::raw::c_int {
        /* One 16-bit value */
        if (*tran).gray as std::os::raw::c_int >=
               (1 as std::os::raw::c_int) << (*png_ptr).bit_depth as std::os::raw::c_int {
            png_app_warning(png_ptr,
                            b"Ignoring attempt to write tRNS chunk out-of-range for bit_depth\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return
        }
        png_save_uint_16(buf.as_mut_ptr(), (*tran).gray as std::os::raw::c_uint);
        png_write_complete_chunk(png_ptr,
                                 (116 as std::os::raw::c_int as png_uint_32) <<
                                     24 as std::os::raw::c_int |
                                     (82 as std::os::raw::c_int as png_uint_32) <<
                                         16 as std::os::raw::c_int |
                                     (78 as std::os::raw::c_int as png_uint_32) <<
                                         8 as std::os::raw::c_int |
                                     (83 as std::os::raw::c_int as png_uint_32) <<
                                         0 as std::os::raw::c_int,
                                 buf.as_mut_ptr() as png_const_bytep,
                                 2 as std::os::raw::c_int as png_size_t);
    } else if color_type == 2 as std::os::raw::c_int {
        /* Three 16-bit values */
        png_save_uint_16(buf.as_mut_ptr(), (*tran).red as std::os::raw::c_uint);
        png_save_uint_16(buf.as_mut_ptr().offset(2 as std::os::raw::c_int as isize),
                         (*tran).green as std::os::raw::c_uint);
        png_save_uint_16(buf.as_mut_ptr().offset(4 as std::os::raw::c_int as isize),
                         (*tran).blue as std::os::raw::c_uint);
        if (*png_ptr).bit_depth as std::os::raw::c_int == 8 as std::os::raw::c_int &&
               buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int |
                   buf[2 as std::os::raw::c_int as usize] as std::os::raw::c_int |
                   buf[4 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                   0 as std::os::raw::c_int {
            png_app_warning(png_ptr,
                            b"Ignoring attempt to write 16-bit tRNS chunk when bit_depth is 8\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return
        }
        png_write_complete_chunk(png_ptr,
                                 (116 as std::os::raw::c_int as png_uint_32) <<
                                     24 as std::os::raw::c_int |
                                     (82 as std::os::raw::c_int as png_uint_32) <<
                                         16 as std::os::raw::c_int |
                                     (78 as std::os::raw::c_int as png_uint_32) <<
                                         8 as std::os::raw::c_int |
                                     (83 as std::os::raw::c_int as png_uint_32) <<
                                         0 as std::os::raw::c_int,
                                 buf.as_mut_ptr() as png_const_bytep,
                                 6 as std::os::raw::c_int as png_size_t);
    } else {
        png_app_warning(png_ptr,
                        b"Can\'t write tRNS with an alpha channel\x00" as
                            *const u8 as *const std::os::raw::c_char);
    };
}
/* Write the background chunk */
#[no_mangle]
pub unsafe extern "C" fn png_write_bKGD(mut png_ptr: png_structrp,
                                        mut back: png_const_color_16p,
                                        mut color_type: std::os::raw::c_int) {
    let mut buf: [png_byte; 6] = [0; 6];
    if color_type == 2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        if (*back).index as std::os::raw::c_int >=
               (*png_ptr).num_palette as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"Invalid background palette index\x00" as *const u8
                            as *const std::os::raw::c_char);
            return
        }
        buf[0 as std::os::raw::c_int as usize] = (*back).index;
        png_write_complete_chunk(png_ptr,
                                 (98 as std::os::raw::c_int as png_uint_32) <<
                                     24 as std::os::raw::c_int |
                                     (75 as std::os::raw::c_int as png_uint_32) <<
                                         16 as std::os::raw::c_int |
                                     (71 as std::os::raw::c_int as png_uint_32) <<
                                         8 as std::os::raw::c_int |
                                     (68 as std::os::raw::c_int as png_uint_32) <<
                                         0 as std::os::raw::c_int,
                                 buf.as_mut_ptr() as png_const_bytep,
                                 1 as std::os::raw::c_int as png_size_t);
    } else if color_type & 2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_save_uint_16(buf.as_mut_ptr(), (*back).red as std::os::raw::c_uint);
        png_save_uint_16(buf.as_mut_ptr().offset(2 as std::os::raw::c_int as isize),
                         (*back).green as std::os::raw::c_uint);
        png_save_uint_16(buf.as_mut_ptr().offset(4 as std::os::raw::c_int as isize),
                         (*back).blue as std::os::raw::c_uint);
        if (*png_ptr).bit_depth as std::os::raw::c_int == 8 as std::os::raw::c_int &&
               buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int |
                   buf[2 as std::os::raw::c_int as usize] as std::os::raw::c_int |
                   buf[4 as std::os::raw::c_int as usize] as std::os::raw::c_int !=
                   0 as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"Ignoring attempt to write 16-bit bKGD chunk when bit_depth is 8\x00"
                            as *const u8 as *const std::os::raw::c_char);
            return
        }
        png_write_complete_chunk(png_ptr,
                                 (98 as std::os::raw::c_int as png_uint_32) <<
                                     24 as std::os::raw::c_int |
                                     (75 as std::os::raw::c_int as png_uint_32) <<
                                         16 as std::os::raw::c_int |
                                     (71 as std::os::raw::c_int as png_uint_32) <<
                                         8 as std::os::raw::c_int |
                                     (68 as std::os::raw::c_int as png_uint_32) <<
                                         0 as std::os::raw::c_int,
                                 buf.as_mut_ptr() as png_const_bytep,
                                 6 as std::os::raw::c_int as png_size_t);
    } else {
        if (*back).gray as std::os::raw::c_int >=
               (1 as std::os::raw::c_int) << (*png_ptr).bit_depth as std::os::raw::c_int {
            png_warning(png_ptr,
                        b"Ignoring attempt to write bKGD chunk out-of-range for bit_depth\x00"
                            as *const u8 as *const std::os::raw::c_char);
            return
        }
        png_save_uint_16(buf.as_mut_ptr(), (*back).gray as std::os::raw::c_uint);
        png_write_complete_chunk(png_ptr,
                                 (98 as std::os::raw::c_int as png_uint_32) <<
                                     24 as std::os::raw::c_int |
                                     (75 as std::os::raw::c_int as png_uint_32) <<
                                         16 as std::os::raw::c_int |
                                     (71 as std::os::raw::c_int as png_uint_32) <<
                                         8 as std::os::raw::c_int |
                                     (68 as std::os::raw::c_int as png_uint_32) <<
                                         0 as std::os::raw::c_int,
                                 buf.as_mut_ptr() as png_const_bytep,
                                 2 as std::os::raw::c_int as png_size_t);
    };
}
/* Write the histogram */
#[no_mangle]
pub unsafe extern "C" fn png_write_hIST(mut png_ptr: png_structrp,
                                        mut hist: png_const_uint_16p,
                                        mut num_hist: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut buf: [png_byte; 3] = [0; 3];
    if num_hist > (*png_ptr).num_palette as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid number of histogram entries specified\x00" as
                        *const u8 as *const std::os::raw::c_char);
        return
    }
    png_write_chunk_header(png_ptr,
                           (104 as std::os::raw::c_int as png_uint_32) <<
                               24 as std::os::raw::c_int |
                               (73 as std::os::raw::c_int as png_uint_32) <<
                                   16 as std::os::raw::c_int |
                               (83 as std::os::raw::c_int as png_uint_32) <<
                                   8 as std::os::raw::c_int |
                               (84 as std::os::raw::c_int as png_uint_32) <<
                                   0 as std::os::raw::c_int,
                           (num_hist * 2 as std::os::raw::c_int) as png_uint_32);
    i = 0 as std::os::raw::c_int;
    while i < num_hist {
        png_save_uint_16(buf.as_mut_ptr(),
                         *hist.offset(i as isize) as std::os::raw::c_uint);
        png_write_chunk_data(png_ptr, buf.as_mut_ptr() as png_const_bytep,
                             2 as std::os::raw::c_int as png_size_t);
        i += 1
    }
    png_write_chunk_end(png_ptr);
}
/* Initializes the row writing capability of libpng */
#[no_mangle]
pub unsafe extern "C" fn png_write_start_row(mut png_ptr: png_structrp) {
    /* Arrays to facilitate easy interlacing - use pass (0 - 6) as index */
    /* Start of interlace block */
    static mut png_pass_start: [png_byte; 7] =
        [0 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 1 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte];
    /* Offset to next interlace block */
    static mut png_pass_inc: [png_byte; 7] =
        [8 as std::os::raw::c_int as png_byte, 8 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         1 as std::os::raw::c_int as png_byte];
    /* Start of interlace block in the y direction */
    static mut png_pass_ystart: [png_byte; 7] =
        [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         1 as std::os::raw::c_int as png_byte];
    /* Offset to next interlace block in the y direction */
    static mut png_pass_yinc: [png_byte; 7] =
        [8 as std::os::raw::c_int as png_byte, 8 as std::os::raw::c_int as png_byte,
         8 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte];
    let mut buf_size: png_alloc_size_t = 0;
    let mut usr_pixel_depth: std::os::raw::c_int = 0;
    let mut filters: png_byte = 0;
    usr_pixel_depth =
        (*png_ptr).usr_channels as std::os::raw::c_int *
            (*png_ptr).usr_bit_depth as std::os::raw::c_int;
    buf_size =
        (if usr_pixel_depth >= 8 as std::os::raw::c_int {
             ((*png_ptr).width as
                  png_size_t).wrapping_mul(usr_pixel_depth as png_size_t >>
                                               3 as std::os::raw::c_int)
         } else {
             (((*png_ptr).width as
                   png_size_t).wrapping_mul(usr_pixel_depth as
                                                png_size_t).wrapping_add(7 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_ulong))
                 >> 3 as std::os::raw::c_int
         }).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    /* 1.5.6: added to allow checking in the row write code. */
    (*png_ptr).transformed_pixel_depth = (*png_ptr).pixel_depth;
    (*png_ptr).maximum_pixel_depth = usr_pixel_depth as png_byte;
    /* Set up row buffer */
    (*png_ptr).row_buf = png_malloc(png_ptr, buf_size) as png_bytep;
    *(*png_ptr).row_buf.offset(0 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as png_byte;
    filters = (*png_ptr).do_filter;
    if (*png_ptr).height == 1 as std::os::raw::c_int as std::os::raw::c_uint {
        filters =
            (filters as std::os::raw::c_int &
                 (0xff as std::os::raw::c_int &
                      !(0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                            0x80 as std::os::raw::c_int))) as png_byte
    }
    if (*png_ptr).width == 1 as std::os::raw::c_int as std::os::raw::c_uint {
        filters =
            (filters as std::os::raw::c_int &
                 (0xff as std::os::raw::c_int &
                      !(0x10 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                            0x80 as std::os::raw::c_int))) as png_byte
    }
    if filters as std::os::raw::c_int == 0 as std::os::raw::c_int {
        filters = 0x8 as std::os::raw::c_int as png_byte
    }
    (*png_ptr).do_filter = filters;
    if filters as std::os::raw::c_int &
           (0x10 as std::os::raw::c_int | 0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                0x80 as std::os::raw::c_int) != 0 as std::os::raw::c_int &&
           (*png_ptr).try_row.is_null() {
        let mut num_filters: std::os::raw::c_int = 0 as std::os::raw::c_int;
        (*png_ptr).try_row = png_malloc(png_ptr, buf_size) as png_bytep;
        if filters as std::os::raw::c_int & 0x10 as std::os::raw::c_int != 0 {
            num_filters += 1
        }
        if filters as std::os::raw::c_int & 0x20 as std::os::raw::c_int != 0 {
            num_filters += 1
        }
        if filters as std::os::raw::c_int & 0x40 as std::os::raw::c_int != 0 {
            num_filters += 1
        }
        if filters as std::os::raw::c_int & 0x80 as std::os::raw::c_int != 0 {
            num_filters += 1
        }
        if num_filters > 1 as std::os::raw::c_int {
            (*png_ptr).tst_row = png_malloc(png_ptr, buf_size) as png_bytep
        }
    }
    /* We only need to keep the previous row if we are using one of the following
    * filters.
    */
    if filters as std::os::raw::c_int &
           (0x40 as std::os::raw::c_int | 0x20 as std::os::raw::c_int | 0x80 as std::os::raw::c_int)
           != 0 as std::os::raw::c_int {
        (*png_ptr).prev_row = png_calloc(png_ptr, buf_size) as png_bytep
    }
    /* WRITE_FILTER */
    /* If interlaced, we need to set up width and height of pass */
    if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if (*png_ptr).transformations & 0x2 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*png_ptr).num_rows =
                (*png_ptr).height.wrapping_add(png_pass_yinc[0 as std::os::raw::c_int
                                                                 as usize] as
                                                   std::os::raw::c_uint).wrapping_sub(1
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_uint).wrapping_sub(png_pass_ystart[0
                                                                                                                                 as
                                                                                                                                 std::os::raw::c_int
                                                                                                                                 as
                                                                                                                                 usize]
                                                                                                                 as
                                                                                                                 std::os::raw::c_uint).wrapping_div(png_pass_yinc[0
                                                                                                                                                              as
                                                                                                                                                              std::os::raw::c_int
                                                                                                                                                              as
                                                                                                                                                              usize]
                                                                                                                                                as
                                                                                                                                                std::os::raw::c_uint);
            (*png_ptr).usr_width =
                (*png_ptr).width.wrapping_add(png_pass_inc[0 as std::os::raw::c_int as
                                                               usize] as
                                                  std::os::raw::c_uint).wrapping_sub(1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 std::os::raw::c_uint).wrapping_sub(png_pass_start[0
                                                                                                                               as
                                                                                                                               std::os::raw::c_int
                                                                                                                               as
                                                                                                                               usize]
                                                                                                                as
                                                                                                                std::os::raw::c_uint).wrapping_div(png_pass_inc[0
                                                                                                                                                            as
                                                                                                                                                            std::os::raw::c_int
                                                                                                                                                            as
                                                                                                                                                            usize]
                                                                                                                                               as
                                                                                                                                               std::os::raw::c_uint)
        } else {
            (*png_ptr).num_rows = (*png_ptr).height;
            (*png_ptr).usr_width = (*png_ptr).width
        }
    } else {
        (*png_ptr).num_rows = (*png_ptr).height;
        (*png_ptr).usr_width = (*png_ptr).width
    };
}
/* Internal use only.  Called when finished processing a row of data. */
#[no_mangle]
pub unsafe extern "C" fn png_write_finish_row(mut png_ptr: png_structrp) {
    /* Arrays to facilitate easy interlacing - use pass (0 - 6) as index */
    /* Start of interlace block */
    static mut png_pass_start: [png_byte; 7] =
        [0 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 1 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte];
    /* Offset to next interlace block */
    static mut png_pass_inc: [png_byte; 7] =
        [8 as std::os::raw::c_int as png_byte, 8 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         1 as std::os::raw::c_int as png_byte];
    /* Start of interlace block in the y direction */
    static mut png_pass_ystart: [png_byte; 7] =
        [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         1 as std::os::raw::c_int as png_byte];
    /* Offset to next interlace block in the y direction */
    static mut png_pass_yinc: [png_byte; 7] =
        [8 as std::os::raw::c_int as png_byte, 8 as std::os::raw::c_int as png_byte,
         8 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte];
    /* Next row */
    (*png_ptr).row_number = (*png_ptr).row_number.wrapping_add(1);
    /* See if we are done */
    if (*png_ptr).row_number < (*png_ptr).num_rows { return }
    /* If interlaced, go to next pass */
    if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int {
        (*png_ptr).row_number = 0 as std::os::raw::c_int as png_uint_32;
        if (*png_ptr).transformations & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*png_ptr).pass = (*png_ptr).pass.wrapping_add(1)
        } else {
            loop 
                 /* Loop until we find a non-zero width or height pass */
                 {
                (*png_ptr).pass = (*png_ptr).pass.wrapping_add(1);
                if (*png_ptr).pass as std::os::raw::c_int >= 7 as std::os::raw::c_int {
                    break ;
                }
                (*png_ptr).usr_width =
                    (*png_ptr).width.wrapping_add(png_pass_inc[(*png_ptr).pass
                                                                   as usize]
                                                      as
                                                      std::os::raw::c_uint).wrapping_sub(1
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_uint).wrapping_sub(png_pass_start[(*png_ptr).pass
                                                                                                                                   as
                                                                                                                                   usize]
                                                                                                                    as
                                                                                                                    std::os::raw::c_uint).wrapping_div(png_pass_inc[(*png_ptr).pass
                                                                                                                                                                as
                                                                                                                                                                usize]
                                                                                                                                                   as
                                                                                                                                                   std::os::raw::c_uint);
                (*png_ptr).num_rows =
                    (*png_ptr).height.wrapping_add(png_pass_yinc[(*png_ptr).pass
                                                                     as usize]
                                                       as
                                                       std::os::raw::c_uint).wrapping_sub(1
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      as
                                                                                      std::os::raw::c_uint).wrapping_sub(png_pass_ystart[(*png_ptr).pass
                                                                                                                                     as
                                                                                                                                     usize]
                                                                                                                     as
                                                                                                                     std::os::raw::c_uint).wrapping_div(png_pass_yinc[(*png_ptr).pass
                                                                                                                                                                  as
                                                                                                                                                                  usize]
                                                                                                                                                    as
                                                                                                                                                    std::os::raw::c_uint);
                if (*png_ptr).transformations & 0x2 as std::os::raw::c_uint !=
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    break ;
                }
                if !((*png_ptr).usr_width == 0 as std::os::raw::c_int as std::os::raw::c_uint
                         ||
                         (*png_ptr).num_rows ==
                             0 as std::os::raw::c_int as std::os::raw::c_uint) {
                    break ;
                }
            }
        }
        /* Reset the row above the image for the next pass */
        if ((*png_ptr).pass as std::os::raw::c_int) < 7 as std::os::raw::c_int {
            if !(*png_ptr).prev_row.is_null() {
                memset((*png_ptr).prev_row as *mut std::os::raw::c_void,
                       0 as std::os::raw::c_int,
                       (if (*png_ptr).usr_channels as std::os::raw::c_int *
                               (*png_ptr).usr_bit_depth as std::os::raw::c_int >=
                               8 as std::os::raw::c_int {
                            ((*png_ptr).width as
                                 png_size_t).wrapping_mul(((*png_ptr).usr_channels
                                                               as std::os::raw::c_int
                                                               *
                                                               (*png_ptr).usr_bit_depth
                                                                   as
                                                                   std::os::raw::c_int)
                                                              as png_size_t >>
                                                              3 as
                                                                  std::os::raw::c_int)
                        } else {
                            (((*png_ptr).width as
                                  png_size_t).wrapping_mul(((*png_ptr).usr_channels
                                                                as std::os::raw::c_int
                                                                *
                                                                (*png_ptr).usr_bit_depth
                                                                    as
                                                                    std::os::raw::c_int)
                                                               as
                                                               png_size_t).wrapping_add(7
                                                                                            as
                                                                                            std::os::raw::c_int
                                                                                            as
                                                                                            std::os::raw::c_ulong))
                                >> 3 as std::os::raw::c_int
                        }).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
            }
            return
        }
    }
    /* If we get here, we've just written the last row, so we need
      to flush the compressor */
    png_compress_IDAT(png_ptr, 0 as png_const_bytep,
                      0 as std::os::raw::c_int as png_alloc_size_t, 4 as std::os::raw::c_int);
}
/* Pick out the correct pixels for the interlace pass.
 * The basic idea here is to go through the row with a source
 * pointer and a destination pointer (sp and dp), and copy the
 * correct pixels for the pass.  As the row gets compacted,
 * sp will always be >= dp, so we should never overwrite anything.
 * See the default: case for the easiest code to understand.
 */
#[no_mangle]
pub unsafe extern "C" fn png_do_write_interlace(mut row_info: png_row_infop,
                                                mut row: png_bytep,
                                                mut pass: std::os::raw::c_int) {
    /* Arrays to facilitate easy interlacing - use pass (0 - 6) as index */
    /* Start of interlace block */
    static mut png_pass_start: [png_byte; 7] =
        [0 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 1 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte];
    /* Offset to next interlace block */
    static mut png_pass_inc: [png_byte; 7] =
        [8 as std::os::raw::c_int as png_byte, 8 as std::os::raw::c_int as png_byte,
         4 as std::os::raw::c_int as png_byte, 4 as std::os::raw::c_int as png_byte,
         2 as std::os::raw::c_int as png_byte, 2 as std::os::raw::c_int as png_byte,
         1 as std::os::raw::c_int as png_byte];
    /* We don't have to do anything on the last pass (6) */
    if pass < 6 as std::os::raw::c_int {
        /* Each pixel depth is handled separately */
        match (*row_info).pixel_depth as std::os::raw::c_int {
            1 => {
                let mut sp: png_bytep = 0 as *mut png_byte;
                let mut dp: png_bytep = 0 as *mut png_byte;
                let mut shift: std::os::raw::c_uint = 0;
                let mut d: std::os::raw::c_int = 0;
                let mut value: std::os::raw::c_int = 0;
                let mut i: png_uint_32 = 0;
                let mut row_width: png_uint_32 = (*row_info).width;
                dp = row;
                d = 0 as std::os::raw::c_int;
                shift = 7 as std::os::raw::c_int as std::os::raw::c_uint;
                i = png_pass_start[pass as usize] as png_uint_32;
                while i < row_width {
                    sp =
                        row.offset((i >> 3 as std::os::raw::c_int) as png_size_t as
                                       isize);
                    value =
                        *sp as std::os::raw::c_int >>
                            7 as std::os::raw::c_int -
                                (i & 0x7 as std::os::raw::c_int as std::os::raw::c_uint) as
                                    std::os::raw::c_int & 0x1 as std::os::raw::c_int;
                    d |= value << shift;
                    if shift == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        shift = 7 as std::os::raw::c_int as std::os::raw::c_uint;
                        let fresh1 = dp;
                        dp = dp.offset(1);
                        *fresh1 = d as png_byte;
                        d = 0 as std::os::raw::c_int
                    } else { shift = shift.wrapping_sub(1) }
                    i =
                        (i as
                             std::os::raw::c_uint).wrapping_add(png_pass_inc[pass as
                                                                         usize]
                                                            as std::os::raw::c_uint)
                            as png_uint_32 as png_uint_32
                }
                if shift != 7 as std::os::raw::c_int as std::os::raw::c_uint {
                    *dp = d as png_byte
                }
            }
            2 => {
                let mut sp_0: png_bytep = 0 as *mut png_byte;
                let mut dp_0: png_bytep = 0 as *mut png_byte;
                let mut shift_0: std::os::raw::c_uint = 0;
                let mut d_0: std::os::raw::c_int = 0;
                let mut value_0: std::os::raw::c_int = 0;
                let mut i_0: png_uint_32 = 0;
                let mut row_width_0: png_uint_32 = (*row_info).width;
                dp_0 = row;
                shift_0 = 6 as std::os::raw::c_int as std::os::raw::c_uint;
                d_0 = 0 as std::os::raw::c_int;
                i_0 = png_pass_start[pass as usize] as png_uint_32;
                while i_0 < row_width_0 {
                    sp_0 =
                        row.offset((i_0 >> 2 as std::os::raw::c_int) as png_size_t as
                                       isize);
                    value_0 =
                        *sp_0 as std::os::raw::c_int >>
                            ((3 as std::os::raw::c_int -
                                  (i_0 & 0x3 as std::os::raw::c_int as std::os::raw::c_uint)
                                      as std::os::raw::c_int) << 1 as std::os::raw::c_int) &
                            0x3 as std::os::raw::c_int;
                    d_0 |= value_0 << shift_0;
                    if shift_0 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        shift_0 = 6 as std::os::raw::c_int as std::os::raw::c_uint;
                        let fresh2 = dp_0;
                        dp_0 = dp_0.offset(1);
                        *fresh2 = d_0 as png_byte;
                        d_0 = 0 as std::os::raw::c_int
                    } else {
                        shift_0 =
                            shift_0.wrapping_sub(2 as std::os::raw::c_int as
                                                     std::os::raw::c_uint)
                    }
                    i_0 =
                        (i_0 as
                             std::os::raw::c_uint).wrapping_add(png_pass_inc[pass as
                                                                         usize]
                                                            as std::os::raw::c_uint)
                            as png_uint_32 as png_uint_32
                }
                if shift_0 != 6 as std::os::raw::c_int as std::os::raw::c_uint {
                    *dp_0 = d_0 as png_byte
                }
            }
            4 => {
                let mut sp_1: png_bytep = 0 as *mut png_byte;
                let mut dp_1: png_bytep = 0 as *mut png_byte;
                let mut shift_1: std::os::raw::c_uint = 0;
                let mut d_1: std::os::raw::c_int = 0;
                let mut value_1: std::os::raw::c_int = 0;
                let mut i_1: png_uint_32 = 0;
                let mut row_width_1: png_uint_32 = (*row_info).width;
                dp_1 = row;
                shift_1 = 4 as std::os::raw::c_int as std::os::raw::c_uint;
                d_1 = 0 as std::os::raw::c_int;
                i_1 = png_pass_start[pass as usize] as png_uint_32;
                while i_1 < row_width_1 {
                    sp_1 =
                        row.offset((i_1 >> 1 as std::os::raw::c_int) as png_size_t as
                                       isize);
                    value_1 =
                        *sp_1 as std::os::raw::c_int >>
                            ((1 as std::os::raw::c_int -
                                  (i_1 & 0x1 as std::os::raw::c_int as std::os::raw::c_uint)
                                      as std::os::raw::c_int) << 2 as std::os::raw::c_int) &
                            0xf as std::os::raw::c_int;
                    d_1 |= value_1 << shift_1;
                    if shift_1 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        shift_1 = 4 as std::os::raw::c_int as std::os::raw::c_uint;
                        let fresh3 = dp_1;
                        dp_1 = dp_1.offset(1);
                        *fresh3 = d_1 as png_byte;
                        d_1 = 0 as std::os::raw::c_int
                    } else {
                        shift_1 =
                            shift_1.wrapping_sub(4 as std::os::raw::c_int as
                                                     std::os::raw::c_uint)
                    }
                    i_1 =
                        (i_1 as
                             std::os::raw::c_uint).wrapping_add(png_pass_inc[pass as
                                                                         usize]
                                                            as std::os::raw::c_uint)
                            as png_uint_32 as png_uint_32
                }
                if shift_1 != 4 as std::os::raw::c_int as std::os::raw::c_uint {
                    *dp_1 = d_1 as png_byte
                }
            }
            _ => {
                let mut sp_2: png_bytep = 0 as *mut png_byte;
                let mut dp_2: png_bytep = 0 as *mut png_byte;
                let mut i_2: png_uint_32 = 0;
                let mut row_width_2: png_uint_32 = (*row_info).width;
                let mut pixel_bytes: png_size_t = 0;
                /* Start at the beginning */
                dp_2 = row;
                /* Find out how many bytes each pixel takes up */
                pixel_bytes =
                    ((*row_info).pixel_depth as std::os::raw::c_int >>
                         3 as std::os::raw::c_int) as png_size_t;
                /* Loop through the row, only looking at the pixels that matter */
                i_2 = png_pass_start[pass as usize] as png_uint_32;
                while i_2 < row_width_2 {
                    /* Find out where the original pixel is */
                    sp_2 =
                        row.offset((i_2 as
                                        png_size_t).wrapping_mul(pixel_bytes)
                                       as isize);
                    /* Move the pixel */
                    if dp_2 != sp_2 {
                        memcpy(dp_2 as *mut std::os::raw::c_void,
                               sp_2 as *const std::os::raw::c_void, pixel_bytes);
                    }
                    /* Next pixel */
                    dp_2 = dp_2.offset(pixel_bytes as isize);
                    i_2 =
                        (i_2 as
                             std::os::raw::c_uint).wrapping_add(png_pass_inc[pass as
                                                                         usize]
                                                            as std::os::raw::c_uint)
                            as png_uint_32 as png_uint_32
                }
            }
        }
        /* Set new row width */
        (*row_info).width =
            (*row_info).width.wrapping_add(png_pass_inc[pass as usize] as
                                               std::os::raw::c_uint).wrapping_sub(1 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_uint).wrapping_sub(png_pass_start[pass
                                                                                                                            as
                                                                                                                            usize]
                                                                                                             as
                                                                                                             std::os::raw::c_uint).wrapping_div(png_pass_inc[pass
                                                                                                                                                         as
                                                                                                                                                         usize]
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_uint);
        (*row_info).rowbytes =
            if (*row_info).pixel_depth as std::os::raw::c_int >= 8 as std::os::raw::c_int {
                ((*row_info).width as
                     png_size_t).wrapping_mul((*row_info).pixel_depth as
                                                  png_size_t >>
                                                  3 as std::os::raw::c_int)
            } else {
                (((*row_info).width as
                      png_size_t).wrapping_mul((*row_info).pixel_depth as
                                                   png_size_t).wrapping_add(7
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_ulong))
                    >> 3 as std::os::raw::c_int
            }
    };
}
unsafe extern "C" fn png_setup_sub_row(mut png_ptr: png_structrp,
                                       bpp: png_uint_32,
                                       row_bytes: png_size_t,
                                       lmins: png_size_t) -> png_size_t {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut lp: png_bytep = 0 as *mut png_byte;
    let mut i: png_size_t = 0;
    let mut sum: png_size_t = 0 as std::os::raw::c_int as png_size_t;
    let mut v: std::os::raw::c_uint = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    while i < bpp as std::os::raw::c_ulong {
        *dp = *rp;
        v = *dp as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        dp = dp.offset(1)
    }
    lp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    while i < row_bytes {
        *dp =
            (*rp as std::os::raw::c_int - *lp as std::os::raw::c_int & 0xff as std::os::raw::c_int) as
                png_byte;
        v = *dp as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        if sum > lmins { break ; }
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        lp = lp.offset(1);
        dp = dp.offset(1)
    }
    return sum;
}
unsafe extern "C" fn png_setup_sub_row_only(mut png_ptr: png_structrp,
                                            bpp: png_uint_32,
                                            row_bytes: png_size_t) {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut lp: png_bytep = 0 as *mut png_byte;
    let mut i: png_size_t = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        1 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    while i < bpp as std::os::raw::c_ulong {
        *dp = *rp;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        dp = dp.offset(1)
    }
    lp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    while i < row_bytes {
        *dp =
            (*rp as std::os::raw::c_int - *lp as std::os::raw::c_int & 0xff as std::os::raw::c_int) as
                png_byte;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        lp = lp.offset(1);
        dp = dp.offset(1)
    };
}
unsafe extern "C" fn png_setup_up_row(mut png_ptr: png_structrp,
                                      row_bytes: png_size_t,
                                      lmins: png_size_t) -> png_size_t {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut pp: png_bytep = 0 as *mut png_byte;
    let mut i: png_size_t = 0;
    let mut sum: png_size_t = 0 as std::os::raw::c_int as png_size_t;
    let mut v: std::os::raw::c_uint = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        2 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < row_bytes {
        *dp =
            (*rp as std::os::raw::c_int - *pp as std::os::raw::c_int & 0xff as std::os::raw::c_int) as
                png_byte;
        v = *dp as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        if sum > lmins { break ; }
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        pp = pp.offset(1);
        dp = dp.offset(1)
    }
    return sum;
}
unsafe extern "C" fn png_setup_up_row_only(mut png_ptr: png_structrp,
                                           row_bytes: png_size_t) {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut pp: png_bytep = 0 as *mut png_byte;
    let mut i: png_size_t = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        2 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < row_bytes {
        *dp =
            (*rp as std::os::raw::c_int - *pp as std::os::raw::c_int & 0xff as std::os::raw::c_int) as
                png_byte;
        i = i.wrapping_add(1);
        rp = rp.offset(1);
        pp = pp.offset(1);
        dp = dp.offset(1)
    };
}
unsafe extern "C" fn png_setup_avg_row(mut png_ptr: png_structrp,
                                       bpp: png_uint_32,
                                       row_bytes: png_size_t,
                                       lmins: png_size_t) -> png_size_t {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut pp: png_bytep = 0 as *mut png_byte;
    let mut lp: png_bytep = 0 as *mut png_byte;
    let mut i: png_uint_32 = 0;
    let mut sum: png_size_t = 0 as std::os::raw::c_int as png_size_t;
    let mut v: std::os::raw::c_uint = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        3 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_uint_32;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < bpp {
        let fresh4 = rp;
        rp = rp.offset(1);
        let fresh5 = pp;
        pp = pp.offset(1);
        let fresh6 = dp;
        dp = dp.offset(1);
        *fresh6 =
            (*fresh4 as std::os::raw::c_int -
                 *fresh5 as std::os::raw::c_int / 2 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int) as png_byte;
        v = *fresh6 as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        i = i.wrapping_add(1)
    }
    lp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    while (i as std::os::raw::c_ulong) < row_bytes {
        let fresh7 = rp;
        rp = rp.offset(1);
        let fresh8 = pp;
        pp = pp.offset(1);
        let fresh9 = lp;
        lp = lp.offset(1);
        let fresh10 = dp;
        dp = dp.offset(1);
        *fresh10 =
            (*fresh7 as std::os::raw::c_int -
                 (*fresh8 as std::os::raw::c_int + *fresh9 as std::os::raw::c_int) /
                     2 as std::os::raw::c_int & 0xff as std::os::raw::c_int) as png_byte;
        v = *fresh10 as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        if sum > lmins { break ; }
        i = i.wrapping_add(1)
    }
    return sum;
}
unsafe extern "C" fn png_setup_avg_row_only(mut png_ptr: png_structrp,
                                            bpp: png_uint_32,
                                            row_bytes: png_size_t) {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut pp: png_bytep = 0 as *mut png_byte;
    let mut lp: png_bytep = 0 as *mut png_byte;
    let mut i: png_uint_32 = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        3 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_uint_32;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < bpp {
        let fresh11 = rp;
        rp = rp.offset(1);
        let fresh12 = pp;
        pp = pp.offset(1);
        let fresh13 = dp;
        dp = dp.offset(1);
        *fresh13 =
            (*fresh11 as std::os::raw::c_int -
                 *fresh12 as std::os::raw::c_int / 2 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int) as png_byte;
        i = i.wrapping_add(1)
    }
    lp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    while (i as std::os::raw::c_ulong) < row_bytes {
        let fresh14 = rp;
        rp = rp.offset(1);
        let fresh15 = pp;
        pp = pp.offset(1);
        let fresh16 = lp;
        lp = lp.offset(1);
        let fresh17 = dp;
        dp = dp.offset(1);
        *fresh17 =
            (*fresh14 as std::os::raw::c_int -
                 (*fresh15 as std::os::raw::c_int + *fresh16 as std::os::raw::c_int) /
                     2 as std::os::raw::c_int & 0xff as std::os::raw::c_int) as png_byte;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn png_setup_paeth_row(mut png_ptr: png_structrp,
                                         bpp: png_uint_32,
                                         row_bytes: png_size_t,
                                         lmins: png_size_t) -> png_size_t {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut pp: png_bytep = 0 as *mut png_byte;
    let mut cp: png_bytep = 0 as *mut png_byte;
    let mut lp: png_bytep = 0 as *mut png_byte;
    let mut i: png_size_t = 0;
    let mut sum: png_size_t = 0 as std::os::raw::c_int as png_size_t;
    let mut v: std::os::raw::c_uint = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        4 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < bpp as std::os::raw::c_ulong {
        let fresh18 = rp;
        rp = rp.offset(1);
        let fresh19 = pp;
        pp = pp.offset(1);
        let fresh20 = dp;
        dp = dp.offset(1);
        *fresh20 =
            (*fresh18 as std::os::raw::c_int - *fresh19 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int) as png_byte;
        v = *fresh20 as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        i = i.wrapping_add(1)
    }
    lp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    cp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < row_bytes {
        let mut a: std::os::raw::c_int = 0;
        let mut b: std::os::raw::c_int = 0;
        let mut c: std::os::raw::c_int = 0;
        let mut pa: std::os::raw::c_int = 0;
        let mut pb: std::os::raw::c_int = 0;
        let mut pc: std::os::raw::c_int = 0;
        let mut p: std::os::raw::c_int = 0;
        let fresh21 = pp;
        pp = pp.offset(1);
        b = *fresh21 as std::os::raw::c_int;
        let fresh22 = cp;
        cp = cp.offset(1);
        c = *fresh22 as std::os::raw::c_int;
        let fresh23 = lp;
        lp = lp.offset(1);
        a = *fresh23 as std::os::raw::c_int;
        p = b - c;
        pc = a - c;
        pa = if p < 0 as std::os::raw::c_int { -p } else { p };
        pb = if pc < 0 as std::os::raw::c_int { -pc } else { pc };
        pc = if p + pc < 0 as std::os::raw::c_int { -(p + pc) } else { (p) + pc };
        p = if pa <= pb && pa <= pc { a } else if pb <= pc { b } else { c };
        let fresh24 = rp;
        rp = rp.offset(1);
        let fresh25 = dp;
        dp = dp.offset(1);
        *fresh25 =
            (*fresh24 as std::os::raw::c_int - p & 0xff as std::os::raw::c_int) as png_byte;
        v = *fresh25 as std::os::raw::c_uint;
        sum =
            (sum as
                 std::os::raw::c_ulong).wrapping_add(if v <
                                                    128 as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                                 v
                                             } else {
                                                 (256 as std::os::raw::c_int as
                                                      std::os::raw::c_uint).wrapping_sub(v)
                                             } as std::os::raw::c_ulong) as png_size_t
                as png_size_t;
        if sum > lmins { break ; }
        i = i.wrapping_add(1)
    }
    return sum;
}
unsafe extern "C" fn png_setup_paeth_row_only(mut png_ptr: png_structrp,
                                              bpp: png_uint_32,
                                              row_bytes: png_size_t) {
    let mut rp: png_bytep = 0 as *mut png_byte;
    let mut dp: png_bytep = 0 as *mut png_byte;
    let mut pp: png_bytep = 0 as *mut png_byte;
    let mut cp: png_bytep = 0 as *mut png_byte;
    let mut lp: png_bytep = 0 as *mut png_byte;
    let mut i: png_size_t = 0;
    *(*png_ptr).try_row.offset(0 as std::os::raw::c_int as isize) =
        4 as std::os::raw::c_int as png_byte;
    i = 0 as std::os::raw::c_int as png_size_t;
    rp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    dp = (*png_ptr).try_row.offset(1 as std::os::raw::c_int as isize);
    pp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < bpp as std::os::raw::c_ulong {
        let fresh26 = rp;
        rp = rp.offset(1);
        let fresh27 = pp;
        pp = pp.offset(1);
        let fresh28 = dp;
        dp = dp.offset(1);
        *fresh28 =
            (*fresh26 as std::os::raw::c_int - *fresh27 as std::os::raw::c_int &
                 0xff as std::os::raw::c_int) as png_byte;
        i = i.wrapping_add(1)
    }
    lp = (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize);
    cp = (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as isize);
    while i < row_bytes {
        let mut a: std::os::raw::c_int = 0;
        let mut b: std::os::raw::c_int = 0;
        let mut c: std::os::raw::c_int = 0;
        let mut pa: std::os::raw::c_int = 0;
        let mut pb: std::os::raw::c_int = 0;
        let mut pc: std::os::raw::c_int = 0;
        let mut p: std::os::raw::c_int = 0;
        let fresh29 = pp;
        pp = pp.offset(1);
        b = *fresh29 as std::os::raw::c_int;
        let fresh30 = cp;
        cp = cp.offset(1);
        c = *fresh30 as std::os::raw::c_int;
        let fresh31 = lp;
        lp = lp.offset(1);
        a = *fresh31 as std::os::raw::c_int;
        p = b - c;
        pc = a - c;
        pa = if p < 0 as std::os::raw::c_int { -p } else { p };
        pb = if pc < 0 as std::os::raw::c_int { -pc } else { pc };
        pc = if p + pc < 0 as std::os::raw::c_int { -(p + pc) } else { (p) + pc };
        p = if pa <= pb && pa <= pc { a } else if pb <= pc { b } else { c };
        let fresh32 = rp;
        rp = rp.offset(1);
        let fresh33 = dp;
        dp = dp.offset(1);
        *fresh33 =
            (*fresh32 as std::os::raw::c_int - p & 0xff as std::os::raw::c_int) as png_byte;
        i = i.wrapping_add(1)
    };
}
/* WRITE_FILTER */
#[no_mangle]
pub unsafe extern "C" fn png_write_find_filter(mut png_ptr: png_structrp,
                                               mut row_info: png_row_infop) {
    let mut filter_to_do: std::os::raw::c_uint = (*png_ptr).do_filter as std::os::raw::c_uint;
    let mut row_buf: png_bytep = 0 as *mut png_byte;
    let mut best_row: png_bytep = 0 as *mut png_byte;
    let mut bpp: png_uint_32 = 0;
    let mut mins: png_size_t = 0;
    let mut row_bytes: png_size_t = (*row_info).rowbytes;
    /* Find out how many bytes offset each pixel is */
    bpp =
        ((*row_info).pixel_depth as std::os::raw::c_int + 7 as std::os::raw::c_int >>
             3 as std::os::raw::c_int) as
            png_uint_32; /* so we can detect potential overflow of the
                               running sum */
    row_buf = (*png_ptr).row_buf;
    mins =
        (-(1 as std::os::raw::c_int) as
             png_size_t).wrapping_sub(256 as std::os::raw::c_int as std::os::raw::c_ulong);
    /* The prediction method we use is to find which method provides the
    * smallest value when summing the absolute values of the distances
    * from zero, using anything >= 128 as negative numbers.  This is known
    * as the "minimum sum of absolute differences" heuristic.  Other
    * heuristics are the "weighted minimum sum of absolute differences"
    * (experimental and can in theory improve compression), and the "zlib
    * predictive" method (not implemented yet), which does test compressions
    * of lines using different filter methods, and then chooses the
    * (series of) filter(s) that give minimum compressed data size (VERY
    * computationally expensive).
    *
    * GRR 980525:  consider also
    *
    *   (1) minimum sum of absolute differences from running average (i.e.,
    *       keep running sum of non-absolute differences & count of bytes)
    *       [track dispersion, too?  restart average if dispersion too large?]
    *
    *  (1b) minimum sum of absolute differences from sliding average, probably
    *       with window size <= deflate window (usually 32K)
    *
    *   (2) minimum sum of squared differences from zero or running average
    *       (i.e., ~ root-mean-square approach)
    */
    /* We don't need to test the 'no filter' case if this is the only filter
    * that has been chosen, as it doesn't actually do anything to the data.
    */
    best_row = (*png_ptr).row_buf;
    if (-(1 as std::os::raw::c_int) as
            png_size_t).wrapping_div(128 as std::os::raw::c_int as std::os::raw::c_ulong) <=
           row_bytes {
        /* Overflow can occur in the calculation, just select the lowest set
       * filter.
       */
        filter_to_do &= (0 as std::os::raw::c_uint).wrapping_sub(filter_to_do)
    } else if filter_to_do & 0x8 as std::os::raw::c_int as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint &&
                  filter_to_do != 0x8 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Overflow not possible and multiple filters in the list, including the
       * 'none' filter.
       */
        let mut rp: png_bytep = 0 as *mut png_byte;
        let mut sum: png_size_t = 0 as std::os::raw::c_int as png_size_t;
        let mut i: png_size_t = 0;
        let mut v: std::os::raw::c_uint = 0;
        i = 0 as std::os::raw::c_int as png_size_t;
        rp = row_buf.offset(1 as std::os::raw::c_int as isize);
        while i < row_bytes {
            v = *rp as std::os::raw::c_uint;
            sum =
                (sum as
                     std::os::raw::c_ulong).wrapping_add(if v <
                                                        128 as std::os::raw::c_int as
                                                            std::os::raw::c_uint {
                                                     v
                                                 } else {
                                                     (256 as std::os::raw::c_int as
                                                          std::os::raw::c_uint).wrapping_sub(v)
                                                 } as std::os::raw::c_ulong) as
                    png_size_t as png_size_t;
            i = i.wrapping_add(1);
            rp = rp.offset(1)
        }
        mins = sum
    }
    /* Sub filter */
    if filter_to_do == 0x10 as std::os::raw::c_int as std::os::raw::c_uint {
        /* It's the only filter so no testing is needed */
        png_setup_sub_row_only(png_ptr, bpp, row_bytes);
        best_row = (*png_ptr).try_row
    } else if filter_to_do & 0x10 as std::os::raw::c_int as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut sum_0: png_size_t = 0;
        let mut lmins: png_size_t = mins;
        sum_0 = png_setup_sub_row(png_ptr, bpp, row_bytes, lmins);
        if sum_0 < mins {
            mins = sum_0;
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row
            }
        }
    }
    /* Up filter */
    if filter_to_do == 0x20 as std::os::raw::c_int as std::os::raw::c_uint {
        png_setup_up_row_only(png_ptr, row_bytes);
        best_row = (*png_ptr).try_row
    } else if filter_to_do & 0x20 as std::os::raw::c_int as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut sum_1: png_size_t = 0;
        let mut lmins_0: png_size_t = mins;
        sum_1 = png_setup_up_row(png_ptr, row_bytes, lmins_0);
        if sum_1 < mins {
            mins = sum_1;
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row
            }
        }
    }
    /* Avg filter */
    if filter_to_do == 0x40 as std::os::raw::c_int as std::os::raw::c_uint {
        png_setup_avg_row_only(png_ptr, bpp, row_bytes);
        best_row = (*png_ptr).try_row
    } else if filter_to_do & 0x40 as std::os::raw::c_int as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut sum_2: png_size_t = 0;
        let mut lmins_1: png_size_t = mins;
        sum_2 = png_setup_avg_row(png_ptr, bpp, row_bytes, lmins_1);
        if sum_2 < mins {
            mins = sum_2;
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row
            }
        }
    }
    /* Paeth filter */
    if filter_to_do == 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
        png_setup_paeth_row_only(png_ptr, bpp, row_bytes);
        best_row = (*png_ptr).try_row
    } else if filter_to_do & 0x80 as std::os::raw::c_int as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut sum_3: png_size_t = 0;
        let mut lmins_2: png_size_t = mins;
        sum_3 = png_setup_paeth_row(png_ptr, bpp, row_bytes, lmins_2);
        if sum_3 < mins {
            best_row = (*png_ptr).try_row;
            if !(*png_ptr).tst_row.is_null() {
                (*png_ptr).try_row = (*png_ptr).tst_row;
                (*png_ptr).tst_row = best_row
            }
        }
    }
    /* Do the actual writing of the filtered row data from the chosen filter. */
    png_write_filtered_row(png_ptr, best_row,
                           (*row_info).rowbytes.wrapping_add(1 as std::os::raw::c_int
                                                                 as
                                                                 std::os::raw::c_ulong));
    /* WRITE_FILTER */
}
/* This filters the row, chooses which filter to use, if it has not already
 * been specified by the application, and then writes the row out with the
 * chosen filter.
 */
/* Do the actual writing of a previously filtered row. */
unsafe extern "C" fn png_write_filtered_row(mut png_ptr: png_structrp,
                                            mut filtered_row: png_bytep,
                                            mut full_row_length: png_size_t) 
 /*includes filter byte*/
 {
    png_compress_IDAT(png_ptr, filtered_row as png_const_bytep,
                      full_row_length, 0 as std::os::raw::c_int);
    /* Swap the current and previous rows */
    if !(*png_ptr).prev_row.is_null() {
        let mut tptr: png_bytep = 0 as *mut png_byte;
        tptr = (*png_ptr).prev_row;
        (*png_ptr).prev_row = (*png_ptr).row_buf;
        (*png_ptr).row_buf = tptr
    }
    /* WRITE_FILTER */
    /* Finish row - updates counters and flushes zlib if last row */
    png_write_finish_row(png_ptr);
    (*png_ptr).flush_rows = (*png_ptr).flush_rows.wrapping_add(1);
    if (*png_ptr).flush_dist > 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*png_ptr).flush_rows >= (*png_ptr).flush_dist {
        png_write_flush(png_ptr);
    };
    /* WRITE_FLUSH */
}
/* WRITE */
