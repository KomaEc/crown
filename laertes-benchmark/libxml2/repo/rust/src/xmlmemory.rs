
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlMutex;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    static mut xmlMemStrdup: xmlStrdupFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    /*
 * xmlRMutex are reentrant mutual exception locks.
 */
    #[no_mangle]
    fn xmlNewMutex() -> xmlMutexPtr;
    #[no_mangle]
    fn xmlFreeMutex(tok: xmlMutexPtr);
    #[no_mangle]
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    #[no_mangle]
    fn xmlMutexLock(tok: xmlMutexPtr);
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
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
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type xmlMallocFunc
    =
    Option<unsafe extern "C" fn(_: size_t) -> *mut std::os::raw::c_void>;
pub type xmlReallocFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
               -> *mut std::os::raw::c_void>;
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
/* *
 * Summary: interfaces for thread handling
 * Description: set of generic threading related routines
 *              should work with pthreads, Windows native or TLS threads
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * xmlMutex are a simple mutual exception locks.
 */
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlMutex = _xmlMutex;
pub type MEMHDR = memnod;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memnod {
    pub mh_tag: std::os::raw::c_uint,
    pub mh_type: std::os::raw::c_uint,
    pub mh_number: std::os::raw::c_ulong,
    pub mh_size: size_t,
    pub mh_file: *const std::os::raw::c_char,
    pub mh_line: std::os::raw::c_uint,
}
/*
 * xmlmemory.c:  libxml memory allocator wrapper.
 *
 * daniel@veillard.com
 */
/* #define DEBUG_MEMORY */
/* *
 * MEM_LIST:
 *
 * keep track of all allocated blocks for error reporting
 * Always build the memory list !
 */
static mut xmlMemInitialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut debugMemSize: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
static mut debugMemBlocks: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
static mut debugMaxMemSize: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
static mut xmlMemMutex: xmlMutexPtr = 0 as *const xmlMutex as xmlMutexPtr;
static mut block: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
static mut xmlMemStopAtBlock: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
static mut xmlMemTraceBlockAt: *mut std::os::raw::c_void =
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void;
/* *
 * xmlMallocBreakpoint:
 *
 * Breakpoint to use in conjunction with xmlMemStopAtBlock. When the block
 * number reaches the specified value this function is called. One need to add a breakpoint
 * to it to get the context in which the given block is allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMallocBreakpoint() {
    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                               b"xmlMallocBreakpoint reached on block %d\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               xmlMemStopAtBlock);
}
/* *
 * xmlMallocLoc:
 * @size:  an int specifying the size in byte to allocate.
 * @file:  the file name or NULL
 * @line:  the line number
 *
 * a malloc() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the allocated area or NULL in case of lack of memory.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMallocLoc(mut size: size_t,
                                      mut file: *const std::os::raw::c_char,
                                      mut line: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut p: *mut MEMHDR = 0 as *mut MEMHDR;
    let mut ret: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if xmlMemInitialized == 0 { xmlInitMemory(); }
    if size >
           (-(1 as std::os::raw::c_int) as
                size_t).wrapping_sub((::std::mem::size_of::<MEMHDR>() as
                                          std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                           as
                                                                           std::os::raw::c_ulong).wrapping_sub(1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                            as
                                                                                                                                                                            std::os::raw::c_ulong))
       {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlMallocLoc : Unsigned overflow\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlMemoryDump();
        return 0 as *mut std::os::raw::c_void
    }
    p =
        malloc((::std::mem::size_of::<MEMHDR>() as
                    std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                     as
                                                     std::os::raw::c_ulong).wrapping_sub(1
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                      as
                                                                                                                      std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                      as
                                                                                                                                                      std::os::raw::c_ulong).wrapping_add(size))
            as *mut MEMHDR;
    if p.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlMallocLoc : Out of free space\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlMemoryDump();
        return 0 as *mut std::os::raw::c_void
    }
    (*p).mh_tag = 0x5aa5 as std::os::raw::c_int as std::os::raw::c_uint;
    (*p).mh_size = size;
    (*p).mh_type = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*p).mh_file = file;
    (*p).mh_line = line as std::os::raw::c_uint;
    xmlMutexLock(xmlMemMutex);
    block = block.wrapping_add(1);
    (*p).mh_number = block as std::os::raw::c_ulong;
    debugMemSize = debugMemSize.wrapping_add(size);
    debugMemBlocks = debugMemBlocks.wrapping_add(1);
    if debugMemSize > debugMaxMemSize { debugMaxMemSize = debugMemSize }
    xmlMutexUnlock(xmlMemMutex);
    if xmlMemStopAtBlock as std::os::raw::c_ulong == (*p).mh_number {
        xmlMallocBreakpoint();
    }
    ret =
        (p as
             *mut std::os::raw::c_char).offset((::std::mem::size_of::<MEMHDR>() as
                                            std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                             as
                                                                             std::os::raw::c_ulong).wrapping_sub(1
                                                                                                             as
                                                                                                             std::os::raw::c_int
                                                                                                             as
                                                                                                             std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                              as
                                                                                                                                              std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                              as
                                                                                                                                                                              std::os::raw::c_ulong)
                                           as isize) as *mut std::os::raw::c_void;
    if xmlMemTraceBlockAt == ret {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"%p : Malloc(%lu) Ok\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   xmlMemTraceBlockAt,
                                                                   size);
        xmlMallocBreakpoint();
    }
    return ret;
}
/* *
 * xmlMallocAtomicLoc:
 * @size:  an unsigned int specifying the size in byte to allocate.
 * @file:  the file name or NULL
 * @line:  the line number
 *
 * a malloc() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the allocated area or NULL in case of lack of memory.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMallocAtomicLoc(mut size: size_t,
                                            mut file: *const std::os::raw::c_char,
                                            mut line: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut p: *mut MEMHDR = 0 as *mut MEMHDR;
    let mut ret: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if xmlMemInitialized == 0 { xmlInitMemory(); }
    if size >
           (-(1 as std::os::raw::c_int) as
                size_t).wrapping_sub((::std::mem::size_of::<MEMHDR>() as
                                          std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                           as
                                                                           std::os::raw::c_ulong).wrapping_sub(1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                            as
                                                                                                                                                                            std::os::raw::c_ulong))
       {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlMallocAtomicLoc : Unsigned overflow\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlMemoryDump();
        return 0 as *mut std::os::raw::c_void
    }
    p =
        malloc((::std::mem::size_of::<MEMHDR>() as
                    std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                     as
                                                     std::os::raw::c_ulong).wrapping_sub(1
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                      as
                                                                                                                      std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                      as
                                                                                                                                                      std::os::raw::c_ulong).wrapping_add(size))
            as *mut MEMHDR;
    if p.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlMallocAtomicLoc : Out of free space\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlMemoryDump();
        return 0 as *mut std::os::raw::c_void
    }
    (*p).mh_tag = 0x5aa5 as std::os::raw::c_int as std::os::raw::c_uint;
    (*p).mh_size = size;
    (*p).mh_type = 4 as std::os::raw::c_int as std::os::raw::c_uint;
    (*p).mh_file = file;
    (*p).mh_line = line as std::os::raw::c_uint;
    xmlMutexLock(xmlMemMutex);
    block = block.wrapping_add(1);
    (*p).mh_number = block as std::os::raw::c_ulong;
    debugMemSize = debugMemSize.wrapping_add(size);
    debugMemBlocks = debugMemBlocks.wrapping_add(1);
    if debugMemSize > debugMaxMemSize { debugMaxMemSize = debugMemSize }
    xmlMutexUnlock(xmlMemMutex);
    if xmlMemStopAtBlock as std::os::raw::c_ulong == (*p).mh_number {
        xmlMallocBreakpoint();
    }
    ret =
        (p as
             *mut std::os::raw::c_char).offset((::std::mem::size_of::<MEMHDR>() as
                                            std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                             as
                                                                             std::os::raw::c_ulong).wrapping_sub(1
                                                                                                             as
                                                                                                             std::os::raw::c_int
                                                                                                             as
                                                                                                             std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                              as
                                                                                                                                              std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                              as
                                                                                                                                                                              std::os::raw::c_ulong)
                                           as isize) as *mut std::os::raw::c_void;
    if xmlMemTraceBlockAt == ret {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"%p : Malloc(%lu) Ok\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   xmlMemTraceBlockAt,
                                                                   size);
        xmlMallocBreakpoint();
    }
    return ret;
}
/* *
 * xmlMemMalloc:
 * @size:  an int specifying the size in byte to allocate.
 *
 * a malloc() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the allocated area or NULL in case of lack of memory.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemMalloc(mut size: size_t) -> *mut std::os::raw::c_void {
    return xmlMallocLoc(size, b"none\x00" as *const u8 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int);
}
/* *
 * xmlReallocLoc:
 * @ptr:  the initial memory block pointer
 * @size:  an int specifying the size in byte to allocate.
 * @file:  the file name or NULL
 * @line:  the line number
 *
 * a realloc() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the allocated area or NULL in case of lack of memory.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReallocLoc(mut ptr: *mut std::os::raw::c_void,
                                       mut size: size_t,
                                       mut file: *const std::os::raw::c_char,
                                       mut line: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut p: *mut MEMHDR = 0 as *mut MEMHDR;
    let mut tmp: *mut MEMHDR = 0 as *mut MEMHDR;
    let mut number: std::os::raw::c_ulong = 0;
    if ptr.is_null() { return xmlMallocLoc(size, file, line) }
    if xmlMemInitialized == 0 { xmlInitMemory(); }
    p =
        (ptr as
             *mut std::os::raw::c_char).offset(-((::std::mem::size_of::<MEMHDR>() as
                                              std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                               as
                                                                               std::os::raw::c_ulong).wrapping_sub(1
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                as
                                                                                                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                                as
                                                                                                                                                                                std::os::raw::c_ulong)
                                             as isize)) as *mut std::os::raw::c_void
            as *mut MEMHDR;
    number = (*p).mh_number;
    if xmlMemStopAtBlock as std::os::raw::c_ulong == number { xmlMallocBreakpoint(); }
    if (*p).mh_tag != 0x5aa5 as std::os::raw::c_int as std::os::raw::c_uint {
        debugmem_tag_error(p as *mut std::os::raw::c_void);
    } else {
        (*p).mh_tag = !(0x5aa5 as std::os::raw::c_int) as std::os::raw::c_uint;
        xmlMutexLock(xmlMemMutex);
        debugMemSize = debugMemSize.wrapping_sub((*p).mh_size);
        debugMemBlocks = debugMemBlocks.wrapping_sub(1);
        xmlMutexUnlock(xmlMemMutex);
        if size >
               (-(1 as std::os::raw::c_int) as
                    size_t).wrapping_sub((::std::mem::size_of::<MEMHDR>() as
                                              std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                               as
                                                                               std::os::raw::c_ulong).wrapping_sub(1
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                as
                                                                                                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                                as
                                                                                                                                                                                std::os::raw::c_ulong))
           {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"xmlReallocLoc : Unsigned overflow\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            xmlMemoryDump();
            return 0 as *mut std::os::raw::c_void
        }
        tmp =
            realloc(p as *mut std::os::raw::c_void,
                    (::std::mem::size_of::<MEMHDR>() as
                         std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                          as
                                                          std::os::raw::c_ulong).wrapping_sub(1
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                           as
                                                                                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                           as
                                                                                                                                                           std::os::raw::c_ulong).wrapping_add(size))
                as *mut MEMHDR;
        if tmp.is_null() {
            free(p as *mut std::os::raw::c_void);
        } else {
            p = tmp;
            if xmlMemTraceBlockAt == ptr {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"%p : Realloced(%lu -> %lu) Ok\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           xmlMemTraceBlockAt,
                                                                           (*p).mh_size,
                                                                           size);
                xmlMallocBreakpoint();
            }
            (*p).mh_tag = 0x5aa5 as std::os::raw::c_int as std::os::raw::c_uint;
            (*p).mh_number = number;
            (*p).mh_type = 2 as std::os::raw::c_int as std::os::raw::c_uint;
            (*p).mh_size = size;
            (*p).mh_file = file;
            (*p).mh_line = line as std::os::raw::c_uint;
            xmlMutexLock(xmlMemMutex);
            debugMemSize = debugMemSize.wrapping_add(size);
            debugMemBlocks = debugMemBlocks.wrapping_add(1);
            if debugMemSize > debugMaxMemSize {
                debugMaxMemSize = debugMemSize
            }
            xmlMutexUnlock(xmlMemMutex);
            return (p as
                        *mut std::os::raw::c_char).offset((::std::mem::size_of::<MEMHDR>()
                                                       as
                                                       std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                                        as
                                                                                        std::os::raw::c_ulong).wrapping_sub(1
                                                                                                                        as
                                                                                                                        std::os::raw::c_int
                                                                                                                        as
                                                                                                                        std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                         as
                                                                                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                                         as
                                                                                                                                                                                         std::os::raw::c_ulong)
                                                      as isize) as
                       *mut std::os::raw::c_void
        }
    }
    return 0 as *mut std::os::raw::c_void;
}
/* *
 * xmlMemRealloc:
 * @ptr:  the initial memory block pointer
 * @size:  an int specifying the size in byte to allocate.
 *
 * a realloc() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the allocated area or NULL in case of lack of memory.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemRealloc(mut ptr: *mut std::os::raw::c_void,
                                       mut size: size_t)
 -> *mut std::os::raw::c_void {
    return xmlReallocLoc(ptr, size,
                         b"none\x00" as *const u8 as *const std::os::raw::c_char,
                         0 as std::os::raw::c_int);
}
/* *
 * xmlMemFree:
 * @ptr:  the memory block pointer
 *
 * a free() equivalent, with error checking.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemFree(mut ptr: *mut std::os::raw::c_void) {
    let mut p: *mut MEMHDR = 0 as *mut MEMHDR;
    let mut target: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if ptr.is_null() { return }
    if ptr == -(1 as std::os::raw::c_int) as *mut std::os::raw::c_void {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"trying to free pointer from freed area\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
    } else {
        if xmlMemTraceBlockAt == ptr {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%p : Freed()\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       xmlMemTraceBlockAt);
            xmlMallocBreakpoint();
        }
        target = ptr as *mut std::os::raw::c_char;
        p =
            (ptr as
                 *mut std::os::raw::c_char).offset(-((::std::mem::size_of::<MEMHDR>()
                                                  as
                                                  std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                                   as
                                                                                   std::os::raw::c_ulong).wrapping_sub(1
                                                                                                                   as
                                                                                                                   std::os::raw::c_int
                                                                                                                   as
                                                                                                                   std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                    as
                                                                                                                                                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                                    as
                                                                                                                                                                                    std::os::raw::c_ulong)
                                                 as isize)) as
                *mut std::os::raw::c_void as *mut MEMHDR;
        if (*p).mh_tag != 0x5aa5 as std::os::raw::c_int as std::os::raw::c_uint {
            debugmem_tag_error(p as *mut std::os::raw::c_void);
        } else {
            if xmlMemStopAtBlock as std::os::raw::c_ulong == (*p).mh_number {
                xmlMallocBreakpoint();
            }
            (*p).mh_tag = !(0x5aa5 as std::os::raw::c_int) as std::os::raw::c_uint;
            memset(target as *mut std::os::raw::c_void, -(1 as std::os::raw::c_int),
                   (*p).mh_size);
            xmlMutexLock(xmlMemMutex);
            debugMemSize = debugMemSize.wrapping_sub((*p).mh_size);
            debugMemBlocks = debugMemBlocks.wrapping_sub(1);
            xmlMutexUnlock(xmlMemMutex);
            free(p as *mut std::os::raw::c_void);
            return
        }
    }
    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                               b"xmlMemFree(%p) error\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               ptr);
    xmlMallocBreakpoint();
}
/* *
 * xmlMemStrdupLoc:
 * @str:  the initial string pointer
 * @file:  the file name or NULL
 * @line:  the line number
 *
 * a strdup() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the new string or NULL if allocation error occurred.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemStrdupLoc(mut str: *const std::os::raw::c_char,
                                         mut file: *const std::os::raw::c_char,
                                         mut line: std::os::raw::c_int)
 -> *mut std::os::raw::c_char {
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut size: size_t =
        strlen(str).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    let mut p: *mut MEMHDR = 0 as *mut MEMHDR;
    if xmlMemInitialized == 0 { xmlInitMemory(); }
    if size >
           (-(1 as std::os::raw::c_int) as
                size_t).wrapping_sub((::std::mem::size_of::<MEMHDR>() as
                                          std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                           as
                                                                           std::os::raw::c_ulong).wrapping_sub(1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                            as
                                                                                                                                            std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                            as
                                                                                                                                                                            std::os::raw::c_ulong))
       {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlMemStrdupLoc : Unsigned overflow\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlMemoryDump();
        return 0 as *mut std::os::raw::c_char
    }
    p =
        malloc((::std::mem::size_of::<MEMHDR>() as
                    std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                     as
                                                     std::os::raw::c_ulong).wrapping_sub(1
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                      as
                                                                                                                      std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                      as
                                                                                                                                                      std::os::raw::c_ulong).wrapping_add(size))
            as *mut MEMHDR;
    if p.is_null() {
        return 0 as *mut std::os::raw::c_char
    } else {
        (*p).mh_tag = 0x5aa5 as std::os::raw::c_int as std::os::raw::c_uint;
        (*p).mh_size = size;
        (*p).mh_type = 3 as std::os::raw::c_int as std::os::raw::c_uint;
        (*p).mh_file = file;
        (*p).mh_line = line as std::os::raw::c_uint;
        xmlMutexLock(xmlMemMutex);
        block = block.wrapping_add(1);
        (*p).mh_number = block as std::os::raw::c_ulong;
        debugMemSize = debugMemSize.wrapping_add(size);
        debugMemBlocks = debugMemBlocks.wrapping_add(1);
        if debugMemSize > debugMaxMemSize { debugMaxMemSize = debugMemSize }
        xmlMutexUnlock(xmlMemMutex);
        s =
            (p as
                 *mut std::os::raw::c_char).offset((::std::mem::size_of::<MEMHDR>() as
                                                std::os::raw::c_ulong).wrapping_add((::std::mem::size_of::<std::os::raw::c_double>()
                                                                                 as
                                                                                 std::os::raw::c_ulong).wrapping_sub(1
                                                                                                                 as
                                                                                                                 std::os::raw::c_int
                                                                                                                 as
                                                                                                                 std::os::raw::c_ulong)).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                  as
                                                                                                                                                  std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                                                                                                                                                  as
                                                                                                                                                                                  std::os::raw::c_ulong)
                                               as isize) as *mut std::os::raw::c_void
                as *mut std::os::raw::c_char;
        if xmlMemStopAtBlock as std::os::raw::c_ulong == (*p).mh_number {
            xmlMallocBreakpoint();
        }
        strcpy(s, str);
        if xmlMemTraceBlockAt == s as *mut std::os::raw::c_void {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%p : Strdup() Ok\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       xmlMemTraceBlockAt);
            xmlMallocBreakpoint();
        }
        return s
    };
}
/* *
 * xmlMemoryStrdup:
 * @str:  the initial string pointer
 *
 * a strdup() equivalent, with logging of the allocation info.
 *
 * Returns a pointer to the new string or NULL if allocation error occurred.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemoryStrdup(mut str: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    return xmlMemStrdupLoc(str,
                           b"none\x00" as *const u8 as *const std::os::raw::c_char,
                           0 as std::os::raw::c_int);
}
/* *
 * xmlMemUsed:
 *
 * Provides the amount of memory currently allocated
 *
 * Returns an int representing the amount of memory allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemUsed() -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = 0;
    xmlMutexLock(xmlMemMutex);
    res = debugMemSize as std::os::raw::c_int;
    xmlMutexUnlock(xmlMemMutex);
    return res;
}
/* *
 * xmlMemBlocks:
 *
 * Provides the number of memory areas currently allocated
 *
 * Returns an int representing the number of blocks
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemBlocks() -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = 0;
    xmlMutexLock(xmlMemMutex);
    res = debugMemBlocks as std::os::raw::c_int;
    xmlMutexUnlock(xmlMemMutex);
    return res;
}
/* *
 * xmlMemDisplayLast:
 * @fp:  a FILE descriptor used as the output file, if NULL, the result is
 *       written to the file .memorylist
 * @nbBytes: the amount of memory to dump
 *
 * the last nbBytes of memory allocated and not freed, useful for dumping
 * the memory left allocated between two places at runtime.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemDisplayLast(mut fp: *mut FILE,
                                           mut nbBytes: std::os::raw::c_long) {
    let mut old_fp: *mut FILE = fp;
    if nbBytes <= 0 as std::os::raw::c_int as std::os::raw::c_long { return }
    if fp.is_null() {
        fp =
            fopen(b".memorylist\x00" as *const u8 as *const std::os::raw::c_char,
                  b"w\x00" as *const u8 as *const std::os::raw::c_char);
        if fp.is_null() { return }
    }
    fprintf(fp,
            b"Memory list not compiled (MEM_LIST not defined !)\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    if old_fp.is_null() { fclose(fp); };
}
/* *
 * xmlMemDisplay:
 * @fp:  a FILE descriptor used as the output file, if NULL, the result is
 *       written to the file .memorylist
 *
 * show in-extenso the memory blocks allocated
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemDisplay(mut fp: *mut FILE) {
    let mut old_fp: *mut FILE = fp;
    if fp.is_null() {
        fp =
            fopen(b".memorylist\x00" as *const u8 as *const std::os::raw::c_char,
                  b"w\x00" as *const u8 as *const std::os::raw::c_char);
        if fp.is_null() { return }
    }
    fprintf(fp,
            b"Memory list not compiled (MEM_LIST not defined !)\n\x00" as
                *const u8 as *const std::os::raw::c_char);
    if old_fp.is_null() { fclose(fp); };
}
/*
 * debugmem_tag_error:
 *
 * internal error function.
 */
unsafe extern "C" fn debugmem_tag_error(mut p: *mut std::os::raw::c_void) {
    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                               b"Memory tag error occurs :%p \n\t bye\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               p);
}
/* *
 * xmlMemShow:
 * @fp:  a FILE descriptor used as the output file
 * @nr:  number of entries to dump
 *
 * show a show display of the memory allocated, and dump
 * the @nr last allocated areas which were not freed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemShow(mut fp: *mut FILE, mut nr: std::os::raw::c_int) {
    if !fp.is_null() {
        fprintf(fp,
                b"      MEMORY ALLOCATED : %lu, MAX was %lu\n\x00" as
                    *const u8 as *const std::os::raw::c_char, debugMemSize,
                debugMaxMemSize);
    };
    /* MEM_LIST */
}
/*
 * These are specific to the XML debug memory wrapper.
 */
/* *
 * xmlMemoryDump:
 *
 * Dump in-extenso the memory blocks allocated to the file .memorylist
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemoryDump() {
    /* MEM_LIST */
}
/* ***************************************************************
 *								*
 *		Initialization Routines				*
 *								*
 ****************************************************************/
/* *
 * xmlInitMemory:
 *
 * Initialize the memory layer.
 *
 * Returns 0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitMemory() -> std::os::raw::c_int {
    let mut breakpoint: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    /*
     This is really not good code (see Bug 130419).  Suggestions for
     improvement will be welcome!
    */
    if xmlMemInitialized != 0 { return -(1 as std::os::raw::c_int) }
    xmlMemInitialized = 1 as std::os::raw::c_int;
    xmlMemMutex = xmlNewMutex();
    breakpoint =
        getenv(b"XML_MEM_BREAKPOINT\x00" as *const u8 as *const std::os::raw::c_char);
    if !breakpoint.is_null() {
        sscanf(breakpoint, b"%ud\x00" as *const u8 as *const std::os::raw::c_char,
               &mut xmlMemStopAtBlock as *mut std::os::raw::c_uint);
    }
    breakpoint =
        getenv(b"XML_MEM_TRACE\x00" as *const u8 as *const std::os::raw::c_char);
    if !breakpoint.is_null() {
        sscanf(breakpoint, b"%p\x00" as *const u8 as *const std::os::raw::c_char,
               &mut xmlMemTraceBlockAt as *mut *mut std::os::raw::c_void);
    }
    return 0 as std::os::raw::c_int;
}
/*
 * Initialization of the memory layer.
 */
/*
 * Cleanup of the memory layer.
 */
/* *
 * xmlCleanupMemory:
 *
 * Free up all the memory allocated by the library for its own
 * use. This should not be called by user level code.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupMemory() {
    if xmlMemInitialized == 0 as std::os::raw::c_int { return }
    xmlFreeMutex(xmlMemMutex);
    xmlMemMutex = 0 as xmlMutexPtr;
    xmlMemInitialized = 0 as std::os::raw::c_int;
}
/* *
 * xmlMemSetup:
 * @freeFunc: the free() function to use
 * @mallocFunc: the malloc() function to use
 * @reallocFunc: the realloc() function to use
 * @strdupFunc: the strdup() function to use
 *
 * Override the default memory access functions with a new set
 * This has to be called before any other libxml routines !
 *
 * Should this be blocked if there was already some allocations
 * done ?
 *
 * Returns 0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemSetup(mut freeFunc: xmlFreeFunc,
                                     mut mallocFunc: xmlMallocFunc,
                                     mut reallocFunc: xmlReallocFunc,
                                     mut strdupFunc: xmlStrdupFunc)
 -> std::os::raw::c_int {
    if freeFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if mallocFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if reallocFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if strdupFunc.is_none() { return -(1 as std::os::raw::c_int) }
    xmlFree = freeFunc;
    xmlMalloc = mallocFunc;
    xmlMallocAtomic = mallocFunc;
    xmlRealloc = reallocFunc;
    xmlMemStrdup = strdupFunc;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlMemGet:
 * @freeFunc: place to save the free() function in use
 * @mallocFunc: place to save the malloc() function in use
 * @reallocFunc: place to save the realloc() function in use
 * @strdupFunc: place to save the strdup() function in use
 *
 * Provides the memory access functions set currently in use
 *
 * Returns 0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMemGet(mut freeFunc: *mut xmlFreeFunc,
                                   mut mallocFunc: *mut xmlMallocFunc,
                                   mut reallocFunc: *mut xmlReallocFunc,
                                   mut strdupFunc: *mut xmlStrdupFunc)
 -> std::os::raw::c_int {
    if !freeFunc.is_null() { *freeFunc = xmlFree }
    if !mallocFunc.is_null() { *mallocFunc = xmlMalloc }
    if !reallocFunc.is_null() { *reallocFunc = xmlRealloc }
    if !strdupFunc.is_null() { *strdupFunc = xmlMemStrdup }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlGcMemSetup:
 * @freeFunc: the free() function to use
 * @mallocFunc: the malloc() function to use
 * @mallocAtomicFunc: the malloc() function to use for atomic allocations
 * @reallocFunc: the realloc() function to use
 * @strdupFunc: the strdup() function to use
 *
 * Override the default memory access functions with a new set
 * This has to be called before any other libxml routines !
 * The mallocAtomicFunc is specialized for atomic block
 * allocations (i.e. of areas  useful for garbage collected memory allocators
 *
 * Should this be blocked if there was already some allocations
 * done ?
 *
 * Returns 0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGcMemSetup(mut freeFunc: xmlFreeFunc,
                                       mut mallocFunc: xmlMallocFunc,
                                       mut mallocAtomicFunc: xmlMallocFunc,
                                       mut reallocFunc: xmlReallocFunc,
                                       mut strdupFunc: xmlStrdupFunc)
 -> std::os::raw::c_int {
    if freeFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if mallocFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if mallocAtomicFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if reallocFunc.is_none() { return -(1 as std::os::raw::c_int) }
    if strdupFunc.is_none() { return -(1 as std::os::raw::c_int) }
    xmlFree = freeFunc;
    xmlMalloc = mallocFunc;
    xmlMallocAtomic = mallocAtomicFunc;
    xmlRealloc = reallocFunc;
    xmlMemStrdup = strdupFunc;
    return 0 as std::os::raw::c_int;
}
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
/* *
 * xmlMallocFunc:
 * @size:  the size requested in bytes
 *
 * Signature for a malloc() implementation.
 *
 * Returns a pointer to the newly allocated block or NULL in case of error.
 */
/* *
 * xmlReallocFunc:
 * @mem: an already allocated block of memory
 * @size:  the new size requested in bytes
 *
 * Signature for a realloc() implementation.
 *
 * Returns a pointer to the newly reallocated block or NULL in case of error.
 */
/* *
 * xmlStrdupFunc:
 * @str: a zero terminated string
 *
 * Signature for an strdup() implementation.
 *
 * Returns the copy of the string or NULL in case of error.
 */
/*
 * The 4 interfaces used for all memory handling within libxml.
LIBXML_DLL_IMPORT xmlFreeFunc xmlFree;
LIBXML_DLL_IMPORT xmlMallocFunc xmlMalloc;
LIBXML_DLL_IMPORT xmlMallocFunc xmlMallocAtomic;
LIBXML_DLL_IMPORT xmlReallocFunc xmlRealloc;
LIBXML_DLL_IMPORT xmlStrdupFunc xmlMemStrdup;
 */
/*
 * The way to overload the existing functions.
 * The xmlGc function have an extra entry for atomic block
 * allocations useful for garbage collected memory allocators
 */
/* *
 * xmlGcMemGet:
 * @freeFunc: place to save the free() function in use
 * @mallocFunc: place to save the malloc() function in use
 * @mallocAtomicFunc: place to save the atomic malloc() function in use
 * @reallocFunc: place to save the realloc() function in use
 * @strdupFunc: place to save the strdup() function in use
 *
 * Provides the memory access functions set currently in use
 * The mallocAtomicFunc is specialized for atomic block
 * allocations (i.e. of areas  useful for garbage collected memory allocators
 *
 * Returns 0 on success
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGcMemGet(mut freeFunc: *mut xmlFreeFunc,
                                     mut mallocFunc: *mut xmlMallocFunc,
                                     mut mallocAtomicFunc: *mut xmlMallocFunc,
                                     mut reallocFunc: *mut xmlReallocFunc,
                                     mut strdupFunc: *mut xmlStrdupFunc)
 -> std::os::raw::c_int {
    if !freeFunc.is_null() { *freeFunc = xmlFree }
    if !mallocFunc.is_null() { *mallocFunc = xmlMalloc }
    if !mallocAtomicFunc.is_null() { *mallocAtomicFunc = xmlMallocAtomic }
    if !reallocFunc.is_null() { *reallocFunc = xmlRealloc }
    if !strdupFunc.is_null() { *strdupFunc = xmlMemStrdup }
    return 0 as std::os::raw::c_int;
}
/* __INCLUDE_ELFGCCHACK */
