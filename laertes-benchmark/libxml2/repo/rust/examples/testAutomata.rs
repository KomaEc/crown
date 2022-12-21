#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
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
    /* *
 * xmlRegExecCtxtPtr:
 *
 * A libxml progressive regular expression evaluation context
 */
    pub type _xmlRegExecCtxt;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(__s: *mut std::os::raw::c_char, __n: std::os::raw::c_int, __stream: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlRegFreeRegexp(regexp: xmlRegexpPtr);
    #[no_mangle]
    fn xmlRegexpPrint(output: *mut FILE, regexp: xmlRegexpPtr);
    /*
 * The progressive API
 */
    #[no_mangle]
    fn xmlRegNewExecCtxt(comp: xmlRegexpPtr, callback: xmlRegExecCallbacks,
                         data: *mut std::os::raw::c_void) -> xmlRegExecCtxtPtr;
    #[no_mangle]
    fn xmlRegFreeExecCtxt(exec: xmlRegExecCtxtPtr);
    #[no_mangle]
    fn xmlRegExecPushString(exec: xmlRegExecCtxtPtr, value: *const xmlChar,
                            data: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    /*
 * Initialization of the memory layer.
 */
    #[no_mangle]
    fn xmlInitMemory() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    /*
 * Building API
 */
    #[no_mangle]
    fn xmlNewAutomata() -> xmlAutomataPtr;
    #[no_mangle]
    fn xmlFreeAutomata(am: xmlAutomataPtr);
    #[no_mangle]
    fn xmlAutomataGetInitState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataSetFinalState(am: xmlAutomataPtr,
                                state: xmlAutomataStatePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlAutomataNewState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewTransition(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                                to: xmlAutomataStatePtr,
                                token: *const xmlChar,
                                data: *mut std::os::raw::c_void)
     -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewCountTrans(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                                to: xmlAutomataStatePtr,
                                token: *const xmlChar, min: std::os::raw::c_int,
                                max: std::os::raw::c_int, data: *mut std::os::raw::c_void)
     -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewEpsilon(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                             to: xmlAutomataStatePtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataCompile(am: xmlAutomataPtr) -> xmlRegexpPtr;
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
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
/*
 * Summary: API to build regexp automata
 * Description: the API to build regexp automata
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlAutomataPtr:
 *
 * A libxml automata description, It can be compiled into a regexp
 */
/* *
 * xmlAutomataStatePtr:
 *
 * A state int the automata description,
 */
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
/* *
 * xmlRegExecCallbacks:
 * @exec: the regular expression context
 * @token: the current token string
 * @transdata: transition data
 * @inputdata: input data
 *
 * Callback function when doing a transition in the automata
 */
pub type xmlRegExecCallbacks
    =
    Option<unsafe extern "C" fn(_: xmlRegExecCtxtPtr, _: *const xmlChar,
                                _: *mut std::os::raw::c_void, _: *mut std::os::raw::c_void)
               -> ()>;
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
unsafe extern "C" fn scanNumber(mut ptr: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cur: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    cur = *ptr;
    while *cur as std::os::raw::c_int >= '0' as i32 &&
              *cur as std::os::raw::c_int <= '9' as i32 {
        ret = ret * 10 as std::os::raw::c_int + (*cur as std::os::raw::c_int - '0' as i32);
        cur = cur.offset(1)
    }
    *ptr = cur;
    return ret;
}
unsafe extern "C" fn testRegexpFile(mut filename: *const std::os::raw::c_char) {
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut expr: [std::os::raw::c_char; 5000] = [0; 5000];
    let mut len: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut am: xmlAutomataPtr = 0 as *mut xmlAutomata;
    let mut states: [xmlAutomataStatePtr; 1000] =
        [0 as *mut xmlAutomataState; 1000];
    let mut regexp: xmlRegexpPtr = 0 as xmlRegexpPtr;
    let mut exec: xmlRegExecCtxtPtr = 0 as xmlRegExecCtxtPtr;
    i = 0 as std::os::raw::c_int;
    while i < 1000 as std::os::raw::c_int {
        states[i as usize] = 0 as xmlAutomataStatePtr;
        i += 1
    }
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
    am = xmlNewAutomata();
    if am.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Cannot create automata\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        fclose(input);
        return
    }
    states[0 as std::os::raw::c_int as usize] = xmlAutomataGetInitState(am);
    if states[0 as std::os::raw::c_int as usize].is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Cannot get start state\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlFreeAutomata(am);
        fclose(input);
        return
    }
    ret = 0 as std::os::raw::c_int;
    while !fgets(expr.as_mut_ptr(), 4500 as std::os::raw::c_int, input).is_null() {
        if expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '#' as i32 {
            continue ;
        }
        len = strlen(expr.as_mut_ptr()) as std::os::raw::c_int;
        len -= 1;
        while len >= 0 as std::os::raw::c_int &&
                  (expr[len as usize] as std::os::raw::c_int == '\n' as i32 ||
                       expr[len as usize] as std::os::raw::c_int == '\t' as i32 ||
                       expr[len as usize] as std::os::raw::c_int == '\r' as i32 ||
                       expr[len as usize] as std::os::raw::c_int == ' ' as i32) {
            len -= 1
        }
        expr[(len + 1 as std::os::raw::c_int) as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        if !(len >= 0 as std::os::raw::c_int) { continue ; }
        if !am.is_null() &&
               expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == 't' as i32 &&
               expr[1 as std::os::raw::c_int as usize] as std::os::raw::c_int == ' ' as i32 {
            let mut ptr: *mut std::os::raw::c_char =
                &mut *expr.as_mut_ptr().offset(2 as std::os::raw::c_int as isize) as
                    *mut std::os::raw::c_char;
            let mut from: std::os::raw::c_int = 0;
            let mut to: std::os::raw::c_int = 0;
            from = scanNumber(&mut ptr);
            if *ptr as std::os::raw::c_int != ' ' as i32 {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Bad line %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           expr.as_mut_ptr());
                break ;
            } else {
                if states[from as usize].is_null() {
                    states[from as usize] = xmlAutomataNewState(am)
                }
                ptr = ptr.offset(1);
                to = scanNumber(&mut ptr);
                if *ptr as std::os::raw::c_int != ' ' as i32 {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Bad line %s\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               expr.as_mut_ptr());
                    break ;
                } else {
                    if states[to as usize].is_null() {
                        states[to as usize] = xmlAutomataNewState(am)
                    }
                    ptr = ptr.offset(1);
                    xmlAutomataNewTransition(am, states[from as usize],
                                             states[to as usize],
                                             ptr as *mut xmlChar,
                                             0 as *mut std::os::raw::c_void);
                }
            }
        } else if !am.is_null() &&
                      expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          'e' as i32 &&
                      expr[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          ' ' as i32 {
            let mut ptr_0: *mut std::os::raw::c_char =
                &mut *expr.as_mut_ptr().offset(2 as std::os::raw::c_int as isize) as
                    *mut std::os::raw::c_char;
            let mut from_0: std::os::raw::c_int = 0;
            let mut to_0: std::os::raw::c_int = 0;
            from_0 = scanNumber(&mut ptr_0);
            if *ptr_0 as std::os::raw::c_int != ' ' as i32 {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Bad line %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           expr.as_mut_ptr());
                break ;
            } else {
                if states[from_0 as usize].is_null() {
                    states[from_0 as usize] = xmlAutomataNewState(am)
                }
                ptr_0 = ptr_0.offset(1);
                to_0 = scanNumber(&mut ptr_0);
                if states[to_0 as usize].is_null() {
                    states[to_0 as usize] = xmlAutomataNewState(am)
                }
                xmlAutomataNewEpsilon(am, states[from_0 as usize],
                                      states[to_0 as usize]);
            }
        } else if !am.is_null() &&
                      expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          'f' as i32 &&
                      expr[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          ' ' as i32 {
            let mut ptr_1: *mut std::os::raw::c_char =
                &mut *expr.as_mut_ptr().offset(2 as std::os::raw::c_int as isize) as
                    *mut std::os::raw::c_char;
            let mut state: std::os::raw::c_int = 0;
            state = scanNumber(&mut ptr_1);
            if states[state as usize].is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Bad state %d : %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           state,
                                                                           expr.as_mut_ptr());
                break ;
            } else { xmlAutomataSetFinalState(am, states[state as usize]); }
        } else if !am.is_null() &&
                      expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          'c' as i32 &&
                      expr[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          ' ' as i32 {
            let mut ptr_2: *mut std::os::raw::c_char =
                &mut *expr.as_mut_ptr().offset(2 as std::os::raw::c_int as isize) as
                    *mut std::os::raw::c_char;
            let mut from_1: std::os::raw::c_int = 0;
            let mut to_1: std::os::raw::c_int = 0;
            let mut min: std::os::raw::c_int = 0;
            let mut max: std::os::raw::c_int = 0;
            from_1 = scanNumber(&mut ptr_2);
            if *ptr_2 as std::os::raw::c_int != ' ' as i32 {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Bad line %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           expr.as_mut_ptr());
                break ;
            } else {
                if states[from_1 as usize].is_null() {
                    states[from_1 as usize] = xmlAutomataNewState(am)
                }
                ptr_2 = ptr_2.offset(1);
                to_1 = scanNumber(&mut ptr_2);
                if *ptr_2 as std::os::raw::c_int != ' ' as i32 {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Bad line %s\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               expr.as_mut_ptr());
                    break ;
                } else {
                    if states[to_1 as usize].is_null() {
                        states[to_1 as usize] = xmlAutomataNewState(am)
                    }
                    ptr_2 = ptr_2.offset(1);
                    min = scanNumber(&mut ptr_2);
                    if *ptr_2 as std::os::raw::c_int != ' ' as i32 {
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Bad line %s\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   expr.as_mut_ptr());
                        break ;
                    } else {
                        ptr_2 = ptr_2.offset(1);
                        max = scanNumber(&mut ptr_2);
                        if *ptr_2 as std::os::raw::c_int != ' ' as i32 {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"Bad line %s\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       expr.as_mut_ptr());
                            break ;
                        } else {
                            ptr_2 = ptr_2.offset(1);
                            xmlAutomataNewCountTrans(am,
                                                     states[from_1 as usize],
                                                     states[to_1 as usize],
                                                     ptr_2 as *mut xmlChar,
                                                     min, max,
                                                     0 as *mut std::os::raw::c_void);
                        }
                    }
                }
            }
        } else if !am.is_null() &&
                      expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          '-' as i32 &&
                      expr[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          '-' as i32 {
            /* end of the automata */
            regexp = xmlAutomataCompile(am);
            xmlFreeAutomata(am);
            am = 0 as xmlAutomataPtr;
            if !regexp.is_null() { continue ; }
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Failed to compile the automata\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            break ;
        } else if expr[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '=' as i32
                      &&
                      expr[1 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                          '>' as i32 {
            if regexp.is_null() {
                printf(b"=> failed not compiled\n\x00" as *const u8 as
                           *const std::os::raw::c_char);
            } else {
                if exec.is_null() {
                    exec =
                        xmlRegNewExecCtxt(regexp, None,
                                          0 as *mut std::os::raw::c_void)
                }
                if ret == 0 as std::os::raw::c_int {
                    ret =
                        xmlRegExecPushString(exec, 0 as *const xmlChar,
                                             0 as *mut std::os::raw::c_void)
                }
                if ret == 1 as std::os::raw::c_int {
                    printf(b"=> Passed\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                } else if ret == 0 as std::os::raw::c_int ||
                              ret == -(1 as std::os::raw::c_int) {
                    printf(b"=> Failed\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                } else if ret < 0 as std::os::raw::c_int {
                    printf(b"=> Error\n\x00" as *const u8 as
                               *const std::os::raw::c_char);
                }
                xmlRegFreeExecCtxt(exec);
                exec = 0 as xmlRegExecCtxtPtr
            }
            ret = 0 as std::os::raw::c_int
        } else if !regexp.is_null() {
            if exec.is_null() {
                exec = xmlRegNewExecCtxt(regexp, None, 0 as *mut std::os::raw::c_void)
            }
            ret =
                xmlRegExecPushString(exec, expr.as_mut_ptr() as *mut xmlChar,
                                     0 as *mut std::os::raw::c_void)
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unexpected line %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       expr.as_mut_ptr());
        }
    }
    fclose(input);
    if !regexp.is_null() { xmlRegFreeRegexp(regexp); }
    if !exec.is_null() { xmlRegFreeExecCtxt(exec); }
    if !am.is_null() { xmlFreeAutomata(am); };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    xmlInitMemory();
    if argc == 1 as std::os::raw::c_int {
        let mut ret: std::os::raw::c_int = 0;
        let mut am: xmlAutomataPtr = 0 as *mut xmlAutomata;
        let mut start: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
        let mut cur: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
        let mut regexp: xmlRegexpPtr = 0 as *mut xmlRegexp;
        let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
        am = xmlNewAutomata();
        start = xmlAutomataGetInitState(am);
        /* generate a[ba]*a */
        cur =
            xmlAutomataNewTransition(am, start, 0 as xmlAutomataStatePtr,
                                     b"a\x00" as *const u8 as
                                         *const std::os::raw::c_char as *mut xmlChar,
                                     0 as *mut std::os::raw::c_void);
        xmlAutomataNewTransition(am, cur, cur,
                                 b"b\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        xmlAutomataNewTransition(am, cur, cur,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        cur =
            xmlAutomataNewCountTrans(am, cur, 0 as xmlAutomataStatePtr,
                                     b"a\x00" as *const u8 as
                                         *const std::os::raw::c_char as *mut xmlChar,
                                     2 as std::os::raw::c_int, 3 as std::os::raw::c_int,
                                     0 as *mut std::os::raw::c_void);
        xmlAutomataSetFinalState(am, cur);
        /* compile it in a regexp and free the automata */
        regexp = xmlAutomataCompile(am);
        xmlFreeAutomata(am);
        /* test the regexp */
        xmlRegexpPrint(stdout, regexp);
        exec = xmlRegNewExecCtxt(regexp, None, 0 as *mut std::os::raw::c_void);
        ret =
            xmlRegExecPushString(exec,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        ret =
            xmlRegExecPushString(exec,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        ret =
            xmlRegExecPushString(exec,
                                 b"b\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        ret =
            xmlRegExecPushString(exec,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        ret =
            xmlRegExecPushString(exec,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        ret =
            xmlRegExecPushString(exec,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        ret =
            xmlRegExecPushString(exec,
                                 b"a\x00" as *const u8 as *const std::os::raw::c_char
                                     as *mut xmlChar, 0 as *mut std::os::raw::c_void);
        if ret == 1 as std::os::raw::c_int {
            printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
        } else if ret < 0 as std::os::raw::c_int {
            printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        if ret == 0 as std::os::raw::c_int {
            ret =
                xmlRegExecPushString(exec, 0 as *const xmlChar,
                                     0 as *mut std::os::raw::c_void);
            if ret == 1 as std::os::raw::c_int {
                printf(b"final\n\x00" as *const u8 as *const std::os::raw::c_char);
            } else if ret < 0 as std::os::raw::c_int {
                printf(b"error\n\x00" as *const u8 as *const std::os::raw::c_char);
            }
        }
        xmlRegFreeExecCtxt(exec);
        /* free the regexp */
        xmlRegFreeRegexp(regexp);
    } else {
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < argc { testRegexpFile(*argv.offset(i as isize)); i += 1 }
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
/* LIBXML_AUTOMATA_ENABLED */
