
extern "C" {
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
    fn xmlGetUTF8Char(utf: *const std::os::raw::c_uchar, len: *mut std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memmove(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __errno_location() -> *mut std::os::raw::c_int;
    /*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    #[no_mangle]
    fn xmlBufferShrink(buf: xmlBufferPtr, len: std::os::raw::c_uint) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufferGrow(buf: xmlBufferPtr, len: std::os::raw::c_uint) -> std::os::raw::c_int;
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
    fn __xmlSimpleError(domain: std::os::raw::c_int, code: std::os::raw::c_int,
                        node: xmlNodePtr, msg: *const std::os::raw::c_char,
                        extra: *const std::os::raw::c_char);
    #[no_mangle]
    fn iconv(__cd: iconv_t, __inbuf: *mut *mut std::os::raw::c_char,
             __inbytesleft: *mut size_t, __outbuf: *mut *mut std::os::raw::c_char,
             __outbytesleft: *mut size_t) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> std::os::raw::c_int;
    /* * DOC_DISABLE */
/*
 * In general the memory allocation entry points are not kept
 * thread specific but this can be overridden by LIBXML_THREAD_ALLOC_ENABLED
 *    - xmlMalloc
 *    - xmlMallocAtomic
 *    - xmlRealloc
 *    - xmlMemStrdup
 *    - xmlFree
 */
    /* !LIBXML_THREAD_ALLOC_ENABLED */
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    static mut xmlMemStrdup: xmlStrdupFunc;
    #[no_mangle]
    fn iconv_open(__tocode: *const std::os::raw::c_char,
                  __fromcode: *const std::os::raw::c_char) -> iconv_t;
    #[no_mangle]
    fn UTF8ToHtml(out: *mut std::os::raw::c_uchar, outlen: *mut std::os::raw::c_int,
                  in_0: *const std::os::raw::c_uchar, inlen: *mut std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufGetAllocationScheme(buf: xmlBufPtr) -> std::os::raw::c_int;
    /* size_t xmlBufShrink(xmlBufPtr buf, size_t len); */
    #[no_mangle]
    fn xmlBufGrow(buf: xmlBufPtr, len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufAvail(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlBufAddLen(buf: xmlBufPtr, len: size_t) -> std::os::raw::c_int;
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
pub type __int32_t = std::os::raw::c_int;
pub type iconv_t = *mut std::os::raw::c_void;
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
pub type xmlCharEncodingOutputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
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
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
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
/* *
 * endElementNsSAX2Func:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 *
 * SAX2 callback when an element end has been detected by the parser.
 * It provides the namespace informations for the element.
 */
pub type endElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
/* ***********************************************************************
 *									*
 *			The SAX version 2 API extensions		*
 *									*
 ************************************************************************/
/* *
 * XML_SAX2_MAGIC:
 *
 * Special constant found in SAX2 blocks initialized fields
 */
/* *
 * startElementNsSAX2Func:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 * @nb_namespaces:  number of namespace definitions on that node
 * @namespaces:  pointer to the array of prefix/URI pairs namespace definitions
 * @nb_attributes:  the number of attributes on that node
 * @nb_defaulted:  the number of defaulted attributes. The defaulted
 *                  ones are at the end of the array
 * @attributes:  pointer to the array of (localname/prefix/URI/value/end)
 *               attribute values.
 *
 * SAX2 callback when an element start has been detected by the parser.
 * It provides the namespace informations for the element, as well as
 * the new namespace declarations on the element.
 */
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
/*
 * Summary: the core parser module
 * Description: Interfaces, constants and types related to the XML parser
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * XML_DEFAULT_VERSION:
 *
 * The default version of XML used: 1.0
 */
/* *
 * xmlParserInput:
 *
 * An xmlParserInput is an input flow for the XML processor.
 * Each entity parsed is associated an xmlParserInput (except the
 * few predefined ones). This is the case both for internal entities
 * - in which case the flow is already completely in memory - or
 * external entities - in which case we use the buf structure for
 * progressive reading and I18N conversions to the internal UTF-8 format.
 */
/* *
 * xmlParserInputDeallocate:
 * @str:  the string to deallocate
 *
 * Callback for freeing some parser input allocations.
 */
/* Input buffer */
/* UTF-8 encoded buffer */
/* The file analyzed, if any */
/* the directory/base of the file */
/* Base of the array to parse */
/* Current char being parsed */
/* end of the array to parse */
/* length if known */
/* Current line */
/* Current column */
/*
     * NOTE: consumed is only tested for equality in the parser code,
     *       so even if there is an overflow this should not give troubles
     *       for parsing very large instances.
     */
/* How many xmlChars already consumed */
/* function to deallocate the base */
/* the encoding string for entity */
/* the version string for entity */
/* Was that entity marked standalone */
/* an unique identifier for the entity */
/* *
 * xmlParserNodeInfo:
 *
 * The parser can be asked to collect Node informations, i.e. at what
 * place in the file they were detected.
 * NOTE: This is off by default and not very well tested.
 */
/* Position & line # that text that created the node begins & ends on */
/* *
 * xmlParserInputState:
 *
 * The parser is now working also as a state based parser.
 * The recursive one use the state info for entities processing.
 */
/* nothing is to be parsed */
/* nothing has been parsed */
/* Misc* before int subset */
/* Within a processing instruction */
/* within some DTD content */
/* Misc* after internal subset */
/* within a comment */
/* within a start tag */
/* within the content */
/* within a CDATA section */
/* within a closing tag */
/* within an entity declaration */
/* within an entity value in a decl */
/* within an attribute value */
/* within a SYSTEM value */
/* the Misc* after the last end tag */
/* within an IGNORED section */
/* within a PUBLIC value */
/* *
 * XML_DETECT_IDS:
 *
 * Bit in the loadsubset context field to tell to do ID/REFs lookups.
 * Use it to initialize xmlLoadExtDtdDefaultValue.
 */
/* *
 * XML_COMPLETE_ATTRS:
 *
 * Bit in the loadsubset context field to tell to do complete the
 * elements attributes lists with the ones defaulted from the DTDs.
 * Use it to initialize xmlLoadExtDtdDefaultValue.
 */
/* *
 * XML_SKIP_IDS:
 *
 * Bit in the loadsubset context field to tell to not do ID/REFs registration.
 * Used to initialize xmlLoadExtDtdDefaultValue in some special cases.
 */
/* *
 * xmlParserMode:
 *
 * A parser can operate in various modes
 */
/* *
 * xmlParserCtxt:
 *
 * The parser context.
 * NOTE This doesn't completely define the parser state, the (current ?)
 *      design of the parser uses recursive function calls since this allow
 *      and easy mapping from the production rules of the specification
 *      to the actual code. The drawback is that the actual function call
 *      also reflect the parser state. However most of the parsing routines
 *      takes as the only argument the parser context pointer, so migrating
 *      to a state based parser for progressive parsing shouldn't be too hard.
 */
/* The SAX handler */
/* For SAX interface only, used by DOM build */
/* the document being built */
/* is the document well formed */
/* shall we replace entities ? */
/* the XML version string */
/* the declared encoding, if any */
/* standalone document */
/* an HTML(1)/Docbook(2) document
                                       * 3 is HTML after <head>
                                       * 10 is HTML after <body>
                                       */
/* Input stream stack */
/* Current input stream */
/* Number of current input streams */
/* Max number of input streams */
/* stack of inputs */
/* Node analysis stack only used for DOM building */
/* Current parsed Node */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodes */
/* Whether node info should be kept */
/* info about each node parsed */
/* error code */
/* reference and external subset */
/* the internal subset has PE refs */
/* are we parsing an external entity */
/* is the document valid */
/* shall we try to validate ? */
/* The validity context */
/* current type of input */
/* next char look-ahead */
/* the data directory */
/* Node name stack */
/* Current parsed Node */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodes */
/* number of xmlChar processed */
/* used by progressive parsing lookup */
/* ugly but ... */
/* SAX callbacks are disabled */
/* Parsing is in int 1/ext 2 subset */
/* name of subset */
/* URI of external subset */
/* SYSTEM ID of external subset */
/* xml:space values */
/* Should the parser preserve spaces */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of space infos */
/* to prevent entity substitution loops */
/* used to check entities boundaries */
/* encoding of the in-memory content
				         actually an xmlCharEncoding */
/* Those two fields are there to */
/* Speed up large node parsing */
/* signal pedantic warnings */
/* For user data, libxml won't touch it */
/* should the external subset be loaded */
/* set line number in element content */
/* document's own catalog */
/* run in recovery mode */
/* is this a progressive parsing */
/* dictionary for the parser */
/* array for the attributes callbacks */
/* the size of the array */
/* use strings from dict to build tree */
/*
     * pre-interned strings
     */
/*
     * Everything below is used only by the new SAX mode
     */
/* operating in the new SAX mode */
/* the number of inherited namespaces */
/* the size of the arrays */
/* the array of prefix/namespace name */
/* which attribute were allocated */
/* array of data for push */
/* defaulted attributes if any */
/* non-CDATA attributes if any */
/* is the document XML Nanespace okay */
/* Extra options */
/*
     * Those fields are needed only for treaming parsing so far
     */
/* Use dictionary names for the tree */
/* number of freed element nodes */
/* List of freed element nodes */
/* number of freed attributes nodes */
/* List of freed attributes nodes */
/*
     * the complete error informations for the last error.
     */
/* the parser mode */
/* number of entities references */
/* size of parsed entities */
/* for use by HTML non-recursive parser */
/* Current NodeInfo */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodeInfos */
/* we need to label inputs */
/* volume of entity copy */
/* *
 * xmlSAXLocator:
 *
 * A SAX Locator.
 */
/* *
 * xmlSAXHandler:
 *
 * A SAX handler is bunch of callbacks called by the parser when processing
 * of the input generate data or structure informations.
 */
/* *
 * resolveEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * Callback:
 * The entity loader, to control the loading of external entities,
 * the application can either:
 *    - override this resolveEntity() callback in the SAX block
 *    - or better use the xmlSetExternalEntityLoader() function to
 *      set up it's own entity resolution routine
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
/* *
 * internalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on internal subset declaration.
 */
/* *
 * externalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on external subset declaration.
 */
/* *
 * getEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The entity name
 *
 * Get an entity by name.
 *
 * Returns the xmlEntityPtr if found.
 */
/* *
 * getParameterEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The entity name
 *
 * Get a parameter entity by name.
 *
 * Returns the xmlEntityPtr if found.
 */
/* *
 * entityDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the entity name
 * @type:  the entity type
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @content: the entity value (without processing).
 *
 * An entity definition has been parsed.
 */
/* *
 * notationDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The name of the notation
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * What to do when a notation declaration has been parsed.
 */
/* *
 * attributeDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @elem:  the name of the element
 * @fullname:  the attribute name
 * @type:  the attribute type
 * @def:  the type of default value
 * @defaultValue: the attribute default value
 * @tree:  the tree of enumerated value set
 *
 * An attribute definition has been parsed.
 */
/* *
 * elementDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the element name
 * @type:  the element type
 * @content: the element value tree
 *
 * An element definition has been parsed.
 */
/* *
 * unparsedEntityDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The name of the entity
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @notationName: the name of the notation
 *
 * What to do when an unparsed entity declaration is parsed.
 */
/* *
 * setDocumentLocatorSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @loc: A SAX Locator
 *
 * Receive the document locator at startup, actually xmlDefaultSAXLocator.
 * Everything is available on the context, so this is useless in our case.
 */
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
/* *
 * startDocumentSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Called when the document start being processed.
 */
/* *
 * endDocumentSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Called when the document end has been detected.
 */
/* *
 * startElementSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The element name, including namespace prefix
 * @atts:  An array of name/value attributes pairs, NULL terminated
 *
 * Called when an opening tag has been processed.
 */
/* *
 * endElementSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The element name
 *
 * Called when the end of an element has been detected.
 */
/* *
 * attributeSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The attribute name, including namespace prefix
 * @value:  The attribute value
 *
 * Handle an attribute that has been read by the parser.
 * The default handling is to convert the attribute into an
 * DOM subtree and past it in a new xmlAttr element added to
 * the element.
 */
/* *
 * referenceSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The entity name
 *
 * Called when an entity reference is detected.
 */
/* *
 * charactersSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * Receiving some chars from the parser.
 */
/* *
 * ignorableWhitespaceSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * Receiving some ignorable whitespaces from the parser.
 * UNUSED: by default the DOM building will use characters.
 */
/* *
 * processingInstructionSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @target:  the target name
 * @data: the PI data's
 *
 * A processing instruction has been parsed.
 */
/* *
 * commentSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @value:  the comment content
 *
 * A comment has been parsed.
 */
/* *
 * cdataBlockSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @value:  The pcdata content
 * @len:  the block length
 *
 * Called when a pcdata block has been parsed.
 */
/* *
 * warningSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, callback.
 */
/* *
 * errorSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format an error messages, callback.
 */
/* *
 * fatalErrorSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format fatal error messages, callback.
 * Note: so far fatalError() SAX callbacks are not used, error()
 *       get all the callbacks for errors.
 */
/* *
 * isStandaloneSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Is this document tagged standalone?
 *
 * Returns 1 if true
 */
/* *
 * hasInternalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Does this document has an internal subset.
 *
 * Returns 1 if true
 */
/* *
 * hasExternalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Does this document has an external subset?
 *
 * Returns 1 if true
 */
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
/* *
 * xmlBuffer:
 *
 * A buffer structure, this old construct is limited to 2GB and
 * is being deprecated, use API with xmlBuf instead
 */
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
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
pub type xmlParserErrors = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub const XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const XML_IO_ESRCH: xmlParserErrors = 1540;
pub const XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const XML_IO_EROFS: xmlParserErrors = 1538;
pub const XML_IO_ERANGE: xmlParserErrors = 1537;
pub const XML_IO_EPIPE: xmlParserErrors = 1536;
pub const XML_IO_EPERM: xmlParserErrors = 1535;
pub const XML_IO_ENXIO: xmlParserErrors = 1534;
pub const XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const XML_IO_ENOENT: xmlParserErrors = 1524;
pub const XML_IO_ENODEV: xmlParserErrors = 1523;
pub const XML_IO_ENFILE: xmlParserErrors = 1522;
pub const XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const XML_IO_EMLINK: xmlParserErrors = 1519;
pub const XML_IO_EMFILE: xmlParserErrors = 1518;
pub const XML_IO_EISDIR: xmlParserErrors = 1517;
pub const XML_IO_EIO: xmlParserErrors = 1516;
pub const XML_IO_EINVAL: xmlParserErrors = 1515;
pub const XML_IO_EINTR: xmlParserErrors = 1514;
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const XML_IO_EFBIG: xmlParserErrors = 1512;
pub const XML_IO_EFAULT: xmlParserErrors = 1511;
pub const XML_IO_EEXIST: xmlParserErrors = 1510;
pub const XML_IO_EDOM: xmlParserErrors = 1509;
pub const XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const XML_IO_ECHILD: xmlParserErrors = 1507;
pub const XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const XML_IO_EBUSY: xmlParserErrors = 1505;
pub const XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const XML_IO_EBADF: xmlParserErrors = 1503;
pub const XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const XML_IO_EACCES: xmlParserErrors = 1501;
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const XML_WAR_NS_URI: xmlParserErrors = 99;
pub const XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlCharEncodingAlias = _xmlCharEncodingAlias;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingAlias {
    pub name: *const std::os::raw::c_char,
    pub alias: *const std::os::raw::c_char,
}
pub type xmlCharEncodingAliasPtr = *mut xmlCharEncodingAlias;
#[inline]
unsafe extern "C" fn toupper(mut __c: std::os::raw::c_int) -> std::os::raw::c_int {
    return if __c >= -(128 as std::os::raw::c_int) && __c < 256 as std::os::raw::c_int {
               *(*__ctype_toupper_loc()).offset(__c as isize)
           } else { __c };
}
/*
 * encoding.c : implements the encoding conversion functions needed for XML
 *
 * Related specs:
 * rfc2044        (UTF-8 and UTF-16) F. Yergeau Alis Technologies
 * rfc2781        UTF-16, an encoding of ISO 10646, P. Hoffman, F. Yergeau
 * [ISO-10646]    UTF-8 and UTF-16 in Annexes
 * [ISO-8859-1]   ISO Latin-1 characters codes.
 * [UNICODE]      The Unicode Consortium, "The Unicode Standard --
 *                Worldwide Character Encoding -- Version 1.0", Addison-
 *                Wesley, Volume 1, 1991, Volume 2, 1992.  UTF-8 is
 *                described in Unicode Technical Report #4.
 * [US-ASCII]     Coded Character Set--7-bit American Standard Code for
 *                Information Interchange, ANSI X3.4-1986.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 *
 * Original code for IsoLatin1 and UTF-16 by "Martin J. Duerst" <duerst@w3.org>
 */
static mut xmlUTF16LEHandler: xmlCharEncodingHandlerPtr =
    0 as *const xmlCharEncodingHandler as xmlCharEncodingHandlerPtr;
static mut xmlUTF16BEHandler: xmlCharEncodingHandlerPtr =
    0 as *const xmlCharEncodingHandler as xmlCharEncodingHandlerPtr;
static mut xmlCharEncodingAliases: xmlCharEncodingAliasPtr =
    0 as *const xmlCharEncodingAlias as xmlCharEncodingAliasPtr;
static mut xmlCharEncodingAliasesNb: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlCharEncodingAliasesMax: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlLittleEndian: std::os::raw::c_int = 1 as std::os::raw::c_int;
/* *
 * xmlEncodingErrMemory:
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlEncodingErrMemory(mut extra: *const std::os::raw::c_char) {
    __xmlSimpleError(XML_FROM_I18N as std::os::raw::c_int,
                     XML_ERR_NO_MEMORY as std::os::raw::c_int, 0 as xmlNodePtr,
                     0 as *const std::os::raw::c_char, extra);
}
/* *
 * xmlErrEncoding:
 * @error:  the error number
 * @msg:  the error message
 *
 * n encoding error
 */
unsafe extern "C" fn xmlEncodingErr(mut error: xmlParserErrors,
                                    mut msg: *const std::os::raw::c_char,
                                    mut val: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_I18N as std::os::raw::c_int, error as std::os::raw::c_int,
                    XML_ERR_FATAL, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    val, 0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, val);
}
/* LIBXML_ICU_ENABLED */
/* ***********************************************************************
 *									*
 *		Conversions To/From UTF8 encoding			*
 *									*
 ************************************************************************/
/* *
 * asciiToUTF8:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of ASCII chars
 * @inlen:  the length of @in
 *
 * Take a block of ASCII chars in and try to convert it to an UTF-8
 * block of chars out.
 * Returns 0 if success, or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
unsafe extern "C" fn asciiToUTF8(mut out: *mut std::os::raw::c_uchar,
                                 mut outlen: *mut std::os::raw::c_int,
                                 mut in_0: *const std::os::raw::c_uchar,
                                 mut inlen: *mut std::os::raw::c_int) -> std::os::raw::c_int {
    let mut outstart: *mut std::os::raw::c_uchar = out;
    let mut base: *const std::os::raw::c_uchar = in_0;
    let mut processed: *const std::os::raw::c_uchar = in_0;
    let mut outend: *mut std::os::raw::c_uchar = out.offset(*outlen as isize);
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut c: std::os::raw::c_uint = 0;
    inend = in_0.offset(*inlen as isize);
    while in_0 < inend &&
              (out.offset_from(outstart) as std::os::raw::c_long +
                   5 as std::os::raw::c_int as std::os::raw::c_long) < *outlen as std::os::raw::c_long
          {
        let fresh0 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh0 as std::os::raw::c_uint;
        if out >= outend { break ; }
        if c < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = c as std::os::raw::c_uchar
        } else {
            *outlen =
                out.offset_from(outstart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            *inlen =
                processed.offset_from(base) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        processed = in_0
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlen =
        processed.offset_from(base) as std::os::raw::c_long as std::os::raw::c_int;
    return *outlen;
}
/* *
 * UTF8Toascii:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to an ASCII
 * block of chars out.
 *
 * Returns 0 if success, -2 if the transcoding fails, or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
unsafe extern "C" fn UTF8Toascii(mut out: *mut std::os::raw::c_uchar,
                                 mut outlen: *mut std::os::raw::c_int,
                                 mut in_0: *const std::os::raw::c_uchar,
                                 mut inlen: *mut std::os::raw::c_int) -> std::os::raw::c_int {
    let mut processed: *const std::os::raw::c_uchar = in_0;
    let mut outend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut outstart: *const std::os::raw::c_uchar = out;
    let mut instart: *const std::os::raw::c_uchar = in_0;
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut c: std::os::raw::c_uint = 0;
    let mut d: std::os::raw::c_uint = 0;
    let mut trailing: std::os::raw::c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if in_0.is_null() {
        /*
	 * initialization nothing to do
	 */
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset(*outlen as isize);
    while in_0 < inend {
        let fresh2 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh2 as std::os::raw::c_uint;
        if d < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            c = d;
            trailing = 0 as std::os::raw::c_int
        } else if d < 0xc0 as std::os::raw::c_int as std::os::raw::c_uint {
            /* trailing byte in leading position */
            *outlen =
                out.offset_from(outstart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            *inlen =
                processed.offset_from(instart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(2 as std::os::raw::c_int)
        } else {
            if d < 0xe0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x1f as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 1 as std::os::raw::c_int
            } else if d < 0xf0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0xf as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 2 as std::os::raw::c_int
            } else if d < 0xf8 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x7 as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 3 as std::os::raw::c_int
            } else {
                /* no chance for this in Ascii */
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlen =
                    processed.offset_from(instart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
        }
        if (inend.offset_from(in_0) as std::os::raw::c_long) <
               trailing as std::os::raw::c_long {
            break ;
        }
        while trailing != 0 {
            if in_0 >= inend ||
                   {
                       let fresh3 = in_0;
                       in_0 = in_0.offset(1);
                       d = *fresh3 as std::os::raw::c_uint;
                       (d & 0xc0 as std::os::raw::c_int as std::os::raw::c_uint) !=
                           0x80 as std::os::raw::c_int as std::os::raw::c_uint
                   } {
                break ;
            }
            c <<= 6 as std::os::raw::c_int;
            c |= d & 0x3f as std::os::raw::c_int as std::os::raw::c_uint;
            trailing -= 1
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            if out >= outend as *mut std::os::raw::c_uchar { break ; }
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = c as std::os::raw::c_uchar;
            processed = in_0
        } else {
            /* no chance for this in Ascii */
            *outlen =
                out.offset_from(outstart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            *inlen =
                processed.offset_from(instart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(2 as std::os::raw::c_int)
        }
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlen =
        processed.offset_from(instart) as std::os::raw::c_long as
            std::os::raw::c_int;
    return *outlen;
}
/* LIBXML_OUTPUT_ENABLED */
/* LIBXML_OUTPUT_ENABLED */
/* *
 * isolat1ToUTF8:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of ISO Latin 1 chars
 * @inlen:  the length of @in
 *
 * Take a block of ISO Latin 1 chars in and try to convert it to an UTF-8
 * block of chars out.
 * Returns the number of bytes written if success, or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
#[no_mangle]
pub unsafe extern "C" fn isolat1ToUTF8(mut out: *mut std::os::raw::c_uchar,
                                       mut outlen: *mut std::os::raw::c_int,
                                       mut in_0: *const std::os::raw::c_uchar,
                                       mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut outstart: *mut std::os::raw::c_uchar = out;
    let mut base: *const std::os::raw::c_uchar = in_0;
    let mut outend: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut instop: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    if out.is_null() || in_0.is_null() || outlen.is_null() || inlen.is_null()
       {
        return -(1 as std::os::raw::c_int)
    }
    outend = out.offset(*outlen as isize);
    inend = in_0.offset(*inlen as isize);
    instop = inend;
    while in_0 < inend && out < outend.offset(-(1 as std::os::raw::c_int as isize)) {
        if *in_0 as std::os::raw::c_int >= 0x80 as std::os::raw::c_int {
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 =
                (*in_0 as std::os::raw::c_int >> 6 as std::os::raw::c_int &
                     0x1f as std::os::raw::c_int | 0xc0 as std::os::raw::c_int) as
                    std::os::raw::c_uchar;
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 =
                (*in_0 as std::os::raw::c_int & 0x3f as std::os::raw::c_int |
                     0x80 as std::os::raw::c_int) as std::os::raw::c_uchar;
            in_0 = in_0.offset(1)
        }
        if instop.offset_from(in_0) as std::os::raw::c_long >
               outend.offset_from(out) as std::os::raw::c_long {
            instop =
                in_0.offset(outend.offset_from(out) as std::os::raw::c_long
                                as isize)
        }
        while in_0 < instop && (*in_0 as std::os::raw::c_int) < 0x80 as std::os::raw::c_int {
            let fresh7 = in_0;
            in_0 = in_0.offset(1);
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = *fresh7
        }
    }
    if in_0 < inend && out < outend &&
           (*in_0 as std::os::raw::c_int) < 0x80 as std::os::raw::c_int {
        let fresh9 = in_0;
        in_0 = in_0.offset(1);
        let fresh10 = out;
        out = out.offset(1);
        *fresh10 = *fresh9
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlen = in_0.offset_from(base) as std::os::raw::c_long as std::os::raw::c_int;
    return *outlen;
}
/* *
 * UTF8ToUTF8:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @inb:  a pointer to an array of UTF-8 chars
 * @inlenb:  the length of @in in UTF-8 chars
 *
 * No op copy operation for UTF8 handling.
 *
 * Returns the number of bytes written, or -1 if lack of space.
 *     The value of *inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 */
unsafe extern "C" fn UTF8ToUTF8(mut out: *mut std::os::raw::c_uchar,
                                mut outlen: *mut std::os::raw::c_int,
                                mut inb: *const std::os::raw::c_uchar,
                                mut inlenb: *mut std::os::raw::c_int) -> std::os::raw::c_int {
    let mut len: std::os::raw::c_int = 0;
    if out.is_null() || outlen.is_null() || inlenb.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if inb.is_null() {
        /* inb == NULL means output is initialized. */
        *outlen = 0 as std::os::raw::c_int;
        *inlenb = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    if *outlen > *inlenb { len = *inlenb } else { len = *outlen }
    if len < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    memcpy(out as *mut std::os::raw::c_void, inb as *const std::os::raw::c_void,
           len as std::os::raw::c_ulong);
    *outlen = len;
    *inlenb = len;
    return *outlen;
}
/*
 * Export a few useful functions
 */
/* *
 * UTF8Toisolat1:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to an ISO Latin 1
 * block of chars out.
 *
 * Returns the number of bytes written if success, -2 if the transcoding fails,
           or -1 otherwise
 * The value of @inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of octets consumed.
 */
#[no_mangle]
pub unsafe extern "C" fn UTF8Toisolat1(mut out: *mut std::os::raw::c_uchar,
                                       mut outlen: *mut std::os::raw::c_int,
                                       mut in_0: *const std::os::raw::c_uchar,
                                       mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut processed: *const std::os::raw::c_uchar = in_0;
    let mut outend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut outstart: *const std::os::raw::c_uchar = out;
    let mut instart: *const std::os::raw::c_uchar = in_0;
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut c: std::os::raw::c_uint = 0;
    let mut d: std::os::raw::c_uint = 0;
    let mut trailing: std::os::raw::c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if in_0.is_null() {
        /*
	 * initialization nothing to do
	 */
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset(*outlen as isize);
    while in_0 < inend {
        let fresh11 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh11 as std::os::raw::c_uint;
        if d < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            c = d;
            trailing = 0 as std::os::raw::c_int
        } else if d < 0xc0 as std::os::raw::c_int as std::os::raw::c_uint {
            /* trailing byte in leading position */
            *outlen =
                out.offset_from(outstart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            *inlen =
                processed.offset_from(instart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(2 as std::os::raw::c_int)
        } else {
            if d < 0xe0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x1f as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 1 as std::os::raw::c_int
            } else if d < 0xf0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0xf as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 2 as std::os::raw::c_int
            } else if d < 0xf8 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x7 as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 3 as std::os::raw::c_int
            } else {
                /* no chance for this in IsoLat1 */
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlen =
                    processed.offset_from(instart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
        }
        if (inend.offset_from(in_0) as std::os::raw::c_long) <
               trailing as std::os::raw::c_long {
            break ;
        }
        while trailing != 0 {
            if in_0 >= inend { break ; }
            let fresh12 = in_0;
            in_0 = in_0.offset(1);
            d = *fresh12 as std::os::raw::c_uint;
            if d & 0xc0 as std::os::raw::c_int as std::os::raw::c_uint !=
                   0x80 as std::os::raw::c_int as std::os::raw::c_uint {
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlen =
                    processed.offset_from(instart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
            c <<= 6 as std::os::raw::c_int;
            c |= d & 0x3f as std::os::raw::c_int as std::os::raw::c_uint;
            trailing -= 1
        }
        /* assertion: c is a single UTF-4 value */
        if c <= 0xff as std::os::raw::c_int as std::os::raw::c_uint {
            if out >= outend as *mut std::os::raw::c_uchar { break ; }
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = c as std::os::raw::c_uchar;
            processed = in_0
        } else {
            /* no chance for this in IsoLat1 */
            *outlen =
                out.offset_from(outstart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            *inlen =
                processed.offset_from(instart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(2 as std::os::raw::c_int)
        }
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlen =
        processed.offset_from(instart) as std::os::raw::c_long as
            std::os::raw::c_int;
    return *outlen;
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * UTF16LEToUTF8:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @inb:  a pointer to an array of UTF-16LE passwd as a byte array
 * @inlenb:  the length of @in in UTF-16LE chars
 *
 * Take a block of UTF-16LE ushorts in and try to convert it to an UTF-8
 * block of chars out. This function assumes the endian property
 * is the same between the native type of this machine and the
 * inputed one.
 *
 * Returns the number of bytes written, or -1 if lack of space, or -2
 *     if the transcoding fails (if *in is not a valid utf16 string)
 *     The value of *inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 */
unsafe extern "C" fn UTF16LEToUTF8(mut out: *mut std::os::raw::c_uchar,
                                   mut outlen: *mut std::os::raw::c_int,
                                   mut inb: *const std::os::raw::c_uchar,
                                   mut inlenb: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut outstart: *mut std::os::raw::c_uchar = out;
    let mut processed: *const std::os::raw::c_uchar = inb;
    let mut outend: *mut std::os::raw::c_uchar = out.offset(*outlen as isize);
    let mut in_0: *mut std::os::raw::c_ushort = inb as *mut std::os::raw::c_ushort;
    let mut inend: *mut std::os::raw::c_ushort = 0 as *mut std::os::raw::c_ushort;
    let mut c: std::os::raw::c_uint = 0;
    let mut d: std::os::raw::c_uint = 0;
    let mut inlen: std::os::raw::c_uint = 0;
    let mut tmp: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut bits: std::os::raw::c_int = 0;
    if *inlenb % 2 as std::os::raw::c_int == 1 as std::os::raw::c_int { *inlenb -= 1 }
    inlen = (*inlenb / 2 as std::os::raw::c_int) as std::os::raw::c_uint;
    inend = in_0.offset(inlen as isize);
    while in_0 < inend &&
              (out.offset_from(outstart) as std::os::raw::c_long +
                   5 as std::os::raw::c_int as std::os::raw::c_long) < *outlen as std::os::raw::c_long
          {
        if xmlLittleEndian != 0 {
            let fresh14 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh14 as std::os::raw::c_uint
        } else {
            tmp = in_0 as *mut std::os::raw::c_uchar;
            let fresh15 = tmp;
            tmp = tmp.offset(1);
            c = *fresh15 as std::os::raw::c_uint;
            c = c | (*tmp as std::os::raw::c_uint) << 8 as std::os::raw::c_int;
            in_0 = in_0.offset(1)
        }
        if c & 0xfc00 as std::os::raw::c_int as std::os::raw::c_uint ==
               0xd800 as std::os::raw::c_int as std::os::raw::c_uint {
            /* surrogates */
            if in_0 >= inend { break ; }
            if xmlLittleEndian != 0 {
                let fresh16 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh16 as std::os::raw::c_uint
            } else {
                tmp = in_0 as *mut std::os::raw::c_uchar;
                let fresh17 = tmp;
                tmp = tmp.offset(1);
                d = *fresh17 as std::os::raw::c_uint;
                d = d | (*tmp as std::os::raw::c_uint) << 8 as std::os::raw::c_int;
                in_0 = in_0.offset(1)
            }
            if d & 0xfc00 as std::os::raw::c_int as std::os::raw::c_uint ==
                   0xdc00 as std::os::raw::c_int as std::os::raw::c_uint {
                c &= 0x3ff as std::os::raw::c_int as std::os::raw::c_uint;
                c <<= 10 as std::os::raw::c_int;
                c |= d & 0x3ff as std::os::raw::c_int as std::os::raw::c_uint;
                c = c.wrapping_add(0x10000 as std::os::raw::c_int as std::os::raw::c_uint)
            } else {
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlenb =
                    processed.offset_from(inb) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
        }
        /* assertion: c is a single UTF-4 value */
        if out >= outend { break ; }
        if c < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh18 = out;
            out = out.offset(1);
            *fresh18 = c as std::os::raw::c_uchar;
            bits = -(6 as std::os::raw::c_int)
        } else if c < 0x800 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 =
                (c >> 6 as std::os::raw::c_int & 0x1f as std::os::raw::c_int as std::os::raw::c_uint |
                     0xc0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits = 0 as std::os::raw::c_int
        } else if c < 0x10000 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh20 = out;
            out = out.offset(1);
            *fresh20 =
                (c >> 12 as std::os::raw::c_int & 0xf as std::os::raw::c_int as std::os::raw::c_uint |
                     0xe0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits = 6 as std::os::raw::c_int
        } else {
            let fresh21 = out;
            out = out.offset(1);
            *fresh21 =
                (c >> 18 as std::os::raw::c_int & 0x7 as std::os::raw::c_int as std::os::raw::c_uint |
                     0xf0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits = 12 as std::os::raw::c_int
        }
        while bits >= 0 as std::os::raw::c_int {
            if out >= outend { break ; }
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 =
                (c >> bits & 0x3f as std::os::raw::c_int as std::os::raw::c_uint |
                     0x80 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits -= 6 as std::os::raw::c_int
        }
        processed = in_0 as *const std::os::raw::c_uchar
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlenb =
        processed.offset_from(inb) as std::os::raw::c_long as std::os::raw::c_int;
    return *outlen;
}
/* *
 * UTF8ToUTF16LE:
 * @outb:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @outb
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to an UTF-16LE
 * block of chars out.
 *
 * Returns the number of bytes written, or -1 if lack of space, or -2
 *     if the transcoding failed.
 */
unsafe extern "C" fn UTF8ToUTF16LE(mut outb: *mut std::os::raw::c_uchar,
                                   mut outlen: *mut std::os::raw::c_int,
                                   mut in_0: *const std::os::raw::c_uchar,
                                   mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut out: *mut std::os::raw::c_ushort = outb as *mut std::os::raw::c_ushort;
    let mut processed: *const std::os::raw::c_uchar = in_0;
    let instart: *const std::os::raw::c_uchar = in_0;
    let mut outstart: *mut std::os::raw::c_ushort = out;
    let mut outend: *mut std::os::raw::c_ushort = 0 as *mut std::os::raw::c_ushort;
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut c: std::os::raw::c_uint = 0;
    let mut d: std::os::raw::c_uint = 0;
    let mut trailing: std::os::raw::c_int = 0;
    let mut tmp: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut tmp1: std::os::raw::c_ushort = 0;
    let mut tmp2: std::os::raw::c_ushort = 0;
    /* UTF16LE encoding has no BOM */
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if in_0.is_null() {
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset((*outlen / 2 as std::os::raw::c_int) as isize);
    while in_0 < inend {
        let fresh23 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh23 as std::os::raw::c_uint;
        if d < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            c = d;
            trailing = 0 as std::os::raw::c_int
        } else if d < 0xc0 as std::os::raw::c_int as std::os::raw::c_uint {
            /* trailing byte in leading position */
            *outlen =
                (out.offset_from(outstart) as std::os::raw::c_long *
                     2 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_int;
            *inlen =
                processed.offset_from(instart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(2 as std::os::raw::c_int)
        } else {
            if d < 0xe0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x1f as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 1 as std::os::raw::c_int
            } else if d < 0xf0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0xf as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 2 as std::os::raw::c_int
            } else if d < 0xf8 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x7 as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 3 as std::os::raw::c_int
            } else {
                /* no chance for this in UTF-16 */
                *outlen =
                    (out.offset_from(outstart) as std::os::raw::c_long *
                         2 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_int;
                *inlen =
                    processed.offset_from(instart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
        }
        if (inend.offset_from(in_0) as std::os::raw::c_long) <
               trailing as std::os::raw::c_long {
            break ;
        }
        while trailing != 0 {
            if in_0 >= inend ||
                   {
                       let fresh24 = in_0;
                       in_0 = in_0.offset(1);
                       d = *fresh24 as std::os::raw::c_uint;
                       (d & 0xc0 as std::os::raw::c_int as std::os::raw::c_uint) !=
                           0x80 as std::os::raw::c_int as std::os::raw::c_uint
                   } {
                break ;
            }
            c <<= 6 as std::os::raw::c_int;
            c |= d & 0x3f as std::os::raw::c_int as std::os::raw::c_uint;
            trailing -= 1
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x10000 as std::os::raw::c_int as std::os::raw::c_uint {
            if out >= outend { break ; }
            if xmlLittleEndian != 0 {
                let fresh25 = out;
                out = out.offset(1);
                *fresh25 = c as std::os::raw::c_ushort
            } else {
                tmp = out as *mut std::os::raw::c_uchar;
                *tmp = c as std::os::raw::c_uchar;
                *tmp.offset(1 as std::os::raw::c_int as isize) =
                    (c >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
                out = out.offset(1)
            }
        } else {
            if !(c < 0x110000 as std::os::raw::c_int as std::os::raw::c_uint) { break ; }
            if out.offset(1 as std::os::raw::c_int as isize) >= outend { break ; }
            c = c.wrapping_sub(0x10000 as std::os::raw::c_int as std::os::raw::c_uint);
            if xmlLittleEndian != 0 {
                let fresh26 = out;
                out = out.offset(1);
                *fresh26 =
                    (0xd800 as std::os::raw::c_int as std::os::raw::c_uint |
                         c >> 10 as std::os::raw::c_int) as std::os::raw::c_ushort;
                let fresh27 = out;
                out = out.offset(1);
                *fresh27 =
                    (0xdc00 as std::os::raw::c_int as std::os::raw::c_uint |
                         c & 0x3ff as std::os::raw::c_int as std::os::raw::c_uint) as
                        std::os::raw::c_ushort
            } else {
                tmp1 =
                    (0xd800 as std::os::raw::c_int as std::os::raw::c_uint |
                         c >> 10 as std::os::raw::c_int) as std::os::raw::c_ushort;
                tmp = out as *mut std::os::raw::c_uchar;
                *tmp = tmp1 as std::os::raw::c_uchar;
                *tmp.offset(1 as std::os::raw::c_int as isize) =
                    (tmp1 as std::os::raw::c_int >> 8 as std::os::raw::c_int) as
                        std::os::raw::c_uchar;
                out = out.offset(1);
                tmp2 =
                    (0xdc00 as std::os::raw::c_int as std::os::raw::c_uint |
                         c & 0x3ff as std::os::raw::c_int as std::os::raw::c_uint) as
                        std::os::raw::c_ushort;
                tmp = out as *mut std::os::raw::c_uchar;
                *tmp = tmp2 as std::os::raw::c_uchar;
                *tmp.offset(1 as std::os::raw::c_int as isize) =
                    (tmp2 as std::os::raw::c_int >> 8 as std::os::raw::c_int) as
                        std::os::raw::c_uchar;
                out = out.offset(1)
            }
        }
        processed = in_0
    }
    *outlen =
        (out.offset_from(outstart) as std::os::raw::c_long *
             2 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_int;
    *inlen =
        processed.offset_from(instart) as std::os::raw::c_long as
            std::os::raw::c_int;
    return *outlen;
}
/* *
 * UTF8ToUTF16:
 * @outb:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @outb
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to an UTF-16
 * block of chars out.
 *
 * Returns the number of bytes written, or -1 if lack of space, or -2
 *     if the transcoding failed.
 */
unsafe extern "C" fn UTF8ToUTF16(mut outb: *mut std::os::raw::c_uchar,
                                 mut outlen: *mut std::os::raw::c_int,
                                 mut in_0: *const std::os::raw::c_uchar,
                                 mut inlen: *mut std::os::raw::c_int) -> std::os::raw::c_int {
    if in_0.is_null() {
        /*
	 * initialization, add the Byte Order Mark for UTF-16LE
	 */
        if *outlen >= 2 as std::os::raw::c_int {
            *outb.offset(0 as std::os::raw::c_int as isize) =
                0xff as std::os::raw::c_int as std::os::raw::c_uchar;
            *outb.offset(1 as std::os::raw::c_int as isize) =
                0xfe as std::os::raw::c_int as std::os::raw::c_uchar;
            *outlen = 2 as std::os::raw::c_int;
            *inlen = 0 as std::os::raw::c_int;
            return 2 as std::os::raw::c_int
        }
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    return UTF8ToUTF16LE(outb, outlen, in_0, inlen);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * UTF16BEToUTF8:
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @inb:  a pointer to an array of UTF-16 passed as a byte array
 * @inlenb:  the length of @in in UTF-16 chars
 *
 * Take a block of UTF-16 ushorts in and try to convert it to an UTF-8
 * block of chars out. This function assumes the endian property
 * is the same between the native type of this machine and the
 * inputed one.
 *
 * Returns the number of bytes written, or -1 if lack of space, or -2
 *     if the transcoding fails (if *in is not a valid utf16 string)
 * The value of *inlen after return is the number of octets consumed
 *     if the return value is positive, else unpredictable.
 */
unsafe extern "C" fn UTF16BEToUTF8(mut out: *mut std::os::raw::c_uchar,
                                   mut outlen: *mut std::os::raw::c_int,
                                   mut inb: *const std::os::raw::c_uchar,
                                   mut inlenb: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut outstart: *mut std::os::raw::c_uchar = out;
    let mut processed: *const std::os::raw::c_uchar = inb;
    let mut outend: *mut std::os::raw::c_uchar = out.offset(*outlen as isize);
    let mut in_0: *mut std::os::raw::c_ushort = inb as *mut std::os::raw::c_ushort;
    let mut inend: *mut std::os::raw::c_ushort = 0 as *mut std::os::raw::c_ushort;
    let mut c: std::os::raw::c_uint = 0;
    let mut d: std::os::raw::c_uint = 0;
    let mut inlen: std::os::raw::c_uint = 0;
    let mut tmp: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut bits: std::os::raw::c_int = 0;
    if *inlenb % 2 as std::os::raw::c_int == 1 as std::os::raw::c_int { *inlenb -= 1 }
    inlen = (*inlenb / 2 as std::os::raw::c_int) as std::os::raw::c_uint;
    inend = in_0.offset(inlen as isize);
    while in_0 < inend {
        if xmlLittleEndian != 0 {
            tmp = in_0 as *mut std::os::raw::c_uchar;
            let fresh28 = tmp;
            tmp = tmp.offset(1);
            c = *fresh28 as std::os::raw::c_uint;
            c = c << 8 as std::os::raw::c_int;
            c = c | *tmp as std::os::raw::c_uint;
            in_0 = in_0.offset(1)
        } else {
            let fresh29 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh29 as std::os::raw::c_uint
        }
        if c & 0xfc00 as std::os::raw::c_int as std::os::raw::c_uint ==
               0xd800 as std::os::raw::c_int as std::os::raw::c_uint {
            /* surrogates */
            if in_0 >= inend {
                /* (in > inend) shouldn't happens */
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlenb =
                    processed.offset_from(inb) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
            if xmlLittleEndian != 0 {
                tmp = in_0 as *mut std::os::raw::c_uchar;
                let fresh30 = tmp;
                tmp = tmp.offset(1);
                d = *fresh30 as std::os::raw::c_uint;
                d = d << 8 as std::os::raw::c_int;
                d = d | *tmp as std::os::raw::c_uint;
                in_0 = in_0.offset(1)
            } else {
                let fresh31 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh31 as std::os::raw::c_uint
            }
            if d & 0xfc00 as std::os::raw::c_int as std::os::raw::c_uint ==
                   0xdc00 as std::os::raw::c_int as std::os::raw::c_uint {
                c &= 0x3ff as std::os::raw::c_int as std::os::raw::c_uint;
                c <<= 10 as std::os::raw::c_int;
                c |= d & 0x3ff as std::os::raw::c_int as std::os::raw::c_uint;
                c = c.wrapping_add(0x10000 as std::os::raw::c_int as std::os::raw::c_uint)
            } else {
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlenb =
                    processed.offset_from(inb) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
        }
        /* assertion: c is a single UTF-4 value */
        if out >= outend { break ; }
        if c < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh32 = out;
            out = out.offset(1);
            *fresh32 = c as std::os::raw::c_uchar;
            bits = -(6 as std::os::raw::c_int)
        } else if c < 0x800 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh33 = out;
            out = out.offset(1);
            *fresh33 =
                (c >> 6 as std::os::raw::c_int & 0x1f as std::os::raw::c_int as std::os::raw::c_uint |
                     0xc0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits = 0 as std::os::raw::c_int
        } else if c < 0x10000 as std::os::raw::c_int as std::os::raw::c_uint {
            let fresh34 = out;
            out = out.offset(1);
            *fresh34 =
                (c >> 12 as std::os::raw::c_int & 0xf as std::os::raw::c_int as std::os::raw::c_uint |
                     0xe0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits = 6 as std::os::raw::c_int
        } else {
            let fresh35 = out;
            out = out.offset(1);
            *fresh35 =
                (c >> 18 as std::os::raw::c_int & 0x7 as std::os::raw::c_int as std::os::raw::c_uint |
                     0xf0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits = 12 as std::os::raw::c_int
        }
        while bits >= 0 as std::os::raw::c_int {
            if out >= outend { break ; }
            let fresh36 = out;
            out = out.offset(1);
            *fresh36 =
                (c >> bits & 0x3f as std::os::raw::c_int as std::os::raw::c_uint |
                     0x80 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
            bits -= 6 as std::os::raw::c_int
        }
        processed = in_0 as *const std::os::raw::c_uchar
    }
    *outlen =
        out.offset_from(outstart) as std::os::raw::c_long as std::os::raw::c_int;
    *inlenb =
        processed.offset_from(inb) as std::os::raw::c_long as std::os::raw::c_int;
    return *outlen;
}
/* *
 * UTF8ToUTF16BE:
 * @outb:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @outb
 * @in:  a pointer to an array of UTF-8 chars
 * @inlen:  the length of @in
 *
 * Take a block of UTF-8 chars in and try to convert it to an UTF-16BE
 * block of chars out.
 *
 * Returns the number of byte written, or -1 by lack of space, or -2
 *     if the transcoding failed.
 */
unsafe extern "C" fn UTF8ToUTF16BE(mut outb: *mut std::os::raw::c_uchar,
                                   mut outlen: *mut std::os::raw::c_int,
                                   mut in_0: *const std::os::raw::c_uchar,
                                   mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut out: *mut std::os::raw::c_ushort = outb as *mut std::os::raw::c_ushort;
    let mut processed: *const std::os::raw::c_uchar = in_0;
    let instart: *const std::os::raw::c_uchar = in_0;
    let mut outstart: *mut std::os::raw::c_ushort = out;
    let mut outend: *mut std::os::raw::c_ushort = 0 as *mut std::os::raw::c_ushort;
    let mut inend: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
    let mut c: std::os::raw::c_uint = 0;
    let mut d: std::os::raw::c_uint = 0;
    let mut trailing: std::os::raw::c_int = 0;
    let mut tmp: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut tmp1: std::os::raw::c_ushort = 0;
    let mut tmp2: std::os::raw::c_ushort = 0;
    /* UTF-16BE has no BOM */
    if outb.is_null() || outlen.is_null() || inlen.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if in_0.is_null() {
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset((*outlen / 2 as std::os::raw::c_int) as isize);
    while in_0 < inend {
        let fresh37 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh37 as std::os::raw::c_uint;
        if d < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
            c = d;
            trailing = 0 as std::os::raw::c_int
        } else if d < 0xc0 as std::os::raw::c_int as std::os::raw::c_uint {
            /* trailing byte in leading position */
            *outlen =
                out.offset_from(outstart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            *inlen =
                processed.offset_from(instart) as std::os::raw::c_long as
                    std::os::raw::c_int;
            return -(2 as std::os::raw::c_int)
        } else {
            if d < 0xe0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x1f as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 1 as std::os::raw::c_int
            } else if d < 0xf0 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0xf as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 2 as std::os::raw::c_int
            } else if d < 0xf8 as std::os::raw::c_int as std::os::raw::c_uint {
                c = d & 0x7 as std::os::raw::c_int as std::os::raw::c_uint;
                trailing = 3 as std::os::raw::c_int
            } else {
                /* no chance for this in UTF-16 */
                *outlen =
                    out.offset_from(outstart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                *inlen =
                    processed.offset_from(instart) as std::os::raw::c_long as
                        std::os::raw::c_int;
                return -(2 as std::os::raw::c_int)
            }
        }
        if (inend.offset_from(in_0) as std::os::raw::c_long) <
               trailing as std::os::raw::c_long {
            break ;
        }
        while trailing != 0 {
            if in_0 >= inend ||
                   {
                       let fresh38 = in_0;
                       in_0 = in_0.offset(1);
                       d = *fresh38 as std::os::raw::c_uint;
                       (d & 0xc0 as std::os::raw::c_int as std::os::raw::c_uint) !=
                           0x80 as std::os::raw::c_int as std::os::raw::c_uint
                   } {
                break ;
            }
            c <<= 6 as std::os::raw::c_int;
            c |= d & 0x3f as std::os::raw::c_int as std::os::raw::c_uint;
            trailing -= 1
        }
        /* assertion: c is a single UTF-4 value */
        if c < 0x10000 as std::os::raw::c_int as std::os::raw::c_uint {
            if out >= outend { break ; }
            if xmlLittleEndian != 0 {
                tmp = out as *mut std::os::raw::c_uchar;
                *tmp = (c >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
                *tmp.offset(1 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
                out = out.offset(1)
            } else {
                let fresh39 = out;
                out = out.offset(1);
                *fresh39 = c as std::os::raw::c_ushort
            }
        } else {
            if !(c < 0x110000 as std::os::raw::c_int as std::os::raw::c_uint) { break ; }
            if out.offset(1 as std::os::raw::c_int as isize) >= outend { break ; }
            c = c.wrapping_sub(0x10000 as std::os::raw::c_int as std::os::raw::c_uint);
            if xmlLittleEndian != 0 {
                tmp1 =
                    (0xd800 as std::os::raw::c_int as std::os::raw::c_uint |
                         c >> 10 as std::os::raw::c_int) as std::os::raw::c_ushort;
                tmp = out as *mut std::os::raw::c_uchar;
                *tmp =
                    (tmp1 as std::os::raw::c_int >> 8 as std::os::raw::c_int) as
                        std::os::raw::c_uchar;
                *tmp.offset(1 as std::os::raw::c_int as isize) =
                    tmp1 as std::os::raw::c_uchar;
                out = out.offset(1);
                tmp2 =
                    (0xdc00 as std::os::raw::c_int as std::os::raw::c_uint |
                         c & 0x3ff as std::os::raw::c_int as std::os::raw::c_uint) as
                        std::os::raw::c_ushort;
                tmp = out as *mut std::os::raw::c_uchar;
                *tmp =
                    (tmp2 as std::os::raw::c_int >> 8 as std::os::raw::c_int) as
                        std::os::raw::c_uchar;
                *tmp.offset(1 as std::os::raw::c_int as isize) =
                    tmp2 as std::os::raw::c_uchar;
                out = out.offset(1)
            } else {
                let fresh40 = out;
                out = out.offset(1);
                *fresh40 =
                    (0xd800 as std::os::raw::c_int as std::os::raw::c_uint |
                         c >> 10 as std::os::raw::c_int) as std::os::raw::c_ushort;
                let fresh41 = out;
                out = out.offset(1);
                *fresh41 =
                    (0xdc00 as std::os::raw::c_int as std::os::raw::c_uint |
                         c & 0x3ff as std::os::raw::c_int as std::os::raw::c_uint) as
                        std::os::raw::c_ushort
            }
        }
        processed = in_0
    }
    *outlen =
        (out.offset_from(outstart) as std::os::raw::c_long *
             2 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_int;
    *inlen =
        processed.offset_from(instart) as std::os::raw::c_long as
            std::os::raw::c_int;
    return *outlen;
}
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *		Generic encoding handling routines			*
 *									*
 ************************************************************************/
/* *
 * xmlDetectCharEncoding:
 * @in:  a pointer to the first bytes of the XML entity, must be at least
 *       2 bytes long (at least 4 if encoding is UTF4 variant).
 * @len:  pointer to the length of the buffer
 *
 * Guess the encoding of the entity using the first bytes of the entity content
 * according to the non-normative appendix F of the XML-1.0 recommendation.
 *
 * Returns one of the XML_CHAR_ENCODING_... values.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDetectCharEncoding(mut in_0: *const std::os::raw::c_uchar,
                                               mut len: std::os::raw::c_int)
 -> xmlCharEncoding {
    if in_0.is_null() { return XML_CHAR_ENCODING_NONE }
    if len >= 4 as std::os::raw::c_int {
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3c as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UCS4BE
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0x3c as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UCS4LE
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3c as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UCS4_2143
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3c as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UCS4_3412
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0x4c as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x6f as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0xa7 as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x94 as std::os::raw::c_int {
            return XML_CHAR_ENCODING_EBCDIC
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0x3c as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3f as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x78 as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x6d as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UTF8
        }
        /*
	 * Although not part of the recommendation, we also
	 * attempt an "auto-recognition" of UTF-16LE and
	 * UTF-16BE encodings.
	 */
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0x3c as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3f as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UTF16LE
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3c as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0x3f as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UTF16BE
        }
    }
    if len >= 3 as std::os::raw::c_int {
        /*
	 * Errata on XML-1.0 June 20 2001
	 * We now allow an UTF8 encoded BOM
	 */
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xef as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0xbb as std::os::raw::c_int &&
               *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0xbf as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UTF8
        }
    }
    /* For UTF-16 we can recognize by the BOM */
    if len >= 2 as std::os::raw::c_int {
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xfe as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0xff as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UTF16BE
        }
        if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xff as std::os::raw::c_int &&
               *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0xfe as std::os::raw::c_int {
            return XML_CHAR_ENCODING_UTF16LE
        }
    }
    return XML_CHAR_ENCODING_NONE;
}
/* *
 * xmlCleanupEncodingAliases:
 *
 * Unregisters all aliases
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupEncodingAliases() {
    let mut i: std::os::raw::c_int = 0;
    if xmlCharEncodingAliases.is_null() { return }
    i = 0 as std::os::raw::c_int;
    while i < xmlCharEncodingAliasesNb {
        if !(*xmlCharEncodingAliases.offset(i as isize)).name.is_null() {
            xmlFree.expect("non-null function pointer")((*xmlCharEncodingAliases.offset(i
                                                                                            as
                                                                                            isize)).name
                                                            as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        if !(*xmlCharEncodingAliases.offset(i as isize)).alias.is_null() {
            xmlFree.expect("non-null function pointer")((*xmlCharEncodingAliases.offset(i
                                                                                            as
                                                                                            isize)).alias
                                                            as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    }
    xmlCharEncodingAliasesNb = 0 as std::os::raw::c_int;
    xmlCharEncodingAliasesMax = 0 as std::os::raw::c_int;
    xmlFree.expect("non-null function pointer")(xmlCharEncodingAliases as
                                                    *mut std::os::raw::c_void);
    xmlCharEncodingAliases = 0 as xmlCharEncodingAliasPtr;
}
/* *
 * xmlGetEncodingAlias:
 * @alias:  the alias name as parsed, in UTF-8 format (ASCII actually)
 *
 * Lookup an encoding name for the given alias.
 *
 * Returns NULL if not found, otherwise the original name
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetEncodingAlias(mut alias: *const std::os::raw::c_char)
 -> *const std::os::raw::c_char {
    let mut i: std::os::raw::c_int = 0;
    let mut upper: [std::os::raw::c_char; 100] = [0; 100];
    if alias.is_null() { return 0 as *const std::os::raw::c_char }
    if xmlCharEncodingAliases.is_null() { return 0 as *const std::os::raw::c_char }
    i = 0 as std::os::raw::c_int;
    while i < 99 as std::os::raw::c_int {
        upper[i as usize] =
            ({
                 let mut __res: std::os::raw::c_int = 0;
                 if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong >
                        1 as std::os::raw::c_int as std::os::raw::c_ulong {
                     if 0 != 0 {
                         let mut __c: std::os::raw::c_int =
                             *alias.offset(i as isize) as std::os::raw::c_int;
                         __res =
                             if __c < -(128 as std::os::raw::c_int) ||
                                    __c > 255 as std::os::raw::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else {
                         __res =
                             toupper(*alias.offset(i as isize) as std::os::raw::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*alias.offset(i as
                                                                            isize)
                                                              as std::os::raw::c_int
                                                              as isize)
                 }
                 __res
             }) as std::os::raw::c_char;
        if upper[i as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
        i += 1
    }
    upper[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    /*
     * Walk down the list looking for a definition of the alias
     */
    i = 0 as std::os::raw::c_int;
    while i < xmlCharEncodingAliasesNb {
        if strcmp((*xmlCharEncodingAliases.offset(i as isize)).alias,
                  upper.as_mut_ptr()) == 0 {
            return (*xmlCharEncodingAliases.offset(i as isize)).name
        }
        i += 1
    }
    return 0 as *const std::os::raw::c_char;
}
/* *
 * xmlAddEncodingAlias:
 * @name:  the encoding name as parsed, in UTF-8 format (ASCII actually)
 * @alias:  the alias name as parsed, in UTF-8 format (ASCII actually)
 *
 * Registers an alias @alias for an encoding named @name. Existing alias
 * will be overwritten.
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddEncodingAlias(mut name: *const std::os::raw::c_char,
                                             mut alias: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut upper: [std::os::raw::c_char; 100] = [0; 100];
    if name.is_null() || alias.is_null() { return -(1 as std::os::raw::c_int) }
    i = 0 as std::os::raw::c_int;
    while i < 99 as std::os::raw::c_int {
        upper[i as usize] =
            ({
                 let mut __res: std::os::raw::c_int = 0;
                 if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong >
                        1 as std::os::raw::c_int as std::os::raw::c_ulong {
                     if 0 != 0 {
                         let mut __c: std::os::raw::c_int =
                             *alias.offset(i as isize) as std::os::raw::c_int;
                         __res =
                             if __c < -(128 as std::os::raw::c_int) ||
                                    __c > 255 as std::os::raw::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else {
                         __res =
                             toupper(*alias.offset(i as isize) as std::os::raw::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*alias.offset(i as
                                                                            isize)
                                                              as std::os::raw::c_int
                                                              as isize)
                 }
                 __res
             }) as std::os::raw::c_char;
        if upper[i as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
        i += 1
    }
    upper[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    if xmlCharEncodingAliases.is_null() {
        xmlCharEncodingAliasesNb = 0 as std::os::raw::c_int;
        xmlCharEncodingAliasesMax = 20 as std::os::raw::c_int;
        xmlCharEncodingAliases =
            xmlMalloc.expect("non-null function pointer")((xmlCharEncodingAliasesMax
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlCharEncodingAlias>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as xmlCharEncodingAliasPtr;
        if xmlCharEncodingAliases.is_null() { return -(1 as std::os::raw::c_int) }
    } else if xmlCharEncodingAliasesNb >= xmlCharEncodingAliasesMax {
        xmlCharEncodingAliasesMax *= 2 as std::os::raw::c_int;
        xmlCharEncodingAliases =
            xmlRealloc.expect("non-null function pointer")(xmlCharEncodingAliases
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           (xmlCharEncodingAliasesMax
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlCharEncodingAlias>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as xmlCharEncodingAliasPtr
    }
    /*
     * Walk down the list looking for a definition of the alias
     */
    i = 0 as std::os::raw::c_int;
    while i < xmlCharEncodingAliasesNb {
        if strcmp((*xmlCharEncodingAliases.offset(i as isize)).alias,
                  upper.as_mut_ptr()) == 0 {
            /*
	     * Replace the definition.
	     */
            xmlFree.expect("non-null function pointer")((*xmlCharEncodingAliases.offset(i
                                                                                            as
                                                                                            isize)).name
                                                            as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
            let ref mut fresh42 =
                (*xmlCharEncodingAliases.offset(i as isize)).name;
            *fresh42 = xmlMemStrdup.expect("non-null function pointer")(name);
            return 0 as std::os::raw::c_int
        }
        i += 1
    }
    /*
     * Add the definition
     */
    let ref mut fresh43 =
        (*xmlCharEncodingAliases.offset(xmlCharEncodingAliasesNb as
                                            isize)).name;
    *fresh43 = xmlMemStrdup.expect("non-null function pointer")(name);
    let ref mut fresh44 =
        (*xmlCharEncodingAliases.offset(xmlCharEncodingAliasesNb as
                                            isize)).alias;
    *fresh44 =
        xmlMemStrdup.expect("non-null function pointer")(upper.as_mut_ptr());
    xmlCharEncodingAliasesNb += 1;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlDelEncodingAlias:
 * @alias:  the alias name as parsed, in UTF-8 format (ASCII actually)
 *
 * Unregisters an encoding alias @alias
 *
 * Returns 0 in case of success, -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDelEncodingAlias(mut alias: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if alias.is_null() { return -(1 as std::os::raw::c_int) }
    if xmlCharEncodingAliases.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * Walk down the list looking for a definition of the alias
     */
    i = 0 as std::os::raw::c_int;
    while i < xmlCharEncodingAliasesNb {
        if strcmp((*xmlCharEncodingAliases.offset(i as isize)).alias, alias)
               == 0 {
            xmlFree.expect("non-null function pointer")((*xmlCharEncodingAliases.offset(i
                                                                                            as
                                                                                            isize)).name
                                                            as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")((*xmlCharEncodingAliases.offset(i
                                                                                            as
                                                                                            isize)).alias
                                                            as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
            xmlCharEncodingAliasesNb -= 1;
            memmove(&mut *xmlCharEncodingAliases.offset(i as isize) as
                        *mut xmlCharEncodingAlias as *mut std::os::raw::c_void,
                    &mut *xmlCharEncodingAliases.offset((i + 1 as std::os::raw::c_int)
                                                            as isize) as
                        *mut xmlCharEncodingAlias as *const std::os::raw::c_void,
                    (::std::mem::size_of::<xmlCharEncodingAlias>() as
                         std::os::raw::c_ulong).wrapping_mul((xmlCharEncodingAliasesNb
                                                          - i) as
                                                         std::os::raw::c_ulong));
            return 0 as std::os::raw::c_int
        }
        i += 1
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlParseCharEncoding:
 * @name:  the encoding name as parsed, in UTF-8 format (ASCII actually)
 *
 * Compare the string to the encoding schemes already known. Note
 * that the comparison is case insensitive accordingly to the section
 * [XML] 4.3.3 Character Encoding in Entities.
 *
 * Returns one of the XML_CHAR_ENCODING_... values or XML_CHAR_ENCODING_NONE
 * if not recognized.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParseCharEncoding(mut name: *const std::os::raw::c_char)
 -> xmlCharEncoding {
    let mut alias: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut upper: [std::os::raw::c_char; 500] = [0; 500];
    let mut i: std::os::raw::c_int = 0;
    if name.is_null() { return XML_CHAR_ENCODING_NONE }
    /*
     * Do the alias resolution
     */
    alias = xmlGetEncodingAlias(name);
    if !alias.is_null() { name = alias }
    i = 0 as std::os::raw::c_int;
    while i < 499 as std::os::raw::c_int {
        upper[i as usize] =
            ({
                 let mut __res: std::os::raw::c_int = 0;
                 if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong >
                        1 as std::os::raw::c_int as std::os::raw::c_ulong {
                     if 0 != 0 {
                         let mut __c: std::os::raw::c_int =
                             *name.offset(i as isize) as std::os::raw::c_int;
                         __res =
                             if __c < -(128 as std::os::raw::c_int) ||
                                    __c > 255 as std::os::raw::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else {
                         __res =
                             toupper(*name.offset(i as isize) as std::os::raw::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*name.offset(i as
                                                                           isize)
                                                              as std::os::raw::c_int
                                                              as isize)
                 }
                 __res
             }) as std::os::raw::c_char;
        if upper[i as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
        i += 1
    }
    upper[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    if strcmp(upper.as_mut_ptr(), b"\x00" as *const u8 as *const std::os::raw::c_char)
           == 0 {
        return XML_CHAR_ENCODING_NONE
    }
    if strcmp(upper.as_mut_ptr(),
              b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UTF8
    }
    if strcmp(upper.as_mut_ptr(),
              b"UTF8\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UTF8
    }
    /*
     * NOTE: if we were able to parse this, the endianness of UTF16 is
     *       already found and in use
     */
    if strcmp(upper.as_mut_ptr(),
              b"UTF-16\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UTF16LE
    }
    if strcmp(upper.as_mut_ptr(),
              b"UTF16\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UTF16LE
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-10646-UCS-2\x00" as *const u8 as *const std::os::raw::c_char) == 0
       {
        return XML_CHAR_ENCODING_UCS2
    }
    if strcmp(upper.as_mut_ptr(),
              b"UCS-2\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UCS2
    }
    if strcmp(upper.as_mut_ptr(),
              b"UCS2\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UCS2
    }
    /*
     * NOTE: if we were able to parse this, the endianness of UCS4 is
     *       already found and in use
     */
    if strcmp(upper.as_mut_ptr(),
              b"ISO-10646-UCS-4\x00" as *const u8 as *const std::os::raw::c_char) == 0
       {
        return XML_CHAR_ENCODING_UCS4LE
    }
    if strcmp(upper.as_mut_ptr(),
              b"UCS-4\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UCS4LE
    }
    if strcmp(upper.as_mut_ptr(),
              b"UCS4\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_UCS4LE
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_1
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-LATIN-1\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_1
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO LATIN 1\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_1
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-2\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_2
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-LATIN-2\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_2
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO LATIN 2\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_2
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-3\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_3
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-4\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_4
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-5\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_5
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-6\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_6
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-7\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_7
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-8\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_8
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-8859-9\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_8859_9
    }
    if strcmp(upper.as_mut_ptr(),
              b"ISO-2022-JP\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_2022_JP
    }
    if strcmp(upper.as_mut_ptr(),
              b"SHIFT_JIS\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_SHIFT_JIS
    }
    if strcmp(upper.as_mut_ptr(),
              b"EUC-JP\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return XML_CHAR_ENCODING_EUC_JP
    }
    return XML_CHAR_ENCODING_ERROR;
}
/* *
 * xmlGetCharEncodingName:
 * @enc:  the encoding
 *
 * The "canonical" name for XML encoding.
 * C.f. http://www.w3.org/TR/REC-xml#charencoding
 * Section 4.3.3  Character Encoding in Entities
 *
 * Returns the canonical name for the given encoding
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetCharEncodingName(mut enc: xmlCharEncoding)
 -> *const std::os::raw::c_char {
    match enc as std::os::raw::c_int {
        -1 => { return 0 as *const std::os::raw::c_char }
        0 => { return 0 as *const std::os::raw::c_char }
        1 => { return b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char }
        2 => { return b"UTF-16\x00" as *const u8 as *const std::os::raw::c_char }
        3 => { return b"UTF-16\x00" as *const u8 as *const std::os::raw::c_char }
        6 => { return b"EBCDIC\x00" as *const u8 as *const std::os::raw::c_char }
        4 => {
            return b"ISO-10646-UCS-4\x00" as *const u8 as *const std::os::raw::c_char
        }
        5 => {
            return b"ISO-10646-UCS-4\x00" as *const u8 as *const std::os::raw::c_char
        }
        7 => {
            return b"ISO-10646-UCS-4\x00" as *const u8 as *const std::os::raw::c_char
        }
        8 => {
            return b"ISO-10646-UCS-4\x00" as *const u8 as *const std::os::raw::c_char
        }
        9 => {
            return b"ISO-10646-UCS-2\x00" as *const u8 as *const std::os::raw::c_char
        }
        10 => { return b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char }
        11 => { return b"ISO-8859-2\x00" as *const u8 as *const std::os::raw::c_char }
        12 => { return b"ISO-8859-3\x00" as *const u8 as *const std::os::raw::c_char }
        13 => { return b"ISO-8859-4\x00" as *const u8 as *const std::os::raw::c_char }
        14 => { return b"ISO-8859-5\x00" as *const u8 as *const std::os::raw::c_char }
        15 => { return b"ISO-8859-6\x00" as *const u8 as *const std::os::raw::c_char }
        16 => { return b"ISO-8859-7\x00" as *const u8 as *const std::os::raw::c_char }
        17 => { return b"ISO-8859-8\x00" as *const u8 as *const std::os::raw::c_char }
        18 => { return b"ISO-8859-9\x00" as *const u8 as *const std::os::raw::c_char }
        19 => {
            return b"ISO-2022-JP\x00" as *const u8 as *const std::os::raw::c_char
        }
        20 => { return b"Shift-JIS\x00" as *const u8 as *const std::os::raw::c_char }
        21 => { return b"EUC-JP\x00" as *const u8 as *const std::os::raw::c_char }
        22 => { return 0 as *const std::os::raw::c_char }
        _ => { }
    }
    return 0 as *const std::os::raw::c_char;
}
static mut handlers: *mut xmlCharEncodingHandlerPtr =
    0 as *const xmlCharEncodingHandlerPtr as *mut xmlCharEncodingHandlerPtr;
static mut nbCharEncodingHandler: std::os::raw::c_int = 0 as std::os::raw::c_int;
/*
 * The default is UTF-8 for XML, that's also the default used for the
 * parser internals, so the default encoding handler is NULL
 */
static mut xmlDefaultCharEncodingHandler: xmlCharEncodingHandlerPtr =
    0 as *const xmlCharEncodingHandler as xmlCharEncodingHandlerPtr;
/* *
 * xmlNewCharEncodingHandler:
 * @name:  the encoding name, in UTF-8 format (ASCII actually)
 * @input:  the xmlCharEncodingInputFunc to read that encoding
 * @output:  the xmlCharEncodingOutputFunc to write that encoding
 *
 * Create and registers an xmlCharEncodingHandler.
 *
 * Returns the xmlCharEncodingHandlerPtr created (or NULL in case of error).
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewCharEncodingHandler(mut name:
                                                       *const std::os::raw::c_char,
                                                   mut input:
                                                       xmlCharEncodingInputFunc,
                                                   mut output:
                                                       xmlCharEncodingOutputFunc)
 -> xmlCharEncodingHandlerPtr {
    let mut handler: xmlCharEncodingHandlerPtr =
        0 as *mut xmlCharEncodingHandler;
    let mut alias: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut upper: [std::os::raw::c_char; 500] = [0; 500];
    let mut i: std::os::raw::c_int = 0;
    let mut up: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    /*
     * Do the alias resolution
     */
    alias = xmlGetEncodingAlias(name);
    if !alias.is_null() { name = alias }
    /*
     * Keep only the uppercase version of the encoding.
     */
    if name.is_null() {
        xmlEncodingErr(XML_I18N_NO_NAME,
                       b"xmlNewCharEncodingHandler : no name !\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const std::os::raw::c_char);
        return 0 as xmlCharEncodingHandlerPtr
    }
    i = 0 as std::os::raw::c_int;
    while i < 499 as std::os::raw::c_int {
        upper[i as usize] =
            ({
                 let mut __res: std::os::raw::c_int = 0;
                 if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong >
                        1 as std::os::raw::c_int as std::os::raw::c_ulong {
                     if 0 != 0 {
                         let mut __c: std::os::raw::c_int =
                             *name.offset(i as isize) as std::os::raw::c_int;
                         __res =
                             if __c < -(128 as std::os::raw::c_int) ||
                                    __c > 255 as std::os::raw::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else {
                         __res =
                             toupper(*name.offset(i as isize) as std::os::raw::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*name.offset(i as
                                                                           isize)
                                                              as std::os::raw::c_int
                                                              as isize)
                 }
                 __res
             }) as std::os::raw::c_char;
        if upper[i as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
        i += 1
    }
    upper[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    up = xmlMemStrdup.expect("non-null function pointer")(upper.as_mut_ptr());
    if up.is_null() {
        xmlEncodingErrMemory(b"xmlNewCharEncodingHandler : out of memory !\n\x00"
                                 as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlCharEncodingHandlerPtr
    }
    /*
     * allocate and fill-up an handler block.
     */
    handler =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlCharEncodingHandler>()
                                                          as std::os::raw::c_ulong) as
            xmlCharEncodingHandlerPtr;
    if handler.is_null() {
        xmlFree.expect("non-null function pointer")(up as *mut std::os::raw::c_void);
        xmlEncodingErrMemory(b"xmlNewCharEncodingHandler : out of memory !\n\x00"
                                 as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlCharEncodingHandlerPtr
    }
    memset(handler as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlCharEncodingHandler>() as std::os::raw::c_ulong);
    (*handler).input = input;
    (*handler).output = output;
    (*handler).name = up;
    (*handler).iconv_in = 0 as *mut std::os::raw::c_void;
    (*handler).iconv_out = 0 as *mut std::os::raw::c_void;
    /*
     * registers and returns the handler.
     */
    xmlRegisterCharEncodingHandler(handler);
    return handler;
}
/* *
 * xmlInitCharEncodingHandlers:
 *
 * Initialize the char encoding support, it registers the default
 * encoding supported.
 * NOTE: while public, this function usually doesn't need to be called
 *       in normal processing.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitCharEncodingHandlers() {
    let mut tst: std::os::raw::c_ushort = 0x1234 as std::os::raw::c_int as std::os::raw::c_ushort;
    let mut ptr: *mut std::os::raw::c_uchar =
        &mut tst as *mut std::os::raw::c_ushort as *mut std::os::raw::c_uchar;
    if !handlers.is_null() { return }
    handlers =
        xmlMalloc.expect("non-null function pointer")((50 as std::os::raw::c_int as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlCharEncodingHandlerPtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut xmlCharEncodingHandlerPtr;
    if *ptr as std::os::raw::c_int == 0x12 as std::os::raw::c_int {
        xmlLittleEndian = 0 as std::os::raw::c_int
    } else if *ptr as std::os::raw::c_int == 0x34 as std::os::raw::c_int {
        xmlLittleEndian = 1 as std::os::raw::c_int
    } else {
        xmlEncodingErr(XML_ERR_INTERNAL_ERROR,
                       b"Odd problem at endianness detection\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const std::os::raw::c_char);
    }
    if handlers.is_null() {
        xmlEncodingErrMemory(b"xmlInitCharEncodingHandlers : out of memory !\n\x00"
                                 as *const u8 as *const std::os::raw::c_char);
        return
    }
    xmlNewCharEncodingHandler(b"UTF-8\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              Some(UTF8ToUTF8 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(UTF8ToUTF8 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int));
    xmlUTF16LEHandler =
        xmlNewCharEncodingHandler(b"UTF-16LE\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  Some(UTF16LEToUTF8 as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int,
                                                                _:
                                                                    *const std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int)
                                               -> std::os::raw::c_int),
                                  Some(UTF8ToUTF16LE as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int,
                                                                _:
                                                                    *const std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int)
                                               -> std::os::raw::c_int));
    xmlUTF16BEHandler =
        xmlNewCharEncodingHandler(b"UTF-16BE\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  Some(UTF16BEToUTF8 as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int,
                                                                _:
                                                                    *const std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int)
                                               -> std::os::raw::c_int),
                                  Some(UTF8ToUTF16BE as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int,
                                                                _:
                                                                    *const std::os::raw::c_uchar,
                                                                _:
                                                                    *mut std::os::raw::c_int)
                                               -> std::os::raw::c_int));
    xmlNewCharEncodingHandler(b"UTF-16\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              Some(UTF16LEToUTF8 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(UTF8ToUTF16 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int));
    xmlNewCharEncodingHandler(b"ISO-8859-1\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              Some(isolat1ToUTF8 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(UTF8Toisolat1 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int));
    xmlNewCharEncodingHandler(b"ASCII\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              Some(asciiToUTF8 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(UTF8Toascii as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int));
    xmlNewCharEncodingHandler(b"US-ASCII\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              Some(asciiToUTF8 as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                              Some(UTF8Toascii as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int));
    xmlNewCharEncodingHandler(b"HTML\x00" as *const u8 as *const std::os::raw::c_char,
                              None,
                              Some(UTF8ToHtml as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int,
                                                            _:
                                                                *const std::os::raw::c_uchar,
                                                            _:
                                                                *mut std::os::raw::c_int)
                                           -> std::os::raw::c_int));
    /* LIBXML_OUTPUT_ENABLED */
}
/* *
 * xmlCleanupCharEncodingHandlers:
 *
 * Cleanup the memory allocated for the char encoding support, it
 * unregisters all the encoding handlers and the aliases.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupCharEncodingHandlers() {
    xmlCleanupEncodingAliases();
    if handlers.is_null() { return }
    while nbCharEncodingHandler > 0 as std::os::raw::c_int {
        nbCharEncodingHandler -= 1;
        if !(*handlers.offset(nbCharEncodingHandler as isize)).is_null() {
            if !(**handlers.offset(nbCharEncodingHandler as
                                       isize)).name.is_null() {
                xmlFree.expect("non-null function pointer")((**handlers.offset(nbCharEncodingHandler
                                                                                   as
                                                                                   isize)).name
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            xmlFree.expect("non-null function pointer")(*handlers.offset(nbCharEncodingHandler
                                                                             as
                                                                             isize)
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    xmlFree.expect("non-null function pointer")(handlers as
                                                    *mut std::os::raw::c_void);
    handlers = 0 as *mut xmlCharEncodingHandlerPtr;
    nbCharEncodingHandler = 0 as std::os::raw::c_int;
    xmlDefaultCharEncodingHandler = 0 as xmlCharEncodingHandlerPtr;
}
/* *
 * xmlRegisterCharEncodingHandler:
 * @handler:  the xmlCharEncodingHandlerPtr handler block
 *
 * Register the char encoding handler, surprising, isn't it ?
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterCharEncodingHandler(mut handler:
                                                            xmlCharEncodingHandlerPtr) {
    if handlers.is_null() { xmlInitCharEncodingHandlers(); }
    if handler.is_null() || handlers.is_null() {
        xmlEncodingErr(XML_I18N_NO_HANDLER,
                       b"xmlRegisterCharEncodingHandler: NULL handler !\n\x00"
                           as *const u8 as *const std::os::raw::c_char,
                       0 as *const std::os::raw::c_char);
        return
    }
    if nbCharEncodingHandler >= 50 as std::os::raw::c_int {
        xmlEncodingErr(XML_I18N_EXCESS_HANDLER,
                       b"xmlRegisterCharEncodingHandler: Too many handler registered, see %s\n\x00"
                           as *const u8 as *const std::os::raw::c_char,
                       b"MAX_ENCODING_HANDLERS\x00" as *const u8 as
                           *const std::os::raw::c_char);
        return
    }
    let fresh45 = nbCharEncodingHandler;
    nbCharEncodingHandler = nbCharEncodingHandler + 1;
    let ref mut fresh46 = *handlers.offset(fresh45 as isize);
    *fresh46 = handler;
}
/* *
 * xmlGetCharEncodingHandler:
 * @enc:  an xmlCharEncoding value.
 *
 * Search in the registered set the handler able to read/write that encoding.
 *
 * Returns the handler or NULL if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetCharEncodingHandler(mut enc: xmlCharEncoding)
 -> xmlCharEncodingHandlerPtr {
    let mut handler: xmlCharEncodingHandlerPtr =
        0 as *mut xmlCharEncodingHandler;
    if handlers.is_null() { xmlInitCharEncodingHandlers(); }
    match enc as std::os::raw::c_int {
        -1 => { return 0 as xmlCharEncodingHandlerPtr }
        0 => { return 0 as xmlCharEncodingHandlerPtr }
        1 => { return 0 as xmlCharEncodingHandlerPtr }
        2 => { return xmlUTF16LEHandler }
        3 => { return xmlUTF16BEHandler }
        6 => {
            handler =
                xmlFindCharEncodingHandler(b"EBCDIC\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"ebcdic\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"EBCDIC-US\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"IBM-037\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        5 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-10646-UCS-4\x00" as *const u8
                                               as *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"UCS-4\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"UCS4\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        4 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-10646-UCS-4\x00" as *const u8
                                               as *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"UCS-4\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"UCS4\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        9 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-10646-UCS-2\x00" as *const u8
                                               as *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"UCS-2\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"UCS2\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        10 => {
            /*
	     * We used to keep ISO Latin encodings native in the
	     * generated data. This led to so many problems that
	     * this has been removed. One can still change this
	     * back by registering no-ops encoders for those
	     */
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-1\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        11 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-2\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        12 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-3\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        13 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-4\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        14 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-5\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        15 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-6\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        16 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-7\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        17 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-8\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        18 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-8859-9\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        19 => {
            handler =
                xmlFindCharEncodingHandler(b"ISO-2022-JP\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        20 => {
            handler =
                xmlFindCharEncodingHandler(b"SHIFT-JIS\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"SHIFT_JIS\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
            handler =
                xmlFindCharEncodingHandler(b"Shift_JIS\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        21 => {
            handler =
                xmlFindCharEncodingHandler(b"EUC-JP\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if !handler.is_null() { return handler }
        }
        7 | 8 | _ => { }
    }
    return 0 as xmlCharEncodingHandlerPtr;
}
/* *
 * xmlFindCharEncodingHandler:
 * @name:  a string describing the char encoding.
 *
 * Search in the registered set the handler able to read/write that encoding.
 *
 * Returns the handler or NULL if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFindCharEncodingHandler(mut name:
                                                        *const std::os::raw::c_char)
 -> xmlCharEncodingHandlerPtr {
    let mut nalias: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut norig: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut alias: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut enc: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
    let mut icv_in: iconv_t = 0 as *mut std::os::raw::c_void;
    let mut icv_out: iconv_t = 0 as *mut std::os::raw::c_void;
    /* LIBXML_ICONV_ENABLED */
    /* LIBXML_ICU_ENABLED */
    let mut upper: [std::os::raw::c_char; 100] = [0; 100];
    let mut i: std::os::raw::c_int = 0;
    if handlers.is_null() { xmlInitCharEncodingHandlers(); }
    if name.is_null() { return xmlDefaultCharEncodingHandler }
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        return xmlDefaultCharEncodingHandler
    }
    /*
     * Do the alias resolution
     */
    norig = name;
    nalias = xmlGetEncodingAlias(name);
    if !nalias.is_null() { name = nalias }
    /*
     * Check first for directly registered encoding names
     */
    i = 0 as std::os::raw::c_int;
    while i < 99 as std::os::raw::c_int {
        upper[i as usize] =
            ({
                 let mut __res: std::os::raw::c_int = 0;
                 if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong >
                        1 as std::os::raw::c_int as std::os::raw::c_ulong {
                     if 0 != 0 {
                         let mut __c: std::os::raw::c_int =
                             *name.offset(i as isize) as std::os::raw::c_int;
                         __res =
                             if __c < -(128 as std::os::raw::c_int) ||
                                    __c > 255 as std::os::raw::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else {
                         __res =
                             toupper(*name.offset(i as isize) as std::os::raw::c_int)
                     }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*name.offset(i as
                                                                           isize)
                                                              as std::os::raw::c_int
                                                              as isize)
                 }
                 __res
             }) as std::os::raw::c_char;
        if upper[i as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
        i += 1
    }
    upper[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    if !handlers.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nbCharEncodingHandler {
            if strcmp(upper.as_mut_ptr(),
                      (**handlers.offset(i as isize)).name) == 0 {
                return *handlers.offset(i as isize)
            }
            i += 1
        }
    }
    /* check whether iconv can handle this */
    icv_in =
        iconv_open(b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char, name);
    icv_out =
        iconv_open(name, b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char);
    if icv_in == -(1 as std::os::raw::c_int) as iconv_t {
        icv_in =
            iconv_open(b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char,
                       upper.as_mut_ptr())
    }
    if icv_out == -(1 as std::os::raw::c_int) as iconv_t {
        icv_out =
            iconv_open(upper.as_mut_ptr(),
                       b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char)
    }
    if icv_in != -(1 as std::os::raw::c_int) as iconv_t &&
           icv_out != -(1 as std::os::raw::c_int) as iconv_t {
        enc =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlCharEncodingHandler>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlCharEncodingHandlerPtr;
        if enc.is_null() {
            iconv_close(icv_in);
            iconv_close(icv_out);
            return 0 as xmlCharEncodingHandlerPtr
        }
        memset(enc as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlCharEncodingHandler>() as
                   std::os::raw::c_ulong);
        (*enc).name = xmlMemStrdup.expect("non-null function pointer")(name);
        (*enc).input = None;
        (*enc).output = None;
        (*enc).iconv_in = icv_in;
        (*enc).iconv_out = icv_out;
        return enc
    } else {
        if icv_in != -(1 as std::os::raw::c_int) as iconv_t ||
               icv_out != -(1 as std::os::raw::c_int) as iconv_t {
            xmlEncodingErr(XML_ERR_INTERNAL_ERROR,
                           b"iconv : problems with filters for \'%s\'\n\x00"
                               as *const u8 as *const std::os::raw::c_char, name);
        }
    }
    /* LIBXML_ICONV_ENABLED */
    /* LIBXML_ICU_ENABLED */
    /*
     * Fallback using the canonical names
     */
    alias = xmlParseCharEncoding(norig);
    if alias as std::os::raw::c_int != XML_CHAR_ENCODING_ERROR as std::os::raw::c_int {
        let mut canon: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        canon = xmlGetCharEncodingName(alias);
        if !canon.is_null() && strcmp(name, canon) != 0 {
            return xmlFindCharEncodingHandler(canon)
        }
    }
    /* If "none of the above", give up */
    return 0 as xmlCharEncodingHandlerPtr;
}
/* ***********************************************************************
 *									*
 *		ICONV based generic conversion functions		*
 *									*
 ************************************************************************/
/* *
 * xmlIconvWrapper:
 * @cd:		iconv converter data structure
 * @out:  a pointer to an array of bytes to store the result
 * @outlen:  the length of @out
 * @in:  a pointer to an array of ISO Latin 1 chars
 * @inlen:  the length of @in
 *
 * Returns 0 if success, or
 *     -1 by lack of space, or
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 *     -3 if there the last byte can't form a single output char.
 *
 * The value of @inlen after return is the number of octets consumed
 *     as the return value is positive, else unpredictable.
 * The value of @outlen after return is the number of ocetes consumed.
 */
unsafe extern "C" fn xmlIconvWrapper(mut cd: iconv_t,
                                     mut out: *mut std::os::raw::c_uchar,
                                     mut outlen: *mut std::os::raw::c_int,
                                     mut in_0: *const std::os::raw::c_uchar,
                                     mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut icv_inlen: size_t = 0;
    let mut icv_outlen: size_t = 0;
    let mut icv_in: *const std::os::raw::c_char = in_0 as *const std::os::raw::c_char;
    let mut icv_out: *mut std::os::raw::c_char = out as *mut std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() || in_0.is_null()
       {
        if !outlen.is_null() { *outlen = 0 as std::os::raw::c_int }
        return -(1 as std::os::raw::c_int)
    }
    icv_inlen = *inlen as size_t;
    icv_outlen = *outlen as size_t;
    ret =
        iconv(cd,
              &mut icv_in as *mut *const std::os::raw::c_char as
                  *mut *mut std::os::raw::c_char, &mut icv_inlen, &mut icv_out,
              &mut icv_outlen) as std::os::raw::c_int;
    *inlen =
        (*inlen as std::os::raw::c_ulong).wrapping_sub(icv_inlen) as std::os::raw::c_int as
            std::os::raw::c_int;
    *outlen =
        (*outlen as std::os::raw::c_ulong).wrapping_sub(icv_outlen) as std::os::raw::c_int as
            std::os::raw::c_int;
    if icv_inlen != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ret == -(1 as std::os::raw::c_int) {
        if *__errno_location() == 84 as std::os::raw::c_int {
            return -(2 as std::os::raw::c_int)
        } else if *__errno_location() == 7 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        } else if *__errno_location() == 22 as std::os::raw::c_int {
            return -(3 as std::os::raw::c_int)
        } else { return -(3 as std::os::raw::c_int) }
    }
    return 0 as std::os::raw::c_int;
}
/* LIBXML_ICONV_ENABLED */
/* ***********************************************************************
 *									*
 *		ICU based generic conversion functions		*
 *									*
 ************************************************************************/
/* LIBXML_ICU_ENABLED */
/* ***********************************************************************
 *									*
 *		The real API used by libxml for on-the-fly conversion	*
 *									*
 ************************************************************************/
unsafe extern "C" fn xmlEncInputChunk(mut handler:
                                          *mut xmlCharEncodingHandler,
                                      mut out: *mut std::os::raw::c_uchar,
                                      mut outlen: *mut std::os::raw::c_int,
                                      mut in_0: *const std::os::raw::c_uchar,
                                      mut inlen: *mut std::os::raw::c_int,
                                      mut flush: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if (*handler).input.is_some() {
        ret =
            (*handler).input.expect("non-null function pointer")(out, outlen,
                                                                 in_0, inlen)
    } else if !(*handler).iconv_in.is_null() {
        ret = xmlIconvWrapper((*handler).iconv_in, out, outlen, in_0, inlen)
    } else {
        /* LIBXML_ICONV_ENABLED */
        /* LIBXML_ICU_ENABLED */
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        ret = -(2 as std::os::raw::c_int)
    }
    return ret;
}
/* Returns -4 if no output function was found. */
unsafe extern "C" fn xmlEncOutputChunk(mut handler:
                                           *mut xmlCharEncodingHandler,
                                       mut out: *mut std::os::raw::c_uchar,
                                       mut outlen: *mut std::os::raw::c_int,
                                       mut in_0: *const std::os::raw::c_uchar,
                                       mut inlen: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if (*handler).output.is_some() {
        ret =
            (*handler).output.expect("non-null function pointer")(out, outlen,
                                                                  in_0, inlen)
    } else if !(*handler).iconv_out.is_null() {
        ret = xmlIconvWrapper((*handler).iconv_out, out, outlen, in_0, inlen)
    } else {
        /* LIBXML_ICONV_ENABLED */
        /* LIBXML_ICU_ENABLED */
        *outlen = 0 as std::os::raw::c_int;
        *inlen = 0 as std::os::raw::c_int;
        ret = -(4 as std::os::raw::c_int)
    }
    return ret;
}
/*
 * Summary: Internal Interfaces for encoding in libxml2
 * Description: this module describes a few interfaces which were
 *              addded along with the API changes in 2.9.0
 *              those are private routines at this point
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlCharEncFirstLineInt:
 * @handler:	char enconding transformation data structure
 * @out:  an xmlBuffer for the output.
 * @in:  an xmlBuffer for the input
 * @len:  number of bytes to convert for the first line, or -1
 *
 * Front-end for the encoding handler input function, but handle only
 * the very first line, i.e. limit itself to 45 chars.
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncFirstLineInt(mut handler:
                                                    *mut xmlCharEncodingHandler,
                                                mut out: xmlBufferPtr,
                                                mut in_0: xmlBufferPtr,
                                                mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut written: std::os::raw::c_int = 0;
    let mut toconv: std::os::raw::c_int = 0;
    if handler.is_null() { return -(1 as std::os::raw::c_int) }
    if out.is_null() { return -(1 as std::os::raw::c_int) }
    if in_0.is_null() { return -(1 as std::os::raw::c_int) }
    /* calculate space available */
    written =
        (*out).size.wrapping_sub((*out).use_0).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_uint)
            as std::os::raw::c_int; /* count '\0' */
    toconv = (*in_0).use_0 as std::os::raw::c_int;
    /*
     * echo '<?xml version="1.0" encoding="UCS4"?>' | wc -c => 38
     * 45 chars should be sufficient to reach the end of the encoding
     * declaration without going too far inside the document content.
     * on UTF-16 this means 90bytes, on UCS4 this means 180
     * The actual value depending on guessed encoding is passed as @len
     * if provided
     */
    if len >= 0 as std::os::raw::c_int {
        if toconv > len { toconv = len }
    } else if toconv > 180 as std::os::raw::c_int { toconv = 180 as std::os::raw::c_int }
    if toconv * 2 as std::os::raw::c_int >= written {
        xmlBufferGrow(out, (toconv * 2 as std::os::raw::c_int) as std::os::raw::c_uint);
        written =
            (*out).size.wrapping_sub((*out).use_0).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint)
                as std::os::raw::c_int
    }
    ret =
        xmlEncInputChunk(handler,
                         &mut *(*out).content.offset((*out).use_0 as isize),
                         &mut written, (*in_0).content, &mut toconv,
                         0 as std::os::raw::c_int);
    xmlBufferShrink(in_0, toconv as std::os::raw::c_uint);
    (*out).use_0 = (*out).use_0.wrapping_add(written as std::os::raw::c_uint);
    *(*out).content.offset((*out).use_0 as isize) =
        0 as std::os::raw::c_int as xmlChar;
    if ret == -(1 as std::os::raw::c_int) { ret = -(3 as std::os::raw::c_int) }
    /* DEBUG_ENCODING */
    /*
     * Ignore when input buffer is not on a boundary
     */
    if ret == -(3 as std::os::raw::c_int) { ret = 0 as std::os::raw::c_int }
    if ret == -(1 as std::os::raw::c_int) { ret = 0 as std::os::raw::c_int }
    return ret;
}
/* *
 * xmlCharEncFirstLine:
 * @handler:	char enconding transformation data structure
 * @out:  an xmlBuffer for the output.
 * @in:  an xmlBuffer for the input
 *
 * Front-end for the encoding handler input function, but handle only
 * the very first line, i.e. limit itself to 45 chars.
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncFirstLine(mut handler:
                                                 *mut xmlCharEncodingHandler,
                                             mut out: xmlBufferPtr,
                                             mut in_0: xmlBufferPtr)
 -> std::os::raw::c_int {
    return xmlCharEncFirstLineInt(handler, out, in_0, -(1 as std::os::raw::c_int));
}
/* *
 * xmlCharEncFirstLineInput:
 * @input: a parser input buffer
 * @len:  number of bytes to convert for the first line, or -1
 *
 * Front-end for the encoding handler input function, but handle only
 * the very first line. Point is that this is based on autodetection
 * of the encoding and once that first line is converted we may find
 * out that a different decoder is needed to process the input.
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncFirstLineInput(mut input:
                                                      xmlParserInputBufferPtr,
                                                  mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0; /* count '\0' */
    let mut written: size_t = 0;
    let mut toconv: size_t = 0;
    let mut c_in: std::os::raw::c_int = 0;
    let mut c_out: std::os::raw::c_int = 0;
    let mut in_0: xmlBufPtr = 0 as *mut xmlBuf;
    let mut out: xmlBufPtr = 0 as *mut xmlBuf;
    if input.is_null() || (*input).encoder.is_null() ||
           (*input).buffer.is_null() || (*input).raw.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    out = (*input).buffer;
    in_0 = (*input).raw;
    toconv = xmlBufUse(in_0);
    if toconv == 0 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as std::os::raw::c_int }
    written =
        xmlBufAvail(out).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    /*
     * echo '<?xml version="1.0" encoding="UCS4"?>' | wc -c => 38
     * 45 chars should be sufficient to reach the end of the encoding
     * declaration without going too far inside the document content.
     * on UTF-16 this means 90bytes, on UCS4 this means 180
     * The actual value depending on guessed encoding is passed as @len
     * if provided
     */
    if len >= 0 as std::os::raw::c_int {
        if toconv > len as std::os::raw::c_uint as std::os::raw::c_ulong {
            toconv = len as size_t
        }
    } else if toconv > 180 as std::os::raw::c_int as std::os::raw::c_ulong {
        toconv = 180 as std::os::raw::c_int as size_t
    }
    if toconv.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) >= written {
        xmlBufGrow(out,
                   toconv.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                       std::os::raw::c_int);
        written =
            xmlBufAvail(out).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    if written > 360 as std::os::raw::c_int as std::os::raw::c_ulong {
        written = 360 as std::os::raw::c_int as size_t
    }
    c_in = toconv as std::os::raw::c_int;
    c_out = written as std::os::raw::c_int;
    ret =
        xmlEncInputChunk((*input).encoder, xmlBufEnd(out), &mut c_out,
                         xmlBufContent(in_0 as *const xmlBuf), &mut c_in,
                         0 as std::os::raw::c_int);
    xmlBufShrink(in_0, c_in as size_t);
    xmlBufAddLen(out, c_out as size_t);
    if ret == -(1 as std::os::raw::c_int) { ret = -(3 as std::os::raw::c_int) }
    match ret {
        -2 => {
            let mut buf: [std::os::raw::c_char; 50] = [0; 50];
            let mut content: *const xmlChar =
                xmlBufContent(in_0 as *const xmlBuf);
            snprintf(&mut *buf.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)
                         as *mut std::os::raw::c_char,
                     49 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"0x%02X 0x%02X 0x%02X 0x%02X\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     *content.offset(0 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *content.offset(1 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *content.offset(2 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *content.offset(3 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int);
            buf[49 as std::os::raw::c_int as usize] =
                0 as std::os::raw::c_int as std::os::raw::c_char;
            xmlEncodingErr(XML_I18N_CONV_FAILED,
                           b"input conversion failed due to input error, bytes %s\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           buf.as_mut_ptr());
        }
        0 | -1 | -3 | _ => { }
    }
    /*
     * Ignore when input buffer is not on a boundary
     */
    if ret == -(3 as std::os::raw::c_int) { ret = 0 as std::os::raw::c_int }
    if ret == -(1 as std::os::raw::c_int) { ret = 0 as std::os::raw::c_int }
    return ret;
}
/* *
 * xmlCharEncInput:
 * @input: a parser input buffer
 * @flush: try to flush all the raw buffer
 *
 * Generic front-end for the encoding handler on parser input
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncInput(mut input: xmlParserInputBufferPtr,
                                         mut flush: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0; /* count '\0' */
    let mut written: size_t = 0;
    let mut toconv: size_t = 0;
    let mut c_in: std::os::raw::c_int = 0;
    let mut c_out: std::os::raw::c_int = 0;
    let mut in_0: xmlBufPtr = 0 as *mut xmlBuf;
    let mut out: xmlBufPtr = 0 as *mut xmlBuf;
    if input.is_null() || (*input).encoder.is_null() ||
           (*input).buffer.is_null() || (*input).raw.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    out = (*input).buffer;
    in_0 = (*input).raw;
    toconv = xmlBufUse(in_0);
    if toconv == 0 as std::os::raw::c_int as std::os::raw::c_ulong { return 0 as std::os::raw::c_int }
    if toconv > (64 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as std::os::raw::c_ulong &&
           flush == 0 as std::os::raw::c_int {
        toconv = (64 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as size_t
    }
    written = xmlBufAvail(out);
    if written > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        written = written.wrapping_sub(1)
    }
    if toconv.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) >= written {
        xmlBufGrow(out,
                   toconv.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                       std::os::raw::c_int);
        written = xmlBufAvail(out);
        if written > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            written = written.wrapping_sub(1)
        }
        /* count '\0' */
    }
    if written > (128 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as std::os::raw::c_ulong
           && flush == 0 as std::os::raw::c_int {
        written = (128 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as size_t
    }
    c_in = toconv as std::os::raw::c_int;
    c_out = written as std::os::raw::c_int;
    ret =
        xmlEncInputChunk((*input).encoder, xmlBufEnd(out), &mut c_out,
                         xmlBufContent(in_0 as *const xmlBuf), &mut c_in,
                         flush);
    xmlBufShrink(in_0, c_in as size_t);
    xmlBufAddLen(out, c_out as size_t);
    if ret == -(1 as std::os::raw::c_int) { ret = -(3 as std::os::raw::c_int) }
    match ret {
        -2 => {
            let mut buf: [std::os::raw::c_char; 50] = [0; 50];
            let mut content: *const xmlChar =
                xmlBufContent(in_0 as *const xmlBuf);
            snprintf(&mut *buf.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)
                         as *mut std::os::raw::c_char,
                     49 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"0x%02X 0x%02X 0x%02X 0x%02X\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     *content.offset(0 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *content.offset(1 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *content.offset(2 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *content.offset(3 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int);
            buf[49 as std::os::raw::c_int as usize] =
                0 as std::os::raw::c_int as std::os::raw::c_char;
            xmlEncodingErr(XML_I18N_CONV_FAILED,
                           b"input conversion failed due to input error, bytes %s\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           buf.as_mut_ptr());
        }
        0 | -1 | -3 | _ => { }
    }
    /*
     * Ignore when input buffer is not on a boundary
     */
    if ret == -(3 as std::os::raw::c_int) { ret = 0 as std::os::raw::c_int }
    return if c_out != 0 { c_out } else { ret };
}
/* *
 * xmlCharEncInFunc:
 * @handler:	char encoding transformation data structure
 * @out:  an xmlBuffer for the output.
 * @in:  an xmlBuffer for the input
 *
 * Generic front-end for the encoding handler input function
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncInFunc(mut handler:
                                              *mut xmlCharEncodingHandler,
                                          mut out: xmlBufferPtr,
                                          mut in_0: xmlBufferPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0; /* count '\0' */
    let mut written: std::os::raw::c_int = 0;
    let mut toconv: std::os::raw::c_int = 0;
    if handler.is_null() { return -(1 as std::os::raw::c_int) }
    if out.is_null() { return -(1 as std::os::raw::c_int) }
    if in_0.is_null() { return -(1 as std::os::raw::c_int) }
    toconv = (*in_0).use_0 as std::os::raw::c_int;
    if toconv == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    written =
        (*out).size.wrapping_sub((*out).use_0).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_uint)
            as std::os::raw::c_int;
    if toconv * 2 as std::os::raw::c_int >= written {
        xmlBufferGrow(out,
                      (*out).size.wrapping_add((toconv * 2 as std::os::raw::c_int) as
                                                   std::os::raw::c_uint));
        written =
            (*out).size.wrapping_sub((*out).use_0).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint)
                as std::os::raw::c_int
    }
    ret =
        xmlEncInputChunk(handler,
                         &mut *(*out).content.offset((*out).use_0 as isize),
                         &mut written, (*in_0).content, &mut toconv,
                         1 as std::os::raw::c_int);
    xmlBufferShrink(in_0, toconv as std::os::raw::c_uint);
    (*out).use_0 = (*out).use_0.wrapping_add(written as std::os::raw::c_uint);
    *(*out).content.offset((*out).use_0 as isize) =
        0 as std::os::raw::c_int as xmlChar;
    if ret == -(1 as std::os::raw::c_int) { ret = -(3 as std::os::raw::c_int) }
    match ret {
        -2 => {
            let mut buf: [std::os::raw::c_char; 50] = [0; 50];
            snprintf(&mut *buf.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)
                         as *mut std::os::raw::c_char,
                     49 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"0x%02X 0x%02X 0x%02X 0x%02X\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     *(*in_0).content.offset(0 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *(*in_0).content.offset(1 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *(*in_0).content.offset(2 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int,
                     *(*in_0).content.offset(3 as std::os::raw::c_int as isize) as
                         std::os::raw::c_int);
            buf[49 as std::os::raw::c_int as usize] =
                0 as std::os::raw::c_int as std::os::raw::c_char;
            xmlEncodingErr(XML_I18N_CONV_FAILED,
                           b"input conversion failed due to input error, bytes %s\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           buf.as_mut_ptr());
        }
        0 | -1 | -3 | _ => { }
    }
    /*
     * Ignore when input buffer is not on a boundary
     */
    if ret == -(3 as std::os::raw::c_int) { ret = 0 as std::os::raw::c_int }
    return if written != 0 { written } else { ret };
}
/* *
 * xmlCharEncOutput:
 * @output: a parser output buffer
 * @init: is this an initialization call without data
 *
 * Generic front-end for the encoding handler on parser output
 * a first call with @init == 1 has to be made first to initiate the
 * output in case of non-stateless encoding needing to initiate their
 * state or the output (like the BOM in UTF16).
 * In case of UTF8 sequence conversion errors for the given encoder,
 * the content will be automatically remapped to a CharRef sequence.
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncOutput(mut output: xmlOutputBufferPtr,
                                          mut init: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut written: size_t = 0;
    let mut writtentot: size_t = 0 as std::os::raw::c_int as size_t;
    let mut toconv: size_t = 0;
    let mut c_in: std::os::raw::c_int = 0;
    let mut c_out: std::os::raw::c_int = 0;
    let mut in_0: xmlBufPtr = 0 as *mut xmlBuf;
    let mut out: xmlBufPtr = 0 as *mut xmlBuf;
    if output.is_null() || (*output).encoder.is_null() ||
           (*output).buffer.is_null() || (*output).conv.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    out = (*output).conv;
    in_0 = (*output).buffer;
    loop 
         /* Can be a limitation of iconv or uconv */
         {
        written = xmlBufAvail(out); /* count '\0' */
        if written > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            written = written.wrapping_sub(1)
        }
        /*
     * First specific handling of the initialization call
     */
        if init != 0 {
            c_in = 0 as std::os::raw::c_int;
            c_out = written as std::os::raw::c_int;
            /* TODO: Check return value. */
            xmlEncOutputChunk((*output).encoder, xmlBufEnd(out), &mut c_out,
                              0 as *const std::os::raw::c_uchar, &mut c_in);
            xmlBufAddLen(out, c_out as size_t);
            return 0 as std::os::raw::c_int
        }
        /*
     * Conversion itself.
     */
        toconv = xmlBufUse(in_0);
        if toconv == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 0 as std::os::raw::c_int
        }
        if toconv > (64 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as std::os::raw::c_ulong
           {
            toconv = (64 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as size_t
        }
        if toconv.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong) >= written {
            xmlBufGrow(out,
                       toconv.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                           as std::os::raw::c_int);
            written =
                xmlBufAvail(out).wrapping_sub(1 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong)
        }
        if written >
               (256 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as std::os::raw::c_ulong {
            written = (256 as std::os::raw::c_int * 1024 as std::os::raw::c_int) as size_t
        }
        c_in = toconv as std::os::raw::c_int;
        c_out = written as std::os::raw::c_int;
        ret =
            xmlEncOutputChunk((*output).encoder, xmlBufEnd(out), &mut c_out,
                              xmlBufContent(in_0 as *const xmlBuf),
                              &mut c_in);
        xmlBufShrink(in_0, c_in as size_t);
        xmlBufAddLen(out, c_out as size_t);
        writtentot =
            (writtentot as std::os::raw::c_ulong).wrapping_add(c_out as std::os::raw::c_ulong)
                as size_t as size_t;
        if ret == -(1 as std::os::raw::c_int) {
            if c_out > 0 as std::os::raw::c_int { continue ; }
            ret = -(3 as std::os::raw::c_int)
        }
        if ret >= 0 as std::os::raw::c_int { output = output.offset(ret as isize) }
        /*
     * Attempt to handle error cases
     */
        match ret {
            -4 => {
                xmlEncodingErr(XML_I18N_NO_OUTPUT,
                               b"xmlCharEncOutFunc: no output function !\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const std::os::raw::c_char);
                ret = -(1 as std::os::raw::c_int);
                break ;
            }
            -2 => {
                let mut charref: [xmlChar; 20] = [0; 20];
                let mut len: std::os::raw::c_int = xmlBufUse(in_0) as std::os::raw::c_int;
                let mut content: *mut xmlChar =
                    xmlBufContent(in_0 as *const xmlBuf);
                let mut cur: std::os::raw::c_int = 0;
                let mut charrefLen: std::os::raw::c_int = 0;
                cur = xmlGetUTF8Char(content, &mut len);
                if cur <= 0 as std::os::raw::c_int { break ; }
                /*
             * Removes the UTF8 sequence, and replace it by a charref
             * and continue the transcoding phase, hoping the error
             * did not mangle the encoder state.
             */
                charrefLen =
                    snprintf(&mut *charref.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize)
                                 as *mut xmlChar as *mut std::os::raw::c_char,
                             ::std::mem::size_of::<[xmlChar; 20]>() as
                                 std::os::raw::c_ulong,
                             b"&#%d;\x00" as *const u8 as *const std::os::raw::c_char,
                             cur);
                xmlBufShrink(in_0, len as size_t);
                xmlBufGrow(out, charrefLen * 4 as std::os::raw::c_int);
                c_out =
                    xmlBufAvail(out).wrapping_sub(1 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong) as
                        std::os::raw::c_int;
                c_in = charrefLen;
                ret =
                    xmlEncOutputChunk((*output).encoder, xmlBufEnd(out),
                                      &mut c_out, charref.as_mut_ptr(),
                                      &mut c_in);
                if ret < 0 as std::os::raw::c_int || c_in != charrefLen {
                    let mut buf: [std::os::raw::c_char; 50] = [0; 50];
                    snprintf(&mut *buf.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                               isize) as
                                 *mut std::os::raw::c_char,
                             49 as std::os::raw::c_int as std::os::raw::c_ulong,
                             b"0x%02X 0x%02X 0x%02X 0x%02X\x00" as *const u8
                                 as *const std::os::raw::c_char,
                             *content.offset(0 as std::os::raw::c_int as isize) as
                                 std::os::raw::c_int,
                             *content.offset(1 as std::os::raw::c_int as isize) as
                                 std::os::raw::c_int,
                             *content.offset(2 as std::os::raw::c_int as isize) as
                                 std::os::raw::c_int,
                             *content.offset(3 as std::os::raw::c_int as isize) as
                                 std::os::raw::c_int);
                    buf[49 as std::os::raw::c_int as usize] =
                        0 as std::os::raw::c_int as std::os::raw::c_char;
                    xmlEncodingErr(XML_I18N_CONV_FAILED,
                                   b"output conversion failed due to conv error, bytes %s\n\x00"
                                       as *const u8 as *const std::os::raw::c_char,
                                   buf.as_mut_ptr());
                    if xmlBufGetAllocationScheme(in_0) !=
                           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int {
                        *content.offset(0 as std::os::raw::c_int as isize) =
                            ' ' as i32 as xmlChar
                    }
                    break ;
                } else {
                    xmlBufAddLen(out, c_out as size_t);
                    writtentot =
                        (writtentot as
                             std::os::raw::c_ulong).wrapping_add(c_out as
                                                             std::os::raw::c_ulong) as
                            size_t as size_t
                }
            }
            0 | -1 | -3 | _ => { break ; }
        }
    }
    return ret;
}
/* *
 * xmlCharEncOutFunc:
 * @handler:	char enconding transformation data structure
 * @out:  an xmlBuffer for the output.
 * @in:  an xmlBuffer for the input
 *
 * Generic front-end for the encoding handler output function
 * a first call with @in == NULL has to be made firs to initiate the
 * output in case of non-stateless encoding needing to initiate their
 * state or the output (like the BOM in UTF16).
 * In case of UTF8 sequence conversion errors for the given encoder,
 * the content will be automatically remapped to a CharRef sequence.
 *
 * Returns the number of byte written if success, or
 *     -1 general error
 *     -2 if the transcoding fails (for *in is not valid utf8 string or
 *        the result of transformation can't fit into the encoding we want), or
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncOutFunc(mut handler:
                                               *mut xmlCharEncodingHandler,
                                           mut out: xmlBufferPtr,
                                           mut in_0: xmlBufferPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut written: std::os::raw::c_int = 0;
    let mut writtentot: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut toconv: std::os::raw::c_int = 0;
    let mut output: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if handler.is_null() { return -(1 as std::os::raw::c_int) }
    if out.is_null() { return -(1 as std::os::raw::c_int) }
    loop 
         /* Can be a limitation of iconv or uconv */
         {
        written =
            (*out).size.wrapping_sub((*out).use_0) as
                std::os::raw::c_int; /* Gennady: count '/0' */
        if written > 0 as std::os::raw::c_int { written -= 1 }
        /*
     * First specific handling of in = NULL, i.e. the initialization call
     */
        if in_0.is_null() {
            toconv = 0 as std::os::raw::c_int;
            /* TODO: Check return value. */
            xmlEncOutputChunk(handler,
                              &mut *(*out).content.offset((*out).use_0 as
                                                              isize),
                              &mut written, 0 as *const std::os::raw::c_uchar,
                              &mut toconv);
            (*out).use_0 = (*out).use_0.wrapping_add(written as std::os::raw::c_uint);
            *(*out).content.offset((*out).use_0 as isize) =
                0 as std::os::raw::c_int as xmlChar;
            return 0 as std::os::raw::c_int
        }
        /*
     * Conversion itself.
     */
        toconv = (*in_0).use_0 as std::os::raw::c_int;
        if toconv == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
        if toconv * 4 as std::os::raw::c_int >= written {
            xmlBufferGrow(out, (toconv * 4 as std::os::raw::c_int) as std::os::raw::c_uint);
            written =
                (*out).size.wrapping_sub((*out).use_0).wrapping_sub(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_uint)
                    as std::os::raw::c_int
        }
        ret =
            xmlEncOutputChunk(handler,
                              &mut *(*out).content.offset((*out).use_0 as
                                                              isize),
                              &mut written, (*in_0).content, &mut toconv);
        xmlBufferShrink(in_0, toconv as std::os::raw::c_uint);
        (*out).use_0 = (*out).use_0.wrapping_add(written as std::os::raw::c_uint);
        writtentot += written;
        *(*out).content.offset((*out).use_0 as isize) =
            0 as std::os::raw::c_int as xmlChar;
        if ret == -(1 as std::os::raw::c_int) {
            if written > 0 as std::os::raw::c_int { continue ; }
            ret = -(3 as std::os::raw::c_int)
        }
        if ret >= 0 as std::os::raw::c_int { output += ret }
        /*
     * Attempt to handle error cases
     */
        match ret {
            -4 => {
                xmlEncodingErr(XML_I18N_NO_OUTPUT,
                               b"xmlCharEncOutFunc: no output function !\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const std::os::raw::c_char);
                ret = -(1 as std::os::raw::c_int);
                break ;
            }
            -2 => {
                let mut charref: [xmlChar; 20] = [0; 20];
                let mut len: std::os::raw::c_int = (*in_0).use_0 as std::os::raw::c_int;
                let mut utf: *const xmlChar =
                    (*in_0).content as *const xmlChar;
                let mut cur: std::os::raw::c_int = 0;
                let mut charrefLen: std::os::raw::c_int = 0;
                cur = xmlGetUTF8Char(utf, &mut len);
                if cur <= 0 as std::os::raw::c_int { break ; }
                /*
             * Removes the UTF8 sequence, and replace it by a charref
             * and continue the transcoding phase, hoping the error
             * did not mangle the encoder state.
             */
                charrefLen =
                    snprintf(&mut *charref.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize)
                                 as *mut xmlChar as *mut std::os::raw::c_char,
                             ::std::mem::size_of::<[xmlChar; 20]>() as
                                 std::os::raw::c_ulong,
                             b"&#%d;\x00" as *const u8 as *const std::os::raw::c_char,
                             cur);
                xmlBufferShrink(in_0, len as std::os::raw::c_uint);
                xmlBufferGrow(out,
                              (charrefLen * 4 as std::os::raw::c_int) as
                                  std::os::raw::c_uint);
                written =
                    (*out).size.wrapping_sub((*out).use_0).wrapping_sub(1 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_uint)
                        as std::os::raw::c_int;
                toconv = charrefLen;
                ret =
                    xmlEncOutputChunk(handler,
                                      &mut *(*out).content.offset((*out).use_0
                                                                      as
                                                                      isize),
                                      &mut written, charref.as_mut_ptr(),
                                      &mut toconv);
                if ret < 0 as std::os::raw::c_int || toconv != charrefLen {
                    let mut buf: [std::os::raw::c_char; 50] = [0; 50];
                    snprintf(&mut *buf.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                               isize) as
                                 *mut std::os::raw::c_char,
                             49 as std::os::raw::c_int as std::os::raw::c_ulong,
                             b"0x%02X 0x%02X 0x%02X 0x%02X\x00" as *const u8
                                 as *const std::os::raw::c_char,
                             *(*in_0).content.offset(0 as std::os::raw::c_int as
                                                         isize) as
                                 std::os::raw::c_int,
                             *(*in_0).content.offset(1 as std::os::raw::c_int as
                                                         isize) as
                                 std::os::raw::c_int,
                             *(*in_0).content.offset(2 as std::os::raw::c_int as
                                                         isize) as
                                 std::os::raw::c_int,
                             *(*in_0).content.offset(3 as std::os::raw::c_int as
                                                         isize) as
                                 std::os::raw::c_int);
                    buf[49 as std::os::raw::c_int as usize] =
                        0 as std::os::raw::c_int as std::os::raw::c_char;
                    xmlEncodingErr(XML_I18N_CONV_FAILED,
                                   b"output conversion failed due to conv error, bytes %s\n\x00"
                                       as *const u8 as *const std::os::raw::c_char,
                                   buf.as_mut_ptr());
                    if (*in_0).alloc as std::os::raw::c_uint !=
                           XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        *(*in_0).content.offset(0 as std::os::raw::c_int as isize) =
                            ' ' as i32 as xmlChar
                    }
                    break ;
                } else {
                    (*out).use_0 =
                        (*out).use_0.wrapping_add(written as std::os::raw::c_uint);
                    writtentot += written;
                    *(*out).content.offset((*out).use_0 as isize) =
                        0 as std::os::raw::c_int as xmlChar
                }
            }
            0 | -1 | -3 | _ => { break ; }
        }
    }
    return ret;
}
/* LIBXML_ICONV_ENABLED */
/* LIBXML_ICU_ENABLED */
/*
 * Interfaces for encoding handlers.
 */
/*
 * Interfaces for encoding names and aliases.
 */
/*
 * Interfaces directly used by the parsers.
 */
/* *
 * xmlCharEncCloseFunc:
 * @handler:	char enconding transformation data structure
 *
 * Generic front-end for encoding handler close function
 *
 * Returns 0 if success, or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCharEncCloseFunc(mut handler:
                                                 *mut xmlCharEncodingHandler)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tofree: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut handler_in_list: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if handler.is_null() { return -(1 as std::os::raw::c_int) }
    if (*handler).name.is_null() { return -(1 as std::os::raw::c_int) }
    if !handlers.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nbCharEncodingHandler {
            if handler == *handlers.offset(i as isize) {
                handler_in_list = 1 as std::os::raw::c_int;
                break ;
            } else { i += 1 }
        }
    }
    /*
     * Iconv handlers can be used only once, free the whole block.
     * and the associated icon resources.
     */
    if handler_in_list == 0 as std::os::raw::c_int &&
           (!(*handler).iconv_out.is_null() || !(*handler).iconv_in.is_null())
       {
        tofree = 1 as std::os::raw::c_int;
        if !(*handler).iconv_out.is_null() {
            if iconv_close((*handler).iconv_out) != 0 {
                ret = -(1 as std::os::raw::c_int)
            }
            (*handler).iconv_out = 0 as *mut std::os::raw::c_void
        }
        if !(*handler).iconv_in.is_null() {
            if iconv_close((*handler).iconv_in) != 0 {
                ret = -(1 as std::os::raw::c_int)
            }
            (*handler).iconv_in = 0 as *mut std::os::raw::c_void
        }
    }
    /* LIBXML_ICONV_ENABLED */
    if tofree != 0 {
        /* free up only dynamic handlers iconv/uconv */
        if !(*handler).name.is_null() {
            xmlFree.expect("non-null function pointer")((*handler).name as
                                                            *mut std::os::raw::c_void);
        }
        (*handler).name = 0 as *mut std::os::raw::c_char;
        xmlFree.expect("non-null function pointer")(handler as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
/*
 * SAX Version 1
 */
/* unused error() get all the errors */
/* *
 * xmlExternalEntityLoader:
 * @URL: The System ID of the resource requested
 * @ID: The Public ID of the resource requested
 * @context: the XML parser context
 *
 * External entity loaders types.
 *
 * Returns the entity input parser.
 */
/*
 * Init/Cleanup
 */
/*
 * Input functions
 */
/*
 * Basic parsing Interfaces
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Recovery mode
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Less common routines and SAX interfaces
 */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_VALID_ENABLE */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_SAX1_ENABLED */
/*
 * Parser contexts handling.
 */
/* LIBXML_SAX1_ENABLED */
/*
 * Reading/setting optional parsing features.
 */
/* LIBXML_LEGACY_ENABLED */
/*
 * Interfaces for the Push mode.
 */
/* LIBXML_PUSH_ENABLED */
/*
 * Special I/O mode.
 */
/*
 * Node infos.
 */
/*
 * External entities handling actually implemented in xmlIO.
 */
/*
 * Index lookup, actually implemented in the encoding module
 */
/* *
 * xmlByteConsumed:
 * @ctxt: an XML parser context
 *
 * This function provides the current index of the parser relative
 * to the start of the current entity. This function is computed in
 * bytes from the beginning starting at zero and finishing at the
 * size in byte of the file if parsing a file. The function is
 * of constant cost if the input is UTF-8 but can be costly if run
 * on non-UTF-8 input.
 *
 * Returns the index in bytes from the beginning of the entity or -1
 *         in case the index could not be computed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlByteConsumed(mut ctxt: xmlParserCtxtPtr)
 -> std::os::raw::c_long {
    let mut in_0: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) as std::os::raw::c_long }
    in_0 = (*ctxt).input;
    if in_0.is_null() { return -(1 as std::os::raw::c_int) as std::os::raw::c_long }
    if !(*in_0).buf.is_null() && !(*(*in_0).buf).encoder.is_null() {
        let mut unused: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut handler: *mut xmlCharEncodingHandler = (*(*in_0).buf).encoder;
        /*
	 * Encoding conversion, compute the number of unused original
	 * bytes from the input not consumed and substract that from
	 * the raw consumed value, this is not a cheap operation
	 */
        if (*in_0).end.offset_from((*in_0).cur) as std::os::raw::c_long >
               0 as std::os::raw::c_int as std::os::raw::c_long {
            let mut convbuf: [std::os::raw::c_uchar; 32000] = [0; 32000];
            let mut cur: *const std::os::raw::c_uchar =
                (*in_0).cur as *const std::os::raw::c_uchar;
            let mut toconv: std::os::raw::c_int =
                (*in_0).end.offset_from((*in_0).cur) as std::os::raw::c_long
                    as std::os::raw::c_int;
            let mut written: std::os::raw::c_int = 32000 as std::os::raw::c_int;
            let mut ret: std::os::raw::c_int = 0;
            loop  {
                toconv =
                    (*in_0).end.offset_from(cur) as std::os::raw::c_long as
                        std::os::raw::c_int;
                written = 32000 as std::os::raw::c_int;
                ret =
                    xmlEncOutputChunk(handler,
                                      &mut *convbuf.as_mut_ptr().offset(0 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize),
                                      &mut written, cur, &mut toconv);
                if ret < 0 as std::os::raw::c_int {
                    if written > 0 as std::os::raw::c_int {
                        ret = -(2 as std::os::raw::c_int)
                    } else { return -(1 as std::os::raw::c_int) as std::os::raw::c_long }
                }
                unused = unused.wrapping_add(written as std::os::raw::c_uint);
                cur = cur.offset(toconv as isize);
                if !(ret == -(2 as std::os::raw::c_int)) { break ; }
            }
        }
        if (*(*in_0).buf).rawconsumed < unused as std::os::raw::c_ulong {
            return -(1 as std::os::raw::c_int) as std::os::raw::c_long
        }
        return (*(*in_0).buf).rawconsumed.wrapping_sub(unused as
                                                           std::os::raw::c_ulong) as
                   std::os::raw::c_long
    }
    return (*in_0).consumed.wrapping_add((*in_0).cur.offset_from((*in_0).base)
                                             as std::os::raw::c_long as std::os::raw::c_ulong)
               as std::os::raw::c_long;
}
/* __INCLUDE_ELFGCCHACK */
