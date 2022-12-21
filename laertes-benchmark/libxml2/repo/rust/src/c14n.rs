
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    /*
 * Summary: lists interfaces
 * Description: this module implement the list support used in
 * various place in the library.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Gary Pennington <Gary.Pennington@uk.sun.com>
 */
    pub type _xmlLink;
    pub type _xmlList;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    /*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlNewNsProp(node: xmlNodePtr, ns: xmlNsPtr, name: *const xmlChar,
                    value: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlFreePropList(cur: xmlAttrPtr);
    /*
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr,
                   nameSpace: *const xmlChar) -> xmlNsPtr;
    #[no_mangle]
    fn xmlHasNsProp(node: *const xmlNode, name: *const xmlChar,
                    nameSpace: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlNodeListGetString(doc: xmlDocPtr, list: *const xmlNode,
                            inLine: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlGetCompressMode() -> std::os::raw::c_int;
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
    /* Creation/Deletion */
    #[no_mangle]
    fn xmlListCreate(deallocator: xmlListDeallocator,
                     compare: xmlListDataCompare) -> xmlListPtr;
    #[no_mangle]
    fn xmlListDelete(l: xmlListPtr);
    /* Basic Operators */
    #[no_mangle]
    fn xmlListSearch(l: xmlListPtr, data: *mut std::os::raw::c_void)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlListInsert(l: xmlListPtr, data: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlListWalk(l: xmlListPtr, walker: xmlListWalker,
                   user: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlAllocOutputBuffer(encoder: xmlCharEncodingHandlerPtr)
     -> xmlOutputBufferPtr;
    #[no_mangle]
    fn xmlOutputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                     encoder: xmlCharEncodingHandlerPtr,
                                     compression: std::os::raw::c_int)
     -> xmlOutputBufferPtr;
    #[no_mangle]
    fn xmlOutputBufferWriteString(out: xmlOutputBufferPtr,
                                  str: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> std::os::raw::c_int;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    /* *
 * NodeSet handling.
 */
    #[no_mangle]
    fn xmlXPathNodeSetContains(cur: xmlNodeSetPtr, val: xmlNodePtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufWriteQuotedString(buf: xmlBufPtr, string: *const xmlChar)
     -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
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
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
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
pub type xmlCharEncodingOutputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
pub type xmlCharEncodingInputFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_uchar, _: *mut std::os::raw::c_int,
                                _: *const std::os::raw::c_uchar, _: *mut std::os::raw::c_int)
               -> std::os::raw::c_int>;
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
pub type xmlDocPtr = *mut xmlDoc;
/* *
 * xmlDoc:
 *
 * An XML document.
 */
pub type xmlDoc = _xmlDoc;
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
pub type xmlNsPtr = *mut xmlNs;
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
pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
/* *
 * xmlListDeallocator:
 * @lk:  the data to deallocate
 *
 * Callback function used to free data from a list.
 */
pub type xmlListDeallocator
    =
    Option<unsafe extern "C" fn(_: xmlLinkPtr) -> ()>;
/* *
 * xmlListDataCompare:
 * @data0: the first data
 * @data1: the second data
 *
 * Callback function used to compare 2 data.
 *
 * Returns 0 is equality, -1 or 1 otherwise depending on the ordering.
 */
pub type xmlListDataCompare
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                _: *const std::os::raw::c_void) -> std::os::raw::c_int>;
/* *
 * xmlListWalker:
 * @data: the data found in the list
 * @user: extra user provided data to the walker
 *
 * Callback function used when walking a list with xmlListWalk().
 *
 * Returns 0 to stop walking the list, 1 otherwise.
 */
pub type xmlListWalker
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_void, _: *mut std::os::raw::c_void)
               -> std::os::raw::c_int>;
pub type C2RustUnnamed_1 = std::os::raw::c_int;
pub const XML_CHAR_ENCODING_ASCII: C2RustUnnamed_1 = 22;
pub const XML_CHAR_ENCODING_EUC_JP: C2RustUnnamed_1 = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: C2RustUnnamed_1 = 20;
pub const XML_CHAR_ENCODING_2022_JP: C2RustUnnamed_1 = 19;
pub const XML_CHAR_ENCODING_8859_9: C2RustUnnamed_1 = 18;
pub const XML_CHAR_ENCODING_8859_8: C2RustUnnamed_1 = 17;
pub const XML_CHAR_ENCODING_8859_7: C2RustUnnamed_1 = 16;
pub const XML_CHAR_ENCODING_8859_6: C2RustUnnamed_1 = 15;
pub const XML_CHAR_ENCODING_8859_5: C2RustUnnamed_1 = 14;
pub const XML_CHAR_ENCODING_8859_4: C2RustUnnamed_1 = 13;
pub const XML_CHAR_ENCODING_8859_3: C2RustUnnamed_1 = 12;
pub const XML_CHAR_ENCODING_8859_2: C2RustUnnamed_1 = 11;
pub const XML_CHAR_ENCODING_8859_1: C2RustUnnamed_1 = 10;
pub const XML_CHAR_ENCODING_UCS2: C2RustUnnamed_1 = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: C2RustUnnamed_1 = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: C2RustUnnamed_1 = 7;
pub const XML_CHAR_ENCODING_EBCDIC: C2RustUnnamed_1 = 6;
pub const XML_CHAR_ENCODING_UCS4BE: C2RustUnnamed_1 = 5;
pub const XML_CHAR_ENCODING_UCS4LE: C2RustUnnamed_1 = 4;
pub const XML_CHAR_ENCODING_UTF16BE: C2RustUnnamed_1 = 3;
pub const XML_CHAR_ENCODING_UTF16LE: C2RustUnnamed_1 = 2;
pub const XML_CHAR_ENCODING_UTF8: C2RustUnnamed_1 = 1;
pub const XML_CHAR_ENCODING_NONE: C2RustUnnamed_1 = 0;
pub const XML_CHAR_ENCODING_ERROR: C2RustUnnamed_1 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut std::os::raw::c_char,
    pub opaque: *mut std::os::raw::c_char,
    pub authority: *mut std::os::raw::c_char,
    pub server: *mut std::os::raw::c_char,
    pub user: *mut std::os::raw::c_char,
    pub port: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub query: *mut std::os::raw::c_char,
    pub fragment: *mut std::os::raw::c_char,
    pub cleanup: std::os::raw::c_int,
    pub query_raw: *mut std::os::raw::c_char,
}
/* *
 * Summary: library of generic URI related routines
 * Description: library of generic URI related routines
 *              Implements RFC 2396
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlURI:
 *
 * A parsed URI reference. This is a struct containing the various fields
 * as described in RFC 2396 but separated for further processing.
 *
 * Note: query is a deprecated field which is incorrectly unescaped.
 * query_raw takes precedence over query if the former is set.
 * See: http://mail.gnome.org/archives/xml/2007-April/thread.html#00127
 */
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlC14NMode = std::os::raw::c_uint;
pub const XML_C14N_1_1: xmlC14NMode = 2;
pub const XML_C14N_EXCLUSIVE_1_0: xmlC14NMode = 1;
pub const XML_C14N_1_0: xmlC14NMode = 0;
/* *
 * This is the core C14N function
 */
/* *
 * xmlC14NIsVisibleCallback:
 * @user_data: user data
 * @node: the curent node
 * @parent: the parent node
 *
 * Signature for a C14N callback on visible nodes
 *
 * Returns 1 if the node should be included
 */
pub type xmlC14NIsVisibleCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlNodePtr,
                                _: xmlNodePtr) -> std::os::raw::c_int>;
pub type xmlC14NCtxPtr = *mut _xmlC14NCtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlC14NCtx {
    pub doc: xmlDocPtr,
    pub is_visible_callback: xmlC14NIsVisibleCallback,
    pub user_data: *mut std::os::raw::c_void,
    pub with_comments: std::os::raw::c_int,
    pub buf: xmlOutputBufferPtr,
    pub pos: xmlC14NPosition,
    pub parent_is_doc: std::os::raw::c_int,
    pub ns_rendered: xmlC14NVisibleNsStackPtr,
    pub mode: xmlC14NMode,
    pub inclusive_ns_prefixes: *mut *mut xmlChar,
    pub error: std::os::raw::c_int,
}
pub type xmlC14NVisibleNsStackPtr = *mut _xmlC14NVisibleNsStack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlC14NVisibleNsStack {
    pub nsCurEnd: std::os::raw::c_int,
    pub nsPrevStart: std::os::raw::c_int,
    pub nsPrevEnd: std::os::raw::c_int,
    pub nsMax: std::os::raw::c_int,
    pub nsTab: *mut xmlNsPtr,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlC14NPosition = std::os::raw::c_uint;
pub const XMLC14N_AFTER_DOCUMENT_ELEMENT: xmlC14NPosition = 2;
pub const XMLC14N_INSIDE_DOCUMENT_ELEMENT: xmlC14NPosition = 1;
pub const XMLC14N_BEFORE_DOCUMENT_ELEMENT: xmlC14NPosition = 0;
pub type xmlC14NVisibleNsStack = _xmlC14NVisibleNsStack;
pub type xmlC14NNormalizationMode = std::os::raw::c_uint;
pub const XMLC14N_NORMALIZE_TEXT: xmlC14NNormalizationMode = 3;
pub const XMLC14N_NORMALIZE_PI: xmlC14NNormalizationMode = 2;
pub const XMLC14N_NORMALIZE_COMMENT: xmlC14NNormalizationMode = 1;
pub const XMLC14N_NORMALIZE_ATTR: xmlC14NNormalizationMode = 0;
pub type xmlC14NCtx = _xmlC14NCtx;
/* input parameters */
/* position in the XML document */
/* C14N mode */
/* exclusive canonicalization */
/* error number */
/* number of nodes in the set */
/* the begginning of the stack for previous visible node */
/* the end of the stack for previous visible node */
/* size of the array as allocated */
/* array of ns in no particular order */
/* array of nodes in no particular order */
/* ***********************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/
/* *
 * xmlC14NErrMemory:
 * @extra:  extra informations
 *
 * Handle a redefinition of memory error
 */
unsafe extern "C" fn xmlC14NErrMemory(mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int,
                    XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Memory allocation failed : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, extra);
}
/* *
 * xmlC14NErrParam:
 * @extra:  extra informations
 *
 * Handle a redefinition of param error
 */
unsafe extern "C" fn xmlC14NErrParam(mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int,
                    XML_ERR_INTERNAL_ERROR as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Invalid parameter : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, extra);
}
/* *
 * xmlC14NErrInternal:
 * @extra:  extra informations
 *
 * Handle a redefinition of internal error
 */
unsafe extern "C" fn xmlC14NErrInternal(mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int,
                    XML_ERR_INTERNAL_ERROR as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Internal error : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, extra);
}
/* *
 * xmlC14NErrInvalidNode:
 * @extra:  extra informations
 *
 * Handle a redefinition of invalid node error
 */
unsafe extern "C" fn xmlC14NErrInvalidNode(mut node_type: *const std::os::raw::c_char,
                                           mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int,
                    XML_C14N_INVALID_NODE as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Node %s is invalid here : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, node_type, extra);
}
/* *
 * xmlC14NErrUnknownNode:
 * @extra:  extra informations
 *
 * Handle a redefinition of unknown node error
 */
unsafe extern "C" fn xmlC14NErrUnknownNode(mut node_type: std::os::raw::c_int,
                                           mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int,
                    XML_C14N_UNKNOW_NODE as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Unknown node type %d found : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, node_type, extra);
}
/* *
 * xmlC14NErrRelativeNamespace:
 * @extra:  extra informations
 *
 * Handle a redefinition of relative namespace error
 */
unsafe extern "C" fn xmlC14NErrRelativeNamespace(mut ns_uri:
                                                     *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int,
                    XML_C14N_RELATIVE_NAMESPACE as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                    b"Relative namespace UR is invalid here : %s\n\x00" as
                        *const u8 as *const std::os::raw::c_char, ns_uri);
}
/* *
 * xmlC14NErr:
 * @ctxt:  a C14N evaluation context
 * @node:  the context node
 * @error:  the erorr code
 * @msg:  the message
 * @extra:  extra informations
 *
 * Handle a redefinition of attribute error
 */
unsafe extern "C" fn xmlC14NErr(mut ctxt: xmlC14NCtxPtr, mut node: xmlNodePtr,
                                mut error: std::os::raw::c_int,
                                mut msg: *const std::os::raw::c_char) {
    if !ctxt.is_null() { (*ctxt).error = error }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, node as *mut std::os::raw::c_void,
                    XML_FROM_C14N as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int,
                    b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg);
}
unsafe extern "C" fn xmlC14NIsNodeInNodeset(mut user_data: *mut std::os::raw::c_void,
                                            mut node: xmlNodePtr,
                                            mut parent: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut nodes: xmlNodeSetPtr = user_data as xmlNodeSetPtr;
    if !nodes.is_null() && !node.is_null() {
        if (*node).type_0 as std::os::raw::c_uint !=
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return xmlXPathNodeSetContains(nodes, node)
        } else {
            let mut ns: xmlNs =
                xmlNs{next: 0 as *mut _xmlNs,
                      type_0: 0 as xmlNsType,
                      href: 0 as *const xmlChar,
                      prefix: 0 as *const xmlChar,
                      _private: 0 as *mut std::os::raw::c_void,
                      context: 0 as *mut _xmlDoc,};
            memcpy(&mut ns as *mut xmlNs as *mut std::os::raw::c_void,
                   node as *const std::os::raw::c_void,
                   ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
            /* this is a libxml hack! check xpath.c for details */
            if !parent.is_null() &&
                   (*parent).type_0 as std::os::raw::c_uint ==
                       XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                ns.next = (*parent).parent as xmlNsPtr
            } else { ns.next = parent as xmlNsPtr }
            /*
	     * If the input is an XPath node-set, then the node-set must explicitly
	     * contain every node to be rendered to the canonical form.
	     */
            return xmlXPathNodeSetContains(nodes,
                                           &mut ns as *mut xmlNs as
                                               xmlNodePtr)
        }
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn xmlC14NVisibleNsStackCreate()
 -> xmlC14NVisibleNsStackPtr {
    let mut ret: xmlC14NVisibleNsStackPtr = 0 as *mut _xmlC14NVisibleNsStack;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlC14NVisibleNsStack>()
                                                          as std::os::raw::c_ulong) as
            xmlC14NVisibleNsStackPtr;
    if ret.is_null() {
        xmlC14NErrMemory(b"creating namespaces stack\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlC14NVisibleNsStackPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlC14NVisibleNsStack>() as std::os::raw::c_ulong);
    return ret;
}
unsafe extern "C" fn xmlC14NVisibleNsStackDestroy(mut cur:
                                                      xmlC14NVisibleNsStackPtr) {
    if cur.is_null() {
        xmlC14NErrParam(b"destroying namespaces stack\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return
    }
    if !(*cur).nsTab.is_null() {
        memset((*cur).nsTab as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ((*cur).nsMax as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                    as std::os::raw::c_ulong));
        xmlFree.expect("non-null function pointer")((*cur).nsTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).nodeTab.is_null() {
        memset((*cur).nodeTab as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ((*cur).nsMax as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                    as std::os::raw::c_ulong));
        xmlFree.expect("non-null function pointer")((*cur).nodeTab as
                                                        *mut std::os::raw::c_void);
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlC14NVisibleNsStack>() as std::os::raw::c_ulong);
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
unsafe extern "C" fn xmlC14NVisibleNsStackAdd(mut cur:
                                                  xmlC14NVisibleNsStackPtr,
                                              mut ns: xmlNsPtr,
                                              mut node: xmlNodePtr) {
    if cur.is_null() || (*cur).nsTab.is_null() && !(*cur).nodeTab.is_null() ||
           !(*cur).nsTab.is_null() && (*cur).nodeTab.is_null() {
        xmlC14NErrParam(b"adding namespace to stack\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return
    }
    if (*cur).nsTab.is_null() && (*cur).nodeTab.is_null() {
        (*cur).nsTab =
            xmlMalloc.expect("non-null function pointer")((16 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNsPtr;
        (*cur).nodeTab =
            xmlMalloc.expect("non-null function pointer")((16 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if (*cur).nsTab.is_null() || (*cur).nodeTab.is_null() {
            xmlC14NErrMemory(b"adding node to stack\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        memset((*cur).nsTab as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               (16 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                    as std::os::raw::c_ulong));
        memset((*cur).nodeTab as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               (16 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                    as std::os::raw::c_ulong));
        (*cur).nsMax = 16 as std::os::raw::c_int
    } else if (*cur).nsMax == (*cur).nsCurEnd {
        let mut tmp: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
        let mut tmpSize: std::os::raw::c_int = 0;
        tmpSize = 2 as std::os::raw::c_int * (*cur).nsMax;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*cur).nsTab as
                                                               *mut std::os::raw::c_void,
                                                           (tmpSize as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNsPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong));
        if tmp.is_null() {
            xmlC14NErrMemory(b"adding node to stack\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        (*cur).nsTab = tmp as *mut xmlNsPtr;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*cur).nodeTab as
                                                               *mut std::os::raw::c_void,
                                                           (tmpSize as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong));
        if tmp.is_null() {
            xmlC14NErrMemory(b"adding node to stack\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        (*cur).nodeTab = tmp as *mut xmlNodePtr;
        (*cur).nsMax = tmpSize
    }
    let ref mut fresh0 = *(*cur).nsTab.offset((*cur).nsCurEnd as isize);
    *fresh0 = ns;
    let ref mut fresh1 = *(*cur).nodeTab.offset((*cur).nsCurEnd as isize);
    *fresh1 = node;
    (*cur).nsCurEnd += 1;
}
unsafe extern "C" fn xmlC14NVisibleNsStackSave(mut cur:
                                                   xmlC14NVisibleNsStackPtr,
                                               mut state:
                                                   xmlC14NVisibleNsStackPtr) {
    if cur.is_null() || state.is_null() {
        xmlC14NErrParam(b"saving namespaces stack\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return
    }
    (*state).nsCurEnd = (*cur).nsCurEnd;
    (*state).nsPrevStart = (*cur).nsPrevStart;
    (*state).nsPrevEnd = (*cur).nsPrevEnd;
}
unsafe extern "C" fn xmlC14NVisibleNsStackRestore(mut cur:
                                                      xmlC14NVisibleNsStackPtr,
                                                  mut state:
                                                      xmlC14NVisibleNsStackPtr) {
    if cur.is_null() || state.is_null() {
        xmlC14NErrParam(b"restoring namespaces stack\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return
    }
    (*cur).nsCurEnd = (*state).nsCurEnd;
    (*cur).nsPrevStart = (*state).nsPrevStart;
    (*cur).nsPrevEnd = (*state).nsPrevEnd;
}
unsafe extern "C" fn xmlC14NVisibleNsStackShift(mut cur:
                                                    xmlC14NVisibleNsStackPtr) {
    if cur.is_null() {
        xmlC14NErrParam(b"shifting namespaces stack\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return
    }
    (*cur).nsPrevStart = (*cur).nsPrevEnd;
    (*cur).nsPrevEnd = (*cur).nsCurEnd;
}
unsafe extern "C" fn xmlC14NStrEqual(mut str1: *const xmlChar,
                                     mut str2: *const xmlChar)
 -> std::os::raw::c_int {
    if str1 == str2 { return 1 as std::os::raw::c_int }
    if str1.is_null() {
        return (*str2 as std::os::raw::c_int == '\u{0}' as i32) as std::os::raw::c_int
    }
    if str2.is_null() {
        return (*str1 as std::os::raw::c_int == '\u{0}' as i32) as std::os::raw::c_int
    }
    loop  {
        let fresh2 = str1;
        str1 = str1.offset(1);
        if *fresh2 as std::os::raw::c_int != *str2 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int
        }
        let fresh3 = str2;
        str2 = str2.offset(1);
        if !(*fresh3 != 0) { break ; }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlC14NVisibleNsStackFind:
 * @ctx:		the C14N context
 * @ns:			the namespace to check
 *
 * Checks whether the given namespace was already rendered or not
 *
 * Returns 1 if we already wrote this namespace or 0 otherwise
 */
unsafe extern "C" fn xmlC14NVisibleNsStackFind(mut cur:
                                                   xmlC14NVisibleNsStackPtr,
                                               mut ns: xmlNsPtr)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut prefix: *const xmlChar = 0 as *const xmlChar;
    let mut href: *const xmlChar = 0 as *const xmlChar;
    let mut has_empty_ns: std::os::raw::c_int = 0;
    if cur.is_null() {
        xmlC14NErrParam(b"searching namespaces stack (c14n)\x00" as *const u8
                            as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    /*
     * if the default namespace xmlns="" is not defined yet then
     * we do not want to print it out
     */
    prefix =
        if ns.is_null() || (*ns).prefix.is_null() {
            b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
        } else { (*ns).prefix };
    href =
        if ns.is_null() || (*ns).href.is_null() {
            b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
        } else { (*ns).href };
    has_empty_ns =
        (xmlC14NStrEqual(prefix, 0 as *const xmlChar) != 0 &&
             xmlC14NStrEqual(href, 0 as *const xmlChar) != 0) as std::os::raw::c_int;
    if !(*cur).nsTab.is_null() {
        let mut start: std::os::raw::c_int =
            if has_empty_ns != 0 {
                0 as std::os::raw::c_int
            } else { (*cur).nsPrevStart };
        i = (*cur).nsCurEnd - 1 as std::os::raw::c_int;
        while i >= start {
            let mut ns1: xmlNsPtr = *(*cur).nsTab.offset(i as isize);
            if xmlC14NStrEqual(prefix,
                               if !ns1.is_null() {
                                   (*ns1).prefix
                               } else { 0 as *const xmlChar }) != 0 {
                return xmlC14NStrEqual(href,
                                       if !ns1.is_null() {
                                           (*ns1).href
                                       } else { 0 as *const xmlChar })
            }
            i -= 1
        }
    }
    return has_empty_ns;
}
unsafe extern "C" fn xmlExcC14NVisibleNsStackFind(mut cur:
                                                      xmlC14NVisibleNsStackPtr,
                                                  mut ns: xmlNsPtr,
                                                  mut ctx: xmlC14NCtxPtr)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut prefix: *const xmlChar = 0 as *const xmlChar;
    let mut href: *const xmlChar = 0 as *const xmlChar;
    let mut has_empty_ns: std::os::raw::c_int = 0;
    if cur.is_null() {
        xmlC14NErrParam(b"searching namespaces stack (exc c14n)\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    /*
     * if the default namespace xmlns="" is not defined yet then
     * we do not want to print it out
     */
    prefix =
        if ns.is_null() || (*ns).prefix.is_null() {
            b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
        } else { (*ns).prefix };
    href =
        if ns.is_null() || (*ns).href.is_null() {
            b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
        } else { (*ns).href };
    has_empty_ns =
        (xmlC14NStrEqual(prefix, 0 as *const xmlChar) != 0 &&
             xmlC14NStrEqual(href, 0 as *const xmlChar) != 0) as std::os::raw::c_int;
    if !(*cur).nsTab.is_null() {
        let mut start: std::os::raw::c_int = 0 as std::os::raw::c_int;
        i = (*cur).nsCurEnd - 1 as std::os::raw::c_int;
        while i >= start {
            let mut ns1: xmlNsPtr = *(*cur).nsTab.offset(i as isize);
            if xmlC14NStrEqual(prefix,
                               if !ns1.is_null() {
                                   (*ns1).prefix
                               } else { 0 as *const xmlChar }) != 0 {
                if xmlC14NStrEqual(href,
                                   if !ns1.is_null() {
                                       (*ns1).href
                                   } else { 0 as *const xmlChar }) != 0 {
                    return if (*ctx).is_visible_callback.is_some() {
                               (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                              ns1
                                                                                                  as
                                                                                                  xmlNodePtr,
                                                                                              *(*cur).nodeTab.offset(i
                                                                                                                         as
                                                                                                                         isize))
                           } else { 1 as std::os::raw::c_int }
                } else { return 0 as std::os::raw::c_int }
            }
            i -= 1
        }
    }
    return has_empty_ns;
}
/* *
 * xmlC14NIsXmlNs:
 * @ns:		the namespace to check
 *
 * Checks whether the given namespace is a default "xml:" namespace
 * with href="http://www.w3.org/XML/1998/namespace"
 *
 * Returns 1 if the node is default or 0 otherwise
 */
/* todo: make it a define? */
unsafe extern "C" fn xmlC14NIsXmlNs(mut ns: xmlNsPtr) -> std::os::raw::c_int {
    return (!ns.is_null() &&
                xmlStrEqual((*ns).prefix,
                            b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                                *mut xmlChar) != 0 &&
                xmlStrEqual((*ns).href,
                            b"http://www.w3.org/XML/1998/namespace\x00" as
                                *const u8 as *const std::os::raw::c_char as
                                *const xmlChar) != 0) as std::os::raw::c_int;
}
/* *
 * xmlC14NNsCompare:
 * @ns1:		the pointer to first namespace
 * @ns2:		the pointer to second namespace
 *
 * Compares the namespaces by names (prefixes).
 *
 * Returns -1 if ns1 < ns2, 0 if ns1 == ns2 or 1 if ns1 > ns2.
 */
unsafe extern "C" fn xmlC14NNsCompare(mut data1: *const std::os::raw::c_void,
                                      mut data2: *const std::os::raw::c_void)
 -> std::os::raw::c_int {
    let ns1: xmlNsPtr = data1 as xmlNsPtr;
    let ns2: xmlNsPtr = data2 as xmlNsPtr;
    if ns1 == ns2 { return 0 as std::os::raw::c_int }
    if ns1.is_null() { return -(1 as std::os::raw::c_int) }
    if ns2.is_null() { return 1 as std::os::raw::c_int }
    return xmlStrcmp((*ns1).prefix, (*ns2).prefix);
}
/* *
 * xmlC14NPrintNamespaces:
 * @ns:			the pointer to namespace
 * @ctx:		the C14N context
 *
 * Prints the given namespace to the output buffer from C14N context.
 *
 * Returns 1 on success or 0 on fail.
 */
unsafe extern "C" fn xmlC14NPrintNamespaces(ns: xmlNsPtr,
                                            mut ctx: xmlC14NCtxPtr)
 -> std::os::raw::c_int {
    if ns.is_null() || ctx.is_null() {
        xmlC14NErrParam(b"writing namespaces\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    if !(*ns).prefix.is_null() {
        xmlOutputBufferWriteString((*ctx).buf,
                                   b" xmlns:\x00" as *const u8 as
                                       *const std::os::raw::c_char);
        xmlOutputBufferWriteString((*ctx).buf,
                                   (*ns).prefix as *const std::os::raw::c_char);
        xmlOutputBufferWriteString((*ctx).buf,
                                   b"=\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    } else {
        xmlOutputBufferWriteString((*ctx).buf,
                                   b" xmlns=\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    }
    if !(*ns).href.is_null() {
        xmlBufWriteQuotedString((*(*ctx).buf).buffer, (*ns).href);
    } else {
        xmlOutputBufferWriteString((*ctx).buf,
                                   b"\"\"\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn xmlC14NPrintNamespacesWalker(mut ns: *const std::os::raw::c_void,
                                                  mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    return xmlC14NPrintNamespaces(ns as xmlNsPtr, ctx as xmlC14NCtxPtr);
}
/* *
 * xmlC14NProcessNamespacesAxis:
 * @ctx:		the C14N context
 * @node:		the current node
 *
 * Prints out canonical namespace axis of the current node to the
 * buffer from C14N context as follows
 *
 * Canonical XML v 1.0 (http://www.w3.org/TR/xml-c14n)
 *
 * Namespace Axis
 * Consider a list L containing only namespace nodes in the
 * axis and in the node-set in lexicographic order (ascending). To begin
 * processing L, if the first node is not the default namespace node (a node
 * with no namespace URI and no local name), then generate a space followed
 * by xmlns="" if and only if the following conditions are met:
 *    - the element E that owns the axis is in the node-set
 *    - The nearest ancestor element of E in the node-set has a default
 *	    namespace node in the node-set (default namespace nodes always
 *      have non-empty values in XPath)
 * The latter condition eliminates unnecessary occurrences of xmlns="" in
 * the canonical form since an element only receives an xmlns="" if its
 * default namespace is empty and if it has an immediate parent in the
 * canonical form that has a non-empty default namespace. To finish
 * processing  L, simply process every namespace node in L, except omit
 * namespace node with local name xml, which defines the xml prefix,
 * if its string value is http://www.w3.org/XML/1998/namespace.
 *
 * Exclusive XML Canonicalization v 1.0 (http://www.w3.org/TR/xml-exc-c14n)
 * Canonical XML applied to a document subset requires the search of the
 * ancestor nodes of each orphan element node for attributes in the xml
 * namespace, such as xml:lang and xml:space. These are copied into the
 * element node except if a declaration of the same attribute is already
 * in the attribute axis of the element (whether or not it is included in
 * the document subset). This search and copying are omitted from the
 * Exclusive XML Canonicalization method.
 *
 * Returns 0 on success or -1 on fail.
 */
unsafe extern "C" fn xmlC14NProcessNamespacesAxis(mut ctx: xmlC14NCtxPtr,
                                                  mut cur: xmlNodePtr,
                                                  mut visible: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut n: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut tmp: xmlNsPtr = 0 as *mut xmlNs;
    let mut list: xmlListPtr = 0 as *mut xmlList;
    let mut already_rendered: std::os::raw::c_int = 0;
    let mut has_empty_ns: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctx.is_null() || cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlC14NErrParam(b"processing namespaces axis (c14n)\x00" as *const u8
                            as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Create a sorted list to store element namespaces
     */
    list =
        xmlListCreate(None,
                      Some(xmlC14NNsCompare as
                               unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                                    _: *const std::os::raw::c_void)
                                   -> std::os::raw::c_int));
    if list.is_null() {
        xmlC14NErrInternal(b"creating namespaces list (c14n)\x00" as *const u8
                               as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /* check all namespaces */
    n = cur;
    while !n.is_null() {
        ns = (*n).nsDef;
        while !ns.is_null() {
            tmp = xmlSearchNs((*cur).doc, cur, (*ns).prefix);
            if tmp == ns && xmlC14NIsXmlNs(ns) == 0 &&
                   (if (*ctx).is_visible_callback.is_some() {
                        (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                       ns
                                                                                           as
                                                                                           xmlNodePtr,
                                                                                       cur)
                    } else { 1 as std::os::raw::c_int }) != 0 {
                already_rendered =
                    xmlC14NVisibleNsStackFind((*ctx).ns_rendered, ns);
                if visible != 0 {
                    xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, ns, cur);
                }
                if already_rendered == 0 {
                    xmlListInsert(list, ns as *mut std::os::raw::c_void);
                }
                if xmlStrlen((*ns).prefix) == 0 as std::os::raw::c_int {
                    has_empty_ns = 1 as std::os::raw::c_int
                }
            }
            ns = (*ns).next
        }
        n = (*n).parent
    }
    /* *
     * if the first node is not the default namespace node (a node with no
     * namespace URI and no local name), then generate a space followed by
     * xmlns="" if and only if the following conditions are met:
     *  - the element E that owns the axis is in the node-set
     *  - the nearest ancestor element of E in the node-set has a default
     *     namespace node in the node-set (default namespace nodes always
     *     have non-empty values in XPath)
     */
    if visible != 0 && has_empty_ns == 0 {
        static mut ns_default: xmlNs =
            xmlNs{next: 0 as *mut _xmlNs,
                  type_0: 0 as xmlNsType,
                  href: 0 as *const xmlChar,
                  prefix: 0 as *const xmlChar,
                  _private: 0 as *mut std::os::raw::c_void,
                  context: 0 as *mut _xmlDoc,};
        memset(&mut ns_default as *mut xmlNs as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
        if xmlC14NVisibleNsStackFind((*ctx).ns_rendered, &mut ns_default) == 0
           {
            xmlC14NPrintNamespaces(&mut ns_default, ctx);
        }
    }
    /*
     * print out all elements from list
     */
    xmlListWalk(list,
                Some(xmlC14NPrintNamespacesWalker as
                         unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void)
                             -> std::os::raw::c_int), ctx as *mut std::os::raw::c_void);
    /*
     * Cleanup
     */
    xmlListDelete(list);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlExcC14NProcessNamespacesAxis:
 * @ctx:		the C14N context
 * @node:		the current node
 *
 * Prints out exclusive canonical namespace axis of the current node to the
 * buffer from C14N context as follows
 *
 * Exclusive XML Canonicalization
 * http://www.w3.org/TR/xml-exc-c14n
 *
 * If the element node is in the XPath subset then output the node in
 * accordance with Canonical XML except for namespace nodes which are
 * rendered as follows:
 *
 * 1. Render each namespace node iff:
 *    * it is visibly utilized by the immediate parent element or one of
 *      its attributes, or is present in InclusiveNamespaces PrefixList, and
 *    * its prefix and value do not appear in ns_rendered. ns_rendered is
 *      obtained by popping the state stack in order to obtain a list of
 *      prefixes and their values which have already been rendered by
 *      an output ancestor of the namespace node's parent element.
 * 2. Append the rendered namespace node to the list ns_rendered of namespace
 * nodes rendered by output ancestors. Push ns_rendered on state stack and
 * recurse.
 * 3. After the recursion returns, pop thestate stack.
 *
 *
 * Returns 0 on success or -1 on fail.
 */
unsafe extern "C" fn xmlExcC14NProcessNamespacesAxis(mut ctx: xmlC14NCtxPtr,
                                                     mut cur: xmlNodePtr,
                                                     mut visible: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut list: xmlListPtr = 0 as *mut xmlList;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut already_rendered: std::os::raw::c_int = 0;
    let mut has_empty_ns: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut has_visibly_utilized_empty_ns: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut has_empty_ns_in_inclusive_list: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctx.is_null() || cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlC14NErrParam(b"processing namespaces axis (exc c14n)\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    if !((*ctx).mode as std::os::raw::c_uint ==
             XML_C14N_EXCLUSIVE_1_0 as std::os::raw::c_int as std::os::raw::c_uint) {
        xmlC14NErrParam(b"processing namespaces axis (exc c14n)\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Create a sorted list to store element namespaces
     */
    list =
        xmlListCreate(None,
                      Some(xmlC14NNsCompare as
                               unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                                    _: *const std::os::raw::c_void)
                                   -> std::os::raw::c_int));
    if list.is_null() {
        xmlC14NErrInternal(b"creating namespaces list (exc c14n)\x00" as
                               *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * process inclusive namespaces:
     * All namespace nodes appearing on inclusive ns list are
     * handled as provided in Canonical XML
     */
    if !(*ctx).inclusive_ns_prefixes.is_null() {
        let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while !(*(*ctx).inclusive_ns_prefixes.offset(i as isize)).is_null() {
            prefix = *(*ctx).inclusive_ns_prefixes.offset(i as isize);
            /*
	     * Special values for namespace with empty prefix
	     */
            if xmlStrEqual(prefix,
                           b"#default\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 ||
                   xmlStrEqual(prefix,
                               b"\x00" as *const u8 as *const std::os::raw::c_char as
                                   *mut xmlChar) != 0 {
                prefix = 0 as *mut xmlChar;
                has_empty_ns_in_inclusive_list = 1 as std::os::raw::c_int
            }
            ns = xmlSearchNs((*cur).doc, cur, prefix);
            if !ns.is_null() && xmlC14NIsXmlNs(ns) == 0 &&
                   (if (*ctx).is_visible_callback.is_some() {
                        (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                       ns
                                                                                           as
                                                                                           xmlNodePtr,
                                                                                       cur)
                    } else { 1 as std::os::raw::c_int }) != 0 {
                already_rendered =
                    xmlC14NVisibleNsStackFind((*ctx).ns_rendered, ns);
                if visible != 0 {
                    xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, ns, cur);
                }
                if already_rendered == 0 {
                    xmlListInsert(list, ns as *mut std::os::raw::c_void);
                }
                if xmlStrlen((*ns).prefix) == 0 as std::os::raw::c_int {
                    has_empty_ns = 1 as std::os::raw::c_int
                }
            }
            i += 1
        }
    }
    /* add node namespace */
    if !(*cur).ns.is_null() {
        ns = (*cur).ns
    } else {
        ns = xmlSearchNs((*cur).doc, cur, 0 as *const xmlChar);
        has_visibly_utilized_empty_ns = 1 as std::os::raw::c_int
    }
    if !ns.is_null() && xmlC14NIsXmlNs(ns) == 0 {
        if visible != 0 &&
               (if (*ctx).is_visible_callback.is_some() {
                    (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                   ns
                                                                                       as
                                                                                       xmlNodePtr,
                                                                                   cur)
                } else { 1 as std::os::raw::c_int }) != 0 {
            if xmlExcC14NVisibleNsStackFind((*ctx).ns_rendered, ns, ctx) == 0
               {
                xmlListInsert(list, ns as *mut std::os::raw::c_void);
            }
        }
        if visible != 0 {
            xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, ns, cur);
        }
        if xmlStrlen((*ns).prefix) == 0 as std::os::raw::c_int {
            has_empty_ns = 1 as std::os::raw::c_int
        }
    }
    /* add attributes */
    attr = (*cur).properties;
    while !attr.is_null() {
        /*
         * we need to check that attribute is visible and has non
         * default namespace (XML Namespaces: "default namespaces
	 * do not apply directly to attributes")
         */
        if !(*attr).ns.is_null() && xmlC14NIsXmlNs((*attr).ns) == 0 &&
               (if (*ctx).is_visible_callback.is_some() {
                    (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                   attr
                                                                                       as
                                                                                       xmlNodePtr,
                                                                                   cur)
                } else { 1 as std::os::raw::c_int }) != 0 {
            already_rendered =
                xmlExcC14NVisibleNsStackFind((*ctx).ns_rendered, (*attr).ns,
                                             ctx);
            xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, (*attr).ns, cur);
            if already_rendered == 0 && visible != 0 {
                xmlListInsert(list, (*attr).ns as *mut std::os::raw::c_void);
            }
            if xmlStrlen((*(*attr).ns).prefix) == 0 as std::os::raw::c_int {
                has_empty_ns = 1 as std::os::raw::c_int
            }
        } else if !(*attr).ns.is_null() &&
                      xmlStrlen((*(*attr).ns).prefix) == 0 as std::os::raw::c_int &&
                      xmlStrlen((*(*attr).ns).href) == 0 as std::os::raw::c_int {
            has_visibly_utilized_empty_ns = 1 as std::os::raw::c_int
        }
        attr = (*attr).next
    }
    /*
     * Process xmlns=""
     */
    if visible != 0 && has_visibly_utilized_empty_ns != 0 && has_empty_ns == 0
           && has_empty_ns_in_inclusive_list == 0 {
        static mut ns_default: xmlNs =
            xmlNs{next: 0 as *mut _xmlNs,
                  type_0: 0 as xmlNsType,
                  href: 0 as *const xmlChar,
                  prefix: 0 as *const xmlChar,
                  _private: 0 as *mut std::os::raw::c_void,
                  context: 0 as *mut _xmlDoc,};
        memset(&mut ns_default as *mut xmlNs as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
        already_rendered =
            xmlExcC14NVisibleNsStackFind((*ctx).ns_rendered, &mut ns_default,
                                         ctx);
        if already_rendered == 0 {
            xmlC14NPrintNamespaces(&mut ns_default, ctx);
        }
    } else if visible != 0 && has_empty_ns == 0 &&
                  has_empty_ns_in_inclusive_list != 0 {
        static mut ns_default_0: xmlNs =
            xmlNs{next: 0 as *mut _xmlNs,
                  type_0: 0 as xmlNsType,
                  href: 0 as *const xmlChar,
                  prefix: 0 as *const xmlChar,
                  _private: 0 as *mut std::os::raw::c_void,
                  context: 0 as *mut _xmlDoc,};
        memset(&mut ns_default_0 as *mut xmlNs as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNs>() as std::os::raw::c_ulong);
        if xmlC14NVisibleNsStackFind((*ctx).ns_rendered, &mut ns_default_0) ==
               0 {
            xmlC14NPrintNamespaces(&mut ns_default_0, ctx);
        }
    }
    /*
     * print out all elements from list
     */
    xmlListWalk(list,
                Some(xmlC14NPrintNamespacesWalker as
                         unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void)
                             -> std::os::raw::c_int), ctx as *mut std::os::raw::c_void);
    /*
     * Cleanup
     */
    xmlListDelete(list);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlC14NIsXmlAttr:
 * @attr:		the attr to check
 *
 * Checks whether the given attribute is a default "xml:" namespace
 * with href="http://www.w3.org/XML/1998/namespace"
 *
 * Returns 1 if the node is default or 0 otherwise
 */
/* todo: make it a define? */
unsafe extern "C" fn xmlC14NIsXmlAttr(mut attr: xmlAttrPtr) -> std::os::raw::c_int {
    return (!(*attr).ns.is_null() &&
                xmlC14NIsXmlNs((*attr).ns) != 0 as std::os::raw::c_int) as
               std::os::raw::c_int;
}
/* *
 * xmlC14NAttrsCompare:
 * @attr1:		the pointer tls o first attr
 * @attr2:		the pointer to second attr
 *
 * Prints the given attribute to the output buffer from C14N context.
 *
 * Returns -1 if attr1 < attr2, 0 if attr1 == attr2 or 1 if attr1 > attr2.
 */
unsafe extern "C" fn xmlC14NAttrsCompare(mut data1: *const std::os::raw::c_void,
                                         mut data2: *const std::os::raw::c_void)
 -> std::os::raw::c_int {
    let attr1: xmlAttrPtr = data1 as xmlAttrPtr;
    let attr2: xmlAttrPtr = data2 as xmlAttrPtr;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
     * Simple cases
     */
    if attr1 == attr2 { return 0 as std::os::raw::c_int }
    if attr1.is_null() { return -(1 as std::os::raw::c_int) }
    if attr2.is_null() { return 1 as std::os::raw::c_int }
    if (*attr1).ns == (*attr2).ns {
        return xmlStrcmp((*attr1).name, (*attr2).name)
    }
    /*
     * Attributes in the default namespace are first
     * because the default namespace is not applied to
     * unqualified attributes
     */
    if (*attr1).ns.is_null() { return -(1 as std::os::raw::c_int) }
    if (*attr2).ns.is_null() { return 1 as std::os::raw::c_int }
    if (*(*attr1).ns).prefix.is_null() { return -(1 as std::os::raw::c_int) }
    if (*(*attr2).ns).prefix.is_null() { return 1 as std::os::raw::c_int }
    ret = xmlStrcmp((*(*attr1).ns).href, (*(*attr2).ns).href);
    if ret == 0 as std::os::raw::c_int {
        ret = xmlStrcmp((*attr1).name, (*attr2).name)
    }
    return ret;
}
/* *
 * xmlC14NPrintAttrs:
 * @attr:		the pointer to attr
 * @ctx:		the C14N context
 *
 * Prints out canonical attribute urrent node to the
 * buffer from C14N context as follows
 *
 * Canonical XML v 1.0 (http://www.w3.org/TR/xml-c14n)
 *
 * Returns 1 on success or 0 on fail.
 */
unsafe extern "C" fn xmlC14NPrintAttrs(mut data: *const std::os::raw::c_void,
                                       mut user: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let attr: xmlAttrPtr = data as xmlAttrPtr;
    let mut ctx: xmlC14NCtxPtr = user as xmlC14NCtxPtr;
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    if attr.is_null() || ctx.is_null() {
        xmlC14NErrParam(b"writing attributes\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    xmlOutputBufferWriteString((*ctx).buf,
                               b" \x00" as *const u8 as *const std::os::raw::c_char);
    if !(*attr).ns.is_null() &&
           xmlStrlen((*(*attr).ns).prefix) > 0 as std::os::raw::c_int {
        xmlOutputBufferWriteString((*ctx).buf,
                                   (*(*attr).ns).prefix as
                                       *const std::os::raw::c_char);
        xmlOutputBufferWriteString((*ctx).buf,
                                   b":\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    }
    xmlOutputBufferWriteString((*ctx).buf,
                               (*attr).name as *const std::os::raw::c_char);
    xmlOutputBufferWriteString((*ctx).buf,
                               b"=\"\x00" as *const u8 as
                                   *const std::os::raw::c_char);
    value =
        xmlNodeListGetString((*ctx).doc, (*attr).children, 1 as std::os::raw::c_int);
    /* todo: should we log an error if value==NULL ? */
    if !value.is_null() {
        buffer = xmlC11NNormalizeString(value, XMLC14N_NORMALIZE_ATTR);
        xmlFree.expect("non-null function pointer")(value as
                                                        *mut std::os::raw::c_void);
        if !buffer.is_null() {
            xmlOutputBufferWriteString((*ctx).buf,
                                       buffer as *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(buffer as
                                                            *mut std::os::raw::c_void);
        } else {
            xmlC14NErrInternal(b"normalizing attributes axis\x00" as *const u8
                                   as *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    xmlOutputBufferWriteString((*ctx).buf,
                               b"\"\x00" as *const u8 as *const std::os::raw::c_char);
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlC14NFindHiddenParentAttr:
 *
 * Finds an attribute in a hidden parent node.
 *
 * Returns a pointer to the attribute node (if found) or NULL otherwise.
 */
unsafe extern "C" fn xmlC14NFindHiddenParentAttr(mut ctx: xmlC14NCtxPtr,
                                                 mut cur: xmlNodePtr,
                                                 mut name: *const xmlChar,
                                                 mut ns: *const xmlChar)
 -> xmlAttrPtr {
    let mut res: xmlAttrPtr = 0 as *mut xmlAttr;
    while !cur.is_null() &&
              (if (*ctx).is_visible_callback.is_some() {
                   (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                  cur,
                                                                                  (*cur).parent
                                                                                      as
                                                                                      xmlNodePtr)
               } else { 1 as std::os::raw::c_int }) == 0 {
        res = xmlHasNsProp(cur as *const xmlNode, name, ns);
        if !res.is_null() { return res }
        cur = (*cur).parent
    }
    return 0 as xmlAttrPtr;
}
/* *
 * xmlC14NFixupBaseAttr:
 *
 * Fixes up the xml:base attribute
 *
 * Returns the newly created attribute or NULL
 */
unsafe extern "C" fn xmlC14NFixupBaseAttr(mut ctx: xmlC14NCtxPtr,
                                          mut xml_base_attr: xmlAttrPtr)
 -> xmlAttrPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut tmp_str: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp_str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp_str_len: std::os::raw::c_int = 0;
    if ctx.is_null() || xml_base_attr.is_null() ||
           (*xml_base_attr).parent.is_null() {
        xmlC14NErrParam(b"processing xml:base attribute\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as xmlAttrPtr
    }
    /* start from current value */
    res =
        xmlNodeListGetString((*ctx).doc, (*xml_base_attr).children,
                             1 as std::os::raw::c_int);
    if res.is_null() {
        xmlC14NErrInternal(b"processing xml:base attribute - can\'t get attr value\x00"
                               as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlAttrPtr
    }
    /* go up the stack until we find a node that we rendered already */
    cur = (*(*xml_base_attr).parent).parent;
    while !cur.is_null() &&
              (if (*ctx).is_visible_callback.is_some() {
                   (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                  cur,
                                                                                  (*cur).parent
                                                                                      as
                                                                                      xmlNodePtr)
               } else { 1 as std::os::raw::c_int }) == 0 {
        attr =
            xmlHasNsProp(cur as *const xmlNode,
                         b"base\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"http://www.w3.org/XML/1998/namespace\x00" as
                             *const u8 as *const std::os::raw::c_char as
                             *const xmlChar);
        if !attr.is_null() {
            /* get attr value */
            tmp_str =
                xmlNodeListGetString((*ctx).doc, (*attr).children,
                                     1 as std::os::raw::c_int);
            if tmp_str.is_null() {
                xmlFree.expect("non-null function pointer")(res as
                                                                *mut std::os::raw::c_void);
                xmlC14NErrInternal(b"processing xml:base attribute - can\'t get attr value\x00"
                                       as *const u8 as *const std::os::raw::c_char);
                return 0 as xmlAttrPtr
            }
            /* we need to add '/' if our current base uri ends with '..' or '.'
            to ensure that we are forced to go "up" all the time */
            tmp_str_len = xmlStrlen(tmp_str);
            if tmp_str_len > 1 as std::os::raw::c_int &&
                   *tmp_str.offset((tmp_str_len - 2 as std::os::raw::c_int) as isize)
                       as std::os::raw::c_int == '.' as i32 {
                tmp_str2 =
                    xmlStrcat(tmp_str,
                              b"/\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar);
                if tmp_str2.is_null() {
                    xmlFree.expect("non-null function pointer")(tmp_str as
                                                                    *mut std::os::raw::c_void);
                    xmlFree.expect("non-null function pointer")(res as
                                                                    *mut std::os::raw::c_void);
                    xmlC14NErrInternal(b"processing xml:base attribute - can\'t modify uri\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char);
                    return 0 as xmlAttrPtr
                }
                tmp_str = tmp_str2
            }
            /* build uri */
            tmp_str2 = xmlBuildURI(res, tmp_str);
            if tmp_str2.is_null() {
                xmlFree.expect("non-null function pointer")(tmp_str as
                                                                *mut std::os::raw::c_void);
                xmlFree.expect("non-null function pointer")(res as
                                                                *mut std::os::raw::c_void);
                xmlC14NErrInternal(b"processing xml:base attribute - can\'t construct uri\x00"
                                       as *const u8 as *const std::os::raw::c_char);
                return 0 as xmlAttrPtr
            }
            /* cleanup and set the new res */
            xmlFree.expect("non-null function pointer")(tmp_str as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(res as
                                                            *mut std::os::raw::c_void);
            res = tmp_str2
        }
        /* next */
        cur = (*cur).parent
    }
    /* check if result uri is empty or not */
    if res.is_null() ||
           xmlStrEqual(res,
                       b"\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
        xmlFree.expect("non-null function pointer")(res as *mut std::os::raw::c_void);
        return 0 as xmlAttrPtr
    }
    /* create and return the new attribute node */
    attr =
        xmlNewNsProp(0 as xmlNodePtr, (*xml_base_attr).ns,
                     b"base\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut xmlChar, res);
    if attr.is_null() {
        xmlFree.expect("non-null function pointer")(res as *mut std::os::raw::c_void);
        xmlC14NErrInternal(b"processing xml:base attribute - can\'t construct attribute\x00"
                               as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlAttrPtr
    }
    /* done */
    xmlFree.expect("non-null function pointer")(res as *mut std::os::raw::c_void);
    return attr;
}
/* *
 * xmlC14NProcessAttrsAxis:
 * @ctx:		the C14N context
 * @cur:		the current node
 * @parent_visible:	the visibility of parent node
 * @all_parents_visible: the visibility of all parent nodes
 *
 * Prints out canonical attribute axis of the current node to the
 * buffer from C14N context as follows
 *
 * Canonical XML v 1.0 (http://www.w3.org/TR/xml-c14n)
 *
 * Attribute Axis
 * In lexicographic order (ascending), process each node that
 * is in the element's attribute axis and in the node-set.
 *
 * The processing of an element node E MUST be modified slightly
 * when an XPath node-set is given as input and the element's
 * parent is omitted from the node-set.
 *
 *
 * Exclusive XML Canonicalization v 1.0 (http://www.w3.org/TR/xml-exc-c14n)
 *
 * Canonical XML applied to a document subset requires the search of the
 * ancestor nodes of each orphan element node for attributes in the xml
 * namespace, such as xml:lang and xml:space. These are copied into the
 * element node except if a declaration of the same attribute is already
 * in the attribute axis of the element (whether or not it is included in
 * the document subset). This search and copying are omitted from the
 * Exclusive XML Canonicalization method.
 *
 * Returns 0 on success or -1 on fail.
 */
unsafe extern "C" fn xmlC14NProcessAttrsAxis(mut ctx: xmlC14NCtxPtr,
                                             mut cur: xmlNodePtr,
                                             mut parent_visible: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut list: xmlListPtr = 0 as *mut xmlList;
    let mut attrs_to_delete: xmlAttrPtr = 0 as xmlAttrPtr;
    /* special processing for 1.1 spec */
    let mut xml_base_attr: xmlAttrPtr = 0 as xmlAttrPtr;
    let mut xml_lang_attr: xmlAttrPtr = 0 as xmlAttrPtr;
    let mut xml_space_attr: xmlAttrPtr = 0 as xmlAttrPtr;
    if ctx.is_null() || cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlC14NErrParam(b"processing attributes axis\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Create a sorted list to store element attributes
     */
    list =
        xmlListCreate(None,
                      Some(xmlC14NAttrsCompare as
                               unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                                    _: *const std::os::raw::c_void)
                                   -> std::os::raw::c_int));
    if list.is_null() {
        xmlC14NErrInternal(b"creating attributes list\x00" as *const u8 as
                               *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    match (*ctx).mode as std::os::raw::c_uint {
        0 => {
            /* The processing of an element node E MUST be modified slightly when an XPath node-set is
         * given as input and the element's parent is omitted from the node-set. The method for processing
         * the attribute axis of an element E in the node-set is enhanced. All element nodes along E's
         * ancestor axis are examined for nearest occurrences of attributes in the xml namespace, such
         * as xml:lang and xml:space (whether or not they are in the node-set). From this list of attributes,
         * remove any that are in E's attribute axis (whether or not they are in the node-set). Then,
         * lexicographically merge this attribute list with the nodes of E's attribute axis that are in
         * the node-set. The result of visiting the attribute axis is computed by processing the attribute
         * nodes in this merged attribute list.
         */
            /*
         * Add all visible attributes from current node.
         */
            attr = (*cur).properties;
            while !attr.is_null() {
                /* check that attribute is visible */
                if if (*ctx).is_visible_callback.is_some() {
                       (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                      attr
                                                                                          as
                                                                                          xmlNodePtr,
                                                                                      cur)
                   } else { 1 as std::os::raw::c_int } != 0 {
                    xmlListInsert(list, attr as *mut std::os::raw::c_void);
                }
                attr = (*attr).next
            }
            /*
         * Handle xml attributes
         */
            if parent_visible != 0 && !(*cur).parent.is_null() &&
                   (if (*ctx).is_visible_callback.is_some() {
                        (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                       (*cur).parent
                                                                                           as
                                                                                           xmlNodePtr,
                                                                                       (*(*cur).parent).parent
                                                                                           as
                                                                                           xmlNodePtr)
                    } else { 1 as std::os::raw::c_int }) == 0 {
                let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                /*
             * If XPath node-set is not specified then the parent is always
             * visible!
             */
                tmp = (*cur).parent;
                while !tmp.is_null() {
                    attr = (*tmp).properties;
                    while !attr.is_null() {
                        if xmlC14NIsXmlAttr(attr) != 0 as std::os::raw::c_int {
                            if xmlListSearch(list,
                                             attr as
                                                 *mut std::os::raw::c_void).is_null()
                               {
                                xmlListInsert(list,
                                              attr as *mut std::os::raw::c_void);
                            }
                        }
                        attr = (*attr).next
                    }
                    tmp = (*tmp).parent
                }
            }
        }
        1 => {
            /* attributes in the XML namespace, such as xml:lang and xml:space
         * are not imported into orphan nodes of the document subset
         */
            /*
         * Add all visible attributes from current node.
         */
            attr = (*cur).properties;
            while !attr.is_null() {
                /* check that attribute is visible */
                if if (*ctx).is_visible_callback.is_some() {
                       (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                      attr
                                                                                          as
                                                                                          xmlNodePtr,
                                                                                      cur)
                   } else { 1 as std::os::raw::c_int } != 0 {
                    xmlListInsert(list, attr as *mut std::os::raw::c_void);
                }
                attr = (*attr).next
            }
        }
        2 => {
            /* The processing of an element node E MUST be modified slightly when an XPath node-set is
         * given as input and some of the element's ancestors are omitted from the node-set.
         *
         * Simple inheritable attributes are attributes that have a value that requires at most a simple
         * redeclaration. This redeclaration is done by supplying a new value in the child axis. The
         * redeclaration of a simple inheritable attribute A contained in one of E's ancestors is done
         * by supplying a value to an attribute Ae inside E with the same name. Simple inheritable attributes
         * are xml:lang and xml:space.
         *
         * The method for processing the attribute axis of an element E in the node-set is hence enhanced.
         * All element nodes along E's ancestor axis are examined for the nearest occurrences of simple
         * inheritable attributes in the xml namespace, such as xml:lang and xml:space (whether or not they
         * are in the node-set). From this list of attributes, any simple inheritable attributes that are
         * already in E's attribute axis (whether or not they are in the node-set) are removed. Then,
         * lexicographically merge this attribute list with the nodes of E's attribute axis that are in
         * the node-set. The result of visiting the attribute axis is computed by processing the attribute
         * nodes in this merged attribute list.
         *
         * The xml:id attribute is not a simple inheritable attribute and no processing of these attributes is
         * performed.
         *
         * The xml:base attribute is not a simple inheritable attribute and requires special processing beyond
         * a simple redeclaration.
         *
         * Attributes in the XML namespace other than xml:base, xml:id, xml:lang, and xml:space MUST be processed
         * as ordinary attributes.
         */
            /*
         * Add all visible attributes from current node.
         */
            attr = (*cur).properties;
            while !attr.is_null() {
                /* special processing for XML attribute kiks in only when we have invisible parents */
                if parent_visible == 0 ||
                       xmlC14NIsXmlAttr(attr) == 0 as std::os::raw::c_int {
                    /* check that attribute is visible */
                    if if (*ctx).is_visible_callback.is_some() {
                           (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                          attr
                                                                                              as
                                                                                              xmlNodePtr,
                                                                                          cur)
                       } else { 1 as std::os::raw::c_int } != 0 {
                        xmlListInsert(list, attr as *mut std::os::raw::c_void);
                    }
                } else {
                    let mut matched: std::os::raw::c_int = 0 as std::os::raw::c_int;
                    /* check for simple inheritance attributes */
                    if matched == 0 && xml_lang_attr.is_null() &&
                           xmlStrEqual((*attr).name,
                                       b"lang\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 {
                        xml_lang_attr = attr;
                        matched = 1 as std::os::raw::c_int
                    }
                    if matched == 0 && xml_space_attr.is_null() &&
                           xmlStrEqual((*attr).name,
                                       b"space\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 {
                        xml_space_attr = attr;
                        matched = 1 as std::os::raw::c_int
                    }
                    /* check for base attr */
                    if matched == 0 && xml_base_attr.is_null() &&
                           xmlStrEqual((*attr).name,
                                       b"base\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 {
                        xml_base_attr = attr;
                        matched = 1 as std::os::raw::c_int
                    }
                    /* otherwise, it is a normal attribute, so just check if it is visible */
                    if matched == 0 &&
                           (if (*ctx).is_visible_callback.is_some() {
                                (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                                               attr
                                                                                                   as
                                                                                                   xmlNodePtr,
                                                                                               cur)
                            } else { 1 as std::os::raw::c_int }) != 0 {
                        xmlListInsert(list, attr as *mut std::os::raw::c_void);
                    }
                }
                /* move to the next one */
                attr = (*attr).next
            }
            /* special processing for XML attribute kiks in only when we have invisible parents */
            if parent_visible != 0 {
                /* simple inheritance attributes - copy */
                if xml_lang_attr.is_null() {
                    xml_lang_attr =
                        xmlC14NFindHiddenParentAttr(ctx, (*cur).parent,
                                                    b"lang\x00" as *const u8
                                                        as *const std::os::raw::c_char
                                                        as *mut xmlChar,
                                                    b"http://www.w3.org/XML/1998/namespace\x00"
                                                        as *const u8 as
                                                        *const std::os::raw::c_char as
                                                        *const xmlChar)
                }
                if !xml_lang_attr.is_null() {
                    xmlListInsert(list, xml_lang_attr as *mut std::os::raw::c_void);
                }
                if xml_space_attr.is_null() {
                    xml_space_attr =
                        xmlC14NFindHiddenParentAttr(ctx, (*cur).parent,
                                                    b"space\x00" as *const u8
                                                        as *const std::os::raw::c_char
                                                        as *mut xmlChar,
                                                    b"http://www.w3.org/XML/1998/namespace\x00"
                                                        as *const u8 as
                                                        *const std::os::raw::c_char as
                                                        *const xmlChar)
                }
                if !xml_space_attr.is_null() {
                    xmlListInsert(list, xml_space_attr as *mut std::os::raw::c_void);
                }
                /* base uri attribute - fix up */
                if xml_base_attr.is_null() {
                    /* if we don't have base uri attribute, check if we have a "hidden" one above */
                    xml_base_attr =
                        xmlC14NFindHiddenParentAttr(ctx, (*cur).parent,
                                                    b"base\x00" as *const u8
                                                        as *const std::os::raw::c_char
                                                        as *mut xmlChar,
                                                    b"http://www.w3.org/XML/1998/namespace\x00"
                                                        as *const u8 as
                                                        *const std::os::raw::c_char as
                                                        *const xmlChar)
                }
                if !xml_base_attr.is_null() {
                    xml_base_attr = xmlC14NFixupBaseAttr(ctx, xml_base_attr);
                    if !xml_base_attr.is_null() {
                        xmlListInsert(list,
                                      xml_base_attr as *mut std::os::raw::c_void);
                        /* note that we MUST delete returned attr node ourselves! */
                        (*xml_base_attr).next = attrs_to_delete;
                        attrs_to_delete = xml_base_attr
                    }
                }
            }
        }
        _ => { }
    }
    /*
     * print out all elements from list
     */
    xmlListWalk(list,
                Some(xmlC14NPrintAttrs as
                         unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void)
                             -> std::os::raw::c_int), ctx as *mut std::os::raw::c_void);
    /*
     * Cleanup
     */
    xmlFreePropList(attrs_to_delete);
    xmlListDelete(list);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlC14NCheckForRelativeNamespaces:
 * @ctx:		the C14N context
 * @cur:		the current element node
 *
 * Checks that current element node has no relative namespaces defined
 *
 * Returns 0 if the node has no relative namespaces or -1 otherwise.
 */
unsafe extern "C" fn xmlC14NCheckForRelativeNamespaces(mut ctx: xmlC14NCtxPtr,
                                                       mut cur: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if ctx.is_null() || cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlC14NErrParam(b"checking for relative namespaces\x00" as *const u8
                            as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    ns = (*cur).nsDef;
    while !ns.is_null() {
        if xmlStrlen((*ns).href) > 0 as std::os::raw::c_int {
            let mut uri: xmlURIPtr = 0 as *mut xmlURI;
            uri = xmlParseURI((*ns).href as *const std::os::raw::c_char);
            if uri.is_null() {
                xmlC14NErrInternal(b"parsing namespace uri\x00" as *const u8
                                       as *const std::os::raw::c_char);
                return -(1 as std::os::raw::c_int)
            }
            if xmlStrlen((*uri).scheme as *const xmlChar) == 0 as std::os::raw::c_int
               {
                xmlC14NErrRelativeNamespace((*uri).scheme);
                xmlFreeURI(uri);
                return -(1 as std::os::raw::c_int)
            }
            xmlFreeURI(uri);
        }
        ns = (*ns).next
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlC14NProcessElementNode:
 * @ctx:		the pointer to C14N context object
 * @cur:		the node to process
 * @visible:    this node is visible
 * @all_parents_visible: whether all the parents of this node are visible
 *
 * Canonical XML v 1.0 (http://www.w3.org/TR/xml-c14n)
 *
 * Element Nodes
 * If the element is not in the node-set, then the result is obtained
 * by processing the namespace axis, then the attribute axis, then
 * processing the child nodes of the element that are in the node-set
 * (in document order). If the element is in the node-set, then the result
 * is an open angle bracket (<), the element QName, the result of
 * processing the namespace axis, the result of processing the attribute
 * axis, a close angle bracket (>), the result of processing the child
 * nodes of the element that are in the node-set (in document order), an
 * open angle bracket, a forward slash (/), the element QName, and a close
 * angle bracket.
 *
 * Returns non-negative value on success or negative value on fail
 */
unsafe extern "C" fn xmlC14NProcessElementNode(mut ctx: xmlC14NCtxPtr,
                                               mut cur: xmlNodePtr,
                                               mut visible: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut state: xmlC14NVisibleNsStack =
        xmlC14NVisibleNsStack{nsCurEnd: 0,
                              nsPrevStart: 0,
                              nsPrevEnd: 0,
                              nsMax: 0,
                              nsTab: 0 as *mut xmlNsPtr,
                              nodeTab: 0 as *mut xmlNodePtr,};
    let mut parent_is_doc: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctx.is_null() || cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlC14NErrParam(b"processing element node\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Check relative relative namespaces:
     * implementations of XML canonicalization MUST report an operation
     * failure on documents containing relative namespace URIs.
     */
    if xmlC14NCheckForRelativeNamespaces(ctx, cur) < 0 as std::os::raw::c_int {
        xmlC14NErrInternal(b"checking for relative namespaces\x00" as
                               *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Save ns_rendered stack position
     */
    memset(&mut state as *mut xmlC14NVisibleNsStack as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlC14NVisibleNsStack>() as std::os::raw::c_ulong);
    xmlC14NVisibleNsStackSave((*ctx).ns_rendered, &mut state);
    if visible != 0 {
        if (*ctx).parent_is_doc != 0 {
            /* save this flag into the stack */
            parent_is_doc = (*ctx).parent_is_doc;
            (*ctx).parent_is_doc = 0 as std::os::raw::c_int;
            (*ctx).pos = XMLC14N_INSIDE_DOCUMENT_ELEMENT
        }
        xmlOutputBufferWriteString((*ctx).buf,
                                   b"<\x00" as *const u8 as
                                       *const std::os::raw::c_char);
        if !(*cur).ns.is_null() &&
               xmlStrlen((*(*cur).ns).prefix) > 0 as std::os::raw::c_int {
            xmlOutputBufferWriteString((*ctx).buf,
                                       (*(*cur).ns).prefix as
                                           *const std::os::raw::c_char);
            xmlOutputBufferWriteString((*ctx).buf,
                                       b":\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        }
        xmlOutputBufferWriteString((*ctx).buf,
                                   (*cur).name as *const std::os::raw::c_char);
    }
    if !((*ctx).mode as std::os::raw::c_uint ==
             XML_C14N_EXCLUSIVE_1_0 as std::os::raw::c_int as std::os::raw::c_uint) {
        ret = xmlC14NProcessNamespacesAxis(ctx, cur, visible)
    } else { ret = xmlExcC14NProcessNamespacesAxis(ctx, cur, visible) }
    if ret < 0 as std::os::raw::c_int {
        xmlC14NErrInternal(b"processing namespaces axis\x00" as *const u8 as
                               *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /* todo: shouldn't this go to "visible only"? */
    if visible != 0 { xmlC14NVisibleNsStackShift((*ctx).ns_rendered); }
    ret = xmlC14NProcessAttrsAxis(ctx, cur, visible);
    if ret < 0 as std::os::raw::c_int {
        xmlC14NErrInternal(b"processing attributes axis\x00" as *const u8 as
                               *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    if visible != 0 {
        xmlOutputBufferWriteString((*ctx).buf,
                                   b">\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    }
    if !(*cur).children.is_null() {
        ret = xmlC14NProcessNodeList(ctx, (*cur).children);
        if ret < 0 as std::os::raw::c_int {
            xmlC14NErrInternal(b"processing childrens list\x00" as *const u8
                                   as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if visible != 0 {
        xmlOutputBufferWriteString((*ctx).buf,
                                   b"</\x00" as *const u8 as
                                       *const std::os::raw::c_char);
        if !(*cur).ns.is_null() &&
               xmlStrlen((*(*cur).ns).prefix) > 0 as std::os::raw::c_int {
            xmlOutputBufferWriteString((*ctx).buf,
                                       (*(*cur).ns).prefix as
                                           *const std::os::raw::c_char);
            xmlOutputBufferWriteString((*ctx).buf,
                                       b":\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        }
        xmlOutputBufferWriteString((*ctx).buf,
                                   (*cur).name as *const std::os::raw::c_char);
        xmlOutputBufferWriteString((*ctx).buf,
                                   b">\x00" as *const u8 as
                                       *const std::os::raw::c_char);
        if parent_is_doc != 0 {
            /* restore this flag from the stack for next node */
            (*ctx).parent_is_doc = parent_is_doc;
            (*ctx).pos = XMLC14N_AFTER_DOCUMENT_ELEMENT
        }
    }
    /*
     * Restore ns_rendered stack position
     */
    xmlC14NVisibleNsStackRestore((*ctx).ns_rendered, &mut state);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlC14NProcessNode:
 * @ctx:		the pointer to C14N context object
 * @cur:		the node to process
 *
 * Processes the given node
 *
 * Returns non-negative value on success or negative value on fail
 */
unsafe extern "C" fn xmlC14NProcessNode(mut ctx: xmlC14NCtxPtr,
                                        mut cur: xmlNodePtr) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut visible: std::os::raw::c_int = 0;
    if ctx.is_null() || cur.is_null() {
        xmlC14NErrParam(b"processing node\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    visible =
        if (*ctx).is_visible_callback.is_some() {
            (*ctx).is_visible_callback.expect("non-null function pointer")((*ctx).user_data,
                                                                           cur,
                                                                           (*cur).parent
                                                                               as
                                                                               xmlNodePtr)
        } else { 1 as std::os::raw::c_int };
    let mut current_block_76: u64;
    match (*cur).type_0 as std::os::raw::c_uint {
        1 => {
            ret = xmlC14NProcessElementNode(ctx, cur, visible);
            current_block_76 = 16108440464692313034;
        }
        4 | 3 => {
            /*
             * Text Nodes
             * the string value, except all ampersands are replaced
             * by &amp;, all open angle brackets (<) are replaced by &lt;, all closing
             * angle brackets (>) are replaced by &gt;, and all #xD characters are
             * replaced by &#xD;.
             */
            /* cdata sections are processed as text nodes */
            /* todo: verify that cdata sections are included in XPath nodes set */
            if visible != 0 && !(*cur).content.is_null() {
                let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
                buffer =
                    xmlC11NNormalizeString((*cur).content,
                                           XMLC14N_NORMALIZE_TEXT);
                if !buffer.is_null() {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               buffer as *const std::os::raw::c_char);
                    xmlFree.expect("non-null function pointer")(buffer as
                                                                    *mut std::os::raw::c_void);
                } else {
                    xmlC14NErrInternal(b"normalizing text node\x00" as
                                           *const u8 as *const std::os::raw::c_char);
                    return -(1 as std::os::raw::c_int)
                }
            }
            current_block_76 = 16108440464692313034;
        }
        7 => {
            /*
             * Processing Instruction (PI) Nodes-
             * The opening PI symbol (<?), the PI target name of the node,
             * a leading space and the string value if it is not empty, and
             * the closing PI symbol (?>). If the string value is empty,
             * then the leading space is not added. Also, a trailing #xA is
             * rendered after the closing PI symbol for PI children of the
             * root node with a lesser document order than the document
             * element, and a leading #xA is rendered before the opening PI
             * symbol of PI children of the root node with a greater document
             * order than the document element.
             */
            if visible != 0 {
                if (*ctx).pos as std::os::raw::c_uint ==
                       XMLC14N_AFTER_DOCUMENT_ELEMENT as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"\n<?\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                } else {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"<?\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                }
                xmlOutputBufferWriteString((*ctx).buf,
                                           (*cur).name as
                                               *const std::os::raw::c_char);
                if !(*cur).content.is_null() &&
                       *(*cur).content as std::os::raw::c_int != '\u{0}' as i32 {
                    let mut buffer_0: *mut xmlChar = 0 as *mut xmlChar;
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b" \x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                    /* todo: do we need to normalize pi? */
                    buffer_0 =
                        xmlC11NNormalizeString((*cur).content,
                                               XMLC14N_NORMALIZE_PI);
                    if !buffer_0.is_null() {
                        xmlOutputBufferWriteString((*ctx).buf,
                                                   buffer_0 as
                                                       *const std::os::raw::c_char);
                        xmlFree.expect("non-null function pointer")(buffer_0
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    } else {
                        xmlC14NErrInternal(b"normalizing pi node\x00" as
                                               *const u8 as
                                               *const std::os::raw::c_char);
                        return -(1 as std::os::raw::c_int)
                    }
                }
                if (*ctx).pos as std::os::raw::c_uint ==
                       XMLC14N_BEFORE_DOCUMENT_ELEMENT as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"?>\n\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                } else {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"?>\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                }
            }
            current_block_76 = 16108440464692313034;
        }
        8 => {
            /*
             * Comment Nodes
             * Nothing if generating canonical XML without  comments. For
             * canonical XML with comments, generate the opening comment
             * symbol (<!--), the string value of the node, and the
             * closing comment symbol (-->). Also, a trailing #xA is rendered
             * after the closing comment symbol for comment children of the
             * root node with a lesser document order than the document
             * element, and a leading #xA is rendered before the opening
             * comment symbol of comment children of the root node with a
             * greater document order than the document element. (Comment
             * children of the root node represent comments outside of the
             * top-level document element and outside of the document type
             * declaration).
             */
            if visible != 0 && (*ctx).with_comments != 0 {
                if (*ctx).pos as std::os::raw::c_uint ==
                       XMLC14N_AFTER_DOCUMENT_ELEMENT as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"\n<!--\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                } else {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"<!--\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                }
                if !(*cur).content.is_null() {
                    let mut buffer_1: *mut xmlChar = 0 as *mut xmlChar;
                    /* todo: do we need to normalize comment? */
                    buffer_1 =
                        xmlC11NNormalizeString((*cur).content,
                                               XMLC14N_NORMALIZE_COMMENT);
                    if !buffer_1.is_null() {
                        xmlOutputBufferWriteString((*ctx).buf,
                                                   buffer_1 as
                                                       *const std::os::raw::c_char);
                        xmlFree.expect("non-null function pointer")(buffer_1
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    } else {
                        xmlC14NErrInternal(b"normalizing comment node\x00" as
                                               *const u8 as
                                               *const std::os::raw::c_char);
                        return -(1 as std::os::raw::c_int)
                    }
                }
                if (*ctx).pos as std::os::raw::c_uint ==
                       XMLC14N_BEFORE_DOCUMENT_ELEMENT as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"-->\n\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                } else {
                    xmlOutputBufferWriteString((*ctx).buf,
                                               b"-->\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                }
            }
            current_block_76 = 16108440464692313034;
        }
        21 => {
            /* should be processed as document? */
            current_block_76 = 14452517466194687152;
        }
        9 | 11 | 13 => { current_block_76 = 14452517466194687152; }
        2 => {
            xmlC14NErrInvalidNode(b"XML_ATTRIBUTE_NODE\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"processing node\x00" as *const u8 as
                                      *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        18 => {
            xmlC14NErrInvalidNode(b"XML_NAMESPACE_DECL\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"processing node\x00" as *const u8 as
                                      *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        5 => {
            xmlC14NErrInvalidNode(b"XML_ENTITY_REF_NODE\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"processing node\x00" as *const u8 as
                                      *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        6 => {
            xmlC14NErrInvalidNode(b"XML_ENTITY_NODE\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"processing node\x00" as *const u8 as
                                      *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        10 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
            current_block_76 = 16108440464692313034;
        }
        _ => {
            xmlC14NErrUnknownNode((*cur).type_0 as std::os::raw::c_int,
                                  b"processing node\x00" as *const u8 as
                                      *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    match current_block_76 {
        14452517466194687152 =>
        /* should be processed as document? */
        /* should be processed as document? */
        {
            if !(*cur).children.is_null() {
                (*ctx).pos = XMLC14N_BEFORE_DOCUMENT_ELEMENT;
                (*ctx).parent_is_doc = 1 as std::os::raw::c_int;
                ret = xmlC14NProcessNodeList(ctx, (*cur).children)
            }
        }
        _ => { }
    }
    return ret;
}
/* *
 * xmlC14NProcessNodeList:
 * @ctx:		the pointer to C14N context object
 * @cur:		the node to start from
 *
 * Processes all nodes in the row starting from cur.
 *
 * Returns non-negative value on success or negative value on fail
 */
unsafe extern "C" fn xmlC14NProcessNodeList(mut ctx: xmlC14NCtxPtr,
                                            mut cur: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if ctx.is_null() {
        xmlC14NErrParam(b"processing node list\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    ret = 0 as std::os::raw::c_int;
    while !cur.is_null() && ret >= 0 as std::os::raw::c_int {
        ret = xmlC14NProcessNode(ctx, cur);
        cur = (*cur).next
    }
    return ret;
}
/* *
 * xmlC14NFreeCtx:
 * @ctx: the pointer to C14N context object
 *
 * Cleanups the C14N context object.
 */
unsafe extern "C" fn xmlC14NFreeCtx(mut ctx: xmlC14NCtxPtr) {
    if ctx.is_null() {
        xmlC14NErrParam(b"freeing context\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return
    }
    if !(*ctx).ns_rendered.is_null() {
        xmlC14NVisibleNsStackDestroy((*ctx).ns_rendered);
    }
    xmlFree.expect("non-null function pointer")(ctx as *mut std::os::raw::c_void);
}
/* *
 * xmlC14NNewCtx:
 * @doc:		the XML document for canonization
 * @is_visible_callback:the function to use to determine is node visible
 *			or not
 * @user_data:		the first parameter for @is_visible_callback function
 *			(in most cases, it is nodes set)
 * @mode:   the c14n mode (see @xmlC14NMode)
 * @inclusive_ns_prefixe the list of inclusive namespace prefixes
 *			ended with a NULL or NULL if there is no
 *			inclusive namespaces (only for `
 *			canonicalization)
 * @with_comments:	include comments in the result (!=0) or not (==0)
 * @buf:		the output buffer to store canonical XML; this
 *			buffer MUST have encoder==NULL because C14N requires
 *			UTF-8 output
 *
 * Creates new C14N context object to store C14N parameters.
 *
 * Returns pointer to newly created object (success) or NULL (fail)
 */
unsafe extern "C" fn xmlC14NNewCtx(mut doc: xmlDocPtr,
                                   mut is_visible_callback:
                                       xmlC14NIsVisibleCallback,
                                   mut user_data: *mut std::os::raw::c_void,
                                   mut mode: xmlC14NMode,
                                   mut inclusive_ns_prefixes:
                                       *mut *mut xmlChar,
                                   mut with_comments: std::os::raw::c_int,
                                   mut buf: xmlOutputBufferPtr)
 -> xmlC14NCtxPtr {
    let mut ctx: xmlC14NCtxPtr = 0 as xmlC14NCtxPtr;
    if doc.is_null() || buf.is_null() {
        xmlC14NErrParam(b"creating new context\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return 0 as xmlC14NCtxPtr
    }
    /*
     *  Validate the encoding output buffer encoding
     */
    if !(*buf).encoder.is_null() {
        xmlC14NErr(ctx, doc as xmlNodePtr,
                   XML_C14N_REQUIRES_UTF8 as std::os::raw::c_int,
                   b"xmlC14NNewCtx: output buffer encoder != NULL but C14N requires UTF8 output\n\x00"
                       as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlC14NCtxPtr
    }
    /*
     *  Validate the XML document encoding value, if provided.
     */
    if (*doc).charset != XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int {
        xmlC14NErr(ctx, doc as xmlNodePtr,
                   XML_C14N_REQUIRES_UTF8 as std::os::raw::c_int,
                   b"xmlC14NNewCtx: source document not in UTF8\n\x00" as
                       *const u8 as *const std::os::raw::c_char);
        return 0 as xmlC14NCtxPtr
    }
    /*
     * Allocate a new xmlC14NCtxPtr and fill the fields.
     */
    ctx =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlC14NCtx>()
                                                          as std::os::raw::c_ulong) as
            xmlC14NCtxPtr;
    if ctx.is_null() {
        xmlC14NErrMemory(b"creating context\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlC14NCtxPtr
    }
    memset(ctx as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlC14NCtx>() as std::os::raw::c_ulong);
    /*
     * initialize C14N context
     */
    (*ctx).doc = doc;
    (*ctx).with_comments = with_comments;
    (*ctx).is_visible_callback = is_visible_callback;
    (*ctx).user_data = user_data;
    (*ctx).buf = buf;
    (*ctx).parent_is_doc = 1 as std::os::raw::c_int;
    (*ctx).pos = XMLC14N_BEFORE_DOCUMENT_ELEMENT;
    (*ctx).ns_rendered = xmlC14NVisibleNsStackCreate();
    if (*ctx).ns_rendered.is_null() {
        xmlC14NErr(ctx, doc as xmlNodePtr,
                   XML_C14N_CREATE_STACK as std::os::raw::c_int,
                   b"xmlC14NNewCtx: xmlC14NVisibleNsStackCreate failed\n\x00"
                       as *const u8 as *const std::os::raw::c_char);
        xmlC14NFreeCtx(ctx);
        return 0 as xmlC14NCtxPtr
    }
    /*
     * Set "mode" flag and remember list of incluseve prefixes
     * for exclusive c14n
     */
    (*ctx).mode = mode;
    if (*ctx).mode as std::os::raw::c_uint ==
           XML_C14N_EXCLUSIVE_1_0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*ctx).inclusive_ns_prefixes = inclusive_ns_prefixes
    }
    return ctx;
}
/* *
 * xmlC14NExecute:
 * @doc:		the XML document for canonization
 * @is_visible_callback:the function to use to determine is node visible
 *			or not
 * @user_data:		the first parameter for @is_visible_callback function
 *			(in most cases, it is nodes set)
 * @mode:	the c14n mode (see @xmlC14NMode)
 * @inclusive_ns_prefixes: the list of inclusive namespace prefixes
 *			ended with a NULL or NULL if there is no
 *			inclusive namespaces (only for exclusive
 *			canonicalization, ignored otherwise)
 * @with_comments:	include comments in the result (!=0) or not (==0)
 * @buf:		the output buffer to store canonical XML; this
 *			buffer MUST have encoder==NULL because C14N requires
 *			UTF-8 output
 *
 * Dumps the canonized image of given XML document into the provided buffer.
 * For details see "Canonical XML" (http://www.w3.org/TR/xml-c14n) or
 * "Exclusive XML Canonicalization" (http://www.w3.org/TR/xml-exc-c14n)
 *
 * Returns non-negative value on success or a negative value on fail
 */
#[no_mangle]
pub unsafe extern "C" fn xmlC14NExecute(mut doc: xmlDocPtr,
                                        mut is_visible_callback:
                                            xmlC14NIsVisibleCallback,
                                        mut user_data: *mut std::os::raw::c_void,
                                        mut mode: std::os::raw::c_int,
                                        mut inclusive_ns_prefixes:
                                            *mut *mut xmlChar,
                                        mut with_comments: std::os::raw::c_int,
                                        mut buf: xmlOutputBufferPtr)
 -> std::os::raw::c_int {
    let mut ctx: xmlC14NCtxPtr = 0 as *mut _xmlC14NCtx;
    let mut c14n_mode: xmlC14NMode = XML_C14N_1_0;
    let mut ret: std::os::raw::c_int = 0;
    if buf.is_null() || doc.is_null() {
        xmlC14NErrParam(b"executing c14n\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /* for backward compatibility, we have to have "mode" as "int"
       and here we check that user gives valid value */
    match mode {
        0 | 1 | 2 => { c14n_mode = mode as xmlC14NMode }
        _ => {
            xmlC14NErrParam(b"invalid mode for executing c14n\x00" as
                                *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    /*
     *  Validate the encoding output buffer encoding
     */
    if !(*buf).encoder.is_null() {
        xmlC14NErr(0 as xmlC14NCtxPtr, doc as xmlNodePtr,
                   XML_C14N_REQUIRES_UTF8 as std::os::raw::c_int,
                   b"xmlC14NExecute: output buffer encoder != NULL but C14N requires UTF8 output\n\x00"
                       as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    ctx =
        xmlC14NNewCtx(doc, is_visible_callback, user_data, c14n_mode,
                      inclusive_ns_prefixes, with_comments, buf);
    if ctx.is_null() {
        xmlC14NErr(0 as xmlC14NCtxPtr, doc as xmlNodePtr,
                   XML_C14N_CREATE_CTXT as std::os::raw::c_int,
                   b"xmlC14NExecute: unable to create C14N context\n\x00" as
                       *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Root Node
     * The root node is the parent of the top-level document element. The
     * result of processing each of its child nodes that is in the node-set
     * in document order. The root node does not generate a byte order mark,
     * XML declaration, nor anything from within the document type
     * declaration.
     */
    if !(*doc).children.is_null() {
        ret = xmlC14NProcessNodeList(ctx, (*doc).children);
        if ret < 0 as std::os::raw::c_int {
            xmlC14NErrInternal(b"processing docs children list\x00" as
                                   *const u8 as *const std::os::raw::c_char);
            xmlC14NFreeCtx(ctx);
            return -(1 as std::os::raw::c_int)
        }
    }
    /*
     * Flush buffer to get number of bytes written
     */
    ret = xmlOutputBufferFlush(buf);
    if ret < 0 as std::os::raw::c_int {
        xmlC14NErrInternal(b"flushing output buffer\x00" as *const u8 as
                               *const std::os::raw::c_char);
        xmlC14NFreeCtx(ctx);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Cleanup
     */
    xmlC14NFreeCtx(ctx);
    return ret;
}
/* *
 * xmlC14NDocSaveTo:
 * @doc:		the XML document for canonization
 * @nodes:		the nodes set to be included in the canonized image
 *		or NULL if all document nodes should be included
 * @mode:		the c14n mode (see @xmlC14NMode)
 * @inclusive_ns_prefixes: the list of inclusive namespace prefixes
 *			ended with a NULL or NULL if there is no
 *			inclusive namespaces (only for exclusive
 *			canonicalization, ignored otherwise)
 * @with_comments:	include comments in the result (!=0) or not (==0)
 * @buf:		the output buffer to store canonical XML; this
 *			buffer MUST have encoder==NULL because C14N requires
 *			UTF-8 output
 *
 * Dumps the canonized image of given XML document into the provided buffer.
 * For details see "Canonical XML" (http://www.w3.org/TR/xml-c14n) or
 * "Exclusive XML Canonicalization" (http://www.w3.org/TR/xml-exc-c14n)
 *
 * Returns non-negative value on success or a negative value on fail
 */
#[no_mangle]
pub unsafe extern "C" fn xmlC14NDocSaveTo(mut doc: xmlDocPtr,
                                          mut nodes: xmlNodeSetPtr,
                                          mut mode: std::os::raw::c_int,
                                          mut inclusive_ns_prefixes:
                                              *mut *mut xmlChar,
                                          mut with_comments: std::os::raw::c_int,
                                          mut buf: xmlOutputBufferPtr)
 -> std::os::raw::c_int {
    return xmlC14NExecute(doc,
                          Some(xmlC14NIsNodeInNodeset as
                                   unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                        _: xmlNodePtr,
                                                        _: xmlNodePtr)
                                       -> std::os::raw::c_int),
                          nodes as *mut std::os::raw::c_void, mode,
                          inclusive_ns_prefixes, with_comments, buf);
}
/* *
 * xmlC14NDocDumpMemory:
 * @doc:		the XML document for canonization
 * @nodes:		the nodes set to be included in the canonized image
 *		or NULL if all document nodes should be included
 * @mode:		the c14n mode (see @xmlC14NMode)
 * @inclusive_ns_prefixes: the list of inclusive namespace prefixes
 *			ended with a NULL or NULL if there is no
 *			inclusive namespaces (only for exclusive
 *			canonicalization, ignored otherwise)
 * @with_comments:	include comments in the result (!=0) or not (==0)
 * @doc_txt_ptr:	the memory pointer for allocated canonical XML text;
 *			the caller of this functions is responsible for calling
 *			xmlFree() to free allocated memory
 *
 * Dumps the canonized image of given XML document into memory.
 * For details see "Canonical XML" (http://www.w3.org/TR/xml-c14n) or
 * "Exclusive XML Canonicalization" (http://www.w3.org/TR/xml-exc-c14n)
 *
 * Returns the number of bytes written on success or a negative value on fail
 */
#[no_mangle]
pub unsafe extern "C" fn xmlC14NDocDumpMemory(mut doc: xmlDocPtr,
                                              mut nodes: xmlNodeSetPtr,
                                              mut mode: std::os::raw::c_int,
                                              mut inclusive_ns_prefixes:
                                                  *mut *mut xmlChar,
                                              mut with_comments: std::os::raw::c_int,
                                              mut doc_txt_ptr:
                                                  *mut *mut xmlChar)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if doc_txt_ptr.is_null() {
        xmlC14NErrParam(b"dumping doc to memory\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    *doc_txt_ptr = 0 as *mut xmlChar;
    /*
     * create memory buffer with UTF8 (default) encoding
     */
    buf = xmlAllocOutputBuffer(0 as xmlCharEncodingHandlerPtr);
    if buf.is_null() {
        xmlC14NErrMemory(b"creating output buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * canonize document and write to buffer
     */
    ret =
        xmlC14NDocSaveTo(doc, nodes, mode, inclusive_ns_prefixes,
                         with_comments, buf);
    if ret < 0 as std::os::raw::c_int {
        xmlC14NErrInternal(b"saving doc to output buffer\x00" as *const u8 as
                               *const std::os::raw::c_char);
        xmlOutputBufferClose(buf);
        return -(1 as std::os::raw::c_int)
    }
    ret = xmlBufUse((*buf).buffer) as std::os::raw::c_int;
    if ret > 0 as std::os::raw::c_int {
        *doc_txt_ptr =
            xmlStrndup(xmlBufContent((*buf).buffer as *const xmlBuf), ret)
    }
    xmlOutputBufferClose(buf);
    if (*doc_txt_ptr).is_null() && ret > 0 as std::os::raw::c_int {
        xmlC14NErrMemory(b"coping canonicanized document\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlC14NDocSave:
 * @doc:		the XML document for canonization
 * @nodes:		the nodes set to be included in the canonized image
 *		or NULL if all document nodes should be included
 * @mode:		the c14n mode (see @xmlC14NMode)
 * @inclusive_ns_prefixes: the list of inclusive namespace prefixes
 *			ended with a NULL or NULL if there is no
 *			inclusive namespaces (only for exclusive
 *			canonicalization, ignored otherwise)
 * @with_comments:	include comments in the result (!=0) or not (==0)
 * @filename:		the filename to store canonical XML image
 * @compression:	the compression level (zlib requred):
 *				-1 - libxml default,
 *				 0 - uncompressed,
 *				>0 - compression level
 *
 * Dumps the canonized image of given XML document into the file.
 * For details see "Canonical XML" (http://www.w3.org/TR/xml-c14n) or
 * "Exclusive XML Canonicalization" (http://www.w3.org/TR/xml-exc-c14n)
 *
 * Returns the number of bytes written success or a negative value on fail
 */
#[no_mangle]
pub unsafe extern "C" fn xmlC14NDocSave(mut doc: xmlDocPtr,
                                        mut nodes: xmlNodeSetPtr,
                                        mut mode: std::os::raw::c_int,
                                        mut inclusive_ns_prefixes:
                                            *mut *mut xmlChar,
                                        mut with_comments: std::os::raw::c_int,
                                        mut filename: *const std::os::raw::c_char,
                                        mut compression: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut ret: std::os::raw::c_int = 0;
    if filename.is_null() {
        xmlC14NErrParam(b"saving doc\x00" as *const u8 as
                            *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    if compression < 0 as std::os::raw::c_int { compression = xmlGetCompressMode() }
    /*
     * save the content to a temp buffer, use default UTF8 encoding.
     */
    buf =
        xmlOutputBufferCreateFilename(filename,
                                      0 as xmlCharEncodingHandlerPtr,
                                      compression);
    if buf.is_null() {
        xmlC14NErrInternal(b"creating temporary filename\x00" as *const u8 as
                               *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * canonize document and write to buffer
     */
    ret =
        xmlC14NDocSaveTo(doc, nodes, mode, inclusive_ns_prefixes,
                         with_comments, buf);
    if ret < 0 as std::os::raw::c_int {
        xmlC14NErrInternal(b"cannicanize document to buffer\x00" as *const u8
                               as *const std::os::raw::c_char);
        xmlOutputBufferClose(buf);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * get the numbers of bytes written
     */
    ret = xmlOutputBufferClose(buf);
    return ret;
}
/*
 * Macro used to grow the current buffer.
 */
/* *
 * xmlC11NNormalizeString:
 * @input:		the input string
 * @mode:		the normalization mode (attribute, comment, PI or text)
 *
 * Converts a string to a canonical (normalized) format. The code is stolen
 * from xmlEncodeEntitiesReentrant(). Added normalization of \x09, \x0a, \x0A
 * and the @mode parameter
 *
 * Returns a normalized string (caller is responsible for calling xmlFree())
 * or NULL if an error occurs
 */
unsafe extern "C" fn xmlC11NNormalizeString(mut input: *const xmlChar,
                                            mut mode:
                                                xmlC14NNormalizationMode)
 -> *mut xmlChar {
    let mut cur: *const xmlChar = input;
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer_size: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if input.is_null() { return 0 as *mut xmlChar }
    /*
     * allocate an translation buffer.
     */
    buffer_size = 1000 as std::os::raw::c_int;
    buffer =
        xmlMallocAtomic.expect("non-null function pointer")((buffer_size as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong))
            as *mut xmlChar;
    if buffer.is_null() {
        xmlC14NErrMemory(b"allocating buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    out = buffer;
    while *cur as std::os::raw::c_int != '\u{0}' as i32 {
        if out.offset_from(buffer) as std::os::raw::c_long >
               (buffer_size - 10 as std::os::raw::c_int) as std::os::raw::c_long {
            let mut indx: std::os::raw::c_int =
                out.offset_from(buffer) as std::os::raw::c_long as
                    std::os::raw::c_int;
            buffer_size *= 2 as std::os::raw::c_int;
            buffer =
                xmlRealloc.expect("non-null function pointer")(buffer as
                                                                   *mut std::os::raw::c_void,
                                                               (buffer_size as
                                                                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                    as
                                                                                                    std::os::raw::c_ulong))
                    as *mut xmlChar;
            if buffer.is_null() {
                xmlC14NErrMemory(b"growing buffer\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return 0 as *mut xmlChar
            }
            out = &mut *buffer.offset(indx as isize) as *mut xmlChar
        }
        if *cur as std::os::raw::c_int == '<' as i32 &&
               (mode as std::os::raw::c_uint ==
                    XMLC14N_NORMALIZE_ATTR as std::os::raw::c_int as std::os::raw::c_uint ||
                    mode as std::os::raw::c_uint ==
                        XMLC14N_NORMALIZE_TEXT as std::os::raw::c_int as std::os::raw::c_uint)
           {
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = '&' as i32 as xmlChar;
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = 'l' as i32 as xmlChar;
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = 't' as i32 as xmlChar;
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = ';' as i32 as xmlChar
        } else if *cur as std::os::raw::c_int == '>' as i32 &&
                      mode as std::os::raw::c_uint ==
                          XMLC14N_NORMALIZE_TEXT as std::os::raw::c_int as
                              std::os::raw::c_uint {
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = '&' as i32 as xmlChar;
            let fresh9 = out;
            out = out.offset(1);
            *fresh9 = 'g' as i32 as xmlChar;
            let fresh10 = out;
            out = out.offset(1);
            *fresh10 = 't' as i32 as xmlChar;
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = ';' as i32 as xmlChar
        } else if *cur as std::os::raw::c_int == '&' as i32 &&
                      (mode as std::os::raw::c_uint ==
                           XMLC14N_NORMALIZE_ATTR as std::os::raw::c_int as
                               std::os::raw::c_uint ||
                           mode as std::os::raw::c_uint ==
                               XMLC14N_NORMALIZE_TEXT as std::os::raw::c_int as
                                   std::os::raw::c_uint) {
            let fresh12 = out;
            out = out.offset(1);
            *fresh12 = '&' as i32 as xmlChar;
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = 'a' as i32 as xmlChar;
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = 'm' as i32 as xmlChar;
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = 'p' as i32 as xmlChar;
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = ';' as i32 as xmlChar
        } else if *cur as std::os::raw::c_int == '\"' as i32 &&
                      mode as std::os::raw::c_uint ==
                          XMLC14N_NORMALIZE_ATTR as std::os::raw::c_int as
                              std::os::raw::c_uint {
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = '&' as i32 as xmlChar;
            let fresh18 = out;
            out = out.offset(1);
            *fresh18 = 'q' as i32 as xmlChar;
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 = 'u' as i32 as xmlChar;
            let fresh20 = out;
            out = out.offset(1);
            *fresh20 = 'o' as i32 as xmlChar;
            let fresh21 = out;
            out = out.offset(1);
            *fresh21 = 't' as i32 as xmlChar;
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = ';' as i32 as xmlChar
        } else if *cur as std::os::raw::c_int == '\t' as i32 &&
                      mode as std::os::raw::c_uint ==
                          XMLC14N_NORMALIZE_ATTR as std::os::raw::c_int as
                              std::os::raw::c_uint {
            let fresh23 = out;
            out = out.offset(1);
            *fresh23 = '&' as i32 as xmlChar;
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = '#' as i32 as xmlChar;
            let fresh25 = out;
            out = out.offset(1);
            *fresh25 = 'x' as i32 as xmlChar;
            let fresh26 = out;
            out = out.offset(1);
            *fresh26 = '9' as i32 as xmlChar;
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = ';' as i32 as xmlChar
        } else if *cur as std::os::raw::c_int == '\n' as i32 &&
                      mode as std::os::raw::c_uint ==
                          XMLC14N_NORMALIZE_ATTR as std::os::raw::c_int as
                              std::os::raw::c_uint {
            let fresh28 = out;
            out = out.offset(1);
            *fresh28 = '&' as i32 as xmlChar;
            let fresh29 = out;
            out = out.offset(1);
            *fresh29 = '#' as i32 as xmlChar;
            let fresh30 = out;
            out = out.offset(1);
            *fresh30 = 'x' as i32 as xmlChar;
            let fresh31 = out;
            out = out.offset(1);
            *fresh31 = 'A' as i32 as xmlChar;
            let fresh32 = out;
            out = out.offset(1);
            *fresh32 = ';' as i32 as xmlChar
        } else if *cur as std::os::raw::c_int == '\r' as i32 &&
                      (mode as std::os::raw::c_uint ==
                           XMLC14N_NORMALIZE_ATTR as std::os::raw::c_int as
                               std::os::raw::c_uint ||
                           mode as std::os::raw::c_uint ==
                               XMLC14N_NORMALIZE_TEXT as std::os::raw::c_int as
                                   std::os::raw::c_uint ||
                           mode as std::os::raw::c_uint ==
                               XMLC14N_NORMALIZE_COMMENT as std::os::raw::c_int as
                                   std::os::raw::c_uint ||
                           mode as std::os::raw::c_uint ==
                               XMLC14N_NORMALIZE_PI as std::os::raw::c_int as
                                   std::os::raw::c_uint) {
            let fresh33 = out;
            out = out.offset(1);
            *fresh33 = '&' as i32 as xmlChar;
            let fresh34 = out;
            out = out.offset(1);
            *fresh34 = '#' as i32 as xmlChar;
            let fresh35 = out;
            out = out.offset(1);
            *fresh35 = 'x' as i32 as xmlChar;
            let fresh36 = out;
            out = out.offset(1);
            *fresh36 = 'D' as i32 as xmlChar;
            let fresh37 = out;
            out = out.offset(1);
            *fresh37 = ';' as i32 as xmlChar
        } else {
            /*
             * Works because on UTF-8, all extended sequences cannot
             * result in bytes in the ASCII range.
             */
            let fresh38 = out;
            out = out.offset(1);
            *fresh38 = *cur
        }
        cur = cur.offset(1)
    }
    *out = 0 as std::os::raw::c_int as xmlChar;
    return buffer;
}
/* LIBXML_C14N_ENABLED */
/* __INCLUDE_ELFGCCHACK */
/* LIBXML_OUTPUT_ENABLED */
