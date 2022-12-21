
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn png_warning(png_ptr: png_const_structrp,
                   warning_message: png_const_charp);
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
    #[no_mangle]
    fn png_do_check_palette_indexes(png_ptr: png_structrp,
                                    row_info: png_row_infop);
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
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
/* pngrtran.c - transforms the data in a row for PNG readers
 *
 * Last changed in libpng 1.6.33 [September 28, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file contains functions optionally called by an application
 * in order to tell libpng how to handle data when reading a PNG.
 * Transformations that are used in both reading and writing are
 * in pngtrans.c.
 */
/* Set the action on getting a CRC error for an ancillary or critical chunk. */
#[no_mangle]
pub unsafe extern "C" fn png_set_crc_action(mut png_ptr: png_structrp,
                                            mut crit_action: std::os::raw::c_int,
                                            mut ancil_action: std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    let mut current_block_8: u64;
    /* Tell libpng how we react to CRC errors in critical chunks */
    match crit_action {
        5 => { current_block_8 = 1841672684692190573; }
        3 => {
            /* Warn/use data */
            (*png_ptr).flags &=
                !(0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x400 as std::os::raw::c_uint;
            current_block_8 = 1841672684692190573;
        }
        4 => {
            /* Quiet/use data */
            (*png_ptr).flags &=
                !(0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint;
            current_block_8 = 1841672684692190573;
        }
        2 => {
            /* Not a valid action for critical data */
            png_warning(png_ptr,
                        b"Can\'t discard critical data on CRC error\x00" as
                            *const u8 as *const std::os::raw::c_char);
            current_block_8 = 71504420348896963;
        }
        1 => {
            /* Error/quit */
            current_block_8 = 71504420348896963;
        }
        0 | _ => { current_block_8 = 71504420348896963; }
    }
    match current_block_8 {
        71504420348896963 =>
        /* FALLTHROUGH */
        {
            (*png_ptr).flags &=
                !(0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint)
        }
        _ => { }
    }
    /* Tell libpng how we react to CRC errors in ancillary chunks */
    match ancil_action {
        5 => { }
        3 => {
            /* Warn/use data */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x100 as std::os::raw::c_uint
        }
        4 => {
            /* Quiet/use data */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint
        }
        1 => {
            /* Error/quit */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x200 as std::os::raw::c_uint
        }
        2 | 0 | _ => {
            /* Warn/discard data */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint)
        }
    };
}
/* READ_BACKGROUND */
/* Scale 16-bit depth files to 8-bit depth.  If both of these are set then the
 * one that pngrtran does first (scale) happens.  This is necessary to allow the
 * TRANSFORM and API behavior to be somewhat consistent, and it's simpler.
 */
/* READ_ALPHA_MODE || READ_GAMMA */
/* READ_QUANTIZE */
/* READ_GAMMA */
/* READ_EXPAND */
/* RGB_TO_GRAY */
/* Initialize everything needed for the read.  This includes modifying
 * the palette.
 */
/* For the moment 'png_init_palette_transformations' and
 * 'png_init_rgb_transformations' only do some flag canceling optimizations.
 * The intent is that these two routines should have palette or rgb operations
 * extracted from 'png_init_read_transformations'.
 */
unsafe extern "C" fn png_init_palette_transformations(mut png_ptr:
                                                          png_structrp) {
    /* Called to handle the (input) palette case.  In png_do_read_transformations
    * the first step is to expand the palette if requested, so this code must
    * take care to only make changes that are invariant with respect to the
    * palette expansion, or only do them if there is no expansion.
    *
    * STRIP_ALPHA has already been handled in the caller (by setting num_trans
    * to 0.)
    */
    let mut input_has_alpha: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut input_has_transparency: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if (*png_ptr).num_trans as std::os::raw::c_int > 0 as std::os::raw::c_int {
        let mut i: std::os::raw::c_int = 0;
        /* Ignore if all the entries are opaque (unlikely!) */
        i = 0 as std::os::raw::c_int;
        while i < (*png_ptr).num_trans as std::os::raw::c_int {
            if !(*(*png_ptr).trans_alpha.offset(i as isize) as std::os::raw::c_int ==
                     255 as std::os::raw::c_int) {
                if *(*png_ptr).trans_alpha.offset(i as isize) as std::os::raw::c_int
                       == 0 as std::os::raw::c_int {
                    input_has_transparency = 1 as std::os::raw::c_int
                } else {
                    input_has_transparency = 1 as std::os::raw::c_int;
                    input_has_alpha = 1 as std::os::raw::c_int;
                    break ;
                }
            }
            i += 1
        }
    }
    /* If no alpha we can optimize. */
    if input_has_alpha == 0 as std::os::raw::c_int {
        /* Any alpha means background and associative alpha processing is
       * required, however if the alpha is 0 or 1 throughout OPTIMIZE_ALPHA
       * and ENCODE_ALPHA are irrelevant.
       */
        (*png_ptr).transformations &= !(0x800000 as std::os::raw::c_uint);
        (*png_ptr).flags &= !(0x2000 as std::os::raw::c_uint);
        if input_has_transparency == 0 as std::os::raw::c_int {
            (*png_ptr).transformations &=
                !(0x80 as std::os::raw::c_uint | 0x100 as std::os::raw::c_uint)
        }
    };
    /* READ_EXPAND && READ_BACKGROUND */
}
unsafe extern "C" fn png_init_rgb_transformations(mut png_ptr: png_structrp) {
    /* Added to libpng-1.5.4: check the color type to determine whether there
    * is any alpha or transparency in the image and simply cancel the
    * background and alpha mode stuff if there isn't.
    */
    let mut input_has_alpha: std::os::raw::c_int =
        ((*png_ptr).color_type as std::os::raw::c_int & 4 as std::os::raw::c_int !=
             0 as std::os::raw::c_int) as std::os::raw::c_int;
    let mut input_has_transparency: std::os::raw::c_int =
        ((*png_ptr).num_trans as std::os::raw::c_int > 0 as std::os::raw::c_int) as
            std::os::raw::c_int;
    /* If no alpha we can optimize. */
    if input_has_alpha == 0 as std::os::raw::c_int {
        /* Any alpha means background and associative alpha processing is
       * required, however if the alpha is 0 or 1 throughout OPTIMIZE_ALPHA
       * and ENCODE_ALPHA are irrelevant.
       */
        if input_has_transparency == 0 as std::os::raw::c_int {
            (*png_ptr).transformations &=
                !(0x80 as std::os::raw::c_uint | 0x100 as std::os::raw::c_uint)
        }
    };
    /* READ_EXPAND && READ_BACKGROUND */
}
#[no_mangle]
pub unsafe extern "C" fn png_init_read_transformations(mut png_ptr:
                                                           png_structrp) {
    /* This internal function is called from png_read_start_row in pngrutil.c
    * and it is called before the 'rowbytes' calculation is done, so the code
    * in here can change or update the transformations flags.
    *
    * First do updates that do not depend on the details of the PNG image data
    * being processed.
    */
    /* Certain transformations have the effect of preventing other
    * transformations that happen afterward in png_do_read_transformations;
    * resolve the interdependencies here.  From the code of
    * png_do_read_transformations the order is:
    *
    *  1) PNG_EXPAND (including PNG_EXPAND_tRNS)
    *  2) PNG_STRIP_ALPHA (if no compose)
    *  3) PNG_RGB_TO_GRAY
    *  4) PNG_GRAY_TO_RGB iff !PNG_BACKGROUND_IS_GRAY
    *  5) PNG_COMPOSE
    *  6) PNG_GAMMA
    *  7) PNG_STRIP_ALPHA (if compose)
    *  8) PNG_ENCODE_ALPHA
    *  9) PNG_SCALE_16_TO_8
    * 10) PNG_16_TO_8
    * 11) PNG_QUANTIZE (converts to palette)
    * 12) PNG_EXPAND_16
    * 13) PNG_GRAY_TO_RGB iff PNG_BACKGROUND_IS_GRAY
    * 14) PNG_INVERT_MONO
    * 15) PNG_INVERT_ALPHA
    * 16) PNG_SHIFT
    * 17) PNG_PACK
    * 18) PNG_BGR
    * 19) PNG_PACKSWAP
    * 20) PNG_FILLER (includes PNG_ADD_ALPHA)
    * 21) PNG_SWAP_ALPHA
    * 22) PNG_SWAP_BYTES
    * 23) PNG_USER_TRANSFORM [must be last]
    */
    /* STRIP_ALPHA supported, no COMPOSE */
    /* READ_GRAY_TO_RGB */
    /* For indexed PNG data (PNG_COLOR_TYPE_PALETTE) many of the transformations
    * can be performed directly on the palette, and some (such as rgb to gray)
    * can be optimized inside the palette.  This is particularly true of the
    * composite (background and alpha) stuff, which can be pretty much all done
    * in the palette even if the result is expanded to RGB or gray afterward.
    *
    * NOTE: this is Not Yet Implemented, the code behaves as in 1.5.1 and
    * earlier and the palette stuff is actually handled on the first row.  This
    * leads to the reported bug that the palette returned by png_get_PLTE is not
    * updated.
    */
    if (*png_ptr).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        png_init_palette_transformations(png_ptr);
    } else { png_init_rgb_transformations(png_ptr); };
    /* READ_BACKGROUND && READ_EXPAND_16 */
    /* NOTE: below 'PNG_READ_ALPHA_MODE_SUPPORTED' is presumed to also enable the
    * background support (see the comments in scripts/pnglibconf.dfa), this
    * allows pre-multiplication of the alpha channel to be implemented as
    * compositing on black.  This is probably sub-optimal and has been done in
    * 1.5.4 betas simply to enable external critique and testing (i.e. to
    * implement the new API quickly, without lots of internal changes.)
    */
    /* READ_GAMMA */
    /* READ_BACKGROUND */
    /* READ_SHIFT */
}
/* Modify the info structure to reflect the transformations.  The
 * info should be updated so a PNG file could be written with it,
 * assuming the transformations result in valid PNG data.
 */
#[no_mangle]
pub unsafe extern "C" fn png_read_transform_info(mut png_ptr: png_structrp,
                                                 mut info_ptr: png_inforp) {
    ((*info_ptr).bit_depth as std::os::raw::c_int) == 16 as std::os::raw::c_int;
    if (*info_ptr).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        (*info_ptr).channels = 1 as std::os::raw::c_int as png_byte
    } else if (*info_ptr).color_type as std::os::raw::c_int & 2 as std::os::raw::c_int !=
                  0 as std::os::raw::c_int {
        (*info_ptr).channels = 3 as std::os::raw::c_int as png_byte
    } else { (*info_ptr).channels = 1 as std::os::raw::c_int as png_byte }
    if (*info_ptr).color_type as std::os::raw::c_int & 4 as std::os::raw::c_int !=
           0 as std::os::raw::c_int {
        (*info_ptr).channels = (*info_ptr).channels.wrapping_add(1)
    }
    (*info_ptr).pixel_depth =
        ((*info_ptr).channels as std::os::raw::c_int *
             (*info_ptr).bit_depth as std::os::raw::c_int) as png_byte;
    (*info_ptr).rowbytes =
        if (*info_ptr).pixel_depth as std::os::raw::c_int >= 8 as std::os::raw::c_int {
            ((*info_ptr).width as
                 png_size_t).wrapping_mul((*info_ptr).pixel_depth as
                                              png_size_t >> 3 as std::os::raw::c_int)
        } else {
            (((*info_ptr).width as
                  png_size_t).wrapping_mul((*info_ptr).pixel_depth as
                                               png_size_t).wrapping_add(7 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong))
                >> 3 as std::os::raw::c_int
        };
    /* Adding in 1.5.4: cache the above value in png_struct so that we can later
    * check in png_rowbytes that the user buffer won't get overwritten.  Note
    * that the field is not always set - if png_read_update_info isn't called
    * the application has to either not do any transforms or get the calculation
    * right itself.
    */
    (*png_ptr).info_rowbytes = (*info_ptr).rowbytes;
    if !png_ptr.is_null() { return };
}
/* READ_BACKGROUND || READ_ALPHA_MODE */
/* READ_QUANTIZE */
/* Transform the row.  The order of transformations is significant,
 * and is very touchy.  If you add a transformation, take care to
 * decide how it fits in with the other transformations here.
 */
#[no_mangle]
pub unsafe extern "C" fn png_do_read_transformations(mut png_ptr:
                                                         png_structrp,
                                                     mut row_info:
                                                         png_row_infop) {
    if (*png_ptr).row_buf.is_null() {
        /* Prior to 1.5.4 this output row/pass where the NULL pointer is, but this
       * error is incredibly rare and incredibly easy to debug without this
       * information.
       */
        png_error(png_ptr,
                  b"NULL row buffer\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* The following is debugging; prior to 1.5.4 the code was never compiled in;
    * in 1.5.4 PNG_FLAG_DETECT_UNINITIALIZED was added and the macro
    * PNG_WARN_UNINITIALIZED_ROW removed.  In 1.6 the new flag is set only for
    * all transformations, however in practice the ROW_INIT always gets done on
    * demand, if necessary.
    */
    if (*png_ptr).flags & 0x4000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*png_ptr).flags & 0x40 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Application has failed to call either png_read_start_image() or
       * png_read_update_info() after setting transforms that expand pixels.
       * This check added to libpng-1.2.19 (but not enabled until 1.5.4).
       */
        png_error(png_ptr,
                  b"Uninitialized row\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* From Andreas Dilger e-mail to png-implement, 26 March 1998:
 *
 *   In most cases, the "simple transparency" should be done prior to doing
 *   gray-to-RGB, or you will have to test 3x as many bytes to check if a
 *   pixel is transparent.  You would also need to make sure that the
 *   transparency information is upgraded to RGB.
 *
 *   To summarize, the current flow is:
 *   - Gray + simple transparency -> compare 1 or 2 gray bytes and composite
 *                                   with background "in place" if transparent,
 *                                   convert to RGB if necessary
 *   - Gray + alpha -> composite with gray background and remove alpha bytes,
 *                                   convert to RGB if necessary
 *
 *   To support RGB backgrounds for gray images we need:
 *   - Gray + simple transparency -> convert to RGB + simple transparency,
 *                                   compare 3 or 6 bytes and composite with
 *                                   background "in place" if transparent
 *                                   (3x compare/pixel compared to doing
 *                                   composite with gray bkgrnd)
 *   - Gray + alpha -> convert to RGB + alpha, composite with background and
 *                                   remove alpha bytes (3x float
 *                                   operations/pixel compared with composite
 *                                   on gray background)
 *
 *  Greg's change will do this.  The reason it wasn't done before is for
 *  performance, as this increases the per-pixel operations.  If we would check
 *  in advance if the background was gray or RGB, and position the gray-to-RGB
 *  transform appropriately, then it would save a lot of work/time.
 */
    /* READ_QUANTIZE */
    /* Added at libpng-1.5.10 */
    if (*row_info).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
           (*png_ptr).num_palette_max >= 0 as std::os::raw::c_int {
        png_do_check_palette_indexes(png_ptr, row_info);
    };
}
/* READ */
/* READ_TRANSFORMS */
