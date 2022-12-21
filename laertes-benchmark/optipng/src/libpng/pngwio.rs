
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type internal_state;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn png_warning(png_ptr: png_const_structrp,
                   warning_message: png_const_charp);
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type png_const_charp = *const std::os::raw::c_char;
pub type png_FILE_p = *mut FILE;
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
pub type png_ptruint = std::os::raw::c_ulong;
/* pngwio.c - functions for data output
 *
 * Last changed in libpng 1.6.24 [August 4, 2016]
 * Copyright (c) 1998-2002,2004,2006-2014,2016 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all output.  Users who need
 * special handling are expected to write functions that have the same
 * arguments as these and perform similar functions, but that possibly
 * use different output methods.  Note that you shouldn't change these
 * functions, but rather write replacement functions and then change
 * them at run time with png_set_write_fn(...).
 */
/* Write the data to whatever output you are using.  The default routine
 * writes to a file pointer.  Note that this routine sometimes gets called
 * with very small lengths, so you should implement some kind of simple
 * buffering if you are using unbuffered writes.  This should never be asked
 * to write more than 64K on a 16-bit machine.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_data(mut png_ptr: png_structrp,
                                        mut data: png_const_bytep,
                                        mut length: png_size_t) {
    /* NOTE: write_data_fn must not change the buffer! */
    if (*png_ptr).write_data_fn.is_some() {
        Some((*png_ptr).write_data_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr,
                                                                                                               data
                                                                                                                   as
                                                                                                                   *const std::os::raw::c_void
                                                                                                                   as
                                                                                                                   png_ptruint
                                                                                                                   as
                                                                                                                   png_bytep,
                                                                                                               length);
    } else {
        png_error(png_ptr,
                  b"Call to NULL write function\x00" as *const u8 as
                      *const std::os::raw::c_char);
    };
}
/* This is the function that does the actual writing of data.  If you are
 * not writing to a standard C stream, you should create a replacement
 * write_data function and use it at run time with png_set_write_fn(), rather
 * than changing the library.
 */
#[no_mangle]
pub unsafe extern "C" fn png_default_write_data(mut png_ptr: png_structp,
                                                mut data: png_bytep,
                                                mut length: png_size_t) {
    let mut check: png_size_t = 0;
    if png_ptr.is_null() { return }
    check =
        fwrite(data as *const std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
               length, (*png_ptr).io_ptr as png_FILE_p);
    if check != length {
        png_error(png_ptr as *const png_struct,
                  b"Write Error\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* This function is called to output any data pending writing (normally
 * to disk).  After png_flush is called, there should be no data pending
 * writing in any buffers.
 */
#[no_mangle]
pub unsafe extern "C" fn png_flush(mut png_ptr: png_structrp) {
    if (*png_ptr).output_flush_fn.is_some() {
        Some((*png_ptr).output_flush_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_default_flush(mut png_ptr: png_structp) {
    let mut io_ptr: png_FILE_p = 0 as *mut FILE;
    if png_ptr.is_null() { return }
    io_ptr = (*png_ptr).io_ptr as png_FILE_p;
    fflush(io_ptr);
}
/* This function allows the application to supply new output functions for
 * libpng if standard C streams aren't being used.
 *
 * This function takes as its arguments:
 * png_ptr       - pointer to a png output data structure
 * io_ptr        - pointer to user supplied structure containing info about
 *                 the output functions.  May be NULL.
 * write_data_fn - pointer to a new output function that takes as its
 *                 arguments a pointer to a png_struct, a pointer to
 *                 data to be written, and a 32-bit unsigned int that is
 *                 the number of bytes to be written.  The new write
 *                 function should call png_error(png_ptr, "Error msg")
 *                 to exit and output any fatal error messages.  May be
 *                 NULL, in which case libpng's default function will
 *                 be used.
 * flush_data_fn - pointer to a new flush function that takes as its
 *                 arguments a pointer to a png_struct.  After a call to
 *                 the flush function, there should be no data in any buffers
 *                 or pending transmission.  If the output method doesn't do
 *                 any buffering of output, a function prototype must still be
 *                 supplied although it doesn't have to do anything.  If
 *                 PNG_WRITE_FLUSH_SUPPORTED is not defined at libpng compile
 *                 time, output_flush_fn will be ignored, although it must be
 *                 supplied for compatibility.  May be NULL, in which case
 *                 libpng's default function will be used, if
 *                 PNG_WRITE_FLUSH_SUPPORTED is defined.  This is not
 *                 a good idea if io_ptr does not point to a standard
 *                 *FILE structure.
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_write_fn(mut png_ptr: png_structrp,
                                          mut io_ptr: png_voidp,
                                          mut write_data_fn: png_rw_ptr,
                                          mut output_flush_fn:
                                              png_flush_ptr) {
    if png_ptr.is_null() { return }
    (*png_ptr).io_ptr = io_ptr;
    if write_data_fn.is_some() {
        (*png_ptr).write_data_fn = write_data_fn
    } else {
        (*png_ptr).write_data_fn =
            Some(png_default_write_data as
                     unsafe extern "C" fn(_: png_structp, _: png_bytep,
                                          _: png_size_t) -> ())
    }
    if output_flush_fn.is_some() {
        (*png_ptr).output_flush_fn = output_flush_fn
    } else {
        (*png_ptr).output_flush_fn =
            Some(png_default_flush as
                     unsafe extern "C" fn(_: png_structp) -> ())
    }
    /* WRITE_FLUSH */
    /* It is an error to read while writing a png file */
    if (*png_ptr).read_data_fn.is_some() {
        (*png_ptr).read_data_fn = None;
        png_warning(png_ptr,
                    b"Can\'t set both read_data_fn and write_data_fn in the same structure\x00"
                        as *const u8 as *const std::os::raw::c_char);
    };
}
/* WRITE */
