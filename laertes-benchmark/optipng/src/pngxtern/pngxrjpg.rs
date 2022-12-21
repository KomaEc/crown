
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    #[no_mangle]
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp)
     -> !;
    #[no_mangle]
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn memcmp(_: *const std::os::raw::c_void, _: *const std::os::raw::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
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
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const std::os::raw::c_char;
pub type png_const_charpp = *mut *const std::os::raw::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
static mut jpeg_sig_jp2: [png_byte; 12] =
    [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
     0 as std::os::raw::c_int as png_byte, 0xc as std::os::raw::c_int as png_byte,
     0x6a as std::os::raw::c_int as png_byte, 0x50 as std::os::raw::c_int as png_byte,
     0x20 as std::os::raw::c_int as png_byte, 0x20 as std::os::raw::c_int as png_byte,
     0xd as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte,
     0x87 as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte];
static mut jpeg_sig_jpc: [png_byte; 4] =
    [0xff as std::os::raw::c_int as png_byte, 0x4f as std::os::raw::c_int as png_byte,
     0xff as std::os::raw::c_int as png_byte, 0x51 as std::os::raw::c_int as png_byte];
static mut jpeg_sig_jng: [png_byte; 8] =
    [0x8b as std::os::raw::c_int as png_byte, 0x4a as std::os::raw::c_int as png_byte,
     0x4e as std::os::raw::c_int as png_byte, 0x47 as std::os::raw::c_int as png_byte,
     0xd as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte,
     0x1a as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte];
static mut jpeg_sig_jng_jhdr: [png_byte; 8] =
    [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
     0 as std::os::raw::c_int as png_byte, 0x1a as std::os::raw::c_int as png_byte,
     0x4a as std::os::raw::c_int as png_byte, 0x48 as std::os::raw::c_int as png_byte,
     0x44 as std::os::raw::c_int as png_byte, 0x52 as std::os::raw::c_int as png_byte];
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_jpeg(mut sig: png_bytep,
                                          mut sig_size: size_t,
                                          mut fmt_name_ptr: png_const_charpp,
                                          mut fmt_long_name_ptr:
                                              png_const_charpp)
 -> std::os::raw::c_int {
    let mut fmt: *const std::os::raw::c_char =
        0 as *const std::os::raw::c_char; /* not JPEG */
    let mut marker: std::os::raw::c_uint = 0;
    let mut result: std::os::raw::c_int = 0;
    if sig_size < 12 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    }
    if *sig.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0xff as std::os::raw::c_int &&
           *sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xd8 as std::os::raw::c_int &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xff as std::os::raw::c_int {
        marker =
            0xff00 as std::os::raw::c_uint |
                *sig.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        if marker >= 0xffc0 as std::os::raw::c_uint &&
               marker <= 0xffcf as std::os::raw::c_uint ||
               marker >= 0xffda as std::os::raw::c_uint &&
                   marker <= 0xfffe as std::os::raw::c_uint {
            fmt = b"JPEG\x00" as *const u8 as *const std::os::raw::c_char;
            result = 1 as std::os::raw::c_int
            /* not JPEG */
            /* JFIF, EXIF, JPEG-LS, codestream, etc. */
        } else { return 0 as std::os::raw::c_int }
    } else if memcmp(sig as *const std::os::raw::c_void,
                     jpeg_sig_jp2.as_ptr() as *const std::os::raw::c_void,
                     12 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int
                  ||
                  memcmp(sig as *const std::os::raw::c_void,
                         jpeg_sig_jpc.as_ptr() as *const std::os::raw::c_void,
                         4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int {
        fmt = b"JPEG-2000\x00" as *const u8 as *const std::os::raw::c_char;
        result = 2 as std::os::raw::c_int
        /* .jp2 or JPEG-2000 codestream */
    } else if memcmp(sig as *const std::os::raw::c_void,
                     jpeg_sig_jng.as_ptr() as *const std::os::raw::c_void,
                     8 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int ||
                  memcmp(sig as *const std::os::raw::c_void,
                         jpeg_sig_jng_jhdr.as_ptr() as *const std::os::raw::c_void,
                         8 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int {
        fmt = b"JNG\x00" as *const u8 as *const std::os::raw::c_char;
        result = 3 as std::os::raw::c_int
        /* JNG, standalone or datastream */
    } else { return 0 as std::os::raw::c_int }
    /* Store the format name. */
    if !fmt_name_ptr.is_null() { *fmt_name_ptr = fmt } /* not a JPEG file */
    if !fmt_long_name_ptr.is_null() { *fmt_long_name_ptr = fmt }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_jpeg(mut png_ptr: png_structp,
                                        mut info_ptr: png_infop,
                                        mut stream: *mut FILE)
 -> std::os::raw::c_int {
    let mut buf: [png_byte; 12] = [0; 12];
    let mut sig_code: std::os::raw::c_int = 0;
    if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
             12 as std::os::raw::c_int as std::os::raw::c_ulong,
             1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    sig_code =
        pngx_sig_is_jpeg(buf.as_mut_ptr(), 12 as std::os::raw::c_int as size_t,
                         0 as png_const_charpp, 0 as png_const_charpp);
    /* TODO: Use the format names passed by pngx_sig_is_jpeg. */
    match sig_code {
        1 => {
            png_error(png_ptr as *const png_struct,
                      b"JPEG decoding is not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
        2 => {
            png_error(png_ptr as *const png_struct,
                      b"JPEG-2000 decoding is not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
        }
        3 => {
            png_error(png_ptr as *const png_struct,
                      b"JNG (JPEG) decoding is not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
        }
        _ => { }
    }
    if info_ptr.is_null() {
        /* dummy, keep compilers happy */
        return 0 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
    /* always fail */
}
