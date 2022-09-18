use ::libc;
extern "C" {
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
#[no_mangle]
pub unsafe extern "C" fn archive_version_details() -> *const libc::c_char {
    static mut str: archive_string = archive_string {
        s: 0 as *const libc::c_char as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    static mut init: libc::c_int = 0 as libc::c_int;
    let mut zlib: *const libc::c_char = archive_zlib_version();
    let mut liblzma: *const libc::c_char = archive_liblzma_version();
    let mut bzlib: *const libc::c_char = archive_bzlib_version();
    let mut liblz4: *const libc::c_char = archive_liblz4_version();
    let mut libzstd: *const libc::c_char = archive_libzstd_version();
    if init == 0 {
        str.s = NULL as *mut libc::c_char;
        str.length = 0 as libc::c_int as size_t;
        str.buffer_length = 0 as libc::c_int as size_t;
        archive_strcat(
            &mut str,
            b"libarchive 3.4.3\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        if !zlib.is_null() {
            archive_strcat(
                &mut str,
                b" zlib/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            archive_strcat(&mut str, zlib as *const libc::c_void);
        }
        if !liblzma.is_null() {
            archive_strcat(
                &mut str,
                b" liblzma/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            archive_strcat(&mut str, liblzma as *const libc::c_void);
        }
        if !bzlib.is_null() {
            let mut p: *const libc::c_char = bzlib;
            let mut sep: *const libc::c_char = strchr(p, ',' as i32);
            if sep.is_null() {
                sep = p.offset(strlen(p) as isize)
            }
            archive_strcat(
                &mut str,
                b" bz2lib/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            archive_strncat(
                &mut str,
                p as *const libc::c_void,
                sep.offset_from(p) as libc::c_long as size_t,
            );
        }
        if !liblz4.is_null() {
            archive_strcat(
                &mut str,
                b" liblz4/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            archive_strcat(&mut str, liblz4 as *const libc::c_void);
        }
        if !libzstd.is_null() {
            archive_strcat(
                &mut str,
                b" libzstd/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            archive_strcat(&mut str, libzstd as *const libc::c_void);
        }
    }
    return str.s;
}
#[no_mangle]
pub unsafe extern "C" fn archive_zlib_version() -> *const libc::c_char {
    return ZLIB_VERSION.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn archive_liblzma_version() -> *const libc::c_char {
    return NULL as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_bzlib_version() -> *const libc::c_char {
    return NULL as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_liblz4_version() -> *const libc::c_char {
    return NULL as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_libzstd_version() -> *const libc::c_char {
    return NULL as *const libc::c_char;
}
