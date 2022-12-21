
extern "C" {
    pub type internal_state;
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
/* pngtrans.c - transforms the data in a row (used by both readers and writers)
 *
 * Last changed in libpng 1.6.33 [September 28, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_interlace_handling(mut png_ptr: png_structrp)
 -> std::os::raw::c_int {
    if !png_ptr.is_null() &&
           (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int {
        (*png_ptr).transformations |= 0x2 as std::os::raw::c_uint;
        return 7 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* PACKSWAP || WRITE_PACKSWAP */
/* READ_BGR || WRITE_BGR */
/* Added at libpng-1.5.10 */
#[no_mangle]
pub unsafe extern "C" fn png_do_check_palette_indexes(mut png_ptr:
                                                          png_structrp,
                                                      mut row_info:
                                                          png_row_infop) {
    if ((*png_ptr).num_palette as std::os::raw::c_int) <
           (1 as std::os::raw::c_int) << (*row_info).bit_depth as std::os::raw::c_int &&
           (*png_ptr).num_palette as std::os::raw::c_int > 0 as std::os::raw::c_int {
        /* num_palette can be 0 in MNG files */
        /* Calculations moved outside switch in an attempt to stop different
       * compiler warnings.  'padding' is in *bits* within the last byte, it is
       * an 'int' because pixel_depth becomes an 'int' in the expression below,
       * and this calculation is used because it avoids warnings that other
       * forms produced on either GCC or MSVC.
       */
        let mut padding: std::os::raw::c_int =
            (8 as std::os::raw::c_int as
                 std::os::raw::c_uint).wrapping_sub(((*row_info).pixel_depth as
                                                 std::os::raw::c_uint).wrapping_mul((*row_info).width.wrapping_rem(8
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               png_uint_32)).wrapping_rem(8
                                                                                                                                              as
                                                                                                                                              std::os::raw::c_int
                                                                                                                                              as
                                                                                                                                              std::os::raw::c_uint)).wrapping_rem(8
                                                                                                                                                                              as
                                                                                                                                                                              std::os::raw::c_int
                                                                                                                                                                              as
                                                                                                                                                                              std::os::raw::c_uint)
                as std::os::raw::c_int;
        let mut rp: png_bytep =
            (*png_ptr).row_buf.offset((*row_info).rowbytes as
                                          isize).offset(-(1 as std::os::raw::c_int as
                                                              isize));
        match (*row_info).bit_depth as std::os::raw::c_int {
            1 => {
                /* in this case, all bytes must be 0 so we don't need
             * to unpack the pixels except for the rightmost one.
             */
                while rp > (*png_ptr).row_buf {
                    if *rp as std::os::raw::c_int >> padding != 0 as std::os::raw::c_int {
                        (*png_ptr).num_palette_max = 1 as std::os::raw::c_int
                    }
                    padding = 0 as std::os::raw::c_int;
                    rp = rp.offset(-1)
                }
            }
            2 => {
                while rp > (*png_ptr).row_buf {
                    let mut i: std::os::raw::c_int =
                        *rp as std::os::raw::c_int >> padding & 0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    i =
                        *rp as std::os::raw::c_int >> padding >> 2 as std::os::raw::c_int &
                            0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    i =
                        *rp as std::os::raw::c_int >> padding >> 4 as std::os::raw::c_int &
                            0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    i =
                        *rp as std::os::raw::c_int >> padding >> 6 as std::os::raw::c_int &
                            0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    padding = 0 as std::os::raw::c_int;
                    rp = rp.offset(-1)
                }
            }
            4 => {
                while rp > (*png_ptr).row_buf {
                    let mut i_0: std::os::raw::c_int =
                        *rp as std::os::raw::c_int >> padding & 0xf as std::os::raw::c_int;
                    if i_0 > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i_0
                    }
                    i_0 =
                        *rp as std::os::raw::c_int >> padding >> 4 as std::os::raw::c_int &
                            0xf as std::os::raw::c_int;
                    if i_0 > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i_0
                    }
                    padding = 0 as std::os::raw::c_int;
                    rp = rp.offset(-1)
                }
            }
            8 => {
                while rp > (*png_ptr).row_buf {
                    if *rp as std::os::raw::c_int > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = *rp as std::os::raw::c_int
                    }
                    rp = rp.offset(-1)
                }
            }
            _ => { }
        }
    };
}
/* READ || WRITE */
/* READ_USER_TRANSFORM || WRITE_USER_TRANSFORM */
/* CHECK_FOR_INVALID_INDEX */
