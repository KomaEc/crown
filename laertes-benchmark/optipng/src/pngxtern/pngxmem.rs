
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    #[no_mangle]
    fn png_get_image_height(png_ptr: png_const_structrp,
                            info_ptr: png_const_inforp) -> png_uint_32;
    #[no_mangle]
    fn png_set_rows(png_ptr: png_const_structrp, info_ptr: png_inforp,
                    row_pointers: png_bytepp);
    #[no_mangle]
    fn png_get_rowbytes(png_ptr: png_const_structrp,
                        info_ptr: png_const_inforp) -> png_size_t;
    #[no_mangle]
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t)
     -> png_voidp;
    #[no_mangle]
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    #[no_mangle]
    fn png_free_data(png_ptr: png_const_structrp, info_ptr: png_inforp,
                     free_me: png_uint_32, num: std::os::raw::c_int);
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub type size_t = std::os::raw::c_ulong;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = size_t;
pub type png_alloc_size_t = png_size_t;
pub type png_voidp = *mut std::os::raw::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const std::os::raw::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
pub type pngx_alloc_size_t = png_alloc_size_t;
/*
 * pngxmem.c - libpng extension: memory allocation utilities.
 *
 * Copyright (C) 2003-2017 Cosmin Truta.
 * This software is distributed under the same licensing and warranty terms
 * as libpng.
 */
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows(mut png_ptr: png_structp,
                                          mut info_ptr: png_infop,
                                          mut filler: std::os::raw::c_int)
 -> png_bytepp {
    return pngx_malloc_rows_extended(png_ptr, info_ptr,
                                     0 as std::os::raw::c_int as pngx_alloc_size_t,
                                     filler);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows_extended(mut png_ptr: png_structp,
                                                   mut info_ptr: png_infop,
                                                   mut min_row_size:
                                                       pngx_alloc_size_t,
                                                   mut filler: std::os::raw::c_int)
 -> png_bytepp {
    let mut row_size: pngx_alloc_size_t = 0;
    let mut row: png_bytep = 0 as *mut png_byte;
    let mut rows: png_bytepp = 0 as *mut *mut png_byte;
    let mut height: png_uint_32 = 0;
    let mut i: png_uint_32 = 0;
    /* Check the image dimensions and calculate the row size. */
    height =
        png_get_image_height(png_ptr as *const png_struct,
                             info_ptr as *const png_info);
    if height == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Missing IHDR\x00" as *const u8 as *const std::os::raw::c_char);
    }
    row_size =
        png_get_rowbytes(png_ptr as *const png_struct,
                         info_ptr as *const png_info);
    /* libpng sets row_size to 0 when the width is too large to process. */
    if row_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           height as pngx_alloc_size_t >
               (-(1 as std::os::raw::c_int) as
                    pngx_alloc_size_t).wrapping_div(::std::mem::size_of::<png_bytep>()
                                                        as std::os::raw::c_ulong) {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle exceedingly large image dimensions\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    if row_size < min_row_size { row_size = min_row_size }
    /* Deallocate the currently-existing rows. */
    png_free_data(png_ptr as *const png_struct, info_ptr,
                  0x40 as std::os::raw::c_uint, 0 as std::os::raw::c_int);
    /* Allocate memory for the row index. */
    rows =
        png_malloc(png_ptr as *const png_struct,
                   (height as
                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_bytep>()
                                                        as std::os::raw::c_ulong)) as
            png_bytepp;
    if rows.is_null() { return 0 as png_bytepp }
    /* Allocate memory for each row. */
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        row = png_malloc(png_ptr as *const png_struct, row_size) as png_bytep;
        if row.is_null() {
            /* Release the memory allocated up to the point of failure. */
            while i > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                i = i.wrapping_sub(1);
                png_free(png_ptr as *const png_struct,
                         *rows.offset(i as isize) as png_voidp);
            }
            png_free(png_ptr as *const png_struct, rows as png_voidp);
            return 0 as png_bytepp
        }
        if filler >= 0 as std::os::raw::c_int {
            memset(row as *mut std::os::raw::c_void, filler, row_size);
        }
        let ref mut fresh0 = *rows.offset(i as isize);
        *fresh0 = row;
        i = i.wrapping_add(1)
    }
    /* Set the row pointers. */
    png_set_rows(png_ptr as *const png_struct, info_ptr, rows);
    return rows;
}
/* PNG_INFO_IMAGE_SUPPORTED */
/* not necessary */
