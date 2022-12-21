
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    /*
 * Default message routines used by SAX and Valid context for error
 * and warning reporting.
 */
    #[no_mangle]
    fn xmlParserError(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                      _: ...);
    #[no_mangle]
    fn xmlParserWarning(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                        _: ...);
    /*
 * Summary: SAX2 parser interface used to build the DOM tree
 * Description: those are the default SAX2 interfaces used by
 *              the library when building DOM tree.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* LIBXML_SAX1_ENABLED or LIBXML_HTML_ENABLED or LIBXML_LEGACY_ENABLED */
    #[no_mangle]
    fn xmlSAX2ProcessingInstruction(ctx: *mut std::os::raw::c_void,
                                    target: *const xmlChar,
                                    data: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2Characters(ctx: *mut std::os::raw::c_void, ch: *const xmlChar,
                         len: std::os::raw::c_int);
    #[no_mangle]
    fn xmlSAX2CDataBlock(ctx: *mut std::os::raw::c_void, value: *const xmlChar,
                         len: std::os::raw::c_int);
    #[no_mangle]
    fn xmlSAX2Reference(ctx: *mut std::os::raw::c_void, name: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2EndElement(ctx: *mut std::os::raw::c_void, name: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2StartElement(ctx: *mut std::os::raw::c_void, fullname: *const xmlChar,
                           atts: *mut *const xmlChar);
    #[no_mangle]
    fn xmlSAX2EndDocument(ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSAX2StartDocument(ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSAX2SetDocumentLocator(ctx: *mut std::os::raw::c_void,
                                 loc: xmlSAXLocatorPtr);
    #[no_mangle]
    fn xmlSAX2UnparsedEntityDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                                 publicId: *const xmlChar,
                                 systemId: *const xmlChar,
                                 notationName: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2NotationDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                           publicId: *const xmlChar,
                           systemId: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2ElementDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                          type_0: std::os::raw::c_int, content: xmlElementContentPtr);
    #[no_mangle]
    fn xmlSAX2AttributeDecl(ctx: *mut std::os::raw::c_void, elem: *const xmlChar,
                            fullname: *const xmlChar, type_0: std::os::raw::c_int,
                            def: std::os::raw::c_int, defaultValue: *const xmlChar,
                            tree: xmlEnumerationPtr);
    #[no_mangle]
    fn xmlSAX2EntityDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                         type_0: std::os::raw::c_int, publicId: *const xmlChar,
                         systemId: *const xmlChar, content: *mut xmlChar);
    #[no_mangle]
    fn xmlSAX2GetParameterEntity(ctx: *mut std::os::raw::c_void, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlSAX2GetEntity(ctx: *mut std::os::raw::c_void, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlSAX2ResolveEntity(ctx: *mut std::os::raw::c_void, publicId: *const xmlChar,
                            systemId: *const xmlChar) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlSAX2HasExternalSubset(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2HasInternalSubset(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2IsStandalone(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2ExternalSubset(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                             ExternalID: *const xmlChar,
                             SystemID: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2InternalSubset(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                             ExternalID: *const xmlChar,
                             SystemID: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2Comment(ctx: *mut std::os::raw::c_void, value: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2IgnorableWhitespace(ctx: *mut std::os::raw::c_void, ch: *const xmlChar,
                                  len: std::os::raw::c_int);
}
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
 * SAX.c : Old SAX v1 handlers to build a tree.
 *         Deprecated except for compatibility
 *
 * See Copyright for the status of this software.
 *
 * Daniel Veillard <daniel@veillard.com>
 */
/* *
 * initxmlDefaultSAXHandler:
 * @hdlr:  the SAX handler
 * @warning:  flag if non-zero sets the handler warning procedure
 *
 * Initialize the default XML SAX version 1 handler
 * DEPRECATED: use xmlSAX2InitDefaultSAXHandler() for the new SAX2 blocks
 */
#[no_mangle]
pub unsafe extern "C" fn initxmlDefaultSAXHandler(mut hdlr:
                                                      *mut xmlSAXHandlerV1,
                                                  mut warning: std::os::raw::c_int) {
    if (*hdlr).initialized == 1 as std::os::raw::c_int as std::os::raw::c_uint { return }
    (*hdlr).internalSubset =
        Some(xmlSAX2InternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: *const xmlChar)
                     -> ());
    (*hdlr).externalSubset =
        Some(xmlSAX2ExternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: *const xmlChar)
                     -> ());
    (*hdlr).isStandalone =
        Some(xmlSAX2IsStandalone as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*hdlr).hasInternalSubset =
        Some(xmlSAX2HasInternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*hdlr).hasExternalSubset =
        Some(xmlSAX2HasExternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*hdlr).resolveEntity =
        Some(xmlSAX2ResolveEntity as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar)
                     -> xmlParserInputPtr);
    (*hdlr).getEntity =
        Some(xmlSAX2GetEntity as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> xmlEntityPtr);
    (*hdlr).getParameterEntity =
        Some(xmlSAX2GetParameterEntity as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> xmlEntityPtr);
    (*hdlr).entityDecl =
        Some(xmlSAX2EntityDecl as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int, _: *const xmlChar,
                                      _: *const xmlChar, _: *mut xmlChar)
                     -> ());
    (*hdlr).attributeDecl =
        Some(xmlSAX2AttributeDecl as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: std::os::raw::c_int,
                                      _: std::os::raw::c_int, _: *const xmlChar,
                                      _: xmlEnumerationPtr) -> ());
    (*hdlr).elementDecl =
        Some(xmlSAX2ElementDecl as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int, _: xmlElementContentPtr)
                     -> ());
    (*hdlr).notationDecl =
        Some(xmlSAX2NotationDecl as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: *const xmlChar)
                     -> ());
    (*hdlr).unparsedEntityDecl =
        Some(xmlSAX2UnparsedEntityDecl as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: *const xmlChar,
                                      _: *const xmlChar) -> ());
    (*hdlr).setDocumentLocator =
        Some(xmlSAX2SetDocumentLocator as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: xmlSAXLocatorPtr) -> ());
    (*hdlr).startDocument =
        Some(xmlSAX2StartDocument as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    (*hdlr).endDocument =
        Some(xmlSAX2EndDocument as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    (*hdlr).startElement =
        Some(xmlSAX2StartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    (*hdlr).endElement =
        Some(xmlSAX2EndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).reference =
        Some(xmlSAX2Reference as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).characters =
        Some(xmlSAX2Characters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).cdataBlock =
        Some(xmlSAX2CDataBlock as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).ignorableWhitespace =
        Some(xmlSAX2Characters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).processingInstruction =
        Some(xmlSAX2ProcessingInstruction as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar) -> ());
    if warning == 0 as std::os::raw::c_int {
        (*hdlr).warning = None
    } else {
        (*hdlr).warning =
            Some(xmlParserWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ())
    }
    (*hdlr).error =
        Some(xmlParserError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).fatalError =
        Some(xmlParserError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).initialized = 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
/* *
 * inithtmlDefaultSAXHandler:
 * @hdlr:  the SAX handler
 *
 * Initialize the default HTML SAX version 1 handler
 * DEPRECATED: use xmlSAX2InitHtmlDefaultSAXHandler() for the new SAX2 blocks
 */
#[no_mangle]
pub unsafe extern "C" fn inithtmlDefaultSAXHandler(mut hdlr:
                                                       *mut xmlSAXHandlerV1) {
    if (*hdlr).initialized == 1 as std::os::raw::c_int as std::os::raw::c_uint { return }
    (*hdlr).internalSubset =
        Some(xmlSAX2InternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: *const xmlChar)
                     -> ());
    (*hdlr).externalSubset = None;
    (*hdlr).isStandalone = None;
    (*hdlr).hasInternalSubset = None;
    (*hdlr).hasExternalSubset = None;
    (*hdlr).resolveEntity = None;
    (*hdlr).getEntity =
        Some(xmlSAX2GetEntity as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> xmlEntityPtr);
    (*hdlr).getParameterEntity = None;
    (*hdlr).entityDecl = None;
    (*hdlr).attributeDecl = None;
    (*hdlr).elementDecl = None;
    (*hdlr).notationDecl = None;
    (*hdlr).unparsedEntityDecl = None;
    (*hdlr).setDocumentLocator =
        Some(xmlSAX2SetDocumentLocator as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: xmlSAXLocatorPtr) -> ());
    (*hdlr).startDocument =
        Some(xmlSAX2StartDocument as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    (*hdlr).endDocument =
        Some(xmlSAX2EndDocument as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    (*hdlr).startElement =
        Some(xmlSAX2StartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    (*hdlr).endElement =
        Some(xmlSAX2EndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).reference = None;
    (*hdlr).characters =
        Some(xmlSAX2Characters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).cdataBlock =
        Some(xmlSAX2CDataBlock as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).ignorableWhitespace =
        Some(xmlSAX2IgnorableWhitespace as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).processingInstruction =
        Some(xmlSAX2ProcessingInstruction as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar) -> ());
    (*hdlr).comment =
        Some(xmlSAX2Comment as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).warning =
        Some(xmlParserWarning as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).error =
        Some(xmlParserError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).fatalError =
        Some(xmlParserError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).initialized = 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
/*
 * Summary: Old SAX version 1 handler, deprecated
 * Description: DEPRECATED set of SAX version 1 interfaces used to
 *              build the DOM tree.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* LIBXML_HTML_ENABLED */
/* *
 * initdocbDefaultSAXHandler:
 * @hdlr:  the SAX handler
 *
 * Initialize the default DocBook SAX version 1 handler
 * DEPRECATED: use xmlSAX2InitDocbDefaultSAXHandler() for the new SAX2 blocks
 */
#[no_mangle]
pub unsafe extern "C" fn initdocbDefaultSAXHandler(mut hdlr:
                                                       *mut xmlSAXHandlerV1) {
    if (*hdlr).initialized == 1 as std::os::raw::c_int as std::os::raw::c_uint { return }
    (*hdlr).internalSubset =
        Some(xmlSAX2InternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar, _: *const xmlChar)
                     -> ());
    (*hdlr).externalSubset = None;
    (*hdlr).isStandalone =
        Some(xmlSAX2IsStandalone as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*hdlr).hasInternalSubset =
        Some(xmlSAX2HasInternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*hdlr).hasExternalSubset =
        Some(xmlSAX2HasExternalSubset as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*hdlr).resolveEntity =
        Some(xmlSAX2ResolveEntity as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *const xmlChar)
                     -> xmlParserInputPtr);
    (*hdlr).getEntity =
        Some(xmlSAX2GetEntity as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> xmlEntityPtr);
    (*hdlr).getParameterEntity = None;
    (*hdlr).entityDecl =
        Some(xmlSAX2EntityDecl as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int, _: *const xmlChar,
                                      _: *const xmlChar, _: *mut xmlChar)
                     -> ());
    (*hdlr).attributeDecl = None;
    (*hdlr).elementDecl = None;
    (*hdlr).notationDecl = None;
    (*hdlr).unparsedEntityDecl = None;
    (*hdlr).setDocumentLocator =
        Some(xmlSAX2SetDocumentLocator as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: xmlSAXLocatorPtr) -> ());
    (*hdlr).startDocument =
        Some(xmlSAX2StartDocument as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    (*hdlr).endDocument =
        Some(xmlSAX2EndDocument as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    (*hdlr).startElement =
        Some(xmlSAX2StartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    (*hdlr).endElement =
        Some(xmlSAX2EndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).reference =
        Some(xmlSAX2Reference as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).characters =
        Some(xmlSAX2Characters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).cdataBlock = None;
    (*hdlr).ignorableWhitespace =
        Some(xmlSAX2IgnorableWhitespace as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*hdlr).processingInstruction = None;
    (*hdlr).comment =
        Some(xmlSAX2Comment as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    (*hdlr).warning =
        Some(xmlParserWarning as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).error =
        Some(xmlParserError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).fatalError =
        Some(xmlParserError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*hdlr).initialized = 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
/* LIBXML_LEGACY_ENABLED */
/* __INCLUDE_ELFGCCHACK */
/* LIBXML_SAX1_ENABLED */
/* LIBXML_DOCB_ENABLED */
