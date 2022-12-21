#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           ptr_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
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
    fn fread(__ptr: *mut std::os::raw::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
    #[no_mangle]
    fn xmlCleanupParser();
    /* LIBXML_SAX1_ENABLED */
    #[no_mangle]
    fn xmlSubstituteEntitiesDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAXUserParseFile(sax: xmlSAXHandlerPtr,
                           user_data: *mut std::os::raw::c_void,
                           filename: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    /* LIBXML_LEGACY_ENABLED */
    /*
 * Interfaces for the Push mode.
 */
    #[no_mangle]
    fn xmlCreatePushParserCtxt(sax: xmlSAXHandlerPtr,
                               user_data: *mut std::os::raw::c_void,
                               chunk: *const std::os::raw::c_char, size: std::os::raw::c_int,
                               filename: *const std::os::raw::c_char)
     -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlParseChunk(ctxt: xmlParserCtxtPtr, chunk: *const std::os::raw::c_char,
                     size: std::os::raw::c_int, terminate: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    static mut testSAX_quiet: std::os::raw::c_int;
    #[no_mangle]
    static mut testSAX_callbacks: std::os::raw::c_int;
    #[no_mangle]
    fn testSAX_warningDebug(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                            _: ...);
    #[no_mangle]
    fn testSAX_errorDebug(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                          _: ...);
    #[no_mangle]
    fn testSAX_fatalErrorDebug(ctx: *mut std::os::raw::c_void,
                               msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn testSAX_startTimer();
    #[no_mangle]
    fn testSAX_endTimer(format: *const std::os::raw::c_char, _: ...);
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
pub type xmlParserMode = std::os::raw::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
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
/* *
 * xmlAttributeDefault:
 *
 * A DTD Attribute default definition.
 */
/* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
/* next one */
/* Enumeration name */
/* *
 * xmlAttribute:
 *
 * An Attribute declaration in a DTD.
 */
/* application data */
/* XML_ATTRIBUTE_DECL, must be second ! */
/* Attribute name */
/* NULL */
/* NULL */
/* -> DTD */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* next in hash table */
/* The attribute type */
/* the default */
/* or the default value */
/* or the enumeration tree if any */
/* the namespace prefix if any */
/* Element holding the attribute */
/* *
 * xmlElementContentType:
 *
 * Possible definitions of element content types.
 */
/* *
 * xmlElementContentOccur:
 *
 * Possible definitions of element content occurrences.
 */
/* *
 * xmlElementContent:
 *
 * An XML Element content as stored after parsing an element definition
 * in a DTD.
 */
/* PCDATA, ELEMENT, SEQ or OR */
/* ONCE, OPT, MULT or PLUS */
/* Element name */
/* first child */
/* second child */
/* parent */
/* Namespace prefix */
/* *
 * xmlElementTypeVal:
 *
 * The different possibilities for an element content type.
 */
/* *
 * xmlElement:
 *
 * An XML Element declaration from a DTD.
 */
/* application data */
/* XML_ELEMENT_DECL, must be second ! */
/* Element name */
/* NULL */
/* NULL */
/* -> DTD */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* The type */
/* the allowed element content */
/* List of the declared attributes */
/* the namespace prefix if any */
/* the validating regexp */
/* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
/* *
 * xmlNs:
 *
 * An XML namespace.
 * Note that prefix == NULL is valid, it defines the default namespace
 * within the subtree (until overridden).
 *
 * xmlNsType is unified with xmlElementType.
 */
/* next Ns link for this node  */
/* global or local */
/* URL for the namespace */
/* prefix for the namespace */
/* application data */
/* normally an xmlDoc */
/* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
/* application data */
/* XML_DTD_NODE, must be second ! */
/* Name of the DTD */
/* the value of the property link */
/* last child link */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* End of common part */
/* Hash table for notations if any */
/* Hash table for elements if any */
/* Hash table for attributes if any */
/* Hash table for entities if any */
/* External identifier for PUBLIC DTD */
/* URI for a SYSTEM or PUBLIC DTD */
/* Hash table for param entities if any */
/* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
/* application data */
/* XML_ATTRIBUTE_NODE, must be second ! */
/* the name of the property */
/* the value of the property */
/* NULL */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* pointer to the associated namespace */
/* the attribute type if validating */
/* for type/PSVI informations */
/* *
 * xmlID:
 *
 * An XML ID instance.
 */
/* next ID */
/* The ID name */
/* The attribute holding it */
/* The attribute if attr is not available */
/* The line number if attr is not available */
/* The document holding the ID */
/* *
 * xmlRef:
 *
 * An XML IDREF instance.
 */
/* next Ref */
/* The Ref name */
/* The attribute holding it */
/* The attribute if attr is not available */
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
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
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
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
/*
 * xmlValidCtxt:
 * An xmlValidCtxt is used for error reporting when validating.
 */
pub type xmlValidCtxt = _xmlValidCtxt;
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
pub type xmlValidState = _xmlValidState;
/* application data */
/* type number, must be second ! */
/* the name of the node, or the entity */
/* parent->childs link */
/* last child link */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* End of common part */
/* pointer to the associated namespace */
/* the content */
/* properties list */
/* namespace definitions on this node */
/* for type/PSVI informations */
/* line number */
/* extra data for XPath/XSLT */
/* *
 * XML_GET_CONTENT:
 *
 * Macro to extract the content pointer of a node.
 */
/* *
 * XML_GET_LINE:
 *
 * Macro to extract the line number of an element node.
 */
/* *
 * xmlDocProperty
 *
 * Set of properties of the document as found by the parser
 * Some of them are linked to similary named xmlParserOption
 */
/* document is XML well formed */
/* document is Namespace valid */
/* parsed with old XML-1.0 parser */
/* DTD validation was successful */
/* XInclude substitution was done */
/* Document was built using the API
                                           and not by parsing an instance */
/* built for internal processing */
/* parsed or built HTML document */
/* *
 * xmlDoc:
 *
 * An XML document.
 */
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
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
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/*
 * testSAX.c : a small tester program for parsing using the SAX API.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut copy: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut recovery: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut push: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut speed: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noent: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nonull: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut sax2: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut repeat: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut timing: std::os::raw::c_int = 0 as std::os::raw::c_int;
/*
 * Timing routines.
 */
/*
 * Internal timing routines to remove the necessity to have unix-specific
 * function calls
 */
/* !HAVE_GETTIMEOFDAY */
// #if defined(HAVE_GETTIMEOFDAY)
// static struct timeval begin, end;
// /*
//  * startTimer: call where you want to start timing
//  */
// static void
// startTimer(void)
// {
//     gettimeofday(&begin, NULL);
// }
// /*
//  * endTimer: call where you want to stop timing and to print out a
//  *           message about the timing performed; format is a printf
//  *           type argument
//  */
// static void XMLCDECL
// endTimer(const char *fmt, ...)
// {
//     long msec;
//     va_list ap;
//     gettimeofday(&end, NULL);
//     msec = end.tv_sec - begin.tv_sec;
//     msec *= 1000;
//     msec += (end.tv_usec - begin.tv_usec) / 1000;
// #ifndef HAVE_STDARG_H
// #error "endTimer required stdarg functions"
// #endif
//     va_start(ap, fmt);
//     vfprintf(stderr, fmt, ap);
//     va_end(ap);
//     fprintf(stderr, " took %ld ms\n", msec);
// }
// #elif defined(HAVE_TIME_H)
// /*
//  * No gettimeofday function, so we have to make do with calling clock.
//  * This is obviously less accurate, but there's little we can do about
//  * that.
//  */
// #ifndef CLOCKS_PER_SEC
// #define CLOCKS_PER_SEC 100
// #endif
// static clock_t begin, end;
// static void
// startTimer(void)
// {
//     begin = clock();
// }
// static void XMLCDECL
// endTimer(const char *fmt, ...)
// {
//     long msec;
//     va_list ap;
//     end = clock();
//     msec = ((end - begin) * 1000) / CLOCKS_PER_SEC;
// #ifndef HAVE_STDARG_H
// #error "endTimer required stdarg functions"
// #endif
//     va_start(ap, fmt);
//     vfprintf(stderr, fmt, ap);
//     va_end(ap);
//     fprintf(stderr, " took %ld ms\n", msec);
// }
// #else
// /*
//  * We don't have a gettimeofday or time.h, so we just don't do timing
//  */
// static void
// startTimer(void)
// {
//     /*
//      * Do nothing
//      */
// }
// static void XMLCDECL
// endTimer(char *format, ...)
// {
//     /*
//      * We cannot do anything because we don't have a timing function
//      */
// #ifdef HAVE_STDARG_H
//     va_start(ap, format);
//     vfprintf(stderr, format, ap);
//     va_end(ap);
//     fprintf(stderr, " was not timed\n", msec);
// #else
//     /* We don't have gettimeofday, time or stdarg.h, what crazy world is
//      * this ?!
//      */
// #endif
// }
// #endif
/*
 * empty SAX block
 */
static mut emptySAXHandlerStruct: xmlSAXHandler =
    {
        let mut init =
            _xmlSAXHandler{internalSubset: None,
                           isStandalone: None,
                           hasInternalSubset: None,
                           hasExternalSubset: None,
                           resolveEntity: None,
                           getEntity: None,
                           entityDecl: None,
                           notationDecl: None,
                           attributeDecl: None,
                           elementDecl: None,
                           unparsedEntityDecl: None,
                           setDocumentLocator: None,
                           startDocument: None,
                           endDocument: None,
                           startElement: None,
                           endElement: None,
                           reference: None,
                           characters: None,
                           ignorableWhitespace: None,
                           processingInstruction: None,
                           comment: None,
                           warning: None,
                           error: None,
                           fatalError: None,
                           getParameterEntity: None,
                           cdataBlock: None,
                           externalSubset: None,
                           initialized: 1 as std::os::raw::c_int as std::os::raw::c_uint,
                           _private:
                               0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
                           startElementNs: None,
                           endElementNs: None,
                           serror: None,};
        init
    };
static mut emptySAXHandler: xmlSAXHandlerPtr =
    unsafe {
        &emptySAXHandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
    };
/* ***********************************************************************
 *									*
 *				Debug Handlers				*
 *									*
 ************************************************************************/
/* *
 * isStandaloneDebug:
 * @ctxt:  An XML parser context
 *
 * Is this document tagged standalone ?
 *
 * Returns 1 if true
 */
unsafe extern "C" fn isStandaloneDebug(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return 0 as std::os::raw::c_int }
    fprintf(stdout,
            b"SAX.isStandalone()\n\x00" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * hasInternalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an internal subset
 *
 * Returns 1 if true
 */
unsafe extern "C" fn hasInternalSubsetDebug(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return 0 as std::os::raw::c_int }
    fprintf(stdout,
            b"SAX.hasInternalSubset()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * hasExternalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an external subset
 *
 * Returns 1 if true
 */
unsafe extern "C" fn hasExternalSubsetDebug(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return 0 as std::os::raw::c_int }
    fprintf(stdout,
            b"SAX.hasExternalSubset()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * internalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an internal subset
 */
unsafe extern "C" fn internalSubsetDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar,
                                         mut ExternalID: *const xmlChar,
                                         mut SystemID: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.internalSubset(%s,\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s,\x00" as *const u8 as *const std::os::raw::c_char,
                ExternalID);
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s)\n\x00" as *const u8 as *const std::os::raw::c_char,
                SystemID);
    };
}
/* *
 * externalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an external subset
 */
unsafe extern "C" fn externalSubsetDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar,
                                         mut ExternalID: *const xmlChar,
                                         mut SystemID: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.externalSubset(%s,\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s,\x00" as *const u8 as *const std::os::raw::c_char,
                ExternalID);
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b" %s)\n\x00" as *const u8 as *const std::os::raw::c_char,
                SystemID);
    };
}
/* *
 * resolveEntityDebug:
 * @ctxt:  An XML parser context
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * Special entity resolver, better left to the parser, it has
 * more context than the application layer.
 * The default behaviour is to NOT resolve the entities, in that case
 * the ENTITY_REF nodes are built in the structure (and the parameter
 * values).
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
unsafe extern "C" fn resolveEntityDebug(mut ctx: *mut std::os::raw::c_void,
                                        mut publicId: *const xmlChar,
                                        mut systemId: *const xmlChar)
 -> xmlParserInputPtr {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return 0 as xmlParserInputPtr }
    /* xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx; */
    fprintf(stdout,
            b"SAX.resolveEntity(\x00" as *const u8 as *const std::os::raw::c_char);
    if !publicId.is_null() {
        fprintf(stdout, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                publicId as *mut std::os::raw::c_char);
    } else { fprintf(stdout, b" \x00" as *const u8 as *const std::os::raw::c_char); }
    if !systemId.is_null() {
        fprintf(stdout, b", %s)\n\x00" as *const u8 as *const std::os::raw::c_char,
                systemId as *mut std::os::raw::c_char);
    } else {
        fprintf(stdout, b", )\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* ********
    if (systemId != NULL) {
        return(xmlNewInputFromFile(ctxt, (char *) systemId));
    }
 *********/
    return 0 as xmlParserInputPtr;
}
/* *
 * getEntityDebug:
 * @ctxt:  An XML parser context
 * @name: The entity name
 *
 * Get an entity by name
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
unsafe extern "C" fn getEntityDebug(mut ctx: *mut std::os::raw::c_void,
                                    mut name: *const xmlChar)
 -> xmlEntityPtr {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return 0 as xmlEntityPtr }
    fprintf(stdout,
            b"SAX.getEntity(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            name);
    return 0 as xmlEntityPtr;
}
/* *
 * getParameterEntityDebug:
 * @ctxt:  An XML parser context
 * @name: The entity name
 *
 * Get a parameter entity by name
 *
 * Returns the xmlParserInputPtr
 */
unsafe extern "C" fn getParameterEntityDebug(mut ctx: *mut std::os::raw::c_void,
                                             mut name: *const xmlChar)
 -> xmlEntityPtr {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return 0 as xmlEntityPtr }
    fprintf(stdout,
            b"SAX.getParameterEntity(%s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name);
    return 0 as xmlEntityPtr;
}
/* *
 * entityDeclDebug:
 * @ctxt:  An XML parser context
 * @name:  the entity name
 * @type:  the entity type
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @content: the entity value (without processing).
 *
 * An entity definition has been parsed
 */
unsafe extern "C" fn entityDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut name: *const xmlChar,
                                     mut type_0: std::os::raw::c_int,
                                     mut publicId: *const xmlChar,
                                     mut systemId: *const xmlChar,
                                     mut content: *mut xmlChar) {
    let mut nullstr: *const xmlChar =
        b"(null)\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar;
    /* not all libraries handle printing null pointers nicely */
    if publicId.is_null() { publicId = nullstr }
    if systemId.is_null() { systemId = nullstr }
    if content.is_null() { content = nullstr as *mut xmlChar }
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.entityDecl(%s, %d, %s, %s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name, type_0, publicId, systemId,
            content);
}
/* *
 * attributeDeclDebug:
 * @ctxt:  An XML parser context
 * @name:  the attribute name
 * @type:  the attribute type
 *
 * An attribute definition has been parsed
 */
unsafe extern "C" fn attributeDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                        mut elem: *const xmlChar,
                                        mut name: *const xmlChar,
                                        mut type_0: std::os::raw::c_int,
                                        mut def: std::os::raw::c_int,
                                        mut defaultValue: *const xmlChar,
                                        mut tree: xmlEnumerationPtr) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    if defaultValue.is_null() {
        fprintf(stdout,
                b"SAX.attributeDecl(%s, %s, %d, %d, NULL, ...)\n\x00" as
                    *const u8 as *const std::os::raw::c_char, elem, name, type_0,
                def);
    } else {
        fprintf(stdout,
                b"SAX.attributeDecl(%s, %s, %d, %d, %s, ...)\n\x00" as
                    *const u8 as *const std::os::raw::c_char, elem, name, type_0, def,
                defaultValue);
    }
    xmlFreeEnumeration(tree);
}
/* *
 * elementDeclDebug:
 * @ctxt:  An XML parser context
 * @name:  the element name
 * @type:  the element type
 * @content: the element value (without processing).
 *
 * An element definition has been parsed
 */
unsafe extern "C" fn elementDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                      mut name: *const xmlChar,
                                      mut type_0: std::os::raw::c_int,
                                      mut content: xmlElementContentPtr) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.elementDecl(%s, %d, ...)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name, type_0);
}
/* *
 * notationDeclDebug:
 * @ctxt:  An XML parser context
 * @name: The name of the notation
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * What to do when a notation declaration has been parsed.
 */
unsafe extern "C" fn notationDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                       mut name: *const xmlChar,
                                       mut publicId: *const xmlChar,
                                       mut systemId: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.notationDecl(%s, %s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name as *mut std::os::raw::c_char,
            publicId as *mut std::os::raw::c_char, systemId as *mut std::os::raw::c_char);
}
/* *
 * unparsedEntityDeclDebug:
 * @ctxt:  An XML parser context
 * @name: The name of the entity
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @notationName: the name of the notation
 *
 * What to do when an unparsed entity declaration is parsed
 */
unsafe extern "C" fn unparsedEntityDeclDebug(mut ctx: *mut std::os::raw::c_void,
                                             mut name: *const xmlChar,
                                             mut publicId: *const xmlChar,
                                             mut systemId: *const xmlChar,
                                             mut notationName:
                                                 *const xmlChar) {
    let mut nullstr: *const xmlChar =
        b"(null)\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar;
    if publicId.is_null() { publicId = nullstr }
    if systemId.is_null() { systemId = nullstr }
    if notationName.is_null() { notationName = nullstr }
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.unparsedEntityDecl(%s, %s, %s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, name as *mut std::os::raw::c_char,
            publicId as *mut std::os::raw::c_char, systemId as *mut std::os::raw::c_char,
            notationName as *mut std::os::raw::c_char);
}
/* *
 * setDocumentLocatorDebug:
 * @ctxt:  An XML parser context
 * @loc: A SAX Locator
 *
 * Receive the document locator at startup, actually xmlDefaultSAXLocator
 * Everything is available on the context, so this is useless in our case.
 */
unsafe extern "C" fn setDocumentLocatorDebug(mut ctx: *mut std::os::raw::c_void,
                                             mut loc: xmlSAXLocatorPtr) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.setDocumentLocator()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
}
/* *
 * startDocumentDebug:
 * @ctxt:  An XML parser context
 *
 * called when the document start being processed.
 */
unsafe extern "C" fn startDocumentDebug(mut ctx: *mut std::os::raw::c_void) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.startDocument()\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * endDocumentDebug:
 * @ctxt:  An XML parser context
 *
 * called when the document end has been detected.
 */
unsafe extern "C" fn endDocumentDebug(mut ctx: *mut std::os::raw::c_void) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.endDocument()\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * startElementDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when an opening tag has been processed.
 */
unsafe extern "C" fn startElementDebug(mut ctx: *mut std::os::raw::c_void,
                                       mut name: *const xmlChar,
                                       mut atts: *mut *const xmlChar) {
    let mut i: std::os::raw::c_int = 0;
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.startElement(%s\x00" as *const u8 as *const std::os::raw::c_char,
            name as *mut std::os::raw::c_char);
    if !atts.is_null() {
        i = 0 as std::os::raw::c_int;
        while !(*atts.offset(i as isize)).is_null() {
            let fresh0 = i;
            i = i + 1;
            fprintf(stdout,
                    b", %s=\'\x00" as *const u8 as *const std::os::raw::c_char,
                    *atts.offset(fresh0 as isize));
            if !(*atts.offset(i as isize)).is_null() {
                fprintf(stdout,
                        b"%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                        *atts.offset(i as isize));
            }
            i += 1
        }
    }
    fprintf(stdout, b")\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * endElementDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when the end of an element has been detected.
 */
unsafe extern "C" fn endElementDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut name: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.endElement(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            name as *mut std::os::raw::c_char);
}
/* *
 * charactersDebug:
 * @ctxt:  An XML parser context
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * receiving some chars from the parser.
 * Question: how much at a time ???
 */
unsafe extern "C" fn charactersDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut ch: *const xmlChar,
                                     mut len: std::os::raw::c_int) {
    let mut output: [std::os::raw::c_char; 40] = [0; 40];
    let mut i: std::os::raw::c_int = 0;
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    i = 0 as std::os::raw::c_int;
    while i < len && i < 30 as std::os::raw::c_int {
        output[i as usize] = *ch.offset(i as isize) as std::os::raw::c_char;
        i += 1
    }
    output[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    fprintf(stdout,
            b"SAX.characters(%s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, output.as_mut_ptr(), len);
}
/* *
 * referenceDebug:
 * @ctxt:  An XML parser context
 * @name:  The entity name
 *
 * called when an entity reference is detected.
 */
unsafe extern "C" fn referenceDebug(mut ctx: *mut std::os::raw::c_void,
                                    mut name: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.reference(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            name);
}
/* *
 * ignorableWhitespaceDebug:
 * @ctxt:  An XML parser context
 * @ch:  a xmlChar string
 * @start: the first char in the string
 * @len: the number of xmlChar
 *
 * receiving some ignorable whitespaces from the parser.
 * Question: how much at a time ???
 */
unsafe extern "C" fn ignorableWhitespaceDebug(mut ctx: *mut std::os::raw::c_void,
                                              mut ch: *const xmlChar,
                                              mut len: std::os::raw::c_int) {
    let mut output: [std::os::raw::c_char; 40] = [0; 40];
    let mut i: std::os::raw::c_int = 0;
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    i = 0 as std::os::raw::c_int;
    while i < len && i < 30 as std::os::raw::c_int {
        output[i as usize] = *ch.offset(i as isize) as std::os::raw::c_char;
        i += 1
    }
    output[i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    fprintf(stdout,
            b"SAX.ignorableWhitespace(%s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, output.as_mut_ptr(), len);
}
/* *
 * processingInstructionDebug:
 * @ctxt:  An XML parser context
 * @target:  the target name
 * @data: the PI data's
 * @len: the number of xmlChar
 *
 * A processing instruction has been parsed.
 */
unsafe extern "C" fn processingInstructionDebug(mut ctx: *mut std::os::raw::c_void,
                                                mut target: *const xmlChar,
                                                mut data: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    if !data.is_null() {
        fprintf(stdout,
                b"SAX.processingInstruction(%s, %s)\n\x00" as *const u8 as
                    *const std::os::raw::c_char, target as *mut std::os::raw::c_char,
                data as *mut std::os::raw::c_char);
    } else {
        fprintf(stdout,
                b"SAX.processingInstruction(%s, NULL)\n\x00" as *const u8 as
                    *const std::os::raw::c_char, target as *mut std::os::raw::c_char);
    };
}
/* *
 * cdataBlockDebug:
 * @ctx: the user data (XML parser context)
 * @value:  The pcdata content
 * @len:  the block length
 *
 * called when a pcdata block has been parsed
 */
unsafe extern "C" fn cdataBlockDebug(mut ctx: *mut std::os::raw::c_void,
                                     mut value: *const xmlChar,
                                     mut len: std::os::raw::c_int) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.pcdata(%.20s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, value as *mut std::os::raw::c_char, len);
}
/* *
 * commentDebug:
 * @ctxt:  An XML parser context
 * @value:  the comment content
 *
 * A comment has been parsed.
 */
unsafe extern "C" fn commentDebug(mut ctx: *mut std::os::raw::c_void,
                                  mut value: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.comment(%s)\n\x00" as *const u8 as *const std::os::raw::c_char,
            value);
}
/* *
 * warningDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL
// warningDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
// {
//     va_list args;
//     testSAX_callbacks++;
//     if (testSAX_quiet)
// 	return;
//     va_start(args, msg);
//     fprintf(stdout, "SAX.warning: ");
//     vfprintf(stdout, msg, args);
//     va_end(args);
// }
/* *
 * errorDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a error messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL
// errorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
// {
//     va_list args;
//     testSAX_callbacks++;
//     if (testSAX_quiet)
// 	return;
//     va_start(args, msg);
//     fprintf(stdout, "SAX.error: ");
//     vfprintf(stdout, msg, args);
//     va_end(args);
// }
/* *
 * fatalErrorDebug:
 * @ctxt:  An XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a fatalError messages, gives file, line, position and
 * extra parameters.
 */
// static void XMLCDECL
// fatalErrorDebug(void *ctx ATTRIBUTE_UNUSED, const char *msg, ...)
// {
//     va_list args;
//     testSAX_callbacks++;
//     if (testSAX_quiet)
// 	return;
//     va_start(args, msg);
//     fprintf(stdout, "SAX.fatalError: ");
//     vfprintf(stdout, msg, args);
//     va_end(args);
// }
static mut debugSAXHandlerStruct: xmlSAXHandler =
    unsafe {
        {
            let mut init =
                _xmlSAXHandler{internalSubset:
                                   Some(internalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               isStandalone:
                                   Some(isStandaloneDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasInternalSubset:
                                   Some(hasInternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasExternalSubset:
                                   Some(hasExternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               resolveEntity:
                                   Some(resolveEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlParserInputPtr),
                               getEntity:
                                   Some(getEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               entityDecl:
                                   Some(entityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *mut xmlChar)
                                                -> ()),
                               notationDecl:
                                   Some(notationDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               attributeDecl:
                                   Some(attributeDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     xmlEnumerationPtr)
                                                -> ()),
                               elementDecl:
                                   Some(elementDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     xmlElementContentPtr)
                                                -> ()),
                               unparsedEntityDecl:
                                   Some(unparsedEntityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               setDocumentLocator:
                                   Some(setDocumentLocatorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     xmlSAXLocatorPtr)
                                                -> ()),
                               startDocument:
                                   Some(startDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               endDocument:
                                   Some(endDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               startElement:
                                   Some(startElementDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *mut *const xmlChar)
                                                -> ()),
                               endElement:
                                   Some(endElementDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               reference:
                                   Some(referenceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               characters:
                                   Some(charactersDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               ignorableWhitespace:
                                   Some(ignorableWhitespaceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               processingInstruction:
                                   Some(processingInstructionDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               comment:
                                   Some(commentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               warning:
                                   Some(testSAX_warningDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               error:
                                   Some(testSAX_errorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               fatalError:
                                   Some(testSAX_fatalErrorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               getParameterEntity:
                                   Some(getParameterEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               cdataBlock:
                                   Some(cdataBlockDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               externalSubset:
                                   Some(externalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               initialized: 1 as std::os::raw::c_int as std::os::raw::c_uint,
                               _private:
                                   0 as *const std::os::raw::c_void as
                                       *mut std::os::raw::c_void,
                               startElementNs: None,
                               endElementNs: None,
                               serror: None,};
            init
        }
    };
#[no_mangle]
pub static mut debugSAXHandler: xmlSAXHandlerPtr =
    unsafe {
        &debugSAXHandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
    };
/*
 * SAX2 specific testSAX_callbacks
 */
/* *
 * startElementNsDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when an opening tag has been processed.
 */
unsafe extern "C" fn startElementNsDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut localname: *const xmlChar,
                                         mut prefix: *const xmlChar,
                                         mut URI: *const xmlChar,
                                         mut nb_namespaces: std::os::raw::c_int,
                                         mut namespaces: *mut *const xmlChar,
                                         mut nb_attributes: std::os::raw::c_int,
                                         mut nb_defaulted: std::os::raw::c_int,
                                         mut attributes:
                                             *mut *const xmlChar) {
    let mut i: std::os::raw::c_int = 0;
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.startElementNs(%s\x00" as *const u8 as *const std::os::raw::c_char,
            localname as *mut std::os::raw::c_char);
    if prefix.is_null() {
        fprintf(stdout, b", NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b", %s\x00" as *const u8 as *const std::os::raw::c_char,
                prefix as *mut std::os::raw::c_char);
    }
    if URI.is_null() {
        fprintf(stdout, b", NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b", \'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                URI as *mut std::os::raw::c_char);
    }
    fprintf(stdout, b", %d\x00" as *const u8 as *const std::os::raw::c_char,
            nb_namespaces);
    if !namespaces.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nb_namespaces * 2 as std::os::raw::c_int {
            fprintf(stdout,
                    b", xmlns\x00" as *const u8 as *const std::os::raw::c_char);
            if !(*namespaces.offset(i as isize)).is_null() {
                fprintf(stdout,
                        b":%s\x00" as *const u8 as *const std::os::raw::c_char,
                        *namespaces.offset(i as isize));
            }
            i += 1;
            fprintf(stdout,
                    b"=\'%s\'\x00" as *const u8 as *const std::os::raw::c_char,
                    *namespaces.offset(i as isize));
            i += 1
        }
    }
    fprintf(stdout, b", %d, %d\x00" as *const u8 as *const std::os::raw::c_char,
            nb_attributes, nb_defaulted);
    if !attributes.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nb_attributes * 5 as std::os::raw::c_int {
            if !(*attributes.offset((i + 1 as std::os::raw::c_int) as
                                        isize)).is_null() {
                fprintf(stdout,
                        b", %s:%s=\'\x00" as *const u8 as *const std::os::raw::c_char,
                        *attributes.offset((i + 1 as std::os::raw::c_int) as isize),
                        *attributes.offset(i as isize));
            } else {
                fprintf(stdout,
                        b", %s=\'\x00" as *const u8 as *const std::os::raw::c_char,
                        *attributes.offset(i as isize));
            }
            fprintf(stdout,
                    b"%.4s...\', %d\x00" as *const u8 as *const std::os::raw::c_char,
                    *attributes.offset((i + 3 as std::os::raw::c_int) as isize),
                    (*attributes.offset((i + 4 as std::os::raw::c_int) as
                                            isize)).offset_from(*attributes.offset((i
                                                                                                 +
                                                                                                 3
                                                                                                     as
                                                                                                     std::os::raw::c_int)
                                                                                                as
                                                                                                isize))
                        as std::os::raw::c_long as std::os::raw::c_int);
            i += 5 as std::os::raw::c_int
        }
    }
    fprintf(stdout, b")\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * endElementDebug:
 * @ctxt:  An XML parser context
 * @name:  The element name
 *
 * called when the end of an element has been detected.
 */
unsafe extern "C" fn endElementNsDebug(mut ctx: *mut std::os::raw::c_void,
                                       mut localname: *const xmlChar,
                                       mut prefix: *const xmlChar,
                                       mut URI: *const xmlChar) {
    testSAX_callbacks += 1;
    if testSAX_quiet != 0 { return }
    fprintf(stdout,
            b"SAX.endElementNs(%s\x00" as *const u8 as *const std::os::raw::c_char,
            localname as *mut std::os::raw::c_char);
    if prefix.is_null() {
        fprintf(stdout, b", NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout, b", %s\x00" as *const u8 as *const std::os::raw::c_char,
                prefix as *mut std::os::raw::c_char);
    }
    if URI.is_null() {
        fprintf(stdout, b", NULL)\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        fprintf(stdout,
                b", \'%s\')\n\x00" as *const u8 as *const std::os::raw::c_char,
                URI as *mut std::os::raw::c_char);
    };
}
static mut debugSAX2HandlerStruct: xmlSAXHandler =
    unsafe {
        {
            let mut init =
                _xmlSAXHandler{internalSubset:
                                   Some(internalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               isStandalone:
                                   Some(isStandaloneDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasInternalSubset:
                                   Some(hasInternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               hasExternalSubset:
                                   Some(hasExternalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               resolveEntity:
                                   Some(resolveEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlParserInputPtr),
                               getEntity:
                                   Some(getEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               entityDecl:
                                   Some(entityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *mut xmlChar)
                                                -> ()),
                               notationDecl:
                                   Some(notationDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               attributeDecl:
                                   Some(attributeDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     xmlEnumerationPtr)
                                                -> ()),
                               elementDecl:
                                   Some(elementDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     xmlElementContentPtr)
                                                -> ()),
                               unparsedEntityDecl:
                                   Some(unparsedEntityDeclDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               setDocumentLocator:
                                   Some(setDocumentLocatorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     xmlSAXLocatorPtr)
                                                -> ()),
                               startDocument:
                                   Some(startDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               endDocument:
                                   Some(endDocumentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> ()),
                               startElement: None,
                               endElement: None,
                               reference:
                                   Some(referenceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               characters:
                                   Some(charactersDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               ignorableWhitespace:
                                   Some(ignorableWhitespaceDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               processingInstruction:
                                   Some(processingInstructionDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               comment:
                                   Some(commentDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               warning:
                                   Some(testSAX_warningDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               error:
                                   Some(testSAX_errorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               fatalError:
                                   Some(testSAX_fatalErrorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               getParameterEntity:
                                   Some(getParameterEntityDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar)
                                                -> xmlEntityPtr),
                               cdataBlock:
                                   Some(cdataBlockDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               externalSubset:
                                   Some(externalSubsetDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               initialized: 0xdeedbeaf as std::os::raw::c_uint,
                               _private:
                                   0 as *const std::os::raw::c_void as
                                       *mut std::os::raw::c_void,
                               startElementNs:
                                   Some(startElementNsDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *mut *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     std::os::raw::c_int,
                                                                 _:
                                                                     *mut *const xmlChar)
                                                -> ()),
                               endElementNs:
                                   Some(endElementNsDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     *const xmlChar)
                                                -> ()),
                               serror: None,};
            init
        }
    };
static mut debugSAX2Handler: xmlSAXHandlerPtr =
    unsafe {
        &debugSAX2HandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
    };
/* ***********************************************************************
 *									*
 *				Debug					*
 *									*
 ************************************************************************/
unsafe extern "C" fn parseAndPrintFile(mut filename: *mut std::os::raw::c_char) {
    let mut res: std::os::raw::c_int = 0;
    if push != 0 {
        let mut f: *mut FILE = 0 as *mut FILE;
        if testSAX_quiet == 0 && nonull == 0 {
            /*
	     * Empty testSAX_callbacks for checking
	     */
            f = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
            if !f.is_null() {
                let mut ret: std::os::raw::c_int = 0;
                let mut chars: [std::os::raw::c_char; 10] = [0; 10];
                let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
                ret =
                    fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                          1 as std::os::raw::c_int as size_t,
                          4 as std::os::raw::c_int as size_t, f) as std::os::raw::c_int;
                if ret > 0 as std::os::raw::c_int {
                    ctxt =
                        xmlCreatePushParserCtxt(emptySAXHandler,
                                                0 as *mut std::os::raw::c_void,
                                                chars.as_mut_ptr(), ret,
                                                filename);
                    loop  {
                        ret =
                            fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                                  1 as std::os::raw::c_int as size_t,
                                  3 as std::os::raw::c_int as size_t, f) as
                                std::os::raw::c_int;
                        if !(ret > 0 as std::os::raw::c_int) { break ; }
                        xmlParseChunk(ctxt, chars.as_mut_ptr(), ret,
                                      0 as std::os::raw::c_int);
                    }
                    xmlParseChunk(ctxt, chars.as_mut_ptr(), 0 as std::os::raw::c_int,
                                  1 as std::os::raw::c_int);
                    xmlFreeParserCtxt(ctxt);
                }
                fclose(f);
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Cannot read file %s\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           filename);
            }
        }
        /*
	 * Debug callback
	 */
        f = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
        if !f.is_null() {
            let mut ret_0: std::os::raw::c_int = 0;
            let mut chars_0: [std::os::raw::c_char; 10] = [0; 10];
            let mut ctxt_0: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            ret_0 =
                fread(chars_0.as_mut_ptr() as *mut std::os::raw::c_void,
                      1 as std::os::raw::c_int as size_t, 4 as std::os::raw::c_int as size_t,
                      f) as std::os::raw::c_int;
            if ret_0 > 0 as std::os::raw::c_int {
                if sax2 != 0 {
                    ctxt_0 =
                        xmlCreatePushParserCtxt(debugSAX2Handler,
                                                0 as *mut std::os::raw::c_void,
                                                chars_0.as_mut_ptr(), ret_0,
                                                filename)
                } else {
                    ctxt_0 =
                        xmlCreatePushParserCtxt(debugSAXHandler,
                                                0 as *mut std::os::raw::c_void,
                                                chars_0.as_mut_ptr(), ret_0,
                                                filename)
                }
                loop  {
                    ret_0 =
                        fread(chars_0.as_mut_ptr() as *mut std::os::raw::c_void,
                              1 as std::os::raw::c_int as size_t,
                              3 as std::os::raw::c_int as size_t, f) as std::os::raw::c_int;
                    if !(ret_0 > 0 as std::os::raw::c_int) { break ; }
                    xmlParseChunk(ctxt_0, chars_0.as_mut_ptr(), ret_0,
                                  0 as std::os::raw::c_int);
                }
                ret_0 =
                    xmlParseChunk(ctxt_0, chars_0.as_mut_ptr(),
                                  0 as std::os::raw::c_int, 1 as std::os::raw::c_int);
                xmlFreeParserCtxt(ctxt_0);
                if ret_0 != 0 as std::os::raw::c_int {
                    fprintf(stdout,
                            b"xmlSAXUserParseFile returned error %d\n\x00" as
                                *const u8 as *const std::os::raw::c_char, ret_0);
                }
            }
            fclose(f);
        }
    } else if speed == 0 {
        /* LIBXML_PUSH_ENABLED */
        /*
	     * Empty testSAX_callbacks for checking
	     */
        if testSAX_quiet == 0 && nonull == 0 {
            res =
                xmlSAXUserParseFile(emptySAXHandler, 0 as *mut std::os::raw::c_void,
                                    filename);
            if res != 0 as std::os::raw::c_int {
                fprintf(stdout,
                        b"xmlSAXUserParseFile returned error %d\n\x00" as
                            *const u8 as *const std::os::raw::c_char, res);
            }
        }
        /*
	     * Debug callback
	     */
        testSAX_callbacks = 0 as std::os::raw::c_int;
        if repeat != 0 {
            let mut i: std::os::raw::c_int = 0;
            i = 0 as std::os::raw::c_int;
            while i < 99 as std::os::raw::c_int {
                if sax2 != 0 {
                    res =
                        xmlSAXUserParseFile(debugSAX2Handler,
                                            0 as *mut std::os::raw::c_void, filename)
                } else {
                    res =
                        xmlSAXUserParseFile(debugSAXHandler,
                                            0 as *mut std::os::raw::c_void, filename)
                }
                i += 1
            }
        }
        if sax2 != 0 {
            res =
                xmlSAXUserParseFile(debugSAX2Handler, 0 as *mut std::os::raw::c_void,
                                    filename)
        } else {
            res =
                xmlSAXUserParseFile(debugSAXHandler, 0 as *mut std::os::raw::c_void,
                                    filename)
        }
        if res != 0 as std::os::raw::c_int {
            fprintf(stdout,
                    b"xmlSAXUserParseFile returned error %d\n\x00" as
                        *const u8 as *const std::os::raw::c_char, res);
        }
        if testSAX_quiet != 0 {
            fprintf(stdout,
                    b"%d testSAX_callbacks generated\n\x00" as *const u8 as
                        *const std::os::raw::c_char, testSAX_callbacks);
        }
    } else {
        /*
	     * test 100x the SAX parse
	     */
        let mut i_0: std::os::raw::c_int = 0; /* be safe, plus calls xmlInitParser */
        i_0 = 0 as std::os::raw::c_int;
        while i_0 < 100 as std::os::raw::c_int {
            res =
                xmlSAXUserParseFile(emptySAXHandler, 0 as *mut std::os::raw::c_void,
                                    filename);
            i_0 += 1
        }
        if res != 0 as std::os::raw::c_int {
            fprintf(stdout,
                    b"xmlSAXUserParseFile returned error %d\n\x00" as
                        *const u8 as *const std::os::raw::c_char, res);
        }
    };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut files: std::os::raw::c_int = 0 as std::os::raw::c_int;
    xmlCheckVersion(20908 as std::os::raw::c_int);
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-debug\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--debug\x00" as *const u8 as *const std::os::raw::c_char) == 0
           {
            debug += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-copy\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--copy\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            copy += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-recover\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--recover\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            recovery += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-push\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--push\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            push += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-speed\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--speed\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            speed += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-timing\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--timing\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            nonull += 1;
            timing += 1;
            testSAX_quiet += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-repeat\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--repeat\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            repeat += 1;
            testSAX_quiet += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-noent\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--noent\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            noent += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-testSAX_quiet\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--testSAX_quiet\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            testSAX_quiet += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-sax2\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--sax2\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            sax2 += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-nonull\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--nonull\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            nonull += 1
        }
        i += 1
    }
    if noent != 0 as std::os::raw::c_int {
        xmlSubstituteEntitiesDefault(1 as std::os::raw::c_int);
    }
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as std::os::raw::c_int as isize) as
               std::os::raw::c_int != '-' as i32 {
            if timing != 0 { testSAX_startTimer(); }
            parseAndPrintFile(*argv.offset(i as isize));
            if timing != 0 {
                testSAX_endTimer(b"Parsing\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
            files += 1
        }
        i += 1
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
/* LIBXML_PUSH_ENABLED */
/* LIBXML_SAX1_ENABLED */
