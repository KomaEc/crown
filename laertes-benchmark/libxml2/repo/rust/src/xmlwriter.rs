
extern "C" {
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
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcasecmp(str1: *const xmlChar, str2: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlSetDocCompressMode(doc: xmlDocPtr, mode: std::os::raw::c_int);
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
    fn xmlListEmpty(l: xmlListPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlListFront(l: xmlListPtr) -> xmlLinkPtr;
    #[no_mangle]
    fn xmlListSize(l: xmlListPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlListPopFront(l: xmlListPtr);
    #[no_mangle]
    fn xmlListPushFront(l: xmlListPtr, data: *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
    /* Link operators */
    #[no_mangle]
    fn xmlLinkGetData(lk: xmlLinkPtr) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlEncodeSpecialChars(doc: *const xmlDoc, input: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlFindCharEncodingHandler(name: *const std::os::raw::c_char)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlOutputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                     encoder: xmlCharEncodingHandlerPtr,
                                     compression: std::os::raw::c_int)
     -> xmlOutputBufferPtr;
    #[no_mangle]
    fn xmlOutputBufferCreateBuffer(buffer: xmlBufferPtr,
                                   encoder: xmlCharEncodingHandlerPtr)
     -> xmlOutputBufferPtr;
    #[no_mangle]
    fn xmlOutputBufferCreateIO(iowrite: xmlOutputWriteCallback,
                               ioclose: xmlOutputCloseCallback,
                               ioctx: *mut std::os::raw::c_void,
                               encoder: xmlCharEncodingHandlerPtr)
     -> xmlOutputBufferPtr;
    #[no_mangle]
    fn xmlOutputBufferWrite(out: xmlOutputBufferPtr, len: std::os::raw::c_int,
                            buf: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlOutputBufferWriteString(out: xmlOutputBufferPtr,
                                  str: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> std::os::raw::c_int;
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
    fn xmlSAX2StartElement(ctx: *mut std::os::raw::c_void, fullname: *const xmlChar,
                           atts: *mut *const xmlChar);
    #[no_mangle]
    fn xmlSAX2EndElement(ctx: *mut std::os::raw::c_void, name: *const xmlChar);
    #[no_mangle]
    fn xmlSAX2InitDefaultSAXHandler(hdlr: *mut xmlSAXHandler,
                                    warning: std::os::raw::c_int);
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
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn htmlNewDocNoDtD(URI: *const xmlChar, ExternalID: *const xmlChar)
     -> htmlDocPtr;
    #[no_mangle]
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    #[no_mangle]
    fn xmlCharEncOutput(output: xmlOutputBufferPtr, init: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /*
 * Summary: Internal Interfaces for saving in libxml2
 * Description: this module describes a few interfaces which were
 *              addded along with the API changes in 2.9.0
 *              those are private routines at this point
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    #[no_mangle]
    fn xmlBufAttrSerializeTxtContent(buf: xmlBufPtr, doc: xmlDocPtr,
                                     attr: xmlAttrPtr,
                                     string: *const xmlChar);
    #[no_mangle]
    fn xmlTextWriterVSprintf(format: *const std::os::raw::c_char,
                             argptr: ::std::ffi::VaList) -> *mut xmlChar;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut std::os::raw::c_void,
    pub reg_save_area: *mut std::os::raw::c_void,
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
pub type xmlBufPtr = *mut xmlBuf;
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
pub type xmlOutputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlOutputWriteCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
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
pub type htmlDocPtr = xmlDocPtr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextWriter {
    pub out: xmlOutputBufferPtr,
    pub nodes: xmlListPtr,
    pub nsstack: xmlListPtr,
    pub level: std::os::raw::c_int,
    pub indent: std::os::raw::c_int,
    pub doindent: std::os::raw::c_int,
    pub ichar: *mut xmlChar,
    pub qchar: std::os::raw::c_char,
    pub ctxt: xmlParserCtxtPtr,
    pub no_doc_free: std::os::raw::c_int,
    pub doc: xmlDocPtr,
}
/*
 * Summary: text writing API for XML
 * Description: text writing API for XML
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Alfred Mickautsch <alfred@mickautsch.de>
 */
pub type xmlTextWriter = _xmlTextWriter;
pub type xmlTextWriterPtr = *mut xmlTextWriter;
pub type xmlTextWriterNsStackEntry = _xmlTextWriterNsStackEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextWriterNsStackEntry {
    pub prefix: *mut xmlChar,
    pub uri: *mut xmlChar,
    pub elem: xmlLinkPtr,
}
pub type xmlTextWriterStackEntry = _xmlTextWriterStackEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextWriterStackEntry {
    pub name: *mut xmlChar,
    pub state: xmlTextWriterState,
}
pub type xmlTextWriterState = std::os::raw::c_uint;
pub const XML_TEXTWRITER_COMMENT: xmlTextWriterState = 16;
pub const XML_TEXTWRITER_DTD_PENT: xmlTextWriterState = 15;
pub const XML_TEXTWRITER_DTD_ENTY_TEXT: xmlTextWriterState = 14;
pub const XML_TEXTWRITER_DTD_ENTY: xmlTextWriterState = 13;
pub const XML_TEXTWRITER_DTD_ATTL_TEXT: xmlTextWriterState = 12;
pub const XML_TEXTWRITER_DTD_ATTL: xmlTextWriterState = 11;
pub const XML_TEXTWRITER_DTD_ELEM_TEXT: xmlTextWriterState = 10;
pub const XML_TEXTWRITER_DTD_ELEM: xmlTextWriterState = 9;
pub const XML_TEXTWRITER_DTD_TEXT: xmlTextWriterState = 8;
pub const XML_TEXTWRITER_DTD: xmlTextWriterState = 7;
pub const XML_TEXTWRITER_CDATA: xmlTextWriterState = 6;
pub const XML_TEXTWRITER_PI_TEXT: xmlTextWriterState = 5;
pub const XML_TEXTWRITER_PI: xmlTextWriterState = 4;
pub const XML_TEXTWRITER_TEXT: xmlTextWriterState = 3;
pub const XML_TEXTWRITER_ATTRIBUTE: xmlTextWriterState = 2;
pub const XML_TEXTWRITER_NAME: xmlTextWriterState = 1;
pub const XML_TEXTWRITER_NONE: xmlTextWriterState = 0;
/*
 * Used by variadic.c
 */
/* *
 * xmlWriterErrMsg:
 * @ctxt:  a writer context
 * @error:  the error number
 * @msg:  the error message
 *
 * Handle a writer error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlWriterErrMsg(mut ctxt: xmlTextWriterPtr,
                                         mut error: xmlParserErrors,
                                         mut msg: *const std::os::raw::c_char) {
    if !ctxt.is_null() {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        (*ctxt).ctxt as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        XML_FROM_WRITER as std::os::raw::c_int, error as std::os::raw::c_int,
                        XML_ERR_FATAL, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg);
    } else {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_WRITER as std::os::raw::c_int, error as std::os::raw::c_int,
                        XML_ERR_FATAL, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg);
    };
}
/* *
 * xmlWriterErrMsgInt:
 * @ctxt:  a writer context
 * @error:  the error number
 * @msg:  the error message
 * @val:  an int
 *
 * Handle a writer error
 */
unsafe extern "C" fn xmlWriterErrMsgInt(mut ctxt: xmlTextWriterPtr,
                                        mut error: xmlParserErrors,
                                        mut msg: *const std::os::raw::c_char,
                                        mut val: std::os::raw::c_int) {
    if !ctxt.is_null() {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        (*ctxt).ctxt as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        XML_FROM_WRITER as std::os::raw::c_int, error as std::os::raw::c_int,
                        XML_ERR_FATAL, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        val, 0 as std::os::raw::c_int, msg, val);
    } else {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_WRITER as std::os::raw::c_int, error as std::os::raw::c_int,
                        XML_ERR_FATAL, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        val, 0 as std::os::raw::c_int, msg, val);
    };
}
/*
 * Constructors & Destructor
 */
/* *
 * xmlNewTextWriter:
 * @out:  an xmlOutputBufferPtr
 *
 * Create a new xmlNewTextWriter structure using an xmlOutputBufferPtr
 * NOTE: the @out parameter will be deallocated when the writer is closed
 *       (if the call succeed.)
 *
 * Returns the new xmlTextWriterPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextWriter(mut out: xmlOutputBufferPtr)
 -> xmlTextWriterPtr {
    let mut ret: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriter>()
                                                          as std::os::raw::c_ulong) as
            xmlTextWriterPtr;
    if ret.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriter : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlTextWriter>() as std::os::raw::c_ulong);
    (*ret).nodes =
        xmlListCreate(Some(xmlFreeTextWriterStackEntry as
                               unsafe extern "C" fn(_: xmlLinkPtr) -> ()),
                      Some(xmlCmpTextWriterStackEntry as
                               unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                                    _: *const std::os::raw::c_void)
                                   -> std::os::raw::c_int));
    if (*ret).nodes.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriter : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlTextWriterPtr
    }
    (*ret).nsstack =
        xmlListCreate(Some(xmlFreeTextWriterNsStackEntry as
                               unsafe extern "C" fn(_: xmlLinkPtr) -> ()),
                      Some(xmlCmpTextWriterNsStackEntry as
                               unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                                    _: *const std::os::raw::c_void)
                                   -> std::os::raw::c_int));
    if (*ret).nsstack.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriter : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlListDelete((*ret).nodes);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlTextWriterPtr
    }
    (*ret).out = out;
    (*ret).ichar =
        xmlStrdup(b" \x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar);
    (*ret).qchar = '\"' as i32 as std::os::raw::c_char;
    if (*ret).ichar.is_null() {
        xmlListDelete((*ret).nodes);
        xmlListDelete((*ret).nsstack);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriter : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    (*ret).doc = xmlNewDoc(0 as *const xmlChar);
    (*ret).no_doc_free = 0 as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlNewTextWriterFilename:
 * @uri:  the URI of the resource for the output
 * @compression:  compress the output?
 *
 * Create a new xmlNewTextWriter structure with @uri as output
 *
 * Returns the new xmlTextWriterPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextWriterFilename(mut uri:
                                                      *const std::os::raw::c_char,
                                                  mut compression:
                                                      std::os::raw::c_int)
 -> xmlTextWriterPtr {
    let mut ret: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut out: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    out =
        xmlOutputBufferCreateFilename(uri, 0 as xmlCharEncodingHandlerPtr,
                                      compression);
    if out.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_IO_EIO,
                        b"xmlNewTextWriterFilename : cannot open uri\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    ret = xmlNewTextWriter(out);
    if ret.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriterFilename : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlOutputBufferClose(out);
        return 0 as xmlTextWriterPtr
    }
    (*ret).indent = 0 as std::os::raw::c_int;
    (*ret).doindent = 0 as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlNewTextWriterMemory:
 * @buf:  xmlBufferPtr
 * @compression:  compress the output?
 *
 * Create a new xmlNewTextWriter structure with @buf as output
 * TODO: handle compression
 *
 * Returns the new xmlTextWriterPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextWriterMemory(mut buf: xmlBufferPtr,
                                                mut compression: std::os::raw::c_int)
 -> xmlTextWriterPtr {
    let mut ret: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut out: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    /*::todo handle compression */
    out = xmlOutputBufferCreateBuffer(buf, 0 as xmlCharEncodingHandlerPtr);
    if out.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriterMemory : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    ret = xmlNewTextWriter(out);
    if ret.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_NO_MEMORY,
                        b"xmlNewTextWriterMemory : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlOutputBufferClose(out);
        return 0 as xmlTextWriterPtr
    }
    return ret;
}
/* *
 * xmlNewTextWriterPushParser:
 * @ctxt: xmlParserCtxtPtr to hold the new XML document tree
 * @compression:  compress the output?
 *
 * Create a new xmlNewTextWriter structure with @ctxt as output
 * NOTE: the @ctxt context will be freed with the resulting writer
 *       (if the call succeeds).
 * TODO: handle compression
 *
 * Returns the new xmlTextWriterPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextWriterPushParser(mut ctxt:
                                                        xmlParserCtxtPtr,
                                                    mut compression:
                                                        std::os::raw::c_int)
 -> xmlTextWriterPtr {
    let mut ret: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut out: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if ctxt.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterPushParser : invalid context!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    out =
        xmlOutputBufferCreateIO(Some(xmlTextWriterWriteDocCallback as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *const std::os::raw::c_char,
                                                              _: std::os::raw::c_int)
                                             -> std::os::raw::c_int),
                                Some(xmlTextWriterCloseDocCallback as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void)
                                             -> std::os::raw::c_int),
                                ctxt as *mut std::os::raw::c_void,
                                0 as xmlCharEncodingHandlerPtr);
    if out.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterPushParser : error at xmlOutputBufferCreateIO!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    ret = xmlNewTextWriter(out);
    if ret.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterPushParser : error at xmlNewTextWriter!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        xmlOutputBufferClose(out);
        return 0 as xmlTextWriterPtr
    }
    (*ret).ctxt = ctxt;
    return ret;
}
/* *
 * xmlNewTextWriterDoc:
 * @doc: address of a xmlDocPtr to hold the new XML document tree
 * @compression:  compress the output?
 *
 * Create a new xmlNewTextWriter structure with @*doc as output
 *
 * Returns the new xmlTextWriterPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextWriterDoc(mut doc: *mut xmlDocPtr,
                                             mut compression: std::os::raw::c_int)
 -> xmlTextWriterPtr {
    let mut ret: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut saxHandler: xmlSAXHandler =
        xmlSAXHandler{internalSubset: None,
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
                      initialized: 0,
                      _private: 0 as *mut std::os::raw::c_void,
                      startElementNs: None,
                      endElementNs: None,
                      serror: None,};
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    memset(&mut saxHandler as *mut xmlSAXHandler as *mut std::os::raw::c_void,
           '\u{0}' as i32,
           ::std::mem::size_of::<xmlSAXHandler>() as std::os::raw::c_ulong);
    xmlSAX2InitDefaultSAXHandler(&mut saxHandler, 1 as std::os::raw::c_int);
    saxHandler.startDocument =
        Some(xmlTextWriterStartDocumentCallback as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    saxHandler.startElement =
        Some(xmlSAX2StartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    saxHandler.endElement =
        Some(xmlSAX2EndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    ctxt =
        xmlCreatePushParserCtxt(&mut saxHandler, 0 as *mut std::os::raw::c_void,
                                0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                                0 as *const std::os::raw::c_char);
    if ctxt.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterDoc : error at xmlCreatePushParserCtxt!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    /*
     * For some reason this seems to completely break if node names
     * are interned.
     */
    (*ctxt).dictNames = 0 as std::os::raw::c_int;
    (*ctxt).myDoc =
        xmlNewDoc(b"1.0\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar);
    if (*ctxt).myDoc.is_null() {
        xmlFreeParserCtxt(ctxt);
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterDoc : error at xmlNewDoc!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    ret = xmlNewTextWriterPushParser(ctxt, compression);
    if ret.is_null() {
        xmlFreeDoc((*ctxt).myDoc);
        xmlFreeParserCtxt(ctxt);
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterDoc : error at xmlNewTextWriterPushParser!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    xmlSetDocCompressMode((*ctxt).myDoc, compression);
    if !doc.is_null() {
        *doc = (*ctxt).myDoc;
        (*ret).no_doc_free = 1 as std::os::raw::c_int
    }
    return ret;
}
/* *
 * xmlNewTextWriterTree:
 * @doc: xmlDocPtr
 * @node: xmlNodePtr or NULL for doc->children
 * @compression:  compress the output?
 *
 * Create a new xmlNewTextWriter structure with @doc as output
 * starting at @node
 *
 * Returns the new xmlTextWriterPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextWriterTree(mut doc: xmlDocPtr,
                                              mut node: xmlNodePtr,
                                              mut compression: std::os::raw::c_int)
 -> xmlTextWriterPtr {
    let mut ret: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut saxHandler: xmlSAXHandler =
        xmlSAXHandler{internalSubset: None,
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
                      initialized: 0,
                      _private: 0 as *mut std::os::raw::c_void,
                      startElementNs: None,
                      endElementNs: None,
                      serror: None,};
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if doc.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterTree : invalid document tree!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    memset(&mut saxHandler as *mut xmlSAXHandler as *mut std::os::raw::c_void,
           '\u{0}' as i32,
           ::std::mem::size_of::<xmlSAXHandler>() as std::os::raw::c_ulong);
    xmlSAX2InitDefaultSAXHandler(&mut saxHandler, 1 as std::os::raw::c_int);
    saxHandler.startDocument =
        Some(xmlTextWriterStartDocumentCallback as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    saxHandler.startElement =
        Some(xmlSAX2StartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    saxHandler.endElement =
        Some(xmlSAX2EndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    ctxt =
        xmlCreatePushParserCtxt(&mut saxHandler, 0 as *mut std::os::raw::c_void,
                                0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                                0 as *const std::os::raw::c_char);
    if ctxt.is_null() {
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterDoc : error at xmlCreatePushParserCtxt!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    /*
     * For some reason this seems to completely break if node names
     * are interned.
     */
    (*ctxt).dictNames = 0 as std::os::raw::c_int;
    ret = xmlNewTextWriterPushParser(ctxt, compression);
    if ret.is_null() {
        xmlFreeParserCtxt(ctxt);
        xmlWriterErrMsg(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlNewTextWriterDoc : error at xmlNewTextWriterPushParser!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return 0 as xmlTextWriterPtr
    }
    (*ctxt).myDoc = doc;
    (*ctxt).node = node;
    (*ret).no_doc_free = 1 as std::os::raw::c_int;
    xmlSetDocCompressMode(doc, compression);
    return ret;
}
/* *
 * xmlFreeTextWriter:
 * @writer:  the xmlTextWriterPtr
 *
 * Deallocate all the resources associated to the writer
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeTextWriter(mut writer: xmlTextWriterPtr) {
    if writer.is_null() { return }
    if !(*writer).out.is_null() { xmlOutputBufferClose((*writer).out); }
    if !(*writer).nodes.is_null() { xmlListDelete((*writer).nodes); }
    if !(*writer).nsstack.is_null() { xmlListDelete((*writer).nsstack); }
    if !(*writer).ctxt.is_null() {
        if !(*(*writer).ctxt).myDoc.is_null() &&
               (*writer).no_doc_free == 0 as std::os::raw::c_int {
            xmlFreeDoc((*(*writer).ctxt).myDoc);
            (*(*writer).ctxt).myDoc = 0 as xmlDocPtr
        }
        xmlFreeParserCtxt((*writer).ctxt);
    }
    if !(*writer).doc.is_null() { xmlFreeDoc((*writer).doc); }
    if !(*writer).ichar.is_null() {
        xmlFree.expect("non-null function pointer")((*writer).ichar as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(writer as *mut std::os::raw::c_void);
}
/*
 * Functions
 */
/*
 * Document
 */
/* *
 * xmlTextWriterStartDocument:
 * @writer:  the xmlTextWriterPtr
 * @version:  the xml version ("1.0") or NULL for default ("1.0")
 * @encoding:  the encoding or NULL for default
 * @standalone: "yes" or "no" or NULL for default
 *
 * Start a new xml document
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartDocument(mut writer:
                                                        xmlTextWriterPtr,
                                                    mut version:
                                                        *const std::os::raw::c_char,
                                                    mut encoding:
                                                        *const std::os::raw::c_char,
                                                    mut standalone:
                                                        *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut encoder: xmlCharEncodingHandlerPtr =
        0 as *mut xmlCharEncodingHandler;
    if writer.is_null() || (*writer).out.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterStartDocument : invalid writer!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() && !xmlLinkGetData(lk).is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterStartDocument : not allowed in this context!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    encoder = 0 as xmlCharEncodingHandlerPtr;
    if !encoding.is_null() {
        encoder = xmlFindCharEncodingHandler(encoding);
        if encoder.is_null() {
            xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                            b"xmlTextWriterStartDocument : out of memory!\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    (*(*writer).out).encoder = encoder;
    if !encoder.is_null() {
        if (*(*writer).out).conv.is_null() {
            (*(*writer).out).conv =
                xmlBufCreateSize(4000 as std::os::raw::c_int as size_t)
        }
        xmlCharEncOutput((*writer).out, 1 as std::os::raw::c_int);
        if !(*writer).doc.is_null() && (*(*writer).doc).encoding.is_null() {
            (*(*writer).doc).encoding =
                xmlStrdup((*(*(*writer).out).encoder).name as *mut xmlChar)
        }
    } else { (*(*writer).out).conv = 0 as xmlBufPtr }
    sum = 0 as std::os::raw::c_int;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<?xml version=\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                             &mut (*writer).qchar);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !version.is_null() {
        count = xmlOutputBufferWriteString((*writer).out, version)
    } else {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"1.0\x00" as *const u8 as
                                           *const std::os::raw::c_char)
    }
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                             &mut (*writer).qchar);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !(*(*writer).out).encoder.is_null() {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" encoding=\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       (*(*(*writer).out).encoder).name);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    if !standalone.is_null() {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" standalone=\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count = xmlOutputBufferWriteString((*writer).out, standalone);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"?>\n\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndDocument:
 * @writer:  the xmlTextWriterPtr
 *
 * End an xml document. All open elements are closed, and
 * the content is flushed to the output.
 *
 * Returns the bytes written or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndDocument(mut writer:
                                                      xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterEndDocument : invalid writer!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    loop  {
        lk = xmlListFront((*writer).nodes);
        if lk.is_null() { break ; }
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if p.is_null() { break ; }
        match (*p).state as std::os::raw::c_uint {
            1 | 2 | 3 => {
                count = xmlTextWriterEndElement(writer);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count
            }
            4 | 5 => {
                count = xmlTextWriterEndPI(writer);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count
            }
            6 => {
                count = xmlTextWriterEndCDATA(writer);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count
            }
            7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
                count = xmlTextWriterEndDTD(writer);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count
            }
            16 => {
                count = xmlTextWriterEndComment(writer);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count
            }
            _ => { }
        }
    }
    if (*writer).indent == 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    sum += xmlTextWriterFlush(writer);
    return sum;
}
/*
 * Comments
 */
/* *
 * xmlTextWriterStartComment:
 * @writer:  the xmlTextWriterPtr
 *
 * Start an xml comment.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartComment(mut writer:
                                                       xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterStartComment : invalid writer!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            match (*p).state as std::os::raw::c_uint {
                3 | 0 => { }
                1 => {
                    /* Output namespace declarations */
                    count = xmlTextWriterOutputNSDecl(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b">\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    if (*writer).indent != 0 {
                        count =
                            xmlOutputBufferWriteString((*writer).out,
                                                       b"\n\x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char);
                        if count < 0 as std::os::raw::c_int {
                            return -(1 as std::os::raw::c_int)
                        }
                        sum += count
                    }
                    (*p).state = XML_TEXTWRITER_TEXT
                }
                _ => { return -(1 as std::os::raw::c_int) }
            }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartElement : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = 0 as *mut xmlChar;
    (*p).state = XML_TEXTWRITER_COMMENT;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    if (*writer).indent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<!--\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndComment:
 * @writer:  the xmlTextWriterPtr
 *
 * End the current xml coment.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndComment(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterEndComment : invalid writer!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterEndComment : not allowed in this context!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    match (*p).state as std::os::raw::c_uint {
        16 => {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b"-->\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatComment:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write an xml comment.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatComment(xmlTextWriterPtr writer,
//                                 const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatComment(writer, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatComment:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write an xml comment.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatComment(mut writer:
                                                              xmlTextWriterPtr,
                                                          mut format:
                                                              *const std::os::raw::c_char,
                                                          mut argptr:
                                                              ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterWriteVFormatComment : invalid writer!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteComment(writer, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteComment:
 * @writer:  the xmlTextWriterPtr
 * @content:  comment string
 *
 * Write an xml comment.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteComment(mut writer:
                                                       xmlTextWriterPtr,
                                                   mut content:
                                                       *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartComment(writer);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndComment(writer);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * Elements
 */
/* *
 * xmlTextWriterStartElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  element name
 *
 * Start an xml element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartElement(mut writer:
                                                       xmlTextWriterPtr,
                                                   mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            let mut current_block_20: u64;
            match (*p).state as std::os::raw::c_uint {
                4 | 5 => { return -(1 as std::os::raw::c_int) }
                2 => {
                    count = xmlTextWriterEndAttribute(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    current_block_20 = 16240089969647018763;
                }
                1 => { current_block_20 = 16240089969647018763; }
                0 | _ => { current_block_20 = 17478428563724192186; }
            }
            match current_block_20 {
                16240089969647018763 =>
                /* fallthrough */
                /* Output namespace declarations */
                {
                    count = xmlTextWriterOutputNSDecl(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b">\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    if (*writer).indent != 0 {
                        count =
                            xmlOutputBufferWriteString((*writer).out,
                                                       b"\n\x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char)
                    }
                    (*p).state = XML_TEXTWRITER_TEXT
                }
                _ => { }
            }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartElement : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = xmlStrdup(name);
    if (*p).name.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartElement : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    (*p).state = XML_TEXTWRITER_NAME;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    if (*writer).indent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   (*p).name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterStartElementNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix or NULL
 * @name:  element local name
 * @namespaceURI:  namespace URI or NULL
 *
 * Start an xml element with namespace support.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartElementNS(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut prefix:
                                                         *const xmlChar,
                                                     mut name: *const xmlChar,
                                                     mut namespaceURI:
                                                         *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    buf = 0 as *mut xmlChar;
    if !prefix.is_null() {
        buf = xmlStrdup(prefix);
        buf =
            xmlStrcat(buf,
                      b":\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar)
    }
    buf = xmlStrcat(buf, name);
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartElement(writer, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !namespaceURI.is_null() {
        let mut p: *mut xmlTextWriterNsStackEntry =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterNsStackEntry>()
                                                              as
                                                              std::os::raw::c_ulong)
                as *mut xmlTextWriterNsStackEntry;
        if p.is_null() {
            xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                            b"xmlTextWriterStartElementNS : out of memory!\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        buf =
            xmlStrdup(b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        if !prefix.is_null() {
            buf =
                xmlStrcat(buf,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            buf = xmlStrcat(buf, prefix)
        }
        (*p).prefix = buf;
        (*p).uri = xmlStrdup(namespaceURI);
        if (*p).uri.is_null() {
            xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                            b"xmlTextWriterStartElementNS : out of memory!\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(p as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        (*p).elem = xmlListFront((*writer).nodes);
        xmlListPushFront((*writer).nsstack, p as *mut std::os::raw::c_void);
    }
    return sum;
}
/* *
 * xmlTextWriterEndElement:
 * @writer:  the xmlTextWriterPtr
 *
 * End the current xml element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndElement(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() {
        xmlListDelete((*writer).nsstack);
        (*writer).nsstack = 0 as xmlListPtr;
        return -(1 as std::os::raw::c_int)
    }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlListDelete((*writer).nsstack);
        (*writer).nsstack = 0 as xmlListPtr;
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    let mut current_block_50: u64;
    match (*p).state as std::os::raw::c_uint {
        2 => {
            count = xmlTextWriterEndAttribute(writer);
            if count < 0 as std::os::raw::c_int {
                xmlListDelete((*writer).nsstack);
                (*writer).nsstack = 0 as xmlListPtr;
                return -(1 as std::os::raw::c_int)
            }
            sum += count;
            current_block_50 = 5073649793417872409;
        }
        1 => { current_block_50 = 5073649793417872409; }
        3 => {
            if (*writer).indent != 0 && (*writer).doindent != 0 {
                count = xmlTextWriterWriteIndent(writer);
                sum += count;
                (*writer).doindent = 1 as std::os::raw::c_int
            } else { (*writer).doindent = 1 as std::os::raw::c_int }
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b"</\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           (*p).name as *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b">\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            current_block_50 = 3160140712158701372;
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    match current_block_50 {
        5073649793417872409 =>
        /* fallthrough */
        /* Output namespace declarations */
        {
            count = xmlTextWriterOutputNSDecl(writer);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            if (*writer).indent != 0 {
                /* next element needs indent */
                (*writer).doindent = 1 as std::os::raw::c_int
            }
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b"/>\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { }
    }
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterFullEndElement:
 * @writer:  the xmlTextWriterPtr
 *
 * End the current xml element. Writes an end tag even if the element is empty
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterFullEndElement(mut writer:
                                                         xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    let mut current_block_41: u64;
    match (*p).state as std::os::raw::c_uint {
        2 => {
            count = xmlTextWriterEndAttribute(writer);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            current_block_41 = 5575406638025445328;
        }
        1 => { current_block_41 = 5575406638025445328; }
        3 => { current_block_41 = 13580001573093494583; }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    match current_block_41 {
        5575406638025445328 =>
        /* fallthrough */
        /* Output namespace declarations */
        {
            count = xmlTextWriterOutputNSDecl(writer);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b">\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            if (*writer).indent != 0 { (*writer).doindent = 0 as std::os::raw::c_int }
        }
        _ => { }
    }
    /* fallthrough */
    if (*writer).indent != 0 && (*writer).doindent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        sum += count;
        (*writer).doindent = 1 as std::os::raw::c_int
    } else { (*writer).doindent = 1 as std::os::raw::c_int }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"</\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   (*p).name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b">\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatRaw:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted raw xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatRaw(xmlTextWriterPtr writer, const char *format,
//                             ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatRaw(writer, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatRaw:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted raw xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatRaw(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut format:
                                                          *const std::os::raw::c_char,
                                                      mut argptr:
                                                          ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteRaw(writer, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteRawLen:
 * @writer:  the xmlTextWriterPtr
 * @content:  text string
 * @len:  length of the text string
 *
 * Write an xml text.
 * TODO: what about entities and special chars??
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteRawLen(mut writer:
                                                      xmlTextWriterPtr,
                                                  mut content: *const xmlChar,
                                                  mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterWriteRawLen : invalid writer!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    if content.is_null() || len < 0 as std::os::raw::c_int {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterWriteRawLen : invalid content!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        count = xmlTextWriterHandleStateDependencies(writer, p);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    if (*writer).indent != 0 { (*writer).doindent = 0 as std::os::raw::c_int }
    if !content.is_null() {
        count =
            xmlOutputBufferWrite((*writer).out, len,
                                 content as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    return sum;
}
/* *
 * xmlTextWriterWriteRaw:
 * @writer:  the xmlTextWriterPtr
 * @content:  text string
 *
 * Write a raw xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteRaw(mut writer: xmlTextWriterPtr,
                                               mut content: *const xmlChar)
 -> std::os::raw::c_int {
    return xmlTextWriterWriteRawLen(writer, content, xmlStrlen(content));
}
/* *
 * xmlTextWriterWriteFormatString:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatString(xmlTextWriterPtr writer, const char *format,
//                                ...)
// {
//     int rc;
//     va_list ap;
//     if ((writer == NULL) || (format == NULL))
//         return -1;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatString(writer, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatString:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatString(mut writer:
                                                             xmlTextWriterPtr,
                                                         mut format:
                                                             *const std::os::raw::c_char,
                                                         mut argptr:
                                                             ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() || format.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteString(writer, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteString:
 * @writer:  the xmlTextWriterPtr
 * @content:  text string
 *
 * Write an xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteString(mut writer:
                                                      xmlTextWriterPtr,
                                                  mut content: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() || content.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    buf = content as *mut xmlChar;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            match (*p).state as std::os::raw::c_uint {
                1 | 3 => {
                    buf = xmlEncodeSpecialChars(0 as *const xmlDoc, content)
                }
                2 => {
                    buf = 0 as *mut xmlChar;
                    xmlBufAttrSerializeTxtContent((*(*writer).out).buffer,
                                                  (*writer).doc,
                                                  0 as xmlAttrPtr, content);
                }
                _ => { }
            }
        }
    }
    if !buf.is_null() {
        count = xmlTextWriterWriteRaw(writer, buf);
        if buf != content as *mut xmlChar {
            /* buf was allocated by us, so free it */
            xmlFree.expect("non-null function pointer")(buf as
                                                            *mut std::os::raw::c_void);
        }
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    return sum;
}
/* *
 * xmlOutputBufferWriteBase64:
 * @out: the xmlOutputBufferPtr
 * @data:   binary data
 * @len:  the number of bytes to encode
 *
 * Write base64 encoded data to an xmlOutputBuffer.
 * Adapted from John Walker's base64.c (http://www.fourmilab.ch/).
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
unsafe extern "C" fn xmlOutputBufferWriteBase64(mut out: xmlOutputBufferPtr,
                                                mut len: std::os::raw::c_int,
                                                mut data:
                                                    *const std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    static mut dtable: [std::os::raw::c_uchar; 64] =
        ['A' as i32 as std::os::raw::c_uchar, 'B' as i32 as std::os::raw::c_uchar,
         'C' as i32 as std::os::raw::c_uchar, 'D' as i32 as std::os::raw::c_uchar,
         'E' as i32 as std::os::raw::c_uchar, 'F' as i32 as std::os::raw::c_uchar,
         'G' as i32 as std::os::raw::c_uchar, 'H' as i32 as std::os::raw::c_uchar,
         'I' as i32 as std::os::raw::c_uchar, 'J' as i32 as std::os::raw::c_uchar,
         'K' as i32 as std::os::raw::c_uchar, 'L' as i32 as std::os::raw::c_uchar,
         'M' as i32 as std::os::raw::c_uchar, 'N' as i32 as std::os::raw::c_uchar,
         'O' as i32 as std::os::raw::c_uchar, 'P' as i32 as std::os::raw::c_uchar,
         'Q' as i32 as std::os::raw::c_uchar, 'R' as i32 as std::os::raw::c_uchar,
         'S' as i32 as std::os::raw::c_uchar, 'T' as i32 as std::os::raw::c_uchar,
         'U' as i32 as std::os::raw::c_uchar, 'V' as i32 as std::os::raw::c_uchar,
         'W' as i32 as std::os::raw::c_uchar, 'X' as i32 as std::os::raw::c_uchar,
         'Y' as i32 as std::os::raw::c_uchar, 'Z' as i32 as std::os::raw::c_uchar,
         'a' as i32 as std::os::raw::c_uchar, 'b' as i32 as std::os::raw::c_uchar,
         'c' as i32 as std::os::raw::c_uchar, 'd' as i32 as std::os::raw::c_uchar,
         'e' as i32 as std::os::raw::c_uchar, 'f' as i32 as std::os::raw::c_uchar,
         'g' as i32 as std::os::raw::c_uchar, 'h' as i32 as std::os::raw::c_uchar,
         'i' as i32 as std::os::raw::c_uchar, 'j' as i32 as std::os::raw::c_uchar,
         'k' as i32 as std::os::raw::c_uchar, 'l' as i32 as std::os::raw::c_uchar,
         'm' as i32 as std::os::raw::c_uchar, 'n' as i32 as std::os::raw::c_uchar,
         'o' as i32 as std::os::raw::c_uchar, 'p' as i32 as std::os::raw::c_uchar,
         'q' as i32 as std::os::raw::c_uchar, 'r' as i32 as std::os::raw::c_uchar,
         's' as i32 as std::os::raw::c_uchar, 't' as i32 as std::os::raw::c_uchar,
         'u' as i32 as std::os::raw::c_uchar, 'v' as i32 as std::os::raw::c_uchar,
         'w' as i32 as std::os::raw::c_uchar, 'x' as i32 as std::os::raw::c_uchar,
         'y' as i32 as std::os::raw::c_uchar, 'z' as i32 as std::os::raw::c_uchar,
         '0' as i32 as std::os::raw::c_uchar, '1' as i32 as std::os::raw::c_uchar,
         '2' as i32 as std::os::raw::c_uchar, '3' as i32 as std::os::raw::c_uchar,
         '4' as i32 as std::os::raw::c_uchar, '5' as i32 as std::os::raw::c_uchar,
         '6' as i32 as std::os::raw::c_uchar, '7' as i32 as std::os::raw::c_uchar,
         '8' as i32 as std::os::raw::c_uchar, '9' as i32 as std::os::raw::c_uchar,
         '+' as i32 as std::os::raw::c_uchar, '/' as i32 as std::os::raw::c_uchar];
    let mut i: std::os::raw::c_int = 0;
    let mut linelen: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if out.is_null() || len < 0 as std::os::raw::c_int || data.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    linelen = 0 as std::os::raw::c_int;
    sum = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    loop  {
        let mut igroup: [std::os::raw::c_uchar; 3] = [0; 3];
        let mut ogroup: [std::os::raw::c_uchar; 4] = [0; 4];
        let mut c: std::os::raw::c_int = 0;
        let mut n: std::os::raw::c_int = 0;
        igroup[2 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        igroup[1 as std::os::raw::c_int as usize] = igroup[2 as std::os::raw::c_int as usize];
        igroup[0 as std::os::raw::c_int as usize] = igroup[1 as std::os::raw::c_int as usize];
        n = 0 as std::os::raw::c_int;
        while n < 3 as std::os::raw::c_int && i < len {
            c = *data.offset(i as isize) as std::os::raw::c_int;
            igroup[n as usize] = c as std::os::raw::c_uchar;
            n += 1;
            i += 1
        }
        if n > 0 as std::os::raw::c_int {
            ogroup[0 as std::os::raw::c_int as usize] =
                dtable[(igroup[0 as std::os::raw::c_int as usize] as std::os::raw::c_int >>
                            2 as std::os::raw::c_int) as usize];
            ogroup[1 as std::os::raw::c_int as usize] =
                dtable[((igroup[0 as std::os::raw::c_int as usize] as std::os::raw::c_int &
                             3 as std::os::raw::c_int) << 4 as std::os::raw::c_int |
                            igroup[1 as std::os::raw::c_int as usize] as std::os::raw::c_int
                                >> 4 as std::os::raw::c_int) as usize];
            ogroup[2 as std::os::raw::c_int as usize] =
                dtable[((igroup[1 as std::os::raw::c_int as usize] as std::os::raw::c_int &
                             0xf as std::os::raw::c_int) << 2 as std::os::raw::c_int |
                            igroup[2 as std::os::raw::c_int as usize] as std::os::raw::c_int
                                >> 6 as std::os::raw::c_int) as usize];
            ogroup[3 as std::os::raw::c_int as usize] =
                dtable[(igroup[2 as std::os::raw::c_int as usize] as std::os::raw::c_int &
                            0x3f as std::os::raw::c_int) as usize];
            if n < 3 as std::os::raw::c_int {
                ogroup[3 as std::os::raw::c_int as usize] =
                    '=' as i32 as std::os::raw::c_uchar;
                if n < 2 as std::os::raw::c_int {
                    ogroup[2 as std::os::raw::c_int as usize] =
                        '=' as i32 as std::os::raw::c_uchar
                }
            }
            if linelen >= 72 as std::os::raw::c_int {
                count =
                    xmlOutputBufferWrite(out, 2 as std::os::raw::c_int,
                                         b"\r\n\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
                sum += count;
                linelen = 0 as std::os::raw::c_int
            }
            count =
                xmlOutputBufferWrite(out, 4 as std::os::raw::c_int,
                                     ogroup.as_mut_ptr() as
                                         *const std::os::raw::c_char);
            if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
            sum += count;
            linelen += 4 as std::os::raw::c_int
        }
        if i >= len { break ; }
    }
    return sum;
}
/* *
 * xmlTextWriterWriteBase64:
 * @writer: the xmlTextWriterPtr
 * @data:   binary data
 * @start:  the position within the data of the first byte to encode
 * @len:  the number of bytes to encode
 *
 * Write an base64 encoded xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteBase64(mut writer:
                                                      xmlTextWriterPtr,
                                                  mut data:
                                                      *const std::os::raw::c_char,
                                                  mut start: std::os::raw::c_int,
                                                  mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || data.is_null() || start < 0 as std::os::raw::c_int ||
           len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            count = xmlTextWriterHandleStateDependencies(writer, p);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
    }
    if (*writer).indent != 0 { (*writer).doindent = 0 as std::os::raw::c_int }
    count =
        xmlOutputBufferWriteBase64((*writer).out, len,
                                   (data as
                                        *mut std::os::raw::c_uchar).offset(start as
                                                                       isize));
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlOutputBufferWriteBinHex:
 * @out: the xmlOutputBufferPtr
 * @data:   binary data
 * @len:  the number of bytes to encode
 *
 * Write hqx encoded data to an xmlOutputBuffer.
 * ::todo
 *
 * Returns the bytes written (may be 0 because of buffering)
 * or -1 in case of error
 */
unsafe extern "C" fn xmlOutputBufferWriteBinHex(mut out: xmlOutputBufferPtr,
                                                mut len: std::os::raw::c_int,
                                                mut data:
                                                    *const std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    static mut hex: [std::os::raw::c_char; 16] =
        ['0' as i32 as std::os::raw::c_char, '1' as i32 as std::os::raw::c_char,
         '2' as i32 as std::os::raw::c_char, '3' as i32 as std::os::raw::c_char,
         '4' as i32 as std::os::raw::c_char, '5' as i32 as std::os::raw::c_char,
         '6' as i32 as std::os::raw::c_char, '7' as i32 as std::os::raw::c_char,
         '8' as i32 as std::os::raw::c_char, '9' as i32 as std::os::raw::c_char,
         'A' as i32 as std::os::raw::c_char, 'B' as i32 as std::os::raw::c_char,
         'C' as i32 as std::os::raw::c_char, 'D' as i32 as std::os::raw::c_char,
         'E' as i32 as std::os::raw::c_char, 'F' as i32 as std::os::raw::c_char];
    let mut i: std::os::raw::c_int = 0;
    if out.is_null() || data.is_null() || len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < len {
        count =
            xmlOutputBufferWrite(out, 1 as std::os::raw::c_int,
                                 &mut *hex.as_mut_ptr().offset((*data.offset(i
                                                                                 as
                                                                                 isize)
                                                                    as
                                                                    std::os::raw::c_int
                                                                    >>
                                                                    4 as
                                                                        std::os::raw::c_int)
                                                                   as isize)
                                     as *mut std::os::raw::c_char as
                                     *const std::os::raw::c_char);
        if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite(out, 1 as std::os::raw::c_int,
                                 &mut *hex.as_mut_ptr().offset((*data.offset(i
                                                                                 as
                                                                                 isize)
                                                                    as
                                                                    std::os::raw::c_int
                                                                    &
                                                                    0xf as
                                                                        std::os::raw::c_int)
                                                                   as isize)
                                     as *mut std::os::raw::c_char as
                                     *const std::os::raw::c_char);
        if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        sum += count;
        i += 1
    }
    return sum;
}
/* *
 * xmlTextWriterWriteBinHex:
 * @writer: the xmlTextWriterPtr
 * @data:   binary data
 * @start:  the position within the data of the first byte to encode
 * @len:  the number of bytes to encode
 *
 * Write a BinHex encoded xml text.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteBinHex(mut writer:
                                                      xmlTextWriterPtr,
                                                  mut data:
                                                      *const std::os::raw::c_char,
                                                  mut start: std::os::raw::c_int,
                                                  mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || data.is_null() || start < 0 as std::os::raw::c_int ||
           len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            count = xmlTextWriterHandleStateDependencies(writer, p);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
    }
    if (*writer).indent != 0 { (*writer).doindent = 0 as std::os::raw::c_int }
    count =
        xmlOutputBufferWriteBinHex((*writer).out, len,
                                   (data as
                                        *mut std::os::raw::c_uchar).offset(start as
                                                                       isize));
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * Attributes
 */
/* *
 * xmlTextWriterStartAttribute:
 * @writer:  the xmlTextWriterPtr
 * @name:  element name
 *
 * Start an xml attribute.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartAttribute(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    match (*p).state as std::os::raw::c_uint {
        2 => {
            count = xmlTextWriterEndAttribute(writer);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        1 => { }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    /* fallthrough */
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b" \x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"=\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                             &mut (*writer).qchar);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    (*p).state = XML_TEXTWRITER_ATTRIBUTE;
    return sum;
}
/* *
 * xmlTextWriterStartAttributeNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix or NULL
 * @name:  element local name
 * @namespaceURI:  namespace URI or NULL
 *
 * Start an xml attribute with namespace support.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartAttributeNS(mut writer:
                                                           xmlTextWriterPtr,
                                                       mut prefix:
                                                           *const xmlChar,
                                                       mut name:
                                                           *const xmlChar,
                                                       mut namespaceURI:
                                                           *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut p: *mut xmlTextWriterNsStackEntry =
        0 as *mut xmlTextWriterNsStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    /* Handle namespace first in case of error */
    if !namespaceURI.is_null() {
        let mut nsentry: xmlTextWriterNsStackEntry =
            xmlTextWriterNsStackEntry{prefix: 0 as *mut xmlChar,
                                      uri: 0 as *mut xmlChar,
                                      elem: 0 as *mut xmlLink,};
        let mut curns: *mut xmlTextWriterNsStackEntry =
            0 as *mut xmlTextWriterNsStackEntry;
        buf =
            xmlStrdup(b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        if !prefix.is_null() {
            buf =
                xmlStrcat(buf,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            buf = xmlStrcat(buf, prefix)
        }
        nsentry.prefix = buf;
        nsentry.uri = namespaceURI as *mut xmlChar;
        nsentry.elem = xmlListFront((*writer).nodes);
        curns =
            xmlListSearch((*writer).nsstack,
                          &mut nsentry as *mut xmlTextWriterNsStackEntry as
                              *mut std::os::raw::c_void) as
                *mut xmlTextWriterNsStackEntry;
        if !curns.is_null() {
            xmlFree.expect("non-null function pointer")(buf as
                                                            *mut std::os::raw::c_void);
            if xmlStrcmp((*curns).uri, namespaceURI) == 0 as std::os::raw::c_int {
                /* Namespace already defined on element skip */
                buf = 0 as *mut xmlChar
            } else {
                /* Prefix mismatch so error out */
                return -(1 as std::os::raw::c_int)
            }
        }
        /* Do not add namespace decl to list - it is already there */
        if !buf.is_null() {
            p =
                xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterNsStackEntry>()
                                                                  as
                                                                  std::os::raw::c_ulong)
                    as *mut xmlTextWriterNsStackEntry;
            if p.is_null() {
                xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                                b"xmlTextWriterStartAttributeNS : out of memory!\n\x00"
                                    as *const u8 as *const std::os::raw::c_char);
                return -(1 as std::os::raw::c_int)
            }
            (*p).prefix = buf;
            (*p).uri = xmlStrdup(namespaceURI);
            if (*p).uri.is_null() {
                xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                                b"xmlTextWriterStartAttributeNS : out of memory!\n\x00"
                                    as *const u8 as *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(p as
                                                                *mut std::os::raw::c_void);
                return -(1 as std::os::raw::c_int)
            }
            (*p).elem = xmlListFront((*writer).nodes);
            xmlListPushFront((*writer).nsstack, p as *mut std::os::raw::c_void);
        }
    }
    buf = 0 as *mut xmlChar;
    if !prefix.is_null() {
        buf = xmlStrdup(prefix);
        buf =
            xmlStrcat(buf,
                      b":\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar)
    }
    buf = xmlStrcat(buf, name);
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartAttribute(writer, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndAttribute:
 * @writer:  the xmlTextWriterPtr
 *
 * End the current xml element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndAttribute(mut writer:
                                                       xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    match (*p).state as std::os::raw::c_uint {
        2 => {
            (*p).state = XML_TEXTWRITER_NAME;
            count =
                xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                     &mut (*writer).qchar);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    return sum;
}
/* *
 * xmlTextWriterWriteFormatAttribute:
 * @writer:  the xmlTextWriterPtr
 * @name:  attribute name
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted xml attribute.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatAttribute(xmlTextWriterPtr writer,
//                                   const xmlChar * name, const char *format,
//                                   ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatAttribute(writer, name, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatAttribute:
 * @writer:  the xmlTextWriterPtr
 * @name:  attribute name
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml attribute.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatAttribute(mut writer:
                                                                xmlTextWriterPtr,
                                                            mut name:
                                                                *const xmlChar,
                                                            mut format:
                                                                *const std::os::raw::c_char,
                                                            mut argptr:
                                                                ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteAttribute(writer, name, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteAttribute:
 * @writer:  the xmlTextWriterPtr
 * @name:  attribute name
 * @content:  attribute content
 *
 * Write an xml attribute.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteAttribute(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut name: *const xmlChar,
                                                     mut content:
                                                         *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartAttribute(writer, name);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndAttribute(writer);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteFormatAttributeNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix
 * @name:  attribute local name
 * @namespaceURI:  namespace URI
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted xml attribute.with namespace support
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatAttributeNS(xmlTextWriterPtr writer,
//                                     const xmlChar * prefix,
//                                     const xmlChar * name,
//                                     const xmlChar * namespaceURI,
//                                     const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatAttributeNS(writer, prefix, name,
//                                               namespaceURI, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatAttributeNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix
 * @name:  attribute local name
 * @namespaceURI:  namespace URI
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml attribute.with namespace support
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatAttributeNS(mut writer:
                                                                  xmlTextWriterPtr,
                                                              mut prefix:
                                                                  *const xmlChar,
                                                              mut name:
                                                                  *const xmlChar,
                                                              mut namespaceURI:
                                                                  *const xmlChar,
                                                              mut format:
                                                                  *const std::os::raw::c_char,
                                                              mut argptr:
                                                                  ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc =
        xmlTextWriterWriteAttributeNS(writer, prefix, name, namespaceURI,
                                      buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteAttributeNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix
 * @name:  attribute local name
 * @namespaceURI:  namespace URI
 * @content:  attribute content
 *
 * Write an xml attribute.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteAttributeNS(mut writer:
                                                           xmlTextWriterPtr,
                                                       mut prefix:
                                                           *const xmlChar,
                                                       mut name:
                                                           *const xmlChar,
                                                       mut namespaceURI:
                                                           *const xmlChar,
                                                       mut content:
                                                           *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartAttributeNS(writer, prefix, name, namespaceURI);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndAttribute(writer);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteFormatElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  element name
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted xml element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatElement(xmlTextWriterPtr writer,
//                                 const xmlChar * name, const char *format,
//                                 ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatElement(writer, name, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  element name
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatElement(mut writer:
                                                              xmlTextWriterPtr,
                                                          mut name:
                                                              *const xmlChar,
                                                          mut format:
                                                              *const std::os::raw::c_char,
                                                          mut argptr:
                                                              ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteElement(writer, name, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  element name
 * @content:  element content
 *
 * Write an xml element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteElement(mut writer:
                                                       xmlTextWriterPtr,
                                                   mut name: *const xmlChar,
                                                   mut content:
                                                       *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartElement(writer, name);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !content.is_null() {
        count = xmlTextWriterWriteString(writer, content);
        if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count = xmlTextWriterEndElement(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteFormatElementNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix
 * @name:  element local name
 * @namespaceURI:  namespace URI
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted xml element with namespace support.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatElementNS(xmlTextWriterPtr writer,
//                                   const xmlChar * prefix,
//                                   const xmlChar * name,
//                                   const xmlChar * namespaceURI,
//                                   const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatElementNS(writer, prefix, name,
//                                             namespaceURI, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatElementNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix
 * @name:  element local name
 * @namespaceURI:  namespace URI
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml element with namespace support.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatElementNS(mut writer:
                                                                xmlTextWriterPtr,
                                                            mut prefix:
                                                                *const xmlChar,
                                                            mut name:
                                                                *const xmlChar,
                                                            mut namespaceURI:
                                                                *const xmlChar,
                                                            mut format:
                                                                *const std::os::raw::c_char,
                                                            mut argptr:
                                                                ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteElementNS(writer, prefix, name, namespaceURI, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteElementNS:
 * @writer:  the xmlTextWriterPtr
 * @prefix:  namespace prefix
 * @name:  element local name
 * @namespaceURI:  namespace URI
 * @content:  element content
 *
 * Write an xml element with namespace support.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteElementNS(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut prefix:
                                                         *const xmlChar,
                                                     mut name: *const xmlChar,
                                                     mut namespaceURI:
                                                         *const xmlChar,
                                                     mut content:
                                                         *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartElementNS(writer, prefix, name, namespaceURI);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndElement(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * PI's
 */
/* *
 * xmlTextWriterStartPI:
 * @writer:  the xmlTextWriterPtr
 * @target:  PI target
 *
 * Start an xml PI.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartPI(mut writer: xmlTextWriterPtr,
                                              mut target: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || target.is_null() ||
           *target as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    if xmlStrcasecmp(target,
                     b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 as std::os::raw::c_int {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterStartPI : target name [Xx][Mm][Ll] is reserved for xml standardization!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            let mut current_block_24: u64;
            match (*p).state as std::os::raw::c_uint {
                2 => {
                    count = xmlTextWriterEndAttribute(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    current_block_24 = 5768334474942797936;
                }
                1 => { current_block_24 = 5768334474942797936; }
                0 | 3 | 7 => { current_block_24 = 13550086250199790493; }
                4 | 5 => {
                    xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                                    b"xmlTextWriterStartPI : nested PI!\n\x00"
                                        as *const u8 as *const std::os::raw::c_char);
                    return -(1 as std::os::raw::c_int)
                }
                _ => { return -(1 as std::os::raw::c_int) }
            }
            match current_block_24 {
                5768334474942797936 =>
                /* fallthrough */
                /* Output namespace declarations */
                {
                    count = xmlTextWriterOutputNSDecl(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b">\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    (*p).state = XML_TEXTWRITER_TEXT
                }
                _ => { }
            }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartPI : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = xmlStrdup(target);
    if (*p).name.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartPI : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    (*p).state = XML_TEXTWRITER_PI;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<?\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   (*p).name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndPI:
 * @writer:  the xmlTextWriterPtr
 *
 * End the current xml PI.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndPI(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return 0 as std::os::raw::c_int }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return 0 as std::os::raw::c_int }
    sum = 0 as std::os::raw::c_int;
    match (*p).state as std::os::raw::c_uint {
        4 | 5 => {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b"?>\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatPI:
 * @writer:  the xmlTextWriterPtr
 * @target:  PI target
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted PI.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatPI(xmlTextWriterPtr writer, const xmlChar * target,
//                            const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatPI(writer, target, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatPI:
 * @writer:  the xmlTextWriterPtr
 * @target:  PI target
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml PI.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatPI(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut target:
                                                         *const xmlChar,
                                                     mut format:
                                                         *const std::os::raw::c_char,
                                                     mut argptr:
                                                         ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWritePI(writer, target, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWritePI:
 * @writer:  the xmlTextWriterPtr
 * @target:  PI target
 * @content:  PI content
 *
 * Write an xml PI.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWritePI(mut writer: xmlTextWriterPtr,
                                              mut target: *const xmlChar,
                                              mut content: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartPI(writer, target);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !content.is_null() {
        count = xmlTextWriterWriteString(writer, content);
        if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count = xmlTextWriterEndPI(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteProcessingInstruction:
 *
 * This macro maps to xmlTextWriterWritePI
 */
/*
 * CDATA
 */
/* *
 * xmlTextWriterStartCDATA:
 * @writer:  the xmlTextWriterPtr
 *
 * Start an xml CDATA section.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartCDATA(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            let mut current_block_20: u64;
            match (*p).state as std::os::raw::c_uint {
                0 | 3 | 4 | 5 => { current_block_20 = 13472856163611868459; }
                2 => {
                    count = xmlTextWriterEndAttribute(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    current_block_20 = 6861481918339214690;
                }
                1 => { current_block_20 = 6861481918339214690; }
                6 => {
                    xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                                    b"xmlTextWriterStartCDATA : CDATA not allowed in this context!\n\x00"
                                        as *const u8 as *const std::os::raw::c_char);
                    return -(1 as std::os::raw::c_int)
                }
                _ => { return -(1 as std::os::raw::c_int) }
            }
            match current_block_20 {
                6861481918339214690 =>
                /* fallthrough */
                /* Output namespace declarations */
                {
                    count = xmlTextWriterOutputNSDecl(writer);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b">\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    (*p).state = XML_TEXTWRITER_TEXT
                }
                _ => { }
            }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartCDATA : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = 0 as *mut xmlChar;
    (*p).state = XML_TEXTWRITER_CDATA;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<![CDATA[\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndCDATA:
 * @writer:  the xmlTextWriterPtr
 *
 * End an xml CDATA section.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndCDATA(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    match (*p).state as std::os::raw::c_uint {
        6 => {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b"]]>\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatCDATA:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted xml CDATA.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatCDATA(xmlTextWriterPtr writer, const char *format,
//                               ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatCDATA(writer, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatCDATA:
 * @writer:  the xmlTextWriterPtr
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted xml CDATA.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatCDATA(mut writer:
                                                            xmlTextWriterPtr,
                                                        mut format:
                                                            *const std::os::raw::c_char,
                                                        mut argptr:
                                                            ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteCDATA(writer, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteCDATA:
 * @writer:  the xmlTextWriterPtr
 * @content:  CDATA content
 *
 * Write an xml CDATA.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteCDATA(mut writer: xmlTextWriterPtr,
                                                 mut content: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartCDATA(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !content.is_null() {
        count = xmlTextWriterWriteString(writer, content);
        if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count = xmlTextWriterEndCDATA(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * DTD
 */
/* *
 * xmlTextWriterStartDTD:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 *
 * Start an xml DTD.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartDTD(mut writer: xmlTextWriterPtr,
                                               mut name: *const xmlChar,
                                               mut pubid: *const xmlChar,
                                               mut sysid: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() && !xmlLinkGetData(lk).is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterStartDTD : DTD allowed only in prolog!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTD : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = xmlStrdup(name);
    if (*p).name.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTD : out of memory!\n\x00" as
                            *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    (*p).state = XML_TEXTWRITER_DTD;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<!DOCTYPE \x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !pubid.is_null() {
        if sysid.is_null() {
            xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                            b"xmlTextWriterStartDTD : system identifier needed!\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        if (*writer).indent != 0 {
            count =
                xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                     b"\n\x00" as *const u8 as
                                         *const std::os::raw::c_char)
        } else {
            count =
                xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                     b" \x00" as *const u8 as
                                         *const std::os::raw::c_char)
        }
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"PUBLIC \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       pubid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    if !sysid.is_null() {
        if pubid.is_null() {
            if (*writer).indent != 0 {
                count =
                    xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                         b"\n\x00" as *const u8 as
                                             *const std::os::raw::c_char)
            } else {
                count =
                    xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                         b" \x00" as *const u8 as
                                             *const std::os::raw::c_char)
            }
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count;
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b"SYSTEM \x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        } else {
            if (*writer).indent != 0 {
                count =
                    xmlOutputBufferWriteString((*writer).out,
                                               b"\n       \x00" as *const u8
                                                   as *const std::os::raw::c_char)
            } else {
                count =
                    xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                         b" \x00" as *const u8 as
                                             *const std::os::raw::c_char)
            }
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       sysid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    return sum;
}
/* *
 * xmlTextWriterEndDTD:
 * @writer:  the xmlTextWriterPtr
 *
 * End an xml DTD.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndDTD(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut loop_0: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    loop_0 = 1 as std::os::raw::c_int;
    let mut current_block_25: u64;
    while loop_0 != 0 {
        lk = xmlListFront((*writer).nodes);
        if lk.is_null() { break ; }
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if p.is_null() { break ; }
        match (*p).state as std::os::raw::c_uint {
            8 => {
                count =
                    xmlOutputBufferWriteString((*writer).out,
                                               b"]\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count;
                current_block_25 = 15755521303720483476;
            }
            7 => { current_block_25 = 15755521303720483476; }
            9 | 10 => {
                count = xmlTextWriterEndDTDElement(writer);
                current_block_25 = 17500079516916021833;
            }
            11 | 12 => {
                count = xmlTextWriterEndDTDAttlist(writer);
                current_block_25 = 17500079516916021833;
            }
            13 | 15 | 14 => {
                count = xmlTextWriterEndDTDEntity(writer);
                current_block_25 = 17500079516916021833;
            }
            16 => {
                count = xmlTextWriterEndComment(writer);
                current_block_25 = 17500079516916021833;
            }
            _ => { loop_0 = 0 as std::os::raw::c_int; continue ; }
        }
        match current_block_25 {
            15755521303720483476 =>
            /* fallthrough */
            {
                count =
                    xmlOutputBufferWriteString((*writer).out,
                                               b">\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                if (*writer).indent != 0 {
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b"\n\x00" as *const u8 as
                                                       *const std::os::raw::c_char)
                }
                xmlListPopFront((*writer).nodes);
            }
            _ => { }
        }
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    return sum;
}
/* *
 * xmlTextWriterWriteFormatDTD:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a DTD with a formatted markup declarations part.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatDTD(xmlTextWriterPtr writer,
//                             const xmlChar * name,
//                             const xmlChar * pubid,
//                             const xmlChar * sysid, const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatDTD(writer, name, pubid, sysid, format,
//                                       ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatDTD:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a DTD with a formatted markup declarations part.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatDTD(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut name:
                                                          *const xmlChar,
                                                      mut pubid:
                                                          *const xmlChar,
                                                      mut sysid:
                                                          *const xmlChar,
                                                      mut format:
                                                          *const std::os::raw::c_char,
                                                      mut argptr:
                                                          ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteDTD(writer, name, pubid, sysid, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteDTD:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 * @subset:  string content of the DTD
 *
 * Write a DTD.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTD(mut writer: xmlTextWriterPtr,
                                               mut name: *const xmlChar,
                                               mut pubid: *const xmlChar,
                                               mut sysid: *const xmlChar,
                                               mut subset: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartDTD(writer, name, pubid, sysid);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !subset.is_null() {
        count = xmlTextWriterWriteString(writer, subset);
        if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count = xmlTextWriterEndDTD(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteDocType:
 *
 * this macro maps to xmlTextWriterWriteDTD
 */
/*
 * DTD element definition
 */
/* *
 * xmlTextWriterStartDTDElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD element
 *
 * Start an xml DTD element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartDTDElement(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut name:
                                                          *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if !p.is_null() {
        match (*p).state as std::os::raw::c_uint {
            7 => {
                count =
                    xmlOutputBufferWriteString((*writer).out,
                                               b" [\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count;
                if (*writer).indent != 0 {
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b"\n\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count
                }
                (*p).state = XML_TEXTWRITER_DTD_TEXT
            }
            8 | 0 => { }
            _ => { return -(1 as std::os::raw::c_int) }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTDElement : out of memory!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = xmlStrdup(name);
    if (*p).name.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTDElement : out of memory!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    (*p).state = XML_TEXTWRITER_DTD_ELEM;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    if (*writer).indent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<!ELEMENT \x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndDTDElement:
 * @writer:  the xmlTextWriterPtr
 *
 * End an xml DTD element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndDTDElement(mut writer:
                                                        xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    match (*p).state as std::os::raw::c_uint {
        9 | 10 => {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b">\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatDTDElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD element
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted DTD element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatDTDElement(xmlTextWriterPtr writer,
//                                    const xmlChar * name,
//                                    const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatDTDElement(writer, name, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatDTDElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD element
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted DTD element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatDTDElement(mut writer:
                                                                 xmlTextWriterPtr,
                                                             mut name:
                                                                 *const xmlChar,
                                                             mut format:
                                                                 *const std::os::raw::c_char,
                                                             mut argptr:
                                                                 ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteDTDElement(writer, name, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteDTDElement:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD element
 * @content:  content of the element
 *
 * Write a DTD element.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDElement(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut name:
                                                          *const xmlChar,
                                                      mut content:
                                                          *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if content.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartDTDElement(writer, name);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndDTDElement(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * DTD attribute list definition
 */
/* *
 * xmlTextWriterStartDTDAttlist:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD ATTLIST
 *
 * Start an xml DTD ATTLIST.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartDTDAttlist(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut name:
                                                          *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if !p.is_null() {
        match (*p).state as std::os::raw::c_uint {
            7 => {
                count =
                    xmlOutputBufferWriteString((*writer).out,
                                               b" [\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count;
                if (*writer).indent != 0 {
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b"\n\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count
                }
                (*p).state = XML_TEXTWRITER_DTD_TEXT
            }
            8 | 0 => { }
            _ => { return -(1 as std::os::raw::c_int) }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTDAttlist : out of memory!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = xmlStrdup(name);
    if (*p).name.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTDAttlist : out of memory!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    (*p).state = XML_TEXTWRITER_DTD_ATTL;
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    if (*writer).indent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<!ATTLIST \x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndDTDAttlist:
 * @writer:  the xmlTextWriterPtr
 *
 * End an xml DTD attribute list.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndDTDAttlist(mut writer:
                                                        xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    match (*p).state as std::os::raw::c_uint {
        11 | 12 => {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b">\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatDTDAttlist:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD ATTLIST
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted DTD ATTLIST.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatDTDAttlist(xmlTextWriterPtr writer,
//                                    const xmlChar * name,
//                                    const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatDTDAttlist(writer, name, format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatDTDAttlist:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD ATTLIST
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted DTD ATTLIST.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatDTDAttlist(mut writer:
                                                                 xmlTextWriterPtr,
                                                             mut name:
                                                                 *const xmlChar,
                                                             mut format:
                                                                 *const std::os::raw::c_char,
                                                             mut argptr:
                                                                 ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteDTDAttlist(writer, name, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteDTDAttlist:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the DTD ATTLIST
 * @content:  content of the ATTLIST
 *
 * Write a DTD ATTLIST.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDAttlist(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut name:
                                                          *const xmlChar,
                                                      mut content:
                                                          *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if content.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartDTDAttlist(writer, name);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndDTDAttlist(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * DTD entity definition
 */
/* *
 * xmlTextWriterStartDTDEntity:
 * @writer:  the xmlTextWriterPtr
 * @pe:  TRUE if this is a parameter entity, FALSE if not
 * @name:  the name of the DTD ATTLIST
 *
 * Start an xml DTD ATTLIST.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterStartDTDEntity(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut pe: std::os::raw::c_int,
                                                     mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if !lk.is_null() {
        p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
        if !p.is_null() {
            match (*p).state as std::os::raw::c_uint {
                7 => {
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b" [\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count;
                    if (*writer).indent != 0 {
                        count =
                            xmlOutputBufferWriteString((*writer).out,
                                                       b"\n\x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char);
                        if count < 0 as std::os::raw::c_int {
                            return -(1 as std::os::raw::c_int)
                        }
                        sum += count
                    }
                    (*p).state = XML_TEXTWRITER_DTD_TEXT
                }
                8 | 0 => { }
                _ => { return -(1 as std::os::raw::c_int) }
            }
        }
    }
    p =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextWriterStackEntry>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlTextWriterStackEntry;
    if p.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTDElement : out of memory!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    (*p).name = xmlStrdup(name);
    if (*p).name.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_NO_MEMORY,
                        b"xmlTextWriterStartDTDElement : out of memory!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    if pe != 0 as std::os::raw::c_int {
        (*p).state = XML_TEXTWRITER_DTD_PENT
    } else { (*p).state = XML_TEXTWRITER_DTD_ENTY }
    xmlListPushFront((*writer).nodes, p as *mut std::os::raw::c_void);
    if (*writer).indent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<!ENTITY \x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if pe != 0 as std::os::raw::c_int {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"% \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterEndDTDEntity:
 * @writer:  the xmlTextWriterPtr
 *
 * End an xml DTD entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterEndDTDEntity(mut writer:
                                                       xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    match (*p).state as std::os::raw::c_uint {
        14 => {
            count =
                xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                     &mut (*writer).qchar);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        13 | 15 => { }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    /* Falls through. */
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b">\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if (*writer).indent != 0 {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b"\n\x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    xmlListPopFront((*writer).nodes);
    return sum;
}
/* *
 * xmlTextWriterWriteFormatDTDInternalEntity:
 * @writer:  the xmlTextWriterPtr
 * @pe:  TRUE if this is a parameter entity, FALSE if not
 * @name:  the name of the DTD entity
 * @format:  format string (see printf)
 * @...:  extra parameters for the format
 *
 * Write a formatted DTD internal entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
// int XMLCDECL
// xmlTextWriterWriteFormatDTDInternalEntity(xmlTextWriterPtr writer,
//                                           int pe,
//                                           const xmlChar * name,
//                                           const char *format, ...)
// {
//     int rc;
//     va_list ap;
//     va_start(ap, format);
//     rc = xmlTextWriterWriteVFormatDTDInternalEntity(writer, pe, name,
//                                                     format, ap);
//     va_end(ap);
//     return rc;
// }
/* *
 * xmlTextWriterWriteVFormatDTDInternalEntity:
 * @writer:  the xmlTextWriterPtr
 * @pe:  TRUE if this is a parameter entity, FALSE if not
 * @name:  the name of the DTD entity
 * @format:  format string (see printf)
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Write a formatted DTD internal entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteVFormatDTDInternalEntity(mut writer:
                                                                        xmlTextWriterPtr,
                                                                    mut pe:
                                                                        std::os::raw::c_int,
                                                                    mut name:
                                                                        *const xmlChar,
                                                                    mut format:
                                                                        *const std::os::raw::c_char,
                                                                    mut argptr:
                                                                        ::std::ffi::VaList)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    buf = xmlTextWriterVSprintf(format, argptr.as_va_list());
    if buf.is_null() { return -(1 as std::os::raw::c_int) }
    rc = xmlTextWriterWriteDTDInternalEntity(writer, pe, name, buf);
    xmlFree.expect("non-null function pointer")(buf as *mut std::os::raw::c_void);
    return rc;
}
/* *
 * xmlTextWriterWriteDTDEntity:
 * @writer:  the xmlTextWriterPtr
 * @pe:  TRUE if this is a parameter entity, FALSE if not
 * @name:  the name of the DTD entity
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 * @ndataid:  the xml notation name.
 * @content:  content of the entity
 *
 * Write a DTD entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDEntity(mut writer:
                                                         xmlTextWriterPtr,
                                                     mut pe: std::os::raw::c_int,
                                                     mut name: *const xmlChar,
                                                     mut pubid:
                                                         *const xmlChar,
                                                     mut sysid:
                                                         *const xmlChar,
                                                     mut ndataid:
                                                         *const xmlChar,
                                                     mut content:
                                                         *const xmlChar)
 -> std::os::raw::c_int {
    if content.is_null() && pubid.is_null() && sysid.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if pe != 0 as std::os::raw::c_int && !ndataid.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if pubid.is_null() && sysid.is_null() {
        return xmlTextWriterWriteDTDInternalEntity(writer, pe, name, content)
    }
    return xmlTextWriterWriteDTDExternalEntity(writer, pe, name, pubid, sysid,
                                               ndataid);
}
/* *
 * xmlTextWriterWriteDTDInternalEntity:
 * @writer:  the xmlTextWriterPtr
 * @pe:  TRUE if this is a parameter entity, FALSE if not
 * @name:  the name of the DTD entity
 * @content:  content of the entity
 *
 * Write a DTD internal entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDInternalEntity(mut writer:
                                                                 xmlTextWriterPtr,
                                                             mut pe:
                                                                 std::os::raw::c_int,
                                                             mut name:
                                                                 *const xmlChar,
                                                             mut content:
                                                                 *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if name.is_null() || *name as std::os::raw::c_int == '\u{0}' as i32 ||
           content.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartDTDEntity(writer, pe, name);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterWriteString(writer, content);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndDTDEntity(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteDTDExternalEntity:
 * @writer:  the xmlTextWriterPtr
 * @pe:  TRUE if this is a parameter entity, FALSE if not
 * @name:  the name of the DTD entity
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 * @ndataid:  the xml notation name.
 *
 * Write a DTD external entity. The entity must have been started with xmlTextWriterStartDTDEntity
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDExternalEntity(mut writer:
                                                                 xmlTextWriterPtr,
                                                             mut pe:
                                                                 std::os::raw::c_int,
                                                             mut name:
                                                                 *const xmlChar,
                                                             mut pubid:
                                                                 *const xmlChar,
                                                             mut sysid:
                                                                 *const xmlChar,
                                                             mut ndataid:
                                                                 *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    if pubid.is_null() && sysid.is_null() { return -(1 as std::os::raw::c_int) }
    if pe != 0 as std::os::raw::c_int && !ndataid.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    count = xmlTextWriterStartDTDEntity(writer, pe, name);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlTextWriterWriteDTDExternalEntityContents(writer, pubid, sysid,
                                                    ndataid);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count = xmlTextWriterEndDTDEntity(writer);
    if count == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/* *
 * xmlTextWriterWriteDTDExternalEntityContents:
 * @writer:  the xmlTextWriterPtr
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 * @ndataid:  the xml notation name.
 *
 * Write the contents of a DTD external entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDExternalEntityContents(mut writer:
                                                                         xmlTextWriterPtr,
                                                                     mut pubid:
                                                                         *const xmlChar,
                                                                     mut sysid:
                                                                         *const xmlChar,
                                                                     mut ndataid:
                                                                         *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterWriteDTDExternalEntityContents: xmlTextWriterPtr invalid!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() {
        xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                        b"xmlTextWriterWriteDTDExternalEntityContents: you must call xmlTextWriterStartDTDEntity before the call to this function!\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return -(1 as std::os::raw::c_int) }
    match (*p).state as std::os::raw::c_uint {
        13 => { }
        15 => {
            if !ndataid.is_null() {
                xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                                b"xmlTextWriterWriteDTDExternalEntityContents: notation not allowed with parameter entities!\n\x00"
                                    as *const u8 as *const std::os::raw::c_char);
                return -(1 as std::os::raw::c_int)
            }
        }
        _ => {
            xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                            b"xmlTextWriterWriteDTDExternalEntityContents: you must call xmlTextWriterStartDTDEntity before the call to this function!\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if !pubid.is_null() {
        if sysid.is_null() {
            xmlWriterErrMsg(writer, XML_ERR_INTERNAL_ERROR,
                            b"xmlTextWriterWriteDTDExternalEntityContents: system identifier needed!\n\x00"
                                as *const u8 as *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" PUBLIC \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       pubid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    if !sysid.is_null() {
        if pubid.is_null() {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b" SYSTEM\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       sysid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    if !ndataid.is_null() {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" NDATA \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       ndataid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    return sum;
}
/*
 * DTD notation definition
 */
/* *
 * xmlTextWriterWriteDTDNotation:
 * @writer:  the xmlTextWriterPtr
 * @name:  the name of the xml notation
 * @pubid:  the public identifier, which is an alternative to the system identifier
 * @sysid:  the system identifier, which is the URI of the DTD
 *
 * Write a DTD entity.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteDTDNotation(mut writer:
                                                           xmlTextWriterPtr,
                                                       mut name:
                                                           *const xmlChar,
                                                       mut pubid:
                                                           *const xmlChar,
                                                       mut sysid:
                                                           *const xmlChar)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if writer.is_null() || name.is_null() ||
           *name as std::os::raw::c_int == '\u{0}' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    sum = 0 as std::os::raw::c_int;
    lk = xmlListFront((*writer).nodes);
    if lk.is_null() { return -(1 as std::os::raw::c_int) }
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if !p.is_null() {
        match (*p).state as std::os::raw::c_uint {
            7 => {
                count =
                    xmlOutputBufferWriteString((*writer).out,
                                               b" [\x00" as *const u8 as
                                                   *const std::os::raw::c_char);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count;
                if (*writer).indent != 0 {
                    count =
                        xmlOutputBufferWriteString((*writer).out,
                                                   b"\n\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
                    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                    sum += count
                }
                (*p).state = XML_TEXTWRITER_DTD_TEXT
            }
            8 => { }
            _ => { return -(1 as std::os::raw::c_int) }
        }
    }
    if (*writer).indent != 0 {
        count = xmlTextWriterWriteIndent(writer);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b"<!NOTATION \x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   name as *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    if !pubid.is_null() {
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" PUBLIC \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       pubid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    if !sysid.is_null() {
        if pubid.is_null() {
            count =
                xmlOutputBufferWriteString((*writer).out,
                                           b" SYSTEM\x00" as *const u8 as
                                               *const std::os::raw::c_char);
            if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            sum += count
        }
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       b" \x00" as *const u8 as
                                           *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWriteString((*writer).out,
                                       sysid as *const std::os::raw::c_char);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count;
        count =
            xmlOutputBufferWrite((*writer).out, 1 as std::os::raw::c_int,
                                 &mut (*writer).qchar);
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    count =
        xmlOutputBufferWriteString((*writer).out,
                                   b">\x00" as *const u8 as
                                       *const std::os::raw::c_char);
    if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    sum += count;
    return sum;
}
/*
 * misc
 */
/* *
 * xmlTextWriterFlush:
 * @writer:  the xmlTextWriterPtr
 *
 * Flush the output buffer.
 *
 * Returns the bytes written (may be 0 because of buffering) or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterFlush(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    if (*writer).out.is_null() {
        count = 0 as std::os::raw::c_int
    } else { count = xmlOutputBufferFlush((*writer).out) }
    return count;
}
/* *
 * misc
 */
/* *
 * xmlFreeTextWriterStackEntry:
 * @lk:  the xmlLinkPtr
 *
 * Free callback for the xmlList.
 */
unsafe extern "C" fn xmlFreeTextWriterStackEntry(mut lk: xmlLinkPtr) {
    let mut p: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    p = xmlLinkGetData(lk) as *mut xmlTextWriterStackEntry;
    if p.is_null() { return }
    if !(*p).name.is_null() {
        xmlFree.expect("non-null function pointer")((*p).name as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
}
/* *
 * xmlCmpTextWriterStackEntry:
 * @data0:  the first data
 * @data1:  the second data
 *
 * Compare callback for the xmlList.
 *
 * Returns -1, 0, 1
 */
unsafe extern "C" fn xmlCmpTextWriterStackEntry(mut data0:
                                                    *const std::os::raw::c_void,
                                                mut data1:
                                                    *const std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut p0: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    let mut p1: *mut xmlTextWriterStackEntry =
        0 as *mut xmlTextWriterStackEntry;
    if data0 == data1 { return 0 as std::os::raw::c_int }
    if data0.is_null() { return -(1 as std::os::raw::c_int) }
    if data1.is_null() { return 1 as std::os::raw::c_int }
    p0 = data0 as *mut xmlTextWriterStackEntry;
    p1 = data1 as *mut xmlTextWriterStackEntry;
    return xmlStrcmp((*p0).name, (*p1).name);
}
/* *
 * misc
 */
/* *
 * xmlTextWriterOutputNSDecl:
 * @writer:  the xmlTextWriterPtr
 *
 * Output the current namespace declarations.
 */
unsafe extern "C" fn xmlTextWriterOutputNSDecl(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut np: *mut xmlTextWriterNsStackEntry =
        0 as *mut xmlTextWriterNsStackEntry;
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    sum = 0 as std::os::raw::c_int;
    while xmlListEmpty((*writer).nsstack) == 0 {
        let mut namespaceURI: *mut xmlChar = 0 as *mut xmlChar;
        let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
        lk = xmlListFront((*writer).nsstack);
        np = xmlLinkGetData(lk) as *mut xmlTextWriterNsStackEntry;
        if !np.is_null() {
            namespaceURI = xmlStrdup((*np).uri);
            prefix = xmlStrdup((*np).prefix)
        }
        xmlListPopFront((*writer).nsstack);
        if !np.is_null() {
            count = xmlTextWriterWriteAttribute(writer, prefix, namespaceURI);
            xmlFree.expect("non-null function pointer")(namespaceURI as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
            if count < 0 as std::os::raw::c_int {
                xmlListDelete((*writer).nsstack);
                (*writer).nsstack = 0 as xmlListPtr;
                return -(1 as std::os::raw::c_int)
            }
            sum += count
        }
    }
    return sum;
}
/* *
 * xmlFreeTextWriterNsStackEntry:
 * @lk:  the xmlLinkPtr
 *
 * Free callback for the xmlList.
 */
unsafe extern "C" fn xmlFreeTextWriterNsStackEntry(mut lk: xmlLinkPtr) {
    let mut p: *mut xmlTextWriterNsStackEntry =
        0 as *mut xmlTextWriterNsStackEntry;
    p = xmlLinkGetData(lk) as *mut xmlTextWriterNsStackEntry;
    if p.is_null() { return }
    if !(*p).prefix.is_null() {
        xmlFree.expect("non-null function pointer")((*p).prefix as
                                                        *mut std::os::raw::c_void);
    }
    if !(*p).uri.is_null() {
        xmlFree.expect("non-null function pointer")((*p).uri as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(p as *mut std::os::raw::c_void);
}
/* *
 * xmlCmpTextWriterNsStackEntry:
 * @data0:  the first data
 * @data1:  the second data
 *
 * Compare callback for the xmlList.
 *
 * Returns -1, 0, 1
 */
unsafe extern "C" fn xmlCmpTextWriterNsStackEntry(mut data0:
                                                      *const std::os::raw::c_void,
                                                  mut data1:
                                                      *const std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut p0: *mut xmlTextWriterNsStackEntry =
        0 as *mut xmlTextWriterNsStackEntry;
    let mut p1: *mut xmlTextWriterNsStackEntry =
        0 as *mut xmlTextWriterNsStackEntry;
    let mut rc: std::os::raw::c_int = 0;
    if data0 == data1 { return 0 as std::os::raw::c_int }
    if data0.is_null() { return -(1 as std::os::raw::c_int) }
    if data1.is_null() { return 1 as std::os::raw::c_int }
    p0 = data0 as *mut xmlTextWriterNsStackEntry;
    p1 = data1 as *mut xmlTextWriterNsStackEntry;
    rc = xmlStrcmp((*p0).prefix, (*p1).prefix);
    if rc != 0 as std::os::raw::c_int || (*p0).elem != (*p1).elem {
        rc = -(1 as std::os::raw::c_int)
    }
    return rc;
}
/* *
 * xmlTextWriterWriteDocCallback:
 * @context:  the xmlBufferPtr
 * @str:  the data to write
 * @len:  the length of the data
 *
 * Write callback for the xmlOutputBuffer with target xmlBuffer
 *
 * Returns -1, 0, 1
 */
unsafe extern "C" fn xmlTextWriterWriteDocCallback(mut context:
                                                       *mut std::os::raw::c_void,
                                                   mut str:
                                                       *const std::os::raw::c_char,
                                                   mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = context as xmlParserCtxtPtr;
    let mut rc: std::os::raw::c_int = 0;
    rc = xmlParseChunk(ctxt, str, len, 0 as std::os::raw::c_int);
    if rc != 0 as std::os::raw::c_int {
        xmlWriterErrMsgInt(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                           b"xmlTextWriterWriteDocCallback : XML error %d !\n\x00"
                               as *const u8 as *const std::os::raw::c_char, rc);
        return -(1 as std::os::raw::c_int)
    }
    return len;
}
/* *
 * xmlTextWriterCloseDocCallback:
 * @context:  the xmlBufferPtr
 *
 * Close callback for the xmlOutputBuffer with target xmlBuffer
 *
 * Returns -1, 0, 1
 */
unsafe extern "C" fn xmlTextWriterCloseDocCallback(mut context:
                                                       *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = context as xmlParserCtxtPtr;
    let mut rc: std::os::raw::c_int = 0;
    rc =
        xmlParseChunk(ctxt, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                      1 as std::os::raw::c_int);
    if rc != 0 as std::os::raw::c_int {
        xmlWriterErrMsgInt(0 as xmlTextWriterPtr, XML_ERR_INTERNAL_ERROR,
                           b"xmlTextWriterCloseDocCallback : XML error %d !\n\x00"
                               as *const u8 as *const std::os::raw::c_char, rc);
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextWriterVSprintf:
 * @format:  see printf
 * @argptr:  pointer to the first member of the variable argument list.
 *
 * Utility function for formatted output
 *
 * Returns a new xmlChar buffer with the data or NULL on error. This buffer must be freed.
 */
// smlChar *
// xmlTextWriterVSprintf(const char *format, va_list argptr)
// {
//     int size;
//     int count;
//     xmlChar *buf;
//     va_list locarg;
//     size = BUFSIZ;
//     buf = (xmlChar *) xmlMalloc(size);
//     if (buf == NULL) {
//         xmlWriterErrMsg(NU_ERR_NO_MEMORY,
//                         "xmlTextWriterVSprintf : out of memory!\n");
//         return NULL;
//     }
//     VA_COPY(locarg, argptr);
//     while (((count = vsnprintf((char *) buf, size, format, locarg)) < 0)
//            || (count == size - 1) || (count == size) || (count > size)) {
// 	va_end(locarg);
//         xmlFree(buf);
//         size += BUFSIZ;
//         buf = (xmlChar *) xmlMalloc(size);
//         if (buf == NULL) {
//             xmlWriterErrMsg(NU_ERR_NO_MEMORY,
//                             "xmlTextWriterVSprintf : out of memory!\n");
//             return NULL;
//         }
// 	VA_COPY(locarg, argptr);
//     }
//     va_end(locarg);
//     return buf;
// }
/* *
 * xmlTextWriterStartDocumentCallback:
 * @ctx: the user data (XML parser context)
 *
 * called at the start of document processing.
 */
unsafe extern "C" fn xmlTextWriterStartDocumentCallback(mut ctx:
                                                            *mut std::os::raw::c_void) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    if (*ctxt).html != 0 {
        if (*ctxt).myDoc.is_null() {
            (*ctxt).myDoc =
                htmlNewDocNoDtD(0 as *const xmlChar, 0 as *const xmlChar)
        }
        if (*ctxt).myDoc.is_null() {
            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).error.is_some() {
                (*(*ctxt).sax).error.expect("non-null function pointer")((*ctxt).userData,
                                                                         b"SAX.startDocument(): out of memory\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const std::os::raw::c_char);
            }
            (*ctxt).errNo = XML_ERR_NO_MEMORY as std::os::raw::c_int;
            (*ctxt).instate = XML_PARSER_EOF;
            (*ctxt).disableSAX = 1 as std::os::raw::c_int;
            return
        }
    } else {
        doc = (*ctxt).myDoc;
        if doc.is_null() {
            (*ctxt).myDoc = xmlNewDoc((*ctxt).version);
            doc = (*ctxt).myDoc
        }
        if !doc.is_null() {
            if (*doc).children.is_null() {
                if !(*ctxt).encoding.is_null() {
                    (*doc).encoding = xmlStrdup((*ctxt).encoding)
                } else { (*doc).encoding = 0 as *const xmlChar }
                (*doc).standalone = (*ctxt).standalone
            }
        } else {
            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).error.is_some() {
                (*(*ctxt).sax).error.expect("non-null function pointer")((*ctxt).userData,
                                                                         b"SAX.startDocument(): out of memory\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const std::os::raw::c_char);
            }
            (*ctxt).errNo = XML_ERR_NO_MEMORY as std::os::raw::c_int;
            (*ctxt).instate = XML_PARSER_EOF;
            (*ctxt).disableSAX = 1 as std::os::raw::c_int;
            return
        }
    }
    if !(*ctxt).myDoc.is_null() && (*(*ctxt).myDoc).URL.is_null() &&
           !(*ctxt).input.is_null() && !(*(*ctxt).input).filename.is_null() {
        (*(*ctxt).myDoc).URL =
            xmlCanonicPath((*(*ctxt).input).filename as *const xmlChar);
        if (*(*ctxt).myDoc).URL.is_null() {
            (*(*ctxt).myDoc).URL =
                xmlStrdup((*(*ctxt).input).filename as *const xmlChar)
        }
    };
}
/*
 * Indentation
 */
/* *
 * xmlTextWriterSetIndent:
 * @writer:  the xmlTextWriterPtr
 * @indent:  do indentation?
 *
 * Set indentation output. indent = 0 do not indentation. indent > 0 do indentation.
 *
 * Returns -1 on error or 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterSetIndent(mut writer: xmlTextWriterPtr,
                                                mut indent: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if writer.is_null() || indent < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    (*writer).indent = indent;
    (*writer).doindent = 1 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextWriterSetIndentString:
 * @writer:  the xmlTextWriterPtr
 * @str:  the xmlChar string
 *
 * Set string indentation.
 *
 * Returns -1 on error or 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterSetIndentString(mut writer:
                                                          xmlTextWriterPtr,
                                                      mut str: *const xmlChar)
 -> std::os::raw::c_int {
    if writer.is_null() || str.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*writer).ichar.is_null() {
        xmlFree.expect("non-null function pointer")((*writer).ichar as
                                                        *mut std::os::raw::c_void);
    }
    (*writer).ichar = xmlStrdup(str);
    if (*writer).ichar.is_null() {
        return -(1 as std::os::raw::c_int)
    } else { return 0 as std::os::raw::c_int };
}
/* *
 * xmlTextWriterSetQuoteChar:
 * @writer:  the xmlTextWriterPtr
 * @quotechar:  the quote character
 *
 * Set the character used for quoting attributes.
 *
 * Returns -1 on error or 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterSetQuoteChar(mut writer:
                                                       xmlTextWriterPtr,
                                                   mut quotechar: xmlChar)
 -> std::os::raw::c_int {
    if writer.is_null() ||
           quotechar as std::os::raw::c_int != '\'' as i32 &&
               quotechar as std::os::raw::c_int != '\"' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    (*writer).qchar = quotechar as std::os::raw::c_char;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextWriterWriteIndent:
 * @writer:  the xmlTextWriterPtr
 *
 * Write indent string.
 *
 * Returns -1 on error or the number of strings written.
 */
unsafe extern "C" fn xmlTextWriterWriteIndent(mut writer: xmlTextWriterPtr)
 -> std::os::raw::c_int {
    let mut lksize: std::os::raw::c_int = 0; /* list is empty */
    let mut i: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    lksize = xmlListSize((*writer).nodes);
    if lksize < 1 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    i = 0 as std::os::raw::c_int;
    while i < lksize - 1 as std::os::raw::c_int {
        ret =
            xmlOutputBufferWriteString((*writer).out,
                                       (*writer).ichar as
                                           *const std::os::raw::c_char);
        if ret == -(1 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
        i += 1
    }
    return lksize - 1 as std::os::raw::c_int;
}
/* *
 * xmlTextWriterHandleStateDependencies:
 * @writer:  the xmlTextWriterPtr
 * @p:  the xmlTextWriterStackEntry
 *
 * Write state dependent strings.
 *
 * Returns -1 on error or the number of characters written.
 */
unsafe extern "C" fn xmlTextWriterHandleStateDependencies(mut writer:
                                                              xmlTextWriterPtr,
                                                          mut p:
                                                              *mut xmlTextWriterStackEntry)
 -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0;
    let mut extra: [std::os::raw::c_char; 3] = [0; 3];
    if writer.is_null() { return -(1 as std::os::raw::c_int) }
    if p.is_null() { return 0 as std::os::raw::c_int }
    sum = 0 as std::os::raw::c_int;
    extra[2 as std::os::raw::c_int as usize] = '\u{0}' as i32 as std::os::raw::c_char;
    extra[1 as std::os::raw::c_int as usize] = extra[2 as std::os::raw::c_int as usize];
    extra[0 as std::os::raw::c_int as usize] = extra[1 as std::os::raw::c_int as usize];
    if !p.is_null() {
        sum = 0 as std::os::raw::c_int;
        match (*p).state as std::os::raw::c_uint {
            1 => {
                /* Output namespace declarations */
                count = xmlTextWriterOutputNSDecl(writer);
                if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                sum += count;
                extra[0 as std::os::raw::c_int as usize] = '>' as i32 as std::os::raw::c_char;
                (*p).state = XML_TEXTWRITER_TEXT
            }
            4 => {
                extra[0 as std::os::raw::c_int as usize] = ' ' as i32 as std::os::raw::c_char;
                (*p).state = XML_TEXTWRITER_PI_TEXT
            }
            7 => {
                extra[0 as std::os::raw::c_int as usize] = ' ' as i32 as std::os::raw::c_char;
                extra[1 as std::os::raw::c_int as usize] = '[' as i32 as std::os::raw::c_char;
                (*p).state = XML_TEXTWRITER_DTD_TEXT
            }
            9 => {
                extra[0 as std::os::raw::c_int as usize] = ' ' as i32 as std::os::raw::c_char;
                (*p).state = XML_TEXTWRITER_DTD_ELEM_TEXT
            }
            11 => {
                extra[0 as std::os::raw::c_int as usize] = ' ' as i32 as std::os::raw::c_char;
                (*p).state = XML_TEXTWRITER_DTD_ATTL_TEXT
            }
            13 | 15 => {
                extra[0 as std::os::raw::c_int as usize] = ' ' as i32 as std::os::raw::c_char;
                extra[1 as std::os::raw::c_int as usize] = (*writer).qchar;
                (*p).state = XML_TEXTWRITER_DTD_ENTY_TEXT
            }
            _ => { }
        }
    }
    if *extra.as_mut_ptr() as std::os::raw::c_int != '\u{0}' as i32 {
        count = xmlOutputBufferWriteString((*writer).out, extra.as_mut_ptr());
        if count < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        sum += count
    }
    return sum;
}
/* __INCLUDE_ELFGCCHACK */
