#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlTextReader;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    /*
 * Summary: compile-time version informations
 * Description: compile-time version informations for the XML library
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /*
 * use those to be sure nothing nasty will happen if
 * your library and includes mismatch
 */
    #[no_mangle]
    fn xmlCheckVersion(version: std::os::raw::c_int);
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn xmlTextReaderByteConsumed(reader: xmlTextReaderPtr) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlTextReaderSetParserProp(reader: xmlTextReaderPtr, prop: std::os::raw::c_int,
                                  value: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderAttributeCount(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    #[no_mangle]
    fn xmlNewTextReaderFilename(URI: *const std::os::raw::c_char) -> xmlTextReaderPtr;
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlSubstituteEntitiesDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
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
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const XML_PARSER_SUBST_ENTITIES: C2RustUnnamed = 4;
pub const XML_PARSER_VALIDATE: C2RustUnnamed = 3;
pub const XML_PARSER_DEFAULTATTRS: C2RustUnnamed = 2;
pub const XML_PARSER_LOADDTD: C2RustUnnamed = 1;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
/*
 * testSAX.c : a small tester program for parsing using the SAX API.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut dump: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noent: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut valid: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut consumed: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn usage(mut progname: *const std::os::raw::c_char) {
    printf(b"Usage : %s [options] XMLfiles ...\n\x00" as *const u8 as
               *const std::os::raw::c_char, progname);
    printf(b"\tParse the XML files using the xmlTextReader API\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    printf(b"\t --count: count the number of attribute and elements\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    printf(b"\t --valid: validate the document\n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b"\t --consumed: count the number of bytes consumed\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    exit(1 as std::os::raw::c_int);
}
static mut elem: std::os::raw::c_int = 0;
static mut attrs: std::os::raw::c_int = 0;
unsafe extern "C" fn processNode(mut reader: xmlTextReaderPtr) {
    let mut type_0: std::os::raw::c_int = 0;
    type_0 = xmlTextReaderNodeType(reader);
    if count != 0 {
        if type_0 == 1 as std::os::raw::c_int {
            elem += 1;
            attrs += xmlTextReaderAttributeCount(reader)
        }
    };
}
unsafe extern "C" fn handleFile(mut filename: *const std::os::raw::c_char) {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: std::os::raw::c_int = 0;
    if count != 0 { elem = 0 as std::os::raw::c_int; attrs = 0 as std::os::raw::c_int }
    reader = xmlNewTextReaderFilename(filename);
    if !reader.is_null() {
        if valid != 0 {
            xmlTextReaderSetParserProp(reader,
                                       XML_PARSER_VALIDATE as std::os::raw::c_int,
                                       1 as std::os::raw::c_int);
        }
        /*
	 * Process all nodes in sequence
	 */
        ret = xmlTextReaderRead(reader);
        while ret == 1 as std::os::raw::c_int {
            processNode(reader);
            ret = xmlTextReaderRead(reader)
        }
        /*
	 * Done, cleanup and status
	 */
        if consumed != 0 {
            printf(b"%ld bytes consumed by parser\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   xmlTextReaderByteConsumed(reader));
        }
        xmlFreeTextReader(reader);
        if ret != 0 as std::os::raw::c_int {
            printf(b"%s : failed to parse\n\x00" as *const u8 as
                       *const std::os::raw::c_char, filename);
        } else if count != 0 {
            printf(b"%s : %d elements, %d attributes\n\x00" as *const u8 as
                       *const std::os::raw::c_char, filename, elem, attrs);
        }
    } else {
        fprintf(stderr,
                b"Unable to open %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
    };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut files: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if argc <= 1 as std::os::raw::c_int {
        usage(*argv.offset(0 as std::os::raw::c_int as isize));
        return 1 as std::os::raw::c_int
    }
    xmlCheckVersion(20908 as std::os::raw::c_int);
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-debug\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--debug\x00" as *const u8 as *const std::os::raw::c_char) == 0
           {
            debug += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-dump\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--dump\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            dump += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-count\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--count\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            count += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-consumed\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--consumed\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            consumed += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-valid\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--valid\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            valid += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-noent\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--noent\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            noent += 1
        }
        i += 1
    }
    if noent != 0 as std::os::raw::c_int {
        xmlSubstituteEntitiesDefault(1 as std::os::raw::c_int);
    }
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as std::os::raw::c_int as isize) as
               std::os::raw::c_int != '-' as i32 {
            handleFile(*argv.offset(i as isize));
            files += 1
        }
        i += 1
    }
    xmlCleanupParser();
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
/* LIBXML_READER_ENABLED */
