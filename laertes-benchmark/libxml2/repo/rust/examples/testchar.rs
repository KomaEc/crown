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
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlSetStructuredErrorFunc(ctx: *mut std::os::raw::c_void,
                                 handler: xmlStructuredErrorFunc);
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlParserInputBufferCreateStatic(mem: *const std::os::raw::c_char,
                                        size: std::os::raw::c_int,
                                        enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    #[no_mangle]
    fn xmlCleanupParser();
    /*
 * Parser contexts handling.
 */
    #[no_mangle]
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                     URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                     options: std::os::raw::c_int) -> xmlDocPtr;
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
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCurrentChar(ctxt: xmlParserCtxtPtr, len: *mut std::os::raw::c_int)
     -> std::os::raw::c_int;
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
pub type xmlChar = std::os::raw::c_uchar;
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
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
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
/* entities.h */
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
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
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
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed = 1547;
pub const XML_IO_WRITE: C2RustUnnamed = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed = 1539;
pub const XML_IO_EROFS: C2RustUnnamed = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed = 1536;
pub const XML_IO_EPERM: C2RustUnnamed = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed = 1517;
pub const XML_IO_EIO: C2RustUnnamed = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed = 1515;
pub const XML_IO_EINTR: C2RustUnnamed = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed = 1510;
pub const XML_IO_EDOM: C2RustUnnamed = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed = 1504;
pub const XML_IO_EBADF: C2RustUnnamed = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed = 1502;
pub const XML_IO_EACCES: C2RustUnnamed = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed = 1;
pub const XML_ERR_OK: C2RustUnnamed = 0;
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
/* *
 * Test the UTF-8 decoding routines
 *
 * author: Daniel Veillard
 * copy: see Copyright for the status of this software.
 */
#[no_mangle]
pub static mut lastError: std::os::raw::c_int = 0;
unsafe extern "C" fn errorHandler(mut unused: *mut std::os::raw::c_void,
                                  mut err: xmlErrorPtr) {
    if unused.is_null() && !err.is_null() && lastError == 0 as std::os::raw::c_int {
        lastError = (*err).code
    };
}
#[no_mangle]
pub static mut document1: [std::os::raw::c_char; 100] =
    unsafe {
        *::std::mem::transmute::<&[u8; 100],
                                 &mut [std::os::raw::c_char; 100]>(b"<doc>XXXX</doc>\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub static mut document2: [std::os::raw::c_char; 100] =
    unsafe {
        *::std::mem::transmute::<&[u8; 100],
                                 &mut [std::os::raw::c_char; 100]>(b"<doc foo=\'XXXX\'/>\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
    };
unsafe extern "C" fn testDocumentRangeByte1(mut ctxt: xmlParserCtxtPtr,
                                            mut document: *mut std::os::raw::c_char,
                                            mut len: std::os::raw::c_int,
                                            mut data: *mut std::os::raw::c_char,
                                            mut forbid1: std::os::raw::c_int,
                                            mut forbid2: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut res: xmlDocPtr = 0 as *mut xmlDoc;
    i = 0 as std::os::raw::c_int;
    while i <= 0xff as std::os::raw::c_int {
        lastError = 0 as std::os::raw::c_int;
        xmlCtxtReset(ctxt);
        *data.offset(0 as std::os::raw::c_int as isize) = i as std::os::raw::c_char;
        res =
            xmlReadMemory(document, len,
                          b"test\x00" as *const u8 as *const std::os::raw::c_char,
                          0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
        if i == forbid1 || i == forbid2 {
            if lastError == 0 as std::os::raw::c_int || !res.is_null() {
                fprintf(stderr,
                        b"Failed to detect invalid char for Byte 0x%02X: %c\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i, i);
            }
        } else if i == '<' as i32 || i == '&' as i32 {
            if lastError == 0 as std::os::raw::c_int || !res.is_null() {
                fprintf(stderr,
                        b"Failed to detect illegal char %c for Byte 0x%02X\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i, i);
            }
        } else if (i < 0x20 as std::os::raw::c_int || i >= 0x80 as std::os::raw::c_int) &&
                      i != 0x9 as std::os::raw::c_int && i != 0xa as std::os::raw::c_int &&
                      i != 0xd as std::os::raw::c_int {
            if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int &&
                   !res.is_null() {
                fprintf(stderr,
                        b"Failed to detect invalid char for Byte 0x%02X\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i);
            }
        } else if res.is_null() {
            fprintf(stderr,
                    b"Failed to parse valid char for Byte 0x%02X : %c\n\x00"
                        as *const u8 as *const std::os::raw::c_char, i, i);
        }
        if !res.is_null() { xmlFreeDoc(res); }
        i += 1
    };
}
unsafe extern "C" fn testDocumentRangeByte2(mut ctxt: xmlParserCtxtPtr,
                                            mut document: *mut std::os::raw::c_char,
                                            mut len: std::os::raw::c_int,
                                            mut data: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut res: xmlDocPtr = 0 as *mut xmlDoc;
    i = 0x80 as std::os::raw::c_int;
    while i <= 0xff as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j <= 0xff as std::os::raw::c_int {
            lastError = 0 as std::os::raw::c_int;
            xmlCtxtReset(ctxt);
            *data.offset(0 as std::os::raw::c_int as isize) = i as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = j as std::os::raw::c_char;
            res =
                xmlReadMemory(document, len,
                              b"test\x00" as *const u8 as *const std::os::raw::c_char,
                              0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
            /* if first bit of first char is set, then second bit must too */
            if i & 0x80 as std::os::raw::c_int != 0 &&
                   i & 0x40 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                if lastError == 0 as std::os::raw::c_int || !res.is_null() {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j);
                }
            } else if i & 0x80 as std::os::raw::c_int != 0 &&
                          j & 0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                if lastError == 0 as std::os::raw::c_int || !res.is_null() {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j);
                }
            } else if i & 0x80 as std::os::raw::c_int != 0 &&
                          i & 0x1e as std::os::raw::c_int == 0 as std::os::raw::c_int {
                if lastError == 0 as std::os::raw::c_int || !res.is_null() {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j);
                }
            } else if i & 0xe0 as std::os::raw::c_int == 0xe0 as std::os::raw::c_int {
                if lastError == 0 as std::os::raw::c_int || !res.is_null() {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x00\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j);
                }
            } else if lastError != 0 as std::os::raw::c_int || res.is_null() {
                fprintf(stderr,
                        b"Failed to parse document for Bytes 0x%02X 0x%02X\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i, j);
            }
            if !res.is_null() { xmlFreeDoc(res); }
            j += 1
        }
        i += 1
    };
}
/*
	 * if first bit of first char is set, then second char first
	 * bits must be 10
	 */
/*
	 * if using a 2 byte encoding then the value must be greater
	 * than 0x80, i.e. one of bits 5 to 1 of i must be set
	 */
/*
	 * if third bit of first char is set, then the sequence would need
	 * at least 3 bytes, but we give only 2 !
	 */
/*
	 * We should see no error in remaning cases
	 */
/* *
 * testDocumentRanges:
 *
 * Test the correct UTF8 character parsing in context of XML documents
 * Those are in-context injection tests checking the parser behaviour on
 * edge case values at different point in content, beginning and end of
 * CDATA in text or in attribute values.
 */
unsafe extern "C" fn testDocumentRanges() {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut data: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    /*
     * Set up a parsing context using the first document as
     * the current input source.
     */
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        fprintf(stderr,
                b"Failed to allocate parser context\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return
    }
    printf(b"testing 1 byte char in document: 1\x00" as *const u8 as
               *const std::os::raw::c_char);
    fflush(stdout);
    data =
        &mut *document1.as_mut_ptr().offset(5 as std::os::raw::c_int as isize) as
            *mut std::os::raw::c_char;
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 1 byte injection at beginning of area */
    testDocumentRangeByte1(ctxt,
                           &mut *document1.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document1.as_mut_ptr()) as std::os::raw::c_int,
                           data, -(1 as std::os::raw::c_int), -(1 as std::os::raw::c_int));
    printf(b" 2\x00" as *const u8 as *const std::os::raw::c_char);
    fflush(stdout);
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 1 byte injection at end of area */
    testDocumentRangeByte1(ctxt,
                           &mut *document1.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document1.as_mut_ptr()) as std::os::raw::c_int,
                           data.offset(3 as std::os::raw::c_int as isize),
                           -(1 as std::os::raw::c_int), -(1 as std::os::raw::c_int));
    printf(b" 3\x00" as *const u8 as *const std::os::raw::c_char);
    fflush(stdout);
    data =
        &mut *document2.as_mut_ptr().offset(10 as std::os::raw::c_int as isize) as
            *mut std::os::raw::c_char;
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 1 byte injection at beginning of area */
    testDocumentRangeByte1(ctxt,
                           &mut *document2.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document2.as_mut_ptr()) as std::os::raw::c_int,
                           data, '\'' as i32, -(1 as std::os::raw::c_int));
    printf(b" 4\x00" as *const u8 as *const std::os::raw::c_char);
    fflush(stdout);
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 1 byte injection at end of area */
    testDocumentRangeByte1(ctxt,
                           &mut *document2.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document2.as_mut_ptr()) as std::os::raw::c_int,
                           data.offset(3 as std::os::raw::c_int as isize),
                           '\'' as i32, -(1 as std::os::raw::c_int));
    printf(b" done\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"testing 2 byte char in document: 1\x00" as *const u8 as
               *const std::os::raw::c_char);
    fflush(stdout);
    data =
        &mut *document1.as_mut_ptr().offset(5 as std::os::raw::c_int as isize) as
            *mut std::os::raw::c_char;
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 2 byte injection at beginning of area */
    testDocumentRangeByte2(ctxt,
                           &mut *document1.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document1.as_mut_ptr()) as std::os::raw::c_int,
                           data);
    printf(b" 2\x00" as *const u8 as *const std::os::raw::c_char);
    fflush(stdout);
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 2 byte injection at end of area */
    testDocumentRangeByte2(ctxt,
                           &mut *document1.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document1.as_mut_ptr()) as std::os::raw::c_int,
                           data.offset(2 as std::os::raw::c_int as isize));
    printf(b" 3\x00" as *const u8 as *const std::os::raw::c_char);
    fflush(stdout);
    data =
        &mut *document2.as_mut_ptr().offset(10 as std::os::raw::c_int as isize) as
            *mut std::os::raw::c_char;
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 2 byte injection at beginning of area */
    testDocumentRangeByte2(ctxt,
                           &mut *document2.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document2.as_mut_ptr()) as std::os::raw::c_int,
                           data);
    printf(b" 4\x00" as *const u8 as *const std::os::raw::c_char);
    fflush(stdout);
    *data.offset(0 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(1 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) = ' ' as i32 as std::os::raw::c_char;
    /* test 2 byte injection at end of area */
    testDocumentRangeByte2(ctxt,
                           &mut *document2.as_mut_ptr().offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize),
                           strlen(document2.as_mut_ptr()) as std::os::raw::c_int,
                           data.offset(2 as std::os::raw::c_int as isize));
    printf(b" done\n\x00" as *const u8 as *const std::os::raw::c_char);
    xmlFreeParserCtxt(ctxt);
}
unsafe extern "C" fn testCharRangeByte1(mut ctxt: xmlParserCtxtPtr,
                                        mut data: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut len: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    *data.offset(1 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    *data.offset(2 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while i <= 0xff as std::os::raw::c_int {
        *data.offset(0 as std::os::raw::c_int as isize) = i as std::os::raw::c_char;
        (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
        lastError = 0 as std::os::raw::c_int;
        c = xmlCurrentChar(ctxt, &mut len);
        if i == 0 as std::os::raw::c_int || i >= 0x80 as std::os::raw::c_int {
            /* we must see an error there */
            if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                fprintf(stderr,
                        b"Failed to detect invalid char for Byte 0x%02X\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i);
            }
        } else if i == 0xd as std::os::raw::c_int {
            if c != 0xa as std::os::raw::c_int || len != 1 as std::os::raw::c_int {
                fprintf(stderr,
                        b"Failed to convert char for Byte 0x%02X\n\x00" as
                            *const u8 as *const std::os::raw::c_char, i);
            }
        } else if c != i || len != 1 as std::os::raw::c_int {
            fprintf(stderr,
                    b"Failed to parse char for Byte 0x%02X\n\x00" as *const u8
                        as *const std::os::raw::c_char, i);
        }
        i += 1
    };
}
unsafe extern "C" fn testCharRangeByte2(mut ctxt: xmlParserCtxtPtr,
                                        mut data: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    *data.offset(2 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    *data.offset(3 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0x80 as std::os::raw::c_int;
    while i <= 0xff as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j <= 0xff as std::os::raw::c_int {
            *data.offset(0 as std::os::raw::c_int as isize) = i as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = j as std::os::raw::c_char;
            (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
            lastError = 0 as std::os::raw::c_int;
            c = xmlCurrentChar(ctxt, &mut len);
            /* if first bit of first char is set, then second bit must too */
            if i & 0x80 as std::os::raw::c_int != 0 &&
                   i & 0x40 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j);
                }
            } else if i & 0x80 as std::os::raw::c_int != 0 &&
                          j & 0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X: %d\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j, c);
                }
            } else if i & 0x80 as std::os::raw::c_int != 0 &&
                          i & 0x1e as std::os::raw::c_int == 0 as std::os::raw::c_int {
                if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X: %d\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j, c);
                }
            } else if i & 0xe0 as std::os::raw::c_int == 0xe0 as std::os::raw::c_int {
                if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                    fprintf(stderr,
                            b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x00\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j);
                }
            } else if lastError != 0 as std::os::raw::c_int || len != 2 as std::os::raw::c_int
             {
                fprintf(stderr,
                        b"Failed to parse char for Bytes 0x%02X 0x%02X\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i, j);
            } else if c !=
                          (j & 0x3f as std::os::raw::c_int) +
                              ((i & 0x1f as std::os::raw::c_int) << 6 as std::os::raw::c_int)
             {
                fprintf(stderr,
                        b"Failed to parse char for Bytes 0x%02X 0x%02X: expect %d got %d\n\x00"
                            as *const u8 as *const std::os::raw::c_char, i, j,
                        (j & 0x3f as std::os::raw::c_int) +
                            ((i & 0x1f as std::os::raw::c_int) << 6 as std::os::raw::c_int),
                        c);
            }
            j += 1
        }
        i += 1
    };
}
unsafe extern "C" fn testCharRangeByte3(mut ctxt: xmlParserCtxtPtr,
                                        mut data: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut K: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut lows: [std::os::raw::c_uchar; 6] =
        [0 as std::os::raw::c_int as std::os::raw::c_uchar,
         0x80 as std::os::raw::c_int as std::os::raw::c_uchar,
         0x81 as std::os::raw::c_int as std::os::raw::c_uchar,
         0xc1 as std::os::raw::c_int as std::os::raw::c_uchar,
         0xff as std::os::raw::c_int as std::os::raw::c_uchar,
         0xbf as std::os::raw::c_int as std::os::raw::c_uchar];
    let mut value: std::os::raw::c_int = 0;
    *data.offset(3 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0xe0 as std::os::raw::c_int;
    while i <= 0xff as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j <= 0xff as std::os::raw::c_int {
            k = 0 as std::os::raw::c_int;
            while k < 6 as std::os::raw::c_int {
                *data.offset(0 as std::os::raw::c_int as isize) = i as std::os::raw::c_char;
                *data.offset(1 as std::os::raw::c_int as isize) = j as std::os::raw::c_char;
                K = lows[k as usize] as std::os::raw::c_int;
                *data.offset(2 as std::os::raw::c_int as isize) = K as std::os::raw::c_char;
                value =
                    (K & 0x3f as std::os::raw::c_int) +
                        ((j & 0x3f as std::os::raw::c_int) << 6 as std::os::raw::c_int) +
                        ((i & 0xf as std::os::raw::c_int) << 12 as std::os::raw::c_int);
                (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
                lastError = 0 as std::os::raw::c_int;
                c = xmlCurrentChar(ctxt, &mut len);
                /*
	     * if first bit of first char is set, then second char first
	     * bits must be 10
	     */
                /*
	     * if using a 2 byte encoding then the value must be greater
	     * than 0x80, i.e. one of bits 5 to 1 of i must be set
	     */
                /*
	     * if third bit of first char is set, then the sequence would need
	     * at least 3 bytes, but we give only 2 !
	     */
                /*
	     * We should see no error in remaning cases
	     */
                /*
	     * Finally check the value is right
	     */
                /*
	 * if fourth bit of first char is set, then the sequence would need
	 * at least 4 bytes, but we give only 3 !
	 */
                if i & 0xf0 as std::os::raw::c_int == 0xf0 as std::os::raw::c_int {
                    if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                        fprintf(stderr,
                                b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x%02X 0x%02X\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, i, j,
                                K,
                                *data.offset(3 as std::os::raw::c_int as isize) as
                                    std::os::raw::c_int);
                    }
                } else if j & 0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int ||
                              K & 0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                    if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                        fprintf(stderr,
                                b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x%02X\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, i, j,
                                K);
                    }
                } else if i & 0xf as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                              j & 0x20 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                        fprintf(stderr,
                                b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x%02X\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, i, j,
                                K);
                    }
                } else if value > 0xd7ff as std::os::raw::c_int &&
                              value < 0xe000 as std::os::raw::c_int ||
                              value > 0xfffd as std::os::raw::c_int &&
                                  value < 0x10000 as std::os::raw::c_int {
                    if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                        fprintf(stderr,
                                b"Failed to detect invalid char 0x%04X for Bytes 0x%02X 0x%02X 0x%02X\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, i, j, K);
                    }
                } else if lastError != 0 as std::os::raw::c_int ||
                              len != 3 as std::os::raw::c_int {
                    fprintf(stderr,
                            b"Failed to parse char for Bytes 0x%02X 0x%02X 0x%02X\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j, K);
                } else if c != value {
                    fprintf(stderr,
                            b"Failed to parse char for Bytes 0x%02X 0x%02X 0x%02X: expect %d got %d\n\x00"
                                as *const u8 as *const std::os::raw::c_char, i, j,
                            *data.offset(2 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int, value, c);
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}
unsafe extern "C" fn testCharRangeByte4(mut ctxt: xmlParserCtxtPtr,
                                        mut data: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut K: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    let mut L: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut lows: [std::os::raw::c_uchar; 6] =
        [0 as std::os::raw::c_int as std::os::raw::c_uchar,
         0x80 as std::os::raw::c_int as std::os::raw::c_uchar,
         0x81 as std::os::raw::c_int as std::os::raw::c_uchar,
         0xc1 as std::os::raw::c_int as std::os::raw::c_uchar,
         0xff as std::os::raw::c_int as std::os::raw::c_uchar,
         0xbf as std::os::raw::c_int as std::os::raw::c_uchar];
    let mut value: std::os::raw::c_int = 0;
    *data.offset(4 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0xf0 as std::os::raw::c_int;
    while i <= 0xff as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j <= 0xff as std::os::raw::c_int {
            k = 0 as std::os::raw::c_int;
            while k < 6 as std::os::raw::c_int {
                l = 0 as std::os::raw::c_int;
                while l < 6 as std::os::raw::c_int {
                    *data.offset(0 as std::os::raw::c_int as isize) =
                        i as std::os::raw::c_char;
                    *data.offset(1 as std::os::raw::c_int as isize) =
                        j as std::os::raw::c_char;
                    K = lows[k as usize] as std::os::raw::c_int;
                    *data.offset(2 as std::os::raw::c_int as isize) =
                        K as std::os::raw::c_char;
                    L = lows[l as usize] as std::os::raw::c_int;
                    *data.offset(3 as std::os::raw::c_int as isize) =
                        L as std::os::raw::c_char;
                    value =
                        (L & 0x3f as std::os::raw::c_int) +
                            ((K & 0x3f as std::os::raw::c_int) << 6 as std::os::raw::c_int) +
                            ((j & 0x3f as std::os::raw::c_int) << 12 as std::os::raw::c_int) +
                            ((i & 0x7 as std::os::raw::c_int) << 18 as std::os::raw::c_int);
                    (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
                    lastError = 0 as std::os::raw::c_int;
                    c = xmlCurrentChar(ctxt, &mut len);
                    /*
	 * The second and the third bytes must start with 10
	 */
                    /*
	 * if using a 3 byte encoding then the value must be greater
	 * than 0x800, i.e. one of bits 4 to 0 of i must be set or
	 * the 6th byte of data[1] must be set
	 */
                    /*
	 * There are values in that range that are not allowed in XML-1.0
	 */
                    /*
	 * We should see no error in remaining cases
	 */
                    /*
	 * Finally check the value is right
	 */
                    /*
	 * if fifth bit of first char is set, then the sequence would need
	 * at least 5 bytes, but we give only 4 !
	 */
                    if i & 0xf8 as std::os::raw::c_int == 0xf8 as std::os::raw::c_int {
                        if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                            fprintf(stderr,
                                    b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x%02X 0x%02X\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    i, j, K,
                                    *data.offset(3 as std::os::raw::c_int as isize) as
                                        std::os::raw::c_int);
                        }
                    } else if j & 0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int
                                  ||
                                  K & 0xc0 as std::os::raw::c_int !=
                                      0x80 as std::os::raw::c_int ||
                                  L & 0xc0 as std::os::raw::c_int !=
                                      0x80 as std::os::raw::c_int {
                        if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                            fprintf(stderr,
                                    b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x%02X 0x%02X\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    i, j, K, L);
                        }
                    } else if i & 0x7 as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                                  j & 0x30 as std::os::raw::c_int == 0 as std::os::raw::c_int
                     {
                        if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                            fprintf(stderr,
                                    b"Failed to detect invalid char for Bytes 0x%02X 0x%02X 0x%02X 0x%02X\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    i, j, K, L);
                        }
                    } else if value > 0xd7ff as std::os::raw::c_int &&
                                  value < 0xe000 as std::os::raw::c_int ||
                                  value > 0xfffd as std::os::raw::c_int &&
                                      value < 0x10000 as std::os::raw::c_int ||
                                  value > 0x10ffff as std::os::raw::c_int {
                        if lastError != XML_ERR_INVALID_CHAR as std::os::raw::c_int {
                            fprintf(stderr,
                                    b"Failed to detect invalid char 0x%04X for Bytes 0x%02X 0x%02X 0x%02X 0x%02X\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    value, i, j, K, L);
                        }
                    } else if lastError != 0 as std::os::raw::c_int ||
                                  len != 4 as std::os::raw::c_int {
                        fprintf(stderr,
                                b"Failed to parse char for Bytes 0x%02X 0x%02X 0x%02X\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, i, j,
                                K);
                    } else if c != value {
                        fprintf(stderr,
                                b"Failed to parse char for Bytes 0x%02X 0x%02X 0x%02X: expect %d got %d\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, i, j,
                                *data.offset(2 as std::os::raw::c_int as isize) as
                                    std::os::raw::c_int, value, c);
                    }
                    l += 1
                }
                k += 1
            }
            j += 1
        }
        i += 1
    };
}
/*
	 * The second, third and fourth bytes must start with 10
	 */
/*
	 * if using a 3 byte encoding then the value must be greater
	 * than 0x10000, i.e. one of bits 3 to 0 of i must be set or
	 * the 6 or 5th byte of j must be set
	 */
/*
	 * There are values in that range that are not allowed in XML-1.0
	 */
/*
	 * We should see no error in remaining cases
	 */
/*
	 * Finally check the value is right
	 */
/* *
 * testCharRanges:
 *
 * Test the correct UTF8 character parsing in isolation i.e.
 * not when parsing a full document, this is less expensive and we can
 * cover the full range of UTF-8 chars accepted by XML-1.0
 */
unsafe extern "C" fn testCharRanges() {
    let mut data: [std::os::raw::c_char; 5] = [0; 5];
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    memset(data.as_mut_ptr() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           5 as std::os::raw::c_int as std::os::raw::c_ulong);
    /*
     * Set up a parsing context using the above data buffer as
     * the current input source.
     */
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        fprintf(stderr,
                b"Failed to allocate parser context\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return
    }
    buf =
        xmlParserInputBufferCreateStatic(data.as_mut_ptr(),
                                         ::std::mem::size_of::<[std::os::raw::c_char; 5]>()
                                             as std::os::raw::c_ulong as std::os::raw::c_int,
                                         XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        fprintf(stderr,
                b"Failed to allocate input buffer\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    } else {
        input = xmlNewInputStream(ctxt);
        if input.is_null() {
            xmlFreeParserInputBuffer(buf);
        } else {
            (*input).filename = 0 as *const std::os::raw::c_char;
            (*input).buf = buf;
            (*input).base =
                xmlBufContent((*(*input).buf).buffer as *const xmlBuf);
            (*input).cur = (*input).base;
            (*input).end = (*input).base.offset(4 as std::os::raw::c_int as isize);
            inputPush(ctxt, input);
            printf(b"testing char range: 1\x00" as *const u8 as
                       *const std::os::raw::c_char);
            fflush(stdout);
            testCharRangeByte1(ctxt, data.as_mut_ptr());
            printf(b" 2\x00" as *const u8 as *const std::os::raw::c_char);
            fflush(stdout);
            testCharRangeByte2(ctxt, data.as_mut_ptr());
            printf(b" 3\x00" as *const u8 as *const std::os::raw::c_char);
            fflush(stdout);
            testCharRangeByte3(ctxt, data.as_mut_ptr());
            printf(b" 4\x00" as *const u8 as *const std::os::raw::c_char);
            fflush(stdout);
            testCharRangeByte4(ctxt, data.as_mut_ptr());
            printf(b" done\n\x00" as *const u8 as *const std::os::raw::c_char);
            fflush(stdout);
        }
    }
    xmlFreeParserCtxt(ctxt);
}
unsafe fn main_0() -> std::os::raw::c_int {
    /*
     * this initialize the library and check potential ABI mismatches
     * between the version it was compiled for and the actual shared
     * library used.
     */
    xmlCheckVersion(20908 as std::os::raw::c_int);
    /*
     * Catch errors separately
     */
    xmlSetStructuredErrorFunc(0 as *mut std::os::raw::c_void,
                              Some(errorHandler as
                                       unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _: xmlErrorPtr)
                                           -> ()));
    /*
     * Run the tests
     */
    testCharRanges();
    testDocumentRanges();
    /*
     * Cleanup function for the XML library.
     */
    xmlCleanupParser();
    /*
     * this is to debug memory for regression tests
     */
    xmlMemoryDump();
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
