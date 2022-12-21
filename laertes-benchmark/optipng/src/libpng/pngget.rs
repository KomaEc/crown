
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn png_check_IHDR(png_ptr: png_const_structrp, width: png_uint_32,
                      height: png_uint_32, bit_depth: std::os::raw::c_int,
                      color_type: std::os::raw::c_int, interlace_type: std::os::raw::c_int,
                      compression_type: std::os::raw::c_int,
                      filter_type: std::os::raw::c_int);
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
pub type png_uint_16p = *mut png_uint_16;
pub type png_const_charp = *const std::os::raw::c_char;
pub type png_bytepp = *mut *mut png_byte;
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
pub type png_const_structp = *const png_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_info_def {
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub valid: png_uint_32,
    pub rowbytes: png_size_t,
    pub palette: png_colorp,
    pub num_palette: png_uint_16,
    pub num_trans: png_uint_16,
    pub bit_depth: png_byte,
    pub color_type: png_byte,
    pub compression_type: png_byte,
    pub filter_type: png_byte,
    pub interlace_type: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
    pub spare_byte: png_byte,
    pub signature: [png_byte; 8],
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub trans_color: png_color_16,
    pub background: png_color_16,
    pub hist: png_uint_16p,
    pub free_me: png_uint_32,
    pub unknown_chunks: png_unknown_chunkp,
    pub unknown_chunks_num: std::os::raw::c_int,
    pub row_pointers: png_bytepp,
}
pub type png_unknown_chunkp = *mut png_unknown_chunk;
pub type png_info = png_info_def;
pub type png_const_infop = *const png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
pub type png_color_16p = *mut png_color_16;
pub type png_color_8p = *mut png_color_8;
pub type png_unknown_chunkpp = *mut *mut png_unknown_chunk;
/* pngget.c - retrieval of values from info struct
 *
 * Last changed in libpng 1.6.32 [August 24, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 */
#[no_mangle]
pub unsafe extern "C" fn png_get_valid(mut png_ptr: png_const_structrp,
                                       mut info_ptr: png_const_inforp,
                                       mut flag: png_uint_32) -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).valid & flag
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_rowbytes(mut png_ptr: png_const_structrp,
                                          mut info_ptr: png_const_inforp)
 -> png_size_t {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).rowbytes
    }
    return 0 as std::os::raw::c_int as png_size_t;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_rows(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_const_inforp)
 -> png_bytepp {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).row_pointers
    }
    return 0 as png_bytepp;
}
/* Easy access to info, added in libpng-0.99 */
#[no_mangle]
pub unsafe extern "C" fn png_get_image_width(mut png_ptr: png_const_structrp,
                                             mut info_ptr: png_const_inforp)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() { return (*info_ptr).width }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_image_height(mut png_ptr: png_const_structrp,
                                              mut info_ptr: png_const_inforp)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() { return (*info_ptr).height }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_bit_depth(mut png_ptr: png_const_structrp,
                                           mut info_ptr: png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).bit_depth
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_color_type(mut png_ptr: png_const_structrp,
                                            mut info_ptr: png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).color_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_filter_type(mut png_ptr: png_const_structrp,
                                             mut info_ptr: png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).filter_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_interlace_type(mut png_ptr:
                                                    png_const_structrp,
                                                mut info_ptr:
                                                    png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).interlace_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_compression_type(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).compression_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_x_pixels_per_meter(mut png_ptr:
                                                        png_const_structrp,
                                                    mut info_ptr:
                                                        png_const_inforp)
 -> png_uint_32 {
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_y_pixels_per_meter(mut png_ptr:
                                                        png_const_structrp,
                                                    mut info_ptr:
                                                        png_const_inforp)
 -> png_uint_32 {
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_pixels_per_meter(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_uint_32 {
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_pixel_aspect_ratio_fixed(mut png_ptr:
                                                              png_const_structrp,
                                                          mut info_ptr:
                                                              png_const_inforp)
 -> png_fixed_point {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_x_offset_microns(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_y_offset_microns(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_x_offset_pixels(mut png_ptr:
                                                     png_const_structrp,
                                                 mut info_ptr:
                                                     png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_y_offset_pixels(mut png_ptr:
                                                     png_const_structrp,
                                                 mut info_ptr:
                                                     png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}
/* INCH_CONVERSIONS */
/* png_get_channels really belongs in here, too, but it's been around longer */
/* EASY_ACCESS */
#[no_mangle]
pub unsafe extern "C" fn png_get_channels(mut png_ptr: png_const_structrp,
                                          mut info_ptr: png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).channels
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_signature(mut png_ptr: png_const_structrp,
                                           mut info_ptr: png_const_inforp)
 -> png_const_bytep {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).signature.as_ptr()
    }
    return 0 as png_const_bytep;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_bKGD(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut background: *mut png_color_16p)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x20 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !background.is_null() {
        *background = &mut (*info_ptr).background;
        return 0x20 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_hIST(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut hist: *mut png_uint_16p)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x40 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !hist.is_null() {
        *hist = (*info_ptr).hist;
        return 0x40 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_IHDR(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_const_inforp,
                                      mut width: *mut png_uint_32,
                                      mut height: *mut png_uint_32,
                                      mut bit_depth: *mut std::os::raw::c_int,
                                      mut color_type: *mut std::os::raw::c_int,
                                      mut interlace_type: *mut std::os::raw::c_int,
                                      mut compression_type: *mut std::os::raw::c_int,
                                      mut filter_type: *mut std::os::raw::c_int)
 -> png_uint_32 {
    if png_ptr.is_null() || info_ptr.is_null() {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if !width.is_null() { *width = (*info_ptr).width }
    if !height.is_null() { *height = (*info_ptr).height }
    if !bit_depth.is_null() {
        *bit_depth = (*info_ptr).bit_depth as std::os::raw::c_int
    }
    if !color_type.is_null() {
        *color_type = (*info_ptr).color_type as std::os::raw::c_int
    }
    if !compression_type.is_null() {
        *compression_type = (*info_ptr).compression_type as std::os::raw::c_int
    }
    if !filter_type.is_null() {
        *filter_type = (*info_ptr).filter_type as std::os::raw::c_int
    }
    if !interlace_type.is_null() {
        *interlace_type = (*info_ptr).interlace_type as std::os::raw::c_int
    }
    /* This is redundant if we can be sure that the info_ptr values were all
    * assigned in png_set_IHDR().  We do the check anyhow in case an
    * application has ignored our advice not to mess with the members
    * of info_ptr directly.
    */
    png_check_IHDR(png_ptr, (*info_ptr).width, (*info_ptr).height,
                   (*info_ptr).bit_depth as std::os::raw::c_int,
                   (*info_ptr).color_type as std::os::raw::c_int,
                   (*info_ptr).interlace_type as std::os::raw::c_int,
                   (*info_ptr).compression_type as std::os::raw::c_int,
                   (*info_ptr).filter_type as std::os::raw::c_int);
    return 1 as std::os::raw::c_int as png_uint_32;
}
/* sCAL */
/* pHYs */
#[no_mangle]
pub unsafe extern "C" fn png_get_PLTE(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut palette: *mut png_colorp,
                                      mut num_palette: *mut std::os::raw::c_int)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x8 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !palette.is_null() {
        *palette = (*info_ptr).palette;
        *num_palette = (*info_ptr).num_palette as std::os::raw::c_int;
        return 0x8 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_sBIT(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut sig_bit: *mut png_color_8p)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !sig_bit.is_null() {
        *sig_bit = &mut (*info_ptr).sig_bit;
        return 0x2 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_tRNS(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut trans_alpha: *mut png_bytep,
                                      mut num_trans: *mut std::os::raw::c_int,
                                      mut trans_color: *mut png_color_16p)
 -> png_uint_32 {
    let mut retval: png_uint_32 = 0 as std::os::raw::c_int as png_uint_32;
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x10 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*info_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            if !trans_alpha.is_null() {
                *trans_alpha = (*info_ptr).trans_alpha;
                retval |= 0x10 as std::os::raw::c_uint
            }
            if !trans_color.is_null() {
                *trans_color = &mut (*info_ptr).trans_color
            }
        } else {
            /* if (info_ptr->color_type != PNG_COLOR_TYPE_PALETTE) */
            if !trans_color.is_null() {
                *trans_color = &mut (*info_ptr).trans_color;
                retval |= 0x10 as std::os::raw::c_uint
            }
            if !trans_alpha.is_null() { *trans_alpha = 0 as png_bytep }
        }
        if !num_trans.is_null() {
            *num_trans = (*info_ptr).num_trans as std::os::raw::c_int;
            retval |= 0x10 as std::os::raw::c_uint
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_unknown_chunks(mut png_ptr:
                                                    png_const_structrp,
                                                mut info_ptr: png_inforp,
                                                mut unknowns:
                                                    png_unknown_chunkpp)
 -> std::os::raw::c_int {
    if !png_ptr.is_null() && !info_ptr.is_null() && !unknowns.is_null() {
        *unknowns = (*info_ptr).unknown_chunks;
        return (*info_ptr).unknown_chunks_num
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_compression_buffer_size(mut png_ptr:
                                                             png_const_structrp)
 -> png_size_t {
    if png_ptr.is_null() { return 0 as std::os::raw::c_int as png_size_t }
    if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        return (*png_ptr).IDAT_read_size as png_size_t
    } else { return (*png_ptr).zbuffer_size as png_size_t };
}
/* These functions were added to libpng 1.2.6 and were enabled
 * by default in libpng-1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_get_user_width_max(mut png_ptr:
                                                    png_const_structrp)
 -> png_uint_32 {
    return if !png_ptr.is_null() {
               (*png_ptr).user_width_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
#[no_mangle]
pub unsafe extern "C" fn png_get_user_height_max(mut png_ptr:
                                                     png_const_structrp)
 -> png_uint_32 {
    return if !png_ptr.is_null() {
               (*png_ptr).user_height_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
/* This function was added to libpng 1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_get_chunk_cache_max(mut png_ptr:
                                                     png_const_structrp)
 -> png_uint_32 {
    return if !png_ptr.is_null() {
               (*png_ptr).user_chunk_cache_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
/* This function was added to libpng 1.4.1 */
#[no_mangle]
pub unsafe extern "C" fn png_get_chunk_malloc_max(mut png_ptr:
                                                      png_const_structrp)
 -> png_alloc_size_t {
    return if !png_ptr.is_null() {
               (*png_ptr).user_chunk_malloc_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
}
/* SET_USER_LIMITS */
/* These functions were added to libpng 1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_get_io_state(mut png_ptr: png_const_structrp)
 -> png_uint_32 {
    return (*png_ptr).io_state;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_io_chunk_type(mut png_ptr:
                                                   png_const_structrp)
 -> png_uint_32 {
    return (*png_ptr).chunk_name;
}
/* IO_STATE */
#[no_mangle]
pub unsafe extern "C" fn png_get_palette_max(mut png_ptr: png_const_structp,
                                             mut info_ptr: png_const_infop)
 -> std::os::raw::c_int {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*png_ptr).num_palette_max
    }
    return -(1 as std::os::raw::c_int);
}
/* READ || WRITE */
