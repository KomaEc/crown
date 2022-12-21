
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    #[no_mangle]
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t)
     -> png_voidp;
    #[no_mangle]
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
    #[no_mangle]
    fn png_warning(png_ptr: png_const_structrp,
                   warning_message: png_const_charp);
    #[no_mangle]
    fn png_set_IHDR(png_ptr: png_const_structrp, info_ptr: png_inforp,
                    width: png_uint_32, height: png_uint_32,
                    bit_depth: std::os::raw::c_int, color_type: std::os::raw::c_int,
                    interlace_method: std::os::raw::c_int,
                    compression_method: std::os::raw::c_int,
                    filter_method: std::os::raw::c_int);
    #[no_mangle]
    fn png_set_sBIT(png_ptr: png_const_structrp, info_ptr: png_inforp,
                    sig_bit: png_const_color_8p);
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ungetc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn pngx_malloc_rows(png_ptr: png_structp, info_ptr: png_infop,
                        filler: std::os::raw::c_int) -> png_bytepp;
    #[no_mangle]
    fn pnm_fget_header(pnm_ptr: *mut pnm_struct, stream: *mut FILE)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn pnm_fget_values(pnm_ptr: *const pnm_struct,
                       sample_values: *mut std::os::raw::c_uint,
                       num_rows: std::os::raw::c_uint, stream: *mut FILE)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn pnm_fget_bytes(pnm_ptr: *const pnm_struct,
                      sample_bytes: *mut std::os::raw::c_uchar, sample_size: size_t,
                      num_rows: std::os::raw::c_uint, stream: *mut FILE)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
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
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = size_t;
pub type png_alloc_size_t = png_size_t;
pub type png_voidp = *mut std::os::raw::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const std::os::raw::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_const_charpp = *mut *const std::os::raw::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_color_8 = png_color_8_struct;
pub type png_const_color_8p = *const png_color_8;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: std::os::raw::c_uint,
    pub depth: std::os::raw::c_uint,
    pub width: std::os::raw::c_uint,
    pub height: std::os::raw::c_uint,
    pub maxval: std::os::raw::c_uint,
}
static mut pbm_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PBM\x00")
    };
static mut pgm_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PGM\x00")
    };
static mut ppm_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PPM\x00")
    };
static mut pam_fmt_name: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"PAM\x00")
    };
static mut pbm_fmt_long_name: [std::os::raw::c_char; 16] =
    unsafe {
        *::std::mem::transmute::<&[u8; 16],
                                 &[std::os::raw::c_char; 16]>(b"Portable Bitmap\x00")
    };
static mut pgm_fmt_long_name: [std::os::raw::c_char; 17] =
    unsafe {
        *::std::mem::transmute::<&[u8; 17],
                                 &[std::os::raw::c_char; 17]>(b"Portable Graymap\x00")
    };
static mut ppm_fmt_long_name: [std::os::raw::c_char; 16] =
    unsafe {
        *::std::mem::transmute::<&[u8; 16],
                                 &[std::os::raw::c_char; 16]>(b"Portable Pixmap\x00")
    };
static mut pam_fmt_long_name: [std::os::raw::c_char; 16] =
    unsafe {
        *::std::mem::transmute::<&[u8; 16],
                                 &[std::os::raw::c_char; 16]>(b"Portable Anymap\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_pnm(mut sig: png_bytep,
                                         mut sig_size: size_t,
                                         mut fmt_name_ptr: png_const_charpp,
                                         mut fmt_long_name_ptr:
                                             png_const_charpp)
 -> std::os::raw::c_int {
    static mut fmt_names: [*const std::os::raw::c_char; 7] =
        unsafe {
            [pbm_fmt_name.as_ptr(), pgm_fmt_name.as_ptr(),
             ppm_fmt_name.as_ptr(), pbm_fmt_name.as_ptr(),
             pgm_fmt_name.as_ptr(), ppm_fmt_name.as_ptr(),
             pam_fmt_name.as_ptr()]
        };
    static mut fmt_long_names: [*const std::os::raw::c_char; 7] =
        unsafe {
            [pbm_fmt_long_name.as_ptr(), pgm_fmt_long_name.as_ptr(),
             ppm_fmt_long_name.as_ptr(), pbm_fmt_long_name.as_ptr(),
             pgm_fmt_long_name.as_ptr(), ppm_fmt_long_name.as_ptr(),
             pam_fmt_long_name.as_ptr()]
        };
    /* Require at least the PNM magic signature and the trailing whitespace. */
    if sig_size < 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    } /* insufficient data */
    if *sig.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != 'P' as i32 ||
           (*sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '1' as i32 ||
           *sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '7' as i32
       {
        return 0 as std::os::raw::c_int
    } /* not PNM */
    if *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int != ' ' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '\t' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '\n' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '\r' as i32 &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int != '#' as i32
       {
        return 0 as std::os::raw::c_int
    } /* not PNM */
    /* Store the format name. */
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr =
            fmt_names[(*sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                           '1' as i32) as usize]
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr =
            fmt_long_names[(*sig.offset(1 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int - '1' as i32) as usize]
    }
    return 1 as std::os::raw::c_int;
    /* PNM */
}
unsafe extern "C" fn pnm_fpeek_eof(mut pnm_ptr: *mut pnm_struct,
                                   mut stream: *mut FILE) -> std::os::raw::c_int {
    let mut ch: std::os::raw::c_int = 0;
    if (*pnm_ptr).format >= PNM_P1 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*pnm_ptr).format <= PNM_P3 as std::os::raw::c_int as std::os::raw::c_uint {
        loop  {
            ch = getc(stream);
            if ch == '#' as i32 {
                /* skip comments */
                loop  {
                    ch = getc(stream);
                    if !(ch != -(1 as std::os::raw::c_int) && ch != '\n' as i32 &&
                             ch != '\r' as i32) {
                        break ;
                    }
                }
            }
            if ch == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int }
            if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 ||
                     ch == '\r' as i32) {
                break ;
            }
        }
    } else {
        ch = getc(stream);
        if ch == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int }
    }
    ungetc(ch, stream);
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_pnm(mut png_ptr: png_structp,
                                       mut info_ptr: png_infop,
                                       mut stream: *mut FILE) -> std::os::raw::c_int {
    let mut pnminfo: pnm_struct =
        pnm_struct{format: 0, depth: 0, width: 0, height: 0, maxval: 0,};
    let mut format: std::os::raw::c_uint = 0;
    let mut depth: std::os::raw::c_uint = 0;
    let mut width: std::os::raw::c_uint = 0;
    let mut height: std::os::raw::c_uint = 0;
    let mut maxval: std::os::raw::c_uint = 0;
    let mut max_width: std::os::raw::c_uint = 0;
    let mut num_samples: std::os::raw::c_uint = 0;
    let mut sample_size: std::os::raw::c_uint = 0;
    let mut pnmrow: *mut std::os::raw::c_uint = 0 as *mut std::os::raw::c_uint;
    let mut row_size: size_t = 0;
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut sig_bit: png_color_8 =
        png_color_8{red: 0, green: 0, blue: 0, gray: 0, alpha: 0,};
    let mut i: std::os::raw::c_uint = 0;
    let mut j: std::os::raw::c_uint = 0;
    let mut failed: std::os::raw::c_int = 0;
    let mut overflow: std::os::raw::c_int = 0;
    /* Read the PNM header. */
    if pnm_fget_header(&mut pnminfo, stream) != 1 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    } /* not PNM */
    format = pnminfo.format;
    depth = pnminfo.depth;
    width = pnminfo.width;
    height = pnminfo.height;
    maxval = pnminfo.maxval;
    if format > PNM_P6 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle PNM formats newer than PPM (\"P6\")\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    max_width =
        if ::std::mem::size_of::<size_t>() as std::os::raw::c_ulong <=
               ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong {
            ((2147483647 as std::os::raw::c_int as
                  std::os::raw::c_uint).wrapping_mul(2 as
                                                 std::os::raw::c_uint).wrapping_add(1
                                                                                as
                                                                                std::os::raw::c_uint)
                 as
                 std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_uint>()
                                                 as
                                                 std::os::raw::c_ulong).wrapping_div(depth
                                                                                 as
                                                                                 std::os::raw::c_ulong)
        } else {
            (2147483647 as std::os::raw::c_int as
                 std::os::raw::c_uint).wrapping_mul(2 as
                                                std::os::raw::c_uint).wrapping_add(1
                                                                               as
                                                                               std::os::raw::c_uint)
                as std::os::raw::c_ulong
        } as std::os::raw::c_uint;
    if max_width > 0x7fffffff as std::os::raw::c_uint {
        max_width = 0x7fffffff as std::os::raw::c_uint
    }
    if width > max_width {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle exceedingly large PNM dimensions\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    sample_size = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    num_samples = depth.wrapping_mul(width);
    row_size = num_samples as size_t;
    if maxval > 65535 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle PNM samples larger than 16 bits\x00" as
                      *const u8 as *const std::os::raw::c_char);
    } else if maxval > 255 as std::os::raw::c_int as std::os::raw::c_uint {
        sample_size = 2 as std::os::raw::c_int as std::os::raw::c_uint;
        row_size =
            (row_size as
                 std::os::raw::c_ulong).wrapping_mul(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t
    }
    /* Set the PNG image type. */
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 if maxval <= 255 as std::os::raw::c_int as std::os::raw::c_uint {
                     8 as std::os::raw::c_int
                 } else { 16 as std::os::raw::c_int },
                 if depth == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                     0 as std::os::raw::c_int
                 } else { 2 as std::os::raw::c_int }, 0 as std::os::raw::c_int,
                 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    i = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    j = 2 as std::os::raw::c_int as std::os::raw::c_uint;
    while j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) < maxval {
        i = i.wrapping_add(1);
        j <<= 1 as std::os::raw::c_int
    }
    if j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) != maxval {
        png_warning(png_ptr as *const png_struct,
                    b"Possibly inexact sample conversion from PNM to PNG\x00"
                        as *const u8 as *const std::os::raw::c_char);
    } else if i.wrapping_rem(8 as std::os::raw::c_int as std::os::raw::c_uint) !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint &&
                  (depth > 1 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (8 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_rem(i) !=
                           0 as std::os::raw::c_int as std::os::raw::c_uint) {
        sig_bit.gray = i as png_byte;
        sig_bit.blue = sig_bit.gray;
        sig_bit.green = sig_bit.blue;
        sig_bit.red = sig_bit.green;
        sig_bit.alpha = 0 as std::os::raw::c_int as png_byte;
        png_set_sBIT(png_ptr as *const png_struct, info_ptr,
                     &mut sig_bit as *mut png_color_8 as png_const_color_8p);
    }
    /* Allocate memory. */
    row_pointers =
        pngx_malloc_rows(png_ptr, info_ptr,
                         -(1 as
                               std::os::raw::c_int)); /* can read raw data directly into row_pointers */
    if format >= PNM_P4 as std::os::raw::c_int as std::os::raw::c_uint &&
           (maxval == 255 as std::os::raw::c_int as std::os::raw::c_uint ||
                maxval == 65535 as std::os::raw::c_int as std::os::raw::c_uint) {
        pnmrow = 0 as *mut std::os::raw::c_uint
    } else {
        pnmrow =
            png_malloc(png_ptr as *const png_struct,
                       (num_samples as
                            std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>()
                                                            as std::os::raw::c_ulong))
                as *mut std::os::raw::c_uint
    }
    /* Read the image data. */
    failed = 0 as std::os::raw::c_int;
    overflow = 0 as std::os::raw::c_int;
    if !pnmrow.is_null() {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < height {
            if pnm_fget_values(&mut pnminfo, pnmrow,
                               1 as std::os::raw::c_int as std::os::raw::c_uint, stream) <=
                   0 as std::os::raw::c_int {
                failed = 1 as std::os::raw::c_int
            }
            /* Transfer the samples, even on partial (unsuccessful) reads. */
            if maxval <= 255 as std::os::raw::c_int as std::os::raw::c_uint {
                j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while j < num_samples {
                    let mut val: std::os::raw::c_uint = *pnmrow.offset(j as isize);
                    if val > maxval {
                        val = 255 as std::os::raw::c_int as std::os::raw::c_uint;
                        overflow = 1 as std::os::raw::c_int
                    } else if maxval != 255 as std::os::raw::c_int as std::os::raw::c_uint {
                        val =
                            val.wrapping_mul(255 as std::os::raw::c_int as
                                                 std::os::raw::c_uint).wrapping_add(maxval.wrapping_div(2
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint)).wrapping_div(maxval)
                    }
                    *(*row_pointers.offset(i as isize)).offset(j as isize) =
                        val as png_byte;
                    j = j.wrapping_add(1)
                }
            } else {
                /* maxval > 255 */
                j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while j < num_samples {
                    let mut val_0: png_uint_32 = *pnmrow.offset(j as isize);
                    if val_0 > maxval {
                        val_0 = 65535 as std::os::raw::c_int as png_uint_32;
                        overflow = 1 as std::os::raw::c_int
                    } else if maxval != 65535 as std::os::raw::c_int as std::os::raw::c_uint {
                        val_0 =
                            val_0.wrapping_mul(65535 as std::os::raw::c_int as
                                                   std::os::raw::c_uint).wrapping_add(maxval.wrapping_div(2
                                                                                                      as
                                                                                                      std::os::raw::c_int
                                                                                                      as
                                                                                                      std::os::raw::c_uint)).wrapping_div(maxval)
                    }
                    *(*row_pointers.offset(i as
                                               isize)).offset((2 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_uint).wrapping_mul(j)
                                                                  as isize) =
                        (val_0 >> 8 as std::os::raw::c_int) as png_byte;
                    *(*row_pointers.offset(i as
                                               isize)).offset((2 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_uint).wrapping_mul(j).wrapping_add(1
                                                                                                                  as
                                                                                                                  std::os::raw::c_int
                                                                                                                  as
                                                                                                                  std::os::raw::c_uint)
                                                                  as isize) =
                        (val_0 & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                            png_byte;
                    j = j.wrapping_add(1)
                }
            }
            if failed != 0 { break ; }
            i = i.wrapping_add(1)
        }
    } else {
        /* read the raw data directly */
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < height {
            if pnm_fget_bytes(&mut pnminfo, *row_pointers.offset(i as isize),
                              sample_size as size_t,
                              1 as std::os::raw::c_int as std::os::raw::c_uint, stream) <=
                   0 as std::os::raw::c_int {
                failed = 1 as std::os::raw::c_int;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    }
    /* Wipe out the portion left unread. */
    while i < height {
        memset(*row_pointers.offset(i as isize) as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int, row_size);
        i = i.wrapping_add(1)
    }
    /* Deallocate the temporary row buffer. */
    if !pnmrow.is_null() {
        png_free(png_ptr as *const png_struct, pnmrow as png_voidp);
    }
    /* Check the results. */
    if overflow != 0 {
        png_warning(png_ptr as *const png_struct,
                    b"Overflow in PNM samples\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    if failed != 0 {
        png_error(png_ptr as *const png_struct,
                  b"Error in PNM image file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    } else if pnm_fpeek_eof(&mut pnminfo, stream) == 0 {
        png_warning(png_ptr as *const png_struct,
                    b"Extraneous data found after PNM image\x00" as *const u8
                        as *const std::os::raw::c_char);
    }
    /* FIXME: A PNM file can have more than one image. */
    return 1 as std::os::raw::c_int;
    /* one image has been successfully read */
}
