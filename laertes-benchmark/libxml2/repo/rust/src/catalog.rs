
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlRMutex;
    pub type _xmlBuf;
    pub type _xmlDict;
    /*
 * Summary: Chained hash tables
 * Description: This module implements the hash table support used in
 *		various places in the library.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Bjorn Reese <bjorn.reese@systematic.dk>
 */
    /*
 * The hash table.
 */
    pub type _xmlHashTable;
    /* *
 * xmlAutomataStatePtr:
 *
 * A state int the automata description,
 */
    pub type _xmlAutomataState;
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
    pub type _xmlAutomata;
    pub type _xmlValidState;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrncmp(str1: *const xmlChar, str2: *const xmlChar,
                  len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn __xstat(__ver: std::os::raw::c_int, __filename: *const std::os::raw::c_char,
               __stat_buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn close(__fd: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(__fd: std::os::raw::c_int, __buf: *mut std::os::raw::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlNewDtd(doc: xmlDocPtr, name: *const xmlChar,
                 ExternalID: *const xmlChar, SystemID: *const xmlChar)
     -> xmlDtdPtr;
    /* LIBXML_LEGACY_ENABLED */
    #[no_mangle]
    fn xmlNewNs(node: xmlNodePtr, href: *const xmlChar,
                prefix: *const xmlChar) -> xmlNsPtr;
    #[no_mangle]
    fn xmlFreeNs(cur: xmlNsPtr);
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr,
                         href: *const xmlChar) -> xmlNsPtr;
    /*
 * Changing the content.
 */
    #[no_mangle]
    fn xmlSetProp(node: xmlNodePtr, name: *const xmlChar,
                  value: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlSetNsProp(node: xmlNodePtr, ns: xmlNsPtr, name: *const xmlChar,
                    value: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlGetNsProp(node: *const xmlNode, name: *const xmlChar,
                    nameSpace: *const xmlChar) -> *mut xmlChar;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlSaveFormatFileTo(buf: xmlOutputBufferPtr, cur: xmlDocPtr,
                           encoding: *const std::os::raw::c_char, format: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /*
 * Constructor and destructor.
 */
    #[no_mangle]
    fn xmlHashCreate(size: std::os::raw::c_int) -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    /*
 * Add a new entry to the hash table.
 */
    #[no_mangle]
    fn xmlHashAddEntry(table: xmlHashTablePtr, name: *const xmlChar,
                       userdata: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    /*
 * Remove an entry from the hash table.
 */
    #[no_mangle]
    fn xmlHashRemoveEntry(table: xmlHashTablePtr, name: *const xmlChar,
                          f: xmlHashDeallocator) -> std::os::raw::c_int;
    /*
 * Retrieve the userdata.
 */
    #[no_mangle]
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlHashSize(table: xmlHashTablePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashScan(table: xmlHashTablePtr, f: xmlHashScanner,
                   data: *mut std::os::raw::c_void);
    /*
 * Internal callback reporting routine
 */
    #[no_mangle]
    fn __xmlRaiseError(schannel: xmlStructuredErrorFunc,
                       channel: xmlGenericErrorFunc, data: *mut std::os::raw::c_void,
                       ctx: *mut std::os::raw::c_void, node: *mut std::os::raw::c_void,
                       domain: std::os::raw::c_int, code: std::os::raw::c_int,
                       level: xmlErrorLevel, file: *const std::os::raw::c_char,
                       line: std::os::raw::c_int, str1: *const std::os::raw::c_char,
                       str2: *const std::os::raw::c_char, str3: *const std::os::raw::c_char,
                       int1: std::os::raw::c_int, col: std::os::raw::c_int,
                       msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlParserInputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                          enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserGetDirectory(filename: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn xmlOutputBufferCreateFile(file: *mut FILE,
                                 encoder: xmlCharEncodingHandlerPtr)
     -> xmlOutputBufferPtr;
    #[no_mangle]
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlGetThreadId() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeRMutex(tok: xmlRMutexPtr);
    #[no_mangle]
    fn xmlRMutexUnlock(tok: xmlRMutexPtr);
    #[no_mangle]
    fn xmlRMutexLock(tok: xmlRMutexPtr);
    #[no_mangle]
    fn xmlNewRMutex() -> xmlRMutexPtr;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr)
     -> std::os::raw::c_int;
    /* *
 * Range checking routine
 */
    #[no_mangle]
    fn xmlCharInRange(val: std::os::raw::c_uint, group: *const xmlChRangeGroup)
     -> std::os::raw::c_int;
    /* *
 * xmlIsBaseChar_ch:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsBaseCharQ:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    #[no_mangle]
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    /* *
 * xmlIsDigit_ch:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsDigitQ:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    #[no_mangle]
    static xmlIsDigitGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsPubidChar_tab: [std::os::raw::c_uchar; 256];
    #[no_mangle]
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr)
     -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type xmlRMutex = _xmlRMutex;
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
/*
 * xmlRMutex are reentrant mutual exception locks.
 */
pub type xmlRMutexPtr = *mut xmlRMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut std::os::raw::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
    pub rawconsumed: std::os::raw::c_ulong,
}
/* *
 * xmlBufPtr:
 *
 * A pointer to a buffer structure, the actual structure internals are not
 * public
 */
pub type xmlBufPtr = *mut xmlBuf;
/* in IO mode we may have a different base */
/* *
 * xmlBuf:
 *
 * A buffer structure, new one, the actual structure internals are not public
 */
pub type xmlBuf = _xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
/*
 * Block defining the handlers for non UTF-8 encodings.
 * If iconv is supported, there are two extra fields.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut std::os::raw::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut std::os::raw::c_void;
/* *
 * xmlCharEncodingOutputFunc:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to another
 * encoding.
 * Note: a first call designed to produce heading info is called with
 * in = NULL. If stateful this should also initialize the encoder state.
 *
 * Returns the number of bytes written, -1 if lack of space, or -2
 *     if the transcoding failed.
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictiable.
 * The value of @outlen after return is the number of octets produced.
 */
pub type xmlCharEncodingOutputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
/*
 * Summary: interface for the encoding conversion functions
 * Description: interface for the encoding conversion functions needed for
 *              XML basic encoding and iconv() support.
 *
 * Related specs are
 * rfc2044        (UTF-8 and UTF-16) F. Yergeau Alis Technologies
 * [ISO-10646]    UTF-8 and UTF-16 in Annexes
 * [ISO-8859-1]   ISO Latin-1 characters codes.
 * [UNICODE]      The Unicode Consortium, "The Unicode Standard --
 *                Worldwide Character Encoding -- Version 1.0", Addison-
 *                Wesley, Volume 1, 1991, Volume 2, 1992.  UTF-8 is
 *                described in Unicode Technical Report #4.
 * [US-ASCII]     Coded Character Set--7-bit American Standard Code for
 *                Information Interchange, ANSI X3.4-1986.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * xmlCharEncoding:
 *
 * Predefined values for some standard encodings.
 * Libxml does not do beforehand translation on UTF8 and ISOLatinX.
 * It also supports ASCII, ISO-8859-1, and UTF16 (LE and BE) by default.
 *
 * Anything else would have to be translated to UTF8 before being
 * given to the parser itself. The BOM for UTF16 and the encoding
 * declaration are looked at and a converter is looked for at that
 * point. If not found the parser stops here as asked by the XML REC. A
 * converter can be registered by the user using xmlRegisterCharEncodingHandler
 * but the current form doesn't allow stateful transcoding (a serious
 * problem agreed !). If iconv has been found it will be used
 * automatically and allow stateful transcoding, the simplest is then
 * to be sure to enable iconv and to provide iconv libs for the encoding
 * support needed.
 *
 * Note that the generic "UTF-16" is not a predefined value.  Instead, only
 * the specific UTF-16LE and UTF-16BE are present.
 */
/* No char encoding detected */
/* No char encoding detected */
/* UTF-8 */
/* UTF-16 little endian */
/* UTF-16 big endian */
/* UCS-4 little endian */
/* UCS-4 big endian */
/* EBCDIC uh! */
/* UCS-4 unusual ordering */
/* UCS-4 unusual ordering */
/* UCS-2 */
/* ISO-8859-1 ISO Latin 1 */
/* ISO-8859-2 ISO Latin 2 */
/* ISO-8859-3 */
/* ISO-8859-4 */
/* ISO-8859-5 */
/* ISO-8859-6 */
/* ISO-8859-7 */
/* ISO-8859-8 */
/* ISO-8859-9 */
/* ISO-2022-JP */
/* Shift_JIS */
/* EUC-JP */
/* pure ASCII */
/* *
 * xmlCharEncodingInputFunc:
 * @out:  a pointer to an array of bytes to store the UTF-8 result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of chars in the original encoding
 * @inlen:  the length of @in
 *
 * Take a block of chars in the original encoding and try to convert
 * it to an UTF-8 block of chars out.
 *
 * Returns the number of bytes written, -1 if lack of space, or -2
 *     if the transcoding failed.
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictiable.
 * The value of @outlen after return is the number of octets consumed.
 */
pub type xmlCharEncodingInputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
pub type xmlInputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlInputReadCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
/*
 * Summary: interfaces for tree manipulation
 * Description: this module describes the structures found in an tree resulting
 *              from an XML or HTML parsing, as well as the API provided for
 *              various processing on that tree
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Some of the basic types pointer to structures:
 */
/* xmlIO.h */
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
/* LIBXML_OUTPUT_ENABLED */
/* I18N conversions to UTF-8 */
/* Local buffer encoded in UTF-8 */
/* if encoder != NULL buffer for raw input */
/* -1=unknown, 0=not compressed, 1=compressed */
/* amount consumed from raw */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut std::os::raw::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: std::os::raw::c_int,
    pub error: std::os::raw::c_int,
}
/* *
 * xmlOutputCloseCallback:
 * @context:  an Output context
 *
 * Callback used in the I/O Output API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
pub type xmlOutputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
/*
 * Summary: interface for the I/O interfaces used by the parser
 * Description: interface for the I/O interfaces used by the parser
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Those are the functions and datatypes for the parser input
 * I/O structures.
 */
/* *
 * xmlInputMatchCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to detect if the current handler
 * can provide input fonctionnalities for this resource.
 *
 * Returns 1 if yes and 0 if another Input module should be used
 */
/* *
 * xmlInputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to open the resource
 *
 * Returns an Input context or NULL in case or error
 */
/* *
 * xmlInputReadCallback:
 * @context:  an Input context
 * @buffer:  the buffer to store data read
 * @len:  the length of the buffer in bytes
 *
 * Callback used in the I/O Input API to read the resource
 *
 * Returns the number of bytes read or -1 in case of error
 */
/* *
 * xmlInputCloseCallback:
 * @context:  an Input context
 *
 * Callback used in the I/O Input API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
/*
 * Those are the functions and datatypes for the library output
 * I/O structures.
 */
/* *
 * xmlOutputMatchCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Output API to detect if the current handler
 * can provide output fonctionnalities for this resource.
 *
 * Returns 1 if yes and 0 if another Output module should be used
 */
/* *
 * xmlOutputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Output API to open the resource
 *
 * Returns an Output context or NULL in case or error
 */
/* *
 * xmlOutputWriteCallback:
 * @context:  an Output context
 * @buffer:  the buffer of data to write
 * @len:  the length of the buffer in bytes
 *
 * Callback used in the I/O Output API to write to the resource
 *
 * Returns the number of bytes written or -1 in case of error
 */
pub type xmlOutputWriteCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const std::os::raw::c_char,
    pub directory: *const std::os::raw::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: std::os::raw::c_int,
    pub line: std::os::raw::c_int,
    pub col: std::os::raw::c_int,
    pub consumed: std::os::raw::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: std::os::raw::c_int,
    pub id: std::os::raw::c_int,
}
pub type xmlParserInputDeallocate
    =
    Option<unsafe extern "C" fn(_: *mut xmlChar) -> ()>;
/* parser.h */
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut std::os::raw::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: std::os::raw::c_int,
    pub replaceEntities: std::os::raw::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: std::os::raw::c_int,
    pub html: std::os::raw::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: std::os::raw::c_int,
    pub inputMax: std::os::raw::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: std::os::raw::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: std::os::raw::c_int,
    pub hasExternalSubset: std::os::raw::c_int,
    pub hasPErefs: std::os::raw::c_int,
    pub external: std::os::raw::c_int,
    pub valid: std::os::raw::c_int,
    pub validate: std::os::raw::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: std::os::raw::c_int,
    pub directory: *mut std::os::raw::c_char,
    pub name: *const xmlChar,
    pub nameNr: std::os::raw::c_int,
    pub nameMax: std::os::raw::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: std::os::raw::c_long,
    pub checkIndex: std::os::raw::c_long,
    pub keepBlanks: std::os::raw::c_int,
    pub disableSAX: std::os::raw::c_int,
    pub inSubset: std::os::raw::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut std::os::raw::c_int,
    pub spaceNr: std::os::raw::c_int,
    pub spaceMax: std::os::raw::c_int,
    pub spaceTab: *mut std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: std::os::raw::c_int,
    pub nodelen: std::os::raw::c_int,
    pub nodemem: std::os::raw::c_int,
    pub pedantic: std::os::raw::c_int,
    pub _private: *mut std::os::raw::c_void,
    pub loadsubset: std::os::raw::c_int,
    pub linenumbers: std::os::raw::c_int,
    pub catalogs: *mut std::os::raw::c_void,
    pub recovery: std::os::raw::c_int,
    pub progressive: std::os::raw::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: std::os::raw::c_int,
    pub docdict: std::os::raw::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: std::os::raw::c_int,
    pub nsNr: std::os::raw::c_int,
    pub nsMax: std::os::raw::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut std::os::raw::c_int,
    pub pushTab: *mut *mut std::os::raw::c_void,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
    pub dictNames: std::os::raw::c_int,
    pub freeElemsNr: std::os::raw::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: std::os::raw::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: std::os::raw::c_ulong,
    pub sizeentities: std::os::raw::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: std::os::raw::c_int,
    pub nodeInfoMax: std::os::raw::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: std::os::raw::c_int,
    pub sizeentcopy: std::os::raw::c_ulong,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: std::os::raw::c_ulong,
    pub begin_line: std::os::raw::c_ulong,
    pub end_pos: std::os::raw::c_ulong,
    pub end_line: std::os::raw::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut std::os::raw::c_void,
    pub line: std::os::raw::c_ushort,
    pub extra: std::os::raw::c_ushort,
}
/* *
 * xmlNs:
 *
 * An XML namespace.
 * Note that prefix == NULL is valid, it defines the default namespace
 * within the subtree (until overridden).
 *
 * xmlNsType is unified with xmlElementType.
 */
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut std::os::raw::c_void,
    pub context: *mut _xmlDoc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *mut std::os::raw::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: std::os::raw::c_int,
    pub standalone: std::os::raw::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut std::os::raw::c_void,
    pub refs: *mut std::os::raw::c_void,
    pub URL: *const xmlChar,
    pub charset: std::os::raw::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut std::os::raw::c_void,
    pub parseFlags: std::os::raw::c_int,
    pub properties: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut std::os::raw::c_void,
    pub elements: *mut std::os::raw::c_void,
    pub attributes: *mut std::os::raw::c_void,
    pub entities: *mut std::os::raw::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut std::os::raw::c_void,
}
pub type xmlElementType = std::os::raw::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
/* the validating regexp */
/* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
pub type xmlNsType = xmlElementType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut std::os::raw::c_void,
}
pub type xmlAttributeType = std::os::raw::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlParserMode = std::os::raw::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: std::os::raw::c_int,
    pub code: std::os::raw::c_int,
    pub message: *mut std::os::raw::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut std::os::raw::c_char,
    pub line: std::os::raw::c_int,
    pub str1: *mut std::os::raw::c_char,
    pub str2: *mut std::os::raw::c_char,
    pub str3: *mut std::os::raw::c_char,
    pub int1: std::os::raw::c_int,
    pub int2: std::os::raw::c_int,
    pub ctxt: *mut std::os::raw::c_void,
    pub node: *mut std::os::raw::c_void,
}
pub type xmlErrorLevel = std::os::raw::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut xmlAttr;
/* Hash table for param entities if any */
/* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputState = std::os::raw::c_int;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
pub type xmlValidCtxt = _xmlValidCtxt;
/* *
 * XML_CTXT_FINISH_DTD_0:
 *
 * Special value for finishDtd field when embedded in an xmlParserCtxt
 */
/* *
 * XML_CTXT_FINISH_DTD_1:
 *
 * Special value for finishDtd field when embedded in an xmlParserCtxt
 */
/*
 * xmlValidCtxt:
 * An xmlValidCtxt is used for error reporting when validating.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub finishDtd: std::os::raw::c_uint,
    pub doc: xmlDocPtr,
    pub valid: std::os::raw::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: std::os::raw::c_int,
    pub vstateMax: std::os::raw::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
/* *
 * xmlDoc:
 *
 * An XML document.
 */
pub type xmlDoc = _xmlDoc;
/* *
 * xmlValidityWarningFunc:
 * @ctx:  usually an xmlValidCtxtPtr to a validity error context,
 *        but comes from ctxt->userData (which normally contains such
 *        a pointer); ctxt->userData can be changed by the user.
 * @msg:  the string to format *printf like vararg
 * @...:  remaining arguments to the format
 *
 * Callback called when a validity warning is found. This is a message
 * oriented function similar to an *printf function.
 */
pub type xmlValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/*
 * Summary: The DTD validation
 * Description: API for the DTD handling and the validity checking
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Validation state added for non-determinist content model.
 */
/* *
 * xmlValidityErrorFunc:
 * @ctx:  usually an xmlValidCtxtPtr to a validity error context,
 *        but comes from ctxt->userData (which normally contains such
 *        a pointer); ctxt->userData can be changed by the user.
 * @msg:  the string to format *printf like vararg
 * @...:  remaining arguments to the format
 *
 * Callback called when a validity error is found. This is a message
 * oriented function similar to an *printf function.
 */
pub type xmlValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: std::os::raw::c_ulong,
    pub length: std::os::raw::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: std::os::raw::c_uint,
    pub _private: *mut std::os::raw::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
/* What part of the library raised this error */
/* The error code, e.g. an xmlParserError */
/* human-readable informative error message */
/* how consequent is the error */
/* the filename */
/* the line number if available */
/* extra string information */
/* extra string information */
/* extra string information */
/* extra number information */
/* error column # or 0 if N/A (todo: rename field when we would brk ABI) */
/* the parser context if available */
/* the node in the tree */
/* *
 * xmlParserError:
 *
 * This is an error that the XML (or HTML) parser can generate
 */
/* 1 */
/* 2 */
/* 3 */
/* 4 */
/* 5 */
/* 6 */
/* 7 */
/* 8 */
/* 9 */
/* 10 */
/* 11 */
/* 12 */
/* 13 */
/* 14 */
/* 15 */
/* 16 */
/* 17 */
/* 18 */
/* 19 */
/* 20 */
/* 21 */
/* 22 */
/* 23 */
/* 24 */
/* 25 */
/* 26 */
/* 27 */
/* 28 */
/* 29 */
/* 30 */
/* 31 */
/* 32 */
/* 33 */
/* 34 */
/* 35 */
/* 36 */
/* 37 */
/* 38 */
/* 39 */
/* 40 */
/* 41 */
/* 42 */
/* 43 */
/* 44 */
/* 45 */
/* 46 */
/* 47 */
/* 48 */
/* 49 */
/* 50 */
/* 51 */
/* 52 */
/* 53 */
/* 54 */
/* 55 */
/* 56 */
/* 57 */
/* 58 */
/* 59 */
/* 60 */
/* 61 */
/* 62 */
/* 63 */
/* 64 */
/* 65 */
/* 66 */
/* 67 */
/* 68 */
/* 69 */
/* 70 */
/* 71 */
/* 72 */
/* 73 */
/* 74 */
/* 75 */
/* 76 */
/* 77 */
/* 78 */
/* 79 */
/* 80 */
/* 81 */
/* 82 */
/* 83 */
/* 84 */
/* 85 */
/* 86 */
/* 87 */
/* 88 */
/* 89 */
/* 90 */
/* 91 */
/* 92 */
/* 93 */
/* 94 */
/* 95 */
/* 96 */
/* 97 */
/* 98 */
/* 99 */
/* 100 */
/* 101 */
/* 102 */
/* 103 */
/* 104 */
/* 105 */
/* 106 */
/* 107 */
/* 108 */
/* 109 */
/* 110 */
/* 111 */
/* 201 */
/* 202 */
/* 203 */
/* 204 */
/* 205 */
/* 501 */
/* 502 */
/* 503 */
/* 504 */
/* 505 */
/* 506 */
/* 507 */
/* 508 */
/* 509 */
/* 510 */
/* 511 */
/* 512 */
/* 513 */
/* 514 */
/* 515 */
/* 516 */
/* 517 */
/* 518 */
/* 519 */
/* 520 */
/* 521 */
/* 522 */
/* 523 */
/* 524 */
/* 525 */
/* 526 */
/* 527 */
/* 528 */
/* 529 */
/* 530 */
/* 531 */
/* 532 */
/* 533 */
/* 534 */
/* 535 */
/* 536 */
/* 537 */
/* 538 */
/* 539 */
/* 540 */
/* 541 */
/* 801 */
/* 1001 */
/* 1002 */
/* 1003 */
/* 1004 */
/* 1005 */
/* 1006 */
/* 1007 */
/* 1008 */
/* 1009 */
/* 1010 */
/* 1011 */
/* 1012 */
/* 1013 */
/* 1014 */
/* 1015 */
/* 1016 */
/* 1017 */
/* 1018 */
/* 1019 */
/* 1020 */
/* 1021 */
/* 1022 */
/* 1023 */
/* 1024 */
/* 1025 */
/* 1026 */
/* 1027 */
/* 1028 */
/* 1029 */
/* 1030 */
/* 1031 */
/* 1032 */
/* 1033 */
/* 1034 */
/* 1035 */
/* 1036 */
/* 1037 */
/* 1038 */
/* 1039 */
/* 1040 */
/* 1041 */
/* 1042 */
/* 1043 */
/* 1044 */
/* 1045 */
/* 1046 */
/* 1047 */
/* 1048 */
/* 1049 */
/* 1050 */
/* 1051 */
/* 1052 */
/* 1053 */
/* 1054 */
/* 1055 */
/* 1056 */
/* 1057 */
/* 1058 */
/* 1059 */
/* 1060 */
/* 1061 */
/* 1062 */
/* 1063 */
/* 1064 */
/* 1065 */
/* 1066 */
/* 1067 */
/* 1068 */
/* 1069 */
/* 1070 */
/* 1071 */
/* 1072 */
/* 1073 */
/* 1074 */
/* 1075 */
/* 1076 */
/* 1077 */
/* 1078 */
/* 1079 */
/* 1080 */
/* 1081 */
/* 1082 */
/* 1083 */
/* 1084 */
/* 1085 */
/* 1086 */
/* 1087 */
/* 1088 */
/* 1089 */
/* 1090 */
/* 1091 */
/* 1092 */
/* 1093 */
/* 1094 */
/* 1095 */
/* 1096 */
/* 1097 */
/* 1098 */
/* 1099 */
/* 1100 */
/* 1101 */
/* 1102 */
/* 1103 */
/* 1104 */
/* 1105 */
/* 1106 */
/* 1107 */
/* 1108 */
/* 1109 */
/* 1110 */
/* 1111 */
/* 1112 */
/* 1113 */
/* 1114 */
/* 1115 */
/* 1116 */
/* 1117 */
/* 1118 */
/* 1119 */
/* 1120 */
/* 1121 */
/* 1122 */
/* 1201 */
/* 1202 */
/* 1203 */
/* 1204 */
/* 1205 */
/* 1206 */
/* 1207 */
/* 1208 */
/* 1209 */
/* 1210 */
/* 1211 */
/* 1212 */
/* 1213 */
/* 1214 */
/* 1215 */
/* 1216 */
/* 1217 */
/* 1218 */
/* 1219 */
/* 1220 */
/* 1221 */
/* 1301 */
/* 1302 */
/* 1303 */
/* 1401 */
/* 1402 */
/* 1403 */
/* 1501 */
/* 1502 */
/* 1503 */
/* 1504 */
/* 1505 */
/* 1506 */
/* 1507 */
/* 1508 */
/* 1509 */
/* 1510 */
/* 1511 */
/* 1512 */
/* 1513 */
/* 1514 */
/* 1515 */
/* 1516 */
/* 1517 */
/* 1518 */
/* 1519 */
/* 1520 */
/* 1521 */
/* 1522 */
/* 1523 */
/* 1524 */
/* 1525 */
/* 1526 */
/* 1527 */
/* 1528 */
/* 1529 */
/* 1530 */
/* 1531 */
/* 1532 */
/* 1533 */
/* 1534 */
/* 1535 */
/* 1536 */
/* 1537 */
/* 1538 */
/* 1539 */
/* 1540 */
/* 1541 */
/* 1542 */
/* 1543 */
/* 1544 */
/* 1545 */
/* 1546 */
/* 1547 */
/* 1548 */
/* 1549 */
/* 1550 */
/* 1551 */
/* 1552 */
/* 1553 */
/* 1554 */
/* 1555 */
/* 1556 */
/* 1601 */
/* 1602 */
/* 1603 */
/* 1604 */
/* 1605 */
/* 1606 */
/* 1607 */
/* 1608 */
/* 1609 */
/* 1610 */
/* 1611 */
/* 1612 */
/* 1613 */
/* 1614 */
/* 1615 */
/* 1616 */
/* 1617 */
/* 1618 */
/* 1651 */
/* 1652 */
/* 1653 */
/* 1654 */
/* 1701 */
/* 1702 */
/* 1703 */
/* 1704 */
/* 1705 */
/* 1706 */
/* 1707 */
/* 1708 */
/* 1709 */
/* 1710 */
/* 1711 */
/* 1712 */
/* 1713 */
/* 1714 */
/* 1715 */
/* 1716 */
/* 1717 */
/* 1718 */
/* 1719 */
/* 1720 */
/* 1721 */
/* 1722 */
/* 1723 */
/* 1724 */
/* 1725 */
/* 1726 */
/* 1727 */
/* 1728 */
/* 1729 */
/* 1730 */
/* 1731 */
/* 1732 */
/* 1733 */
/* 1734 */
/* 1735 */
/* 1736 */
/* 1737 */
/* 1738 */
/* 1739 */
/* 1740 */
/* 1741 */
/* 1742 */
/* 1743 */
/* 1744 */
/* 1745 */
/* 1746 */
/* 1747 */
/* 1748 */
/* 1749 */
/* 1750 */
/* 1751 */
/* 1752 */
/* 1753 */
/* 1754 */
/* 1755 */
/* 1756 */
/* 1757 */
/* 1758 */
/* 1759 */
/* 1760 */
/* 1761 */
/* 1762 */
/* 1763 */
/* 1764 */
/* 1765 */
/* 1766 */
/* 1767 */
/* 1768 */
/* 1769 */
/* 1770 */
/* 1771 */
/* 1772 */
/* 1773 */
/* 1774 */
/* 1775 */
/* 1776 */
/* 1777 */
/* 1778 */
/* 1779 */
/* 1780 */
/* 1781 */
/* 1782 */
/* 1783 */
/* 1784 */
/* 1785 */
/* 1786 */
/* 1787 */
/* 1788 */
/* 1789 */
/* 1790 */
/* 1791 */
/* 1792 */
/* 1793 */
/* 1794 */
/* 1795 */
/* 1796 */
/* 1797 */
/* 1798 */
/* 1799 */
/* 1800 */
/* 1802 */
/* 1803 */
/* 1804 */
/* 1805 */
/* 1806 */
/* 1807 */
/* 1808 */
/* 1809 */
/* 1810 */
/* 1811 */
/* 1812 */
/* 1813 */
/* 1814 */
/* 1815 */
/* 1816 */
/* 1817 */
/* 1818 */
/* 1819 */
/* 1820 */
/* 1821 */
/* 1822 */
/* 1823 */
/* 1824 */
/* 1825 */
/* 1826 */
/* 1827 */
/* 1828 */
/* 1829 */
/* 1830 */
/* 1831 */
/* 1832 */
/* 1833 */
/* 1834 */
/* 1835 */
/* 1836 */
/* 1837 */
/* 1838 */
/* 1839 */
/* 1840 */
/* 1841 */
/* 1842 */
/* 1843 */
/* 1844 */
/* 1845 */
/* 1846 */
/* 1847 */
/* 1848 */
/* 1849 */
/* 1850 */
/* 1851 */
/* 1852 */
/* 1853 */
/* 1854 */
/* 1855 */
/* 1856 */
/* 1857 */
/* 1858 */
/* 1859 */
/* 1860 */
/* 1861 */
/* 1862 */
/* 1863 */
/* 1864 */
/* 1865 */
/* 1866 */
/* 1867 */
/* 1868 */
/* 1869 */
/* 1870 */
/* 1871 */
/* 1872 */
/* 1873 */
/* 1874 */
/* 1875 */
/* 1876 */
/* 1877 */
/* 1878 */
/* 1879 */
/* 1901 */
/* 1902 */
/* 1903 */
/* 1951 */
/* 1952 */
/* 1953 */
/* 1954 */
/* 1955 */
/* 2001 */
/* 2002 */
/* 2003 */
/* 2021 */
/* 2022 */
/* 3001 */
/* 3002 */
/* 3003 */
/* 3004 */
/* 3005 */
/* 3006 */
/* 3007 */
/* 3008 */
/* 3009 */
/* 3010 */
/* 3011 */
/* 3012 */
/* 3013 */
/* 3014 */
/* 3015 */
/* 3016 */
/* 3017 */
/* 3018 */
/* 3019 */
/* 3020 */
/* 3021 */
/* 3022 */
/* 3023 */
/* 3024 */
/* 3025 */
/* 3026 */
/* 3027 */
/* 3028 */
/* 3029 */
/* 3030 */
/* 3031 */
/* 3032 */
/* 3033 */
/* 3034 */
/* 3035 */
/* 3036 */
/* 3037 */
/* 3038 */
/* 3039 */
/* 3040 */
/* 3041 */
/* 3042 */
/* 3043 */
/* 3044 */
/* 3045 */
/* 3046 */
/* 3047 */
/* 3048 */
/* 3049 */
/* 3050 */
/* 3051 */
/* 3052 */
/* 3053 */
/* 3054 */
/* 3055 */
/* 3056 */
/* 3057 */
/* 3058 */
/* 3059 */
/* 3060 */
/* 3061 */
/* 3062 */
/* 3063 */
/* 3064 */
/* 3065 */
/* 3066 */
/* 3067 */
/* 3068 */
/* 3069 non-W3C */
/* 3070 non-W3C */
/* 3071 */
/* 3072 */
/* 3073 */
/* 3074 */
/* 3075 */
/* 3076 */
/* 3077 */
/* 3078 */
/* 3079 */
/* 3080 */
/* 3081 */
/* 3082 */
/* 3083 */
/* 3084 */
/* 3085 */
/* 3085 */
/* 3086 */
/* 3087 */
/* 3088 */
/* 3089 */
/* 3090 */
/* 4000 */
/* 4900 */
/* 4901 */
/* 5001 */
/* 5002 */
/* 5003 */
/* 5004 */
/* 5005 */
/* 5006 */
/* 5007 */
/* 5008 */
/* 5009 */
/* 5010 */
/* 5011 */
/* 5012 */
/* 5013 */
/* 5014 */
/* 5015 */
/* 5016 */
/* 5017 */
/* 5018 */
/* 5019 */
/* 5020 */
/* 5021 */
/* 5022 */
/* 5023 */
/* 5024 */
/* 5025 */
/* 5026 */
/* 5027 */
/* 5028 */
/* 5029 */
/* 5030 */
/* 5031 */
/* 5032 */
/* 5033 */
/* 5034 */
/* 5035 */
/* 5036 */
/* 5037 */
/* 6001 */
/* 6002 */
/* 6003 */
/* 6004 */
/* *
 * xmlGenericErrorFunc:
 * @ctx:  a parsing context
 * @msg:  the message
 * @...:  the extra arguments of the varags to format the message
 *
 * Signature of the function to use when there is an error and
 * no parsing or validity context available .
 */
/* *
 * xmlStructuredErrorFunc:
 * @userData:  user provided data for the error callback
 * @error:  the error being raised.
 *
 * Signature of the function to use when there is an error and
 * the module handles the new error reporting mechanism.
 */
pub type xmlStructuredErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlErrorPtr) -> ()>;
/*
 * Summary: error handling
 * Description: the API used to report errors
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlErrorLevel:
 *
 * Indicates the level of an error
 */
/* A simple warning */
/* A recoverable error */
/* A fatal error */
/* *
 * xmlErrorDomain:
 *
 * Indicates where an error may have come from
 */
/* The XML parser */
/* The tree module */
/* The XML Namespace module */
/* The XML DTD validation with parser context*/
/* The HTML parser */
/* The memory allocator */
/* The serialization code */
/* The Input/Output stack */
/* The FTP module */
/* The HTTP module */
/* The XInclude processing */
/* The XPath module */
/* The XPointer module */
/* The regular expressions module */
/* The W3C XML Schemas Datatype module */
/* The W3C XML Schemas parser module */
/* The W3C XML Schemas validation module */
/* The Relax-NG parser module */
/* The Relax-NG validator module */
/* The Catalog module */
/* The Canonicalization module */
/* The XSLT engine from libxslt */
/* The XML DTD validation with valid context */
/* The error checking module */
/* The xmlwriter module */
/* The dynamically loaded module module*/
/* The module handling character conversion */
/* The Schematron validator module */
/* The buffers module */
/* The URI module */
/* *
 * xmlError:
 *
 * An XML Error instance.
 */
pub type xmlErrorPtr = *mut xmlError;
pub type endElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type startElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *mut *const xmlChar,
                                _: std::os::raw::c_int, _: std::os::raw::c_int,
                                _: *mut *const xmlChar) -> ()>;
pub type externalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type cdataBlockSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type getParameterEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
pub type xmlEntityPtr = *mut xmlEntity;
/* entities.h */
pub type xmlEntity = _xmlEntity;
/*
 * An unit of storage for an entity, contains the string, the value
 * and the linkind data needed for the linking in the hash table.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: std::os::raw::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: std::os::raw::c_int,
    pub checked: std::os::raw::c_int,
}
/*
 * Summary: interface for the XML entities handling
 * Description: this module provides some of the entity API needed
 *              for the parser and applications.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * The different valid entity types.
 */
pub type xmlEntityType = std::os::raw::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type errorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type warningSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type commentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type processingInstructionSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
pub type ignorableWhitespaceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type charactersSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type referenceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type endElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
pub type startElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *mut *const xmlChar) -> ()>;
pub type endDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type startDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlSAXLocatorPtr)
               -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                  -> std::os::raw::c_int>,
    pub getColumnNumber: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                    -> std::os::raw::c_int>,
}
pub type unparsedEntityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
pub type elementDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: xmlElementContentPtr)
               -> ()>;
pub type xmlElementContentPtr = *mut xmlElementContent;
/* *
 * xmlElementContent:
 *
 * An XML Element content as stored after parsing an element definition
 * in a DTD.
 */
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
pub type xmlElementContentOccur = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: std::os::raw::c_int,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: xmlEnumerationPtr) -> ()>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
/* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type entityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: *const xmlChar, _: *mut xmlChar) -> ()>;
pub type getEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
pub type resolveEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlParserInputPtr>;
pub type hasExternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type hasInternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type isStandaloneSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type internalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlNsPtr = *mut xmlNs;
/* normally an xmlDoc */
/* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
/*
 * Recent version of gcc produce a warning when a function pointer is assigned
 * to an object pointer, or vice versa.  The following macro is a dirty hack
 * to allow suppression of the warning.  If your architecture has function
 * pointers which are a different size than a void pointer, there may be some
 * serious trouble within the library.
 */
/* *
 * XML_CAST_FPTR:
 * @fptr:  pointer to a function
 *
 * Macro to do a casting from an object pointer to a
 * function pointer without encountering a warning from
 * gcc
 *
 * #define XML_CAST_FPTR(fptr) (*(void **)(&fptr))
 * This macro violated ISO C aliasing rules (gcc4 on s390 broke)
 * so it is disabled now
 */
/*
 * function types:
 */
/* *
 * xmlHashDeallocator:
 * @payload:  the data in the hash
 * @name:  the name associated
 *
 * Callback to free data from a hash.
 */
pub type xmlHashDeallocator
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
/* *
 * xmlHashScanner:
 * @payload:  the data in the hash
 * @data:  extra scannner data
 * @name:  the name associated
 *
 * Callback when scanning data in a hash with the simple scanner.
 */
pub type xmlHashScanner
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_void,
                                _: *const xmlChar) -> ()>;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const XML_FROM_URI: C2RustUnnamed = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed = 28;
pub const XML_FROM_I18N: C2RustUnnamed = 27;
pub const XML_FROM_MODULE: C2RustUnnamed = 26;
pub const XML_FROM_WRITER: C2RustUnnamed = 25;
pub const XML_FROM_CHECK: C2RustUnnamed = 24;
pub const XML_FROM_VALID: C2RustUnnamed = 23;
pub const XML_FROM_XSLT: C2RustUnnamed = 22;
pub const XML_FROM_C14N: C2RustUnnamed = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed = 13;
pub const XML_FROM_XPATH: C2RustUnnamed = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed = 11;
pub const XML_FROM_HTTP: C2RustUnnamed = 10;
pub const XML_FROM_FTP: C2RustUnnamed = 9;
pub const XML_FROM_IO: C2RustUnnamed = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
pub const XML_FROM_HTML: C2RustUnnamed = 5;
pub const XML_FROM_DTD: C2RustUnnamed = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandlerV1 {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: std::os::raw::c_uint,
}
pub type xmlSAXHandlerV1 = _xmlSAXHandlerV1;
pub type xmlCharEncoding = std::os::raw::c_int;
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
/*
 * Summary: Unicode character range checking
 * Description: this module exports interfaces for the character
 *               range validation APIs
 *
 * This file is automatically generated from the cvs source
 * definition files using the genChRanges.py Python script
 *
 * Generation date: Mon Mar 27 11:09:48 2006
 * Sources: chvalid.def
 * Author: William Brack <wbrack@mmm.com.hk>
 */
/*
 * Define our typedefs and structures
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: std::os::raw::c_ushort,
    pub high: std::os::raw::c_ushort,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: std::os::raw::c_uint,
    pub high: std::os::raw::c_uint,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: std::os::raw::c_int,
    pub nbLongRange: std::os::raw::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
pub type xmlCatalogPrefer = std::os::raw::c_uint;
pub const XML_CATA_PREFER_SYSTEM: xmlCatalogPrefer = 2;
pub const XML_CATA_PREFER_PUBLIC: xmlCatalogPrefer = 1;
pub const XML_CATA_PREFER_NONE: xmlCatalogPrefer = 0;
pub type xmlCatalogAllow = std::os::raw::c_uint;
pub const XML_CATA_ALLOW_ALL: xmlCatalogAllow = 3;
pub const XML_CATA_ALLOW_DOCUMENT: xmlCatalogAllow = 2;
pub const XML_CATA_ALLOW_GLOBAL: xmlCatalogAllow = 1;
pub const XML_CATA_ALLOW_NONE: xmlCatalogAllow = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCatalog {
    pub type_0: xmlCatalogType,
    pub catalTab: [*mut std::os::raw::c_char; 10],
    pub catalNr: std::os::raw::c_int,
    pub catalMax: std::os::raw::c_int,
    pub sgml: xmlHashTablePtr,
    pub prefer: xmlCatalogPrefer,
    pub xml: xmlCatalogEntryPtr,
}
pub type xmlCatalogEntryPtr = *mut xmlCatalogEntry;
pub type xmlCatalogEntry = _xmlCatalogEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCatalogEntry {
    pub next: *mut _xmlCatalogEntry,
    pub parent: *mut _xmlCatalogEntry,
    pub children: *mut _xmlCatalogEntry,
    pub type_0: xmlCatalogEntryType,
    pub name: *mut xmlChar,
    pub value: *mut xmlChar,
    pub URL: *mut xmlChar,
    pub prefer: xmlCatalogPrefer,
    pub dealloc: std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub group: *mut _xmlCatalogEntry,
}
pub type xmlCatalogEntryType = std::os::raw::c_int;
pub const SGML_CATA_SGMLDECL: xmlCatalogEntryType = 24;
pub const SGML_CATA_DOCUMENT: xmlCatalogEntryType = 23;
pub const SGML_CATA_CATALOG: xmlCatalogEntryType = 22;
pub const SGML_CATA_BASE: xmlCatalogEntryType = 21;
pub const SGML_CATA_DELEGATE: xmlCatalogEntryType = 20;
pub const SGML_CATA_NOTATION: xmlCatalogEntryType = 19;
pub const SGML_CATA_LINKTYPE: xmlCatalogEntryType = 18;
pub const SGML_CATA_DOCTYPE: xmlCatalogEntryType = 17;
pub const SGML_CATA_PENTITY: xmlCatalogEntryType = 16;
pub const SGML_CATA_ENTITY: xmlCatalogEntryType = 15;
pub const SGML_CATA_PUBLIC: xmlCatalogEntryType = 14;
pub const SGML_CATA_SYSTEM: xmlCatalogEntryType = 13;
pub const XML_CATA_DELEGATE_URI: xmlCatalogEntryType = 12;
pub const XML_CATA_REWRITE_URI: xmlCatalogEntryType = 11;
pub const XML_CATA_URI: xmlCatalogEntryType = 10;
pub const XML_CATA_DELEGATE_SYSTEM: xmlCatalogEntryType = 9;
pub const XML_CATA_DELEGATE_PUBLIC: xmlCatalogEntryType = 8;
pub const XML_CATA_REWRITE_SYSTEM: xmlCatalogEntryType = 7;
pub const XML_CATA_SYSTEM: xmlCatalogEntryType = 6;
pub const XML_CATA_PUBLIC: xmlCatalogEntryType = 5;
pub const XML_CATA_GROUP: xmlCatalogEntryType = 4;
pub const XML_CATA_NEXT_CATALOG: xmlCatalogEntryType = 3;
pub const XML_CATA_BROKEN_CATALOG: xmlCatalogEntryType = 2;
pub const XML_CATA_CATALOG: xmlCatalogEntryType = 1;
pub const XML_CATA_NONE: xmlCatalogEntryType = 0;
pub const XML_CATA_REMOVED: xmlCatalogEntryType = -1;
pub type xmlCatalogType = std::os::raw::c_uint;
pub const XML_SGML_CATALOG_TYPE: xmlCatalogType = 2;
pub const XML_XML_CATALOG_TYPE: xmlCatalogType = 1;
pub type xmlCatalog = _xmlCatalog;
pub type xmlCatalogPtr = *mut xmlCatalog;
#[inline]
unsafe extern "C" fn stat(mut __path: *const std::os::raw::c_char,
                          mut __statbuf: *mut stat) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
/* ***********************************************************************
 *									*
 *			Global variables				*
 *									*
 ************************************************************************/
/*
 * Those are preferences
 */
static mut xmlDebugCatalogs: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* used for debugging */
static mut xmlCatalogDefaultAllow: xmlCatalogAllow = XML_CATA_ALLOW_ALL;
static mut xmlCatalogDefaultPrefer: xmlCatalogPrefer = XML_CATA_PREFER_PUBLIC;
/*
 * Hash table containing all the trees of XML catalogs parsed by
 * the application.
 */
static mut xmlCatalogXMLFiles: xmlHashTablePtr =
    0 as *const xmlHashTable as xmlHashTablePtr;
/*
 * The default catalog in use by the application
 */
static mut xmlDefaultCatalog: xmlCatalogPtr =
    0 as *const xmlCatalog as xmlCatalogPtr;
/*
 * A mutex for modifying the shared global catalog(s)
 * xmlDefaultCatalog tree.
 * It also protects xmlCatalogXMLFiles
 * The core of this readers/writer scheme is in xmlFetchXMLCatalogFile()
 */
static mut xmlCatalogMutex: xmlRMutexPtr =
    0 as *const xmlRMutex as xmlRMutexPtr;
/*
 * Whether the catalog support was initialized.
 */
static mut xmlCatalogInitialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* ***********************************************************************
 *									*
 *			Catalog error handlers				*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogErrMemory:
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlCatalogErrMemory(mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_CATALOG as std::os::raw::c_int,
                    XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Memory allocation failed : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, extra);
}
/* *
 * xmlCatalogErr:
 * @catal: the Catalog entry
 * @node: the context node
 * @msg:  the error message
 * @extra:  extra informations
 *
 * Handle a catalog error
 */
unsafe extern "C" fn xmlCatalogErr(mut catal: xmlCatalogEntryPtr,
                                   mut node: xmlNodePtr,
                                   mut error: std::os::raw::c_int,
                                   mut msg: *const std::os::raw::c_char,
                                   mut str1: *const xmlChar,
                                   mut str2: *const xmlChar,
                                   mut str3: *const xmlChar) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    catal as *mut std::os::raw::c_void, node as *mut std::os::raw::c_void,
                    XML_FROM_CATALOG as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    str3 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2, str3);
}
/* ***********************************************************************
 *									*
 *			Allocation and Freeing				*
 *									*
 ************************************************************************/
/* *
 * xmlNewCatalogEntry:
 * @type:  type of entry
 * @name:  name of the entry
 * @value:  value of the entry
 * @prefer:  the PUBLIC vs. SYSTEM current preference value
 * @group:  for members of a group, the group entry
 *
 * create a new Catalog entry, this type is shared both by XML and
 * SGML catalogs, but the acceptable types values differs.
 *
 * Returns the xmlCatalogEntryPtr or NULL in case of error
 */
unsafe extern "C" fn xmlNewCatalogEntry(mut type_0: xmlCatalogEntryType,
                                        mut name: *const xmlChar,
                                        mut value: *const xmlChar,
                                        mut URL: *const xmlChar,
                                        mut prefer: xmlCatalogPrefer,
                                        mut group: xmlCatalogEntryPtr)
 -> xmlCatalogEntryPtr {
    let mut ret: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut normid: *mut xmlChar = 0 as *mut xmlChar;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlCatalogEntry>()
                                                          as std::os::raw::c_ulong) as
            xmlCatalogEntryPtr;
    if ret.is_null() {
        xmlCatalogErrMemory(b"allocating catalog entry\x00" as *const u8 as
                                *const std::os::raw::c_char);
        return 0 as xmlCatalogEntryPtr
    }
    (*ret).next = 0 as *mut _xmlCatalogEntry;
    (*ret).parent = 0 as *mut _xmlCatalogEntry;
    (*ret).children = 0 as *mut _xmlCatalogEntry;
    (*ret).type_0 = type_0;
    if type_0 as std::os::raw::c_int == XML_CATA_PUBLIC as std::os::raw::c_int ||
           type_0 as std::os::raw::c_int == XML_CATA_DELEGATE_PUBLIC as std::os::raw::c_int {
        normid = xmlCatalogNormalizePublic(name);
        if !normid.is_null() {
            name =
                if *normid as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    normid
                } else { 0 as *mut xmlChar }
        }
    }
    if !name.is_null() {
        (*ret).name = xmlStrdup(name)
    } else { (*ret).name = 0 as *mut xmlChar }
    if !normid.is_null() {
        xmlFree.expect("non-null function pointer")(normid as
                                                        *mut std::os::raw::c_void);
    }
    if !value.is_null() {
        (*ret).value = xmlStrdup(value)
    } else { (*ret).value = 0 as *mut xmlChar }
    if URL.is_null() { URL = value }
    if !URL.is_null() {
        (*ret).URL = xmlStrdup(URL)
    } else { (*ret).URL = 0 as *mut xmlChar }
    (*ret).prefer = prefer;
    (*ret).dealloc = 0 as std::os::raw::c_int;
    (*ret).depth = 0 as std::os::raw::c_int;
    (*ret).group = group;
    return ret;
}
/* *
 * xmlFreeCatalogEntry:
 * @payload:  a Catalog entry
 *
 * Free the memory allocated to a Catalog entry
 */
unsafe extern "C" fn xmlFreeCatalogEntry(mut payload: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar) {
    let mut ret: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    if ret.is_null() { return }
    /*
     * Entries stored in the file hash must be deallocated
     * only by the file hash cleaner !
     */
    if (*ret).dealloc == 1 as std::os::raw::c_int { return }
    if xmlDebugCatalogs != 0 {
        if !(*ret).name.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Free catalog entry %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       (*ret).name);
        } else if !(*ret).value.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Free catalog entry %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       (*ret).value);
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Free catalog entry\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
        }
    }
    if !(*ret).name.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).name as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ret).value.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).value as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ret).URL.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).URL as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
}
/* *
 * xmlFreeCatalogEntryList:
 * @ret:  a Catalog entry list
 *
 * Free the memory allocated to a full chained list of Catalog entries
 */
unsafe extern "C" fn xmlFreeCatalogEntryList(mut ret: xmlCatalogEntryPtr) {
    let mut next: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    while !ret.is_null() {
        next = (*ret).next;
        xmlFreeCatalogEntry(ret as *mut std::os::raw::c_void, 0 as *const xmlChar);
        ret = next
    };
}
/* *
 * xmlFreeCatalogHashEntryList:
 * @payload:  a Catalog entry list
 *
 * Free the memory allocated to list of Catalog entries from the
 * catalog file hash.
 */
unsafe extern "C" fn xmlFreeCatalogHashEntryList(mut payload:
                                                     *mut std::os::raw::c_void,
                                                 mut name: *const xmlChar) {
    let mut catal: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    let mut children: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut next: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    if catal.is_null() { return }
    children = (*catal).children;
    while !children.is_null() {
        next = (*children).next;
        (*children).dealloc = 0 as std::os::raw::c_int;
        (*children).children = 0 as *mut _xmlCatalogEntry;
        xmlFreeCatalogEntry(children as *mut std::os::raw::c_void,
                            0 as *const xmlChar);
        children = next
    }
    (*catal).dealloc = 0 as std::os::raw::c_int;
    xmlFreeCatalogEntry(catal as *mut std::os::raw::c_void, 0 as *const xmlChar);
}
/* *
 * xmlCreateNewCatalog:
 * @type:  type of catalog
 * @prefer:  the PUBLIC vs. SYSTEM current preference value
 *
 * create a new Catalog, this type is shared both by XML and
 * SGML catalogs, but the acceptable types values differs.
 *
 * Returns the xmlCatalogPtr or NULL in case of error
 */
unsafe extern "C" fn xmlCreateNewCatalog(mut type_0: xmlCatalogType,
                                         mut prefer: xmlCatalogPrefer)
 -> xmlCatalogPtr {
    let mut ret: xmlCatalogPtr = 0 as *mut xmlCatalog;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlCatalog>()
                                                          as std::os::raw::c_ulong) as
            xmlCatalogPtr;
    if ret.is_null() {
        xmlCatalogErrMemory(b"allocating catalog\x00" as *const u8 as
                                *const std::os::raw::c_char);
        return 0 as xmlCatalogPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlCatalog>() as std::os::raw::c_ulong);
    (*ret).type_0 = type_0;
    (*ret).catalNr = 0 as std::os::raw::c_int;
    (*ret).catalMax = 10 as std::os::raw::c_int;
    (*ret).prefer = prefer;
    if (*ret).type_0 as std::os::raw::c_uint ==
           XML_SGML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        (*ret).sgml = xmlHashCreate(10 as std::os::raw::c_int)
    }
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlFreeCatalog:
 * @catal:  a Catalog
 *
 * Free the memory allocated to a Catalog
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeCatalog(mut catal: xmlCatalogPtr) {
    if catal.is_null() { return }
    if !(*catal).xml.is_null() { xmlFreeCatalogEntryList((*catal).xml); }
    if !(*catal).sgml.is_null() {
        xmlHashFree((*catal).sgml,
                    Some(xmlFreeCatalogEntry as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()));
    }
    xmlFree.expect("non-null function pointer")(catal as *mut std::os::raw::c_void);
}
/* ***********************************************************************
 *									*
 *			Serializing Catalogs				*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogDumpEntry:
 * @entry:  the catalog entry
 * @out:  the file.
 *
 * Serialize an SGML Catalog entry
 */
unsafe extern "C" fn xmlCatalogDumpEntry(mut payload: *mut std::os::raw::c_void,
                                         mut data: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar) {
    let mut entry: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    let mut out: *mut FILE = data as *mut FILE;
    if entry.is_null() || out.is_null() { return }
    match (*entry).type_0 as std::os::raw::c_int {
        15 => {
            fprintf(out, b"ENTITY \x00" as *const u8 as *const std::os::raw::c_char);
        }
        16 => {
            fprintf(out,
                    b"ENTITY %%\x00" as *const u8 as *const std::os::raw::c_char);
        }
        17 => {
            fprintf(out, b"DOCTYPE \x00" as *const u8 as *const std::os::raw::c_char);
        }
        18 => {
            fprintf(out,
                    b"LINKTYPE \x00" as *const u8 as *const std::os::raw::c_char);
        }
        19 => {
            fprintf(out,
                    b"NOTATION \x00" as *const u8 as *const std::os::raw::c_char);
        }
        14 => {
            fprintf(out, b"PUBLIC \x00" as *const u8 as *const std::os::raw::c_char);
        }
        13 => {
            fprintf(out, b"SYSTEM \x00" as *const u8 as *const std::os::raw::c_char);
        }
        20 => {
            fprintf(out,
                    b"DELEGATE \x00" as *const u8 as *const std::os::raw::c_char);
        }
        21 => {
            fprintf(out, b"BASE \x00" as *const u8 as *const std::os::raw::c_char);
        }
        22 => {
            fprintf(out, b"CATALOG \x00" as *const u8 as *const std::os::raw::c_char);
        }
        23 => {
            fprintf(out,
                    b"DOCUMENT \x00" as *const u8 as *const std::os::raw::c_char);
        }
        24 => {
            fprintf(out,
                    b"SGMLDECL \x00" as *const u8 as *const std::os::raw::c_char);
        }
        _ => { return }
    }
    match (*entry).type_0 as std::os::raw::c_int {
        15 | 16 | 17 | 18 | 19 => {
            fprintf(out, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                    (*entry).name as *const std::os::raw::c_char);
        }
        14 | 13 | 24 | 23 | 22 | 21 | 20 => {
            fprintf(out, b"\"%s\"\x00" as *const u8 as *const std::os::raw::c_char,
                    (*entry).name);
        }
        _ => { }
    }
    match (*entry).type_0 as std::os::raw::c_int {
        15 | 16 | 17 | 18 | 19 | 14 | 13 | 20 => {
            fprintf(out, b" \"%s\"\x00" as *const u8 as *const std::os::raw::c_char,
                    (*entry).value);
        }
        _ => { }
    }
    fprintf(out, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlDumpXMLCatalogNode:
 * @catal:  top catalog entry
 * @catalog: pointer to the xml tree
 * @doc: the containing document
 * @ns: the current namespace
 * @cgroup: group node for group members
 *
 * Serializes a Catalog entry, called by xmlDumpXMLCatalog and recursively
 * for group entries
 */
unsafe extern "C" fn xmlDumpXMLCatalogNode(mut catal: xmlCatalogEntryPtr,
                                           mut catalog: xmlNodePtr,
                                           mut doc: xmlDocPtr,
                                           mut ns: xmlNsPtr,
                                           mut cgroup: xmlCatalogEntryPtr) {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    /*
     * add all the catalog entries
     */
    cur = catal;
    let mut current_block_49: u64;
    while !cur.is_null() {
        if (*cur).group == cgroup {
            match (*cur).type_0 as std::os::raw::c_int {
                2 | 1 => {
                    current_block_49 = 1134324000648431478;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                3 => {
                    current_block_49 = 7962216398677578206;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                4 => {
                    current_block_49 = 3113147720494587438;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                5 => {
                    current_block_49 = 6845406576219733746;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                6 => {
                    current_block_49 = 14448122729317246829;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                7 => {
                    current_block_49 = 2797560868220527165;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                8 => {
                    current_block_49 = 3925049384301371317;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                9 => {
                    current_block_49 = 15125366499419599337;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                10 => {
                    current_block_49 = 4386451087681909888;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                11 => {
                    current_block_49 = 1296397779060594135;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                12 => {
                    current_block_49 = 6617409623915549803;
                    match current_block_49 {
                        1134324000648431478 => {
                            if cur == catal {
                                cur = (*cur).children;
                                continue ;
                            }
                        }
                        1296397779060594135 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        4386451087681909888 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"uri\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"name\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        15125366499419599337 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3925049384301371317 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegatePublic\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        2797560868220527165 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"rewriteSystem\x00" as
                                                  *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemIdStartString\x00" as *const u8
                                           as *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"rewritePrefix\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        14448122729317246829 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"system\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"systemId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        6845406576219733746 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"public\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"publicId\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"uri\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        3113147720494587438 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"group\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"id\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = 0 as *mut xmlNs;
                                xns =
                                    xmlSearchNsByHref(doc, node,
                                                      b"http://www.w3.org/XML/1998/namespace\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char
                                                          as *const xmlChar);
                                if !xns.is_null() {
                                    xmlSetNsProp(node, xns,
                                                 b"base\x00" as *const u8 as
                                                     *const std::os::raw::c_char as
                                                     *mut xmlChar,
                                                 (*cur).value);
                                }
                            }
                            match (*cur).prefer as std::os::raw::c_uint {
                                1 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"public\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                2 => {
                                    xmlSetProp(node,
                                               b"prefer\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               b"system\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar);
                                }
                                0 | _ => { }
                            }
                            xmlDumpXMLCatalogNode((*cur).next, node, doc, ns,
                                                  cur);
                            xmlAddChild(catalog, node);
                        }
                        7962216398677578206 => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"nextCatalog\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node =
                                xmlNewDocNode(doc, ns,
                                              b"delegateURI\x00" as *const u8
                                                  as *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            xmlSetProp(node,
                                       b"uriStartString\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).name);
                            xmlSetProp(node,
                                       b"catalog\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*cur).value);
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                -1 | 0 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23
                | 24 | _ => {
                }
            }
        }
        cur = (*cur).next
    };
}
unsafe extern "C" fn xmlDumpXMLCatalog(mut out: *mut FILE,
                                       mut catal: xmlCatalogEntryPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut catalog: xmlNodePtr = 0 as *mut xmlNode;
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    /*
     * Rebuild a catalog
     */
    doc = xmlNewDoc(0 as *const xmlChar);
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    dtd =
        xmlNewDtd(doc,
                  b"catalog\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar,
                  b"-//OASIS//DTD Entity Resolution XML Catalog V1.0//EN\x00"
                      as *const u8 as *const std::os::raw::c_char as *mut xmlChar,
                  b"http://www.oasis-open.org/committees/entity/release/1.0/catalog.dtd\x00"
                      as *const u8 as *const std::os::raw::c_char as *mut xmlChar);
    xmlAddChild(doc as xmlNodePtr, dtd as xmlNodePtr);
    ns =
        xmlNewNs(0 as xmlNodePtr,
                 b"urn:oasis:names:tc:entity:xmlns:xml:catalog\x00" as
                     *const u8 as *const std::os::raw::c_char as *const xmlChar,
                 0 as *const xmlChar);
    if ns.is_null() { xmlFreeDoc(doc); return -(1 as std::os::raw::c_int) }
    catalog =
        xmlNewDocNode(doc, ns,
                      b"catalog\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar, 0 as *const xmlChar);
    if catalog.is_null() {
        xmlFreeNs(ns);
        xmlFreeDoc(doc);
        return -(1 as std::os::raw::c_int)
    }
    (*catalog).nsDef = ns;
    xmlAddChild(doc as xmlNodePtr, catalog);
    xmlDumpXMLCatalogNode(catal, catalog, doc, ns, 0 as xmlCatalogEntryPtr);
    /*
     * reserialize it
     */
    buf = xmlOutputBufferCreateFile(out, 0 as xmlCharEncodingHandlerPtr);
    if buf.is_null() { xmlFreeDoc(doc); return -(1 as std::os::raw::c_int) }
    ret =
        xmlSaveFormatFileTo(buf, doc, 0 as *const std::os::raw::c_char,
                            1 as std::os::raw::c_int);
    /*
     * Free it
     */
    xmlFreeDoc(doc);
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *			Converting SGML Catalogs to XML			*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogConvertEntry:
 * @entry:  the entry
 * @catal:  pointer to the catalog being converted
 *
 * Convert one entry from the catalog
 */
unsafe extern "C" fn xmlCatalogConvertEntry(mut payload: *mut std::os::raw::c_void,
                                            mut data: *mut std::os::raw::c_void,
                                            mut name: *const xmlChar) {
    let mut entry: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    let mut catal: xmlCatalogPtr = data as xmlCatalogPtr;
    if entry.is_null() || catal.is_null() || (*catal).sgml.is_null() ||
           (*catal).xml.is_null() {
        return
    }
    match (*entry).type_0 as std::os::raw::c_int {
        15 => { (*entry).type_0 = XML_CATA_PUBLIC }
        16 => { (*entry).type_0 = XML_CATA_PUBLIC }
        17 => { (*entry).type_0 = XML_CATA_PUBLIC }
        18 => { (*entry).type_0 = XML_CATA_PUBLIC }
        19 => { (*entry).type_0 = XML_CATA_PUBLIC }
        14 => { (*entry).type_0 = XML_CATA_PUBLIC }
        13 => { (*entry).type_0 = XML_CATA_SYSTEM }
        20 => { (*entry).type_0 = XML_CATA_DELEGATE_PUBLIC }
        22 => { (*entry).type_0 = XML_CATA_CATALOG }
        _ => {
            xmlHashRemoveEntry((*catal).sgml, (*entry).name,
                               Some(xmlFreeCatalogEntry as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void,
                                                             _:
                                                                 *const xmlChar)
                                            -> ()));
            return
        }
    }
    /*
     * Conversion successful, remove from the SGML catalog
     * and add it to the default XML one
     */
    xmlHashRemoveEntry((*catal).sgml, (*entry).name, None);
    (*entry).parent = (*catal).xml;
    (*entry).next = 0 as *mut _xmlCatalogEntry;
    if (*(*catal).xml).children.is_null() {
        (*(*catal).xml).children = entry
    } else {
        let mut prev: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
        prev = (*(*catal).xml).children;
        while !(*prev).next.is_null() { prev = (*prev).next }
        (*prev).next = entry
    };
}
/* *
 * xmlConvertSGMLCatalog:
 * @catal: the catalog
 *
 * Convert all the SGML catalog entries as XML ones
 *
 * Returns the number of entries converted if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlConvertSGMLCatalog(mut catal: xmlCatalogPtr)
 -> std::os::raw::c_int {
    if catal.is_null() ||
           (*catal).type_0 as std::os::raw::c_uint !=
               XML_SGML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Converting SGML catalog to XML\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
    }
    xmlHashScan((*catal).sgml,
                Some(xmlCatalogConvertEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()),
                &mut catal as *mut xmlCatalogPtr as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *			Helper function					*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogUnWrapURN:
 * @urn:  an "urn:publicid:" to unwrap
 *
 * Expand the URN into the equivalent Public Identifier
 *
 * Returns the new identifier or NULL, the string must be deallocated
 *         by the caller.
 */
unsafe extern "C" fn xmlCatalogUnWrapURN(mut urn: *const xmlChar)
 -> *mut xmlChar {
    let mut result: [xmlChar; 2000] = [0; 2000];
    let mut i: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if xmlStrncmp(urn,
                  b"urn:publicid:\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar,
                  (::std::mem::size_of::<[std::os::raw::c_char; 14]>() as
                       std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) as
                      std::os::raw::c_int) != 0 {
        return 0 as *mut xmlChar
    }
    urn =
        urn.offset((::std::mem::size_of::<[std::os::raw::c_char; 14]>() as
                        std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                       isize);
    while *urn as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if i as std::os::raw::c_ulong >
               (::std::mem::size_of::<[xmlChar; 2000]>() as
                    std::os::raw::c_ulong).wrapping_sub(4 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong) {
            break ;
        }
        if *urn as std::os::raw::c_int == '+' as i32 {
            let fresh0 = i;
            i = i.wrapping_add(1);
            result[fresh0 as usize] = ' ' as i32 as xmlChar;
            urn = urn.offset(1)
        } else if *urn as std::os::raw::c_int == ':' as i32 {
            let fresh1 = i;
            i = i.wrapping_add(1);
            result[fresh1 as usize] = '/' as i32 as xmlChar;
            let fresh2 = i;
            i = i.wrapping_add(1);
            result[fresh2 as usize] = '/' as i32 as xmlChar;
            urn = urn.offset(1)
        } else if *urn as std::os::raw::c_int == ';' as i32 {
            let fresh3 = i;
            i = i.wrapping_add(1);
            result[fresh3 as usize] = ':' as i32 as xmlChar;
            let fresh4 = i;
            i = i.wrapping_add(1);
            result[fresh4 as usize] = ':' as i32 as xmlChar;
            urn = urn.offset(1)
        } else if *urn as std::os::raw::c_int == '%' as i32 {
            if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '2' as i32 &&
                   *urn.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'B' as i32 {
                let fresh5 = i;
                i = i.wrapping_add(1);
                result[fresh5 as usize] = '+' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '3' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'A' as i32 {
                let fresh6 = i;
                i = i.wrapping_add(1);
                result[fresh6 as usize] = ':' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '2' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'F' as i32 {
                let fresh7 = i;
                i = i.wrapping_add(1);
                result[fresh7 as usize] = '/' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '3' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'B' as i32 {
                let fresh8 = i;
                i = i.wrapping_add(1);
                result[fresh8 as usize] = ';' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '2' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == '7' as i32 {
                let fresh9 = i;
                i = i.wrapping_add(1);
                result[fresh9 as usize] = '\'' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '3' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'F' as i32 {
                let fresh10 = i;
                i = i.wrapping_add(1);
                result[fresh10 as usize] = '?' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '2' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == '3' as i32 {
                let fresh11 = i;
                i = i.wrapping_add(1);
                result[fresh11 as usize] = '#' as i32 as xmlChar
            } else if *urn.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                          '2' as i32 &&
                          *urn.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == '5' as i32 {
                let fresh12 = i;
                i = i.wrapping_add(1);
                result[fresh12 as usize] = '%' as i32 as xmlChar
            } else {
                let fresh13 = i;
                i = i.wrapping_add(1);
                result[fresh13 as usize] = *urn;
                urn = urn.offset(1);
                continue ;
            }
            urn = urn.offset(3 as std::os::raw::c_int as isize)
        } else {
            let fresh14 = i;
            i = i.wrapping_add(1);
            result[fresh14 as usize] = *urn;
            urn = urn.offset(1)
        }
    }
    result[i as usize] = 0 as std::os::raw::c_int as xmlChar;
    return xmlStrdup(result.as_mut_ptr());
}
/* *
 * xmlParseCatalogFile:
 * @filename:  the filename
 *
 * parse an XML file and build a tree. It's like xmlParseFile()
 * except it bypass all catalog lookups.
 *
 * Returns the resulting document tree or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParseCatalogFile(mut filename:
                                                 *const std::os::raw::c_char)
 -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut directory: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        if (*__xmlDefaultSAXHandler()).error.is_some() {
            (*__xmlDefaultSAXHandler()).error.expect("non-null function pointer")(0
                                                                                      as
                                                                                      *mut std::os::raw::c_void,
                                                                                  b"out of memory\n\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const std::os::raw::c_char);
        }
        return 0 as xmlDocPtr
    }
    buf =
        xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if buf.is_null() { xmlFreeParserCtxt(ctxt); return 0 as xmlDocPtr }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt(ctxt);
        return 0 as xmlDocPtr
    }
    (*inputStream).filename =
        xmlCanonicPath(filename as *const xmlChar) as *mut std::os::raw::c_char;
    (*inputStream).buf = buf;
    xmlBufResetInput((*buf).buffer, inputStream);
    inputPush(ctxt, inputStream);
    if (*ctxt).directory.is_null() && directory.is_null() {
        directory = xmlParserGetDirectory(filename)
    }
    if (*ctxt).directory.is_null() && !directory.is_null() {
        (*ctxt).directory = directory
    }
    (*ctxt).valid = 0 as std::os::raw::c_int;
    (*ctxt).validate = 0 as std::os::raw::c_int;
    (*ctxt).loadsubset = 0 as std::os::raw::c_int;
    (*ctxt).pedantic = 0 as std::os::raw::c_int;
    (*ctxt).dictNames = 1 as std::os::raw::c_int;
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 {
        ret = (*ctxt).myDoc
    } else {
        ret = 0 as xmlDocPtr;
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = 0 as xmlDocPtr
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
/* *
 * xmlLoadFileContent:
 * @filename:  a file path
 *
 * Load a file content into memory.
 *
 * Returns a pointer to the 0 terminated string or NULL in case of error
 */
unsafe extern "C" fn xmlLoadFileContent(mut filename: *const std::os::raw::c_char)
 -> *mut xmlChar {
    let mut fd: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_long = 0;
    let mut info: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut content: *mut xmlChar = 0 as *mut xmlChar;
    if filename.is_null() { return 0 as *mut xmlChar }
    if stat(filename, &mut info) < 0 as std::os::raw::c_int {
        return 0 as *mut xmlChar
    }
    fd = open(filename, 0 as std::os::raw::c_int);
    if fd < 0 as std::os::raw::c_int { return 0 as *mut xmlChar }
    size = info.st_size;
    content =
        xmlMallocAtomic.expect("non-null function pointer")((size +
                                                                 10 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     std::os::raw::c_long)
                                                                as size_t) as
            *mut xmlChar;
    if content.is_null() {
        xmlCatalogErrMemory(b"allocating catalog data\x00" as *const u8 as
                                *const std::os::raw::c_char);
        close(fd);
        return 0 as *mut xmlChar
    }
    len =
        read(fd, content as *mut std::os::raw::c_void, size as size_t) as std::os::raw::c_int;
    close(fd);
    if len < 0 as std::os::raw::c_int {
        xmlFree.expect("non-null function pointer")(content as
                                                        *mut std::os::raw::c_void);
        return 0 as *mut xmlChar
    }
    *content.offset(len as isize) = 0 as std::os::raw::c_int as xmlChar;
    return content;
}
/* *
 * xmlCatalogNormalizePublic:
 * @pubID:  the public ID string
 *
 *  Normalizes the Public Identifier
 *
 * Implements 6.2. Public Identifier Normalization
 * from http://www.oasis-open.org/committees/entity/spec-2001-08-06.html
 *
 * Returns the new string or NULL, the string must be deallocated
 *         by the caller.
 */
unsafe extern "C" fn xmlCatalogNormalizePublic(mut pubID: *const xmlChar)
 -> *mut xmlChar {
    let mut ok: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut white: std::os::raw::c_int = 0;
    let mut p: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut q: *mut xmlChar = 0 as *mut xmlChar;
    if pubID.is_null() { return 0 as *mut xmlChar }
    white = 1 as std::os::raw::c_int;
    p = pubID;
    while *p as std::os::raw::c_int != 0 as std::os::raw::c_int && ok != 0 {
        if !(*p as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                 0x9 as std::os::raw::c_int <= *p as std::os::raw::c_int &&
                     *p as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                 *p as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            white = 0 as std::os::raw::c_int
        } else if *p as std::os::raw::c_int == 0x20 as std::os::raw::c_int && white == 0 {
            white = 1 as std::os::raw::c_int
        } else { ok = 0 as std::os::raw::c_int }
        p = p.offset(1)
    }
    if ok != 0 && white == 0 {
        /* is normalized */
        return 0 as *mut xmlChar
    }
    ret = xmlStrdup(pubID);
    q = ret;
    white = 0 as std::os::raw::c_int;
    p = pubID;
    while *p as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *p as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *p as std::os::raw::c_int &&
                   *p as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *p as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if q != ret { white = 1 as std::os::raw::c_int }
        } else {
            if white != 0 {
                let fresh15 = q;
                q = q.offset(1);
                *fresh15 = 0x20 as std::os::raw::c_int as xmlChar;
                white = 0 as std::os::raw::c_int
            }
            let fresh16 = q;
            q = q.offset(1);
            *fresh16 = *p
        }
        p = p.offset(1)
    }
    *q = 0 as std::os::raw::c_int as xmlChar;
    return ret;
}
/* *
 * xmlGetXMLCatalogEntryType:
 * @name:  the name
 *
 * lookup the internal type associated to an XML catalog entry name
 *
 * Returns the type associated with that name
 */
unsafe extern "C" fn xmlGetXMLCatalogEntryType(mut name: *const xmlChar)
 -> xmlCatalogEntryType {
    let mut type_0: xmlCatalogEntryType = XML_CATA_NONE;
    if xmlStrEqual(name,
                   b"system\x00" as *const u8 as *const std::os::raw::c_char as
                       *const xmlChar) != 0 {
        type_0 = XML_CATA_SYSTEM
    } else if xmlStrEqual(name,
                          b"public\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar) != 0 {
        type_0 = XML_CATA_PUBLIC
    } else if xmlStrEqual(name,
                          b"rewriteSystem\x00" as *const u8 as
                              *const std::os::raw::c_char as *const xmlChar) != 0 {
        type_0 = XML_CATA_REWRITE_SYSTEM
    } else if xmlStrEqual(name,
                          b"delegatePublic\x00" as *const u8 as
                              *const std::os::raw::c_char as *const xmlChar) != 0 {
        type_0 = XML_CATA_DELEGATE_PUBLIC
    } else if xmlStrEqual(name,
                          b"delegateSystem\x00" as *const u8 as
                              *const std::os::raw::c_char as *const xmlChar) != 0 {
        type_0 = XML_CATA_DELEGATE_SYSTEM
    } else if xmlStrEqual(name,
                          b"uri\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar) != 0 {
        type_0 = XML_CATA_URI
    } else if xmlStrEqual(name,
                          b"rewriteURI\x00" as *const u8 as
                              *const std::os::raw::c_char as *const xmlChar) != 0 {
        type_0 = XML_CATA_REWRITE_URI
    } else if xmlStrEqual(name,
                          b"delegateURI\x00" as *const u8 as
                              *const std::os::raw::c_char as *const xmlChar) != 0 {
        type_0 = XML_CATA_DELEGATE_URI
    } else if xmlStrEqual(name,
                          b"nextCatalog\x00" as *const u8 as
                              *const std::os::raw::c_char as *const xmlChar) != 0 {
        type_0 = XML_CATA_NEXT_CATALOG
    } else if xmlStrEqual(name,
                          b"catalog\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = XML_CATA_CATALOG
    }
    return type_0;
}
/* *
 * xmlParseXMLCatalogOneNode:
 * @cur:  the XML node
 * @type:  the type of Catalog entry
 * @name:  the name of the node
 * @attrName:  the attribute holding the value
 * @uriAttrName:  the attribute holding the URI-Reference
 * @prefer:  the PUBLIC vs. SYSTEM current preference value
 * @cgroup:  the group which includes this node
 *
 * Finishes the examination of an XML tree node of a catalog and build
 * a Catalog entry from it.
 *
 * Returns the new Catalog entry node or NULL in case of error.
 */
unsafe extern "C" fn xmlParseXMLCatalogOneNode(mut cur: xmlNodePtr,
                                               mut type_0:
                                                   xmlCatalogEntryType,
                                               mut name: *const xmlChar,
                                               mut attrName: *const xmlChar,
                                               mut uriAttrName:
                                                   *const xmlChar,
                                               mut prefer: xmlCatalogPrefer,
                                               mut cgroup: xmlCatalogEntryPtr)
 -> xmlCatalogEntryPtr {
    let mut ok: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut uriValue: *mut xmlChar = 0 as *mut xmlChar;
    let mut nameValue: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: xmlCatalogEntryPtr = 0 as xmlCatalogEntryPtr;
    if !attrName.is_null() {
        nameValue = xmlGetProp(cur as *const xmlNode, attrName);
        if nameValue.is_null() {
            xmlCatalogErr(ret, cur, XML_CATALOG_MISSING_ATTR as std::os::raw::c_int,
                          b"%s entry lacks \'%s\'\n\x00" as *const u8 as
                              *const std::os::raw::c_char, name, attrName,
                          0 as *const xmlChar);
            ok = 0 as std::os::raw::c_int
        }
    }
    uriValue = xmlGetProp(cur as *const xmlNode, uriAttrName);
    if uriValue.is_null() {
        xmlCatalogErr(ret, cur, XML_CATALOG_MISSING_ATTR as std::os::raw::c_int,
                      b"%s entry lacks \'%s\'\n\x00" as *const u8 as
                          *const std::os::raw::c_char, name, uriAttrName,
                      0 as *const xmlChar);
        ok = 0 as std::os::raw::c_int
    }
    if ok == 0 {
        if !nameValue.is_null() {
            xmlFree.expect("non-null function pointer")(nameValue as
                                                            *mut std::os::raw::c_void);
        }
        if !uriValue.is_null() {
            xmlFree.expect("non-null function pointer")(uriValue as
                                                            *mut std::os::raw::c_void);
        }
        return 0 as xmlCatalogEntryPtr
    }
    base = xmlNodeGetBase((*cur).doc, cur as *const xmlNode);
    URL = xmlBuildURI(uriValue, base);
    if !URL.is_null() {
        if xmlDebugCatalogs > 1 as std::os::raw::c_int {
            if !nameValue.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Found %s: \'%s\' \'%s\'\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           name,
                                                                           nameValue,
                                                                           URL);
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Found %s: \'%s\'\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           name,
                                                                           URL);
            }
        }
        ret =
            xmlNewCatalogEntry(type_0, nameValue, uriValue, URL, prefer,
                               cgroup)
    } else {
        xmlCatalogErr(ret, cur, XML_CATALOG_ENTRY_BROKEN as std::os::raw::c_int,
                      b"%s entry \'%s\' broken ?: %s\n\x00" as *const u8 as
                          *const std::os::raw::c_char, name, uriAttrName, uriValue);
    }
    if !nameValue.is_null() {
        xmlFree.expect("non-null function pointer")(nameValue as
                                                        *mut std::os::raw::c_void);
    }
    if !uriValue.is_null() {
        xmlFree.expect("non-null function pointer")(uriValue as
                                                        *mut std::os::raw::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as
                                                        *mut std::os::raw::c_void);
    }
    if !URL.is_null() {
        xmlFree.expect("non-null function pointer")(URL as *mut std::os::raw::c_void);
    }
    return ret;
}
/* *
 * xmlParseXMLCatalogNode:
 * @cur:  the XML node
 * @prefer:  the PUBLIC vs. SYSTEM current preference value
 * @parent:  the parent Catalog entry
 * @cgroup:  the group which includes this node
 *
 * Examines an XML tree node of a catalog and build
 * a Catalog entry from it adding it to its parent. The examination can
 * be recursive.
 */
unsafe extern "C" fn xmlParseXMLCatalogNode(mut cur: xmlNodePtr,
                                            mut prefer: xmlCatalogPrefer,
                                            mut parent: xmlCatalogEntryPtr,
                                            mut cgroup: xmlCatalogEntryPtr) {
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut entry: xmlCatalogEntryPtr = 0 as xmlCatalogEntryPtr;
    if cur.is_null() { return }
    if xmlStrEqual((*cur).name,
                   b"group\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        let mut prop: *mut xmlChar = 0 as *mut xmlChar;
        let mut pref: xmlCatalogPrefer = XML_CATA_PREFER_NONE;
        prop =
            xmlGetProp(cur as *const xmlNode,
                       b"prefer\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        if !prop.is_null() {
            if xmlStrEqual(prop,
                           b"system\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 {
                prefer = XML_CATA_PREFER_SYSTEM
            } else if xmlStrEqual(prop,
                                  b"public\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                prefer = XML_CATA_PREFER_PUBLIC
            } else {
                xmlCatalogErr(parent, cur,
                              XML_CATALOG_PREFER_VALUE as std::os::raw::c_int,
                              b"Invalid value for prefer: \'%s\'\n\x00" as
                                  *const u8 as *const std::os::raw::c_char, prop,
                              0 as *const xmlChar, 0 as *const xmlChar);
            }
            xmlFree.expect("non-null function pointer")(prop as
                                                            *mut std::os::raw::c_void);
            pref = prefer
        }
        prop =
            xmlGetProp(cur as *const xmlNode,
                       b"id\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        base =
            xmlGetNsProp(cur as *const xmlNode,
                         b"base\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"http://www.w3.org/XML/1998/namespace\x00" as
                             *const u8 as *const std::os::raw::c_char as
                             *const xmlChar);
        entry =
            xmlNewCatalogEntry(XML_CATA_GROUP, prop, base,
                               0 as *const xmlChar, pref, cgroup);
        xmlFree.expect("non-null function pointer")(prop as
                                                        *mut std::os::raw::c_void);
    } else if xmlStrEqual((*cur).name,
                          b"public\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_PUBLIC,
                                      b"public\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"publicId\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"uri\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"system\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_SYSTEM,
                                      b"system\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"systemId\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"uri\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"rewriteSystem\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_REWRITE_SYSTEM,
                                      b"rewriteSystem\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"systemIdStartString\x00" as *const u8
                                          as *const std::os::raw::c_char as
                                          *mut xmlChar,
                                      b"rewritePrefix\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"delegatePublic\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_DELEGATE_PUBLIC,
                                      b"delegatePublic\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"publicIdStartString\x00" as *const u8
                                          as *const std::os::raw::c_char as
                                          *mut xmlChar,
                                      b"catalog\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"delegateSystem\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_DELEGATE_SYSTEM,
                                      b"delegateSystem\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"systemIdStartString\x00" as *const u8
                                          as *const std::os::raw::c_char as
                                          *mut xmlChar,
                                      b"catalog\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"uri\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_URI,
                                      b"uri\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"name\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"uri\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"rewriteURI\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_REWRITE_URI,
                                      b"rewriteURI\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"uriStartString\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"rewritePrefix\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"delegateURI\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_DELEGATE_URI,
                                      b"delegateURI\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"uriStartString\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      b"catalog\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    } else if xmlStrEqual((*cur).name,
                          b"nextCatalog\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        entry =
            xmlParseXMLCatalogOneNode(cur, XML_CATA_NEXT_CATALOG,
                                      b"nextCatalog\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      0 as *const xmlChar,
                                      b"catalog\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar,
                                      prefer, cgroup)
    }
    if !entry.is_null() {
        if !parent.is_null() {
            (*entry).parent = parent;
            if (*parent).children.is_null() {
                (*parent).children = entry
            } else {
                let mut prev: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
                prev = (*parent).children;
                while !(*prev).next.is_null() { prev = (*prev).next }
                (*prev).next = entry
            }
        }
        if (*entry).type_0 as std::os::raw::c_int == XML_CATA_GROUP as std::os::raw::c_int {
            /*
	     * Recurse to propagate prefer to the subtree
	     * (xml:base handling is automated)
	     */
            xmlParseXMLCatalogNodeList((*cur).children, prefer, parent,
                                       entry);
        }
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlParseXMLCatalogNodeList:
 * @cur:  the XML node list of siblings
 * @prefer:  the PUBLIC vs. SYSTEM current preference value
 * @parent:  the parent Catalog entry
 * @cgroup:  the group which includes this list
 *
 * Examines a list of XML sibling nodes of a catalog and build
 * a list of Catalog entry from it adding it to the parent.
 * The examination will recurse to examine node subtrees.
 */
unsafe extern "C" fn xmlParseXMLCatalogNodeList(mut cur: xmlNodePtr,
                                                mut prefer: xmlCatalogPrefer,
                                                mut parent:
                                                    xmlCatalogEntryPtr,
                                                mut cgroup:
                                                    xmlCatalogEntryPtr) {
    while !cur.is_null() {
        if !(*cur).ns.is_null() && !(*(*cur).ns).href.is_null() &&
               xmlStrEqual((*(*cur).ns).href,
                           b"urn:oasis:names:tc:entity:xmlns:xml:catalog\x00"
                               as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar) != 0 {
            xmlParseXMLCatalogNode(cur, prefer, parent, cgroup);
        }
        cur = (*cur).next
    };
    /* TODO: sort the list according to REWRITE lengths and prefer value */
}
/* ***********************************************************************
 *									*
 *			The XML Catalog parser				*
 *									*
 ************************************************************************/
/* *
 * xmlParseXMLCatalogFile:
 * @prefer:  the PUBLIC vs. SYSTEM current preference value
 * @filename:  the filename for the catalog
 *
 * Parses the catalog file to extract the XML tree and then analyze the
 * tree to build a list of Catalog entries corresponding to this catalog
 *
 * Returns the resulting Catalog entries list
 */
unsafe extern "C" fn xmlParseXMLCatalogFile(mut prefer: xmlCatalogPrefer,
                                            mut filename: *const xmlChar)
 -> xmlCatalogEntryPtr {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    let mut parent: xmlCatalogEntryPtr = 0 as xmlCatalogEntryPtr;
    if filename.is_null() { return 0 as xmlCatalogEntryPtr }
    doc = xmlParseCatalogFile(filename as *const std::os::raw::c_char);
    if doc.is_null() {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Failed to parse catalog %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       filename);
        }
        return 0 as xmlCatalogEntryPtr
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"%d Parsing catalog %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   xmlGetThreadId(),
                                                                   filename);
    }
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if !cur.is_null() &&
           xmlStrEqual((*cur).name,
                       b"catalog\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 && !(*cur).ns.is_null() &&
           !(*(*cur).ns).href.is_null() &&
           xmlStrEqual((*(*cur).ns).href,
                       b"urn:oasis:names:tc:entity:xmlns:xml:catalog\x00" as
                           *const u8 as *const std::os::raw::c_char as *const xmlChar)
               != 0 {
        parent =
            xmlNewCatalogEntry(XML_CATA_CATALOG, 0 as *const xmlChar,
                               filename, 0 as *const xmlChar, prefer,
                               0 as xmlCatalogEntryPtr);
        if parent.is_null() {
            xmlFreeDoc(doc);
            return 0 as xmlCatalogEntryPtr
        }
        prop =
            xmlGetProp(cur as *const xmlNode,
                       b"prefer\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        if !prop.is_null() {
            if xmlStrEqual(prop,
                           b"system\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 {
                prefer = XML_CATA_PREFER_SYSTEM
            } else if xmlStrEqual(prop,
                                  b"public\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                prefer = XML_CATA_PREFER_PUBLIC
            } else {
                xmlCatalogErr(0 as xmlCatalogEntryPtr, cur,
                              XML_CATALOG_PREFER_VALUE as std::os::raw::c_int,
                              b"Invalid value for prefer: \'%s\'\n\x00" as
                                  *const u8 as *const std::os::raw::c_char, prop,
                              0 as *const xmlChar, 0 as *const xmlChar);
            }
            xmlFree.expect("non-null function pointer")(prop as
                                                            *mut std::os::raw::c_void);
        }
        cur = (*cur).children;
        xmlParseXMLCatalogNodeList(cur, prefer, parent,
                                   0 as xmlCatalogEntryPtr);
    } else {
        xmlCatalogErr(0 as xmlCatalogEntryPtr, doc as xmlNodePtr,
                      XML_CATALOG_NOT_CATALOG as std::os::raw::c_int,
                      b"File %s is not an XML Catalog\n\x00" as *const u8 as
                          *const std::os::raw::c_char, filename, 0 as *const xmlChar,
                      0 as *const xmlChar);
        xmlFreeDoc(doc);
        return 0 as xmlCatalogEntryPtr
    }
    xmlFreeDoc(doc);
    return parent;
}
/* *
 * xmlFetchXMLCatalogFile:
 * @catal:  an existing but incomplete catalog entry
 *
 * Fetch and parse the subcatalog referenced by an entry
 *
 * Returns 0 in case of success, -1 otherwise
 */
unsafe extern "C" fn xmlFetchXMLCatalogFile(mut catal: xmlCatalogEntryPtr)
 -> std::os::raw::c_int {
    let mut doc: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    if catal.is_null() { return -(1 as std::os::raw::c_int) }
    if (*catal).URL.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * lock the whole catalog for modification
     */
    xmlRMutexLock(xmlCatalogMutex);
    if !(*catal).children.is_null() {
        /* Okay someone else did it in the meantime */
        xmlRMutexUnlock(xmlCatalogMutex);
        return 0 as std::os::raw::c_int
    }
    if !xmlCatalogXMLFiles.is_null() {
        doc =
            xmlHashLookup(xmlCatalogXMLFiles, (*catal).URL) as
                xmlCatalogEntryPtr;
        if !doc.is_null() {
            if xmlDebugCatalogs != 0 {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Found %s in file hash\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           (*catal).URL);
            }
            if (*catal).type_0 as std::os::raw::c_int ==
                   XML_CATA_CATALOG as std::os::raw::c_int {
                (*catal).children = (*doc).children
            } else { (*catal).children = doc }
            (*catal).dealloc = 0 as std::os::raw::c_int;
            xmlRMutexUnlock(xmlCatalogMutex);
            return 0 as std::os::raw::c_int
        }
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s not found in file hash\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       (*catal).URL);
        }
    }
    /*
     * Fetch and parse. Note that xmlParseXMLCatalogFile does not
     * use the existing catalog, there is no recursion allowed at
     * that level.
     */
    doc = xmlParseXMLCatalogFile((*catal).prefer, (*catal).URL);
    if doc.is_null() {
        (*catal).type_0 = XML_CATA_BROKEN_CATALOG;
        xmlRMutexUnlock(xmlCatalogMutex);
        return -(1 as std::os::raw::c_int)
    }
    if (*catal).type_0 as std::os::raw::c_int == XML_CATA_CATALOG as std::os::raw::c_int {
        (*catal).children = (*doc).children
    } else { (*catal).children = doc }
    (*doc).dealloc = 1 as std::os::raw::c_int;
    if xmlCatalogXMLFiles.is_null() {
        xmlCatalogXMLFiles = xmlHashCreate(10 as std::os::raw::c_int)
    }
    if !xmlCatalogXMLFiles.is_null() {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"%s added to file hash\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       (*catal).URL);
        }
        xmlHashAddEntry(xmlCatalogXMLFiles, (*catal).URL,
                        doc as *mut std::os::raw::c_void);
    }
    xmlRMutexUnlock(xmlCatalogMutex);
    return 0 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *			XML Catalog handling				*
 *									*
 ************************************************************************/
/* *
 * xmlAddXMLCatalog:
 * @catal:  top of an XML catalog
 * @type:  the type of record to add to the catalog
 * @orig:  the system, public or prefix to match (or NULL)
 * @replace:  the replacement value for the match
 *
 * Add an entry in the XML catalog, it may overwrite existing but
 * different entries.
 *
 * Returns 0 if successful, -1 otherwise
 */
unsafe extern "C" fn xmlAddXMLCatalog(mut catal: xmlCatalogEntryPtr,
                                      mut type_0: *const xmlChar,
                                      mut orig: *const xmlChar,
                                      mut replace: *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut typ: xmlCatalogEntryType = XML_CATA_NONE;
    let mut doregister: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if catal.is_null() ||
           (*catal).type_0 as std::os::raw::c_int != XML_CATA_CATALOG as std::os::raw::c_int
               &&
               (*catal).type_0 as std::os::raw::c_int !=
                   XML_CATA_BROKEN_CATALOG as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if (*catal).children.is_null() { xmlFetchXMLCatalogFile(catal); }
    if (*catal).children.is_null() { doregister = 1 as std::os::raw::c_int }
    typ = xmlGetXMLCatalogEntryType(type_0);
    if typ as std::os::raw::c_int == XML_CATA_NONE as std::os::raw::c_int {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Failed to add unknown element %s to catalog\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       type_0);
        }
        return -(1 as std::os::raw::c_int)
    }
    cur = (*catal).children;
    /*
     * Might be a simple "update in place"
     */
    if !cur.is_null() {
        while !cur.is_null() {
            if !orig.is_null() &&
                   (*cur).type_0 as std::os::raw::c_int == typ as std::os::raw::c_int &&
                   xmlStrEqual(orig, (*cur).name) != 0 {
                if xmlDebugCatalogs != 0 {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Updating element %s to catalog\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               type_0);
                }
                if !(*cur).value.is_null() {
                    xmlFree.expect("non-null function pointer")((*cur).value
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
                if !(*cur).URL.is_null() {
                    xmlFree.expect("non-null function pointer")((*cur).URL as
                                                                    *mut std::os::raw::c_void);
                }
                (*cur).value = xmlStrdup(replace);
                (*cur).URL = xmlStrdup(replace);
                return 0 as std::os::raw::c_int
            }
            if (*cur).next.is_null() { break ; }
            cur = (*cur).next
        }
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Adding element %s to catalog\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   type_0);
    }
    if cur.is_null() {
        (*catal).children =
            xmlNewCatalogEntry(typ, orig, replace, 0 as *const xmlChar,
                               (*catal).prefer, 0 as xmlCatalogEntryPtr)
    } else {
        (*cur).next =
            xmlNewCatalogEntry(typ, orig, replace, 0 as *const xmlChar,
                               (*catal).prefer, 0 as xmlCatalogEntryPtr)
    }
    if doregister != 0 {
        (*catal).type_0 = XML_CATA_CATALOG;
        cur =
            xmlHashLookup(xmlCatalogXMLFiles, (*catal).URL) as
                xmlCatalogEntryPtr;
        if !cur.is_null() { (*cur).children = (*catal).children }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlDelXMLCatalog:
 * @catal:  top of an XML catalog
 * @value:  the value to remove from the catalog
 *
 * Remove entries in the XML catalog where the value or the URI
 * is equal to @value
 *
 * Returns the number of entries removed if successful, -1 otherwise
 */
unsafe extern "C" fn xmlDelXMLCatalog(mut catal: xmlCatalogEntryPtr,
                                      mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if catal.is_null() ||
           (*catal).type_0 as std::os::raw::c_int != XML_CATA_CATALOG as std::os::raw::c_int
               &&
               (*catal).type_0 as std::os::raw::c_int !=
                   XML_CATA_BROKEN_CATALOG as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    if (*catal).children.is_null() { xmlFetchXMLCatalogFile(catal); }
    /*
     * Scan the children
     */
    cur = (*catal).children;
    while !cur.is_null() {
        if !(*cur).name.is_null() && xmlStrEqual(value, (*cur).name) != 0 ||
               xmlStrEqual(value, (*cur).value) != 0 {
            if xmlDebugCatalogs != 0 {
                if !(*cur).name.is_null() {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Removing element %s from catalog\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               (*cur).name);
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                               b"Removing element %s from catalog\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               (*cur).value);
                }
            }
            (*cur).type_0 = XML_CATA_REMOVED
        }
        cur = (*cur).next
    }
    return ret;
}
/* *
 * xmlCatalogXMLResolve:
 * @catal:  a catalog list
 * @pubID:  the public ID string
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier for a
 * list of catalog entries.
 *
 * Implements (or tries to) 7.1. External Identifier Resolution
 * from http://www.oasis-open.org/committees/entity/spec-2001-08-06.html
 *
 * Returns the URI of the resource or NULL if not found
 */
unsafe extern "C" fn xmlCatalogXMLResolve(mut catal: xmlCatalogEntryPtr,
                                          mut pubID: *const xmlChar,
                                          mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut haveDelegate: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut haveNext: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
     * protection against loops
     */
    if (*catal).depth > 50 as std::os::raw::c_int {
        xmlCatalogErr(catal, 0 as xmlNodePtr,
                      XML_CATALOG_RECURSION as std::os::raw::c_int,
                      b"Detected recursion in catalog %s\n\x00" as *const u8
                          as *const std::os::raw::c_char, (*catal).name,
                      0 as *const xmlChar, 0 as *const xmlChar);
        return 0 as *mut xmlChar
    }
    (*catal).depth += 1;
    /*
     * First tries steps 2/ 3/ 4/ if a system ID is provided.
     */
    if !sysID.is_null() {
        let mut rewrite: xmlCatalogEntryPtr = 0 as xmlCatalogEntryPtr;
        let mut lenrewrite: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut len: std::os::raw::c_int = 0;
        cur = catal;
        haveDelegate = 0 as std::os::raw::c_int;
        while !cur.is_null() {
            match (*cur).type_0 as std::os::raw::c_int {
                6 => {
                    if xmlStrEqual(sysID, (*cur).name) != 0 {
                        if xmlDebugCatalogs != 0 {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"Found system match %s, using %s\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       (*cur).name,
                                                                                       (*cur).URL);
                        }
                        (*catal).depth -= 1;
                        return xmlStrdup((*cur).URL)
                    }
                }
                7 => {
                    len = xmlStrlen((*cur).name);
                    if len > lenrewrite &&
                           xmlStrncmp(sysID, (*cur).name, len) == 0 {
                        lenrewrite = len;
                        rewrite = cur
                    }
                }
                9 => {
                    if xmlStrncmp(sysID, (*cur).name, xmlStrlen((*cur).name))
                           == 0 {
                        haveDelegate += 1
                    }
                }
                3 => { haveNext += 1 }
                _ => { }
            }
            cur = (*cur).next
        }
        if !rewrite.is_null() {
            if xmlDebugCatalogs != 0 {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Using rewriting rule %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           (*rewrite).name);
            }
            ret = xmlStrdup((*rewrite).URL);
            if !ret.is_null() {
                ret = xmlStrcat(ret, &*sysID.offset(lenrewrite as isize))
            }
            (*catal).depth -= 1;
            return ret
        }
        if haveDelegate != 0 {
            let mut delegates: [*const xmlChar; 50] =
                [0 as *const xmlChar; 50];
            let mut nbList: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut i: std::os::raw::c_int = 0;
            /*
	     * Assume the entries have been sorted by decreasing substring
	     * matches when the list was produced.
	     */
            cur = catal;
            while !cur.is_null() {
                if (*cur).type_0 as std::os::raw::c_int ==
                       XML_CATA_DELEGATE_SYSTEM as std::os::raw::c_int &&
                       xmlStrncmp(sysID, (*cur).name, xmlStrlen((*cur).name))
                           == 0 {
                    i = 0 as std::os::raw::c_int;
                    while i < nbList {
                        if xmlStrEqual((*cur).URL, delegates[i as usize]) != 0
                           {
                            break ;
                        }
                        i += 1
                    }
                    if i < nbList {
                        cur = (*cur).next;
                        continue ;
                    } else {
                        if nbList < 50 as std::os::raw::c_int {
                            let fresh17 = nbList;
                            nbList = nbList + 1;
                            delegates[fresh17 as usize] = (*cur).URL
                        }
                        if (*cur).children.is_null() {
                            xmlFetchXMLCatalogFile(cur);
                        }
                        if !(*cur).children.is_null() {
                            if xmlDebugCatalogs != 0 {
                                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                           b"Trying system delegate %s\n\x00"
                                                                                               as
                                                                                               *const u8
                                                                                               as
                                                                                               *const std::os::raw::c_char,
                                                                                           (*cur).URL);
                            }
                            ret =
                                xmlCatalogListXMLResolve((*cur).children,
                                                         0 as *const xmlChar,
                                                         sysID);
                            if !ret.is_null() {
                                (*catal).depth -= 1;
                                return ret
                            }
                        }
                    }
                }
                cur = (*cur).next
            }
            /*
	     * Apply the cut algorithm explained in 4/
	     */
            (*catal).depth -= 1;
            return -(1 as std::os::raw::c_int) as *mut xmlChar
        }
    }
    /*
     * Then tries 5/ 6/ if a public ID is provided
     */
    if !pubID.is_null() {
        cur = catal;
        haveDelegate = 0 as std::os::raw::c_int;
        while !cur.is_null() {
            match (*cur).type_0 as std::os::raw::c_int {
                5 => {
                    if xmlStrEqual(pubID, (*cur).name) != 0 {
                        if xmlDebugCatalogs != 0 {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"Found public match %s\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       (*cur).name);
                        }
                        (*catal).depth -= 1;
                        return xmlStrdup((*cur).URL)
                    }
                }
                8 => {
                    if xmlStrncmp(pubID, (*cur).name, xmlStrlen((*cur).name))
                           == 0 &&
                           (*cur).prefer as std::os::raw::c_uint ==
                               XML_CATA_PREFER_PUBLIC as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        haveDelegate += 1
                    }
                }
                3 => { if sysID.is_null() { haveNext += 1 } }
                _ => { }
            }
            cur = (*cur).next
        }
        if haveDelegate != 0 {
            let mut delegates_0: [*const xmlChar; 50] =
                [0 as *const xmlChar; 50];
            let mut nbList_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut i_0: std::os::raw::c_int = 0;
            /*
	     * Assume the entries have been sorted by decreasing substring
	     * matches when the list was produced.
	     */
            cur = catal;
            while !cur.is_null() {
                if (*cur).type_0 as std::os::raw::c_int ==
                       XML_CATA_DELEGATE_PUBLIC as std::os::raw::c_int &&
                       (*cur).prefer as std::os::raw::c_uint ==
                           XML_CATA_PREFER_PUBLIC as std::os::raw::c_int as
                               std::os::raw::c_uint &&
                       xmlStrncmp(pubID, (*cur).name, xmlStrlen((*cur).name))
                           == 0 {
                    i_0 = 0 as std::os::raw::c_int;
                    while i_0 < nbList_0 {
                        if xmlStrEqual((*cur).URL, delegates_0[i_0 as usize])
                               != 0 {
                            break ;
                        }
                        i_0 += 1
                    }
                    if i_0 < nbList_0 {
                        cur = (*cur).next;
                        continue ;
                    } else {
                        if nbList_0 < 50 as std::os::raw::c_int {
                            let fresh18 = nbList_0;
                            nbList_0 = nbList_0 + 1;
                            delegates_0[fresh18 as usize] = (*cur).URL
                        }
                        if (*cur).children.is_null() {
                            xmlFetchXMLCatalogFile(cur);
                        }
                        if !(*cur).children.is_null() {
                            if xmlDebugCatalogs != 0 {
                                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                           b"Trying public delegate %s\n\x00"
                                                                                               as
                                                                                               *const u8
                                                                                               as
                                                                                               *const std::os::raw::c_char,
                                                                                           (*cur).URL);
                            }
                            ret =
                                xmlCatalogListXMLResolve((*cur).children,
                                                         pubID,
                                                         0 as *const xmlChar);
                            if !ret.is_null() {
                                (*catal).depth -= 1;
                                return ret
                            }
                        }
                    }
                }
                cur = (*cur).next
            }
            /*
	     * Apply the cut algorithm explained in 4/
	     */
            (*catal).depth -= 1;
            return -(1 as std::os::raw::c_int) as *mut xmlChar
        }
    }
    if haveNext != 0 {
        cur = catal;
        while !cur.is_null() {
            if (*cur).type_0 as std::os::raw::c_int ==
                   XML_CATA_NEXT_CATALOG as std::os::raw::c_int {
                if (*cur).children.is_null() { xmlFetchXMLCatalogFile(cur); }
                if !(*cur).children.is_null() {
                    ret =
                        xmlCatalogListXMLResolve((*cur).children, pubID,
                                                 sysID);
                    if !ret.is_null() {
                        (*catal).depth -= 1;
                        return ret
                    } else {
                        if (*catal).depth > 50 as std::os::raw::c_int {
                            return 0 as *mut xmlChar
                        }
                    }
                }
            }
            cur = (*cur).next
        }
    }
    (*catal).depth -= 1;
    return 0 as *mut xmlChar;
}
/* *
 * xmlCatalogXMLResolveURI:
 * @catal:  a catalog list
 * @URI:  the URI
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier for a
 * list of catalog entries.
 *
 * Implements (or tries to) 7.2.2. URI Resolution
 * from http://www.oasis-open.org/committees/entity/spec-2001-08-06.html
 *
 * Returns the URI of the resource or NULL if not found
 */
unsafe extern "C" fn xmlCatalogXMLResolveURI(mut catal: xmlCatalogEntryPtr,
                                             mut URI: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut haveDelegate: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut haveNext: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut rewrite: xmlCatalogEntryPtr = 0 as xmlCatalogEntryPtr;
    let mut lenrewrite: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut len: std::os::raw::c_int = 0;
    if catal.is_null() { return 0 as *mut xmlChar }
    if URI.is_null() { return 0 as *mut xmlChar }
    if (*catal).depth > 50 as std::os::raw::c_int {
        xmlCatalogErr(catal, 0 as xmlNodePtr,
                      XML_CATALOG_RECURSION as std::os::raw::c_int,
                      b"Detected recursion in catalog %s\n\x00" as *const u8
                          as *const std::os::raw::c_char, (*catal).name,
                      0 as *const xmlChar, 0 as *const xmlChar);
        return 0 as *mut xmlChar
    }
    /*
     * First tries steps 2/ 3/ 4/ if a system ID is provided.
     */
    cur = catal;
    haveDelegate = 0 as std::os::raw::c_int;
    while !cur.is_null() {
        match (*cur).type_0 as std::os::raw::c_int {
            10 => {
                if xmlStrEqual(URI, (*cur).name) != 0 {
                    if xmlDebugCatalogs != 0 {
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Found URI match %s\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   (*cur).name);
                    }
                    return xmlStrdup((*cur).URL)
                }
            }
            11 => {
                len = xmlStrlen((*cur).name);
                if len > lenrewrite && xmlStrncmp(URI, (*cur).name, len) == 0
                   {
                    lenrewrite = len;
                    rewrite = cur
                }
            }
            12 => {
                if xmlStrncmp(URI, (*cur).name, xmlStrlen((*cur).name)) == 0 {
                    haveDelegate += 1
                }
            }
            3 => { haveNext += 1 }
            _ => { }
        }
        cur = (*cur).next
    }
    if !rewrite.is_null() {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Using rewriting rule %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       (*rewrite).name);
        }
        ret = xmlStrdup((*rewrite).URL);
        if !ret.is_null() {
            ret = xmlStrcat(ret, &*URI.offset(lenrewrite as isize))
        }
        return ret
    }
    if haveDelegate != 0 {
        let mut delegates: [*const xmlChar; 50] = [0 as *const xmlChar; 50];
        let mut nbList: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut i: std::os::raw::c_int = 0;
        /*
	 * Assume the entries have been sorted by decreasing substring
	 * matches when the list was produced.
	 */
        cur = catal;
        while !cur.is_null() {
            if ((*cur).type_0 as std::os::raw::c_int ==
                    XML_CATA_DELEGATE_SYSTEM as std::os::raw::c_int ||
                    (*cur).type_0 as std::os::raw::c_int ==
                        XML_CATA_DELEGATE_URI as std::os::raw::c_int) &&
                   xmlStrncmp(URI, (*cur).name, xmlStrlen((*cur).name)) == 0 {
                i = 0 as std::os::raw::c_int;
                while i < nbList {
                    if xmlStrEqual((*cur).URL, delegates[i as usize]) != 0 {
                        break ;
                    }
                    i += 1
                }
                if i < nbList {
                    cur = (*cur).next;
                    continue ;
                } else {
                    if nbList < 50 as std::os::raw::c_int {
                        let fresh19 = nbList;
                        nbList = nbList + 1;
                        delegates[fresh19 as usize] = (*cur).URL
                    }
                    if (*cur).children.is_null() {
                        xmlFetchXMLCatalogFile(cur);
                    }
                    if !(*cur).children.is_null() {
                        if xmlDebugCatalogs != 0 {
                            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                       b"Trying URI delegate %s\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       (*cur).URL);
                        }
                        ret =
                            xmlCatalogListXMLResolveURI((*cur).children, URI);
                        if !ret.is_null() { return ret }
                    }
                }
            }
            cur = (*cur).next
        }
        /*
	 * Apply the cut algorithm explained in 4/
	 */
        return -(1 as std::os::raw::c_int) as *mut xmlChar
    }
    if haveNext != 0 {
        cur = catal;
        while !cur.is_null() {
            if (*cur).type_0 as std::os::raw::c_int ==
                   XML_CATA_NEXT_CATALOG as std::os::raw::c_int {
                if (*cur).children.is_null() { xmlFetchXMLCatalogFile(cur); }
                if !(*cur).children.is_null() {
                    ret = xmlCatalogListXMLResolveURI((*cur).children, URI);
                    if !ret.is_null() { return ret }
                }
            }
            cur = (*cur).next
        }
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlCatalogListXMLResolve:
 * @catal:  a catalog list
 * @pubID:  the public ID string
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier for a
 * list of catalogs
 *
 * Implements (or tries to) 7.1. External Identifier Resolution
 * from http://www.oasis-open.org/committees/entity/spec-2001-08-06.html
 *
 * Returns the URI of the resource or NULL if not found
 */
unsafe extern "C" fn xmlCatalogListXMLResolve(mut catal: xmlCatalogEntryPtr,
                                              mut pubID: *const xmlChar,
                                              mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut urnID: *mut xmlChar = 0 as *mut xmlChar;
    let mut normid: *mut xmlChar = 0 as *mut xmlChar;
    if catal.is_null() { return 0 as *mut xmlChar }
    if pubID.is_null() && sysID.is_null() { return 0 as *mut xmlChar }
    normid = xmlCatalogNormalizePublic(pubID);
    if !normid.is_null() {
        pubID =
            if *normid as std::os::raw::c_int != 0 as std::os::raw::c_int {
                normid
            } else { 0 as *mut xmlChar }
    }
    if xmlStrncmp(pubID,
                  b"urn:publicid:\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar,
                  (::std::mem::size_of::<[std::os::raw::c_char; 14]>() as
                       std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) as
                      std::os::raw::c_int) == 0 {
        urnID = xmlCatalogUnWrapURN(pubID);
        if xmlDebugCatalogs != 0 {
            if urnID.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Public URN ID %s expanded to NULL\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           pubID);
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Public URN ID expanded to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           urnID);
            }
        }
        ret = xmlCatalogListXMLResolve(catal, urnID, sysID);
        if !urnID.is_null() {
            xmlFree.expect("non-null function pointer")(urnID as
                                                            *mut std::os::raw::c_void);
        }
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as
                                                            *mut std::os::raw::c_void);
        }
        return ret
    }
    if xmlStrncmp(sysID,
                  b"urn:publicid:\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar,
                  (::std::mem::size_of::<[std::os::raw::c_char; 14]>() as
                       std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) as
                      std::os::raw::c_int) == 0 {
        urnID = xmlCatalogUnWrapURN(sysID);
        if xmlDebugCatalogs != 0 {
            if urnID.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"System URN ID %s expanded to NULL\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           sysID);
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"System URN ID expanded to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           urnID);
            }
        }
        if pubID.is_null() {
            ret = xmlCatalogListXMLResolve(catal, urnID, 0 as *const xmlChar)
        } else if xmlStrEqual(pubID, urnID) != 0 {
            ret = xmlCatalogListXMLResolve(catal, pubID, 0 as *const xmlChar)
        } else { ret = xmlCatalogListXMLResolve(catal, pubID, urnID) }
        if !urnID.is_null() {
            xmlFree.expect("non-null function pointer")(urnID as
                                                            *mut std::os::raw::c_void);
        }
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as
                                                            *mut std::os::raw::c_void);
        }
        return ret
    }
    while !catal.is_null() {
        if (*catal).type_0 as std::os::raw::c_int == XML_CATA_CATALOG as std::os::raw::c_int {
            if (*catal).children.is_null() { xmlFetchXMLCatalogFile(catal); }
            if !(*catal).children.is_null() {
                ret = xmlCatalogXMLResolve((*catal).children, pubID, sysID);
                if !ret.is_null() { break ; }
                if !(*catal).children.is_null() &&
                       (*(*catal).children).depth > 50 as std::os::raw::c_int {
                    ret = 0 as *mut xmlChar;
                    break ;
                }
            }
        }
        catal = (*catal).next
    }
    if !normid.is_null() {
        xmlFree.expect("non-null function pointer")(normid as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
/* *
 * xmlCatalogListXMLResolveURI:
 * @catal:  a catalog list
 * @URI:  the URI
 *
 * Do a complete resolution lookup of an URI for a list of catalogs
 *
 * Implements (or tries to) 7.2. URI Resolution
 * from http://www.oasis-open.org/committees/entity/spec-2001-08-06.html
 *
 * Returns the URI of the resource or NULL if not found
 */
unsafe extern "C" fn xmlCatalogListXMLResolveURI(mut catal:
                                                     xmlCatalogEntryPtr,
                                                 mut URI: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut urnID: *mut xmlChar = 0 as *mut xmlChar;
    if catal.is_null() { return 0 as *mut xmlChar }
    if URI.is_null() { return 0 as *mut xmlChar }
    if xmlStrncmp(URI,
                  b"urn:publicid:\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar,
                  (::std::mem::size_of::<[std::os::raw::c_char; 14]>() as
                       std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) as
                      std::os::raw::c_int) == 0 {
        urnID = xmlCatalogUnWrapURN(URI);
        if xmlDebugCatalogs != 0 {
            if urnID.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"URN ID %s expanded to NULL\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           URI);
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"URN ID expanded to %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           urnID);
            }
        }
        ret = xmlCatalogListXMLResolve(catal, urnID, 0 as *const xmlChar);
        if !urnID.is_null() {
            xmlFree.expect("non-null function pointer")(urnID as
                                                            *mut std::os::raw::c_void);
        }
        return ret
    }
    while !catal.is_null() {
        if (*catal).type_0 as std::os::raw::c_int == XML_CATA_CATALOG as std::os::raw::c_int {
            if (*catal).children.is_null() { xmlFetchXMLCatalogFile(catal); }
            if !(*catal).children.is_null() {
                ret = xmlCatalogXMLResolveURI((*catal).children, URI);
                if !ret.is_null() { return ret }
            }
        }
        catal = (*catal).next
    }
    return ret;
}
/* *
 * xmlParseSGMLCatalogComment:
 * @cur:  the current character
 *
 * Skip a comment in an SGML catalog
 *
 * Returns new current character
 */
unsafe extern "C" fn xmlParseSGMLCatalogComment(mut cur: *const xmlChar)
 -> *const xmlChar {
    if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '-' as i32 ||
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int != '-' as i32
       {
        return cur
    }
    cur = cur.offset(2 as std::os::raw::c_int as isize);
    while *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
              0 as std::os::raw::c_int &&
              (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   '-' as i32 ||
                   *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                       '-' as i32) {
        cur = cur.offset(1)
    }
    if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        return 0 as *const xmlChar
    }
    return cur.offset(2 as std::os::raw::c_int as isize);
}
/* *
 * xmlParseSGMLCatalogPubid:
 * @cur:  the current character
 * @id:  the return location
 *
 * Parse an SGML catalog ID
 *
 * Returns new current character and store the value in @id
 */
unsafe extern "C" fn xmlParseSGMLCatalogPubid(mut cur: *const xmlChar,
                                              mut id: *mut *mut xmlChar)
 -> *const xmlChar {
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut size: std::os::raw::c_int = 50 as std::os::raw::c_int;
    let mut stop: xmlChar = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    *id = 0 as *mut xmlChar;
    if *cur as std::os::raw::c_int == '\"' as i32 {
        cur = cur.offset(1);
        stop = '\"' as i32 as xmlChar
    } else if *cur as std::os::raw::c_int == '\'' as i32 {
        cur = cur.offset(1);
        stop = '\'' as i32 as xmlChar
    } else { stop = ' ' as i32 as xmlChar }
    buf =
        xmlMallocAtomic.expect("non-null function pointer")((size as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong))
            as *mut xmlChar;
    if buf.is_null() {
        xmlCatalogErrMemory(b"allocating public ID\x00" as *const u8 as
                                *const std::os::raw::c_char);
        return 0 as *const xmlChar
    }
    while xmlIsPubidChar_tab[*cur as usize] as std::os::raw::c_int != 0 ||
              *cur as std::os::raw::c_int == '?' as i32 {
        if *cur as std::os::raw::c_int == stop as std::os::raw::c_int &&
               stop as std::os::raw::c_int != ' ' as i32 {
            break ;
        }
        if stop as std::os::raw::c_int == ' ' as i32 &&
               (*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                    0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                        *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                    *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            break ;
        }
        if len + 1 as std::os::raw::c_int >= size {
            size *= 2 as std::os::raw::c_int;
            tmp =
                xmlRealloc.expect("non-null function pointer")(buf as
                                                                   *mut std::os::raw::c_void,
                                                               (size as
                                                                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                    as
                                                                                                    std::os::raw::c_ulong))
                    as *mut xmlChar;
            if tmp.is_null() {
                xmlCatalogErrMemory(b"allocating public ID\x00" as *const u8
                                        as *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(buf as
                                                                *mut std::os::raw::c_void);
                return 0 as *const xmlChar
            }
            buf = tmp
        }
        let fresh20 = len;
        len = len + 1;
        *buf.offset(fresh20 as isize) = *cur;
        count += 1;
        cur = cur.offset(1)
    }
    *buf.offset(len as isize) = 0 as std::os::raw::c_int as xmlChar;
    if stop as std::os::raw::c_int == ' ' as i32 {
        if !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                 0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                     *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                 *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")(buf as
                                                            *mut std::os::raw::c_void);
            return 0 as *const xmlChar
        }
    } else {
        if *cur as std::os::raw::c_int != stop as std::os::raw::c_int {
            xmlFree.expect("non-null function pointer")(buf as
                                                            *mut std::os::raw::c_void);
            return 0 as *const xmlChar
        }
        cur = cur.offset(1)
    }
    *id = buf;
    return cur;
}
/* *
 * xmlParseSGMLCatalogName:
 * @cur:  the current character
 * @name:  the return location
 *
 * Parse an SGML catalog name
 *
 * Returns new current character and store the value in @name
 */
unsafe extern "C" fn xmlParseSGMLCatalogName(mut cur: *const xmlChar,
                                             mut name: *mut *mut xmlChar)
 -> *const xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut c: std::os::raw::c_int = 0;
    *name = 0 as *mut xmlChar;
    /*
     * Handler for more complex cases
     */
    c = *cur as std::os::raw::c_int;
    if !((if c < 0x100 as std::os::raw::c_int {
              (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                   0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                   0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                   0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                   0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
          } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
             != 0 ||
             (if c < 0x100 as std::os::raw::c_int {
                  0 as std::os::raw::c_int
              } else {
                  (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int ||
                       c == 0x3007 as std::os::raw::c_int ||
                       0x3021 as std::os::raw::c_int <= c &&
                           c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
              }) != 0) && c != '_' as i32 && c != ':' as i32 {
        return 0 as *const xmlChar
    }
    while (if c < 0x100 as std::os::raw::c_int {
               (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                    0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                    0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                    0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                    0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
           } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup) })
              != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   0 as std::os::raw::c_int
               } else {
                   (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int
                        || c == 0x3007 as std::os::raw::c_int ||
                        0x3021 as std::os::raw::c_int <= c &&
                            c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
               }) != 0 ||
              (if c < 0x100 as std::os::raw::c_int {
                   (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int) as
                       std::os::raw::c_int
               } else { xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup) })
                  != 0 || c == '.' as i32 || c == '-' as i32 ||
              c == '_' as i32 || c == ':' as i32 {
        let fresh21 = len;
        len = len + 1;
        buf[fresh21 as usize] = c as xmlChar;
        cur = cur.offset(1);
        c = *cur as std::os::raw::c_int;
        if len >= 100 as std::os::raw::c_int { return 0 as *const xmlChar }
    }
    *name = xmlStrndup(buf.as_mut_ptr(), len);
    return cur;
}
/* *
 * xmlGetSGMLCatalogEntryType:
 * @name:  the entry name
 *
 * Get the Catalog entry type for a given SGML Catalog name
 *
 * Returns Catalog entry type
 */
unsafe extern "C" fn xmlGetSGMLCatalogEntryType(mut name: *const xmlChar)
 -> xmlCatalogEntryType {
    let mut type_0: xmlCatalogEntryType = XML_CATA_NONE;
    if xmlStrEqual(name,
                   b"SYSTEM\x00" as *const u8 as *const std::os::raw::c_char as
                       *const xmlChar) != 0 {
        type_0 = SGML_CATA_SYSTEM
    } else if xmlStrEqual(name,
                          b"PUBLIC\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar) != 0 {
        type_0 = SGML_CATA_PUBLIC
    } else if xmlStrEqual(name,
                          b"DELEGATE\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_DELEGATE
    } else if xmlStrEqual(name,
                          b"ENTITY\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar) != 0 {
        type_0 = SGML_CATA_ENTITY
    } else if xmlStrEqual(name,
                          b"DOCTYPE\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_DOCTYPE
    } else if xmlStrEqual(name,
                          b"LINKTYPE\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_LINKTYPE
    } else if xmlStrEqual(name,
                          b"NOTATION\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_NOTATION
    } else if xmlStrEqual(name,
                          b"SGMLDECL\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_SGMLDECL
    } else if xmlStrEqual(name,
                          b"DOCUMENT\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_DOCUMENT
    } else if xmlStrEqual(name,
                          b"CATALOG\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar) != 0 {
        type_0 = SGML_CATA_CATALOG
    } else if xmlStrEqual(name,
                          b"BASE\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar) != 0 {
        type_0 = SGML_CATA_BASE
    }
    return type_0;
}
/* *
 * xmlParseSGMLCatalog:
 * @catal:  the SGML Catalog
 * @value:  the content of the SGML Catalog serialization
 * @file:  the filepath for the catalog
 * @super:  should this be handled as a Super Catalog in which case
 *          parsing is not recursive
 *
 * Parse an SGML catalog content and fill up the @catal hash table with
 * the new entries found.
 *
 * Returns 0 in case of success, -1 in case of error.
 */
unsafe extern "C" fn xmlParseSGMLCatalog(mut catal: xmlCatalogPtr,
                                         mut value: *const xmlChar,
                                         mut file: *const std::os::raw::c_char,
                                         mut super_0: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = value;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut res: std::os::raw::c_int = 0;
    if cur.is_null() || file.is_null() { return -(1 as std::os::raw::c_int) }
    base = xmlStrdup(file as *const xmlChar);
    while !cur.is_null() &&
              *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                  0 as std::os::raw::c_int {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
        if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
            break ;
        }
        if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '-' as i32
               &&
               *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '-' as i32 {
            cur = xmlParseSGMLCatalogComment(cur);
            if !cur.is_null() { continue ; }
            /* error */
            break ;
        } else {
            let mut sysid: *mut xmlChar = 0 as *mut xmlChar;
            let mut name: *mut xmlChar = 0 as *mut xmlChar;
            let mut type_0: xmlCatalogEntryType = XML_CATA_NONE;
            cur = xmlParseSGMLCatalogName(cur, &mut name);
            if name.is_null() {
                /* error */
                break ;
            } else if !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                /* error */
                break ;
            } else {
                while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                              *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                          *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    cur = cur.offset(1)
                }
                if xmlStrEqual(name,
                               b"SYSTEM\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar) != 0
                   {
                    type_0 = SGML_CATA_SYSTEM
                } else if xmlStrEqual(name,
                                      b"PUBLIC\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_PUBLIC
                } else if xmlStrEqual(name,
                                      b"DELEGATE\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_DELEGATE
                } else if xmlStrEqual(name,
                                      b"ENTITY\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_ENTITY
                } else if xmlStrEqual(name,
                                      b"DOCTYPE\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_DOCTYPE
                } else if xmlStrEqual(name,
                                      b"LINKTYPE\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_LINKTYPE
                } else if xmlStrEqual(name,
                                      b"NOTATION\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_NOTATION
                } else if xmlStrEqual(name,
                                      b"SGMLDECL\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_SGMLDECL
                } else if xmlStrEqual(name,
                                      b"DOCUMENT\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_DOCUMENT
                } else if xmlStrEqual(name,
                                      b"CATALOG\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_CATALOG
                } else if xmlStrEqual(name,
                                      b"BASE\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    type_0 = SGML_CATA_BASE
                } else if xmlStrEqual(name,
                                      b"OVERRIDE\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *const xmlChar) != 0 {
                    xmlFree.expect("non-null function pointer")(name as
                                                                    *mut std::os::raw::c_void);
                    cur = xmlParseSGMLCatalogName(cur, &mut name);
                    if name.is_null() { break ; }
                    xmlFree.expect("non-null function pointer")(name as
                                                                    *mut std::os::raw::c_void);
                    continue ;
                }
                xmlFree.expect("non-null function pointer")(name as
                                                                *mut std::os::raw::c_void);
                name = 0 as *mut xmlChar;
                let mut current_block_55: u64;
                match type_0 as std::os::raw::c_int {
                    15 => {
                        if *cur as std::os::raw::c_int == '%' as i32 {
                            type_0 = SGML_CATA_PENTITY
                        }
                        current_block_55 = 18379703266420589864;
                    }
                    16 | 17 | 18 | 19 => {
                        current_block_55 = 18379703266420589864;
                    }
                    14 | 13 | 20 => {
                        cur = xmlParseSGMLCatalogPubid(cur, &mut name);
                        if !cur.is_null() {
                            if type_0 as std::os::raw::c_int !=
                                   SGML_CATA_SYSTEM as std::os::raw::c_int {
                                let mut normid: *mut xmlChar =
                                    0 as *mut xmlChar;
                                normid = xmlCatalogNormalizePublic(name);
                                if !normid.is_null() {
                                    if !name.is_null() {
                                        xmlFree.expect("non-null function pointer")(name
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                    }
                                    if *normid as std::os::raw::c_int !=
                                           0 as std::os::raw::c_int {
                                        name = normid
                                    } else {
                                        xmlFree.expect("non-null function pointer")(normid
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                        name = 0 as *mut xmlChar
                                    }
                                }
                            }
                            if *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                   0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int
                                       &&
                                       *cur as std::os::raw::c_int <=
                                           0xa as std::os::raw::c_int ||
                                   *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                                while *cur as std::os::raw::c_int ==
                                          0x20 as std::os::raw::c_int ||
                                          0x9 as std::os::raw::c_int <=
                                              *cur as std::os::raw::c_int &&
                                              *cur as std::os::raw::c_int <=
                                                  0xa as std::os::raw::c_int ||
                                          *cur as std::os::raw::c_int ==
                                              0xd as std::os::raw::c_int {
                                    cur = cur.offset(1)
                                }
                                cur =
                                    xmlParseSGMLCatalogPubid(cur, &mut sysid);
                                cur.is_null();
                            }
                        }
                        current_block_55 = 16313536926714486912;
                    }
                    21 | 22 | 23 | 24 => {
                        cur = xmlParseSGMLCatalogPubid(cur, &mut sysid);
                        cur.is_null();
                        current_block_55 = 16313536926714486912;
                    }
                    _ => { current_block_55 = 16313536926714486912; }
                }
                match current_block_55 {
                    18379703266420589864 =>
                    /* Falls through. */
                    {
                        cur = xmlParseSGMLCatalogName(cur, &mut name);
                        if !cur.is_null() {
                            if *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                   0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int
                                       &&
                                       *cur as std::os::raw::c_int <=
                                           0xa as std::os::raw::c_int ||
                                   *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                                while *cur as std::os::raw::c_int ==
                                          0x20 as std::os::raw::c_int ||
                                          0x9 as std::os::raw::c_int <=
                                              *cur as std::os::raw::c_int &&
                                              *cur as std::os::raw::c_int <=
                                                  0xa as std::os::raw::c_int ||
                                          *cur as std::os::raw::c_int ==
                                              0xd as std::os::raw::c_int {
                                    cur = cur.offset(1)
                                }
                                cur =
                                    xmlParseSGMLCatalogPubid(cur, &mut sysid);
                                cur.is_null();
                            }
                        }
                    }
                    _ => { }
                }
                if cur.is_null() {
                    if !name.is_null() {
                        xmlFree.expect("non-null function pointer")(name as
                                                                        *mut std::os::raw::c_void);
                    }
                    if !sysid.is_null() {
                        xmlFree.expect("non-null function pointer")(sysid as
                                                                        *mut std::os::raw::c_void);
                    }
                    break ;
                } else {
                    if type_0 as std::os::raw::c_int == SGML_CATA_BASE as std::os::raw::c_int
                       {
                        if !base.is_null() {
                            xmlFree.expect("non-null function pointer")(base
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                        base = xmlStrdup(sysid)
                    } else if type_0 as std::os::raw::c_int ==
                                  SGML_CATA_PUBLIC as std::os::raw::c_int ||
                                  type_0 as std::os::raw::c_int ==
                                      SGML_CATA_SYSTEM as std::os::raw::c_int {
                        let mut filename: *mut xmlChar = 0 as *mut xmlChar;
                        filename = xmlBuildURI(sysid, base);
                        if !filename.is_null() {
                            let mut entry: xmlCatalogEntryPtr =
                                0 as *mut xmlCatalogEntry;
                            entry =
                                xmlNewCatalogEntry(type_0, name, filename,
                                                   0 as *const xmlChar,
                                                   XML_CATA_PREFER_NONE,
                                                   0 as xmlCatalogEntryPtr);
                            res =
                                xmlHashAddEntry((*catal).sgml, name,
                                                entry as *mut std::os::raw::c_void);
                            if res < 0 as std::os::raw::c_int {
                                xmlFreeCatalogEntry(entry as
                                                        *mut std::os::raw::c_void,
                                                    0 as *const xmlChar);
                            }
                            xmlFree.expect("non-null function pointer")(filename
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    } else if type_0 as std::os::raw::c_int ==
                                  SGML_CATA_CATALOG as std::os::raw::c_int {
                        if super_0 != 0 {
                            let mut entry_0: xmlCatalogEntryPtr =
                                0 as *mut xmlCatalogEntry;
                            entry_0 =
                                xmlNewCatalogEntry(type_0, sysid,
                                                   0 as *const xmlChar,
                                                   0 as *const xmlChar,
                                                   XML_CATA_PREFER_NONE,
                                                   0 as xmlCatalogEntryPtr);
                            res =
                                xmlHashAddEntry((*catal).sgml, sysid,
                                                entry_0 as *mut std::os::raw::c_void);
                            if res < 0 as std::os::raw::c_int {
                                xmlFreeCatalogEntry(entry_0 as
                                                        *mut std::os::raw::c_void,
                                                    0 as *const xmlChar);
                            }
                        } else {
                            let mut filename_0: *mut xmlChar =
                                0 as *mut xmlChar;
                            filename_0 = xmlBuildURI(sysid, base);
                            if !filename_0.is_null() {
                                xmlExpandCatalog(catal,
                                                 filename_0 as
                                                     *const std::os::raw::c_char);
                                xmlFree.expect("non-null function pointer")(filename_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                        }
                    }
                    /*
	     * drop anything else we won't handle it
	     */
                    if !name.is_null() {
                        xmlFree.expect("non-null function pointer")(name as
                                                                        *mut std::os::raw::c_void);
                    }
                    if !sysid.is_null() {
                        xmlFree.expect("non-null function pointer")(sysid as
                                                                        *mut std::os::raw::c_void);
                    }
                }
            }
        }
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as
                                                        *mut std::os::raw::c_void);
    }
    if cur.is_null() { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *			SGML Catalog handling				*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogGetSGMLPublic:
 * @catal:  an SGML catalog hash
 * @pubID:  the public ID string
 *
 * Try to lookup the catalog local reference associated to a public ID
 *
 * Returns the local resource if found or NULL otherwise.
 */
unsafe extern "C" fn xmlCatalogGetSGMLPublic(mut catal: xmlHashTablePtr,
                                             mut pubID: *const xmlChar)
 -> *const xmlChar {
    let mut entry: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut normid: *mut xmlChar = 0 as *mut xmlChar;
    if catal.is_null() { return 0 as *const xmlChar }
    normid = xmlCatalogNormalizePublic(pubID);
    if !normid.is_null() {
        pubID =
            if *normid as std::os::raw::c_int != 0 as std::os::raw::c_int {
                normid
            } else { 0 as *mut xmlChar }
    }
    entry = xmlHashLookup(catal, pubID) as xmlCatalogEntryPtr;
    if entry.is_null() {
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as
                                                            *mut std::os::raw::c_void);
        }
        return 0 as *const xmlChar
    }
    if (*entry).type_0 as std::os::raw::c_int == SGML_CATA_PUBLIC as std::os::raw::c_int {
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as
                                                            *mut std::os::raw::c_void);
        }
        return (*entry).URL
    }
    if !normid.is_null() {
        xmlFree.expect("non-null function pointer")(normid as
                                                        *mut std::os::raw::c_void);
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlCatalogGetSGMLSystem:
 * @catal:  an SGML catalog hash
 * @sysID:  the system ID string
 *
 * Try to lookup the catalog local reference for a system ID
 *
 * Returns the local resource if found or NULL otherwise.
 */
unsafe extern "C" fn xmlCatalogGetSGMLSystem(mut catal: xmlHashTablePtr,
                                             mut sysID: *const xmlChar)
 -> *const xmlChar {
    let mut entry: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    if catal.is_null() { return 0 as *const xmlChar }
    entry = xmlHashLookup(catal, sysID) as xmlCatalogEntryPtr;
    if entry.is_null() { return 0 as *const xmlChar }
    if (*entry).type_0 as std::os::raw::c_int == SGML_CATA_SYSTEM as std::os::raw::c_int {
        return (*entry).URL
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlCatalogSGMLResolve:
 * @catal:  the SGML catalog
 * @pubID:  the public ID string
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier
 *
 * Returns the URI of the resource or NULL if not found
 */
unsafe extern "C" fn xmlCatalogSGMLResolve(mut catal: xmlCatalogPtr,
                                           mut pubID: *const xmlChar,
                                           mut sysID: *const xmlChar)
 -> *const xmlChar {
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if (*catal).sgml.is_null() { return 0 as *const xmlChar }
    if !pubID.is_null() {
        ret = xmlCatalogGetSGMLPublic((*catal).sgml, pubID)
    }
    if !ret.is_null() { return ret }
    if !sysID.is_null() {
        ret = xmlCatalogGetSGMLSystem((*catal).sgml, sysID)
    }
    if !ret.is_null() { return ret }
    return 0 as *const xmlChar;
}
/* ***********************************************************************
 *									*
 *			Specific Public interfaces			*
 *									*
 ************************************************************************/
/* *
 * xmlLoadSGMLSuperCatalog:
 * @filename:  a file path
 *
 * Load an SGML super catalog. It won't expand CATALOG or DELEGATE
 * references. This is only needed for manipulating SGML Super Catalogs
 * like adding and removing CATALOG or DELEGATE entries.
 *
 * Returns the catalog parsed or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLoadSGMLSuperCatalog(mut filename:
                                                     *const std::os::raw::c_char)
 -> xmlCatalogPtr {
    let mut content: *mut xmlChar = 0 as *mut xmlChar;
    let mut catal: xmlCatalogPtr = 0 as *mut xmlCatalog;
    let mut ret: std::os::raw::c_int = 0;
    content = xmlLoadFileContent(filename);
    if content.is_null() { return 0 as xmlCatalogPtr }
    catal =
        xmlCreateNewCatalog(XML_SGML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
    if catal.is_null() {
        xmlFree.expect("non-null function pointer")(content as
                                                        *mut std::os::raw::c_void);
        return 0 as xmlCatalogPtr
    }
    ret = xmlParseSGMLCatalog(catal, content, filename, 1 as std::os::raw::c_int);
    xmlFree.expect("non-null function pointer")(content as *mut std::os::raw::c_void);
    if ret < 0 as std::os::raw::c_int {
        xmlFreeCatalog(catal);
        return 0 as xmlCatalogPtr
    }
    return catal;
}
/* *
 * xmlLoadACatalog:
 * @filename:  a file path
 *
 * Load the catalog and build the associated data structures.
 * This can be either an XML Catalog or an SGML Catalog
 * It will recurse in SGML CATALOG entries. On the other hand XML
 * Catalogs are not handled recursively.
 *
 * Returns the catalog parsed or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLoadACatalog(mut filename: *const std::os::raw::c_char)
 -> xmlCatalogPtr {
    let mut content: *mut xmlChar = 0 as *mut xmlChar;
    let mut first: *mut xmlChar = 0 as *mut xmlChar;
    let mut catal: xmlCatalogPtr = 0 as *mut xmlCatalog;
    let mut ret: std::os::raw::c_int = 0;
    content = xmlLoadFileContent(filename);
    if content.is_null() { return 0 as xmlCatalogPtr }
    first = content;
    while *first as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *first as std::os::raw::c_int != '-' as i32 &&
              *first as std::os::raw::c_int != '<' as i32 &&
              !(*first as std::os::raw::c_int >= 'A' as i32 &&
                    *first as std::os::raw::c_int <= 'Z' as i32 ||
                    *first as std::os::raw::c_int >= 'a' as i32 &&
                        *first as std::os::raw::c_int <= 'z' as i32) {
        first = first.offset(1)
    }
    if *first as std::os::raw::c_int != '<' as i32 {
        catal =
            xmlCreateNewCatalog(XML_SGML_CATALOG_TYPE,
                                xmlCatalogDefaultPrefer);
        if catal.is_null() {
            xmlFree.expect("non-null function pointer")(content as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlCatalogPtr
        }
        ret = xmlParseSGMLCatalog(catal, content, filename, 0 as std::os::raw::c_int);
        if ret < 0 as std::os::raw::c_int {
            xmlFreeCatalog(catal);
            xmlFree.expect("non-null function pointer")(content as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlCatalogPtr
        }
    } else {
        catal =
            xmlCreateNewCatalog(XML_XML_CATALOG_TYPE,
                                xmlCatalogDefaultPrefer);
        if catal.is_null() {
            xmlFree.expect("non-null function pointer")(content as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlCatalogPtr
        }
        (*catal).xml =
            xmlNewCatalogEntry(XML_CATA_CATALOG, 0 as *const xmlChar,
                               0 as *const xmlChar, filename as *mut xmlChar,
                               xmlCatalogDefaultPrefer,
                               0 as xmlCatalogEntryPtr)
    }
    xmlFree.expect("non-null function pointer")(content as *mut std::os::raw::c_void);
    return catal;
}
/* *
 * xmlExpandCatalog:
 * @catal:  a catalog
 * @filename:  a file path
 *
 * Load the catalog and expand the existing catal structure.
 * This can be either an XML Catalog or an SGML Catalog
 *
 * Returns 0 in case of success, -1 in case of error
 */
unsafe extern "C" fn xmlExpandCatalog(mut catal: xmlCatalogPtr,
                                      mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if catal.is_null() || filename.is_null() { return -(1 as std::os::raw::c_int) }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_SGML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        let mut content: *mut xmlChar = 0 as *mut xmlChar;
        content = xmlLoadFileContent(filename);
        if content.is_null() { return -(1 as std::os::raw::c_int) }
        ret = xmlParseSGMLCatalog(catal, content, filename, 0 as std::os::raw::c_int);
        if ret < 0 as std::os::raw::c_int {
            xmlFree.expect("non-null function pointer")(content as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        xmlFree.expect("non-null function pointer")(content as
                                                        *mut std::os::raw::c_void);
    } else {
        let mut tmp: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
        let mut cur: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
        tmp =
            xmlNewCatalogEntry(XML_CATA_CATALOG, 0 as *const xmlChar,
                               0 as *const xmlChar, filename as *mut xmlChar,
                               xmlCatalogDefaultPrefer,
                               0 as xmlCatalogEntryPtr);
        cur = (*catal).xml;
        if cur.is_null() {
            (*catal).xml = tmp
        } else {
            while !(*cur).next.is_null() { cur = (*cur).next }
            (*cur).next = tmp
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlACatalogResolveSystem:
 * @catal:  a Catalog
 * @sysID:  the system ID string
 *
 * Try to lookup the catalog resource for a system ID
 *
 * Returns the resource if found or NULL otherwise, the value returned
 *      must be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolveSystem(mut catal: xmlCatalogPtr,
                                                  mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if sysID.is_null() || catal.is_null() { return 0 as *mut xmlChar }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Resolve sysID %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   sysID);
    }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        ret =
            xmlCatalogListXMLResolve((*catal).xml, 0 as *const xmlChar,
                                     sysID);
        if ret == -(1 as std::os::raw::c_int) as *mut xmlChar {
            ret = 0 as *mut xmlChar
        }
    } else {
        let mut sgml: *const xmlChar = 0 as *const xmlChar;
        sgml = xmlCatalogGetSGMLSystem((*catal).sgml, sysID);
        if !sgml.is_null() { ret = xmlStrdup(sgml) }
    }
    return ret;
}
/* *
 * xmlACatalogResolvePublic:
 * @catal:  a Catalog
 * @pubID:  the public ID string
 *
 * Try to lookup the catalog local reference associated to a public ID in that catalog
 *
 * Returns the local resource if found or NULL otherwise, the value returned
 *      must be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolvePublic(mut catal: xmlCatalogPtr,
                                                  mut pubID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if pubID.is_null() || catal.is_null() { return 0 as *mut xmlChar }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Resolve pubID %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   pubID);
    }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        ret =
            xmlCatalogListXMLResolve((*catal).xml, pubID,
                                     0 as *const xmlChar);
        if ret == -(1 as std::os::raw::c_int) as *mut xmlChar {
            ret = 0 as *mut xmlChar
        }
    } else {
        let mut sgml: *const xmlChar = 0 as *const xmlChar;
        sgml = xmlCatalogGetSGMLPublic((*catal).sgml, pubID);
        if !sgml.is_null() { ret = xmlStrdup(sgml) }
    }
    return ret;
}
/* *
 * xmlACatalogResolve:
 * @catal:  a Catalog
 * @pubID:  the public ID string
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier
 *
 * Returns the URI of the resource or NULL if not found, it must be freed
 *      by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolve(mut catal: xmlCatalogPtr,
                                            mut pubID: *const xmlChar,
                                            mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if pubID.is_null() && sysID.is_null() || catal.is_null() {
        return 0 as *mut xmlChar
    }
    if xmlDebugCatalogs != 0 {
        if !pubID.is_null() && !sysID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Resolve: pubID %s sysID %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pubID,
                                                                       sysID);
        } else if !pubID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Resolve: pubID %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pubID);
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Resolve: sysID %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       sysID);
        }
    }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        ret = xmlCatalogListXMLResolve((*catal).xml, pubID, sysID);
        if ret == -(1 as std::os::raw::c_int) as *mut xmlChar {
            ret = 0 as *mut xmlChar
        }
    } else {
        let mut sgml: *const xmlChar = 0 as *const xmlChar;
        sgml = xmlCatalogSGMLResolve(catal, pubID, sysID);
        if !sgml.is_null() { ret = xmlStrdup(sgml) }
    }
    return ret;
}
/* *
 * xmlACatalogResolveURI:
 * @catal:  a Catalog
 * @URI:  the URI
 *
 * Do a complete resolution lookup of an URI
 *
 * Returns the URI of the resource or NULL if not found, it must be freed
 *      by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolveURI(mut catal: xmlCatalogPtr,
                                               mut URI: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if URI.is_null() || catal.is_null() { return 0 as *mut xmlChar }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Resolve URI %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   URI);
    }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        ret = xmlCatalogListXMLResolveURI((*catal).xml, URI);
        if ret == -(1 as std::os::raw::c_int) as *mut xmlChar {
            ret = 0 as *mut xmlChar
        }
    } else {
        let mut sgml: *const xmlChar = 0 as *const xmlChar;
        sgml = xmlCatalogSGMLResolve(catal, 0 as *const xmlChar, URI);
        if !sgml.is_null() { ret = xmlStrdup(sgml) }
    }
    return ret;
}
/* *
 * xmlACatalogDump:
 * @catal:  a Catalog
 * @out:  the file.
 *
 * Dump the given catalog to the given file.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogDump(mut catal: xmlCatalogPtr,
                                         mut out: *mut FILE) {
    if out.is_null() || catal.is_null() { return }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlDumpXMLCatalog(out, (*catal).xml);
    } else {
        xmlHashScan((*catal).sgml,
                    Some(xmlCatalogDumpEntry as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    out as *mut std::os::raw::c_void);
    };
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlACatalogAdd:
 * @catal:  a Catalog
 * @type:  the type of record to add to the catalog
 * @orig:  the system, public or prefix to match
 * @replace:  the replacement value for the match
 *
 * Add an entry in the catalog, it may overwrite existing but
 * different entries.
 *
 * Returns 0 if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogAdd(mut catal: xmlCatalogPtr,
                                        mut type_0: *const xmlChar,
                                        mut orig: *const xmlChar,
                                        mut replace: *const xmlChar)
 -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if catal.is_null() { return -(1 as std::os::raw::c_int) }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        res = xmlAddXMLCatalog((*catal).xml, type_0, orig, replace)
    } else {
        let mut cattype: xmlCatalogEntryType = XML_CATA_NONE;
        cattype = xmlGetSGMLCatalogEntryType(type_0);
        if cattype as std::os::raw::c_int != XML_CATA_NONE as std::os::raw::c_int {
            let mut entry: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
            entry =
                xmlNewCatalogEntry(cattype, orig, replace,
                                   0 as *const xmlChar, XML_CATA_PREFER_NONE,
                                   0 as xmlCatalogEntryPtr);
            if (*catal).sgml.is_null() {
                (*catal).sgml = xmlHashCreate(10 as std::os::raw::c_int)
            }
            res =
                xmlHashAddEntry((*catal).sgml, orig,
                                entry as *mut std::os::raw::c_void)
        }
    }
    return res;
}
/* *
 * xmlACatalogRemove:
 * @catal:  a Catalog
 * @value:  the value to remove
 *
 * Remove an entry from the catalog
 *
 * Returns the number of entries removed if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogRemove(mut catal: xmlCatalogPtr,
                                           mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if catal.is_null() || value.is_null() { return -(1 as std::os::raw::c_int) }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        res = xmlDelXMLCatalog((*catal).xml, value)
    } else {
        res =
            xmlHashRemoveEntry((*catal).sgml, value,
                               Some(xmlFreeCatalogEntry as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void,
                                                             _:
                                                                 *const xmlChar)
                                            -> ()));
        if res == 0 as std::os::raw::c_int { res = 1 as std::os::raw::c_int }
    }
    return res;
}
/*
 * Operations on a given catalog.
 */
/* *
 * xmlNewCatalog:
 * @sgml:  should this create an SGML catalog
 *
 * create a new Catalog.
 *
 * Returns the xmlCatalogPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewCatalog(mut sgml: std::os::raw::c_int)
 -> xmlCatalogPtr {
    let mut catal: xmlCatalogPtr = 0 as xmlCatalogPtr;
    if sgml != 0 {
        catal =
            xmlCreateNewCatalog(XML_SGML_CATALOG_TYPE,
                                xmlCatalogDefaultPrefer);
        if !catal.is_null() && (*catal).sgml.is_null() {
            (*catal).sgml = xmlHashCreate(10 as std::os::raw::c_int)
        }
    } else {
        catal =
            xmlCreateNewCatalog(XML_XML_CATALOG_TYPE, xmlCatalogDefaultPrefer)
    }
    return catal;
}
/* *
 * xmlCatalogIsEmpty:
 * @catal:  should this create an SGML catalog
 *
 * Check is a catalog is empty
 *
 * Returns 1 if the catalog is empty, 0 if not, amd -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogIsEmpty(mut catal: xmlCatalogPtr)
 -> std::os::raw::c_int {
    if catal.is_null() { return -(1 as std::os::raw::c_int) }
    if (*catal).type_0 as std::os::raw::c_uint ==
           XML_XML_CATALOG_TYPE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*catal).xml.is_null() { return 1 as std::os::raw::c_int }
        if (*(*catal).xml).type_0 as std::os::raw::c_int !=
               XML_CATA_CATALOG as std::os::raw::c_int &&
               (*(*catal).xml).type_0 as std::os::raw::c_int !=
                   XML_CATA_BROKEN_CATALOG as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
        if (*(*catal).xml).children.is_null() { return 1 as std::os::raw::c_int }
        return 0 as std::os::raw::c_int
    } else {
        let mut res: std::os::raw::c_int = 0;
        if (*catal).sgml.is_null() { return 1 as std::os::raw::c_int }
        res = xmlHashSize((*catal).sgml);
        if res == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
        if res < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    }
    return 0 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *   Public interfaces manipulating the global shared default catalog	*
 *									*
 ************************************************************************/
/* *
 * xmlInitializeCatalogData:
 *
 * Do the catalog initialization only of global data, doesn't try to load
 * any catalog actually.
 * this function is not thread safe, catalog initialization should
 * preferably be done once at startup
 */
unsafe extern "C" fn xmlInitializeCatalogData() {
    if xmlCatalogInitialized != 0 as std::os::raw::c_int { return }
    if !getenv(b"XML_DEBUG_CATALOG\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null() {
        xmlDebugCatalogs = 1 as std::os::raw::c_int
    }
    xmlCatalogMutex = xmlNewRMutex();
    xmlCatalogInitialized = 1 as std::os::raw::c_int;
}
/*
 * Global operations.
 */
/* *
 * xmlInitializeCatalog:
 *
 * Do the catalog initialization.
 * this function is not thread safe, catalog initialization should
 * preferably be done once at startup
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeCatalog() {
    if xmlCatalogInitialized != 0 as std::os::raw::c_int { return }
    xmlInitializeCatalogData();
    xmlRMutexLock(xmlCatalogMutex);
    if !getenv(b"XML_DEBUG_CATALOG\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null() {
        xmlDebugCatalogs = 1 as std::os::raw::c_int
    }
    if xmlDefaultCatalog.is_null() {
        let mut catalogs: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut paths: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        let mut catal: xmlCatalogPtr = 0 as *mut xmlCatalog;
        let mut nextent: *mut xmlCatalogEntryPtr =
            0 as *mut xmlCatalogEntryPtr;
        catalogs =
            getenv(b"XML_CATALOG_FILES\x00" as *const u8 as
                       *const std::os::raw::c_char) as *const std::os::raw::c_char;
        if catalogs.is_null() {
            catalogs =
                b"file:///etc/xml/catalog\x00" as *const u8 as
                    *const std::os::raw::c_char
        }
        catal =
            xmlCreateNewCatalog(XML_XML_CATALOG_TYPE,
                                xmlCatalogDefaultPrefer);
        if !catal.is_null() {
            /* the XML_CATALOG_FILES envvar is allowed to contain a
	       space-separated list of entries. */
            cur = catalogs;
            nextent = &mut (*catal).xml;
            while *cur as std::os::raw::c_int != '\u{0}' as i32 {
                while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                              *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                          *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    cur = cur.offset(1)
                }
                if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    paths = cur;
                    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                              !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                    0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int
                                        &&
                                        *cur as std::os::raw::c_int <=
                                            0xa as std::os::raw::c_int ||
                                    *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int)
                          {
                        cur = cur.offset(1)
                    }
                    path =
                        xmlStrndup(paths as *const xmlChar,
                                   cur.offset_from(paths) as
                                       std::os::raw::c_long as std::os::raw::c_int) as
                            *mut std::os::raw::c_char;
                    if !path.is_null() {
                        *nextent =
                            xmlNewCatalogEntry(XML_CATA_CATALOG,
                                               0 as *const xmlChar,
                                               0 as *const xmlChar,
                                               path as *mut xmlChar,
                                               xmlCatalogDefaultPrefer,
                                               0 as xmlCatalogEntryPtr);
                        if !(*nextent).is_null() {
                            nextent = &mut (**nextent).next
                        }
                        xmlFree.expect("non-null function pointer")(path as
                                                                        *mut std::os::raw::c_void);
                    }
                }
            }
            xmlDefaultCatalog = catal
        }
    }
    xmlRMutexUnlock(xmlCatalogMutex);
}
/* *
 * xmlLoadCatalog:
 * @filename:  a file path
 *
 * Load the catalog and makes its definitions effective for the default
 * external entity loader. It will recurse in SGML CATALOG entries.
 * this function is not thread safe, catalog initialization should
 * preferably be done once at startup
 *
 * Returns 0 in case of success -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLoadCatalog(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut catal: xmlCatalogPtr = 0 as *mut xmlCatalog;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalogData(); }
    xmlRMutexLock(xmlCatalogMutex);
    if xmlDefaultCatalog.is_null() {
        catal = xmlLoadACatalog(filename);
        if catal.is_null() {
            xmlRMutexUnlock(xmlCatalogMutex);
            return -(1 as std::os::raw::c_int)
        }
        xmlDefaultCatalog = catal;
        xmlRMutexUnlock(xmlCatalogMutex);
        return 0 as std::os::raw::c_int
    }
    ret = xmlExpandCatalog(xmlDefaultCatalog, filename);
    xmlRMutexUnlock(xmlCatalogMutex);
    return ret;
}
/* *
 * xmlLoadCatalogs:
 * @pathss:  a list of directories separated by a colon or a space.
 *
 * Load the catalogs and makes their definitions effective for the default
 * external entity loader.
 * this function is not thread safe, catalog initialization should
 * preferably be done once at startup
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLoadCatalogs(mut pathss: *const std::os::raw::c_char) {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut paths: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut path: *mut xmlChar = 0 as *mut xmlChar;
    if pathss.is_null() { return }
    cur = pathss;
    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
        if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
            paths = cur;
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int != ':' as i32 &&
                      !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                cur = cur.offset(1)
            }
            path =
                xmlStrndup(paths as *const xmlChar,
                           cur.offset_from(paths) as std::os::raw::c_long as
                               std::os::raw::c_int);
            if !path.is_null() {
                xmlLoadCatalog(path as *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(path as
                                                                *mut std::os::raw::c_void);
            }
        }
        while *cur as std::os::raw::c_int == ':' as i32 { cur = cur.offset(1) }
    };
}
/* *
 * xmlCatalogCleanup:
 *
 * Free up all the memory associated with catalogs
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogCleanup() {
    if xmlCatalogInitialized == 0 as std::os::raw::c_int { return }
    xmlRMutexLock(xmlCatalogMutex);
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Catalogs cleanup\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
    }
    if !xmlCatalogXMLFiles.is_null() {
        xmlHashFree(xmlCatalogXMLFiles,
                    Some(xmlFreeCatalogHashEntryList as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()));
    }
    xmlCatalogXMLFiles = 0 as xmlHashTablePtr;
    if !xmlDefaultCatalog.is_null() { xmlFreeCatalog(xmlDefaultCatalog); }
    xmlDefaultCatalog = 0 as xmlCatalogPtr;
    xmlDebugCatalogs = 0 as std::os::raw::c_int;
    xmlCatalogInitialized = 0 as std::os::raw::c_int;
    xmlRMutexUnlock(xmlCatalogMutex);
    xmlFreeRMutex(xmlCatalogMutex);
}
/* *
 * xmlCatalogResolveSystem:
 * @sysID:  the system ID string
 *
 * Try to lookup the catalog resource for a system ID
 *
 * Returns the resource if found or NULL otherwise, the value returned
 *      must be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolveSystem(mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    ret = xmlACatalogResolveSystem(xmlDefaultCatalog, sysID);
    return ret;
}
/* *
 * xmlCatalogResolvePublic:
 * @pubID:  the public ID string
 *
 * Try to lookup the catalog reference associated to a public ID
 *
 * Returns the resource if found or NULL otherwise, the value returned
 *      must be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolvePublic(mut pubID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    ret = xmlACatalogResolvePublic(xmlDefaultCatalog, pubID);
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlCatalogResolve:
 * @pubID:  the public ID string
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier
 *
 * Returns the URI of the resource or NULL if not found, it must be freed
 *      by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolve(mut pubID: *const xmlChar,
                                           mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    ret = xmlACatalogResolve(xmlDefaultCatalog, pubID, sysID);
    return ret;
}
/* *
 * xmlCatalogResolveURI:
 * @URI:  the URI
 *
 * Do a complete resolution lookup of an URI
 *
 * Returns the URI of the resource or NULL if not found, it must be freed
 *      by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolveURI(mut URI: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    ret = xmlACatalogResolveURI(xmlDefaultCatalog, URI);
    return ret;
}
/* *
 * xmlCatalogDump:
 * @out:  the file.
 *
 * Dump all the global catalog content to the given file.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogDump(mut out: *mut FILE) {
    if out.is_null() { return }
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    xmlACatalogDump(xmlDefaultCatalog, out);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlCatalogAdd:
 * @type:  the type of record to add to the catalog
 * @orig:  the system, public or prefix to match
 * @replace:  the replacement value for the match
 *
 * Add an entry in the catalog, it may overwrite existing but
 * different entries.
 * If called before any other catalog routine, allows to override the
 * default shared catalog put in place by xmlInitializeCatalog();
 *
 * Returns 0 if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogAdd(mut type_0: *const xmlChar,
                                       mut orig: *const xmlChar,
                                       mut replace: *const xmlChar)
 -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if xmlCatalogInitialized == 0 { xmlInitializeCatalogData(); }
    xmlRMutexLock(xmlCatalogMutex);
    /*
     * Specific case where one want to override the default catalog
     * put in place by xmlInitializeCatalog();
     */
    if xmlDefaultCatalog.is_null() &&
           xmlStrEqual(type_0,
                       b"catalog\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
        xmlDefaultCatalog =
            xmlCreateNewCatalog(XML_XML_CATALOG_TYPE,
                                xmlCatalogDefaultPrefer);
        (*xmlDefaultCatalog).xml =
            xmlNewCatalogEntry(XML_CATA_CATALOG, 0 as *const xmlChar, orig,
                               0 as *const xmlChar, xmlCatalogDefaultPrefer,
                               0 as xmlCatalogEntryPtr);
        xmlRMutexUnlock(xmlCatalogMutex);
        return 0 as std::os::raw::c_int
    }
    res = xmlACatalogAdd(xmlDefaultCatalog, type_0, orig, replace);
    xmlRMutexUnlock(xmlCatalogMutex);
    return res;
}
/* *
 * xmlCatalogRemove:
 * @value:  the value to remove
 *
 * Remove an entry from the catalog
 *
 * Returns the number of entries removed if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogRemove(mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = 0;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    xmlRMutexLock(xmlCatalogMutex);
    res = xmlACatalogRemove(xmlDefaultCatalog, value);
    xmlRMutexUnlock(xmlCatalogMutex);
    return res;
}
/* *
 * xmlCatalogConvert:
 *
 * Convert all the SGML catalog entries as XML ones
 *
 * Returns the number of entries converted if successful, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogConvert() -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    xmlRMutexLock(xmlCatalogMutex);
    res = xmlConvertSGMLCatalog(xmlDefaultCatalog);
    xmlRMutexUnlock(xmlCatalogMutex);
    return res;
}
/* ***********************************************************************
 *									*
 *	Public interface manipulating the common preferences		*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogGetDefaults:
 *
 * Used to get the user preference w.r.t. to what catalogs should
 * be accepted
 *
 * Returns the current xmlCatalogAllow value
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogGetDefaults() -> xmlCatalogAllow {
    return xmlCatalogDefaultAllow;
}
/* *
 * xmlCatalogSetDefaults:
 * @allow:  what catalogs should be accepted
 *
 * Used to set the user preference w.r.t. to what catalogs should
 * be accepted
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogSetDefaults(mut allow: xmlCatalogAllow) {
    if xmlDebugCatalogs != 0 {
        match allow as std::os::raw::c_uint {
            0 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Disabling catalog usage\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            }
            1 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Allowing only global catalogs\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            }
            2 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Allowing only catalogs from the document\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            }
            3 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Allowing all catalogs\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            }
            _ => { }
        }
    }
    xmlCatalogDefaultAllow = allow;
}
/* *
 * xmlCatalogSetDefaultPrefer:
 * @prefer:  the default preference for delegation
 *
 * Allows to set the preference between public and system for deletion
 * in XML Catalog resolution. C.f. section 4.1.1 of the spec
 * Values accepted are XML_CATA_PREFER_PUBLIC or XML_CATA_PREFER_SYSTEM
 *
 * Returns the previous value of the default preference for delegation
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogSetDefaultPrefer(mut prefer:
                                                        xmlCatalogPrefer)
 -> xmlCatalogPrefer {
    let mut ret: xmlCatalogPrefer = xmlCatalogDefaultPrefer;
    if prefer as std::os::raw::c_uint ==
           XML_CATA_PREFER_NONE as std::os::raw::c_int as std::os::raw::c_uint {
        return ret
    }
    if xmlDebugCatalogs != 0 {
        match prefer as std::os::raw::c_uint {
            1 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Setting catalog preference to PUBLIC\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            }
            2 => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Setting catalog preference to SYSTEM\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char);
            }
            _ => { return ret }
        }
    }
    xmlCatalogDefaultPrefer = prefer;
    return ret;
}
/*
 * Preference settings.
 */
/* *
 * xmlCatalogSetDebug:
 * @level:  the debug level of catalogs required
 *
 * Used to set the debug level for catalog operation, 0 disable
 * debugging, 1 enable it
 *
 * Returns the previous value of the catalog debugging level
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogSetDebug(mut level: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = xmlDebugCatalogs;
    if level <= 0 as std::os::raw::c_int {
        xmlDebugCatalogs = 0 as std::os::raw::c_int
    } else { xmlDebugCatalogs = level }
    return ret;
}
/*
 * Strictly minimal interfaces for per-document catalogs used
 * by the parser.
 */
/* ***********************************************************************
 *									*
 *   Minimal interfaces used for per-document catalogs by the parser	*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogFreeLocal:
 * @catalogs:  a document's list of catalogs
 *
 * Free up the memory associated to the catalog list
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogFreeLocal(mut catalogs:
                                                 *mut std::os::raw::c_void) {
    let mut catal: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    catal = catalogs as xmlCatalogEntryPtr;
    if !catal.is_null() { xmlFreeCatalogEntryList(catal); };
}
/* *
 * xmlCatalogAddLocal:
 * @catalogs:  a document's list of catalogs
 * @URL:  the URL to a new local catalog
 *
 * Add the new entry to the catalog list
 *
 * Returns the updated list
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogAddLocal(mut catalogs: *mut std::os::raw::c_void,
                                            mut URL: *const xmlChar)
 -> *mut std::os::raw::c_void {
    let mut catal: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut add: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    if URL.is_null() { return catalogs }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Adding document catalog %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   URL);
    }
    add =
        xmlNewCatalogEntry(XML_CATA_CATALOG, 0 as *const xmlChar, URL,
                           0 as *const xmlChar, xmlCatalogDefaultPrefer,
                           0 as xmlCatalogEntryPtr);
    if add.is_null() { return catalogs }
    catal = catalogs as xmlCatalogEntryPtr;
    if catal.is_null() { return add as *mut std::os::raw::c_void }
    while !(*catal).next.is_null() { catal = (*catal).next }
    (*catal).next = add;
    return catalogs;
}
/* *
 * xmlCatalogLocalResolve:
 * @catalogs:  a document's list of catalogs
 * @pubID:  the public ID string
 * @sysID:  the system ID string
 *
 * Do a complete resolution lookup of an External Identifier using a
 * document's private catalog list
 *
 * Returns the URI of the resource or NULL if not found, it must be freed
 *      by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogLocalResolve(mut catalogs:
                                                    *mut std::os::raw::c_void,
                                                mut pubID: *const xmlChar,
                                                mut sysID: *const xmlChar)
 -> *mut xmlChar {
    let mut catal: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    if pubID.is_null() && sysID.is_null() { return 0 as *mut xmlChar }
    if xmlDebugCatalogs != 0 {
        if !pubID.is_null() && !sysID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Local Resolve: pubID %s sysID %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pubID,
                                                                       sysID);
        } else if !pubID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Local Resolve: pubID %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       pubID);
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Local Resolve: sysID %s\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       sysID);
        }
    }
    catal = catalogs as xmlCatalogEntryPtr;
    if catal.is_null() { return 0 as *mut xmlChar }
    ret = xmlCatalogListXMLResolve(catal, pubID, sysID);
    if !ret.is_null() && ret != -(1 as std::os::raw::c_int) as *mut xmlChar {
        return ret
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlCatalogLocalResolveURI:
 * @catalogs:  a document's list of catalogs
 * @URI:  the URI
 *
 * Do a complete resolution lookup of an URI using a
 * document's private catalog list
 *
 * Returns the URI of the resource or NULL if not found, it must be freed
 *      by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogLocalResolveURI(mut catalogs:
                                                       *mut std::os::raw::c_void,
                                                   mut URI: *const xmlChar)
 -> *mut xmlChar {
    let mut catal: xmlCatalogEntryPtr = 0 as *mut xmlCatalogEntry;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    if URI.is_null() { return 0 as *mut xmlChar }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Resolve URI %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   URI);
    }
    catal = catalogs as xmlCatalogEntryPtr;
    if catal.is_null() { return 0 as *mut xmlChar }
    ret = xmlCatalogListXMLResolveURI(catal, URI);
    if !ret.is_null() && ret != -(1 as std::os::raw::c_int) as *mut xmlChar {
        return ret
    }
    return 0 as *mut xmlChar;
}
/* DEPRECATED interfaces */
/* ***********************************************************************
 *									*
 *			Deprecated interfaces				*
 *									*
 ************************************************************************/
/* *
 * xmlCatalogGetSystem:
 * @sysID:  the system ID string
 *
 * Try to lookup the catalog reference associated to a system ID
 * DEPRECATED, use xmlCatalogResolveSystem()
 *
 * Returns the resource if found or NULL otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogGetSystem(mut sysID: *const xmlChar)
 -> *const xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    static mut result: [xmlChar; 1000] = [0; 1000];
    static mut msg: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    if msg == 0 as std::os::raw::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Use of deprecated xmlCatalogGetSystem() call\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        msg += 1
    }
    if sysID.is_null() { return 0 as *const xmlChar }
    /*
     * Check first the XML catalogs
     */
    if !xmlDefaultCatalog.is_null() {
        ret =
            xmlCatalogListXMLResolve((*xmlDefaultCatalog).xml,
                                     0 as *const xmlChar, sysID);
        if !ret.is_null() && ret != -(1 as std::os::raw::c_int) as *mut xmlChar {
            snprintf(result.as_mut_ptr() as *mut std::os::raw::c_char,
                     (::std::mem::size_of::<[xmlChar; 1000]>() as
                          std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong),
                     b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                     ret as *mut std::os::raw::c_char);
            result[(::std::mem::size_of::<[xmlChar; 1000]>() as
                        std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                       usize] = 0 as std::os::raw::c_int as xmlChar;
            return result.as_mut_ptr()
        }
    }
    if !xmlDefaultCatalog.is_null() {
        return xmlCatalogGetSGMLSystem((*xmlDefaultCatalog).sgml, sysID)
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlCatalogGetPublic:
 * @pubID:  the public ID string
 *
 * Try to lookup the catalog reference associated to a public ID
 * DEPRECATED, use xmlCatalogResolvePublic()
 *
 * Returns the resource if found or NULL otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogGetPublic(mut pubID: *const xmlChar)
 -> *const xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    static mut result: [xmlChar; 1000] = [0; 1000];
    static mut msg: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if xmlCatalogInitialized == 0 { xmlInitializeCatalog(); }
    if msg == 0 as std::os::raw::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Use of deprecated xmlCatalogGetPublic() call\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        msg += 1
    }
    if pubID.is_null() { return 0 as *const xmlChar }
    /*
     * Check first the XML catalogs
     */
    if !xmlDefaultCatalog.is_null() {
        ret =
            xmlCatalogListXMLResolve((*xmlDefaultCatalog).xml, pubID,
                                     0 as *const xmlChar);
        if !ret.is_null() && ret != -(1 as std::os::raw::c_int) as *mut xmlChar {
            snprintf(result.as_mut_ptr() as *mut std::os::raw::c_char,
                     (::std::mem::size_of::<[xmlChar; 1000]>() as
                          std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong),
                     b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                     ret as *mut std::os::raw::c_char);
            result[(::std::mem::size_of::<[xmlChar; 1000]>() as
                        std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                       usize] = 0 as std::os::raw::c_int as xmlChar;
            return result.as_mut_ptr()
        }
    }
    if !xmlDefaultCatalog.is_null() {
        return xmlCatalogGetSGMLPublic((*xmlDefaultCatalog).sgml, pubID)
    }
    return 0 as *const xmlChar;
}
/* LIBXML_CATALOG_ENABLED */
/* __INCLUDE_ELFGCCHACK */
