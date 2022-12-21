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
    /*
 * Summary: implementation of the Relax-NG validation
 * Description: implementation of the Relax-NG validation
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGValidCtxt;
    /*
    XML_SCHEMA_VAL_XSI_ASSEMBLE			= 1<<1,
	* assemble schemata using
	* xsi:schemaLocation and
	* xsi:noNamespaceSchemaLocation
*/
    /* *
 * The schemas related types are kept internal
 */
    pub type _xmlSchema;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
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
    #[no_mangle]
    fn xmlMemSetup(freeFunc: xmlFreeFunc, mallocFunc: xmlMallocFunc,
                   reallocFunc: xmlReallocFunc, strdupFunc: xmlStrdupFunc)
     -> std::os::raw::c_int;
    /*
 * These are specific to the XML debug memory wrapper.
 */
    #[no_mangle]
    fn xmlMemUsed() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlMemMalloc(size: size_t) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlMemRealloc(ptr: *mut std::os::raw::c_void, size: size_t)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlMemFree(ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlMemoryStrdup(str: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn xmlSetStructuredErrorFunc(ctx: *mut std::os::raw::c_void,
                                 handler: xmlStructuredErrorFunc);
    #[no_mangle]
    fn xmlResetLastError();
    #[no_mangle]
    fn xmlRegisterInputCallbacks(matchFunc: xmlInputMatchCallback,
                                 openFunc: xmlInputOpenCallback,
                                 readFunc: xmlInputReadCallback,
                                 closeFunc: xmlInputCloseCallback)
     -> std::os::raw::c_int;
    /*
 * A predefined entity loader disabling network accesses
 */
    #[no_mangle]
    fn xmlNoNetExternalEntityLoader(URL: *const std::os::raw::c_char,
                                    ID: *const std::os::raw::c_char,
                                    ctxt: xmlParserCtxtPtr)
     -> xmlParserInputPtr;
    /*
 * Init/Cleanup
 */
    #[no_mangle]
    fn xmlInitParser();
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlPedanticParserDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    /*
 * Parser contexts handling.
 */
    #[no_mangle]
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    /*
 * External entities handling actually implemented in xmlIO.
 */
    #[no_mangle]
    fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader);
    #[no_mangle]
    fn xmlCtxtReadFile(ctxt: xmlParserCtxtPtr, filename: *const std::os::raw::c_char,
                       encoding: *const std::os::raw::c_char, options: std::os::raw::c_int)
     -> xmlDocPtr;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn __xmlGetWarningsDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    #[no_mangle]
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlReaderForFile(filename: *const std::os::raw::c_char,
                        encoding: *const std::os::raw::c_char, options: std::os::raw::c_int)
     -> xmlTextReaderPtr;
    #[no_mangle]
    fn channel_testrecurse(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                           _: ...);
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strncpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn __xstat(__ver: std::os::raw::c_int, __filename: *const std::os::raw::c_char,
               __stat_buf: *mut stat) -> std::os::raw::c_int;
    /* parser options for the test */
    #[no_mangle]
    static mut testErrors_recurse: [std::os::raw::c_char; 32769];
    #[no_mangle]
    static mut testErrorsSize_recurse: std::os::raw::c_int;
    #[no_mangle]
    fn glob(__pattern: *const std::os::raw::c_char, __flags: std::os::raw::c_int,
            __errfunc:
                Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
            __pglob: *mut glob_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn globfree(__pglob: *mut glob_t);
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
pub type externalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
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
pub type cdataBlockSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
pub type getParameterEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> xmlEntityPtr>;
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
/* entities.h */
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
/* *
 * BASE_BUFFER_SIZE:
 *
 * default buffer size 4000.
 */
/* *
 * LIBXML_NAMESPACE_DICT:
 *
 * Defines experimental behaviour:
 * 1) xmlNs gets an additional field @context (a xmlDoc)
 * 2) when creating a tree, xmlNs->href is stored in the dict of xmlDoc.
 */
/* #define LIBXML_NAMESPACE_DICT */
/* *
 * xmlBufferAllocationScheme:
 *
 * A buffer allocation scheme can be defined to either match exactly the
 * need or double it's allocated size each time it is found too small.
 */
/* double each time one need to grow */
/* grow only to the minimal size */
/* immutable buffer */
/* special allocation scheme used for I/O */
/* exact up to a threshold, and doubleit thereafter */
/* limit the upper size of the buffer */
/* *
 * xmlBuffer:
 *
 * A buffer structure, this old construct is limited to 2GB and
 * is being deprecated, use API with xmlBuf instead
 */
/* The buffer content UTF8 */
/* The buffer size used */
/* The buffer size */
/* The realloc method */
/* in IO mode we may have a different base */
/* *
 * xmlBuf:
 *
 * A buffer structure, new one, the actual structure internals are not public
 */
/* *
 * xmlBufPtr:
 *
 * A pointer to a buffer structure, the actual structure internals are not
 * public
 */
/*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
/*
 * LIBXML2_NEW_BUFFER:
 *
 * Macro used to express that the API use the new buffers for
 * xmlParserInputBuffer and xmlOutputBuffer. The change was
 * introduced in 2.9.0.
 */
/* *
 * XML_XML_NAMESPACE:
 *
 * This is the namespace for the special xml: prefix predefined in the
 * XML Namespace specification.
 */
/* *
 * XML_XML_ID:
 *
 * This is the name for the special xml:id attribute
 */
/*
 * The different element types carried by an XML tree.
 *
 * NOTE: This is synchronized with DOM Level1 values
 *       See http://www.w3.org/TR/REC-DOM-Level-1/
 *
 * Actually this had diverged a bit, and now XML_DOCUMENT_TYPE_NODE should
 * be deprecated to use an XML_DTD_NODE.
 */
/* *
 * xmlNotation:
 *
 * A DTD Notation definition.
 */
/* Notation name */
/* Public identifier, if any */
/* System identifier, if any */
/* *
 * xmlAttributeType:
 *
 * A DTD Attribute type definition.
 */
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
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
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
pub type C2RustUnnamed = std::os::raw::c_uint;
/* The URI module */
/* The buffers module */
pub const XML_FROM_URI: C2RustUnnamed = 30;
/* The Schematron validator module */
pub const XML_FROM_BUFFER: C2RustUnnamed = 29;
/* The module handling character conversion */
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed = 28;
/* The dynamically loaded module module*/
pub const XML_FROM_I18N: C2RustUnnamed = 27;
/* The xmlwriter module */
pub const XML_FROM_MODULE: C2RustUnnamed = 26;
/* The error checking module */
pub const XML_FROM_WRITER: C2RustUnnamed = 25;
/* The XML DTD validation with valid context */
pub const XML_FROM_CHECK: C2RustUnnamed = 24;
/* The XSLT engine from libxslt */
pub const XML_FROM_VALID: C2RustUnnamed = 23;
/* The Canonicalization module */
pub const XML_FROM_XSLT: C2RustUnnamed = 22;
/* The Catalog module */
pub const XML_FROM_C14N: C2RustUnnamed = 21;
/* The Relax-NG validator module */
pub const XML_FROM_CATALOG: C2RustUnnamed = 20;
/* The Relax-NG parser module */
pub const XML_FROM_RELAXNGV: C2RustUnnamed = 19;
/* The W3C XML Schemas validation module */
pub const XML_FROM_RELAXNGP: C2RustUnnamed = 18;
/* The W3C XML Schemas parser module */
pub const XML_FROM_SCHEMASV: C2RustUnnamed = 17;
/* The W3C XML Schemas Datatype module */
pub const XML_FROM_SCHEMASP: C2RustUnnamed = 16;
/* The regular expressions module */
pub const XML_FROM_DATATYPE: C2RustUnnamed = 15;
/* The XPointer module */
pub const XML_FROM_REGEXP: C2RustUnnamed = 14;
/* The XPath module */
pub const XML_FROM_XPOINTER: C2RustUnnamed = 13;
/* The XInclude processing */
pub const XML_FROM_XPATH: C2RustUnnamed = 12;
/* The HTTP module */
pub const XML_FROM_XINCLUDE: C2RustUnnamed = 11;
/* The FTP module */
pub const XML_FROM_HTTP: C2RustUnnamed = 10;
/* The Input/Output stack */
pub const XML_FROM_FTP: C2RustUnnamed = 9;
/* The serialization code */
pub const XML_FROM_IO: C2RustUnnamed = 8;
/* The memory allocator */
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
/* The HTML parser */
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
/* The XML DTD validation with parser context*/
pub const XML_FROM_HTML: C2RustUnnamed = 5;
/* The XML Namespace module */
pub const XML_FROM_DTD: C2RustUnnamed = 4;
/* The tree module */
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
/* The XML parser */
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
/* 6004 */
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
/* 6003 */
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
/* 6002 */
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
/* 6001 */
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
/* 5037 */
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
/* 5036 */
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
/* 5035 */
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
/* 5034 */
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
/* 5033 */
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
/* 5032 */
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
/* 5031 */
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
/* 5030 */
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
/* 5029 */
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
/* 5028 */
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
/* 5027 */
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
/* 5026 */
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
/* 5025 */
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
/* 5024 */
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
/* 5023 */
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
/* 5022 */
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
/* 5021 */
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
/* 5020 */
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
/* 5019 */
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
/* 5018 */
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
/* 5017 */
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
/* 5016 */
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
/* 5015 */
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
/* 5014 */
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
/* 5013 */
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
/* 5012 */
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
/* 5011 */
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
/* 5010 */
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
/* 5009 */
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
/* 5008 */
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
/* 5007 */
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
/* 5006 */
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
/* 5005 */
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
/* 5004 */
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
/* 5003 */
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
/* 5002 */
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
/* 5001 */
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
/* 4901 */
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
/* 4900 */
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
/* 4000 */
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
/* 3090 */
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
/* 3089 */
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
/* 3088 */
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
/* 3087 */
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
/* 3086 */
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
/* 3085 */
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
/* 3085 */
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
/* 3084 */
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
/* 3083 */
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
/* 3082 */
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
/* 3081 */
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
/* 3080 */
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
/* 3079 */
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
/* 3078 */
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
/* 3077 */
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
/* 3076 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
/* 3075 */
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
/* 3074 */
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
/* 3073 */
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
/* 3072 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
/* 3071 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
/* 3070 non-W3C */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
/* 3069 non-W3C */
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
/* 3068 */
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
/* 3067 */
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
/* 3066 */
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
/* 3065 */
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
/* 3064 */
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
/* 3063 */
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
/* 3062 */
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
/* 3061 */
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
/* 3060 */
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
/* 3059 */
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
/* 3058 */
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
/* 3057 */
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
/* 3056 */
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
/* 3055 */
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
/* 3054 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
/* 3053 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
/* 3052 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
/* 3051 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
/* 3050 */
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
/* 3049 */
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
/* 3048 */
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
/* 3047 */
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
/* 3046 */
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
/* 3045 */
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
/* 3044 */
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
/* 3043 */
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
/* 3042 */
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
/* 3041 */
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
/* 3040 */
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
/* 3039 */
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
/* 3038 */
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
/* 3037 */
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
/* 3036 */
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
/* 3035 */
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
/* 3034 */
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
/* 3033 */
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
/* 3032 */
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
/* 3031 */
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
/* 3030 */
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
/* 3029 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
/* 3028 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
/* 3027 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
/* 3026 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
/* 3025 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
/* 3024 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
/* 3023 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
/* 3022 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
/* 3021 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
/* 3020 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
/* 3019 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
/* 3018 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
/* 3017 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
/* 3016 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
/* 3015 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
/* 3014 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
/* 3013 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
/* 3012 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
/* 3011 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
/* 3010 */
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
/* 3009 */
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
/* 3008 */
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
/* 3007 */
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
/* 3006 */
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 =
    3007;
/* 3005 */
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
/* 3004 */
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 =
    3005;
/* 3003 */
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
/* 3002 */
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
/* 3001 */
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
/* 2022 */
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
/* 2021 */
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
/* 2003 */
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
/* 2002 */
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
/* 2001 */
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
/* 1955 */
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
/* 1954 */
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
/* 1953 */
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
/* 1952 */
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
/* 1951 */
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
/* 1903 */
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
/* 1902 */
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
/* 1901 */
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
/* 1879 */
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
/* 1878 */
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
/* 1877 */
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
/* 1876 */
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
/* 1875 */
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
/* 1874 */
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
/* 1873 */
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
/* 1872 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
/* 1871 */
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
/* 1870 */
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
/* 1869 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
/* 1868 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
/* 1867 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
/* 1866 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
/* 1865 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
/* 1864 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
/* 1863 */
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
/* 1862 */
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
/* 1861 */
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
/* 1860 */
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
/* 1859 */
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
/* 1858 */
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
/* 1857 */
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
/* 1856 */
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
/* 1855 */
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
/* 1854 */
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
/* 1853 */
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
/* 1852 */
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
/* 1851 */
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
/* 1850 */
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
/* 1849 */
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
/* 1848 */
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
/* 1847 */
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
/* 1846 */
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
/* 1845 */
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
/* 1844 */
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
/* 1843 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
/* 1842 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
/* 1841 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
/* 1840 */
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
/* 1839 */
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
/* 1838 */
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
/* 1837 */
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
/* 1836 */
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
/* 1835 */
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
/* 1834 */
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
/* 1833 */
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
/* 1832 */
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
/* 1831 */
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
/* 1830 */
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
/* 1829 */
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
/* 1828 */
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
/* 1827 */
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
/* 1826 */
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
/* 1825 */
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
/* 1824 */
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
/* 1823 */
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
/* 1822 */
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
/* 1821 */
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
/* 1820 */
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
/* 1819 */
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
/* 1818 */
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
/* 1817 */
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
/* 1816 */
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
/* 1815 */
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
/* 1814 */
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
/* 1813 */
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
/* 1812 */
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
/* 1811 */
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
/* 1810 */
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
/* 1809 */
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
/* 1808 */
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
/* 1807 */
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
/* 1806 */
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
/* 1805 */
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
/* 1804 */
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
/* 1803 */
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
/* 1802 */
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
/* 1800 */
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
/* 1799 */
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
/* 1798 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
/* 1797 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
/* 1796 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
/* 1795 */
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
/* 1794 */
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
/* 1793 */
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
/* 1792 */
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
/* 1791 */
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
/* 1790 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
/* 1789 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
/* 1788 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
/* 1787 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
/* 1786 */
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
/* 1785 */
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
/* 1784 */
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
/* 1783 */
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
/* 1782 */
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
/* 1781 */
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
/* 1780 */
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
/* 1779 */
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
/* 1778 */
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
/* 1777 */
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
/* 1776 */
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
/* 1775 */
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
/* 1774 */
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
/* 1773 */
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
/* 1772 */
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
/* 1771 */
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
/* 1770 */
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
/* 1769 */
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
/* 1768 */
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
/* 1767 */
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
/* 1766 */
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
/* 1765 */
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
/* 1764 */
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
/* 1763 */
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
/* 1762 */
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
/* 1761 */
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
/* 1760 */
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
/* 1759 */
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
/* 1758 */
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
/* 1757 */
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
/* 1756 */
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
/* 1755 */
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
/* 1754 */
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
/* 1753 */
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
/* 1752 */
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
/* 1751 */
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
/* 1750 */
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
/* 1749 */
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
/* 1748 */
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
/* 1747 */
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
/* 1746 */
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
/* 1745 */
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
/* 1744 */
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
/* 1743 */
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
/* 1742 */
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
/* 1741 */
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
/* 1740 */
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
/* 1739 */
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
/* 1738 */
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
/* 1737 */
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
/* 1736 */
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
/* 1735 */
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
/* 1734 */
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
/* 1733 */
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
/* 1732 */
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
/* 1731 */
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
/* 1730 */
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
/* 1729 */
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
/* 1728 */
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
/* 1727 */
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
/* 1726 */
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
/* 1725 */
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
/* 1724 */
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
/* 1723 */
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
/* 1722 */
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
/* 1721 */
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
/* 1720 */
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
/* 1719 */
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
/* 1718 */
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
/* 1717 */
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
/* 1716 */
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
/* 1715 */
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
/* 1714 */
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
/* 1713 */
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
/* 1712 */
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
/* 1711 */
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
/* 1710 */
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
/* 1709 */
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
/* 1708 */
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
/* 1707 */
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
/* 1706 */
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
/* 1705 */
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
/* 1704 */
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
/* 1703 */
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
/* 1702 */
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
/* 1701 */
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
/* 1654 */
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
/* 1653 */
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
/* 1652 */
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
/* 1651 */
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
/* 1618 */
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
/* 1617 */
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
/* 1616 */
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
/* 1615 */
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
/* 1614 */
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
/* 1613 */
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
/* 1612 */
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
/* 1611 */
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
/* 1610 */
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
/* 1609 */
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
/* 1608 */
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
/* 1607 */
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
/* 1606 */
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
/* 1605 */
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
/* 1604 */
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
/* 1603 */
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
/* 1602 */
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
/* 1601 */
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
/* 1556 */
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
/* 1555 */
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
/* 1554 */
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
/* 1553 */
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
/* 1552 */
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
/* 1551 */
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
/* 1550 */
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
/* 1549 */
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
/* 1548 */
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
/* 1547 */
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
/* 1546 */
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
/* 1545 */
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
/* 1544 */
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
/* 1543 */
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
/* 1542 */
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
/* 1541 */
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
/* 1540 */
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
/* 1539 */
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
/* 1538 */
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
/* 1537 */
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
/* 1536 */
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
/* 1535 */
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
/* 1534 */
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
/* 1533 */
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
/* 1532 */
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
/* 1531 */
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
/* 1530 */
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
/* 1529 */
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
/* 1528 */
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
/* 1527 */
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
/* 1526 */
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
/* 1525 */
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
/* 1524 */
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
/* 1523 */
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
/* 1522 */
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
/* 1521 */
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
/* 1520 */
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
/* 1519 */
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
/* 1518 */
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
/* 1517 */
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
/* 1516 */
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
/* 1515 */
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
/* 1514 */
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
/* 1513 */
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
/* 1512 */
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
/* 1511 */
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
/* 1510 */
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
/* 1509 */
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
/* 1508 */
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
/* 1507 */
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
/* 1506 */
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
/* 1505 */
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
/* 1504 */
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
/* 1503 */
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
/* 1502 */
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
/* 1501 */
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
/* 1403 */
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
/* 1402 */
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
/* 1401 */
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
/* 1303 */
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
/* 1302 */
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
/* 1301 */
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
/* 1221 */
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
/* 1220 */
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
/* 1219 */
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
/* 1218 */
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
/* 1217 */
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
/* 1216 */
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
/* 1215 */
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
/* 1214 */
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
/* 1213 */
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
/* 1212 */
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
/* 1211 */
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
/* 1210 */
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
/* 1209 */
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
/* 1208 */
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
/* 1207 */
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
/* 1206 */
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
/* 1205 */
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
/* 1204 */
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
/* 1203 */
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
/* 1202 */
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
/* 1201 */
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
/* 1122 */
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
/* 1121 */
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
/* 1120 */
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
/* 1119 */
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
/* 1118 */
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
/* 1117 */
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
/* 1116 */
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
/* 1115 */
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
/* 1114 */
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
/* 1113 */
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
/* 1112 */
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
/* 1111 */
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
/* 1110 */
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
/* 1109 */
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
/* 1108 */
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
/* 1107 */
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
/* 1106 */
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
/* 1105 */
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
/* 1104 */
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
/* 1103 */
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
/* 1102 */
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
/* 1101 */
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
/* 1100 */
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
/* 1099 */
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
/* 1098 */
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
/* 1097 */
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
/* 1096 */
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
/* 1095 */
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
/* 1094 */
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
/* 1093 */
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
/* 1092 */
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
/* 1091 */
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
/* 1090 */
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
/* 1089 */
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
/* 1088 */
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
/* 1087 */
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
/* 1086 */
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
/* 1085 */
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
/* 1084 */
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
/* 1083 */
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
/* 1082 */
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
/* 1081 */
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
/* 1080 */
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
/* 1079 */
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
/* 1078 */
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
/* 1077 */
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
/* 1076 */
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
/* 1075 */
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
/* 1074 */
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
/* 1073 */
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
/* 1072 */
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
/* 1071 */
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
/* 1070 */
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
/* 1069 */
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
/* 1068 */
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
/* 1067 */
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
/* 1066 */
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
/* 1065 */
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
/* 1064 */
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
/* 1063 */
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
/* 1062 */
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
/* 1061 */
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
/* 1060 */
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
/* 1059 */
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
/* 1058 */
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
/* 1057 */
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
/* 1056 */
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
/* 1055 */
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
/* 1054 */
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
/* 1053 */
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
/* 1052 */
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
/* 1051 */
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
/* 1050 */
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
/* 1049 */
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
/* 1048 */
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
/* 1047 */
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
/* 1046 */
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
/* 1045 */
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
/* 1044 */
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
/* 1043 */
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
/* 1042 */
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
/* 1041 */
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
/* 1040 */
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
/* 1039 */
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
/* 1038 */
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
/* 1037 */
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
/* 1036 */
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
/* 1035 */
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
/* 1034 */
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
/* 1033 */
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
/* 1032 */
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
/* 1031 */
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
/* 1030 */
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
/* 1029 */
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
/* 1028 */
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
/* 1027 */
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
/* 1026 */
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
/* 1025 */
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
/* 1024 */
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
/* 1023 */
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
/* 1022 */
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
/* 1021 */
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
/* 1020 */
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
/* 1019 */
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
/* 1018 */
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
/* 1017 */
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
/* 1016 */
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
/* 1015 */
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
/* 1014 */
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
/* 1013 */
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
/* 1012 */
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
/* 1011 */
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
/* 1010 */
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
/* 1009 */
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
/* 1008 */
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
/* 1007 */
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
/* 1006 */
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
/* 1005 */
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
/* 1004 */
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
/* 1003 */
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
/* 1002 */
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
/* 1001 */
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
/* 801 */
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
/* 541 */
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
/* 540 */
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
/* 539 */
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
/* 538 */
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
/* 537 */
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
/* 536 */
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
/* 535 */
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
/* 534 */
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
/* 533 */
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
/* 532 */
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
/* 531 */
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
/* 530 */
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
/* 529 */
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
/* 528 */
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
/* 527 */
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
/* 526 */
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
/* 525 */
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
/* 524 */
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
/* 523 */
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
/* 522 */
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
/* 521 */
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
/* 520 */
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
/* 519 */
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
/* 518 */
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
/* 517 */
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
/* 516 */
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
/* 515 */
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
/* 514 */
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
/* 513 */
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
/* 512 */
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
/* 511 */
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
/* 510 */
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
/* 509 */
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
/* 508 */
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
/* 507 */
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
/* 506 */
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
/* 505 */
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
/* 504 */
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
/* 503 */
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
/* 502 */
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
/* 501 */
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
/* 205 */
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
/* 204 */
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
/* 203 */
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
/* 202 */
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
/* 201 */
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
/* 111 */
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
/* 110 */
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
/* 109 */
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
/* 108 */
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
/* 107 */
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
/* 106 */
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
/* 105 */
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
/* 104 */
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
/* 103 */
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
/* 102 */
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
/* 101 */
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
/* 100 */
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
/* 99 */
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
/* 98 */
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
/* 97 */
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
/* 96 */
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
/* 95 */
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
/* 94 */
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
/* 93 */
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
/* 92 */
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
/* 91 */
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
/* 90 */
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
/* 89 */
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
/* 88 */
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
/* 87 */
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
/* 86 */
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
/* 85 */
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
/* 84 */
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
/* 83 */
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
/* 82 */
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
/* 81 */
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
/* 80 */
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
/* 79 */
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
/* 78 */
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
/* 77 */
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
/* 76 */
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
/* 75 */
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
/* 74 */
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
/* 73 */
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
/* 72 */
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
/* 71 */
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
/* 70 */
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
/* 69 */
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
/* 68 */
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
/* 67 */
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
/* 66 */
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
/* 65 */
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
/* 64 */
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
/* 63 */
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
/* 62 */
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
/* 61 */
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
/* 60 */
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
/* 59 */
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
/* 58 */
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
/* 57 */
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
/* 56 */
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
/* 55 */
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
/* 54 */
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
/* 53 */
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
/* 52 */
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
/* 51 */
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
/* 50 */
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
/* 49 */
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
/* 48 */
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
/* 47 */
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
/* 46 */
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
/* 45 */
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
/* 44 */
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
/* 43 */
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
/* 42 */
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
/* 41 */
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
/* 40 */
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
/* 39 */
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
/* 38 */
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
/* 37 */
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
/* 36 */
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
/* 35 */
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
/* 34 */
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
/* 33 */
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
/* 32 */
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
/* 31 */
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
/* 30 */
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
/* 29 */
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
/* 28 */
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
/* 27 */
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
/* 26 */
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
/* 25 */
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
/* 24 */
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
/* 23 */
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
/* 22 */
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
/* 21 */
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
/* 20 */
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
/* 19 */
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
/* 18 */
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
/* 17 */
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
/* 16 */
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
/* 15 */
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
/* 14 */
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
/* 13 */
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
/* 12 */
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
/* 11 */
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
/* 10 */
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
/* 9 */
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
/* 8 */
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
/* 7 */
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
/* 6 */
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
/* 5 */
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
/* 4 */
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
/* 3 */
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
/* 2 */
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
/* 1 */
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
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
pub type xmlExternalEntityLoader
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: *const std::os::raw::c_char, _: xmlParserCtxtPtr)
               -> xmlParserInputPtr>;
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
pub type xmlInputMatchCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> std::os::raw::c_int>;
/* *
 * xmlInputOpenCallback:
 * @filename: the filename or URI
 *
 * Callback used in the I/O Input API to open the resource
 *
 * Returns an Input context or NULL in case or error
 */
pub type xmlInputOpenCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_void>;
/*
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
/* Store big lines numbers in text PSVI field */
/* ignore internal document encoding hint */
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_1 = 4194304;
/* parse using SAX2 interface before 2.7.0 */
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_1 = 2097152;
/* relax any hardcoded limit from the parser */
pub const XML_PARSE_OLDSAX: C2RustUnnamed_1 = 1048576;
/* do not fixup XINCLUDE xml:base uris */
pub const XML_PARSE_HUGE: C2RustUnnamed_1 = 524288;
/* parse using XML-1.0 before update 5 */
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_1 = 262144;
/* compact small text nodes; no modification of
                                   the tree allowed afterwards (will possibly
				   crash if you try to modify the tree) */
pub const XML_PARSE_OLD10: C2RustUnnamed_1 = 131072;
/* do not generate XINCLUDE START/END nodes */
pub const XML_PARSE_COMPACT: C2RustUnnamed_1 = 65536;
/* merge CDATA as text nodes */
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_1 = 32768;
/* remove redundant namespaces declarations */
pub const XML_PARSE_NOCDATA: C2RustUnnamed_1 = 16384;
/* Do not reuse the context dictionary */
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_1 = 8192;
/* Forbid network access */
pub const XML_PARSE_NODICT: C2RustUnnamed_1 = 4096;
/* Implement XInclude substitition  */
pub const XML_PARSE_NONET: C2RustUnnamed_1 = 2048;
/* use the SAX1 interface internally */
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_1 = 1024;
/* remove blank nodes */
pub const XML_PARSE_SAX1: C2RustUnnamed_1 = 512;
/* pedantic error reporting */
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_1 = 256;
/* suppress warning reports */
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_1 = 128;
/* suppress error reports */
pub const XML_PARSE_NOWARNING: C2RustUnnamed_1 = 64;
/* validate with the DTD */
pub const XML_PARSE_NOERROR: C2RustUnnamed_1 = 32;
/* default DTD attributes */
pub const XML_PARSE_DTDVALID: C2RustUnnamed_1 = 16;
/* load the external subset */
pub const XML_PARSE_DTDATTR: C2RustUnnamed_1 = 8;
/* substitute entities */
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_1 = 4;
/* recover on errors */
pub const XML_PARSE_NOENT: C2RustUnnamed_1 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
// #include <libxml/xmlwriter.h>
// from xmllint.c:
// from xmlreader.c:
/* the parsing mode */
/* when walking an existing doc */
/* is there any validation */
/* what structure were deallocated */
/* the parser context */
/* the parser SAX callbacks */
/* the input */
/* initial SAX callbacks */
/* idem */
/* idem */
/* idem */
/* base of the segment in the input */
/* current position in the input */
/* current node */
/* current attribute node */
/* depth of the current node */
/* fake xmlNs chld */
/* preserve the resulting document */
/* used to return const xmlChar * */
/* the context dictionary */
/* entity stack when traversing entities content */
/* Current Entity Ref Node */
/* Depth of the entities stack */
/* Max depth of the entities stack */
/* array of entities */
/* error handling */
/* callback function */
/* callback function user argument */
/* Handling of RelaxNG validation */
/* The Relax NG schemas */
/* The Relax NG validation context */
/* 1 if the context was provided by the user */
/* The number of errors detected */
/* the node if RNG not progressive */
/* Handling of Schemas validation */
/* The Schemas schemas */
/* The Schemas validation context */
/* 1 if the context was provided by the user */
/* The number of errors detected */
/* the schemas plug in SAX pipeline */
/* Handling of XInclude processing */
/* is xinclude asked for */
/* the xinclude name from dict */
/* the xinclude context */
/* counts for xinclude */
/* number of preserve patterns */
/* max preserve patterns */
/* array of preserve patterns */
/* level of preserves */
/* the set of options set */
/* Structured error handling */
/* callback function */
// from xmlschemas.c:
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaSAXPlug {
    pub magic: std::os::raw::c_uint,
    pub user_sax_ptr: *mut xmlSAXHandlerPtr,
    pub user_sax: xmlSAXHandlerPtr,
    pub user_data_ptr: *mut *mut std::os::raw::c_void,
    pub user_data: *mut std::os::raw::c_void,
    pub schemas_sax: xmlSAXHandler,
    pub ctxt: xmlSchemaValidCtxtPtr,
}
/*
 * Interface to insert Schemas SAX validation in a SAX stream
 */
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
/*
 * Summary: the XMLReader implementation
 * Description: API of the XML streaming API based on C# interfaces.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlParserSeverities:
 *
 * How severe an error callback is when the per-reader error callback API
 * is used.
 */
pub type xmlParserSeverities = std::os::raw::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextReader {
    pub mode: std::os::raw::c_int,
    pub doc: xmlDocPtr,
    pub validate: xmlTextReaderValidate,
    pub allocs: std::os::raw::c_int,
    pub state: xmlTextReaderState,
    pub ctxt: xmlParserCtxtPtr,
    pub sax: xmlSAXHandlerPtr,
    pub input: xmlParserInputBufferPtr,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub characters: charactersSAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub base: std::os::raw::c_uint,
    pub cur: std::os::raw::c_uint,
    pub node: xmlNodePtr,
    pub curnode: xmlNodePtr,
    pub depth: std::os::raw::c_int,
    pub faketext: xmlNodePtr,
    pub preserve: std::os::raw::c_int,
    pub buffer: xmlBufPtr,
    pub dict: xmlDictPtr,
    pub ent: xmlNodePtr,
    pub entNr: std::os::raw::c_int,
    pub entMax: std::os::raw::c_int,
    pub entTab: *mut xmlNodePtr,
    pub errorFunc: xmlTextReaderErrorFunc,
    pub errorFuncArg: *mut std::os::raw::c_void,
    pub rngSchemas: xmlRelaxNGPtr,
    pub rngValidCtxt: xmlRelaxNGValidCtxtPtr,
    pub rngPreserveCtxt: std::os::raw::c_int,
    pub rngValidErrors: std::os::raw::c_int,
    pub rngFullNode: xmlNodePtr,
    pub xsdSchemas: xmlSchemaPtr,
    pub xsdValidCtxt: xmlSchemaValidCtxtPtr,
    pub xsdPreserveCtxt: std::os::raw::c_int,
    pub xsdValidErrors: std::os::raw::c_int,
    pub xsdPlug: xmlSchemaSAXPlugPtr,
    pub xinclude: std::os::raw::c_int,
    pub xinclude_name: *const xmlChar,
    pub xincctxt: xmlXIncludeCtxtPtr,
    pub in_xinclude: std::os::raw::c_int,
    pub patternNr: std::os::raw::c_int,
    pub patternMax: std::os::raw::c_int,
    pub patternTab: *mut xmlPatternPtr,
    pub preserves: std::os::raw::c_int,
    pub parserFlags: std::os::raw::c_int,
    pub sErrorFunc: xmlStructuredErrorFunc,
}
/*
 * Summary: pattern expression handling
 * Description: allows to compile and test pattern expressions for nodes
 *              either in a tree or based on a parser state.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlPattern:
 *
 * A compiled (XPath based) pattern to select nodes
 */
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
/*
 * Summary: implementation of XInclude
 * Description: API to handle XInclude processing,
 * implements the
 * World Wide Web Consortium Last Call Working Draft 10 November 2003
 * http://www.w3.org/TR/2003/WD-xinclude-20031110
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * XINCLUDE_NS:
 *
 * Macro defining the Xinclude namespace: http://www.w3.org/2003/XInclude
 */
/* *
 * XINCLUDE_OLD_NS:
 *
 * Macro defining the draft Xinclude namespace: http://www.w3.org/2001/XInclude
 */
/* *
 * XINCLUDE_NODE:
 *
 * Macro defining "include"
 */
/* *
 * XINCLUDE_FALLBACK:
 *
 * Macro defining "fallback"
 */
/* *
 * XINCLUDE_HREF:
 *
 * Macro defining "href"
 */
/* *
 * XINCLUDE_PARSE:
 *
 * Macro defining "parse"
 */
/* *
 * XINCLUDE_PARSE_XML:
 *
 * Macro defining "xml"
 */
/* *
 * XINCLUDE_PARSE_TEXT:
 *
 * Macro defining "text"
 */
/* *
 * XINCLUDE_PARSE_ENCODING:
 *
 * Macro defining "encoding"
 */
/* *
 * XINCLUDE_PARSE_XPOINTER:
 *
 * Macro defining "xpointer"
 */
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
/* *
 * xmlTextReader:
 *
 * Structure for an xmlReader context.
 */
/* *
 * xmlTextReaderPtr:
 *
 * Pointer to an xmlReader context.
 */
/*
 * Constructors & Destructor
 */
/*
 * Iterators
 */
/*
 * Attributes of the node
 */
/*
 * use the Const version of the routine for
 * better performance and simpler code
 */
/*
 * Methods of the XmlTextReader
 */
/*
 * Extensions
 */
/* LIBXML_PATTERN_ENABLED */
/*
 * Index lookup
 */
/*
 * New more complete APIs for simpler creation and reuse of readers
 */
/*
 * Error handling extensions
 */
/* *
 * xmlTextReaderErrorFunc:
 * @arg: the user argument
 * @msg: the message
 * @severity: the severity of the error
 * @locator: a locator indicating where the error occurred
 *
 * Signature of an error callback from a reader parser
 */
pub type xmlTextReaderErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: xmlParserSeverities,
                                _: xmlTextReaderLocatorPtr) -> ()>;
pub type xmlTextReaderLocatorPtr = *mut std::os::raw::c_void;
pub type xmlTextReaderState = std::os::raw::c_int;
pub const XML_TEXTREADER_ERROR: xmlTextReaderState = 6;
pub const XML_TEXTREADER_DONE: xmlTextReaderState = 5;
pub const XML_TEXTREADER_BACKTRACK: xmlTextReaderState = 4;
pub const XML_TEXTREADER_EMPTY: xmlTextReaderState = 3;
pub const XML_TEXTREADER_END: xmlTextReaderState = 2;
pub const XML_TEXTREADER_ELEMENT: xmlTextReaderState = 1;
pub const XML_TEXTREADER_START: xmlTextReaderState = 0;
pub const XML_TEXTREADER_NONE: xmlTextReaderState = -1;
pub type xmlTextReaderValidate = std::os::raw::c_uint;
pub const XML_TEXTREADER_VALIDATE_XSD: xmlTextReaderValidate = 4;
pub const XML_TEXTREADER_VALIDATE_RNG: xmlTextReaderValidate = 2;
pub const XML_TEXTREADER_VALIDATE_DTD: xmlTextReaderValidate = 1;
pub const XML_TEXTREADER_NOT_VALIDATE: xmlTextReaderValidate = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
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
/*
 * testrecurse.c: C program to run libxml2 regression tests checking entities
 *            recursions
 *
 * To compile on Unixes:
 * cc -o testrecurse `xml2-config --cflags` testrecurse.c `xml2-config --libs` -lpthread
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
/*
 * O_BINARY is just for Windows compatibility - if it isn't defined
 * on this system, avoid any compilation error
 */
pub type functest
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: *const std::os::raw::c_char,
                                _: *const std::os::raw::c_char, _: std::os::raw::c_int)
               -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct testDesc {
    pub desc: *const std::os::raw::c_char,
    pub func: functest,
    pub in_0: *const std::os::raw::c_char,
    pub out: *const std::os::raw::c_char,
    pub suffix: *const std::os::raw::c_char,
    pub err: *const std::os::raw::c_char,
    pub options: std::os::raw::c_int,
}
pub type testDescPtr = *mut testDesc;
pub type __size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut std::os::raw::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: std::os::raw::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                               -> *mut std::os::raw::c_void>,
    pub gl_opendir: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char)
                               -> *mut std::os::raw::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                              _: *mut std::os::raw::c_void)
                             -> std::os::raw::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                             _: *mut std::os::raw::c_void)
                            -> std::os::raw::c_int>,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const std::os::raw::c_char,
                          mut __statbuf: *mut stat) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
/* ***********************************************************************
 *									*
 *		Huge document generator					*
 *									*
 ************************************************************************/
static mut start: *const std::os::raw::c_char =
    b"<!DOCTYPE foo [<!ENTITY f \'some internal data\'> <!ENTITY e \'&f;&f;\'> <!ENTITY d \'&e;&e;\'> ]> <foo>\x00"
        as *const u8 as *const std::os::raw::c_char;
static mut segment: *const std::os::raw::c_char =
    b"  <bar>&e; &f; &d;</bar>\n\x00" as *const u8 as *const std::os::raw::c_char;
static mut finish: *const std::os::raw::c_char =
    b"</foo>\x00" as *const u8 as *const std::os::raw::c_char;
static mut curseg: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut current: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
static mut rlen: std::os::raw::c_int = 0;
/* *
 * hugeMatch:
 * @URI: an URI to test
 *
 * Check for an huge: query
 *
 * Returns 1 if yes and 0 if another Input module should be used
 */
unsafe extern "C" fn hugeMatch(mut URI: *const std::os::raw::c_char) -> std::os::raw::c_int {
    if !URI.is_null() &&
           strncmp(URI, b"huge:\x00" as *const u8 as *const std::os::raw::c_char,
                   4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * hugeOpen:
 * @URI: an URI to test
 *
 * Return a pointer to the huge: query handler, in this example simply
 * the current pointer...
 *
 * Returns an Input context or NULL in case or error
 */
unsafe extern "C" fn hugeOpen(mut URI: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    if URI.is_null() ||
           strncmp(URI, b"huge:\x00" as *const u8 as *const std::os::raw::c_char,
                   4 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 {
        return 0 as *mut std::os::raw::c_void
    }
    rlen = strlen(start) as std::os::raw::c_int;
    current = start;
    return current as *mut std::os::raw::c_void;
}
/* *
 * hugeClose:
 * @context: the read context
 *
 * Close the huge: query handler
 *
 * Returns 0 or -1 in case of error
 */
unsafe extern "C" fn hugeClose(mut context: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    if context.is_null() { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * hugeRead:
 * @context: the read context
 * @buffer: where to store data
 * @len: number of bytes to read
 *
 * Implement an huge: query read.
 *
 * Returns the number of bytes read or -1 in case of error
 */
unsafe extern "C" fn hugeRead(mut context: *mut std::os::raw::c_void,
                              mut buffer: *mut std::os::raw::c_char,
                              mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    if context.is_null() || buffer.is_null() || len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if len >= rlen {
        if curseg >= 1000000 as std::os::raw::c_int + 1 as std::os::raw::c_int {
            rlen = 0 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
        len = rlen;
        rlen = 0 as std::os::raw::c_int;
        memcpy(buffer as *mut std::os::raw::c_void, current as *const std::os::raw::c_void,
               len as std::os::raw::c_ulong);
        curseg += 1;
        if curseg == 1000000 as std::os::raw::c_int {
            fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            rlen = strlen(finish) as std::os::raw::c_int;
            current = finish
        } else {
            if curseg % (1000000 as std::os::raw::c_int / 10 as std::os::raw::c_int) ==
                   0 as std::os::raw::c_int {
                fprintf(stderr, b".\x00" as *const u8 as *const std::os::raw::c_char);
            }
            rlen = strlen(segment) as std::os::raw::c_int;
            current = segment
        }
    } else {
        memcpy(buffer as *mut std::os::raw::c_void, current as *const std::os::raw::c_void,
               len as std::os::raw::c_ulong);
        rlen -= len;
        current = current.offset(len as isize)
    }
    return len;
}
/* ***********************************************************************
 *									*
 *		Libxml2 specific routines				*
 *									*
 ************************************************************************/
static mut nb_tests: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_errors: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_leaks: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut extraMemoryFromResolver: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn fatalError() -> std::os::raw::c_int {
    fprintf(stderr,
            b"Exitting tests on fatal error\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    exit(1 as std::os::raw::c_int);
}
/*
 * We need to trap calls to the resolver to not account memory for the catalog
 * which is shared to the current running test. We also don't want to have
 * network downloads modifying tests.
 */
unsafe extern "C" fn testExternalEntityLoader(mut URL: *const std::os::raw::c_char,
                                              mut ID: *const std::os::raw::c_char,
                                              mut ctxt: xmlParserCtxtPtr)
 -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if checkTestFile(URL) != 0 {
        ret = xmlNoNetExternalEntityLoader(URL, ID, ctxt)
    } else {
        let mut memused: std::os::raw::c_int = xmlMemUsed();
        ret = xmlNoNetExternalEntityLoader(URL, ID, ctxt);
        extraMemoryFromResolver += xmlMemUsed() - memused
    }
    return ret;
}
/* *
 * xmlParserPrintFileContext:
 * @input:  an xmlParserInputPtr input
 *
 * Displays current context within the input content for error tracking
 */
unsafe extern "C" fn xmlParserPrintFileContextInternal(mut input:
                                                           xmlParserInputPtr,
                                                       mut chanl:
                                                           xmlGenericErrorFunc,
                                                       mut data:
                                                           *mut std::os::raw::c_void) {
    let mut cur: *const xmlChar =
        0 as
            *const xmlChar; /* GCC warns if signed, because compared with sizeof() */
    let mut base: *const xmlChar =
        0 as *const xmlChar; /* space for 80 chars + line terminator */
    let mut n: std::os::raw::c_uint = 0;
    let mut col: std::os::raw::c_uint = 0;
    let mut content: [xmlChar; 81] = [0; 81];
    let mut ctnt: *mut xmlChar = 0 as *mut xmlChar;
    if input.is_null() { return }
    cur = (*input).cur;
    base = (*input).base;
    /* skip backwards over any end-of-lines */
    while cur > base &&
              (*cur as std::os::raw::c_int == '\n' as i32 ||
                   *cur as std::os::raw::c_int == '\r' as i32) {
        cur = cur.offset(-1)
    }
    n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    loop 
         /* search backwards for beginning-of-line (to max buff size) */
         {
        let fresh0 = n;
        n = n.wrapping_add(1);
        if !((fresh0 as std::os::raw::c_ulong) <
                 (::std::mem::size_of::<[xmlChar; 81]>() as
                      std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong) &&
                 cur > base && *cur as std::os::raw::c_int != '\n' as i32 &&
                 *cur as std::os::raw::c_int != '\r' as i32) {
            break ;
        }
        cur = cur.offset(-1)
    }
    if *cur as std::os::raw::c_int == '\n' as i32 ||
           *cur as std::os::raw::c_int == '\r' as i32 {
        cur = cur.offset(1)
    }
    /* calculate the error position in terms of the current position */
    col =
        (*input).cur.offset_from(cur) as std::os::raw::c_long as
            std::os::raw::c_uint;
    /* search forward for end-of-line (to max buff size) */
    n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    ctnt = content.as_mut_ptr();
    /* copy selected text to our buffer */
    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *cur as std::os::raw::c_int != '\n' as i32 &&
              *cur as std::os::raw::c_int != '\r' as i32 &&
              (n as std::os::raw::c_ulong) <
                  (::std::mem::size_of::<[xmlChar; 81]>() as
                       std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) {
        let fresh1 = cur;
        cur = cur.offset(1);
        let fresh2 = ctnt;
        ctnt = ctnt.offset(1);
        *fresh2 = *fresh1;
        n = n.wrapping_add(1)
    }
    *ctnt = 0 as std::os::raw::c_int as xmlChar;
    /* print out the selected text */
    chanl.expect("non-null function pointer")(data,
                                              b"%s\n\x00" as *const u8 as
                                                  *const std::os::raw::c_char,
                                              content.as_mut_ptr());
    /* create blank line with problem pointer */
    n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    ctnt = content.as_mut_ptr();
    /* (leave buffer space for pointer + line terminator) */
    while n < col &&
              {
                  let fresh3 = n;
                  n = n.wrapping_add(1);
                  ((fresh3 as std::os::raw::c_ulong)) <
                      (::std::mem::size_of::<[xmlChar; 81]>() as
                           std::os::raw::c_ulong).wrapping_sub(2 as std::os::raw::c_int as
                                                           std::os::raw::c_ulong)
              } && *ctnt as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *ctnt as std::os::raw::c_int != '\t' as i32 {
            *ctnt = ' ' as i32 as xmlChar
        }
        ctnt = ctnt.offset(1)
    }
    let fresh4 = ctnt;
    ctnt = ctnt.offset(1);
    *fresh4 = '^' as i32 as xmlChar;
    *ctnt = 0 as std::os::raw::c_int as xmlChar;
    chanl.expect("non-null function pointer")(data,
                                              b"%s\n\x00" as *const u8 as
                                                  *const std::os::raw::c_char,
                                              content.as_mut_ptr());
}
unsafe extern "C" fn testStructuredErrorHandler(mut ctx: *mut std::os::raw::c_void,
                                                mut err: xmlErrorPtr) {
    let mut file: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut line: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut code: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut domain: std::os::raw::c_int = 0;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut level: xmlErrorLevel = XML_ERR_NONE;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut cur: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut ctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    if err.is_null() { return }
    file = (*err).file;
    line = (*err).line;
    code = (*err).code;
    domain = (*err).domain;
    level = (*err).level;
    node = (*err).node as xmlNodePtr;
    if domain == XML_FROM_PARSER as std::os::raw::c_int ||
           domain == XML_FROM_HTML as std::os::raw::c_int ||
           domain == XML_FROM_DTD as std::os::raw::c_int ||
           domain == XML_FROM_NAMESPACE as std::os::raw::c_int ||
           domain == XML_FROM_IO as std::os::raw::c_int ||
           domain == XML_FROM_VALID as std::os::raw::c_int {
        ctxt = (*err).ctxt as xmlParserCtxtPtr
    }
    str = (*err).message;
    if code == XML_ERR_OK as std::os::raw::c_int { return }
    if !node.is_null() &&
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        name = (*node).name
    }
    /*
     * Maintain the compatibility with the legacy error handling
     */
    if !ctxt.is_null() {
        input = (*ctxt).input;
        if !input.is_null() && (*input).filename.is_null() &&
               (*ctxt).inputNr > 1 as std::os::raw::c_int {
            cur = input;
            input =
                *(*ctxt).inputTab.offset(((*ctxt).inputNr - 2 as std::os::raw::c_int)
                                             as isize)
        }
        if !input.is_null() {
            if !(*input).filename.is_null() {
                channel_testrecurse(data,
                                    b"%s:%d: \x00" as *const u8 as
                                        *const std::os::raw::c_char,
                                    (*input).filename, (*input).line);
            } else if line != 0 as std::os::raw::c_int &&
                          domain == XML_FROM_PARSER as std::os::raw::c_int {
                channel_testrecurse(data,
                                    b"Entity: line %d: \x00" as *const u8 as
                                        *const std::os::raw::c_char, (*input).line);
            }
        }
    } else if !file.is_null() {
        channel_testrecurse(data,
                            b"%s:%d: \x00" as *const u8 as
                                *const std::os::raw::c_char, file, line);
    } else if line != 0 as std::os::raw::c_int &&
                  domain == XML_FROM_PARSER as std::os::raw::c_int {
        channel_testrecurse(data,
                            b"Entity: line %d: \x00" as *const u8 as
                                *const std::os::raw::c_char, line);
    }
    if !name.is_null() {
        channel_testrecurse(data,
                            b"element %s: \x00" as *const u8 as
                                *const std::os::raw::c_char, name);
    }
    if code == XML_ERR_OK as std::os::raw::c_int { return }
    match domain {
        1 => {
            channel_testrecurse(data,
                                b"parser \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        3 => {
            channel_testrecurse(data,
                                b"namespace \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        4 | 23 => {
            channel_testrecurse(data,
                                b"validity \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        5 => {
            channel_testrecurse(data,
                                b"HTML parser \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        6 => {
            channel_testrecurse(data,
                                b"memory \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        7 => {
            channel_testrecurse(data,
                                b"output \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        8 => {
            channel_testrecurse(data,
                                b"I/O \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        11 => {
            channel_testrecurse(data,
                                b"XInclude \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        12 => {
            channel_testrecurse(data,
                                b"XPath \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        13 => {
            channel_testrecurse(data,
                                b"parser \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        14 => {
            channel_testrecurse(data,
                                b"regexp \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        26 => {
            channel_testrecurse(data,
                                b"module \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        17 => {
            channel_testrecurse(data,
                                b"Schemas validity \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        16 => {
            channel_testrecurse(data,
                                b"Schemas parser \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        18 => {
            channel_testrecurse(data,
                                b"Relax-NG parser \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        19 => {
            channel_testrecurse(data,
                                b"Relax-NG validity \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        20 => {
            channel_testrecurse(data,
                                b"Catalog \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        21 => {
            channel_testrecurse(data,
                                b"C14N \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        22 => {
            channel_testrecurse(data,
                                b"XSLT \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        _ => { }
    }
    if code == XML_ERR_OK as std::os::raw::c_int { return }
    match level as std::os::raw::c_uint {
        0 => {
            channel_testrecurse(data,
                                b": \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        1 => {
            channel_testrecurse(data,
                                b"warning : \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        2 => {
            channel_testrecurse(data,
                                b"error : \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        3 => {
            channel_testrecurse(data,
                                b"error : \x00" as *const u8 as
                                    *const std::os::raw::c_char);
        }
        _ => { }
    }
    if code == XML_ERR_OK as std::os::raw::c_int { return }
    if !str.is_null() {
        let mut len: std::os::raw::c_int = 0;
        len = xmlStrlen(str as *const xmlChar);
        if len > 0 as std::os::raw::c_int &&
               *str.offset((len - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                   != '\n' as i32 {
            channel_testrecurse(data,
                                b"%s\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, str);
        } else {
            channel_testrecurse(data,
                                b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                                str);
        }
    } else {
        channel_testrecurse(data,
                            b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                            b"out of memory error\x00" as *const u8 as
                                *const std::os::raw::c_char);
    }
    if code == XML_ERR_OK as std::os::raw::c_int { return }
    if !ctxt.is_null() {
        xmlParserPrintFileContextInternal(input,
                                          Some(channel_testrecurse as
                                                   unsafe extern "C" fn(_:
                                                                            *mut std::os::raw::c_void,
                                                                        _:
                                                                            *const std::os::raw::c_char,
                                                                        _:
                                                                            ...)
                                                       -> ()), data);
        if !cur.is_null() {
            if !(*cur).filename.is_null() {
                channel_testrecurse(data,
                                    b"%s:%d: \n\x00" as *const u8 as
                                        *const std::os::raw::c_char, (*cur).filename,
                                    (*cur).line);
            } else if line != 0 as std::os::raw::c_int &&
                          domain == XML_FROM_PARSER as std::os::raw::c_int {
                channel_testrecurse(data,
                                    b"Entity: line %d: \n\x00" as *const u8 as
                                        *const std::os::raw::c_char, (*cur).line);
            }
            xmlParserPrintFileContextInternal(cur,
                                              Some(channel_testrecurse as
                                                       unsafe extern "C" fn(_:
                                                                                *mut std::os::raw::c_void,
                                                                            _:
                                                                                *const std::os::raw::c_char,
                                                                            _:
                                                                                ...)
                                                           -> ()), data);
        }
    }
    if domain == XML_FROM_XPATH as std::os::raw::c_int && !(*err).str1.is_null() &&
           (*err).int1 >= 0 as std::os::raw::c_int && (*err).int1 < 100 as std::os::raw::c_int
           && (*err).int1 < xmlStrlen((*err).str1 as *const xmlChar) {
        let mut buf: [xmlChar; 150] = [0; 150];
        let mut i: std::os::raw::c_int = 0;
        channel_testrecurse(data,
                            b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                            (*err).str1);
        i = 0 as std::os::raw::c_int;
        while i < (*err).int1 {
            buf[i as usize] = ' ' as i32 as xmlChar;
            i += 1
        }
        let fresh5 = i;
        i = i + 1;
        buf[fresh5 as usize] = '^' as i32 as xmlChar;
        buf[i as usize] = 0 as std::os::raw::c_int as xmlChar;
        channel_testrecurse(data,
                            b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                            buf.as_mut_ptr());
    };
}
unsafe extern "C" fn initializeLibxml2() {
    *__xmlGetWarningsDefaultValue() = 0 as std::os::raw::c_int;
    xmlPedanticParserDefault(0 as std::os::raw::c_int);
    xmlMemSetup(Some(xmlMemFree as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()),
                Some(xmlMemMalloc as
                         unsafe extern "C" fn(_: size_t)
                             -> *mut std::os::raw::c_void),
                Some(xmlMemRealloc as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
                             -> *mut std::os::raw::c_void),
                Some(xmlMemoryStrdup as
                         unsafe extern "C" fn(_: *const std::os::raw::c_char)
                             -> *mut std::os::raw::c_char));
    xmlInitParser();
    xmlSetExternalEntityLoader(Some(testExternalEntityLoader as
                                        unsafe extern "C" fn(_:
                                                                 *const std::os::raw::c_char,
                                                             _:
                                                                 *const std::os::raw::c_char,
                                                             _:
                                                                 xmlParserCtxtPtr)
                                            -> xmlParserInputPtr));
    xmlSetStructuredErrorFunc(0 as *mut std::os::raw::c_void,
                              Some(testStructuredErrorHandler as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _: xmlErrorPtr)
                                           -> ()));
    /*
     * register the new I/O handlers
     */
    if xmlRegisterInputCallbacks(Some(hugeMatch as
                                          unsafe extern "C" fn(_:
                                                                   *const std::os::raw::c_char)
                                              -> std::os::raw::c_int),
                                 Some(hugeOpen as
                                          unsafe extern "C" fn(_:
                                                                   *const std::os::raw::c_char)
                                              -> *mut std::os::raw::c_void),
                                 Some(hugeRead as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void,
                                                               _:
                                                                   *mut std::os::raw::c_char,
                                                               _: std::os::raw::c_int)
                                              -> std::os::raw::c_int),
                                 Some(hugeClose as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void)
                                              -> std::os::raw::c_int)) <
           0 as std::os::raw::c_int {
        fprintf(stderr,
                b"failed to register Huge handler\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    };
}
/* ***********************************************************************
 *									*
 *		File name and path utilities				*
 *									*
 ************************************************************************/
unsafe extern "C" fn baseFilename(mut filename: *const std::os::raw::c_char)
 -> *const std::os::raw::c_char {
    let mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if filename.is_null() { return 0 as *const std::os::raw::c_char }
    cur =
        &*filename.offset((strlen as
                               unsafe extern "C" fn(_: *const std::os::raw::c_char)
                                   -> std::os::raw::c_ulong)(filename) as isize) as
            *const std::os::raw::c_char;
    while cur > filename && *cur as std::os::raw::c_int != '/' as i32 {
        cur = cur.offset(-1)
    }
    if *cur as std::os::raw::c_int == '/' as i32 {
        return cur.offset(1 as std::os::raw::c_int as isize)
    }
    return cur;
}
unsafe extern "C" fn resultFilename(mut filename: *const std::os::raw::c_char,
                                    mut out: *const std::os::raw::c_char,
                                    mut suffix: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut res: [std::os::raw::c_char; 500] = [0; 500];
    let mut suffixbuff: [std::os::raw::c_char; 500] = [0; 500];
    /* ************
    if ((filename[0] == 't') && (filename[1] == 'e') &&
        (filename[2] == 's') && (filename[3] == 't') &&
	(filename[4] == '/'))
	filename = &filename[5];
 *************/
    base = baseFilename(filename);
    if suffix.is_null() {
        suffix = b".tmp\x00" as *const u8 as *const std::os::raw::c_char
    }
    if out.is_null() { out = b"\x00" as *const u8 as *const std::os::raw::c_char }
    strncpy(suffixbuff.as_mut_ptr(), suffix,
            499 as std::os::raw::c_int as std::os::raw::c_ulong);
    snprintf(res.as_mut_ptr(), 499 as std::os::raw::c_int as std::os::raw::c_ulong,
             b"%s%s%s\x00" as *const u8 as *const std::os::raw::c_char, out, base,
             suffixbuff.as_mut_ptr());
    res[499 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return strdup(res.as_mut_ptr());
}
unsafe extern "C" fn checkTestFile(mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut buf: stat =
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
    if stat(filename, &mut buf) == -(1 as std::os::raw::c_int) {
        return 0 as std::os::raw::c_int
    }
    if !(buf.st_mode & 0o170000 as std::os::raw::c_int as std::os::raw::c_uint ==
             0o100000 as std::os::raw::c_int as std::os::raw::c_uint) {
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *		Test to detect or not recursive entities		*
 *									*
 ************************************************************************/
/* *
 * recursiveDetectTest:
 * @filename: the file to parse
 * @result: the file with expected result
 * @err: the file with error messages: unused
 *
 * Parse a file loading DTD and replacing entities check it fails for
 * lol cases
 *
 * Returns 0 in case of success, an error code otherwise
 */
unsafe extern "C" fn recursiveDetectTest(mut filename: *const std::os::raw::c_char,
                                         mut result: *const std::os::raw::c_char,
                                         mut err: *const std::os::raw::c_char,
                                         mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut res: std::os::raw::c_int = 0 as std::os::raw::c_int;
    nb_tests += 1;
    ctxt = xmlNewParserCtxt();
    /*
     * base of the test, parse with the old API
     */
    doc =
        xmlCtxtReadFile(ctxt, filename, 0 as *const std::os::raw::c_char,
                        XML_PARSE_NOENT as std::os::raw::c_int |
                            XML_PARSE_DTDLOAD as std::os::raw::c_int);
    if !doc.is_null() ||
           (*ctxt).lastError.code != XML_ERR_ENTITY_LOOP as std::os::raw::c_int {
        fprintf(stderr,
                b"Failed to detect recursion in %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        xmlFreeParserCtxt(ctxt);
        xmlFreeDoc(doc);
        return 1 as std::os::raw::c_int
    }
    xmlFreeParserCtxt(ctxt);
    return res;
}
/* *
 * notRecursiveDetectTest:
 * @filename: the file to parse
 * @result: the file with expected result
 * @err: the file with error messages: unused
 *
 * Parse a file loading DTD and replacing entities check it works for
 * good cases
 *
 * Returns 0 in case of success, an error code otherwise
 */
unsafe extern "C" fn notRecursiveDetectTest(mut filename: *const std::os::raw::c_char,
                                            mut result: *const std::os::raw::c_char,
                                            mut err: *const std::os::raw::c_char,
                                            mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut res: std::os::raw::c_int = 0 as std::os::raw::c_int;
    nb_tests += 1;
    ctxt = xmlNewParserCtxt();
    /*
     * base of the test, parse with the old API
     */
    doc =
        xmlCtxtReadFile(ctxt, filename, 0 as *const std::os::raw::c_char,
                        XML_PARSE_NOENT as std::os::raw::c_int |
                            XML_PARSE_DTDLOAD as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Failed to parse correct file %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        xmlFreeParserCtxt(ctxt);
        return 1 as std::os::raw::c_int
    }
    xmlFreeDoc(doc);
    xmlFreeParserCtxt(ctxt);
    return res;
}
/* *
 * notRecursiveHugeTest:
 * @filename: the file to parse
 * @result: the file with expected result
 * @err: the file with error messages: unused
 *
 * Parse a memory generated file
 * good cases
 *
 * Returns 0 in case of success, an error code otherwise
 */
unsafe extern "C" fn notRecursiveHugeTest(mut filename: *const std::os::raw::c_char,
                                          mut result: *const std::os::raw::c_char,
                                          mut err: *const std::os::raw::c_char,
                                          mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut res: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 0;
    nb_tests += 1;
    reader =
        xmlReaderForFile(b"huge:test\x00" as *const u8 as *const std::os::raw::c_char,
                         0 as *const std::os::raw::c_char,
                         XML_PARSE_NOENT as std::os::raw::c_int |
                             XML_PARSE_DTDLOAD as std::os::raw::c_int);
    if reader.is_null() {
        fprintf(stderr,
                b"Failed to open huge:test\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return 1 as std::os::raw::c_int
    }
    ret = xmlTextReaderRead(reader);
    while ret == 1 as std::os::raw::c_int { ret = xmlTextReaderRead(reader) }
    if ret != 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"Failed to parser huge:test with entities\n\x00" as *const u8
                    as *const std::os::raw::c_char);
        res = 1 as std::os::raw::c_int
    }
    xmlFreeTextReader(reader);
    return res;
}
/* ***********************************************************************
 *									*
 *			Tests Descriptions				*
 *									*
 ************************************************************************/
static mut testDescriptions: [testDesc; 4] =
    unsafe {
        [{
             let mut init =
                 testDesc{desc:
                              b"Parsing recursive test cases\x00" as *const u8
                                  as *const std::os::raw::c_char,
                          func:
                              Some(recursiveDetectTest as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                          in_0:
                              b"./test/recurse/lol*.xml\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                          out: 0 as *const std::os::raw::c_char,
                          suffix: 0 as *const std::os::raw::c_char,
                          err: 0 as *const std::os::raw::c_char,
                          options: 0 as std::os::raw::c_int,};
             init
         },
         {
             let mut init =
                 testDesc{desc:
                              b"Parsing non-recursive test cases\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                          func:
                              Some(notRecursiveDetectTest as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                          in_0:
                              b"./test/recurse/good*.xml\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                          out: 0 as *const std::os::raw::c_char,
                          suffix: 0 as *const std::os::raw::c_char,
                          err: 0 as *const std::os::raw::c_char,
                          options: 0 as std::os::raw::c_int,};
             init
         },
         {
             let mut init =
                 testDesc{desc:
                              b"Parsing non-recursive huge case\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                          func:
                              Some(notRecursiveHugeTest as
                                       unsafe extern "C" fn(_:
                                                                *const std::os::raw::c_char,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _:
                                                                *const std::os::raw::c_char,
                                                            _: std::os::raw::c_int)
                                           -> std::os::raw::c_int),
                          in_0: 0 as *const std::os::raw::c_char,
                          out: 0 as *const std::os::raw::c_char,
                          suffix: 0 as *const std::os::raw::c_char,
                          err: 0 as *const std::os::raw::c_char,
                          options: 0 as std::os::raw::c_int,};
             init
         },
         {
             let mut init =
                 testDesc{desc: 0 as *const std::os::raw::c_char,
                          func: None,
                          in_0: 0 as *const std::os::raw::c_char,
                          out: 0 as *const std::os::raw::c_char,
                          suffix: 0 as *const std::os::raw::c_char,
                          err: 0 as *const std::os::raw::c_char,
                          options: 0 as std::os::raw::c_int,};
             init
         }]
    };
/* ***********************************************************************
 *									*
 *		The main code driving the tests				*
 *									*
 ************************************************************************/
unsafe extern "C" fn launchTests(mut tst: testDescPtr) -> std::os::raw::c_int {
    let mut res: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut err: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: size_t = 0;
    let mut result: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut error: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut mem: std::os::raw::c_int = 0;
    if tst.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*tst).in_0.is_null() {
        let mut globbuf: glob_t =
            glob_t{gl_pathc: 0,
                   gl_pathv: 0 as *mut *mut std::os::raw::c_char,
                   gl_offs: 0,
                   gl_flags: 0,
                   gl_closedir: None,
                   gl_readdir: None,
                   gl_opendir: None,
                   gl_lstat: None,
                   gl_stat: None,};
        globbuf.gl_offs = 0 as std::os::raw::c_int as __size_t;
        glob((*tst).in_0, (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int, None,
             &mut globbuf);
        i = 0 as std::os::raw::c_int as size_t;
        while i < globbuf.gl_pathc {
            if !(checkTestFile(*globbuf.gl_pathv.offset(i as isize)) == 0) {
                if !(*tst).suffix.is_null() {
                    result =
                        resultFilename(*globbuf.gl_pathv.offset(i as isize),
                                       (*tst).out, (*tst).suffix);
                    if result.is_null() {
                        fprintf(stderr,
                                b"Out of memory !\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                        fatalError();
                    }
                } else { result = 0 as *mut std::os::raw::c_char }
                if !(*tst).err.is_null() {
                    error =
                        resultFilename(*globbuf.gl_pathv.offset(i as isize),
                                       (*tst).out, (*tst).err);
                    if error.is_null() {
                        fprintf(stderr,
                                b"Out of memory !\n\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                        fatalError();
                    }
                } else { error = 0 as *mut std::os::raw::c_char }
                if !result.is_null() && checkTestFile(result) == 0 {
                    fprintf(stderr,
                            b"Missing result file %s\n\x00" as *const u8 as
                                *const std::os::raw::c_char, result);
                } else if !error.is_null() && checkTestFile(error) == 0 {
                    fprintf(stderr,
                            b"Missing error file %s\n\x00" as *const u8 as
                                *const std::os::raw::c_char, error);
                } else {
                    mem = xmlMemUsed();
                    extraMemoryFromResolver = 0 as std::os::raw::c_int;
                    testErrorsSize_recurse = 0 as std::os::raw::c_int;
                    testErrors_recurse[0 as std::os::raw::c_int as usize] =
                        0 as std::os::raw::c_int as std::os::raw::c_char;
                    res =
                        (*tst).func.expect("non-null function pointer")(*globbuf.gl_pathv.offset(i
                                                                                                     as
                                                                                                     isize),
                                                                        result,
                                                                        error,
                                                                        (*tst).options
                                                                            |
                                                                            XML_PARSE_COMPACT
                                                                                as
                                                                                std::os::raw::c_int);
                    xmlResetLastError();
                    if res != 0 as std::os::raw::c_int {
                        fprintf(stderr,
                                b"File %s generated an error\n\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                *globbuf.gl_pathv.offset(i as isize));
                        nb_errors += 1;
                        err += 1
                    } else if xmlMemUsed() != mem {
                        if xmlMemUsed() != mem &&
                               extraMemoryFromResolver == 0 as std::os::raw::c_int {
                            fprintf(stderr,
                                    b"File %s leaked %d bytes\n\x00" as
                                        *const u8 as *const std::os::raw::c_char,
                                    *globbuf.gl_pathv.offset(i as isize),
                                    xmlMemUsed() - mem);
                            nb_leaks += 1;
                            err += 1
                        }
                    }
                    testErrorsSize_recurse = 0 as std::os::raw::c_int
                }
                if !result.is_null() { free(result as *mut std::os::raw::c_void); }
                if !error.is_null() { free(error as *mut std::os::raw::c_void); }
            }
            i = i.wrapping_add(1)
        }
        globfree(&mut globbuf);
    } else {
        testErrorsSize_recurse = 0 as std::os::raw::c_int;
        testErrors_recurse[0 as std::os::raw::c_int as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        extraMemoryFromResolver = 0 as std::os::raw::c_int;
        res =
            (*tst).func.expect("non-null function pointer")(0 as
                                                                *const std::os::raw::c_char,
                                                            0 as
                                                                *const std::os::raw::c_char,
                                                            0 as
                                                                *const std::os::raw::c_char,
                                                            (*tst).options);
        if res != 0 as std::os::raw::c_int { nb_errors += 1; err += 1 }
    }
    return err;
}
static mut verbose: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut tests_quiet: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn runtest(mut i: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut res: std::os::raw::c_int = 0;
    let mut old_errors: std::os::raw::c_int = 0;
    let mut old_tests: std::os::raw::c_int = 0;
    let mut old_leaks: std::os::raw::c_int = 0;
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    if tests_quiet == 0 as std::os::raw::c_int &&
           !testDescriptions[i as usize].desc.is_null() {
        printf(b"## %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               testDescriptions[i as usize].desc);
    }
    res = launchTests(&mut *testDescriptions.as_mut_ptr().offset(i as isize));
    if res != 0 as std::os::raw::c_int { ret += 1 }
    if verbose != 0 {
        if nb_errors == old_errors && nb_leaks == old_leaks {
            printf(b"Ran %d tests, no errors\n\x00" as *const u8 as
                       *const std::os::raw::c_char, nb_tests - old_tests);
        } else {
            printf(b"Ran %d tests, %d errors, %d leaks\n\x00" as *const u8 as
                       *const std::os::raw::c_char, nb_tests - old_tests,
                   nb_errors - old_errors, nb_leaks - old_leaks);
        }
    }
    return ret;
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut a: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut subset: std::os::raw::c_int = 0 as std::os::raw::c_int;
    initializeLibxml2();
    a = 1 as std::os::raw::c_int;
    while a < argc {
        if strcmp(*argv.offset(a as isize),
                  b"-v\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
            verbose = 1 as std::os::raw::c_int
        } else if strcmp(*argv.offset(a as isize),
                         b"-quiet\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 {
            tests_quiet = 1 as std::os::raw::c_int
        } else {
            i = 0 as std::os::raw::c_int;
            while testDescriptions[i as usize].func.is_some() {
                if !strstr(testDescriptions[i as usize].desc,
                           *argv.offset(a as isize)).is_null() {
                    ret += runtest(i);
                    subset += 1
                }
                i += 1
            }
        }
        a += 1
    }
    if subset == 0 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int;
        while testDescriptions[i as usize].func.is_some() {
            ret += runtest(i);
            i += 1
        }
    }
    if nb_errors == 0 as std::os::raw::c_int && nb_leaks == 0 as std::os::raw::c_int {
        ret = 0 as std::os::raw::c_int;
        printf(b"Total %d tests, no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests);
    } else {
        ret = 1 as std::os::raw::c_int;
        printf(b"Total %d tests, %d errors, %d leaks\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests, nb_errors, nb_leaks);
    }
    xmlCleanupParser();
    xmlMemoryDump();
    return ret;
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
