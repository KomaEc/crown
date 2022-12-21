
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlMutex;
    #[no_mangle]
    fn __xmlGlobalInitMutexDestroy();
    #[no_mangle]
    static mut __xmlRegisterCallbacks: std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlCharStrdup(cur: *const std::os::raw::c_char) -> *mut xmlChar;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
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
    #[no_mangle]
    fn __xmlParserInputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                            enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn __xmlOutputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                       encoder: xmlCharEncodingHandlerPtr,
                                       compression: std::os::raw::c_int)
     -> xmlOutputBufferPtr;
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
    /*
 * Library wide APIs.
 */
    #[no_mangle]
    fn xmlGetGlobalState() -> xmlGlobalStatePtr;
    #[no_mangle]
    fn xmlIsMainThread() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    #[no_mangle]
    fn xmlMutexLock(tok: xmlMutexPtr);
    #[no_mangle]
    fn xmlFreeMutex(tok: xmlMutexPtr);
    #[no_mangle]
    fn xmlNewMutex() -> xmlMutexPtr;
    #[no_mangle]
    fn xmlSAX2CDataBlock(ctx: *mut std::os::raw::c_void, value: *const xmlChar,
                         len: std::os::raw::c_int);
    #[no_mangle]
    fn initxmlDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1,
                                warning: std::os::raw::c_int);
    #[no_mangle]
    fn inithtmlDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1);
    #[no_mangle]
    fn initdocbDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1);
    #[no_mangle]
    fn xmlSAX2GetPublicId(ctx: *mut std::os::raw::c_void) -> *const xmlChar;
    #[no_mangle]
    fn xmlSAX2GetSystemId(ctx: *mut std::os::raw::c_void) -> *const xmlChar;
    #[no_mangle]
    fn xmlSAX2SetDocumentLocator(ctx: *mut std::os::raw::c_void,
                                 loc: xmlSAXLocatorPtr);
    #[no_mangle]
    fn xmlSAX2GetLineNumber(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2GetColumnNumber(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2IsStandalone(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2HasInternalSubset(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2HasExternalSubset(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSAX2InternalSubset(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                             ExternalID: *const xmlChar,
                             SystemID: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2ExternalSubset(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                             ExternalID: *const xmlChar,
                             SystemID: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2GetEntity(ctx: *mut std::os::raw::c_void, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlSAX2GetParameterEntity(ctx: *mut std::os::raw::c_void, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlSAX2ResolveEntity(ctx: *mut std::os::raw::c_void, publicId: *const xmlChar,
                            systemId: *const xmlChar) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlSAX2EntityDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                         type_0: std::os::raw::c_int, publicId: *const xmlChar,
                         systemId: *const xmlChar, content: *mut xmlChar);
    #[no_mangle]
    fn xmlSAX2AttributeDecl(ctx: *mut std::os::raw::c_void, elem: *const xmlChar,
                            fullname: *const xmlChar, type_0: std::os::raw::c_int,
                            def: std::os::raw::c_int, defaultValue: *const xmlChar,
                            tree: xmlEnumerationPtr);
    #[no_mangle]
    fn xmlSAX2ElementDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                          type_0: std::os::raw::c_int, content: xmlElementContentPtr);
    #[no_mangle]
    fn xmlSAX2NotationDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                           publicId: *const xmlChar,
                           systemId: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2UnparsedEntityDecl(ctx: *mut std::os::raw::c_void, name: *const xmlChar,
                                 publicId: *const xmlChar,
                                 systemId: *const xmlChar,
                                 notationName: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2StartDocument(ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSAX2EndDocument(ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSAX2StartElement(ctx: *mut std::os::raw::c_void, fullname: *const xmlChar,
                           atts: *mut *const xmlChar);
    #[no_mangle]
    fn xmlSAX2EndElement(ctx: *mut std::os::raw::c_void, name: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2Reference(ctx: *mut std::os::raw::c_void, name: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2Characters(ctx: *mut std::os::raw::c_void, ch: *const xmlChar,
                         len: std::os::raw::c_int);
    #[no_mangle]
    fn xmlSAX2IgnorableWhitespace(ctx: *mut std::os::raw::c_void, ch: *const xmlChar,
                                  len: std::os::raw::c_int);
    #[no_mangle]
    fn xmlSAX2ProcessingInstruction(ctx: *mut std::os::raw::c_void,
                                    target: *const xmlChar,
                                    data: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2Comment(ctx: *mut std::os::raw::c_void, value: *const xmlChar);
    /*
 * Error handling
 */
    /* xmlGenericErrorFunc xmlGenericError = xmlGenericErrorDefaultFunc; */
/* Must initialize xmlGenericError in xmlInitParser */
    #[no_mangle]
    fn xmlGenericErrorDefaultFunc(ctx: *mut std::os::raw::c_void,
                                  msg: *const std::os::raw::c_char, _: ...);
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
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
pub type xmlStrdupFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char>;
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlMutex = _xmlMutex;
pub type xmlParserInputBufferCreateFilenameFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char, _: xmlCharEncoding)
               -> xmlParserInputBufferPtr>;
pub type xmlOutputBufferCreateFilenameFunc
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                _: xmlCharEncodingHandlerPtr, _: std::os::raw::c_int)
               -> xmlOutputBufferPtr>;
pub type xmlRegisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
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
/*
 * Mutex to protect "ForNewThreads" variables
 */
static mut xmlThrDefMutex: xmlMutexPtr = 0 as *const xmlMutex as xmlMutexPtr;
/* *
 * xmlInitGlobals:
 *
 * Additional initialisation for multi-threading
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitGlobals() {
    if xmlThrDefMutex.is_null() { xmlThrDefMutex = xmlNewMutex() };
}
/* *
 * xmlCleanupGlobals:
 *
 * Additional cleanup for multi-threading
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupGlobals() {
    if !xmlThrDefMutex.is_null() {
        xmlFreeMutex(xmlThrDefMutex);
        xmlThrDefMutex = 0 as xmlMutexPtr
    }
    __xmlGlobalInitMutexDestroy();
}
/* ***********************************************************************
 *									*
 *	All the user accessible global variables of the library		*
 *									*
 ************************************************************************/
/*
 * Memory allocation routines
 */
/* *
 * xmlFree:
 * @mem: an already allocated block of memory
 *
 * The variable holding the libxml free() implementation
 */
#[no_mangle]
pub static mut xmlFree: xmlFreeFunc =
    unsafe { Some(free as unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()) };
/* *
 * xmlMalloc:
 * @size:  the size requested in bytes
 *
 * The variable holding the libxml malloc() implementation
 *
 * Returns a pointer to the newly allocated block or NULL in case of error
 */
#[no_mangle]
pub static mut xmlMalloc: xmlMallocFunc =
    unsafe {
        Some(malloc as
                 unsafe extern "C" fn(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void)
    };
/* *
 * xmlMallocAtomic:
 * @size:  the size requested in bytes
 *
 * The variable holding the libxml malloc() implementation for atomic
 * data (i.e. blocks not containings pointers), useful when using a
 * garbage collecting allocator.
 *
 * Returns a pointer to the newly allocated block or NULL in case of error
 */
#[no_mangle]
pub static mut xmlMallocAtomic: xmlMallocFunc =
    unsafe {
        Some(malloc as
                 unsafe extern "C" fn(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void)
    };
/* *
 * xmlRealloc:
 * @mem: an already allocated block of memory
 * @size:  the new size requested in bytes
 *
 * The variable holding the libxml realloc() implementation
 *
 * Returns a pointer to the newly reallocated block or NULL in case of error
 */
#[no_mangle]
pub static mut xmlRealloc: xmlReallocFunc =
    unsafe {
        Some(realloc as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong)
                     -> *mut std::os::raw::c_void)
    };
/* *
 * xmlPosixStrdup
 * @cur:  the input char *
 *
 * a strdup implementation with a type signature matching POSIX
 *
 * Returns a new xmlChar * or NULL
 */
unsafe extern "C" fn xmlPosixStrdup(mut cur: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    return xmlCharStrdup(cur) as *mut std::os::raw::c_char;
}
/* *
 * xmlMemStrdup:
 * @str: a zero terminated string
 *
 * The variable holding the libxml strdup() implementation
 *
 * Returns the copy of the string or NULL in case of error
 */
#[no_mangle]
pub static mut xmlMemStrdup: xmlStrdupFunc =
    unsafe {
        Some(xmlPosixStrdup as
                 unsafe extern "C" fn(_: *const std::os::raw::c_char)
                     -> *mut std::os::raw::c_char)
    };
/* DEBUG_MEMORY_LOCATION || DEBUG_MEMORY */
/* *
 * xmlParserVersion:
 *
 * Constant string describing the internal version of the library
 */
#[no_mangle]
pub static mut xmlParserVersion: *const std::os::raw::c_char =
    b"20908\x00" as *const u8 as *const std::os::raw::c_char;
/* *
 * xmlBufferAllocScheme:
 *
 * Global setting, default allocation policy for buffers, default is
 * XML_BUFFER_ALLOC_EXACT
 */
#[no_mangle]
pub static mut xmlBufferAllocScheme: xmlBufferAllocationScheme =
    XML_BUFFER_ALLOC_EXACT;
static mut xmlBufferAllocSchemeThrDef: xmlBufferAllocationScheme =
    XML_BUFFER_ALLOC_EXACT;
/* *
 * xmlDefaultBufferSize:
 *
 * Global setting, default buffer size. Default value is BASE_BUFFER_SIZE
 */
#[no_mangle]
pub static mut xmlDefaultBufferSize: std::os::raw::c_int = 4096 as std::os::raw::c_int;
static mut xmlDefaultBufferSizeThrDef: std::os::raw::c_int = 4096 as std::os::raw::c_int;
/*
 * Parser defaults
 */
/* *
 * oldXMLWDcompatibility:
 *
 * Global setting, DEPRECATED.
 */
#[no_mangle]
pub static mut oldXMLWDcompatibility: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* DEPRECATED */
/* *
 * xmlParserDebugEntities:
 *
 * Global setting, asking the parser to print out debugging informations.
 * while handling entities.
 * Disabled by default
 */
#[no_mangle]
pub static mut xmlParserDebugEntities: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlParserDebugEntitiesThrDef: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* *
 * xmlDoValidityCheckingDefaultValue:
 *
 * Global setting, indicate that the parser should work in validating mode.
 * Disabled by default.
 */
#[no_mangle]
pub static mut xmlDoValidityCheckingDefaultValue: std::os::raw::c_int =
    0 as std::os::raw::c_int;
static mut xmlDoValidityCheckingDefaultValueThrDef: std::os::raw::c_int =
    0 as std::os::raw::c_int;
/* *
 * xmlGetWarningsDefaultValue:
 *
 * Global setting, indicate that the parser should provide warnings.
 * Activated by default.
 */
#[no_mangle]
pub static mut xmlGetWarningsDefaultValue: std::os::raw::c_int = 1 as std::os::raw::c_int;
static mut xmlGetWarningsDefaultValueThrDef: std::os::raw::c_int = 1 as std::os::raw::c_int;
/* *
 * xmlLoadExtDtdDefaultValue:
 *
 * Global setting, indicate that the parser should load DTD while not
 * validating.
 * Disabled by default.
 */
#[no_mangle]
pub static mut xmlLoadExtDtdDefaultValue: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlLoadExtDtdDefaultValueThrDef: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* *
 * xmlPedanticParserDefaultValue:
 *
 * Global setting, indicate that the parser be pedantic
 * Disabled by default.
 */
#[no_mangle]
pub static mut xmlPedanticParserDefaultValue: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlPedanticParserDefaultValueThrDef: std::os::raw::c_int =
    0 as std::os::raw::c_int;
/* *
 * xmlLineNumbersDefaultValue:
 *
 * Global setting, indicate that the parser should store the line number
 * in the content field of elements in the DOM tree.
 * Disabled by default since this may not be safe for old classes of
 * applicaton.
 */
#[no_mangle]
pub static mut xmlLineNumbersDefaultValue: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlLineNumbersDefaultValueThrDef: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* *
 * xmlKeepBlanksDefaultValue:
 *
 * Global setting, indicate that the parser should keep all blanks
 * nodes found in the content
 * Activated by default, this is actually needed to have the parser
 * conformant to the XML Recommendation, however the option is kept
 * for some applications since this was libxml1 default behaviour.
 */
#[no_mangle]
pub static mut xmlKeepBlanksDefaultValue: std::os::raw::c_int = 1 as std::os::raw::c_int;
static mut xmlKeepBlanksDefaultValueThrDef: std::os::raw::c_int = 1 as std::os::raw::c_int;
/* *
 * xmlSubstituteEntitiesDefaultValue:
 *
 * Global setting, indicate that the parser should not generate entity
 * references but replace them with the actual content of the entity
 * Disabled by default, this should be activated when using XPath since
 * the XPath data model requires entities replacement and the XPath
 * engine does not handle entities references transparently.
 */
#[no_mangle]
pub static mut xmlSubstituteEntitiesDefaultValue: std::os::raw::c_int =
    0 as std::os::raw::c_int;
static mut xmlSubstituteEntitiesDefaultValueThrDef: std::os::raw::c_int =
    0 as std::os::raw::c_int;
#[no_mangle]
pub static mut xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc = None;
static mut xmlRegisterNodeDefaultValueThrDef: xmlRegisterNodeFunc = None;
#[no_mangle]
pub static mut xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc = None;
static mut xmlDeregisterNodeDefaultValueThrDef: xmlDeregisterNodeFunc = None;
#[no_mangle]
pub static mut xmlParserInputBufferCreateFilenameValue:
           xmlParserInputBufferCreateFilenameFunc =
    None;
static mut xmlParserInputBufferCreateFilenameValueThrDef:
       xmlParserInputBufferCreateFilenameFunc =
    None;
#[no_mangle]
pub static mut xmlOutputBufferCreateFilenameValue:
           xmlOutputBufferCreateFilenameFunc =
    None;
static mut xmlOutputBufferCreateFilenameValueThrDef:
       xmlOutputBufferCreateFilenameFunc =
    None;
/* *
 * xmlGenericError:
 *
 * Global setting: function used for generic error callbacks
 */
#[no_mangle]
pub static mut xmlGenericError: xmlGenericErrorFunc =
    unsafe {
        Some(xmlGenericErrorDefaultFunc as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ())
    };
static mut xmlGenericErrorThrDef: xmlGenericErrorFunc =
    unsafe {
        Some(xmlGenericErrorDefaultFunc as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ())
    };
/* *
 * xmlStructuredError:
 *
 * Global setting: function used for structured error callbacks
 */
#[no_mangle]
pub static mut xmlStructuredError: xmlStructuredErrorFunc = None;
static mut xmlStructuredErrorThrDef: xmlStructuredErrorFunc = None;
/* *
 * xmlGenericErrorContext:
 *
 * Global setting passed to generic error callbacks
 */
#[no_mangle]
pub static mut xmlGenericErrorContext: *mut std::os::raw::c_void =
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void;
static mut xmlGenericErrorContextThrDef: *mut std::os::raw::c_void =
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void;
/* *
 * xmlStructuredErrorContext:
 *
 * Global setting passed to structured error callbacks
 */
#[no_mangle]
pub static mut xmlStructuredErrorContext: *mut std::os::raw::c_void =
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void;
static mut xmlStructuredErrorContextThrDef: *mut std::os::raw::c_void =
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void;
#[no_mangle]
pub static mut xmlLastError: xmlError =
    xmlError{domain: 0,
             code: 0,
             message: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
             level: XML_ERR_NONE,
             file: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
             line: 0,
             str1: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
             str2: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
             str3: 0 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
             int1: 0,
             int2: 0,
             ctxt: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
             node: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,};
/*
 * output defaults
 */
/* *
 * xmlIndentTreeOutput:
 *
 * Global setting, asking the serializer to indent the output tree by default
 * Enabled by default
 */
#[no_mangle]
pub static mut xmlIndentTreeOutput: std::os::raw::c_int = 1 as std::os::raw::c_int;
static mut xmlIndentTreeOutputThrDef: std::os::raw::c_int = 1 as std::os::raw::c_int;
/* *
 * xmlTreeIndentString:
 *
 * The string used to do one-level indent. By default is equal to "  " (two spaces)
 */
#[no_mangle]
pub static mut xmlTreeIndentString: *const std::os::raw::c_char =
    b"  \x00" as *const u8 as *const std::os::raw::c_char;
static mut xmlTreeIndentStringThrDef: *const std::os::raw::c_char =
    b"  \x00" as *const u8 as *const std::os::raw::c_char;
/* *
 * xmlSaveNoEmptyTags:
 *
 * Global setting, asking the serializer to not output empty tags
 * as <empty/> but <empty></empty>. those two forms are undistinguishable
 * once parsed.
 * Disabled by default
 */
#[no_mangle]
pub static mut xmlSaveNoEmptyTags: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlSaveNoEmptyTagsThrDef: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* *
 * xmlDefaultSAXHandler:
 *
 * Default SAX version1 handler for XML, builds the DOM tree
 */
#[no_mangle]
pub static mut xmlDefaultSAXHandler: xmlSAXHandlerV1 =
    unsafe {
        {
            let mut init =
                _xmlSAXHandlerV1{internalSubset:
                                     Some(xmlSAX2InternalSubset as
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
                                     Some(xmlSAX2IsStandalone as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> std::os::raw::c_int),
                                 hasInternalSubset:
                                     Some(xmlSAX2HasInternalSubset as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> std::os::raw::c_int),
                                 hasExternalSubset:
                                     Some(xmlSAX2HasExternalSubset as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> std::os::raw::c_int),
                                 resolveEntity:
                                     Some(xmlSAX2ResolveEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlParserInputPtr),
                                 getEntity:
                                     Some(xmlSAX2GetEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlEntityPtr),
                                 entityDecl:
                                     Some(xmlSAX2EntityDecl as
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
                                     Some(xmlSAX2NotationDecl as
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
                                     Some(xmlSAX2AttributeDecl as
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
                                     Some(xmlSAX2ElementDecl as
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
                                     Some(xmlSAX2UnparsedEntityDecl as
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
                                     Some(xmlSAX2SetDocumentLocator as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       xmlSAXLocatorPtr)
                                                  -> ()),
                                 startDocument:
                                     Some(xmlSAX2StartDocument as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()),
                                 endDocument:
                                     Some(xmlSAX2EndDocument as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()),
                                 startElement:
                                     Some(xmlSAX2StartElement as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *mut *const xmlChar)
                                                  -> ()),
                                 endElement:
                                     Some(xmlSAX2EndElement as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 reference:
                                     Some(xmlSAX2Reference as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 characters:
                                     Some(xmlSAX2Characters as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 ignorableWhitespace:
                                     Some(xmlSAX2Characters as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 processingInstruction:
                                     Some(xmlSAX2ProcessingInstruction as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 comment:
                                     Some(xmlSAX2Comment as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 warning:
                                     Some(xmlParserWarning as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 error:
                                     Some(xmlParserError as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 fatalError:
                                     Some(xmlParserError as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 getParameterEntity:
                                     Some(xmlSAX2GetParameterEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlEntityPtr),
                                 cdataBlock:
                                     Some(xmlSAX2CDataBlock as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 externalSubset:
                                     Some(xmlSAX2ExternalSubset as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 initialized:
                                     0 as std::os::raw::c_int as std::os::raw::c_uint,};
            init
        }
    };
/* LIBXML_SAX1_ENABLED */
/* *
 * xmlDefaultSAXLocator:
 *
 * The default SAX Locator
 * { getPublicId, getSystemId, getLineNumber, getColumnNumber}
 */
#[no_mangle]
pub static mut xmlDefaultSAXLocator: xmlSAXLocator =
    unsafe {
        {
            let mut init =
                _xmlSAXLocator{getPublicId:
                                   Some(xmlSAX2GetPublicId as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> *const xmlChar),
                               getSystemId:
                                   Some(xmlSAX2GetSystemId as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> *const xmlChar),
                               getLineNumber:
                                   Some(xmlSAX2GetLineNumber as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),
                               getColumnNumber:
                                   Some(xmlSAX2GetColumnNumber as
                                            unsafe extern "C" fn(_:
                                                                     *mut std::os::raw::c_void)
                                                -> std::os::raw::c_int),};
            init
        }
    };
/* *
 * htmlDefaultSAXHandler:
 *
 * Default old SAX v1 handler for HTML, builds the DOM tree
 */
#[no_mangle]
pub static mut htmlDefaultSAXHandler: xmlSAXHandlerV1 =
    unsafe {
        {
            let mut init =
                _xmlSAXHandlerV1{internalSubset:
                                     Some(xmlSAX2InternalSubset as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 isStandalone: None,
                                 hasInternalSubset: None,
                                 hasExternalSubset: None,
                                 resolveEntity: None,
                                 getEntity:
                                     Some(xmlSAX2GetEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlEntityPtr),
                                 entityDecl: None,
                                 notationDecl: None,
                                 attributeDecl: None,
                                 elementDecl: None,
                                 unparsedEntityDecl: None,
                                 setDocumentLocator:
                                     Some(xmlSAX2SetDocumentLocator as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       xmlSAXLocatorPtr)
                                                  -> ()),
                                 startDocument:
                                     Some(xmlSAX2StartDocument as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()),
                                 endDocument:
                                     Some(xmlSAX2EndDocument as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()),
                                 startElement:
                                     Some(xmlSAX2StartElement as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *mut *const xmlChar)
                                                  -> ()),
                                 endElement:
                                     Some(xmlSAX2EndElement as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 reference: None,
                                 characters:
                                     Some(xmlSAX2Characters as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 ignorableWhitespace:
                                     Some(xmlSAX2IgnorableWhitespace as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 processingInstruction:
                                     Some(xmlSAX2ProcessingInstruction as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 comment:
                                     Some(xmlSAX2Comment as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 warning:
                                     Some(xmlParserWarning as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 error:
                                     Some(xmlParserError as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 fatalError:
                                     Some(xmlParserError as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 getParameterEntity:
                                     Some(xmlSAX2GetParameterEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlEntityPtr),
                                 cdataBlock:
                                     Some(xmlSAX2CDataBlock as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 externalSubset: None,
                                 initialized:
                                     0 as std::os::raw::c_int as std::os::raw::c_uint,};
            init
        }
    };
/* LIBXML_HTML_ENABLED */
/* *
 * docbDefaultSAXHandler:
 *
 * Default old SAX v1 handler for SGML DocBook, builds the DOM tree
 */
#[no_mangle]
pub static mut docbDefaultSAXHandler: xmlSAXHandlerV1 =
    unsafe {
        {
            let mut init =
                _xmlSAXHandlerV1{internalSubset:
                                     Some(xmlSAX2InternalSubset as
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
                                     Some(xmlSAX2IsStandalone as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> std::os::raw::c_int),
                                 hasInternalSubset:
                                     Some(xmlSAX2HasInternalSubset as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> std::os::raw::c_int),
                                 hasExternalSubset:
                                     Some(xmlSAX2HasExternalSubset as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> std::os::raw::c_int),
                                 resolveEntity:
                                     Some(xmlSAX2ResolveEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlParserInputPtr),
                                 getEntity:
                                     Some(xmlSAX2GetEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlEntityPtr),
                                 entityDecl:
                                     Some(xmlSAX2EntityDecl as
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
                                 notationDecl: None,
                                 attributeDecl: None,
                                 elementDecl: None,
                                 unparsedEntityDecl: None,
                                 setDocumentLocator:
                                     Some(xmlSAX2SetDocumentLocator as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       xmlSAXLocatorPtr)
                                                  -> ()),
                                 startDocument:
                                     Some(xmlSAX2StartDocument as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()),
                                 endDocument:
                                     Some(xmlSAX2EndDocument as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()),
                                 startElement:
                                     Some(xmlSAX2StartElement as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       *mut *const xmlChar)
                                                  -> ()),
                                 endElement:
                                     Some(xmlSAX2EndElement as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 reference:
                                     Some(xmlSAX2Reference as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 characters:
                                     Some(xmlSAX2Characters as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 ignorableWhitespace:
                                     Some(xmlSAX2IgnorableWhitespace as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar,
                                                                   _:
                                                                       std::os::raw::c_int)
                                                  -> ()),
                                 processingInstruction: None,
                                 comment:
                                     Some(xmlSAX2Comment as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> ()),
                                 warning:
                                     Some(xmlParserWarning as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 error:
                                     Some(xmlParserError as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 fatalError:
                                     Some(xmlParserError as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 getParameterEntity:
                                     Some(xmlSAX2GetParameterEntity as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const xmlChar)
                                                  -> xmlEntityPtr),
                                 cdataBlock: None,
                                 externalSubset: None,
                                 initialized:
                                     0 as std::os::raw::c_int as std::os::raw::c_uint,};
            init
        }
    };
/* LIBXML_DOCB_ENABLED */
/* *
 * xmlInitializeGlobalState:
 * @gs: a pointer to a newly allocated global state
 *
 * xmlInitializeGlobalState() initialize a global state with all the
 * default values of the library.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeGlobalState(mut gs: xmlGlobalStatePtr) {
    /*
     * Perform initialization as required by libxml
     */
    if xmlThrDefMutex.is_null() { xmlInitGlobals(); }
    xmlMutexLock(xmlThrDefMutex);
    initdocbDefaultSAXHandler(&mut (*gs).docbDefaultSAXHandler);
    inithtmlDefaultSAXHandler(&mut (*gs).htmlDefaultSAXHandler);
    (*gs).oldXMLWDcompatibility = 0 as std::os::raw::c_int;
    (*gs).xmlBufferAllocScheme = xmlBufferAllocSchemeThrDef;
    (*gs).xmlDefaultBufferSize = xmlDefaultBufferSizeThrDef;
    initxmlDefaultSAXHandler(&mut (*gs).xmlDefaultSAXHandler,
                             1 as std::os::raw::c_int);
    /* LIBXML_SAX1_ENABLED */
    (*gs).xmlDefaultSAXLocator.getPublicId =
        Some(xmlSAX2GetPublicId as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                     -> *const xmlChar);
    (*gs).xmlDefaultSAXLocator.getSystemId =
        Some(xmlSAX2GetSystemId as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                     -> *const xmlChar);
    (*gs).xmlDefaultSAXLocator.getLineNumber =
        Some(xmlSAX2GetLineNumber as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*gs).xmlDefaultSAXLocator.getColumnNumber =
        Some(xmlSAX2GetColumnNumber as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int);
    (*gs).xmlDoValidityCheckingDefaultValue =
        xmlDoValidityCheckingDefaultValueThrDef;
    (*gs).xmlFree =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void)
                                           -> ()>,
                                xmlFreeFunc>(Some(free as
                                                      unsafe extern "C" fn(_:
                                                                               *mut std::os::raw::c_void)
                                                          -> ()));
    (*gs).xmlMalloc =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: std::os::raw::c_ulong)
                                           -> *mut std::os::raw::c_void>,
                                xmlMallocFunc>(Some(malloc as
                                                        unsafe extern "C" fn(_:
                                                                                 std::os::raw::c_ulong)
                                                            ->
                                                                *mut std::os::raw::c_void));
    (*gs).xmlMallocAtomic =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: std::os::raw::c_ulong)
                                           -> *mut std::os::raw::c_void>,
                                xmlMallocFunc>(Some(malloc as
                                                        unsafe extern "C" fn(_:
                                                                                 std::os::raw::c_ulong)
                                                            ->
                                                                *mut std::os::raw::c_void));
    (*gs).xmlRealloc =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut std::os::raw::c_void,
                                                            _: std::os::raw::c_ulong)
                                           -> *mut std::os::raw::c_void>,
                                xmlReallocFunc>(Some(realloc as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut std::os::raw::c_void,
                                                                              _:
                                                                                  std::os::raw::c_ulong)
                                                             ->
                                                                 *mut std::os::raw::c_void));
    (*gs).xmlMemStrdup =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *const xmlChar)
                                           -> *mut xmlChar>,
                                xmlStrdupFunc>(Some(xmlStrdup as
                                                        unsafe extern "C" fn(_:
                                                                                 *const xmlChar)
                                                            -> *mut xmlChar));
    (*gs).xmlGetWarningsDefaultValue = xmlGetWarningsDefaultValueThrDef;
    (*gs).xmlIndentTreeOutput = xmlIndentTreeOutputThrDef;
    (*gs).xmlTreeIndentString = xmlTreeIndentStringThrDef;
    (*gs).xmlKeepBlanksDefaultValue = xmlKeepBlanksDefaultValueThrDef;
    (*gs).xmlLineNumbersDefaultValue = xmlLineNumbersDefaultValueThrDef;
    (*gs).xmlLoadExtDtdDefaultValue = xmlLoadExtDtdDefaultValueThrDef;
    (*gs).xmlParserDebugEntities = xmlParserDebugEntitiesThrDef;
    (*gs).xmlParserVersion = b"20908\x00" as *const u8 as *const std::os::raw::c_char;
    (*gs).xmlPedanticParserDefaultValue = xmlPedanticParserDefaultValueThrDef;
    (*gs).xmlSaveNoEmptyTags = xmlSaveNoEmptyTagsThrDef;
    (*gs).xmlSubstituteEntitiesDefaultValue =
        xmlSubstituteEntitiesDefaultValueThrDef;
    (*gs).xmlGenericError = xmlGenericErrorThrDef;
    (*gs).xmlStructuredError = xmlStructuredErrorThrDef;
    (*gs).xmlGenericErrorContext = xmlGenericErrorContextThrDef;
    (*gs).xmlStructuredErrorContext = xmlStructuredErrorContextThrDef;
    (*gs).xmlRegisterNodeDefaultValue = xmlRegisterNodeDefaultValueThrDef;
    (*gs).xmlDeregisterNodeDefaultValue = xmlDeregisterNodeDefaultValueThrDef;
    (*gs).xmlParserInputBufferCreateFilenameValue =
        xmlParserInputBufferCreateFilenameValueThrDef;
    (*gs).xmlOutputBufferCreateFilenameValue =
        xmlOutputBufferCreateFilenameValueThrDef;
    memset(&mut (*gs).xmlLastError as *mut xmlError as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlError>() as std::os::raw::c_ulong);
    xmlMutexUnlock(xmlThrDefMutex);
}
/* *
 * DOC_DISABLE : we ignore missing doc for the xmlThrDef functions,
 *               those are really internal work
 */
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetGenericErrorFunc(mut ctx:
                                                          *mut std::os::raw::c_void,
                                                      mut handler:
                                                          xmlGenericErrorFunc) {
    xmlMutexLock(xmlThrDefMutex);
    xmlGenericErrorContextThrDef = ctx;
    if handler.is_some() {
        xmlGenericErrorThrDef = handler
    } else {
        xmlGenericErrorThrDef =
            Some(xmlGenericErrorDefaultFunc as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ())
    }
    xmlMutexUnlock(xmlThrDefMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetStructuredErrorFunc(mut ctx:
                                                             *mut std::os::raw::c_void,
                                                         mut handler:
                                                             xmlStructuredErrorFunc) {
    xmlMutexLock(xmlThrDefMutex);
    xmlStructuredErrorContextThrDef = ctx;
    xmlStructuredErrorThrDef = handler;
    xmlMutexUnlock(xmlThrDefMutex);
}
/* *
 * xmlRegisterNodeDefault:
 * @func: function pointer to the new RegisterNodeFunc
 *
 * Registers a callback for node creation
 *
 * Returns the old value of the registration function
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterNodeDefault(mut func: xmlRegisterNodeFunc)
 -> xmlRegisterNodeFunc {
    let mut old: xmlRegisterNodeFunc = xmlRegisterNodeDefaultValue;
    __xmlRegisterCallbacks = 1 as std::os::raw::c_int;
    xmlRegisterNodeDefaultValue = func;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefRegisterNodeDefault(mut func:
                                                          xmlRegisterNodeFunc)
 -> xmlRegisterNodeFunc {
    let mut old: xmlRegisterNodeFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlRegisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as std::os::raw::c_int;
    xmlRegisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
/* *
 * xmlDeregisterNodeDefault:
 * @func: function pointer to the new DeregisterNodeFunc
 *
 * Registers a callback for node destruction
 *
 * Returns the previous value of the deregistration function
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDeregisterNodeDefault(mut func:
                                                      xmlDeregisterNodeFunc)
 -> xmlDeregisterNodeFunc {
    let mut old: xmlDeregisterNodeFunc = xmlDeregisterNodeDefaultValue;
    __xmlRegisterCallbacks = 1 as std::os::raw::c_int;
    xmlDeregisterNodeDefaultValue = func;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDeregisterNodeDefault(mut func:
                                                            xmlDeregisterNodeFunc)
 -> xmlDeregisterNodeFunc {
    let mut old: xmlDeregisterNodeFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlDeregisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as std::os::raw::c_int;
    xmlDeregisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserInputBufferCreateFilenameDefault(mut func:
                                                                             xmlParserInputBufferCreateFilenameFunc)
 -> xmlParserInputBufferCreateFilenameFunc {
    let mut old: xmlParserInputBufferCreateFilenameFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlParserInputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old =
            Some(__xmlParserInputBufferCreateFilename as
                     unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                          _: xmlCharEncoding)
                         -> xmlParserInputBufferPtr)
    }
    xmlParserInputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefOutputBufferCreateFilenameDefault(mut func:
                                                                        xmlOutputBufferCreateFilenameFunc)
 -> xmlOutputBufferCreateFilenameFunc {
    let mut old: xmlOutputBufferCreateFilenameFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlOutputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old =
            Some(__xmlOutputBufferCreateFilename as
                     unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                          _: xmlCharEncodingHandlerPtr,
                                          _: std::os::raw::c_int)
                         -> xmlOutputBufferPtr)
    }
    xmlOutputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn __docbDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThread() != 0 {
        return &mut docbDefaultSAXHandler
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).docbDefaultSAXHandler
    };
}
#[no_mangle]
pub unsafe extern "C" fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThread() != 0 {
        return &mut htmlDefaultSAXHandler
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).htmlDefaultSAXHandler
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLastError() -> *mut xmlError {
    if xmlIsMainThread() != 0 {
        return &mut xmlLastError
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlLastError
    };
}
/*
 * The following memory routines were apparently lost at some point,
 * and were re-inserted at this point on June 10, 2004.  Hope it's
 * the right place for them :-)
 */
/*
 * Everything starting from the line below is
 * Automatically generated by build_glob.py.
 * Do not modify the previous line.
 */
#[no_mangle]
pub unsafe extern "C" fn __oldXMLWDcompatibility() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut oldXMLWDcompatibility
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).oldXMLWDcompatibility
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlBufferAllocScheme()
 -> *mut xmlBufferAllocationScheme {
    if xmlIsMainThread() != 0 {
        return &mut xmlBufferAllocScheme
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlBufferAllocScheme
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefBufferAllocScheme(mut v:
                                                        xmlBufferAllocationScheme)
 -> xmlBufferAllocationScheme {
    let mut ret: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_DOUBLEIT;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlBufferAllocSchemeThrDef;
    xmlBufferAllocSchemeThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultBufferSize() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlDefaultBufferSize
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlDefaultBufferSize
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDefaultBufferSize(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlDefaultBufferSizeThrDef;
    xmlDefaultBufferSizeThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThread() != 0 {
        return &mut xmlDefaultSAXHandler
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlDefaultSAXHandler
    };
}
/* LIBXML_SAX1_ENABLED */
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator {
    if xmlIsMainThread() != 0 {
        return &mut xmlDefaultSAXLocator
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlDefaultSAXLocator
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDoValidityCheckingDefaultValue()
 -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlDoValidityCheckingDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlDoValidityCheckingDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDoValidityCheckingDefaultValue(mut v:
                                                                     std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlDoValidityCheckingDefaultValueThrDef;
    xmlDoValidityCheckingDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericError() -> *mut xmlGenericErrorFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlGenericError
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlGenericError
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredError()
 -> *mut xmlStructuredErrorFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlStructuredError
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlStructuredError
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericErrorContext()
 -> *mut *mut std::os::raw::c_void {
    if xmlIsMainThread() != 0 {
        return &mut xmlGenericErrorContext
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlGenericErrorContext
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredErrorContext()
 -> *mut *mut std::os::raw::c_void {
    if xmlIsMainThread() != 0 {
        return &mut xmlStructuredErrorContext
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlStructuredErrorContext
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGetWarningsDefaultValue() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlGetWarningsDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlGetWarningsDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefGetWarningsDefaultValue(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlGetWarningsDefaultValueThrDef;
    xmlGetWarningsDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlIndentTreeOutput() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlIndentTreeOutput
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlIndentTreeOutput
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefIndentTreeOutput(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlIndentTreeOutputThrDef;
    xmlIndentTreeOutputThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlTreeIndentString() -> *mut *const std::os::raw::c_char {
    if xmlIsMainThread() != 0 {
        return &mut xmlTreeIndentString
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlTreeIndentString
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefTreeIndentString(mut v: *const std::os::raw::c_char)
 -> *const std::os::raw::c_char {
    let mut ret: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlTreeIndentStringThrDef;
    xmlTreeIndentStringThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlKeepBlanksDefaultValue() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlKeepBlanksDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlKeepBlanksDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefKeepBlanksDefaultValue(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlKeepBlanksDefaultValueThrDef;
    xmlKeepBlanksDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLineNumbersDefaultValue() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlLineNumbersDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlLineNumbersDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLineNumbersDefaultValue(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlLineNumbersDefaultValueThrDef;
    xmlLineNumbersDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLoadExtDtdDefaultValue() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlLoadExtDtdDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlLoadExtDtdDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLoadExtDtdDefaultValue(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlLoadExtDtdDefaultValueThrDef;
    xmlLoadExtDtdDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserDebugEntities() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlParserDebugEntities
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlParserDebugEntities
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserDebugEntities(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlParserDebugEntitiesThrDef;
    xmlParserDebugEntitiesThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserVersion() -> *mut *const std::os::raw::c_char {
    if xmlIsMainThread() != 0 {
        return &mut xmlParserVersion
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlParserVersion
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlPedanticParserDefaultValue()
 -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlPedanticParserDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlPedanticParserDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefPedanticParserDefaultValue(mut v:
                                                                 std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlPedanticParserDefaultValueThrDef;
    xmlPedanticParserDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSaveNoEmptyTags() -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlSaveNoEmptyTags
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               -> xmlGlobalStatePtr)()).xmlSaveNoEmptyTags
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSaveNoEmptyTags(mut v: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlSaveNoEmptyTagsThrDef;
    xmlSaveNoEmptyTagsThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSubstituteEntitiesDefaultValue()
 -> *mut std::os::raw::c_int {
    if xmlIsMainThread() != 0 {
        return &mut xmlSubstituteEntitiesDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlSubstituteEntitiesDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSubstituteEntitiesDefaultValue(mut v:
                                                                     std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlSubstituteEntitiesDefaultValueThrDef;
    xmlSubstituteEntitiesDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRegisterNodeDefaultValue()
 -> *mut xmlRegisterNodeFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlRegisterNodeDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlRegisterNodeDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDeregisterNodeDefaultValue()
 -> *mut xmlDeregisterNodeFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlDeregisterNodeDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlDeregisterNodeDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilenameValue()
 -> *mut xmlParserInputBufferCreateFilenameFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlParserInputBufferCreateFilenameValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlParserInputBufferCreateFilenameValue
    };
}
/*
 * Summary: interface for all global variables of the library
 * Description: all the global variables and thread handling for
 *              those variables is handled by this module.
 *
 * The bottom of this file is automatically generated by build_glob.py
 * based on the description file global.data
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Gary Pennington <Gary.Pennington@uk.sun.com>, Daniel Veillard
 */
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
/* *
 * xmlDeregisterNodeFunc:
 * @node: the current node
 *
 * Signature for the deregistration callback of a discarded node
 */
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
/* LIBXML_THREAD_ALLOC_ENABLED */
/*
 * Everything starting from the line below is
 * Automatically generated by build_glob.py.
 * Do not modify the previous line.
 */
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilenameValue()
 -> *mut xmlOutputBufferCreateFilenameFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlOutputBufferCreateFilenameValue
    } else {
        return &mut (*(xmlGetGlobalState as
                           unsafe extern "C" fn()
                               ->
                                   xmlGlobalStatePtr)()).xmlOutputBufferCreateFilenameValue
    };
}
/* __INCLUDE_ELFGCCHACK */
