#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /*
 * Summary: string dictionary
 * Description: dictionary of reusable strings, just used to avoid allocation
 *         and freeing operations.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /*
 * The dictionary.
 */
    pub type _xmlDict;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    /* *
 * BAD_CAST:
 *
 * Macro to cast a string to an xmlChar * when one know its safe.
 */
    /*
 * xmlChar handling
 */
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrncatNew(str1: *const xmlChar, str2: *const xmlChar,
                     len: std::os::raw::c_int) -> *mut xmlChar;
    /*
 * Constructor and destructor.
 */
    #[no_mangle]
    fn xmlDictCreate() -> xmlDictPtr;
    #[no_mangle]
    fn xmlDictCreateSub(sub: xmlDictPtr) -> xmlDictPtr;
    #[no_mangle]
    fn xmlDictFree(dict: xmlDictPtr);
    /*
 * Lookup of entry in the dictionary.
 */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlDictQLookup(dict: xmlDictPtr, prefix: *const xmlChar,
                      name: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> std::os::raw::c_int;
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
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn xmlCleanupParser();
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
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
/*
 * Summary: interface for the memory allocator
 * Description: provides interfaces for the memory allocator,
 *              including debugging capabilities.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * DEBUG_MEMORY:
 *
 * DEBUG_MEMORY replaces the allocator with a collect and debug
 * shell to the libc allocator.
 * DEBUG_MEMORY should only be activated when debugging
 * libxml i.e. if libxml has been configured with --with-debug-mem too.
 */
/* #define DEBUG_MEMORY_FREED */
/* #define DEBUG_MEMORY_LOCATION */
/* *
 * DEBUG_MEMORY_LOCATION:
 *
 * DEBUG_MEMORY_LOCATION should be activated only when debugging
 * libxml i.e. if libxml has been configured with --with-debug-mem too.
 */
/*
 * The XML memory wrapper support 4 basic overloadable functions.
 */
/* *
 * xmlFreeFunc:
 * @mem: an already allocated block of memory
 *
 * Signature for a free() implementation.
 */
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
/* #define WITH_PRINT */
static mut seeds1: [*const std::os::raw::c_char; 13] =
    [b"a\x00" as *const u8 as *const std::os::raw::c_char,
     b"b\x00" as *const u8 as *const std::os::raw::c_char,
     b"c\x00" as *const u8 as *const std::os::raw::c_char,
     b"d\x00" as *const u8 as *const std::os::raw::c_char,
     b"e\x00" as *const u8 as *const std::os::raw::c_char,
     b"f\x00" as *const u8 as *const std::os::raw::c_char,
     b"g\x00" as *const u8 as *const std::os::raw::c_char,
     b"h\x00" as *const u8 as *const std::os::raw::c_char,
     b"i\x00" as *const u8 as *const std::os::raw::c_char,
     b"j\x00" as *const u8 as *const std::os::raw::c_char,
     b"k\x00" as *const u8 as *const std::os::raw::c_char,
     b"l\x00" as *const u8 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char];
static mut seeds2: [*const std::os::raw::c_char; 13] =
    [b"m\x00" as *const u8 as *const std::os::raw::c_char,
     b"n\x00" as *const u8 as *const std::os::raw::c_char,
     b"o\x00" as *const u8 as *const std::os::raw::c_char,
     b"p\x00" as *const u8 as *const std::os::raw::c_char,
     b"q\x00" as *const u8 as *const std::os::raw::c_char,
     b"r\x00" as *const u8 as *const std::os::raw::c_char,
     b"s\x00" as *const u8 as *const std::os::raw::c_char,
     b"t\x00" as *const u8 as *const std::os::raw::c_char,
     b"u\x00" as *const u8 as *const std::os::raw::c_char,
     b"v\x00" as *const u8 as *const std::os::raw::c_char,
     b"w\x00" as *const u8 as *const std::os::raw::c_char,
     b"x\x00" as *const u8 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char];
static mut strings1: [*mut xmlChar; 10000] =
    [0 as *const xmlChar as *mut xmlChar; 10000];
static mut strings2: [*mut xmlChar; 10000] =
    [0 as *const xmlChar as *mut xmlChar; 10000];
static mut test1: [*const xmlChar; 10000] = [0 as *const xmlChar; 10000];
static mut test2: [*const xmlChar; 10000] = [0 as *const xmlChar; 10000];
static mut nbErrors: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn fill_strings() {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    /*
     * That's a bit nasty but the output is fine and it doesn't take hours
     * there is a small but sufficient number of duplicates, and we have
     * ":xxx" and full QNames in the last NB_STRINGS_NS values
     */
    i = 0 as std::os::raw::c_int;
    while !seeds1[i as usize].is_null() {
        strings1[i as usize] =
            xmlStrdup(seeds1[i as usize] as *const xmlChar);
        if strings1[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings1\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        i += 1
    }
    j = 0 as std::os::raw::c_int;
    k = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int {
        strings1[i as usize] =
            xmlStrncatNew(strings1[j as usize], strings1[k as usize],
                          -(1 as std::os::raw::c_int));
        if strings1[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings1\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        if j >= 50 as std::os::raw::c_int { j = 0 as std::os::raw::c_int; k += 1 }
        i += 1;
        j += 1
    }
    j = 0 as std::os::raw::c_int;
    while j < 50 as std::os::raw::c_int && i < 10000 as std::os::raw::c_int {
        strings1[i as usize] =
            xmlStrncatNew(strings1[j as usize],
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar, -(1 as std::os::raw::c_int));
        if strings1[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings1\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        i += 1;
        j += 2 as std::os::raw::c_int
    }
    j = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    k = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        strings1[i as usize] =
            xmlStrncatNew(strings1[j as usize], strings1[k as usize],
                          -(1 as std::os::raw::c_int));
        if strings1[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings1\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        k += 3 as std::os::raw::c_int;
        if k >= 50 as std::os::raw::c_int { k = 0 as std::os::raw::c_int }
        i += 1;
        j += 1
    }
    /*
     * Now do the same with the second pool of strings
     */
    i = 0 as std::os::raw::c_int;
    while !seeds2[i as usize].is_null() {
        strings2[i as usize] =
            xmlStrdup(seeds2[i as usize] as *const xmlChar);
        if strings2[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings2\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        i += 1
    }
    j = 0 as std::os::raw::c_int;
    k = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int {
        strings2[i as usize] =
            xmlStrncatNew(strings2[j as usize], strings2[k as usize],
                          -(1 as std::os::raw::c_int));
        if strings2[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings2\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        if j >= 50 as std::os::raw::c_int { j = 0 as std::os::raw::c_int; k += 1 }
        i += 1;
        j += 1
    }
    j = 0 as std::os::raw::c_int;
    while j < 50 as std::os::raw::c_int && i < 10000 as std::os::raw::c_int {
        strings2[i as usize] =
            xmlStrncatNew(strings2[j as usize],
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar, -(1 as std::os::raw::c_int));
        if strings2[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings2\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        i += 1;
        j += 2 as std::os::raw::c_int
    }
    j = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    k = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        strings2[i as usize] =
            xmlStrncatNew(strings2[j as usize], strings2[k as usize],
                          -(1 as std::os::raw::c_int));
        if strings2[i as usize].is_null() {
            fprintf(stderr,
                    b"Out of memory while generating strings2\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            exit(1 as std::os::raw::c_int);
        }
        k += 3 as std::os::raw::c_int;
        if k >= 50 as std::os::raw::c_int { k = 0 as std::os::raw::c_int }
        i += 1;
        j += 1
    };
}
unsafe extern "C" fn clean_strings() {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if !strings1[i as usize].is_null() {
            /* really should not happen */
            xmlFree.expect("non-null function pointer")(strings1[i as usize]
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if !strings2[i as usize].is_null() {
            /* really should not happen */
            xmlFree.expect("non-null function pointer")(strings2[i as usize]
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    };
}
/*
 * This tests the sub-dictionary support
 */
unsafe extern "C" fn run_test2(mut parent: xmlDictPtr) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut prefix: [xmlChar; 40] = [0; 40];
    let mut cur: *mut xmlChar = 0 as *mut xmlChar;
    let mut pref: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    dict = xmlDictCreateSub(parent);
    if dict.is_null() {
        fprintf(stderr,
                b"Out of memory while creating sub-dictionary\n\x00" as
                    *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    memset(test2.as_mut_ptr() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<[*const xmlChar; 10000]>() as std::os::raw::c_ulong);
    /*
     * Fill in NB_STRINGS_MIN, at this point the dictionary should not grow
     * and we allocate all those doing the fast key computations
     * All the strings are based on a different seeds subset so we know
     * they are allocated in the main dictionary, not coming from the parent
     */
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        test2[i as usize] =
            xmlDictLookup(dict, strings2[i as usize], -(1 as std::os::raw::c_int));
        if test2[i as usize].is_null() {
            fprintf(stderr,
                    b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, strings2[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    j = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    /* ":foo" like strings2 */
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        test2[j as usize] =
            xmlDictLookup(dict, strings2[j as usize],
                          xmlStrlen(strings2[j as usize]));
        if test2[j as usize].is_null() {
            fprintf(stderr,
                    b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, strings2[j as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1;
        j += 1
    }
    /* "a:foo" like strings2 */
    j = 10000 as std::os::raw::c_int - 10 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        test2[j as usize] =
            xmlDictLookup(dict, strings2[j as usize],
                          xmlStrlen(strings2[j as usize]));
        if test2[j as usize].is_null() {
            fprintf(stderr,
                    b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, strings2[j as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1;
        j += 1
    }
    /*
     * At this point allocate all the strings
     * the dictionary will grow in the process, reallocate more string tables
     * and switch to the better key generator
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if test2[i as usize].is_null() {
            test2[i as usize] =
                xmlDictLookup(dict, strings2[i as usize],
                              -(1 as std::os::raw::c_int));
            if test2[i as usize].is_null() {
                fprintf(stderr,
                        b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, strings2[i as usize]);
                ret = 1 as std::os::raw::c_int;
                nbErrors += 1
            }
        }
        i += 1
    }
    /*
     * Now we can start to test things, first that all strings2 belongs to
     * the dict, and that none of them was actually allocated in the parent
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if xmlDictOwns(dict, test2[i as usize]) == 0 {
            fprintf(stderr,
                    b"Failed ownership failure for \'%s\'\n\x00" as *const u8
                        as *const std::os::raw::c_char, strings2[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        if xmlDictOwns(parent, test2[i as usize]) != 0 {
            fprintf(stderr,
                    b"Failed parent ownership failure for \'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    strings2[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * Also verify that all strings from the parent are seen from the subdict
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if xmlDictOwns(dict, test1[i as usize]) == 0 {
            fprintf(stderr,
                    b"Failed sub-ownership failure for \'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    strings1[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * Then that another lookup to the string in sub will return the same
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if xmlDictLookup(dict, strings2[i as usize], -(1 as std::os::raw::c_int)) !=
               test2[i as usize] {
            fprintf(stderr,
                    b"Failed re-lookup check for %d, \'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char, i,
                    strings2[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * But also that any lookup for a string in the parent will be provided
     * as in the parent
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if xmlDictLookup(dict, strings1[i as usize], -(1 as std::os::raw::c_int)) !=
               test1[i as usize] {
            fprintf(stderr,
                    b"Failed parent string lookup check for %d, \'%s\'\n\x00"
                        as *const u8 as *const std::os::raw::c_char, i,
                    strings1[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * check the QName lookups
     */
    i = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        cur = strings2[i as usize];
        pref =
            &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
                *mut xmlChar;
        while *cur as std::os::raw::c_int != ':' as i32 {
            let fresh0 = cur;
            cur = cur.offset(1);
            let fresh1 = pref;
            pref = pref.offset(1);
            *fresh1 = *fresh0
        }
        cur = cur.offset(1);
        *pref = 0 as std::os::raw::c_int as xmlChar;
        tmp =
            xmlDictQLookup(dict,
                           &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int
                                                                as isize),
                           cur);
        if tmp != test2[i as usize] {
            fprintf(stderr,
                    b"Failed lookup check for \'%s\':\'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                         isize) as
                        *mut xmlChar, cur);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * check the QName lookups for strings from the parent
     */
    i = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        cur = strings1[i as usize];
        pref =
            &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
                *mut xmlChar;
        while *cur as std::os::raw::c_int != ':' as i32 {
            let fresh2 = cur;
            cur = cur.offset(1);
            let fresh3 = pref;
            pref = pref.offset(1);
            *fresh3 = *fresh2
        }
        cur = cur.offset(1);
        *pref = 0 as std::os::raw::c_int as xmlChar;
        tmp =
            xmlDictQLookup(dict,
                           &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int
                                                                as isize),
                           cur);
        if xmlDictQLookup(dict,
                          &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                               isize), cur) !=
               test1[i as usize] {
            fprintf(stderr,
                    b"Failed parent lookup check for \'%s\':\'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                         isize) as
                        *mut xmlChar, cur);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    xmlDictFree(dict);
    return ret;
}
/*
 * Test a single dictionary
 */
unsafe extern "C" fn run_test1() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut prefix: [xmlChar; 40] = [0; 40];
    let mut cur: *mut xmlChar = 0 as *mut xmlChar;
    let mut pref: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    dict = xmlDictCreate();
    if dict.is_null() {
        fprintf(stderr,
                b"Out of memory while creating dictionary\n\x00" as *const u8
                    as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    memset(test1.as_mut_ptr() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<[*const xmlChar; 10000]>() as std::os::raw::c_ulong);
    /*
     * Fill in NB_STRINGS_MIN, at this point the dictionary should not grow
     * and we allocate all those doing the fast key computations
     */
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        test1[i as usize] =
            xmlDictLookup(dict, strings1[i as usize], -(1 as std::os::raw::c_int));
        if test1[i as usize].is_null() {
            fprintf(stderr,
                    b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, strings1[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    j = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    /* ":foo" like strings1 */
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        test1[j as usize] =
            xmlDictLookup(dict, strings1[j as usize],
                          xmlStrlen(strings1[j as usize]));
        if test1[j as usize].is_null() {
            fprintf(stderr,
                    b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, strings1[j as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1;
        j += 1
    }
    /* "a:foo" like strings1 */
    j = 10000 as std::os::raw::c_int - 10 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        test1[j as usize] =
            xmlDictLookup(dict, strings1[j as usize],
                          xmlStrlen(strings1[j as usize]));
        if test1[j as usize].is_null() {
            fprintf(stderr,
                    b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, strings1[j as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1;
        j += 1
    }
    /*
     * At this point allocate all the strings
     * the dictionary will grow in the process, reallocate more string tables
     * and switch to the better key generator
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if test1[i as usize].is_null() {
            test1[i as usize] =
                xmlDictLookup(dict, strings1[i as usize],
                              -(1 as std::os::raw::c_int));
            if test1[i as usize].is_null() {
                fprintf(stderr,
                        b"Failed lookup for \'%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, strings1[i as usize]);
                ret = 1 as std::os::raw::c_int;
                nbErrors += 1
            }
        }
        i += 1
    }
    /*
     * Now we can start to test things, first that all strings1 belongs to
     * the dict
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if xmlDictOwns(dict, test1[i as usize]) == 0 {
            fprintf(stderr,
                    b"Failed ownership failure for \'%s\'\n\x00" as *const u8
                        as *const std::os::raw::c_char, strings1[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * Then that another lookup to the string will return the same
     */
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        if xmlDictLookup(dict, strings1[i as usize], -(1 as std::os::raw::c_int)) !=
               test1[i as usize] {
            fprintf(stderr,
                    b"Failed re-lookup check for %d, \'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char, i,
                    strings1[i as usize]);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    /*
     * More complex, check the QName lookups
     */
    i = 10000 as std::os::raw::c_int - 100 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        cur = strings1[i as usize];
        pref =
            &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
                *mut xmlChar;
        while *cur as std::os::raw::c_int != ':' as i32 {
            let fresh4 = cur;
            cur = cur.offset(1);
            let fresh5 = pref;
            pref = pref.offset(1);
            *fresh5 = *fresh4
        }
        cur = cur.offset(1);
        *pref = 0 as std::os::raw::c_int as xmlChar;
        tmp =
            xmlDictQLookup(dict,
                           &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int
                                                                as isize),
                           cur);
        if tmp != test1[i as usize] {
            fprintf(stderr,
                    b"Failed lookup check for \'%s\':\'%s\'\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    &mut *prefix.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                         isize) as
                        *mut xmlChar, cur);
            ret = 1 as std::os::raw::c_int;
            nbErrors += 1
        }
        i += 1
    }
    run_test2(dict);
    xmlDictFree(dict);
    return ret;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlCheckVersion(20908 as std::os::raw::c_int);
    fill_strings();
    ret = run_test1();
    if ret == 0 as std::os::raw::c_int {
        printf(b"dictionary tests succeeded %d strings\n\x00" as *const u8 as
                   *const std::os::raw::c_char,
               2 as std::os::raw::c_int * 10000 as std::os::raw::c_int);
    } else {
        printf(b"dictionary tests failed with %d errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nbErrors);
    }
    clean_strings();
    xmlCleanupParser();
    xmlMemoryDump();
    return ret;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
