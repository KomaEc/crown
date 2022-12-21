
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
    #[no_mangle]
    fn png_warning(png_ptr: png_const_structrp,
                   warning_message: png_const_charp);
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
pub type png_const_voidp = *const std::os::raw::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
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
/* pngmem.c - stub functions for memory allocation
 *
 * Last changed in libpng 1.6.26 [October 20, 2016]
 * Copyright (c) 1998-2002,2004,2006-2014,2016 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all memory allocation.  Users who
 * need special memory handling are expected to supply replacement
 * functions for png_malloc() and png_free(), and to use
 * png_create_read_struct_2() and png_create_write_struct_2() to
 * identify the replacement functions.
 */
/* Free a png_struct */
#[no_mangle]
pub unsafe extern "C" fn png_destroy_png_struct(mut png_ptr: png_structrp) {
    if !png_ptr.is_null() {
        /* png_free might call png_error and may certainly call
       * png_get_mem_ptr, so fake a temporary png_struct to support this.
       */
        let mut dummy_struct: png_struct = *png_ptr;
        memset(png_ptr as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<png_struct>() as std::os::raw::c_ulong);
        png_free(&mut dummy_struct, png_ptr as png_voidp);
    };
}
/* Allocate memory.  For reasonable files, size should never exceed
 * 64K.  However, zlib may allocate more than 64K if you don't tell
 * it not to.  See zconf.h and png.h for more information.  zlib does
 * need to allocate exactly 64K, so whatever you call here must
 * have the ability to do that.
 */
#[no_mangle]
pub unsafe extern "C" fn png_calloc(mut png_ptr: png_const_structrp,
                                    mut size: png_alloc_size_t) -> png_voidp {
    let mut ret: png_voidp = 0 as *mut std::os::raw::c_void;
    ret = png_malloc(png_ptr, size);
    if !ret.is_null() { memset(ret, 0 as std::os::raw::c_int, size); }
    return ret;
}
/* png_malloc_base, an internal function added at libpng 1.6.0, does the work of
 * allocating memory, taking into account limits and PNG_USER_MEM_SUPPORTED.
 * Checking and error handling must happen outside this routine; it returns NULL
 * if the allocation cannot be done (for any reason.)
 */
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_malloc_base(mut png_ptr: png_const_structrp,
                                         mut size: png_alloc_size_t)
 -> png_voidp {
    /* Moved to png_malloc_base from png_malloc_default in 1.6.0; the DOS
    * allocators have also been removed in 1.6.0, so any 16-bit system now has
    * to implement a user memory handler.  This checks to be sure it isn't
    * called with big numbers.
    */
    /* Some compilers complain that this is always true.  However, it
    * can be false when integer overflow happens.
    */
    if size > 0 as std::os::raw::c_int as std::os::raw::c_ulong &&
           size <= -(1 as std::os::raw::c_int) as png_size_t {
        return malloc(size)
        /* checked for truncation above */
    } else { return 0 as *mut std::os::raw::c_void };
}
/* This is really here only to work round a spurious warning in GCC 4.6 and 4.7
 * that arises because of the checks in png_realloc_array that are repeated in
 * png_malloc_array.
 */
unsafe extern "C" fn png_malloc_array_checked(mut png_ptr: png_const_structrp,
                                              mut nelements: std::os::raw::c_int,
                                              mut element_size: size_t)
 -> png_voidp {
    let mut req: png_alloc_size_t =
        nelements as png_alloc_size_t; /* known to be > 0 */
    if req <= (-(1 as std::os::raw::c_int) as png_size_t).wrapping_div(element_size) {
        return png_malloc_base(png_ptr, req.wrapping_mul(element_size))
    }
    /* The failure case when the request is too large */
    return 0 as *mut std::os::raw::c_void;
}
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_malloc_array(mut png_ptr: png_const_structrp,
                                          mut nelements: std::os::raw::c_int,
                                          mut element_size: size_t)
 -> png_voidp {
    if nelements <= 0 as std::os::raw::c_int ||
           element_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        png_error(png_ptr,
                  b"internal error: array alloc\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    return png_malloc_array_checked(png_ptr, nelements, element_size);
}
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_realloc_array(mut png_ptr: png_const_structrp,
                                           mut old_array: png_const_voidp,
                                           mut old_elements: std::os::raw::c_int,
                                           mut add_elements: std::os::raw::c_int,
                                           mut element_size: size_t)
 -> png_voidp {
    /* These are internal errors: */
    if add_elements <= 0 as std::os::raw::c_int ||
           element_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           old_elements < 0 as std::os::raw::c_int ||
           old_array.is_null() && old_elements > 0 as std::os::raw::c_int {
        png_error(png_ptr,
                  b"internal error: array realloc\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* Check for overflow on the elements count (so the caller does not have to
    * check.)
    */
    if add_elements <= 2147483647 as std::os::raw::c_int - old_elements {
        let mut new_array: png_voidp =
            png_malloc_array_checked(png_ptr, old_elements + add_elements,
                                     element_size);
        if !new_array.is_null() {
            /* Because png_malloc_array worked the size calculations below cannot
          * overflow.
          */
            if old_elements > 0 as std::os::raw::c_int {
                memcpy(new_array, old_array,
                       element_size.wrapping_mul(old_elements as std::os::raw::c_uint
                                                     as std::os::raw::c_ulong));
            }
            memset((new_array as
                        *mut std::os::raw::c_char).offset(element_size.wrapping_mul(old_elements
                                                                                as
                                                                                std::os::raw::c_uint
                                                                                as
                                                                                std::os::raw::c_ulong)
                                                      as isize) as
                       *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                   element_size.wrapping_mul(add_elements as std::os::raw::c_uint as
                                                 std::os::raw::c_ulong));
            return new_array
        }
    }
    return 0 as *mut std::os::raw::c_void;
    /* error */
}
/* TEXT || sPLT || STORE_UNKNOWN_CHUNKS */
/* Various functions that have different error handling are derived from this.
 * png_malloc always exists, but if PNG_USER_MEM_SUPPORTED is defined a separate
 * function png_malloc_default is also provided.
 */
#[no_mangle]
pub unsafe extern "C" fn png_malloc(mut png_ptr: png_const_structrp,
                                    mut size: png_alloc_size_t) -> png_voidp {
    let mut ret: png_voidp =
        0 as *mut std::os::raw::c_void; /* 'm' means png_malloc */
    if png_ptr.is_null() { return 0 as *mut std::os::raw::c_void }
    ret = png_malloc_base(png_ptr, size);
    if ret.is_null() {
        png_error(png_ptr,
                  b"Out of memory\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* USER_MEM */
/* This function was added at libpng version 1.2.3.  The png_malloc_warn()
 * function will issue a png_warning and return NULL instead of issuing a
 * png_error, if it fails to allocate the requested memory.
 */
#[no_mangle]
pub unsafe extern "C" fn png_malloc_warn(mut png_ptr: png_const_structrp,
                                         mut size: png_alloc_size_t)
 -> png_voidp {
    if !png_ptr.is_null() {
        let mut ret: png_voidp = png_malloc_base(png_ptr, size);
        if !ret.is_null() { return ret }
        png_warning(png_ptr,
                    b"Out of memory\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as *mut std::os::raw::c_void;
}
/* Free a pointer allocated by png_malloc().  If ptr is NULL, return
 * without taking any action.
 */
#[no_mangle]
pub unsafe extern "C" fn png_free(mut png_ptr: png_const_structrp,
                                  mut ptr: png_voidp) {
    if png_ptr.is_null() || ptr.is_null() { return }
    /* USER_MEM */
    free(ptr);
}
/* READ || WRITE */
/* USER_MEM */
