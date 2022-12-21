
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
    /*
 * Summary: regular expressions handling
 * Description: basic API for libxml regular expressions handling used
 *              for XML Schemas and validation.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlRegexpPtr:
 *
 * A libxml regular expression, they can actually be far more complex
 * thank the POSIX regex expressions.
 */
    pub type _xmlRegexp;
    /*
 * Formal regular expression handling
 * Its goal is to do some formal work on content models
 */
    /* expressions are used within a context */
    pub type _xmlExpCtxt;
    /* Expressions are trees but the tree is opaque */
    pub type _xmlExpNode;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(__s: *mut std::os::raw::c_char, __n: std::os::raw::c_int, __stream: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    /*
 * The POSIX like API
 */
    #[no_mangle]
    fn xmlRegexpCompile(regexp: *const xmlChar) -> xmlRegexpPtr;
    #[no_mangle]
    fn xmlRegFreeRegexp(regexp: xmlRegexpPtr);
    #[no_mangle]
    fn xmlRegexpExec(comp: xmlRegexpPtr, value: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRegexpPrint(output: *mut FILE, regexp: xmlRegexpPtr);
    #[no_mangle]
    fn xmlExpFreeCtxt(ctxt: xmlExpCtxtPtr);
    #[no_mangle]
    fn xmlExpNewCtxt(maxNodes: std::os::raw::c_int, dict: xmlDictPtr)
     -> xmlExpCtxtPtr;
    #[no_mangle]
    fn xmlExpCtxtNbNodes(ctxt: xmlExpCtxtPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlExpCtxtNbCons(ctxt: xmlExpCtxtPtr) -> std::os::raw::c_int;
    /*
 * Expressions are reference counted internally
 */
    #[no_mangle]
    fn xmlExpFree(ctxt: xmlExpCtxtPtr, expr: xmlExpNodePtr);
    /*
 * constructors can be either manual or from a string
 */
    #[no_mangle]
    fn xmlExpParse(ctxt: xmlExpCtxtPtr, expr: *const std::os::raw::c_char)
     -> xmlExpNodePtr;
    /*
 * The really interesting APIs
 */
    #[no_mangle]
    fn xmlExpIsNillable(expr: xmlExpNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlExpMaxToken(expr: xmlExpNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlExpGetLanguage(ctxt: xmlExpCtxtPtr, expr: xmlExpNodePtr,
                         langList: *mut *const xmlChar, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlExpStringDerive(ctxt: xmlExpCtxtPtr, expr: xmlExpNodePtr,
                          str: *const xmlChar, len: std::os::raw::c_int)
     -> xmlExpNodePtr;
    #[no_mangle]
    fn xmlExpExpDerive(ctxt: xmlExpCtxtPtr, expr: xmlExpNodePtr,
                       sub: xmlExpNodePtr) -> xmlExpNodePtr;
    #[no_mangle]
    fn xmlExpSubsume(ctxt: xmlExpCtxtPtr, expr: xmlExpNodePtr,
                     sub: xmlExpNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlExpDump(buf: xmlBufferPtr, expr: xmlExpNodePtr);
    #[no_mangle]
    fn xmlBufferCreate() -> xmlBufferPtr;
    #[no_mangle]
    fn xmlBufferFree(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlBufferEmpty(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlBufferContent(buf: *const xmlBuffer) -> *const xmlChar;
    /*
 * Initialization of the memory layer.
 */
    #[no_mangle]
    fn xmlInitMemory() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
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
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlBufferAllocationScheme = std::os::raw::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: std::os::raw::c_uint,
    pub size: std::os::raw::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
pub type xmlExpCtxt = _xmlExpCtxt;
pub type xmlExpCtxtPtr = *mut xmlExpCtxt;
pub type xmlExpNode = _xmlExpNode;
pub type xmlExpNodePtr = *mut xmlExpNode;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/*
 * testRegexp.c: simple module for testing regular expressions
 *
 * See Copyright for the status of this software.
 *
 * Daniel Veillard <veillard@redhat.com>
 */
static mut repeat: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn testRegexp(mut comp: xmlRegexpPtr,
                                mut value: *const std::os::raw::c_char) {
    let mut ret: std::os::raw::c_int = 0;
    ret = xmlRegexpExec(comp, value as *const xmlChar);
    if ret == 1 as std::os::raw::c_int {
        printf(b"%s: Ok\n\x00" as *const u8 as *const std::os::raw::c_char, value);
    } else if ret == 0 as std::os::raw::c_int {
        printf(b"%s: Fail\n\x00" as *const u8 as *const std::os::raw::c_char, value);
    } else {
        printf(b"%s: Error: %d\n\x00" as *const u8 as *const std::os::raw::c_char,
               value, ret);
    }
    if repeat != 0 {
        let mut j: std::os::raw::c_int = 0;
        j = 0 as std::os::raw::c_int;
        while j < 999999 as std::os::raw::c_int {
            xmlRegexpExec(comp, value as *const xmlChar);
            j += 1
        }
    };
}
unsafe extern "C" fn testRegexpFile(mut filename: *const std::os::raw::c_char) {
    let mut comp: xmlRegexpPtr = 0 as xmlRegexpPtr;
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut expression: [std::os::raw::c_char; 5000] = [0; 5000];
    let mut len: std::os::raw::c_int = 0;
    input = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if input.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Cannot open %s for reading\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   filename);
        return
    }
    while !fgets(expression.as_mut_ptr(), 4500 as std::os::raw::c_int,
                 input).is_null() {
        len = strlen(expression.as_mut_ptr()) as std::os::raw::c_int;
        len -= 1;
        while len >= 0 as std::os::raw::c_int &&
                  (expression[len as usize] as std::os::raw::c_int == '\n' as i32 ||
                       expression[len as usize] as std::os::raw::c_int == '\t' as i32
                       ||
                       expression[len as usize] as std::os::raw::c_int == '\r' as i32
                       ||
                       expression[len as usize] as std::os::raw::c_int == ' ' as i32)
              {
            len -= 1
        }
        expression[(len + 1 as std::os::raw::c_int) as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        if !(len >= 0 as std::os::raw::c_int) { continue ; }
        if expression[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '#' as i32
           {
            continue ;
        }
        if expression[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '=' as i32
               &&
               expression[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   '>' as i32 {
            let mut pattern: *mut std::os::raw::c_char =
                &mut *expression.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                                         isize) as
                    *mut std::os::raw::c_char;
            if !comp.is_null() {
                xmlRegFreeRegexp(comp);
                comp = 0 as xmlRegexpPtr
            }
            printf(b"Regexp: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                   pattern);
            comp = xmlRegexpCompile(pattern as *const xmlChar);
            if !comp.is_null() { continue ; }
            printf(b"   failed to compile\n\x00" as *const u8 as
                       *const std::os::raw::c_char);
            break ;
        } else if comp.is_null() {
            printf(b"Regexp: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                   expression.as_mut_ptr());
            comp =
                xmlRegexpCompile(expression.as_mut_ptr() as *const xmlChar);
            if !comp.is_null() { continue ; }
            printf(b"   failed to compile\n\x00" as *const u8 as
                       *const std::os::raw::c_char);
            break ;
        } else if !comp.is_null() {
            testRegexp(comp, expression.as_mut_ptr());
        }
    }
    fclose(input);
    if !comp.is_null() { xmlRegFreeRegexp(comp); };
}
unsafe extern "C" fn runFileTest(mut ctxt: xmlExpCtxtPtr,
                                 mut filename: *const std::os::raw::c_char) {
    let mut expr: xmlExpNodePtr = 0 as xmlExpNodePtr;
    let mut sub: xmlExpNodePtr = 0 as *mut xmlExpNode;
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut expression: [std::os::raw::c_char; 5000] = [0; 5000];
    let mut len: std::os::raw::c_int = 0;
    input = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if input.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Cannot open %s for reading\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   filename);
        return
    }
    while !fgets(expression.as_mut_ptr(), 4500 as std::os::raw::c_int,
                 input).is_null() {
        len = strlen(expression.as_mut_ptr()) as std::os::raw::c_int;
        len -= 1;
        while len >= 0 as std::os::raw::c_int &&
                  (expression[len as usize] as std::os::raw::c_int == '\n' as i32 ||
                       expression[len as usize] as std::os::raw::c_int == '\t' as i32
                       ||
                       expression[len as usize] as std::os::raw::c_int == '\r' as i32
                       ||
                       expression[len as usize] as std::os::raw::c_int == ' ' as i32)
              {
            len -= 1
        }
        expression[(len + 1 as std::os::raw::c_int) as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        if !(len >= 0 as std::os::raw::c_int) { continue ; }
        if expression[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '#' as i32
           {
            continue ;
        }
        if expression[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '=' as i32
               &&
               expression[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   '>' as i32 {
            let mut str: *mut std::os::raw::c_char =
                &mut *expression.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                                         isize) as
                    *mut std::os::raw::c_char;
            if !expr.is_null() {
                xmlExpFree(ctxt, expr);
                if xmlExpCtxtNbNodes(ctxt) != 0 as std::os::raw::c_int {
                    printf(b" Parse/free of Expression leaked %d\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           xmlExpCtxtNbNodes(ctxt));
                }
                expr = 0 as xmlExpNodePtr
            }
            printf(b"Expression: %s\n\x00" as *const u8 as
                       *const std::os::raw::c_char, str);
            expr = xmlExpParse(ctxt, str);
            if !expr.is_null() { continue ; }
            printf(b"   parsing Failed\n\x00" as *const u8 as
                       *const std::os::raw::c_char);
            break ;
        } else {
            if expr.is_null() { continue ; }
            let mut expect: std::os::raw::c_int = -(1 as std::os::raw::c_int);
            let mut nodes1: std::os::raw::c_int = 0;
            let mut nodes2: std::os::raw::c_int = 0;
            if expression[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   '0' as i32 {
                expect = 0 as std::os::raw::c_int
            }
            if expression[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   '1' as i32 {
                expect = 1 as std::os::raw::c_int
            }
            printf(b"Subexp: %s\x00" as *const u8 as *const std::os::raw::c_char,
                   expression.as_mut_ptr().offset(2 as std::os::raw::c_int as isize));
            nodes1 = xmlExpCtxtNbNodes(ctxt);
            sub =
                xmlExpParse(ctxt,
                            expression.as_mut_ptr().offset(2 as std::os::raw::c_int as
                                                               isize));
            if sub.is_null() {
                printf(b"   parsing Failed\n\x00" as *const u8 as
                           *const std::os::raw::c_char);
                break ;
            } else {
                let mut ret: std::os::raw::c_int = 0;
                nodes2 = xmlExpCtxtNbNodes(ctxt);
                ret = xmlExpSubsume(ctxt, expr, sub);
                if expect == 1 as std::os::raw::c_int && ret == 1 as std::os::raw::c_int {
                    printf(b" => accept, Ok\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                } else if expect == 0 as std::os::raw::c_int &&
                              ret == 0 as std::os::raw::c_int {
                    printf(b" => reject, Ok\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                } else if expect == 1 as std::os::raw::c_int &&
                              ret == 0 as std::os::raw::c_int {
                    printf(b" => reject, Failed\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                } else if expect == 0 as std::os::raw::c_int &&
                              ret == 1 as std::os::raw::c_int {
                    printf(b" => accept, Failed\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                } else {
                    printf(b" => fail internally\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                }
                if xmlExpCtxtNbNodes(ctxt) > nodes2 {
                    printf(b" Subsume leaked %d\n\x00" as *const u8 as
                               *const std::os::raw::c_char,
                           xmlExpCtxtNbNodes(ctxt) - nodes2);
                    nodes1 += xmlExpCtxtNbNodes(ctxt) - nodes2
                }
                xmlExpFree(ctxt, sub);
                if xmlExpCtxtNbNodes(ctxt) > nodes1 {
                    printf(b" Parse/free leaked %d\n\x00" as *const u8 as
                               *const std::os::raw::c_char,
                           xmlExpCtxtNbNodes(ctxt) - nodes1);
                }
            }
        }
    }
    if !expr.is_null() {
        xmlExpFree(ctxt, expr);
        if xmlExpCtxtNbNodes(ctxt) != 0 as std::os::raw::c_int {
            printf(b" Parse/free of Expression leaked %d\n\x00" as *const u8
                       as *const std::os::raw::c_char, xmlExpCtxtNbNodes(ctxt));
        }
    }
    fclose(input);
}
unsafe extern "C" fn testReduce(mut ctxt: xmlExpCtxtPtr,
                                mut expr: xmlExpNodePtr,
                                mut tst: *const std::os::raw::c_char) {
    let mut xmlExpBuf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut sub: xmlExpNodePtr = 0 as *mut xmlExpNode;
    let mut deriv: xmlExpNodePtr = 0 as *mut xmlExpNode;
    xmlExpBuf = xmlBufferCreate();
    sub = xmlExpParse(ctxt, tst);
    if sub.is_null() {
        printf(b"Subset %s failed to parse\n\x00" as *const u8 as
                   *const std::os::raw::c_char, tst);
        return
    }
    xmlExpDump(xmlExpBuf, sub);
    printf(b"Subset parsed as: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
           xmlBufferContent(xmlExpBuf as *const xmlBuffer) as
               *const std::os::raw::c_char);
    deriv = xmlExpExpDerive(ctxt, expr, sub);
    if deriv.is_null() {
        printf(b"Derivation led to an internal error, report this !\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    } else {
        xmlBufferEmpty(xmlExpBuf);
        xmlExpDump(xmlExpBuf, deriv);
        if xmlExpIsNillable(deriv) != 0 {
            printf(b"Resulting nillable derivation: %s\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   xmlBufferContent(xmlExpBuf as *const xmlBuffer) as
                       *const std::os::raw::c_char);
        } else {
            printf(b"Resulting derivation: %s\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   xmlBufferContent(xmlExpBuf as *const xmlBuffer) as
                       *const std::os::raw::c_char);
        }
        xmlExpFree(ctxt, deriv);
    }
    xmlExpFree(ctxt, sub);
}
unsafe extern "C" fn exprDebug(mut ctxt: xmlExpCtxtPtr,
                               mut expr: xmlExpNodePtr) {
    let mut xmlExpBuf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut deriv: xmlExpNodePtr = 0 as *mut xmlExpNode;
    let mut list: [*const std::os::raw::c_char; 40] = [0 as *const std::os::raw::c_char; 40];
    let mut ret: std::os::raw::c_int = 0;
    xmlExpBuf = xmlBufferCreate();
    if expr.is_null() {
        printf(b"Failed to parse\n\x00" as *const u8 as *const std::os::raw::c_char);
        return
    }
    xmlExpDump(xmlExpBuf, expr);
    printf(b"Parsed as: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
           xmlBufferContent(xmlExpBuf as *const xmlBuffer) as
               *const std::os::raw::c_char);
    printf(b"Max token input = %d\n\x00" as *const u8 as *const std::os::raw::c_char,
           xmlExpMaxToken(expr));
    if xmlExpIsNillable(expr) == 1 as std::os::raw::c_int {
        printf(b"Is nillable\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    ret =
        xmlExpGetLanguage(ctxt, expr,
                          &mut *list.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                             isize) as
                              *mut *const std::os::raw::c_char as *mut *const xmlChar,
                          40 as std::os::raw::c_int);
    if ret < 0 as std::os::raw::c_int {
        printf(b"Failed to get list: %d\n\x00" as *const u8 as
                   *const std::os::raw::c_char, ret);
    } else {
        let mut i: std::os::raw::c_int = 0;
        printf(b"Language has %d strings, testing string derivations\n\x00" as
                   *const u8 as *const std::os::raw::c_char, ret);
        i = 0 as std::os::raw::c_int;
        while i < ret {
            deriv =
                xmlExpStringDerive(ctxt, expr,
                                   list[i as usize] as *mut xmlChar,
                                   -(1 as std::os::raw::c_int));
            if deriv.is_null() {
                printf(b"  %s -> derivation failed\n\x00" as *const u8 as
                           *const std::os::raw::c_char, list[i as usize]);
            } else {
                xmlBufferEmpty(xmlExpBuf);
                xmlExpDump(xmlExpBuf, deriv);
                printf(b"  %s -> %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, list[i as usize],
                       xmlBufferContent(xmlExpBuf as *const xmlBuffer) as
                           *const std::os::raw::c_char);
            }
            xmlExpFree(ctxt, deriv);
            i += 1
        }
    }
    xmlBufferFree(xmlExpBuf);
}
unsafe extern "C" fn usage(mut name: *const std::os::raw::c_char) {
    fprintf(stderr,
            b"Usage: %s [flags]\n\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    fprintf(stderr,
            b"Testing tool for libxml2 string and pattern regexps\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"   --debug: switch on debugging\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(stderr,
            b"   --repeat: loop on the operation\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(stderr,
            b"   --expr: test xmlExp and not xmlRegexp\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(stderr,
            b"   --input filename: use the given filename for regexp\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"   --input filename: use the given filename for exp\n\x00" as
                *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut comp: xmlRegexpPtr = 0 as xmlRegexpPtr;
    let mut expr: xmlExpNodePtr = 0 as xmlExpNodePtr;
    let mut use_exp: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ctxt: xmlExpCtxtPtr = 0 as xmlExpCtxtPtr;
    let mut pattern: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut filename: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    xmlInitMemory();
    if argc <= 1 as std::os::raw::c_int {
        usage(*argv.offset(0 as std::os::raw::c_int as isize));
        return 1 as std::os::raw::c_int
    }
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            break ;
        }
        if !(*(*argv.offset(i as isize)).offset(0 as std::os::raw::c_int as isize) as
                 std::os::raw::c_int != '-' as i32) {
            if strcmp(*argv.offset(i as isize),
                      b"--\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
                break ;
            }
            if strcmp(*argv.offset(i as isize),
                      b"-debug\x00" as *const u8 as *const std::os::raw::c_char) == 0
                   ||
                   strcmp(*argv.offset(i as isize),
                          b"--debug\x00" as *const u8 as *const std::os::raw::c_char)
                       == 0 {
                debug += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-repeat\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--repeat\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                repeat += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-expr\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--expr\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                use_exp += 1
            } else if strcmp(*argv.offset(i as isize),
                             b"-i\x00" as *const u8 as *const std::os::raw::c_char) ==
                          0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"-f\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 ||
                          strcmp(*argv.offset(i as isize),
                                 b"--input\x00" as *const u8 as
                                     *const std::os::raw::c_char) == 0 {
                i += 1;
                filename = *argv.offset(i as isize)
            } else {
                fprintf(stderr,
                        b"Unknown option %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, *argv.offset(i as isize));
                usage(*argv.offset(0 as std::os::raw::c_int as isize));
            }
        }
        i += 1
    }
    if use_exp != 0 {
        ctxt = xmlExpNewCtxt(0 as std::os::raw::c_int, 0 as xmlDictPtr)
    }
    if !filename.is_null() {
        if use_exp != 0 {
            runFileTest(ctxt, filename);
        } else { testRegexpFile(filename); }
    } else {
        let mut data: std::os::raw::c_int = 0 as std::os::raw::c_int;
        if use_exp != 0 {
            i = 1 as std::os::raw::c_int;
            while i < argc {
                if strcmp(*argv.offset(i as isize),
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    data = 1 as std::os::raw::c_int
                } else if *(*argv.offset(i as
                                             isize)).offset(0 as std::os::raw::c_int
                                                                as isize) as
                              std::os::raw::c_int != '-' as i32 ||
                              strcmp(*argv.offset(i as isize),
                                     b"-\x00" as *const u8 as
                                         *const std::os::raw::c_char) ==
                                  0 as std::os::raw::c_int || data == 1 as std::os::raw::c_int
                 {
                    if pattern.is_null() {
                        pattern = *argv.offset(i as isize);
                        printf(b"Testing expr %s:\n\x00" as *const u8 as
                                   *const std::os::raw::c_char, pattern);
                        expr = xmlExpParse(ctxt, pattern);
                        if expr.is_null() {
                            printf(b"   failed to compile\n\x00" as *const u8
                                       as *const std::os::raw::c_char);
                            break ;
                        } else if debug != 0 { exprDebug(ctxt, expr); }
                    } else {
                        testReduce(ctxt, expr, *argv.offset(i as isize));
                    }
                }
                i += 1
            }
            if !expr.is_null() {
                xmlExpFree(ctxt, expr);
                expr = 0 as xmlExpNodePtr
            }
        } else {
            i = 1 as std::os::raw::c_int;
            while i < argc {
                if strcmp(*argv.offset(i as isize),
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    data = 1 as std::os::raw::c_int
                } else if *(*argv.offset(i as
                                             isize)).offset(0 as std::os::raw::c_int
                                                                as isize) as
                              std::os::raw::c_int != '-' as i32 ||
                              strcmp(*argv.offset(i as isize),
                                     b"-\x00" as *const u8 as
                                         *const std::os::raw::c_char) ==
                                  0 as std::os::raw::c_int || data == 1 as std::os::raw::c_int
                 {
                    if pattern.is_null() {
                        pattern = *argv.offset(i as isize);
                        printf(b"Testing %s:\n\x00" as *const u8 as
                                   *const std::os::raw::c_char, pattern);
                        comp = xmlRegexpCompile(pattern as *const xmlChar);
                        if comp.is_null() {
                            printf(b"   failed to compile\n\x00" as *const u8
                                       as *const std::os::raw::c_char);
                            break ;
                        } else if debug != 0 { xmlRegexpPrint(stdout, comp); }
                    } else { testRegexp(comp, *argv.offset(i as isize)); }
                }
                i += 1
            }
            if !comp.is_null() { xmlRegFreeRegexp(comp); }
        }
    }
    if !ctxt.is_null() {
        printf(b"Ops: %d nodes, %d cons\n\x00" as *const u8 as
                   *const std::os::raw::c_char, xmlExpCtxtNbNodes(ctxt),
               xmlExpCtxtNbCons(ctxt));
        xmlExpFreeCtxt(ctxt);
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
/* LIBXML_REGEXP_ENABLED */
