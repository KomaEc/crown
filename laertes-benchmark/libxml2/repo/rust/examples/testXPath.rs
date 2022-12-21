#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    pub type _xmlXPathCompExpr;
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
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlDefaultSAXHandlerInit();
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn __xmlDoValidityCheckingDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlXPathFreeCompExpr(comp: xmlXPathCompExprPtr);
    #[no_mangle]
    fn xmlXPathCompiledEval(comp: xmlXPathCompExprPtr,
                            ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathCompile(str: *const xmlChar) -> xmlXPathCompExprPtr;
    #[no_mangle]
    fn xmlXPathEvalExpression(str: *const xmlChar, ctxt: xmlXPathContextPtr)
     -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlReadDoc(cur: *const xmlChar, URL: *const std::os::raw::c_char,
                  encoding: *const std::os::raw::c_char, options: std::os::raw::c_int)
     -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    #[no_mangle]
    fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
    #[no_mangle]
    fn __xmlLoadExtDtdDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlSubstituteEntitiesDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn xmlXPathDebugDumpObject(output: *mut FILE, cur: xmlXPathObjectPtr,
                               depth: std::os::raw::c_int);
    #[no_mangle]
    fn xmlXPathDebugDumpCompExpr(output: *mut FILE, comp: xmlXPathCompExprPtr,
                                 depth: std::os::raw::c_int);
    /*
 * Functions.
 */
    #[no_mangle]
    fn xmlXPtrNewContext(doc: xmlDocPtr, here: xmlNodePtr, origin: xmlNodePtr)
     -> xmlXPathContextPtr;
    #[no_mangle]
    fn xmlXPtrEval(str: *const xmlChar, ctx: xmlXPathContextPtr)
     -> xmlXPathObjectPtr;
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
/* *
 * xmlOutputCloseCallback:
 * @context:  an Output context
 *
 * Callback used in the I/O Output API to close the resource
 *
 * Returns 0 or -1 in case of error
 */
/* LIBXML_OUTPUT_ENABLED */
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
/* *
 * hasInternalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Does this document has an internal subset.
 *
 * Returns 1 if true
 */
pub type hasInternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
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
pub type isStandaloneSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
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
pub type internalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type xmlNsPtr = *mut xmlNs;
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
/*
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
pub type C2RustUnnamed = std::os::raw::c_uint;
/* Store big lines numbers in text PSVI field */
/* ignore internal document encoding hint */
pub const XML_PARSE_BIG_LINES: C2RustUnnamed = 4194304;
/* parse using SAX2 interface before 2.7.0 */
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed = 2097152;
/* relax any hardcoded limit from the parser */
pub const XML_PARSE_OLDSAX: C2RustUnnamed = 1048576;
/* do not fixup XINCLUDE xml:base uris */
pub const XML_PARSE_HUGE: C2RustUnnamed = 524288;
/* parse using XML-1.0 before update 5 */
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed = 262144;
/* compact small text nodes; no modification of
                                   the tree allowed afterwards (will possibly
				   crash if you try to modify the tree) */
pub const XML_PARSE_OLD10: C2RustUnnamed = 131072;
/* do not generate XINCLUDE START/END nodes */
pub const XML_PARSE_COMPACT: C2RustUnnamed = 65536;
/* merge CDATA as text nodes */
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed = 32768;
/* remove redundant namespaces declarations */
pub const XML_PARSE_NOCDATA: C2RustUnnamed = 16384;
/* Do not reuse the context dictionary */
pub const XML_PARSE_NSCLEAN: C2RustUnnamed = 8192;
/* Forbid network access */
pub const XML_PARSE_NODICT: C2RustUnnamed = 4096;
/* Implement XInclude substitition  */
pub const XML_PARSE_NONET: C2RustUnnamed = 2048;
/* use the SAX1 interface internally */
pub const XML_PARSE_XINCLUDE: C2RustUnnamed = 1024;
/* remove blank nodes */
pub const XML_PARSE_SAX1: C2RustUnnamed = 512;
/* pedantic error reporting */
pub const XML_PARSE_NOBLANKS: C2RustUnnamed = 256;
/* suppress warning reports */
pub const XML_PARSE_PEDANTIC: C2RustUnnamed = 128;
/* suppress error reports */
pub const XML_PARSE_NOWARNING: C2RustUnnamed = 64;
/* validate with the DTD */
pub const XML_PARSE_NOERROR: C2RustUnnamed = 32;
/* default DTD attributes */
pub const XML_PARSE_DTDVALID: C2RustUnnamed = 16;
/* load the external subset */
pub const XML_PARSE_DTDATTR: C2RustUnnamed = 8;
/* substitute entities */
pub const XML_PARSE_DTDLOAD: C2RustUnnamed = 4;
/* recover on errors */
pub const XML_PARSE_NOENT: C2RustUnnamed = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: std::os::raw::c_int,
    pub max_variables_unused: std::os::raw::c_int,
    pub varHash: xmlHashTablePtr,
    pub nb_types: std::os::raw::c_int,
    pub max_types: std::os::raw::c_int,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: std::os::raw::c_int,
    pub max_funcs_unused: std::os::raw::c_int,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: std::os::raw::c_int,
    pub max_axis: std::os::raw::c_int,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: std::os::raw::c_int,
    pub user: *mut std::os::raw::c_void,
    pub contextSize: std::os::raw::c_int,
    pub proximityPosition: std::os::raw::c_int,
    pub xptr: std::os::raw::c_int,
    pub here: xmlNodePtr,
    pub origin: xmlNodePtr,
    pub nsHash: xmlHashTablePtr,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut std::os::raw::c_void,
    pub extra: *mut std::os::raw::c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut std::os::raw::c_void,
    pub tmpNsList: *mut xmlNsPtr,
    pub tmpNsNr: std::os::raw::c_int,
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: std::os::raw::c_int,
    pub cache: *mut std::os::raw::c_void,
}
pub type xmlXPathFuncLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathFunction>;
pub type xmlXPathFunction
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: std::os::raw::c_int)
               -> ()>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: std::os::raw::c_int,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: std::os::raw::c_int,
    pub valueMax: std::os::raw::c_int,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: std::os::raw::c_int,
    pub ancestor: xmlNodePtr,
    pub valueFrame: std::os::raw::c_int,
}
pub type xmlXPathCompExprPtr = *mut xmlXPathCompExpr;
pub type xmlXPathCompExpr = _xmlXPathCompExpr;
pub type xmlXPathObjectPtr = *mut xmlXPathObject;
pub type xmlXPathObject = _xmlXPathObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathObject {
    pub type_0: xmlXPathObjectType,
    pub nodesetval: xmlNodeSetPtr,
    pub boolval: std::os::raw::c_int,
    pub floatval: std::os::raw::c_double,
    pub stringval: *mut xmlChar,
    pub user: *mut std::os::raw::c_void,
    pub index: std::os::raw::c_int,
    pub user2: *mut std::os::raw::c_void,
    pub index2: std::os::raw::c_int,
}
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: std::os::raw::c_int,
    pub nodeMax: std::os::raw::c_int,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlXPathObjectType = std::os::raw::c_uint;
pub const XPATH_XSLT_TREE: xmlXPathObjectType = 9;
pub const XPATH_USERS: xmlXPathObjectType = 8;
pub const XPATH_LOCATIONSET: xmlXPathObjectType = 7;
pub const XPATH_RANGE: xmlXPathObjectType = 6;
pub const XPATH_POINT: xmlXPathObjectType = 5;
pub const XPATH_STRING: xmlXPathObjectType = 4;
pub const XPATH_NUMBER: xmlXPathObjectType = 3;
pub const XPATH_BOOLEAN: xmlXPathObjectType = 2;
pub const XPATH_NODESET: xmlXPathObjectType = 1;
pub const XPATH_UNDEFINED: xmlXPathObjectType = 0;
pub type xmlXPathContextPtr = *mut xmlXPathContext;
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathObjectPtr>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
pub type xmlXPathAxisFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr,
                                _: xmlXPathObjectPtr) -> xmlXPathObjectPtr>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
pub type xmlXPathConvertFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathObjectPtr, _: std::os::raw::c_int)
               -> std::os::raw::c_int>;
/*
 * testXPath.c : a small tester program for XPath.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
static mut xptr: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut valid: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut expr: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut tree: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nocdata: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut document: xmlDocPtr = 0 as *const xmlDoc as xmlDocPtr;
/*
 * Default document
 */
static mut buffer: [xmlChar; 447] =
    unsafe {
        *::std::mem::transmute::<&[u8; 447],
                                 &mut [xmlChar; 447]>(b"<?xml version=\"1.0\"?>\n<EXAMPLE prop1=\"gnome is great\" prop2=\"&amp; linux too\">\n  <head>\n   <title>Welcome to Gnome</title>\n  </head>\n  <chapter>\n   <title>The Linux adventure</title>\n   <p>bla bla bla ...</p>\n   <image href=\"linus.gif\"/>\n   <p>...</p>\n  </chapter>\n  <chapter>\n   <title>Chapter 2</title>\n   <p>this is chapter 2 ...</p>\n  </chapter>\n  <chapter>\n   <title>Chapter 3</title>\n   <p>this is chapter 3 ...</p>\n  </chapter>\n</EXAMPLE>\n\x00")
    };
unsafe extern "C" fn testXPath(mut str: *const std::os::raw::c_char) {
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    if xptr != 0 {
        ctxt = xmlXPtrNewContext(document, 0 as xmlNodePtr, 0 as xmlNodePtr);
        res = xmlXPtrEval(str as *mut xmlChar, ctxt)
    } else {
        ctxt = xmlXPathNewContext(document);
        (*ctxt).node = xmlDocGetRootElement(document as *const xmlDoc);
        if expr != 0 {
            res = xmlXPathEvalExpression(str as *mut xmlChar, ctxt)
        } else {
            /* res = xmlXPathEval(BAD_CAST str, ctxt); */
            let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
            comp = xmlXPathCompile(str as *mut xmlChar);
            if !comp.is_null() {
                if tree != 0 {
                    xmlXPathDebugDumpCompExpr(stdout, comp, 0 as std::os::raw::c_int);
                }
                res = xmlXPathCompiledEval(comp, ctxt);
                xmlXPathFreeCompExpr(comp);
            } else { res = 0 as xmlXPathObjectPtr }
        }
    }
    xmlXPathDebugDumpObject(stdout, res, 0 as std::os::raw::c_int);
    xmlXPathFreeObject(res);
    xmlXPathFreeContext(ctxt);
}
unsafe extern "C" fn testXPathFile(mut filename: *const std::os::raw::c_char) {
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
        if len >= 0 as std::os::raw::c_int {
            printf(b"\n========================\nExpression: %s\n\x00" as
                       *const u8 as *const std::os::raw::c_char,
                   expression.as_mut_ptr());
            testXPath(expression.as_mut_ptr());
        }
    }
    fclose(input);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut strings: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut usefile: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut filename: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-xptr\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--xptr\x00" as *const u8 as *const std::os::raw::c_char) == 0
           {
            xptr += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-debug\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--debug\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            debug += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-valid\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--valid\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            valid += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-expr\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--expr\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            expr += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-tree\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--tree\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            tree += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-nocdata\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--nocdata\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            nocdata += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-i\x00" as *const u8 as *const std::os::raw::c_char) == 0
                      ||
                      strcmp(*argv.offset(i as isize),
                             b"--input\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1;
            filename = *argv.offset(i as isize)
        } else if strcmp(*argv.offset(i as isize),
                         b"-f\x00" as *const u8 as *const std::os::raw::c_char) == 0
                      ||
                      strcmp(*argv.offset(i as isize),
                             b"--file\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            usefile += 1
        }
        i += 1
    }
    if valid != 0 as std::os::raw::c_int {
        *__xmlDoValidityCheckingDefaultValue() = 1 as std::os::raw::c_int
    }
    *__xmlLoadExtDtdDefaultValue() |= 2 as std::os::raw::c_int;
    *__xmlLoadExtDtdDefaultValue() |= 4 as std::os::raw::c_int;
    *__xmlSubstituteEntitiesDefaultValue() = 1 as std::os::raw::c_int;
    if nocdata != 0 as std::os::raw::c_int {
        xmlDefaultSAXHandlerInit();
        let ref mut fresh0 = (*__xmlDefaultSAXHandler()).cdataBlock;
        *fresh0 = None
    }
    if document.is_null() {
        if filename.is_null() {
            document =
                xmlReadDoc(buffer.as_mut_ptr(), 0 as *const std::os::raw::c_char,
                           0 as *const std::os::raw::c_char,
                           XML_PARSE_COMPACT as std::os::raw::c_int)
        } else {
            document =
                xmlReadFile(filename, 0 as *const std::os::raw::c_char,
                            XML_PARSE_COMPACT as std::os::raw::c_int)
        }
    }
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-i\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--input\x00" as *const u8 as *const std::os::raw::c_char) == 0
           {
            i += 1
        } else if *(*argv.offset(i as
                                     isize)).offset(0 as std::os::raw::c_int as isize)
                      as std::os::raw::c_int != '-' as i32 {
            if usefile != 0 {
                testXPathFile(*argv.offset(i as isize));
            } else { testXPath(*argv.offset(i as isize)); }
            strings += 1
        }
        i += 1
    }
    if strings == 0 as std::os::raw::c_int {
        printf(b"Usage : %s [--debug] [--copy] stringsorfiles ...\n\x00" as
                   *const u8 as *const std::os::raw::c_char,
               *argv.offset(0 as std::os::raw::c_int as isize));
        printf(b"\tParse the XPath strings and output the result of the parsing\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        printf(b"\t--debug : dump a debug version of the result\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        printf(b"\t--valid : switch on DTD support in the parser\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        printf(b"\t--xptr : expressions are XPointer expressions\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        printf(b"\t--expr : debug XPath expressions only\n\x00" as *const u8
                   as *const std::os::raw::c_char);
        printf(b"\t--tree : show the compiled XPath tree\n\x00" as *const u8
                   as *const std::os::raw::c_char);
        printf(b"\t--nocdata : do not generate CDATA nodes\n\x00" as *const u8
                   as *const std::os::raw::c_char);
        printf(b"\t--input filename : or\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
        printf(b"\t-i filename      : read the document from filename\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        printf(b"\t--file : or\n\x00" as *const u8 as *const std::os::raw::c_char);
        printf(b"\t-f     : read queries from files, args\n\x00" as *const u8
                   as *const std::os::raw::c_char);
    }
    if !document.is_null() { xmlFreeDoc(document); }
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
/* LIBXML_XPATH_ENABLED */
