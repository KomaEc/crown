
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
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
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
    fn xmlReaderForFile(filename: *const std::os::raw::c_char,
                        encoding: *const std::os::raw::c_char, options: std::os::raw::c_int)
     -> xmlTextReaderPtr;
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    #[no_mangle]
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderDepth(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderHasValue(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextReaderConstName(reader: xmlTextReaderPtr) -> *const xmlChar;
    #[no_mangle]
    fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const xmlChar;
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
pub type xmlChar = std::os::raw::c_uchar;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
/* *
 * section: xmlReader
 * synopsis: Parse an XML file with an xmlReader
 * purpose: Demonstrate the use of xmlReaderForFile() to parse an XML file
 *          and dump the informations about the nodes found in the process.
 *          (Note that the XMLReader functions require libxml2 version later
 *          than 2.6.)
 * usage: reader1 <filename>
 * test: reader1 test2.xml > reader1.tmp && diff reader1.tmp $(srcdir)/reader1.res
 * author: Daniel Veillard
 * copy: see Copyright for the status of this software.
 */
/* *
 * processNode:
 * @reader: the xmlReader
 *
 * Dump information about the current node
 */
unsafe extern "C" fn processNode(mut reader: xmlTextReaderPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    name = xmlTextReaderConstName(reader);
    if name.is_null() {
        name = b"--\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
    }
    value = xmlTextReaderConstValue(reader);
    printf(b"%d %d %s %d %d\x00" as *const u8 as *const std::os::raw::c_char,
           xmlTextReaderDepth(reader), xmlTextReaderNodeType(reader), name,
           xmlTextReaderIsEmptyElement(reader),
           xmlTextReaderHasValue(reader));
    if value.is_null() {
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else if xmlStrlen(value) > 40 as std::os::raw::c_int {
        printf(b" %.40s...\n\x00" as *const u8 as *const std::os::raw::c_char, value);
    } else {
        printf(b" %s\n\x00" as *const u8 as *const std::os::raw::c_char, value);
    };
}
/* *
 * streamFile:
 * @filename: the file name to parse
 *
 * Parse and print information about an XML file.
 */
unsafe extern "C" fn streamFile(mut filename: *const std::os::raw::c_char) {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: std::os::raw::c_int = 0;
    reader =
        xmlReaderForFile(filename, 0 as *const std::os::raw::c_char,
                         0 as std::os::raw::c_int);
    if !reader.is_null() {
        ret = xmlTextReaderRead(reader);
        while ret == 1 as std::os::raw::c_int {
            processNode(reader);
            ret = xmlTextReaderRead(reader)
        }
        xmlFreeTextReader(reader);
        if ret != 0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"%s : failed to parse\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
        }
    } else {
        fprintf(stderr,
                b"Unable to open %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
    };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    if argc != 2 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    /*
     * this initialize the library and check potential ABI mismatches
     * between the version it was compiled for and the actual shared
     * library used.
     */
    xmlCheckVersion(20908 as std::os::raw::c_int);
    streamFile(*argv.offset(1 as std::os::raw::c_int as isize));
    /*
     * Cleanup function for the XML library.
     */
    xmlCleanupParser();
    /*
     * this is to debug memory for regression tests
     */
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
