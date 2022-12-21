
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    #[no_mangle]
    fn __xmlInitializeDict() -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlResetError(err: xmlErrorPtr);
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlInitializeGlobalState(gs: xmlGlobalStatePtr);
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn pthread_self() -> pthread_t;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                          __mutexattr: *const pthread_mutexattr_t)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_once(__once_control: *mut pthread_once_t,
                    __init_routine: Option<unsafe extern "C" fn() -> ()>)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_cond_init(__cond: *mut pthread_cond_t,
                         __cond_attr: *const pthread_condattr_t)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t,
                         __mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_key_create(__key: *mut pthread_key_t,
                          __destr_function:
                              Option<unsafe extern "C" fn(_:
                                                              *mut std::os::raw::c_void)
                                         -> ()>) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_key_delete(__key: pthread_key_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn pthread_setspecific(__key: pthread_key_t,
                           __pointer: *const std::os::raw::c_void) -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [std::os::raw::c_char; 40],
    pub __align: std::os::raw::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: std::os::raw::c_int,
    pub __count: std::os::raw::c_uint,
    pub __owner: std::os::raw::c_int,
    pub __nusers: std::os::raw::c_uint,
    pub __kind: std::os::raw::c_int,
    pub __spins: std::os::raw::c_short,
    pub __elision: std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
/* __GNUC__, __GLIBC__, __linux__ */
/* __GNUC__, __GLIBC__, __linux__ */
/* HAVE_PTHREAD_H */
/*
 * TODO: this module still uses malloc/free and not xmlMalloc/xmlFree
 *       to avoid some crazyness since xmlMalloc/xmlFree may actually
 *       be hosted on allocated blocks needing them for the allocation ...
 */
/*
 * xmlMutex are a simple mutual exception locks
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
pub type xmlMutex = _xmlMutex;
pub type xmlMutexPtr = *mut xmlMutex;
/*
 * xmlRMutex are reentrant mutual exception locks
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRMutex {
    pub lock: pthread_mutex_t,
    pub held: std::os::raw::c_uint,
    pub waiters: std::os::raw::c_uint,
    pub tid: pthread_t,
    pub cv: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [std::os::raw::c_char; 48],
    pub __align: std::os::raw::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [std::os::raw::c_uint; 2],
    pub __g_size: [std::os::raw::c_uint; 2],
    pub __g1_orig_size: std::os::raw::c_uint,
    pub __wrefs: std::os::raw::c_uint,
    pub __g_signals: [std::os::raw::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: std::os::raw::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: std::os::raw::c_uint,
    pub __high: std::os::raw::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: std::os::raw::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: std::os::raw::c_uint,
    pub __high: std::os::raw::c_uint,
}
pub type pthread_t = std::os::raw::c_ulong;
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
pub type xmlBufPtr = *mut xmlBuf;
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
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
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
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
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
pub type xmlErrorPtr = *mut xmlError;
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
pub type xmlEntity = _xmlEntity;
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
pub type xmlBufferAllocationScheme = std::os::raw::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/*
 * SAX Version 1
 */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [std::os::raw::c_char; 4],
    pub __align: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [std::os::raw::c_char; 4],
    pub __align: std::os::raw::c_int,
}
pub type pthread_key_t = std::os::raw::c_uint;
pub type pthread_once_t = std::os::raw::c_int;
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
/* *
 * xmlMallocFunc:
 * @size:  the size requested in bytes
 *
 * Signature for a malloc() implementation.
 *
 * Returns a pointer to the newly allocated block or NULL in case of error.
 */
pub type xmlMallocFunc
    =
    Option<unsafe extern "C" fn(_: size_t) -> *mut std::os::raw::c_void>;
/* *
 * xmlReallocFunc:
 * @mem: an already allocated block of memory
 * @size:  the new size requested in bytes
 *
 * Signature for a realloc() implementation.
 *
 * Returns a pointer to the newly reallocated block or NULL in case of error.
 */
pub type xmlReallocFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
               -> *mut std::os::raw::c_void>;
/* *
 * xmlStrdupFunc:
 * @str: a zero terminated string
 *
 * Signature for an strdup() implementation.
 *
 * Returns the copy of the string or NULL in case of error.
 */
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
/* *
 * xmlParserInputBufferCreateFilenameFunc:
 * @URI: the URI to read from
 * @enc: the requested source encoding
 *
 * Signature for the function doing the lookup for a suitable input method
 * corresponding to an URI.
 *
 * Returns the new xmlParserInputBufferPtr in case of success or NULL if no
 *         method was found.
 */
pub type xmlParserInputBufferCreateFilenameFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char, _: xmlCharEncoding)
               -> xmlParserInputBufferPtr>;
/* *
 * xmlOutputBufferCreateFilenameFunc:
 * @URI: the URI to write to
 * @enc: the requested target encoding
 *
 * Signature for the function doing the lookup for a suitable output method
 * corresponding to an URI.
 *
 * Returns the new xmlOutputBufferPtr in case of success or NULL if no
 *         method was found.
 */
pub type xmlOutputBufferCreateFilenameFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: xmlCharEncodingHandlerPtr, _: std::os::raw::c_int)
               -> xmlOutputBufferPtr>;
/*
 * Externally global symbols which need to be protected for backwards
 * compatibility support.
 */
/* *
 * xmlRegisterNodeFunc:
 * @node: the current node
 *
 * Signature for the registration callback of a created node
 */
pub type xmlRegisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
/* *
 * xmlDeregisterNodeFunc:
 * @node: the current node
 *
 * Signature for the deregistration callback of a discarded node
 */
pub type xmlDeregisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlGlobalState {
    pub xmlParserVersion: *const std::os::raw::c_char,
    pub xmlDefaultSAXLocator: xmlSAXLocator,
    pub xmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub docbDefaultSAXHandler: xmlSAXHandlerV1,
    pub htmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub xmlFree: xmlFreeFunc,
    pub xmlMalloc: xmlMallocFunc,
    pub xmlMemStrdup: xmlStrdupFunc,
    pub xmlRealloc: xmlReallocFunc,
    pub xmlGenericError: xmlGenericErrorFunc,
    pub xmlStructuredError: xmlStructuredErrorFunc,
    pub xmlGenericErrorContext: *mut std::os::raw::c_void,
    pub oldXMLWDcompatibility: std::os::raw::c_int,
    pub xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub xmlDefaultBufferSize: std::os::raw::c_int,
    pub xmlSubstituteEntitiesDefaultValue: std::os::raw::c_int,
    pub xmlDoValidityCheckingDefaultValue: std::os::raw::c_int,
    pub xmlGetWarningsDefaultValue: std::os::raw::c_int,
    pub xmlKeepBlanksDefaultValue: std::os::raw::c_int,
    pub xmlLineNumbersDefaultValue: std::os::raw::c_int,
    pub xmlLoadExtDtdDefaultValue: std::os::raw::c_int,
    pub xmlParserDebugEntities: std::os::raw::c_int,
    pub xmlPedanticParserDefaultValue: std::os::raw::c_int,
    pub xmlSaveNoEmptyTags: std::os::raw::c_int,
    pub xmlIndentTreeOutput: std::os::raw::c_int,
    pub xmlTreeIndentString: *const std::os::raw::c_char,
    pub xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc,
    pub xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc,
    pub xmlMallocAtomic: xmlMallocFunc,
    pub xmlLastError: xmlError,
    pub xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub xmlStructuredErrorContext: *mut std::os::raw::c_void,
}
pub type xmlGlobalState = _xmlGlobalState;
pub type xmlGlobalStatePtr = *mut xmlGlobalState;
pub type C2RustUnnamed_3 = std::os::raw::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
#[inline]
unsafe extern "C" fn pthread_equal(mut __thread1: pthread_t,
                                   mut __thread2: pthread_t) -> std::os::raw::c_int {
    return (__thread1 == __thread2) as std::os::raw::c_int;
}
/* *
 * threads.c: set of generic threading related routines
 *
 * See Copyright for the status of this software.
 *
 * Gary Pennington <Gary.Pennington@uk.sun.com>
 * daniel@veillard.com
 */
/* #define DEBUG_THREADS */
static mut libxml_is_threaded: std::os::raw::c_int = -(1 as std::os::raw::c_int);
/*
 * This module still has some internal static data.
 *   - xmlLibraryLock a global lock
 *   - globalkey used for per-thread data
 */
static mut globalkey: pthread_key_t = 0;
static mut mainthread: pthread_t = 0;
static mut once_control: pthread_once_t = 0 as std::os::raw::c_int;
static mut once_control_init: pthread_once_t = 0 as std::os::raw::c_int;
static mut global_init_lock: pthread_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as std::os::raw::c_int,
                                                  __count:
                                                      0 as std::os::raw::c_int as
                                                          std::os::raw::c_uint,
                                                  __owner: 0 as std::os::raw::c_int,
                                                  __nusers:
                                                      0 as std::os::raw::c_int as
                                                          std::os::raw::c_uint,
                                                  __kind:
                                                      PTHREAD_MUTEX_TIMED_NP
                                                          as std::os::raw::c_int,
                                                  __spins:
                                                      0 as std::os::raw::c_int as
                                                          std::os::raw::c_short,
                                                  __elision:
                                                      0 as std::os::raw::c_int as
                                                          std::os::raw::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
static mut xmlLibraryLock: xmlRMutexPtr =
    0 as *const xmlRMutex as xmlRMutexPtr;
/* *
 * xmlNewMutex:
 *
 * xmlNewMutex() is used to allocate a libxml2 token struct for use in
 * synchronizing access to data.
 *
 * Returns a new simple mutex pointer or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewMutex() -> xmlMutexPtr {
    let mut tok: xmlMutexPtr = 0 as *mut xmlMutex;
    tok =
        malloc(::std::mem::size_of::<xmlMutex>() as std::os::raw::c_ulong) as
            xmlMutexPtr;
    if tok.is_null() { return 0 as xmlMutexPtr }
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_mutex_init(&mut (*tok).lock, 0 as *const pthread_mutexattr_t);
    }
    return tok;
}
/* *
 * xmlFreeMutex:
 * @tok:  the simple mutex
 *
 * xmlFreeMutex() is used to reclaim resources associated with a libxml2 token
 * struct.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeMutex(mut tok: xmlMutexPtr) {
    if tok.is_null() { return }
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_mutex_destroy(&mut (*tok).lock);
    }
    free(tok as *mut std::os::raw::c_void);
}
/* *
 * xmlMutexLock:
 * @tok:  the simple mutex
 *
 * xmlMutexLock() is used to lock a libxml2 token.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMutexLock(mut tok: xmlMutexPtr) {
    if tok.is_null() { return }
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_mutex_lock(&mut (*tok).lock);
    };
}
/* *
 * xmlMutexUnlock:
 * @tok:  the simple mutex
 *
 * xmlMutexUnlock() is used to unlock a libxml2 token.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlMutexUnlock(mut tok: xmlMutexPtr) {
    if tok.is_null() { return }
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_mutex_unlock(&mut (*tok).lock);
    };
}
/* *
 * xmlNewRMutex:
 *
 * xmlRNewMutex() is used to allocate a reentrant mutex for use in
 * synchronizing access to data. token_r is a re-entrant lock and thus useful
 * for synchronizing access to data structures that may be manipulated in a
 * recursive fashion.
 *
 * Returns the new reentrant mutex pointer or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewRMutex() -> xmlRMutexPtr {
    let mut tok: xmlRMutexPtr = 0 as *mut xmlRMutex;
    tok =
        malloc(::std::mem::size_of::<xmlRMutex>() as std::os::raw::c_ulong) as
            xmlRMutexPtr;
    if tok.is_null() { return 0 as xmlRMutexPtr }
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_mutex_init(&mut (*tok).lock, 0 as *const pthread_mutexattr_t);
        (*tok).held = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        (*tok).waiters = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        pthread_cond_init(&mut (*tok).cv, 0 as *const pthread_condattr_t);
    }
    return tok;
}
/* *
 * xmlFreeRMutex:
 * @tok:  the reentrant mutex
 *
 * xmlRFreeMutex() is used to reclaim resources associated with a
 * reentrant mutex.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeRMutex(mut tok: xmlRMutexPtr) {
    if tok.is_null() { return }
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_mutex_destroy(&mut (*tok).lock);
        pthread_cond_destroy(&mut (*tok).cv);
    }
    free(tok as *mut std::os::raw::c_void);
}
/* *
 * xmlRMutexLock:
 * @tok:  the reentrant mutex
 *
 * xmlRMutexLock() is used to lock a libxml2 token_r.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRMutexLock(mut tok: xmlRMutexPtr) {
    if tok.is_null() { return }
    if libxml_is_threaded == 0 as std::os::raw::c_int { return }
    pthread_mutex_lock(&mut (*tok).lock);
    if (*tok).held != 0 {
        if pthread_equal((*tok).tid, pthread_self()) != 0 {
            (*tok).held = (*tok).held.wrapping_add(1);
            pthread_mutex_unlock(&mut (*tok).lock);
            return
        } else {
            (*tok).waiters = (*tok).waiters.wrapping_add(1);
            while (*tok).held != 0 {
                pthread_cond_wait(&mut (*tok).cv, &mut (*tok).lock);
            }
            (*tok).waiters = (*tok).waiters.wrapping_sub(1)
        }
    }
    (*tok).tid = pthread_self();
    (*tok).held = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    pthread_mutex_unlock(&mut (*tok).lock);
}
/* *
 * xmlRMutexUnlock:
 * @tok:  the reentrant mutex
 *
 * xmlRMutexUnlock() is used to unlock a libxml2 token_r.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRMutexUnlock(mut tok: xmlRMutexPtr) {
    if tok.is_null() { return }
    if libxml_is_threaded == 0 as std::os::raw::c_int { return }
    pthread_mutex_lock(&mut (*tok).lock);
    (*tok).held = (*tok).held.wrapping_sub(1);
    if (*tok).held == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*tok).waiters != 0 { pthread_cond_signal(&mut (*tok).cv); }
        memset(&mut (*tok).tid as *mut pthread_t as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<pthread_t>() as std::os::raw::c_ulong);
    }
    pthread_mutex_unlock(&mut (*tok).lock);
}
/* *
 * xmlGlobalInitMutexLock
 *
 * Makes sure that the global initialization mutex is initialized and
 * locks it.
 */
#[no_mangle]
pub unsafe extern "C" fn __xmlGlobalInitMutexLock() {
    /* Make sure the global init lock is initialized and then lock it. */
    /* The mutex is statically initialized, so we just lock it. */
    if Some(pthread_mutex_lock as
                unsafe extern "C" fn(_: *mut pthread_mutex_t)
                    -> std::os::raw::c_int).is_none() {
        return
    }
    /* XML_PTHREAD_WEAK */
    pthread_mutex_lock(&mut global_init_lock);
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGlobalInitMutexUnlock() {
    if Some(pthread_mutex_unlock as
                unsafe extern "C" fn(_: *mut pthread_mutex_t)
                    -> std::os::raw::c_int).is_none() {
        return
    }
    /* XML_PTHREAD_WEAK */
    pthread_mutex_unlock(&mut global_init_lock);
}
/* *
 * xmlGlobalInitMutexDestroy
 *
 * Makes sure that the global initialization mutex is destroyed before
 * application termination.
 */
#[no_mangle]
pub unsafe extern "C" fn __xmlGlobalInitMutexDestroy() { }
/* ***********************************************************************
 *									*
 *			Per thread global state handling		*
 *									*
 ************************************************************************/
/* *
 * xmlFreeGlobalState:
 * @state:  a thread global state
 *
 * xmlFreeGlobalState() is called when a thread terminates with a non-NULL
 * global state. It is is used here to reclaim memory resources.
 */
unsafe extern "C" fn xmlFreeGlobalState(mut state: *mut std::os::raw::c_void) {
    let mut gs: *mut xmlGlobalState = state as *mut xmlGlobalState;
    /* free any memory allocated in the thread's xmlLastError */
    xmlResetError(&mut (*gs).xmlLastError);
    free(state);
}
/* *
 * xmlNewGlobalState:
 *
 * xmlNewGlobalState() allocates a global state. This structure is used to
 * hold all data for use by a thread when supporting backwards compatibility
 * of libxml2 to pre-thread-safe behaviour.
 *
 * Returns the newly allocated xmlGlobalStatePtr or NULL in case of error
 */
unsafe extern "C" fn xmlNewGlobalState() -> xmlGlobalStatePtr {
    let mut gs: *mut xmlGlobalState = 0 as *mut xmlGlobalState;
    gs =
        malloc(::std::mem::size_of::<xmlGlobalState>() as std::os::raw::c_ulong) as
            *mut xmlGlobalState;
    if gs.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlGetGlobalState: out of memory\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return 0 as xmlGlobalStatePtr
    }
    memset(gs as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlGlobalState>() as std::os::raw::c_ulong);
    xmlInitializeGlobalState(gs);
    return gs;
}
/* LIBXML_THREAD_ENABLED */
/* HAVE_WIN32_THREADS */
/* *
 * xmlGetGlobalState:
 *
 * xmlGetGlobalState() is called to retrieve the global state for a thread.
 *
 * Returns the thread global state or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetGlobalState() -> xmlGlobalStatePtr {
    let mut globalval: *mut xmlGlobalState = 0 as *mut xmlGlobalState;
    if libxml_is_threaded == 0 as std::os::raw::c_int {
        return 0 as xmlGlobalStatePtr
    }
    pthread_once(&mut once_control,
                 Some(xmlOnceInit as unsafe extern "C" fn() -> ()));
    globalval = pthread_getspecific(globalkey) as *mut xmlGlobalState;
    if globalval.is_null() {
        let mut tsd: *mut xmlGlobalState = xmlNewGlobalState();
        if tsd.is_null() { return 0 as xmlGlobalStatePtr }
        pthread_setspecific(globalkey, tsd as *const std::os::raw::c_void);
        return tsd
    }
    return globalval;
}
/* ***********************************************************************
 *									*
 *			Library wide thread interfaces			*
 *									*
 ************************************************************************/
/* *
 * xmlGetThreadId:
 *
 * xmlGetThreadId() find the current thread ID number
 * Note that this is likely to be broken on some platforms using pthreads
 * as the specification doesn't mandate pthread_t to be an integer type
 *
 * Returns the current thread ID number
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetThreadId() -> std::os::raw::c_int {
    let mut id: pthread_t = 0;
    let mut ret: std::os::raw::c_int = 0;
    if libxml_is_threaded == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    id = pthread_self();
    /* horrible but preserves compat, see warning above */
    memcpy(&mut ret as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
           &mut id as *mut pthread_t as *const std::os::raw::c_void,
           ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong);
    return ret;
}
/* *
 * xmlIsMainThread:
 *
 * xmlIsMainThread() check whether the current thread is the main thread.
 *
 * Returns 1 if the current thread is the main thread, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIsMainThread() -> std::os::raw::c_int {
    if libxml_is_threaded == -(1 as std::os::raw::c_int) { xmlInitThreads(); }
    if libxml_is_threaded == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    pthread_once(&mut once_control,
                 Some(xmlOnceInit as unsafe extern "C" fn() -> ()));
    return pthread_equal(mainthread, pthread_self());
}
/* *
 * xmlLockLibrary:
 *
 * xmlLockLibrary() is used to take out a re-entrant lock on the libxml2
 * library.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLockLibrary() { xmlRMutexLock(xmlLibraryLock); }
/* *
 * xmlUnlockLibrary:
 *
 * xmlUnlockLibrary() is used to release a re-entrant lock on the libxml2
 * library.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlUnlockLibrary() {
    xmlRMutexUnlock(xmlLibraryLock);
}
/* *
 * xmlInitThreads:
 *
 * xmlInitThreads() is used to to initialize all the thread related
 * data of the libxml2 library.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitThreads() {
    if libxml_is_threaded == -(1 as std::os::raw::c_int) {
        if Some(pthread_once as
                    unsafe extern "C" fn(_: *mut pthread_once_t,
                                         _:
                                             Option<unsafe extern "C" fn()
                                                        -> ()>)
                        -> std::os::raw::c_int).is_some() &&
               Some(pthread_getspecific as
                        unsafe extern "C" fn(_: pthread_key_t)
                            -> *mut std::os::raw::c_void).is_some() &&
               Some(pthread_setspecific as
                        unsafe extern "C" fn(_: pthread_key_t,
                                             _: *const std::os::raw::c_void)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_key_create as
                        unsafe extern "C" fn(_: *mut pthread_key_t,
                                             _:
                                                 Option<unsafe extern "C" fn(_:
                                                                                 *mut std::os::raw::c_void)
                                                            -> ()>)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_key_delete as
                        unsafe extern "C" fn(_: pthread_key_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_mutex_init as
                        unsafe extern "C" fn(_: *mut pthread_mutex_t,
                                             _: *const pthread_mutexattr_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_mutex_destroy as
                        unsafe extern "C" fn(_: *mut pthread_mutex_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_mutex_lock as
                        unsafe extern "C" fn(_: *mut pthread_mutex_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_mutex_unlock as
                        unsafe extern "C" fn(_: *mut pthread_mutex_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_cond_init as
                        unsafe extern "C" fn(_: *mut pthread_cond_t,
                                             _: *const pthread_condattr_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_cond_destroy as
                        unsafe extern "C" fn(_: *mut pthread_cond_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_cond_wait as
                        unsafe extern "C" fn(_: *mut pthread_cond_t,
                                             _: *mut pthread_mutex_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_equal as
                        unsafe extern "C" fn(_: pthread_t, _: pthread_t)
                            -> std::os::raw::c_int).is_some() &&
               Some(pthread_self as
                        unsafe extern "C" fn() -> pthread_t).is_some() &&
               Some(pthread_cond_signal as
                        unsafe extern "C" fn(_: *mut pthread_cond_t)
                            -> std::os::raw::c_int).is_some() {
            libxml_is_threaded = 1 as std::os::raw::c_int
            /* fprintf(stderr, "Running multithreaded\n"); */
        } else {
            /* fprintf(stderr, "Running without multithread\n"); */
            libxml_is_threaded = 0 as std::os::raw::c_int
        }
    };
    /* XML_PTHREAD_WEAK */
}
/*
 * Library wide APIs.
 */
/* *
 * xmlCleanupThreads:
 *
 * xmlCleanupThreads() is used to to cleanup all the thread related
 * data of the libxml2 library once processing has ended.
 *
 * WARNING: if your application is multithreaded or has plugin support
 *          calling this may crash the application if another thread or
 *          a plugin is still using libxml2. It's sometimes very hard to
 *          guess if libxml2 is in use in the application, some libraries
 *          or plugins may use it without notice. In case of doubt abstain
 *          from calling this function or do it just before calling exit()
 *          to avoid leak reports from valgrind !
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupThreads() {
    if libxml_is_threaded != 0 as std::os::raw::c_int {
        pthread_key_delete(globalkey);
    }
    once_control = once_control_init;
}
/* *
 * xmlOnceInit
 *
 * xmlOnceInit() is used to initialize the value of mainthread for use
 * in other routines. This function should only be called using
 * pthread_once() in association with the once_control variable to ensure
 * that the function is only called once. See man pthread_once for more
 * details.
 */
unsafe extern "C" fn xmlOnceInit() {
    pthread_key_create(&mut globalkey,
                       Some(xmlFreeGlobalState as
                                unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                    -> ()));
    mainthread = pthread_self();
    __xmlInitializeDict();
}
/* __INCLUDE_ELFGCCHACK */
/* *
 * DllMain:
 * @hinstDLL: handle to DLL instance
 * @fdwReason: Reason code for entry
 * @lpvReserved: generic pointer (depends upon reason code)
 *
 * Entry point for Windows library. It is being used to free thread-specific
 * storage.
 *
 * Returns TRUE always
 */
