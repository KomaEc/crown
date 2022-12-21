#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(__s: *mut std::os::raw::c_char, __n: std::os::raw::c_int, __stream: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    /* the query string (as it appears in the URI) */
    /*
 * This function is in tree.h:
 * xmlChar *	xmlNodeGetBase	(xmlDocPtr doc,
 *                               xmlNodePtr cur);
 */
    #[no_mangle]
    fn xmlCreateURI() -> xmlURIPtr;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base_0: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseURIReference(uri: xmlURIPtr, str: *const std::os::raw::c_char)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlPrintURI(stream: *mut FILE, uri: xmlURIPtr);
    #[no_mangle]
    fn xmlNormalizeURIPath(path: *mut std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlURIEscape(str: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
}
pub type xmlChar = std::os::raw::c_uchar;
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
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut std::os::raw::c_char,
    pub opaque: *mut std::os::raw::c_char,
    pub authority: *mut std::os::raw::c_char,
    pub server: *mut std::os::raw::c_char,
    pub user: *mut std::os::raw::c_char,
    pub port: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub query: *mut std::os::raw::c_char,
    pub fragment: *mut std::os::raw::c_char,
    pub cleanup: std::os::raw::c_int,
    pub query_raw: *mut std::os::raw::c_char,
}
/* *
 * Summary: library of generic URI related routines
 * Description: library of generic URI related routines
 *              Implements RFC 2396
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlURI:
 *
 * A parsed URI reference. This is a struct containing the various fields
 * as described in RFC 2396 but separated for further processing.
 *
 * Note: query is a deprecated field which is incorrectly unescaped.
 * query_raw takes precedence over query if the former is set.
 * See: http://mail.gnome.org/archives/xml/2007-April/thread.html#00127
 */
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
/*
 * testURI.c : a small tester program for XML input.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
static mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
static mut escape: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn handleURI(mut str: *const std::os::raw::c_char) {
    let mut ret: std::os::raw::c_int = 0;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    let mut parsed: *mut xmlChar = 0 as *mut xmlChar;
    uri = xmlCreateURI();
    if base.is_null() {
        ret = xmlParseURIReference(uri, str);
        if ret != 0 as std::os::raw::c_int {
            printf(b"%s : error %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                   str, ret);
        } else {
            if debug != 0 {
                if !(*uri).scheme.is_null() {
                    printf(b"scheme: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).scheme);
                }
                if !(*uri).opaque.is_null() {
                    printf(b"opaque: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).opaque);
                }
                if !(*uri).authority.is_null() {
                    printf(b"authority: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).authority);
                }
                if !(*uri).server.is_null() {
                    printf(b"server: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).server);
                }
                if !(*uri).user.is_null() {
                    printf(b"user: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).user);
                }
                if (*uri).port != 0 as std::os::raw::c_int {
                    printf(b"port: %d\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).port);
                }
                if !(*uri).path.is_null() {
                    printf(b"path: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).path);
                }
                if !(*uri).query.is_null() {
                    printf(b"query: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).query);
                }
                if !(*uri).fragment.is_null() {
                    printf(b"fragment: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).fragment);
                }
                if !(*uri).query_raw.is_null() {
                    printf(b"query_raw: %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*uri).query_raw);
                }
                if (*uri).cleanup != 0 as std::os::raw::c_int {
                    printf(b"cleanup\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                }
            }
            xmlNormalizeURIPath((*uri).path);
            if escape != 0 as std::os::raw::c_int {
                parsed = xmlSaveUri(uri);
                res = xmlURIEscape(parsed);
                printf(b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                       res as *mut std::os::raw::c_char);
            } else {
                xmlPrintURI(stdout, uri);
                printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
    } else {
        res = xmlBuildURI(str as *mut xmlChar, base as *mut xmlChar);
        if !res.is_null() {
            printf(b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                   res as *mut std::os::raw::c_char);
        } else {
            printf(b"::ERROR::\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
    }
    if !res.is_null() {
        xmlFree.expect("non-null function pointer")(res as *mut std::os::raw::c_void);
    }
    if !parsed.is_null() {
        xmlFree.expect("non-null function pointer")(parsed as
                                                        *mut std::os::raw::c_void);
    }
    xmlFreeURI(uri);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut arg: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if argc > arg && !(*argv.offset(arg as isize)).is_null() &&
           (strcmp(*argv.offset(arg as isize),
                   b"-base\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
                strcmp(*argv.offset(arg as isize),
                       b"--base\x00" as *const u8 as *const std::os::raw::c_char) ==
                    0) {
        arg += 1;
        base = *argv.offset(arg as isize);
        if !base.is_null() { arg += 1 }
    }
    if argc > arg && !(*argv.offset(arg as isize)).is_null() &&
           (strcmp(*argv.offset(arg as isize),
                   b"-escape\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
                strcmp(*argv.offset(arg as isize),
                       b"--escape\x00" as *const u8 as *const std::os::raw::c_char) ==
                    0) {
        arg += 1;
        escape += 1
    }
    if argc > arg && !(*argv.offset(arg as isize)).is_null() &&
           (strcmp(*argv.offset(arg as isize),
                   b"-debug\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
                strcmp(*argv.offset(arg as isize),
                       b"--debug\x00" as *const u8 as *const std::os::raw::c_char) ==
                    0) {
        arg += 1;
        debug += 1
    }
    if (*argv.offset(arg as isize)).is_null() {
        let mut str: [std::os::raw::c_char; 1024] = [0; 1024];
        /*
	     * read one line in string buffer.
	     */
        while !fgets(&mut *str.as_mut_ptr().offset(0 as std::os::raw::c_int as isize),
                     (::std::mem::size_of::<[std::os::raw::c_char; 1024]>() as
                          std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                         std::os::raw::c_int, stdin).is_null() {
            /*
	     * remove the ending spaces
	     */
            i = strlen(str.as_mut_ptr()) as std::os::raw::c_int;
            while i > 0 as std::os::raw::c_int &&
                      (str[(i - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int ==
                           '\n' as i32 ||
                           str[(i - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int
                               == '\r' as i32 ||
                           str[(i - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int
                               == ' ' as i32 ||
                           str[(i - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int
                               == '\t' as i32) {
                i -= 1;
                str[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char
            }
            handleURI(str.as_mut_ptr());
        }
    } else {
        while !(*argv.offset(arg as isize)).is_null() {
            handleURI(*argv.offset(arg as isize));
            arg += 1
        }
    }
    xmlMemoryDump();
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
