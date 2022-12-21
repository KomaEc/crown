#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlModule;
    #[no_mangle]
    fn xmlStrPrintf(buf: *mut xmlChar, len: std::os::raw::c_int,
                    msg: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlModuleOpen(filename: *const std::os::raw::c_char, options: std::os::raw::c_int)
     -> xmlModulePtr;
    #[no_mangle]
    fn xmlModuleSymbol(module: xmlModulePtr, name: *const std::os::raw::c_char,
                       result: *mut *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlModuleClose(module: xmlModulePtr) -> std::os::raw::c_int;
}
/*
 * Summary: set of routines to process strings
 * Description: type and interfaces needed for the internal string handling
 *              of the library, especially UTF8 processing.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlChar:
 *
 * This is a basic byte in an UTF-8 encoded string.
 * It's unsigned allowing to pinpoint case where char * are assigned
 * to xmlChar * (possibly making serialization back impossible).
 */
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
/*
 * Summary: dynamic module loading
 * Description: basic API for dynamic module loading, used by
 *              libexslt added in 2.6.17
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Joel W. Reed
 */
/* *
 * xmlModulePtr:
 *
 * A handle to a dynamically loaded module
 */
pub type xmlModule = _xmlModule;
pub type xmlModulePtr = *mut xmlModule;
/* Used for SCO Openserver*/
pub type hello_world_t = Option<unsafe extern "C" fn() -> std::os::raw::c_int>;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut filename: [xmlChar; 4096] = [0; 4096];
    let mut module: xmlModulePtr = 0 as xmlModulePtr;
    let mut hello_world: hello_world_t = None;
    /* build the module filename, and confirm the module exists */
    xmlStrPrintf(filename.as_mut_ptr(),
                 ::std::mem::size_of::<[xmlChar; 4096]>() as std::os::raw::c_ulong as
                     std::os::raw::c_int,
                 b"%s/testdso%s\x00" as *const u8 as *const std::os::raw::c_char,
                 b".libs\x00" as *const u8 as *const std::os::raw::c_char as
                     *const xmlChar,
                 b".so\x00" as *const u8 as *const std::os::raw::c_char as
                     *const xmlChar);
    module =
        xmlModuleOpen(filename.as_mut_ptr() as *const std::os::raw::c_char,
                      0 as std::os::raw::c_int);
    if !module.is_null() {
        if xmlModuleSymbol(module,
                           b"hello_world\x00" as *const u8 as
                               *const std::os::raw::c_char,
                           &mut hello_world as *mut hello_world_t as
                               *mut *mut std::os::raw::c_void) != 0 {
            fprintf(stderr,
                    b"Failure to lookup\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            return 1 as std::os::raw::c_int
        }
        if hello_world.is_none() {
            fprintf(stderr,
                    b"Lookup returned NULL\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            return 1 as std::os::raw::c_int
        }
        Some(hello_world.expect("non-null function pointer")).expect("non-null function pointer")();
        xmlModuleClose(module);
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
/* LIBXML_SCHEMAS_ENABLED */
