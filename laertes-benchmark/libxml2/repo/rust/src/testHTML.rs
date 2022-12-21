
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
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut std::os::raw::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlCopyDoc(doc: xmlDocPtr, recursive: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn testHTML_fatalErrorDebug(ctx: *mut std::os::raw::c_void,
                                msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn testHTML_warningDebug(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                             _: ...);
    #[no_mangle]
    fn testHTML_errorDebug(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                           _: ...);
    #[no_mangle]
    fn htmlSAXParseFile(filename: *const std::os::raw::c_char,
                        encoding_0: *const std::os::raw::c_char,
                        sax_0: htmlSAXHandlerPtr, userData: *mut std::os::raw::c_void)
     -> htmlDocPtr;
    #[no_mangle]
    fn htmlEncodeEntities(out: *mut std::os::raw::c_uchar, outlen: *mut std::os::raw::c_int,
                          in_0: *const std::os::raw::c_uchar, inlen: *mut std::os::raw::c_int,
                          quoteChar: std::os::raw::c_int) -> std::os::raw::c_int;
    /* *
 * Interfaces for the Push mode.
 */
    #[no_mangle]
    fn htmlCreatePushParserCtxt(sax_0: htmlSAXHandlerPtr,
                                user_data: *mut std::os::raw::c_void,
                                chunk: *const std::os::raw::c_char, size: std::os::raw::c_int,
                                filename: *const std::os::raw::c_char,
                                enc: xmlCharEncoding) -> htmlParserCtxtPtr;
    #[no_mangle]
    fn htmlParseChunk(ctxt: htmlParserCtxtPtr, chunk: *const std::os::raw::c_char,
                      size: std::os::raw::c_int, terminate: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /* LIBXML_PUSH_ENABLED */
    #[no_mangle]
    fn htmlFreeParserCtxt(ctxt: htmlParserCtxtPtr);
    #[no_mangle]
    fn htmlReadFile(URL: *const std::os::raw::c_char, encoding_0: *const std::os::raw::c_char,
                    options_0: std::os::raw::c_int) -> htmlDocPtr;
    #[no_mangle]
    fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn htmlSaveFileEnc(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                       encoding_0: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlDebugDumpDocument(output: *mut FILE, doc: xmlDocPtr);
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
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
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
/* *
 * xmlDoc:
 *
 * An XML document.
 */
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
/* Hash table for param entities if any */
/* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
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
/* *
 * xmlError:
 *
 * An XML Error instance.
 */
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
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
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
/* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
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
pub type getEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
pub type resolveEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlParserInputPtr>;
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
/* parser.h */
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
/* *
 * xmlGenericErrorFunc:
 * @ctx:  a parsing context
 * @msg:  the message
 * @...:  the extra arguments of the varags to format the message
 *
 * Signature of the function to use when there is an error and
 * no parsing or validity context available .
 */
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
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
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type htmlSAXHandlerPtr = xmlSAXHandlerPtr;
pub type htmlDocPtr = xmlDocPtr;
/*
 * testHTML.c : a small tester program for HTML input.
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut copy: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut sax: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut repeat: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noout: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut push: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* LIBXML_PUSH_ENABLED */
static mut encoding: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
static mut options: std::os::raw::c_int = 0 as std::os::raw::c_int;
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
    fprintf(stdout,
            b"SAX.hasExternalSubset()\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
/* *
 * hasInternalSubsetDebug:
 * @ctxt:  An XML parser context
 *
 * Does this document has an internal subset
 */
unsafe extern "C" fn internalSubsetDebug(mut ctx: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar,
                                         mut ExternalID: *const xmlChar,
                                         mut SystemID: *const xmlChar) {
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
    fprintf(stdout,
            b"SAX.attributeDecl(%s, %s, %d, %d, %s, ...)\n\x00" as *const u8
                as *const std::os::raw::c_char, elem, name, type_0, def,
            defaultValue);
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
    fprintf(stdout,
            b"SAX.startElement(%s\x00" as *const u8 as *const std::os::raw::c_char,
            name as *mut std::os::raw::c_char);
    if !atts.is_null() {
        i = 0 as std::os::raw::c_int;
        while !(*atts.offset(i as isize)).is_null() {
            let fresh0 = i;
            i = i + 1;
            fprintf(stdout, b", %s\x00" as *const u8 as *const std::os::raw::c_char,
                    *atts.offset(fresh0 as isize));
            if !(*atts.offset(i as isize)).is_null() {
                let mut output: [std::os::raw::c_uchar; 40] = [0; 40];
                let mut att: *const std::os::raw::c_uchar = *atts.offset(i as isize);
                let mut outlen: std::os::raw::c_int = 0;
                let mut attlen: std::os::raw::c_int = 0;
                fprintf(stdout,
                        b"=\'\x00" as *const u8 as *const std::os::raw::c_char);
                loop  {
                    attlen = strlen(att as *mut std::os::raw::c_char) as std::os::raw::c_int;
                    if !(attlen > 0 as std::os::raw::c_int) { break ; }
                    outlen =
                        (::std::mem::size_of::<[std::os::raw::c_uchar; 40]>() as
                             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                            std::os::raw::c_int;
                    htmlEncodeEntities(output.as_mut_ptr(), &mut outlen, att,
                                       &mut attlen, '\'' as i32);
                    output[outlen as usize] =
                        0 as std::os::raw::c_int as std::os::raw::c_uchar;
                    fprintf(stdout,
                            b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                            output.as_mut_ptr() as *mut std::os::raw::c_char);
                    att = att.offset(attlen as isize)
                }
                fprintf(stdout,
                        b"\'\x00" as *const u8 as *const std::os::raw::c_char);
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
    let mut output: [std::os::raw::c_uchar; 40] = [0; 40];
    let mut inlen: std::os::raw::c_int = len;
    let mut outlen: std::os::raw::c_int = 30 as std::os::raw::c_int;
    htmlEncodeEntities(output.as_mut_ptr(), &mut outlen, ch, &mut inlen,
                       0 as std::os::raw::c_int);
    output[outlen as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    fprintf(stdout,
            b"SAX.characters(%s, %d)\n\x00" as *const u8 as
                *const std::os::raw::c_char, output.as_mut_ptr(), len);
}
/* *
 * cdataDebug:
 * @ctxt:  An XML parser context
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * receiving some cdata chars from the parser.
 * Question: how much at a time ???
 */
unsafe extern "C" fn cdataDebug(mut ctx: *mut std::os::raw::c_void,
                                mut ch: *const xmlChar,
                                mut len: std::os::raw::c_int) {
    let mut output: [std::os::raw::c_uchar; 40] = [0; 40];
    let mut inlen: std::os::raw::c_int = len;
    let mut outlen: std::os::raw::c_int = 30 as std::os::raw::c_int;
    htmlEncodeEntities(output.as_mut_ptr(), &mut outlen, ch, &mut inlen,
                       0 as std::os::raw::c_int);
    output[outlen as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    fprintf(stdout,
            b"SAX.cdata(%s, %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
            output.as_mut_ptr(), len);
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
    fprintf(stdout,
            b"SAX.processingInstruction(%s, %s)\n\x00" as *const u8 as
                *const std::os::raw::c_char, target as *mut std::os::raw::c_char,
            data as *mut std::os::raw::c_char);
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
                                   Some(testHTML_warningDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               error:
                                   Some(testHTML_errorDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const std::os::raw::c_char,
                                                                 _: ...)
                                                -> ()),
                               fatalError:
                                   Some(testHTML_fatalErrorDebug as
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
                                   Some(cdataDebug as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void,
                                                                 _:
                                                                     *const xmlChar,
                                                                 _:
                                                                     std::os::raw::c_int)
                                                -> ()),
                               externalSubset: None,
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
/* ***********************************************************************
 *									*
 *				Debug					*
 *									*
 ************************************************************************/
unsafe extern "C" fn parseSAXFile(mut filename: *mut std::os::raw::c_char) {
    let mut doc: htmlDocPtr = 0 as htmlDocPtr;
    /*
     * Empty callbacks for checking
     */
    if push != 0 {
        let mut f: *mut FILE = 0 as *mut FILE;
        f = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
        if !f.is_null() {
            let mut res: std::os::raw::c_int = 0;
            let mut size: std::os::raw::c_int = 3 as std::os::raw::c_int;
            let mut chars: [std::os::raw::c_char; 4096] = [0; 4096];
            let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            /* if (repeat) */
            size = 4096 as std::os::raw::c_int;
            res =
                fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                      1 as std::os::raw::c_int as size_t, 4 as std::os::raw::c_int as size_t,
                      f) as std::os::raw::c_int;
            if res > 0 as std::os::raw::c_int {
                ctxt =
                    htmlCreatePushParserCtxt(emptySAXHandler,
                                             0 as *mut std::os::raw::c_void,
                                             chars.as_mut_ptr(), res,
                                             filename,
                                             XML_CHAR_ENCODING_NONE);
                loop  {
                    res =
                        fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                              1 as std::os::raw::c_int as size_t, size as size_t, f)
                            as std::os::raw::c_int;
                    if !(res > 0 as std::os::raw::c_int) { break ; }
                    htmlParseChunk(ctxt, chars.as_mut_ptr(), res,
                                   0 as std::os::raw::c_int);
                }
                htmlParseChunk(ctxt, chars.as_mut_ptr(), 0 as std::os::raw::c_int,
                               1 as std::os::raw::c_int);
                doc = (*ctxt).myDoc;
                htmlFreeParserCtxt(ctxt);
            }
            if !doc.is_null() {
                fprintf(stdout,
                        b"htmlSAXParseFile returned non-NULL\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
                xmlFreeDoc(doc);
            }
            fclose(f);
        }
        if noout == 0 {
            f = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
            if !f.is_null() {
                let mut res_0: std::os::raw::c_int = 0;
                let mut size_0: std::os::raw::c_int = 3 as std::os::raw::c_int;
                let mut chars_0: [std::os::raw::c_char; 4096] = [0; 4096];
                let mut ctxt_0: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
                /* if (repeat) */
                size_0 = 4096 as std::os::raw::c_int;
                res_0 =
                    fread(chars_0.as_mut_ptr() as *mut std::os::raw::c_void,
                          1 as std::os::raw::c_int as size_t,
                          4 as std::os::raw::c_int as size_t, f) as std::os::raw::c_int;
                if res_0 > 0 as std::os::raw::c_int {
                    ctxt_0 =
                        htmlCreatePushParserCtxt(debugSAXHandler,
                                                 0 as *mut std::os::raw::c_void,
                                                 chars_0.as_mut_ptr(), res_0,
                                                 filename,
                                                 XML_CHAR_ENCODING_NONE);
                    loop  {
                        res_0 =
                            fread(chars_0.as_mut_ptr() as *mut std::os::raw::c_void,
                                  1 as std::os::raw::c_int as size_t,
                                  size_0 as size_t, f) as std::os::raw::c_int;
                        if !(res_0 > 0 as std::os::raw::c_int) { break ; }
                        htmlParseChunk(ctxt_0, chars_0.as_mut_ptr(), res_0,
                                       0 as std::os::raw::c_int);
                    }
                    htmlParseChunk(ctxt_0, chars_0.as_mut_ptr(),
                                   0 as std::os::raw::c_int, 1 as std::os::raw::c_int);
                    doc = (*ctxt_0).myDoc;
                    htmlFreeParserCtxt(ctxt_0);
                }
                if !doc.is_null() {
                    fprintf(stdout,
                            b"htmlSAXParseFile returned non-NULL\n\x00" as
                                *const u8 as *const std::os::raw::c_char);
                    xmlFreeDoc(doc);
                }
                fclose(f);
            }
        }
    } else {
        /* LIBXML_PUSH_ENABLED */
        doc =
            htmlSAXParseFile(filename, 0 as *const std::os::raw::c_char,
                             emptySAXHandler, 0 as *mut std::os::raw::c_void);
        if !doc.is_null() {
            fprintf(stdout,
                    b"htmlSAXParseFile returned non-NULL\n\x00" as *const u8
                        as *const std::os::raw::c_char);
            xmlFreeDoc(doc);
        }
        if noout == 0 {
            /*
	     * Debug callback
	     */
            doc =
                htmlSAXParseFile(filename, 0 as *const std::os::raw::c_char,
                                 debugSAXHandler, 0 as *mut std::os::raw::c_void);
            if !doc.is_null() {
                fprintf(stdout,
                        b"htmlSAXParseFile returned non-NULL\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
                xmlFreeDoc(doc);
            }
        }
    };
    /* LIBXML_PUSH_ENABLED */
}
unsafe extern "C" fn parseAndPrintFile(mut filename: *mut std::os::raw::c_char) {
    let mut doc: htmlDocPtr = 0 as htmlDocPtr;
    /*
     * build an HTML tree from a string;
     */
    if push != 0 {
        let mut f: *mut FILE = 0 as *mut FILE;
        f = fopen(filename, b"r\x00" as *const u8 as *const std::os::raw::c_char);
        if !f.is_null() {
            let mut res: std::os::raw::c_int = 0;
            let mut size: std::os::raw::c_int = 3 as std::os::raw::c_int;
            let mut chars: [std::os::raw::c_char; 4096] = [0; 4096];
            let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            /* if (repeat) */
            size = 4096 as std::os::raw::c_int;
            res =
                fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                      1 as std::os::raw::c_int as size_t, 4 as std::os::raw::c_int as size_t,
                      f) as std::os::raw::c_int;
            if res > 0 as std::os::raw::c_int {
                ctxt =
                    htmlCreatePushParserCtxt(0 as htmlSAXHandlerPtr,
                                             0 as *mut std::os::raw::c_void,
                                             chars.as_mut_ptr(), res,
                                             filename,
                                             XML_CHAR_ENCODING_NONE);
                loop  {
                    res =
                        fread(chars.as_mut_ptr() as *mut std::os::raw::c_void,
                              1 as std::os::raw::c_int as size_t, size as size_t, f)
                            as std::os::raw::c_int;
                    if !(res > 0 as std::os::raw::c_int) { break ; }
                    htmlParseChunk(ctxt, chars.as_mut_ptr(), res,
                                   0 as std::os::raw::c_int);
                }
                htmlParseChunk(ctxt, chars.as_mut_ptr(), 0 as std::os::raw::c_int,
                               1 as std::os::raw::c_int);
                doc = (*ctxt).myDoc;
                htmlFreeParserCtxt(ctxt);
            }
            fclose(f);
        }
    } else { doc = htmlReadFile(filename, 0 as *const std::os::raw::c_char, options) }
    if doc.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Could not parse %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   filename);
    }
    /*
     * test intermediate copy if needed.
     */
    if copy != 0 {
        let mut tmp: htmlDocPtr = 0 as *mut xmlDoc;
        tmp = doc;
        doc = xmlCopyDoc(doc, 1 as std::os::raw::c_int);
        xmlFreeDoc(tmp);
    }
    /*
     * print it.
     */
    if noout == 0 {
        if debug == 0 {
            if !encoding.is_null() {
                htmlSaveFileEnc(b"-\x00" as *const u8 as *const std::os::raw::c_char,
                                doc, encoding);
            } else { htmlDocDump(stdout, doc); }
        } else { xmlDebugDumpDocument(stdout, doc); }
    }
    /* LIBXML_OUTPUT_ENABLED */
    /*
     * free it.
     */
    xmlFreeDoc(doc);
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0;
    let mut files: std::os::raw::c_int = 0 as std::os::raw::c_int;
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
                         b"-push\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--push\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            push += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-sax\x00" as *const u8 as *const std::os::raw::c_char) == 0
                      ||
                      strcmp(*argv.offset(i as isize),
                             b"--sax\x00" as *const u8 as *const std::os::raw::c_char)
                          == 0 {
            sax += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-noout\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--noout\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            noout += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-repeat\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--repeat\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            repeat += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-encode\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--encode\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            i += 1;
            encoding = *argv.offset(i as isize)
        }
        i += 1
    }
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-encode\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--encode\x00" as *const u8 as *const std::os::raw::c_char) ==
                   0 {
            i += 1
        } else if *(*argv.offset(i as
                                     isize)).offset(0 as std::os::raw::c_int as isize)
                      as std::os::raw::c_int != '-' as i32 {
            if repeat != 0 {
                count = 0 as std::os::raw::c_int;
                while count < 100 as std::os::raw::c_int * repeat {
                    if sax != 0 {
                        parseSAXFile(*argv.offset(i as isize));
                    } else { parseAndPrintFile(*argv.offset(i as isize)); }
                    count += 1
                }
            } else if sax != 0 {
                parseSAXFile(*argv.offset(i as isize));
            } else { parseAndPrintFile(*argv.offset(i as isize)); }
            files += 1
        }
        i += 1
    }
    if files == 0 as std::os::raw::c_int {
        printf(b"Usage : %s [--debug] [--copy] [--copy] HTMLfiles ...\n\x00"
                   as *const u8 as *const std::os::raw::c_char,
               *argv.offset(0 as std::os::raw::c_int as isize));
        printf(b"\tParse the HTML files and output the result of the parsing\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        printf(b"\t--debug : dump a debug tree of the in-memory document\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        printf(b"\t--copy : used to test the internal copy implementation\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        printf(b"\t--sax : debug the sequence of SAX callbacks\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        printf(b"\t--repeat : parse the file 100 times, for timing\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        printf(b"\t--noout : do not print the result\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
        printf(b"\t--push : use the push mode parser\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
        /* LIBXML_PUSH_ENABLED */
        /* LIBXML_PUSH_ENABLED */
        printf(b"\t--encode encoding : output in the given encoding\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
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
/* !LIBXML_HTML_ENABLED */
