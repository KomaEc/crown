
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
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn __xmlLoaderErr(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                      filename: *const std::os::raw::c_char);
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    /*
 * Constructor and destructor.
 */
    #[no_mangle]
    fn xmlDictCreate() -> xmlDictPtr;
    #[no_mangle]
    fn xmlDictSetLimit(dict: xmlDictPtr, limit: size_t) -> size_t;
    #[no_mangle]
    fn xmlDictFree(dict: xmlDictPtr);
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
    #[no_mangle]
    fn xmlInitParser();
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    #[no_mangle]
    fn xmlHashDefaultDeallocator(entry: *mut std::os::raw::c_void,
                                 name: *const xmlChar);
    #[no_mangle]
    fn xmlParserValidityError(ctx: *mut std::os::raw::c_void,
                              msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlParserValidityWarning(ctx: *mut std::os::raw::c_void,
                                msg: *const std::os::raw::c_char, _: ...);
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
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParserInputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                          enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferRead(in_0: xmlParserInputBufferPtr,
                                len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParserInputBufferGrow(in_0: xmlParserInputBufferPtr,
                                len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    #[no_mangle]
    fn xmlParserGetDirectory(filename: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    /* LIBXML_HTTP_ENABLED */
    /* LIBXML_OUTPUT_ENABLED */
    #[no_mangle]
    fn xmlCheckHTTPInput(ctxt: xmlParserCtxtPtr, ret: xmlParserInputPtr)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn __xmlSubstituteEntitiesDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlKeepBlanksDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlIndentTreeOutput() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn __xmlPedanticParserDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlLineNumbersDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn __xmlGetWarningsDefaultValue() -> *mut std::os::raw::c_int;
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
    fn xmlSAX2IgnorableWhitespace(ctx: *mut std::os::raw::c_void, ch: *const xmlChar,
                                  len: std::os::raw::c_int);
    #[no_mangle]
    fn __xmlDoValidityCheckingDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __xmlLoadExtDtdDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    /* LIBXML_SAX1_ENABLED */
    #[no_mangle]
    fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlDefaultSAXHandlerInit();
    /*
 * External entities handling actually implemented in xmlIO.
 */
    /*
 * Index lookup, actually implemented in the encoding module
 */
    /*
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
    /* recover on errors */
    /* substitute entities */
    /* load the external subset */
    /* default DTD attributes */
    /* validate with the DTD */
    /* suppress error reports */
    /* suppress warning reports */
    /* pedantic error reporting */
    /* remove blank nodes */
    /* use the SAX1 interface internally */
    /* Implement XInclude substitition  */
    /* Forbid network access */
    /* Do not reuse the context dictionary */
    /* remove redundant namespaces declarations */
    /* merge CDATA as text nodes */
    /* do not generate XINCLUDE START/END nodes */
    /* compact small text nodes; no modification of
                                   the tree allowed afterwards (will possibly
				   crash if you try to modify the tree) */
    /* parse using XML-1.0 before update 5 */
    /* do not fixup XINCLUDE xml:base uris */
    /* relax any hardcoded limit from the parser */
    /* parse using SAX2 interface before 2.7.0 */
    /* ignore internal document encoding hint */
    /* Store big lines numbers in text PSVI field */
    #[no_mangle]
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn __xmlParserDebugEntities() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    fn xmlLoadExternalEntity(URL: *const std::os::raw::c_char,
                             ID: *const std::os::raw::c_char, ctxt: xmlParserCtxtPtr)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn inputPop(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlCharInRange(val: std::os::raw::c_uint, group: *const xmlChRangeGroup)
     -> std::os::raw::c_int;
    #[no_mangle]
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    /*
 * Strictly minimal interfaces for per-document catalogs used
 * by the parser.
 */
    #[no_mangle]
    fn xmlCatalogFreeLocal(catalogs: *mut std::os::raw::c_void);
    /*
 * Summary: Internal Interfaces for memory buffers in libxml2
 * Description: this module describes most of the new xmlBuf buffer
 *              entry points, those are private routines, with a
 *              few exceptions exported in tree.h. This was added
 *              in 2.9.0.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    #[no_mangle]
    fn xmlBufCreate() -> xmlBufPtr;
    /* size_t xmlBufUse(const xmlBufPtr buf); */
    #[no_mangle]
    fn xmlBufIsEmpty(buf: xmlBufPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCharEncFirstLineInput(input: xmlParserInputBufferPtr,
                                len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCharEncInput(input: xmlParserInputBufferPtr, flush: std::os::raw::c_int)
     -> std::os::raw::c_int;
}
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type FILE = _IO_FILE;
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
pub type size_t = std::os::raw::c_ulong;
pub type __off64_t = std::os::raw::c_long;
pub type _IO_lock_t = ();
pub type __off_t = std::os::raw::c_long;
pub type xmlChar = std::os::raw::c_uchar;
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
/*
 * Recent version of gcc produce a warning when a function pointer is assigned
 * to an object pointer, or vice versa.  The following macro is a dirty hack
 * to allow suppression of the warning.  If your architecture has function
 * pointers which are a different size than a void pointer, there may be some
 * serious trouble within the library.
 */
/* *
 * XML_CAST_FPTR:
 * @fptr:  pointer to a function
 *
 * Macro to do a casting from an object pointer to a
 * function pointer without encountering a warning from
 * gcc
 *
 * #define XML_CAST_FPTR(fptr) (*(void **)(&fptr))
 * This macro violated ISO C aliasing rules (gcc4 on s390 broke)
 * so it is disabled now
 */
/*
 * function types:
 */
/* *
 * xmlHashDeallocator:
 * @payload:  the data in the hash
 * @name:  the name associated
 *
 * Callback to free data from a hash.
 */
pub type xmlHashDeallocator
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
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
pub type xmlParserNodeInfoPtr = *mut xmlParserNodeInfo;
pub type xmlParserNodeInfoSeqPtr = *mut xmlParserNodeInfoSeq;
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
pub const XML_PARSE_NOENT: C2RustUnnamed_0 = 2;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_0 = 16;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_0 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_0 = 128;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_0 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_0 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_0 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_0 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_0 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_0 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_0 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_0 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_0 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_0 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_0 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_0 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_0 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_0 = 512;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_0 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_0 = 32;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_0 = 8;
pub const XML_PARSE_RECOVER: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: std::os::raw::c_ushort,
    pub high: std::os::raw::c_ushort,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: std::os::raw::c_uint,
    pub high: std::os::raw::c_uint,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: std::os::raw::c_int,
    pub nbLongRange: std::os::raw::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
/*
 * Various global defaults for parsing
 */
/* *
 * xmlCheckVersion:
 * @version: the include version number
 *
 * check the compiled lib version against the include one.
 * This can warn or immediately kill the application
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCheckVersion(mut version: std::os::raw::c_int) {
    let mut myversion: std::os::raw::c_int = 20908 as std::os::raw::c_int;
    xmlInitParser();
    if myversion / 10000 as std::os::raw::c_int != version / 10000 as std::os::raw::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Fatal: program compiled against libxml %d using libxml %d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   version /
                                                                       10000
                                                                           as
                                                                           std::os::raw::c_int,
                                                                   myversion /
                                                                       10000
                                                                           as
                                                                           std::os::raw::c_int);
        fprintf(stderr,
                b"Fatal: program compiled against libxml %d using libxml %d\n\x00"
                    as *const u8 as *const std::os::raw::c_char,
                version / 10000 as std::os::raw::c_int,
                myversion / 10000 as std::os::raw::c_int);
    }
    if (myversion / 100 as std::os::raw::c_int) < version / 100 as std::os::raw::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Warning: program compiled against libxml %d using older %d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   version /
                                                                       100 as
                                                                           std::os::raw::c_int,
                                                                   myversion /
                                                                       100 as
                                                                           std::os::raw::c_int);
    };
}
/* *
 * Entities
 */
/* LIBXML_LEGACY_ENABLED */
/*
 * internal only
 */
/* ***********************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/
/* *
 * xmlErrMemory:
 * @ctxt:  an XML parser context
 * @extra:  extra informations
 *
 * Handle a redefinition of attribute error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlErrMemory(mut ctxt: xmlParserCtxtPtr,
                                      mut extra: *const std::os::raw::c_char) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_NO_MEMORY as std::os::raw::c_int;
        (*ctxt).instate = XML_PARSER_EOF;
        (*ctxt).disableSAX = 1 as std::os::raw::c_int
    }
    if !extra.is_null() {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_PARSER as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"Memory allocation failed : %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, extra);
    } else {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_PARSER as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        b"Memory allocation failed\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
    };
}
/* internal error reporting */
/* *
 * __xmlErrEncoding:
 * @ctxt:  an XML parser context
 * @xmlerr:  the error number
 * @msg:  the error message
 * @str1:  an string info
 * @str2:  an string info
 *
 * Handle an encoding error
 */
#[no_mangle]
pub unsafe extern "C" fn __xmlErrEncoding(mut ctxt: xmlParserCtxtPtr,
                                          mut xmlerr: xmlParserErrors,
                                          mut msg: *const std::os::raw::c_char,
                                          mut str1: *const xmlChar,
                                          mut str2: *const xmlChar) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() { (*ctxt).errNo = xmlerr as std::os::raw::c_int }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_PARSER as std::os::raw::c_int, xmlerr as std::os::raw::c_int,
                    XML_ERR_FATAL, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2);
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as std::os::raw::c_int;
        if (*ctxt).recovery == 0 as std::os::raw::c_int {
            (*ctxt).disableSAX = 1 as std::os::raw::c_int
        }
    };
}
/* *
 * xmlErrInternal:
 * @ctxt:  an XML parser context
 * @msg:  the error message
 * @str:  error informations
 *
 * Handle an internal error
 */
unsafe extern "C" fn xmlErrInternal(mut ctxt: xmlParserCtxtPtr,
                                    mut msg: *const std::os::raw::c_char,
                                    mut str: *const xmlChar) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_INTERNAL_ERROR as std::os::raw::c_int
    }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_PARSER as std::os::raw::c_int,
                    XML_ERR_INTERNAL_ERROR as std::os::raw::c_int, XML_ERR_FATAL,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str);
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as std::os::raw::c_int;
        if (*ctxt).recovery == 0 as std::os::raw::c_int {
            (*ctxt).disableSAX = 1 as std::os::raw::c_int
        }
    };
}
/* *
 * xmlErrEncodingInt:
 * @ctxt:  an XML parser context
 * @error:  the error number
 * @msg:  the error message
 * @val:  an integer value
 *
 * n encoding error
 */
unsafe extern "C" fn xmlErrEncodingInt(mut ctxt: xmlParserCtxtPtr,
                                       mut error: xmlParserErrors,
                                       mut msg: *const std::os::raw::c_char,
                                       mut val: std::os::raw::c_int) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() { (*ctxt).errNo = error as std::os::raw::c_int }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_PARSER as std::os::raw::c_int, error as std::os::raw::c_int,
                    XML_ERR_FATAL, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, val, 0 as std::os::raw::c_int, msg,
                    val);
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as std::os::raw::c_int;
        if (*ctxt).recovery == 0 as std::os::raw::c_int {
            (*ctxt).disableSAX = 1 as std::os::raw::c_int
        }
    };
}
/*
 * Function to finish the work of the macros where needed.
 */
/* *
 * xmlIsLetter:
 * @c:  an unicode character (int)
 *
 * Check whether the character is allowed by the production
 * [84] Letter ::= BaseChar | Ideographic
 *
 * Returns 0 if not, non-zero otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIsLetter(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return ((if c < 0x100 as std::os::raw::c_int {
                 (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                      0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int ||
                      0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int ||
                      0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int ||
                      0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
             } else {
                 xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup)
             }) != 0 ||
                (if c < 0x100 as std::os::raw::c_int {
                     0 as std::os::raw::c_int
                 } else {
                     (0x4e00 as std::os::raw::c_int <= c && c <= 0x9fa5 as std::os::raw::c_int
                          || c == 0x3007 as std::os::raw::c_int ||
                          0x3021 as std::os::raw::c_int <= c &&
                              c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
                 }) != 0) as std::os::raw::c_int;
}
/* *
 * xmlParserInputRead:
 * @in:  an XML parser input
 * @len:  an indicative size for the lookahead
 *
 * This function was internal and is deprecated.
 *
 * Returns -1 as this is an error to use it.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputRead(mut in_0: xmlParserInputPtr,
                                            mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlParserInputGrow:
 * @in:  an XML parser input
 * @len:  an indicative size for the lookahead
 *
 * This function increase the input for the parser. It tries to
 * preserve pointers to the input buffer, and keep already read data
 *
 * Returns the amount of char read, or -1 in case of error, 0 indicate the
 * end of this entity
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputGrow(mut in_0: xmlParserInputPtr,
                                            mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut indx: size_t = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    if in_0.is_null() || len < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    if (*in_0).buf.is_null() { return -(1 as std::os::raw::c_int) }
    if (*in_0).base.is_null() { return -(1 as std::os::raw::c_int) }
    if (*in_0).cur.is_null() { return -(1 as std::os::raw::c_int) }
    if (*(*in_0).buf).buffer.is_null() { return -(1 as std::os::raw::c_int) }
    indx =
        (*in_0).cur.offset_from((*in_0).base) as std::os::raw::c_long as
            size_t;
    if xmlBufUse((*(*in_0).buf).buffer) >
           (indx as
                std::os::raw::c_uint).wrapping_add(250 as std::os::raw::c_int as std::os::raw::c_uint)
               as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if (*(*in_0).buf).readcallback.is_some() {
        ret = xmlParserInputBufferGrow((*in_0).buf, len)
    } else { return 0 as std::os::raw::c_int }
    /*
     * NOTE : in->base may be a "dangling" i.e. freed pointer in this
     *        block, but we use it really as an integer to do some
     *        pointer arithmetic. Insure will raise it as a bug but in
     *        that specific case, that's not !
     */
    content = xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf);
    if (*in_0).base != content {
        /*
	 * the buffer has been reallocated
	 */
        indx =
            (*in_0).cur.offset_from((*in_0).base) as std::os::raw::c_long as
                size_t;
        (*in_0).base = content;
        (*in_0).cur = &*content.offset(indx as isize) as *const xmlChar
    }
    (*in_0).end = xmlBufEnd((*(*in_0).buf).buffer);
    return ret;
}
/* *
 * xmlParserInputShrink:
 * @in:  an XML parser input
 *
 * This function removes used input for the parser.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputShrink(mut in_0: xmlParserInputPtr) {
    let mut used: size_t = 0;
    let mut ret: size_t = 0;
    let mut indx: size_t = 0;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    if in_0.is_null() { return }
    if (*in_0).buf.is_null() { return }
    if (*in_0).base.is_null() { return }
    if (*in_0).cur.is_null() { return }
    if (*(*in_0).buf).buffer.is_null() { return }
    used =
        (*in_0).cur.offset_from(xmlBufContent((*(*in_0).buf).buffer
                                                           as *const xmlBuf))
            as std::os::raw::c_long as size_t;
    /*
     * Do not shrink on large buffers whose only a tiny fraction
     * was consumed
     */
    if used > 250 as std::os::raw::c_int as std::os::raw::c_ulong {
        ret =
            xmlBufShrink((*(*in_0).buf).buffer,
                         used.wrapping_sub(80 as std::os::raw::c_int as
                                               std::os::raw::c_ulong));
        if ret > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            (*in_0).cur = (*in_0).cur.offset(-(ret as isize));
            (*in_0).consumed = (*in_0).consumed.wrapping_add(ret)
        }
        (*in_0).end = xmlBufEnd((*(*in_0).buf).buffer)
    }
    if xmlBufUse((*(*in_0).buf).buffer) > 250 as std::os::raw::c_int as std::os::raw::c_ulong
       {
        return
    }
    xmlParserInputBufferRead((*in_0).buf,
                             2 as std::os::raw::c_int * 250 as std::os::raw::c_int);
    content = xmlBufContent((*(*in_0).buf).buffer as *const xmlBuf);
    if (*in_0).base != content {
        /*
	 * the buffer has been reallocated
	 */
        indx =
            (*in_0).cur.offset_from((*in_0).base) as std::os::raw::c_long as
                size_t;
        (*in_0).base = content;
        (*in_0).cur = &*content.offset(indx as isize) as *const xmlChar
    }
    (*in_0).end = xmlBufEnd((*(*in_0).buf).buffer);
}
/* ***********************************************************************
 *									*
 *		UTF8 character input and related functions		*
 *									*
 ************************************************************************/
/* *
 * xmlNextChar:
 * @ctxt:  the XML parser context
 *
 * Skip to the next char input char.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNextChar(mut ctxt: xmlParserCtxtPtr) {
    let mut current_block: u64;
    if ctxt.is_null() ||
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int ||
           (*ctxt).input.is_null() {
        return
    }
    if !((*(*ctxt).input).cur <= (*(*ctxt).input).end) {
        xmlErrInternal(ctxt,
                       b"Parser input data memory error\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar);
        (*ctxt).errNo = XML_ERR_INTERNAL_ERROR as std::os::raw::c_int;
        xmlStopParser(ctxt);
        return
    }
    if *(*(*ctxt).input).cur as std::os::raw::c_int == 0 as std::os::raw::c_int &&
           xmlParserInputGrow((*ctxt).input, 250 as std::os::raw::c_int) <=
               0 as std::os::raw::c_int {
        return
    }
    if (*ctxt).charset == XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int {
        let mut cur: *const std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar;
        let mut c: std::os::raw::c_uchar = 0;
        /*
         *   2.11 End-of-Line Handling
         *   the literal two-character sequence "#xD#xA" or a standalone
         *   literal #xD, an XML processor must pass to the application
         *   the single character #xA.
         */
        if *(*(*ctxt).input).cur as std::os::raw::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as std::os::raw::c_int
        } else { (*(*ctxt).input).col += 1 }
        /*
         * We are supposed to handle UTF8, check it's valid
         * From rfc2044: encoding of the Unicode values on UTF-8:
         *
         * UCS-4 range (hex.)           UTF-8 octet sequence (binary)
         * 0000 0000-0000 007F   0xxxxxxx
         * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
         * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
         *
         * Check for the 0x110000 limit too
         */
        cur = (*(*ctxt).input).cur;
        c = *cur;
        if c as std::os::raw::c_int & 0x80 as std::os::raw::c_int != 0 {
            if c as std::os::raw::c_int == 0xc0 as std::os::raw::c_int {
                current_block = 17790562316993263864;
            } else {
                if *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                    xmlParserInputGrow((*ctxt).input, 250 as std::os::raw::c_int);
                    cur = (*(*ctxt).input).cur
                }
                if *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                       0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                    current_block = 17790562316993263864;
                } else if c as std::os::raw::c_int & 0xe0 as std::os::raw::c_int ==
                              0xe0 as std::os::raw::c_int {
                    let mut val: std::os::raw::c_uint = 0;
                    if *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 0 as std::os::raw::c_int {
                        xmlParserInputGrow((*ctxt).input, 250 as std::os::raw::c_int);
                        cur = (*(*ctxt).input).cur
                    }
                    if *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                           0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                        current_block = 17790562316993263864;
                    } else {
                        if c as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                               0xf0 as std::os::raw::c_int {
                            if *cur.offset(3 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int == 0 as std::os::raw::c_int {
                                xmlParserInputGrow((*ctxt).input,
                                                   250 as std::os::raw::c_int);
                                cur = (*(*ctxt).input).cur
                            }
                            if c as std::os::raw::c_int & 0xf8 as std::os::raw::c_int !=
                                   0xf0 as std::os::raw::c_int ||
                                   *cur.offset(3 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int & 0xc0 as std::os::raw::c_int !=
                                       0x80 as std::os::raw::c_int {
                                current_block = 17790562316993263864;
                            } else {
                                /* 4-byte code */
                                (*(*ctxt).input).cur =
                                    (*(*ctxt).input).cur.offset(4 as
                                                                    std::os::raw::c_int
                                                                    as isize);
                                val =
                                    ((*cur.offset(0 as std::os::raw::c_int as isize)
                                          as std::os::raw::c_int & 0x7 as std::os::raw::c_int)
                                         << 18 as std::os::raw::c_int) as
                                        std::os::raw::c_uint;
                                val |=
                                    ((*cur.offset(1 as std::os::raw::c_int as isize)
                                          as std::os::raw::c_int &
                                          0x3f as std::os::raw::c_int) <<
                                         12 as std::os::raw::c_int) as std::os::raw::c_uint;
                                val |=
                                    ((*cur.offset(2 as std::os::raw::c_int as isize)
                                          as std::os::raw::c_int &
                                          0x3f as std::os::raw::c_int) <<
                                         6 as std::os::raw::c_int) as std::os::raw::c_uint;
                                val |=
                                    (*cur.offset(3 as std::os::raw::c_int as isize) as
                                         std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                        std::os::raw::c_uint;
                                current_block = 15004371738079956865;
                            }
                        } else {
                            /* 3-byte code */
                            (*(*ctxt).input).cur =
                                (*(*ctxt).input).cur.offset(3 as std::os::raw::c_int
                                                                as isize);
                            val =
                                ((*cur.offset(0 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0xf as std::os::raw::c_int) <<
                                     12 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                ((*cur.offset(1 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0x3f as std::os::raw::c_int) <<
                                     6 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                (*cur.offset(2 as std::os::raw::c_int as isize) as
                                     std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                    std::os::raw::c_uint;
                            current_block = 15004371738079956865;
                        }
                        match current_block {
                            17790562316993263864 => { }
                            _ => {
                                if val > 0xd7ff as std::os::raw::c_int as std::os::raw::c_uint
                                       &&
                                       val <
                                           0xe000 as std::os::raw::c_int as
                                               std::os::raw::c_uint ||
                                       val >
                                           0xfffd as std::os::raw::c_int as
                                               std::os::raw::c_uint &&
                                           val <
                                               0x10000 as std::os::raw::c_int as
                                                   std::os::raw::c_uint ||
                                       val >=
                                           0x110000 as std::os::raw::c_int as
                                               std::os::raw::c_uint {
                                    xmlErrEncodingInt(ctxt,
                                                      XML_ERR_INVALID_CHAR,
                                                      b"Char 0x%X out of allowed range\n\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char,
                                                      val as std::os::raw::c_int);
                                }
                                current_block = 14945149239039849694;
                            }
                        }
                    }
                } else {
                    /* 2-byte code */
                    (*(*ctxt).input).cur =
                        (*(*ctxt).input).cur.offset(2 as std::os::raw::c_int as
                                                        isize);
                    current_block = 14945149239039849694;
                }
            }
            match current_block {
                14945149239039849694 => { }
                _ => {
                    /*
     * If we detect an UTF8 error that probably mean that the
     * input encoding didn't get properly advertised in the
     * declaration header. Report the error and switch the encoding
     * to ISO-Latin-1 (if you don't like this policy, just declare the
     * encoding !)
     */
                    if ctxt.is_null() || (*ctxt).input.is_null() ||
                           ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as std::os::raw::c_long) <
                               4 as std::os::raw::c_int as std::os::raw::c_long {
                        __xmlErrEncoding(ctxt, XML_ERR_INVALID_CHAR,
                                         b"Input is not proper UTF-8, indicate encoding !\n\x00"
                                             as *const u8 as
                                             *const std::os::raw::c_char,
                                         0 as *const xmlChar,
                                         0 as *const xmlChar);
                    } else {
                        let mut buffer: [std::os::raw::c_char; 150] = [0; 150];
                        snprintf(buffer.as_mut_ptr(),
                                 149 as std::os::raw::c_int as std::os::raw::c_ulong,
                                 b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int
                                                                  as isize) as
                                     std::os::raw::c_int,
                                 *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int
                                                                  as isize) as
                                     std::os::raw::c_int,
                                 *(*(*ctxt).input).cur.offset(2 as std::os::raw::c_int
                                                                  as isize) as
                                     std::os::raw::c_int,
                                 *(*(*ctxt).input).cur.offset(3 as std::os::raw::c_int
                                                                  as isize) as
                                     std::os::raw::c_int);
                        __xmlErrEncoding(ctxt, XML_ERR_INVALID_CHAR,
                                         b"Input is not proper UTF-8, indicate encoding !\n%s\x00"
                                             as *const u8 as
                                             *const std::os::raw::c_char,
                                         buffer.as_mut_ptr() as *mut xmlChar,
                                         0 as *const xmlChar);
                    }
                    (*ctxt).charset = XML_CHAR_ENCODING_8859_1 as std::os::raw::c_int;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
                    return
                }
            }
        } else {
            /* 1-byte code */
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
        }
        (*ctxt).nbChars += 1
    } else {
        /*
         * Assume it's a fixed length encoding (1) with
         * a compatible encoding for the ASCII set, since
         * XML constructs only use < 128 chars
         */
        if *(*(*ctxt).input).cur as std::os::raw::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as std::os::raw::c_int
        } else { (*(*ctxt).input).col += 1 }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        (*ctxt).nbChars += 1
    }
    if *(*(*ctxt).input).cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
        xmlParserInputGrow((*ctxt).input, 250 as std::os::raw::c_int);
    };
}
/*
 * Really core function shared with HTML parser.
 */
/* *
 * xmlCurrentChar:
 * @ctxt:  the XML parser context
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer. Implement the end of line normalization:
 * 2.11 End-of-Line Handling
 * Wherever an external parsed entity or the literal entity value
 * of an internal parsed entity contains either the literal two-character
 * sequence "#xD#xA" or a standalone literal #xD, an XML processor
 * must pass to the application the single character #xA.
 * This behavior can conveniently be produced by normalizing all
 * line breaks to #xA on input, before parsing.)
 *
 * Returns the current char value and its length
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCurrentChar(mut ctxt: xmlParserCtxtPtr,
                                        mut len: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    if ctxt.is_null() || len.is_null() || (*ctxt).input.is_null() {
        return 0 as std::os::raw::c_int
    }
    if (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    if *(*(*ctxt).input).cur as std::os::raw::c_int >= 0x20 as std::os::raw::c_int &&
           *(*(*ctxt).input).cur as std::os::raw::c_int <= 0x7f as std::os::raw::c_int {
        *len = 1 as std::os::raw::c_int;
        return *(*(*ctxt).input).cur as std::os::raw::c_int
    }
    if (*ctxt).charset == XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int {
        /*
	 * We are supposed to handle UTF8, check it's valid
	 * From rfc2044: encoding of the Unicode values on UTF-8:
	 *
	 * UCS-4 range (hex.)           UTF-8 octet sequence (binary)
	 * 0000 0000-0000 007F   0xxxxxxx
	 * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
	 * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
	 *
	 * Check for the 0x110000 limit too
	 */
        let mut cur: *const std::os::raw::c_uchar = (*(*ctxt).input).cur;
        let mut c: std::os::raw::c_uchar = 0;
        let mut val: std::os::raw::c_uint = 0;
        c = *cur;
        if c as std::os::raw::c_int & 0x80 as std::os::raw::c_int != 0 {
            if !(c as std::os::raw::c_int & 0x40 as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                     c as std::os::raw::c_int == 0xc0 as std::os::raw::c_int) {
                if *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                    xmlParserInputGrow((*ctxt).input, 250 as std::os::raw::c_int);
                    cur = (*(*ctxt).input).cur
                }
                if !(*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                         0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int) {
                    if c as std::os::raw::c_int & 0xe0 as std::os::raw::c_int ==
                           0xe0 as std::os::raw::c_int {
                        if *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == 0 as std::os::raw::c_int {
                            xmlParserInputGrow((*ctxt).input,
                                               250 as std::os::raw::c_int);
                            cur = (*(*ctxt).input).cur
                        }
                        if *cur.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int & 0xc0 as std::os::raw::c_int !=
                               0x80 as std::os::raw::c_int {
                            current_block = 14263334547792153707;
                        } else if c as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                                      0xf0 as std::os::raw::c_int {
                            if *cur.offset(3 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int == 0 as std::os::raw::c_int {
                                xmlParserInputGrow((*ctxt).input,
                                                   250 as std::os::raw::c_int);
                                cur = (*(*ctxt).input).cur
                            }
                            if c as std::os::raw::c_int & 0xf8 as std::os::raw::c_int !=
                                   0xf0 as std::os::raw::c_int ||
                                   *cur.offset(3 as std::os::raw::c_int as isize) as
                                       std::os::raw::c_int & 0xc0 as std::os::raw::c_int !=
                                       0x80 as std::os::raw::c_int {
                                current_block = 14263334547792153707;
                            } else {
                                /* 4-byte code */
                                *len = 4 as std::os::raw::c_int;
                                val =
                                    ((*cur.offset(0 as std::os::raw::c_int as isize)
                                          as std::os::raw::c_int & 0x7 as std::os::raw::c_int)
                                         << 18 as std::os::raw::c_int) as
                                        std::os::raw::c_uint;
                                val |=
                                    ((*cur.offset(1 as std::os::raw::c_int as isize)
                                          as std::os::raw::c_int &
                                          0x3f as std::os::raw::c_int) <<
                                         12 as std::os::raw::c_int) as std::os::raw::c_uint;
                                val |=
                                    ((*cur.offset(2 as std::os::raw::c_int as isize)
                                          as std::os::raw::c_int &
                                          0x3f as std::os::raw::c_int) <<
                                         6 as std::os::raw::c_int) as std::os::raw::c_uint;
                                val |=
                                    (*cur.offset(3 as std::os::raw::c_int as isize) as
                                         std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                        std::os::raw::c_uint;
                                if val <
                                       0x10000 as std::os::raw::c_int as std::os::raw::c_uint
                                   {
                                    current_block = 14263334547792153707;
                                } else {
                                    current_block = 3938820862080741272;
                                }
                            }
                        } else {
                            /* 3-byte code */
                            *len = 3 as std::os::raw::c_int;
                            val =
                                ((*cur.offset(0 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0xf as std::os::raw::c_int) <<
                                     12 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                ((*cur.offset(1 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0x3f as std::os::raw::c_int) <<
                                     6 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                (*cur.offset(2 as std::os::raw::c_int as isize) as
                                     std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                    std::os::raw::c_uint;
                            if val < 0x800 as std::os::raw::c_int as std::os::raw::c_uint {
                                current_block = 14263334547792153707;
                            } else { current_block = 3938820862080741272; }
                        }
                    } else {
                        /* 2-byte code */
                        *len = 2 as std::os::raw::c_int;
                        val =
                            ((*cur.offset(0 as std::os::raw::c_int as isize) as
                                  std::os::raw::c_int & 0x1f as std::os::raw::c_int) <<
                                 6 as std::os::raw::c_int) as std::os::raw::c_uint;
                        val |=
                            (*cur.offset(1 as std::os::raw::c_int as isize) as
                                 std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                std::os::raw::c_uint;
                        if val < 0x80 as std::os::raw::c_int as std::os::raw::c_uint {
                            current_block = 14263334547792153707;
                        } else { current_block = 3938820862080741272; }
                    }
                    match current_block {
                        14263334547792153707 => { }
                        _ => {
                            if if val < 0x100 as std::os::raw::c_int as std::os::raw::c_uint {
                                   (0x9 as std::os::raw::c_int as std::os::raw::c_uint <= val
                                        &&
                                        val <=
                                            0xa as std::os::raw::c_int as std::os::raw::c_uint
                                        ||
                                        val ==
                                            0xd as std::os::raw::c_int as std::os::raw::c_uint
                                        ||
                                        0x20 as std::os::raw::c_int as std::os::raw::c_uint <=
                                            val) as std::os::raw::c_int
                               } else {
                                   (0x100 as std::os::raw::c_int as std::os::raw::c_uint <=
                                        val &&
                                        val <=
                                            0xd7ff as std::os::raw::c_int as
                                                std::os::raw::c_uint ||
                                        0xe000 as std::os::raw::c_int as std::os::raw::c_uint
                                            <= val &&
                                            val <=
                                                0xfffd as std::os::raw::c_int as
                                                    std::os::raw::c_uint ||
                                        0x10000 as std::os::raw::c_int as std::os::raw::c_uint
                                            <= val &&
                                            val <=
                                                0x10ffff as std::os::raw::c_int as
                                                    std::os::raw::c_uint) as
                                       std::os::raw::c_int
                               } == 0 {
                                xmlErrEncodingInt(ctxt, XML_ERR_INVALID_CHAR,
                                                  b"Char 0x%X out of allowed range\n\x00"
                                                      as *const u8 as
                                                      *const std::os::raw::c_char,
                                                  val as std::os::raw::c_int);
                            }
                            return val as std::os::raw::c_int
                        }
                    }
                }
            }
            /*
     * An encoding problem may arise from a truncated input buffer
     * splitting a character in the middle. In that case do not raise
     * an error but return 0 to endicate an end of stream problem
     */
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                    as std::os::raw::c_long) < 4 as std::os::raw::c_int as std::os::raw::c_long {
                *len = 0 as std::os::raw::c_int;
                return 0 as std::os::raw::c_int
            }
            /*
     * If we detect an UTF8 error that probably mean that the
     * input encoding didn't get properly advertised in the
     * declaration header. Report the error and switch the encoding
     * to ISO-Latin-1 (if you don't like this policy, just declare the
     * encoding !)
     */
            let mut buffer: [std::os::raw::c_char; 150] = [0; 150];
            snprintf(&mut *buffer.as_mut_ptr().offset(0 as std::os::raw::c_int as
                                                          isize) as
                         *mut std::os::raw::c_char,
                     149 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8
                         as *const std::os::raw::c_char,
                     *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int,
                     *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int,
                     *(*(*ctxt).input).cur.offset(2 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int,
                     *(*(*ctxt).input).cur.offset(3 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int);
            __xmlErrEncoding(ctxt, XML_ERR_INVALID_CHAR,
                             b"Input is not proper UTF-8, indicate encoding !\n%s\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             buffer.as_mut_ptr() as *mut xmlChar,
                             0 as *const xmlChar);
            (*ctxt).charset = XML_CHAR_ENCODING_8859_1 as std::os::raw::c_int;
            *len = 1 as std::os::raw::c_int;
            return *(*(*ctxt).input).cur as std::os::raw::c_int
        } else {
            /* 1-byte code */
            *len = 1 as std::os::raw::c_int;
            if *(*(*ctxt).input).cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                xmlParserInputGrow((*ctxt).input, 250 as std::os::raw::c_int);
            }
            if *(*(*ctxt).input).cur as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                   (*(*ctxt).input).end > (*(*ctxt).input).cur {
                xmlErrEncodingInt(ctxt, XML_ERR_INVALID_CHAR,
                                  b"Char 0x0 out of allowed range\n\x00" as
                                      *const u8 as *const std::os::raw::c_char,
                                  0 as std::os::raw::c_int);
            }
            if *(*(*ctxt).input).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                if *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xa as std::os::raw::c_int {
                    (*ctxt).nbChars += 1;
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
                }
                return 0xa as std::os::raw::c_int
            }
            return *(*(*ctxt).input).cur as std::os::raw::c_int
        }
    } else {
        /*
     * Assume it's a fixed length encoding (1) with
     * a compatible encoding for the ASCII set, since
     * XML constructs only use < 128 chars
     */
        *len = 1 as std::os::raw::c_int;
        if *(*(*ctxt).input).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == 0xa as std::os::raw::c_int {
                (*ctxt).nbChars += 1;
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1)
            }
            return 0xa as std::os::raw::c_int
        }
        return *(*(*ctxt).input).cur as std::os::raw::c_int
    };
}
/* *
 * xmlStringCurrentChar:
 * @ctxt:  the XML parser context
 * @cur:  pointer to the beginning of the char
 * @len:  pointer to the length of the char read
 *
 * The current char value, if using UTF-8 this may actually span multiple
 * bytes in the input buffer.
 *
 * Returns the current char value and its length
 */
#[no_mangle]
pub unsafe extern "C" fn xmlStringCurrentChar(mut ctxt: xmlParserCtxtPtr,
                                              mut cur: *const xmlChar,
                                              mut len: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    if len.is_null() || cur.is_null() { return 0 as std::os::raw::c_int }
    if ctxt.is_null() ||
           (*ctxt).charset == XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int {
        /*
         * We are supposed to handle UTF8, check it's valid
         * From rfc2044: encoding of the Unicode values on UTF-8:
         *
         * UCS-4 range (hex.)           UTF-8 octet sequence (binary)
         * 0000 0000-0000 007F   0xxxxxxx
         * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
         * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
         *
         * Check for the 0x110000 limit too
         */
        let mut c: std::os::raw::c_uchar = 0;
        let mut val: std::os::raw::c_uint = 0;
        c = *cur;
        if c as std::os::raw::c_int & 0x80 as std::os::raw::c_int != 0 {
            if !(*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                     0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int) {
                if c as std::os::raw::c_int & 0xe0 as std::os::raw::c_int ==
                       0xe0 as std::os::raw::c_int {
                    if *cur.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                           0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                        current_block = 8211107853514384623;
                    } else if c as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                                  0xf0 as std::os::raw::c_int {
                        if c as std::os::raw::c_int & 0xf8 as std::os::raw::c_int !=
                               0xf0 as std::os::raw::c_int ||
                               *cur.offset(3 as std::os::raw::c_int as isize) as
                                   std::os::raw::c_int & 0xc0 as std::os::raw::c_int !=
                                   0x80 as std::os::raw::c_int {
                            current_block = 8211107853514384623;
                        } else {
                            /* 4-byte code */
                            *len = 4 as std::os::raw::c_int;
                            val =
                                ((*cur.offset(0 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0x7 as std::os::raw::c_int) <<
                                     18 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                ((*cur.offset(1 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0x3f as std::os::raw::c_int) <<
                                     12 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                ((*cur.offset(2 as std::os::raw::c_int as isize) as
                                      std::os::raw::c_int & 0x3f as std::os::raw::c_int) <<
                                     6 as std::os::raw::c_int) as std::os::raw::c_uint;
                            val |=
                                (*cur.offset(3 as std::os::raw::c_int as isize) as
                                     std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                    std::os::raw::c_uint;
                            current_block = 11298138898191919651;
                        }
                    } else {
                        /* 3-byte code */
                        *len = 3 as std::os::raw::c_int;
                        val =
                            ((*cur.offset(0 as std::os::raw::c_int as isize) as
                                  std::os::raw::c_int & 0xf as std::os::raw::c_int) <<
                                 12 as std::os::raw::c_int) as std::os::raw::c_uint;
                        val |=
                            ((*cur.offset(1 as std::os::raw::c_int as isize) as
                                  std::os::raw::c_int & 0x3f as std::os::raw::c_int) <<
                                 6 as std::os::raw::c_int) as std::os::raw::c_uint;
                        val |=
                            (*cur.offset(2 as std::os::raw::c_int as isize) as
                                 std::os::raw::c_int & 0x3f as std::os::raw::c_int) as
                                std::os::raw::c_uint;
                        current_block = 11298138898191919651;
                    }
                } else {
                    /* 2-byte code */
                    *len = 2 as std::os::raw::c_int;
                    val =
                        ((*cur.offset(0 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int & 0x1f as std::os::raw::c_int) <<
                             6 as std::os::raw::c_int) as std::os::raw::c_uint;
                    val |=
                        (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                             & 0x3f as std::os::raw::c_int) as std::os::raw::c_uint;
                    current_block = 11298138898191919651;
                }
                match current_block {
                    8211107853514384623 => { }
                    _ => {
                        if if val < 0x100 as std::os::raw::c_int as std::os::raw::c_uint {
                               (0x9 as std::os::raw::c_int as std::os::raw::c_uint <= val &&
                                    val <= 0xa as std::os::raw::c_int as std::os::raw::c_uint
                                    ||
                                    val == 0xd as std::os::raw::c_int as std::os::raw::c_uint
                                    ||
                                    0x20 as std::os::raw::c_int as std::os::raw::c_uint <=
                                        val) as std::os::raw::c_int
                           } else {
                               (0x100 as std::os::raw::c_int as std::os::raw::c_uint <= val &&
                                    val <=
                                        0xd7ff as std::os::raw::c_int as std::os::raw::c_uint
                                    ||
                                    0xe000 as std::os::raw::c_int as std::os::raw::c_uint <=
                                        val &&
                                        val <=
                                            0xfffd as std::os::raw::c_int as
                                                std::os::raw::c_uint ||
                                    0x10000 as std::os::raw::c_int as std::os::raw::c_uint <=
                                        val &&
                                        val <=
                                            0x10ffff as std::os::raw::c_int as
                                                std::os::raw::c_uint) as std::os::raw::c_int
                           } == 0 {
                            xmlErrEncodingInt(ctxt, XML_ERR_INVALID_CHAR,
                                              b"Char 0x%X out of allowed range\n\x00"
                                                  as *const u8 as
                                                  *const std::os::raw::c_char,
                                              val as std::os::raw::c_int);
                        }
                        return val as std::os::raw::c_int
                    }
                }
            }
            /*
     * An encoding problem may arise from a truncated input buffer
     * splitting a character in the middle. In that case do not raise
     * an error but return 0 to endicate an end of stream problem
     */
            if ctxt.is_null() || (*ctxt).input.is_null() ||
                   ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                        as std::os::raw::c_long) < 4 as std::os::raw::c_int as std::os::raw::c_long {
                *len = 0 as std::os::raw::c_int;
                return 0 as std::os::raw::c_int
            }
            /*
     * If we detect an UTF8 error that probably mean that the
     * input encoding didn't get properly advertised in the
     * declaration header. Report the error and switch the encoding
     * to ISO-Latin-1 (if you don't like this policy, just declare the
     * encoding !)
     */
            let mut buffer: [std::os::raw::c_char; 150] = [0; 150];
            snprintf(buffer.as_mut_ptr(), 149 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\x00" as *const u8
                         as *const std::os::raw::c_char,
                     *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int,
                     *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int,
                     *(*(*ctxt).input).cur.offset(2 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int,
                     *(*(*ctxt).input).cur.offset(3 as std::os::raw::c_int as isize)
                         as std::os::raw::c_int);
            __xmlErrEncoding(ctxt, XML_ERR_INVALID_CHAR,
                             b"Input is not proper UTF-8, indicate encoding !\n%s\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             buffer.as_mut_ptr() as *mut xmlChar,
                             0 as *const xmlChar);
            *len = 1 as std::os::raw::c_int;
            return *cur as std::os::raw::c_int
        } else {
            /* 1-byte code */
            *len = 1 as std::os::raw::c_int;
            return *cur as std::os::raw::c_int
        }
    } else {
        /*
     * Assume it's a fixed length encoding (1) with
     * a compatible encoding for the ASCII set, since
     * XML constructs only use < 128 chars
     */
        *len = 1 as std::os::raw::c_int;
        return *cur as std::os::raw::c_int
    };
}
/* *
 * xmlCopyCharMultiByte:
 * @out:  pointer to an array of xmlChar
 * @val:  the char value
 *
 * append the char value in the array
 *
 * Returns the number of xmlChar written
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyCharMultiByte(mut out: *mut xmlChar,
                                              mut val: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if out.is_null() { return 0 as std::os::raw::c_int }
    /*
     * We are supposed to handle UTF8, check it's valid
     * From rfc2044: encoding of the Unicode values on UTF-8:
     *
     * UCS-4 range (hex.)           UTF-8 octet sequence (binary)
     * 0000 0000-0000 007F   0xxxxxxx
     * 0000 0080-0000 07FF   110xxxxx 10xxxxxx
     * 0000 0800-0000 FFFF   1110xxxx 10xxxxxx 10xxxxxx
     */
    if val >= 0x80 as std::os::raw::c_int {
        let mut savedout: *mut xmlChar = out;
        let mut bits: std::os::raw::c_int = 0;
        if val < 0x800 as std::os::raw::c_int {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 =
                (val >> 6 as std::os::raw::c_int | 0xc0 as std::os::raw::c_int) as xmlChar;
            bits = 0 as std::os::raw::c_int
        } else if val < 0x10000 as std::os::raw::c_int {
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 =
                (val >> 12 as std::os::raw::c_int | 0xe0 as std::os::raw::c_int) as xmlChar;
            bits = 6 as std::os::raw::c_int
        } else if val < 0x110000 as std::os::raw::c_int {
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 =
                (val >> 18 as std::os::raw::c_int | 0xf0 as std::os::raw::c_int) as xmlChar;
            bits = 12 as std::os::raw::c_int
        } else {
            xmlErrEncodingInt(0 as xmlParserCtxtPtr, XML_ERR_INVALID_CHAR,
                              b"Internal error, xmlCopyCharMultiByte 0x%X out of bound\n\x00"
                                  as *const u8 as *const std::os::raw::c_char, val);
            return 0 as std::os::raw::c_int
        }
        while bits >= 0 as std::os::raw::c_int {
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 =
                (val >> bits & 0x3f as std::os::raw::c_int | 0x80 as std::os::raw::c_int) as
                    xmlChar;
            bits -= 6 as std::os::raw::c_int
        }
        return out.offset_from(savedout) as std::os::raw::c_long as
                   std::os::raw::c_int
    }
    *out = val as xmlChar;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlCopyChar:
 * @len:  Ignored, compatibility
 * @out:  pointer to an array of xmlChar
 * @val:  the char value
 *
 * append the char value in the array
 *
 * Returns the number of xmlChar written
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyChar(mut len: std::os::raw::c_int,
                                     mut out: *mut xmlChar,
                                     mut val: std::os::raw::c_int) -> std::os::raw::c_int {
    if out.is_null() { return 0 as std::os::raw::c_int }
    /* the len parameter is ignored */
    if val >= 0x80 as std::os::raw::c_int { return xmlCopyCharMultiByte(out, val) }
    *out = val as xmlChar;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlSwitchEncoding:
 * @ctxt:  the parser context
 * @enc:  the encoding value (number)
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSwitchEncoding(mut ctxt: xmlParserCtxtPtr,
                                           mut enc: xmlCharEncoding)
 -> std::os::raw::c_int {
    let mut handler: xmlCharEncodingHandlerPtr =
        0 as *mut xmlCharEncodingHandler;
    let mut len: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut ret: std::os::raw::c_int = 0;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    match enc as std::os::raw::c_int {
        -1 => {
            __xmlErrEncoding(ctxt, XML_ERR_UNKNOWN_ENCODING,
                             b"encoding unknown\n\x00" as *const u8 as
                                 *const std::os::raw::c_char, 0 as *const xmlChar,
                             0 as *const xmlChar);
            return -(1 as std::os::raw::c_int)
        }
        0 => {
            /* let's assume it's UTF-8 without the XML decl */
            (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
        1 => {
            /* default encoding, no conversion should be needed */
            (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
            /*
	     * Errata on XML-1.0 June 20 2001
	     * Specific handling of the Byte Order Mark for
	     * UTF-8
	     */
            if !(*ctxt).input.is_null() &&
                   *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xef as std::os::raw::c_int &&
                   *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xbb as std::os::raw::c_int &&
                   *(*(*ctxt).input).cur.offset(2 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xbf as std::os::raw::c_int {
                (*(*ctxt).input).cur =
                    (*(*ctxt).input).cur.offset(3 as std::os::raw::c_int as isize)
            }
            return 0 as std::os::raw::c_int
        }
        2 | 3 => {
            /*The raw input characters are encoded
         *in UTF-16. As we expect this function
         *to be called after xmlCharEncInFunc, we expect
         *ctxt->input->cur to contain UTF-8 encoded characters.
         *So the raw UTF16 Byte Order Mark
         *has also been converted into
         *an UTF-8 BOM. Let's skip that BOM.
         */
            if !(*ctxt).input.is_null() && !(*(*ctxt).input).cur.is_null() &&
                   *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xef as std::os::raw::c_int &&
                   *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xbb as std::os::raw::c_int &&
                   *(*(*ctxt).input).cur.offset(2 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xbf as std::os::raw::c_int {
                (*(*ctxt).input).cur =
                    (*(*ctxt).input).cur.offset(3 as std::os::raw::c_int as isize)
            }
            len = 90 as std::os::raw::c_int
        }
        9 => { len = 90 as std::os::raw::c_int }
        5 | 4 | 7 | 8 => { len = 180 as std::os::raw::c_int }
        6 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 22 | 19 | 20 | 21 =>
        {
            len = 45 as std::os::raw::c_int
        }
        _ => { }
    }
    handler = xmlGetCharEncodingHandler(enc);
    if handler.is_null() {
        /*
	 * Default handlers.
	 */
        match enc as std::os::raw::c_int {
            22 => {
                /* default encoding, no conversion should be needed */
                (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
                return 0 as std::os::raw::c_int
            }
            4 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"USC4 little endian\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            5 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"USC4 big endian\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            6 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"EBCDIC\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            7 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"UCS4 2143\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            8 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"UCS4 3412\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            9 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"UCS2\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 => {
                /*
		 * We used to keep the internal content in the
		 * document encoding however this turns being unmaintainable
		 * So xmlGetCharEncodingHandler() will return non-null
		 * values for this now.
		 */
                if (*ctxt).inputNr == 1 as std::os::raw::c_int &&
                       (*ctxt).encoding.is_null() && !(*ctxt).input.is_null()
                       && !(*(*ctxt).input).encoding.is_null() {
                    (*ctxt).encoding = xmlStrdup((*(*ctxt).input).encoding)
                }
                (*ctxt).charset = enc as std::os::raw::c_int;
                return 0 as std::os::raw::c_int
            }
            19 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"ISO-2022-JP\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            20 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"Shift_JIS\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            21 => {
                __xmlErrEncoding(ctxt, XML_ERR_UNSUPPORTED_ENCODING,
                                 b"encoding not supported %s\n\x00" as
                                     *const u8 as *const std::os::raw::c_char,
                                 b"EUC-JP\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 0 as *const xmlChar);
            }
            2 | 3 | _ => { }
        }
    }
    /*
     * TODO: We could recover from errors in external entites if we
     * didn't stop the parser. But most callers of this function don't
     * check the return value.
     */
    if handler.is_null() { xmlStopParser(ctxt); return -(1 as std::os::raw::c_int) }
    (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
    ret = xmlSwitchToEncodingInt(ctxt, handler, len);
    if ret < 0 as std::os::raw::c_int ||
           (*ctxt).errNo == XML_I18N_CONV_FAILED as std::os::raw::c_int {
        /*
	 * on encoding conversion errors, stop the parser
	 */
        xmlStopParser(ctxt);
        (*ctxt).errNo = XML_I18N_CONV_FAILED as std::os::raw::c_int
    }
    return ret;
}
/* *
 * xmlSwitchInputEncoding:
 * @ctxt:  the parser context
 * @input:  the input stream
 * @handler:  the encoding handler
 * @len:  the number of bytes to convert for the first line or -1
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
unsafe extern "C" fn xmlSwitchInputEncodingInt(mut ctxt: xmlParserCtxtPtr,
                                               mut input: xmlParserInputPtr,
                                               mut handler:
                                                   xmlCharEncodingHandlerPtr,
                                               mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut nbchars: std::os::raw::c_int = 0;
    if handler.is_null() { return -(1 as std::os::raw::c_int) }
    if input.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*input).buf.is_null() {
        if !(*(*input).buf).encoder.is_null() {
            /*
             * Check in case the auto encoding detetection triggered
             * in already.
             */
            if (*(*input).buf).encoder == handler { return 0 as std::os::raw::c_int }
            /*
             * "UTF-16" can be used for both LE and BE
             if ((!xmlStrncmp(BAD_CAST input->buf->encoder->name,
             BAD_CAST "UTF-16", 6)) &&
             (!xmlStrncmp(BAD_CAST handler->name,
             BAD_CAST "UTF-16", 6))) {
             return(0);
             }
             */
            /*
             * Note: this is a bit dangerous, but that's what it
             * takes to use nearly compatible signature for different
             * encodings.
             */
            xmlCharEncCloseFunc((*(*input).buf).encoder);
            (*(*input).buf).encoder = handler;
            return 0 as std::os::raw::c_int
        }
        (*(*input).buf).encoder = handler;
        /*
         * Is there already some content down the pipe to convert ?
         */
        if xmlBufIsEmpty((*(*input).buf).buffer) == 0 as std::os::raw::c_int {
            let mut processed: std::os::raw::c_int = 0;
            let mut use_0: std::os::raw::c_uint = 0;
            /*
             * Specific handling of the Byte Order Mark for
             * UTF-16
             */
            if !(*handler).name.is_null() &&
                   (strcmp((*handler).name,
                           b"UTF-16LE\x00" as *const u8 as
                               *const std::os::raw::c_char) == 0 ||
                        strcmp((*handler).name,
                               b"UTF-16\x00" as *const u8 as
                                   *const std::os::raw::c_char) == 0) &&
                   *(*input).cur.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xff as std::os::raw::c_int &&
                   *(*input).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xfe as std::os::raw::c_int {
                (*input).cur = (*input).cur.offset(2 as std::os::raw::c_int as isize)
            }
            if !(*handler).name.is_null() &&
                   strcmp((*handler).name,
                          b"UTF-16BE\x00" as *const u8 as *const std::os::raw::c_char)
                       == 0 &&
                   *(*input).cur.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xfe as std::os::raw::c_int &&
                   *(*input).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xff as std::os::raw::c_int {
                (*input).cur = (*input).cur.offset(2 as std::os::raw::c_int as isize)
            }
            /*
             * Errata on XML-1.0 June 20 2001
             * Specific handling of the Byte Order Mark for
             * UTF-8
             */
            if !(*handler).name.is_null() &&
                   strcmp((*handler).name,
                          b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 &&
                   *(*input).cur.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xef as std::os::raw::c_int &&
                   *(*input).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xbb as std::os::raw::c_int &&
                   *(*input).cur.offset(2 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == 0xbf as std::os::raw::c_int {
                (*input).cur = (*input).cur.offset(3 as std::os::raw::c_int as isize)
            }
            /*
             * Shrink the current input buffer.
             * Move it as the raw buffer and create a new input buffer
             */
            processed =
                (*input).cur.offset_from((*input).base) as
                    std::os::raw::c_long as std::os::raw::c_int;
            xmlBufShrink((*(*input).buf).buffer, processed as size_t);
            (*(*input).buf).raw = (*(*input).buf).buffer;
            (*(*input).buf).buffer = xmlBufCreate();
            (*(*input).buf).rawconsumed = processed as std::os::raw::c_ulong;
            use_0 = xmlBufUse((*(*input).buf).raw) as std::os::raw::c_uint;
            if (*ctxt).html != 0 {
                /*
                 * convert as much as possible of the buffer
                 */
                nbchars = xmlCharEncInput((*input).buf, 1 as std::os::raw::c_int)
            } else {
                /*
                 * convert just enough to get
                 * '<?xml version="1.0" encoding="xxx"?>'
                 * parsed with the autodetected encoding
                 * into the parser reading buffer.
                 */
                nbchars = xmlCharEncFirstLineInput((*input).buf, len)
            }
            xmlBufResetInput((*(*input).buf).buffer, input);
            if nbchars < 0 as std::os::raw::c_int {
                xmlErrInternal(ctxt,
                               b"switching encoding: encoder error\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar);
                return -(1 as std::os::raw::c_int)
            }
            (*(*input).buf).rawconsumed =
                (*(*input).buf).rawconsumed.wrapping_add((use_0 as
                                                              std::os::raw::c_ulong).wrapping_sub(xmlBufUse((*(*input).buf).raw)))
        }
        return 0 as std::os::raw::c_int
    } else {
        if (*input).length == 0 as std::os::raw::c_int {
            /*
	 * When parsing a static memory array one must know the
	 * size to be able to convert the buffer.
	 */
            xmlErrInternal(ctxt,
                           b"switching encoding : no input\n\x00" as *const u8
                               as *const std::os::raw::c_char, 0 as *const xmlChar);
            return -(1 as std::os::raw::c_int)
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSwitchInputEncoding:
 * @ctxt:  the parser context
 * @input:  the input stream
 * @handler:  the encoding handler
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSwitchInputEncoding(mut ctxt: xmlParserCtxtPtr,
                                                mut input: xmlParserInputPtr,
                                                mut handler:
                                                    xmlCharEncodingHandlerPtr)
 -> std::os::raw::c_int {
    return xmlSwitchInputEncodingInt(ctxt, input, handler,
                                     -(1 as std::os::raw::c_int));
}
/* ***********************************************************************
 *									*
 *		Commodity functions to switch encodings			*
 *									*
 ************************************************************************/
/* *
 * xmlSwitchToEncodingInt:
 * @ctxt:  the parser context
 * @handler:  the encoding handler
 * @len: the length to convert or -1
 *
 * change the input functions when discovering the character encoding
 * of a given entity, and convert only @len bytes of the output, this
 * is needed on auto detect to allows any declared encoding later to
 * convert the actual content after the xmlDecl
 *
 * Returns 0 in case of success, -1 otherwise
 */
unsafe extern "C" fn xmlSwitchToEncodingInt(mut ctxt: xmlParserCtxtPtr,
                                            mut handler:
                                                xmlCharEncodingHandlerPtr,
                                            mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !handler.is_null() {
        if !(*ctxt).input.is_null() {
            ret = xmlSwitchInputEncodingInt(ctxt, (*ctxt).input, handler, len)
        } else {
            xmlErrInternal(ctxt,
                           b"xmlSwitchToEncoding : no input\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar);
            return -(1 as std::os::raw::c_int)
        }
        /*
	 * The parsing is now done in UTF8 natively
	 */
        (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int
    } else { return -(1 as std::os::raw::c_int) }
    return ret;
}
/* *
 * xmlSwitchToEncoding:
 * @ctxt:  the parser context
 * @handler:  the encoding handler
 *
 * change the input functions when discovering the character encoding
 * of a given entity.
 *
 * Returns 0 in case of success, -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSwitchToEncoding(mut ctxt: xmlParserCtxtPtr,
                                             mut handler:
                                                 xmlCharEncodingHandlerPtr)
 -> std::os::raw::c_int {
    return xmlSwitchToEncodingInt(ctxt, handler, -(1 as std::os::raw::c_int));
}
/* ***********************************************************************
 *									*
 *	Commodity functions to handle entities processing		*
 *									*
 ************************************************************************/
/* *
 * xmlFreeInputStream:
 * @input:  an xmlParserInputPtr
 *
 * Free up an input stream.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeInputStream(mut input: xmlParserInputPtr) {
    if input.is_null() { return }
    if !(*input).filename.is_null() {
        xmlFree.expect("non-null function pointer")((*input).filename as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*input).directory.is_null() {
        xmlFree.expect("non-null function pointer")((*input).directory as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*input).encoding.is_null() {
        xmlFree.expect("non-null function pointer")((*input).encoding as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*input).version.is_null() {
        xmlFree.expect("non-null function pointer")((*input).version as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if (*input).free.is_some() && !(*input).base.is_null() {
        (*input).free.expect("non-null function pointer")((*input).base as
                                                              *mut xmlChar);
    }
    if !(*input).buf.is_null() { xmlFreeParserInputBuffer((*input).buf); }
    xmlFree.expect("non-null function pointer")(input as *mut std::os::raw::c_void);
}
/* *
 * xmlNewInputStream:
 * @ctxt:  an XML parser context
 *
 * Create a new input stream structure.
 *
 * Returns the new input stream or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewInputStream(mut ctxt: xmlParserCtxtPtr)
 -> xmlParserInputPtr {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    input =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlParserInput>()
                                                          as std::os::raw::c_ulong) as
            xmlParserInputPtr;
    if input.is_null() {
        xmlErrMemory(ctxt,
                     b"couldn\'t allocate a new input stream\n\x00" as
                         *const u8 as *const std::os::raw::c_char);
        return 0 as xmlParserInputPtr
    }
    memset(input as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlParserInput>() as std::os::raw::c_ulong);
    (*input).line = 1 as std::os::raw::c_int;
    (*input).col = 1 as std::os::raw::c_int;
    (*input).standalone = -(1 as std::os::raw::c_int);
    /*
     * If the context is NULL the id cannot be initialized, but that
     * should not happen while parsing which is the situation where
     * the id is actually needed.
     */
    if !ctxt.is_null() {
        let fresh4 = (*ctxt).input_id;
        (*ctxt).input_id = (*ctxt).input_id + 1;
        (*input).id = fresh4
    }
    return input;
}
/* *
 * xmlNewIOInputStream:
 * @ctxt:  an XML parser context
 * @input:  an I/O Input
 * @enc:  the charset encoding if known
 *
 * Create a new input stream structure encapsulating the @input into
 * a stream suitable for the parser.
 *
 * Returns the new input stream or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewIOInputStream(mut ctxt: xmlParserCtxtPtr,
                                             mut input:
                                                 xmlParserInputBufferPtr,
                                             mut enc: xmlCharEncoding)
 -> xmlParserInputPtr {
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if input.is_null() { return 0 as xmlParserInputPtr }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"new input from I/O\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() { return 0 as xmlParserInputPtr }
    (*inputStream).filename = 0 as *const std::os::raw::c_char;
    (*inputStream).buf = input;
    xmlBufResetInput((*(*inputStream).buf).buffer, inputStream);
    if enc as std::os::raw::c_int != XML_CHAR_ENCODING_NONE as std::os::raw::c_int {
        xmlSwitchEncoding(ctxt, enc);
    }
    return inputStream;
}
/* *
 * xmlNewEntityInputStream:
 * @ctxt:  an XML parser context
 * @entity:  an Entity pointer
 *
 * Create a new input stream based on an xmlEntityPtr
 *
 * Returns the new input stream or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewEntityInputStream(mut ctxt: xmlParserCtxtPtr,
                                                 mut entity: xmlEntityPtr)
 -> xmlParserInputPtr {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if entity.is_null() {
        xmlErrInternal(ctxt,
                       b"xmlNewEntityInputStream entity = NULL\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar);
        return 0 as xmlParserInputPtr
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"new input from entity: %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   (*entity).name);
    }
    if (*entity).content.is_null() {
        match (*entity).etype as std::os::raw::c_uint {
            3 => {
                xmlErrInternal(ctxt,
                               b"Cannot parse entity %s\n\x00" as *const u8 as
                                   *const std::os::raw::c_char, (*entity).name);
            }
            2 | 5 => {
                return xmlLoadExternalEntity((*entity).URI as
                                                 *mut std::os::raw::c_char,
                                             (*entity).ExternalID as
                                                 *mut std::os::raw::c_char, ctxt)
            }
            1 => {
                xmlErrInternal(ctxt,
                               b"Internal entity %s without content !\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               (*entity).name);
            }
            4 => {
                xmlErrInternal(ctxt,
                               b"Internal parameter entity %s without content !\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               (*entity).name);
            }
            6 => {
                xmlErrInternal(ctxt,
                               b"Predefined entity %s without content !\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               (*entity).name);
            }
            _ => { }
        }
        return 0 as xmlParserInputPtr
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() { return 0 as xmlParserInputPtr }
    if !(*entity).URI.is_null() {
        (*input).filename =
            xmlStrdup((*entity).URI as *mut xmlChar) as *mut std::os::raw::c_char
    }
    (*input).base = (*entity).content;
    if (*entity).length == 0 as std::os::raw::c_int {
        (*entity).length = xmlStrlen((*entity).content)
    }
    (*input).cur = (*entity).content;
    (*input).length = (*entity).length;
    (*input).end =
        &mut *(*entity).content.offset((*input).length as isize) as
            *mut xmlChar;
    return input;
}
/* *
 * Input Streams.
 */
/* *
 * xmlNewStringInputStream:
 * @ctxt:  an XML parser context
 * @buffer:  an memory buffer
 *
 * Create a new input stream based on a memory buffer.
 * Returns the new input stream
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewStringInputStream(mut ctxt: xmlParserCtxtPtr,
                                                 mut buffer: *const xmlChar)
 -> xmlParserInputPtr {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if buffer.is_null() {
        xmlErrInternal(ctxt,
                       b"xmlNewStringInputStream string = NULL\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar);
        return 0 as xmlParserInputPtr
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"new fixed input: %.30s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   buffer);
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() {
        xmlErrMemory(ctxt,
                     b"couldn\'t allocate a new input stream\n\x00" as
                         *const u8 as *const std::os::raw::c_char);
        return 0 as xmlParserInputPtr
    }
    (*input).base = buffer;
    (*input).cur = buffer;
    (*input).length = xmlStrlen(buffer);
    (*input).end =
        &*buffer.offset((*input).length as isize) as *const xmlChar;
    return input;
}
/* *
 * xmlNewInputFromFile:
 * @ctxt:  an XML parser context
 * @filename:  the filename to use as entity
 *
 * Create a new input stream based on a file or an URL.
 *
 * Returns the new input stream or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewInputFromFile(mut ctxt: xmlParserCtxtPtr,
                                             mut filename:
                                                 *const std::os::raw::c_char)
 -> xmlParserInputPtr {
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut directory: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"new input from file: %s\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   filename);
    }
    if ctxt.is_null() { return 0 as xmlParserInputPtr }
    buf =
        xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        if filename.is_null() {
            __xmlLoaderErr(ctxt as *mut std::os::raw::c_void,
                           b"failed to load external entity: NULL filename \n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const std::os::raw::c_char);
        } else {
            __xmlLoaderErr(ctxt as *mut std::os::raw::c_void,
                           b"failed to load external entity \"%s\"\n\x00" as
                               *const u8 as *const std::os::raw::c_char, filename);
        }
        return 0 as xmlParserInputPtr
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() { return 0 as xmlParserInputPtr }
    (*inputStream).buf = buf;
    inputStream = xmlCheckHTTPInput(ctxt, inputStream);
    if inputStream.is_null() { return 0 as xmlParserInputPtr }
    if (*inputStream).filename.is_null() {
        URI = xmlStrdup(filename as *mut xmlChar)
    } else { URI = xmlStrdup((*inputStream).filename as *mut xmlChar) }
    directory = xmlParserGetDirectory(URI as *const std::os::raw::c_char);
    if !(*inputStream).filename.is_null() {
        xmlFree.expect("non-null function pointer")((*inputStream).filename as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    (*inputStream).filename =
        xmlCanonicPath(URI as *const xmlChar) as *mut std::os::raw::c_char;
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut std::os::raw::c_char
                                                        as *mut std::os::raw::c_void);
    }
    (*inputStream).directory = directory;
    xmlBufResetInput((*(*inputStream).buf).buffer, inputStream);
    if (*ctxt).directory.is_null() && !directory.is_null() {
        (*ctxt).directory =
            xmlStrdup(directory as *const xmlChar) as *mut std::os::raw::c_char
    }
    return inputStream;
}
/* ***********************************************************************
 *									*
 *		Commodity functions to handle parser contexts		*
 *									*
 ************************************************************************/
/* *
 * xmlInitParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Initialize a parser context
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitParserCtxt(mut ctxt: xmlParserCtxtPtr)
 -> std::os::raw::c_int {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() {
        xmlErrInternal(0 as xmlParserCtxtPtr,
                       b"Got NULL parser context\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    xmlDefaultSAXHandlerInit();
    if (*ctxt).dict.is_null() { (*ctxt).dict = xmlDictCreate() }
    if (*ctxt).dict.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot initialize parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    xmlDictSetLimit((*ctxt).dict, 10000000 as std::os::raw::c_int as size_t);
    if (*ctxt).sax.is_null() {
        (*ctxt).sax =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSAXHandler>()
                                                              as
                                                              std::os::raw::c_ulong)
                as *mut xmlSAXHandler
    }
    if (*ctxt).sax.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot initialize parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    } else { xmlSAXVersion((*ctxt).sax, 2 as std::os::raw::c_int); }
    (*ctxt).maxatts = 0 as std::os::raw::c_int;
    (*ctxt).atts = 0 as *mut *const xmlChar;
    /* Allocate the Input stack */
    if (*ctxt).inputTab.is_null() {
        (*ctxt).inputTab =
            xmlMalloc.expect("non-null function pointer")((5 as std::os::raw::c_int as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlParserInputPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlParserInputPtr;
        (*ctxt).inputMax = 5 as std::os::raw::c_int
    }
    if (*ctxt).inputTab.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot initialize parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        (*ctxt).inputNr = 0 as std::os::raw::c_int;
        (*ctxt).inputMax = 0 as std::os::raw::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        return -(1 as std::os::raw::c_int)
    }
    loop  {
        input = inputPop(ctxt);
        if input.is_null() { break ; }
        /* Non consuming */
        xmlFreeInputStream(input);
    }
    (*ctxt).inputNr = 0 as std::os::raw::c_int;
    (*ctxt).input = 0 as xmlParserInputPtr;
    (*ctxt).version = 0 as *const xmlChar;
    (*ctxt).encoding = 0 as *const xmlChar;
    (*ctxt).standalone = -(1 as std::os::raw::c_int);
    (*ctxt).hasExternalSubset = 0 as std::os::raw::c_int;
    (*ctxt).hasPErefs = 0 as std::os::raw::c_int;
    (*ctxt).html = 0 as std::os::raw::c_int;
    (*ctxt).external = 0 as std::os::raw::c_int;
    (*ctxt).instate = XML_PARSER_START;
    (*ctxt).token = 0 as std::os::raw::c_int;
    (*ctxt).directory = 0 as *mut std::os::raw::c_char;
    /* Allocate the Node stack */
    if (*ctxt).nodeTab.is_null() {
        (*ctxt).nodeTab =
            xmlMalloc.expect("non-null function pointer")((10 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        (*ctxt).nodeMax = 10 as std::os::raw::c_int
    }
    if (*ctxt).nodeTab.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot initialize parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        (*ctxt).nodeNr = 0 as std::os::raw::c_int;
        (*ctxt).nodeMax = 0 as std::os::raw::c_int;
        (*ctxt).node = 0 as xmlNodePtr;
        (*ctxt).inputNr = 0 as std::os::raw::c_int;
        (*ctxt).inputMax = 0 as std::os::raw::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        return -(1 as std::os::raw::c_int)
    }
    (*ctxt).nodeNr = 0 as std::os::raw::c_int;
    (*ctxt).node = 0 as xmlNodePtr;
    /* Allocate the Name stack */
    if (*ctxt).nameTab.is_null() {
        (*ctxt).nameTab =
            xmlMalloc.expect("non-null function pointer")((10 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut *const xmlChar;
        (*ctxt).nameMax = 10 as std::os::raw::c_int
    }
    if (*ctxt).nameTab.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot initialize parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        (*ctxt).nodeNr = 0 as std::os::raw::c_int;
        (*ctxt).nodeMax = 0 as std::os::raw::c_int;
        (*ctxt).node = 0 as xmlNodePtr;
        (*ctxt).inputNr = 0 as std::os::raw::c_int;
        (*ctxt).inputMax = 0 as std::os::raw::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        (*ctxt).nameNr = 0 as std::os::raw::c_int;
        (*ctxt).nameMax = 0 as std::os::raw::c_int;
        (*ctxt).name = 0 as *const xmlChar;
        return -(1 as std::os::raw::c_int)
    }
    (*ctxt).nameNr = 0 as std::os::raw::c_int;
    (*ctxt).name = 0 as *const xmlChar;
    /* Allocate the space stack */
    if (*ctxt).spaceTab.is_null() {
        (*ctxt).spaceTab =
            xmlMalloc.expect("non-null function pointer")((10 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut std::os::raw::c_int;
        (*ctxt).spaceMax = 10 as std::os::raw::c_int
    }
    if (*ctxt).spaceTab.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot initialize parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        (*ctxt).nodeNr = 0 as std::os::raw::c_int;
        (*ctxt).nodeMax = 0 as std::os::raw::c_int;
        (*ctxt).node = 0 as xmlNodePtr;
        (*ctxt).inputNr = 0 as std::os::raw::c_int;
        (*ctxt).inputMax = 0 as std::os::raw::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        (*ctxt).nameNr = 0 as std::os::raw::c_int;
        (*ctxt).nameMax = 0 as std::os::raw::c_int;
        (*ctxt).name = 0 as *const xmlChar;
        (*ctxt).spaceNr = 0 as std::os::raw::c_int;
        (*ctxt).spaceMax = 0 as std::os::raw::c_int;
        (*ctxt).space = 0 as *mut std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    (*ctxt).spaceNr = 1 as std::os::raw::c_int;
    (*ctxt).spaceMax = 10 as std::os::raw::c_int;
    *(*ctxt).spaceTab.offset(0 as std::os::raw::c_int as isize) = -(1 as std::os::raw::c_int);
    (*ctxt).space =
        &mut *(*ctxt).spaceTab.offset(0 as std::os::raw::c_int as isize) as
            *mut std::os::raw::c_int;
    (*ctxt).userData = ctxt as *mut std::os::raw::c_void;
    (*ctxt).myDoc = 0 as xmlDocPtr;
    (*ctxt).wellFormed = 1 as std::os::raw::c_int;
    (*ctxt).nsWellFormed = 1 as std::os::raw::c_int;
    (*ctxt).valid = 1 as std::os::raw::c_int;
    (*ctxt).loadsubset = *__xmlLoadExtDtdDefaultValue();
    if (*ctxt).loadsubset != 0 {
        (*ctxt).options |= XML_PARSE_DTDLOAD as std::os::raw::c_int
    }
    (*ctxt).validate = *__xmlDoValidityCheckingDefaultValue();
    (*ctxt).pedantic = *__xmlPedanticParserDefaultValue();
    if (*ctxt).pedantic != 0 {
        (*ctxt).options |= XML_PARSE_PEDANTIC as std::os::raw::c_int
    }
    (*ctxt).linenumbers = *__xmlLineNumbersDefaultValue();
    (*ctxt).keepBlanks = *__xmlKeepBlanksDefaultValue();
    if (*ctxt).keepBlanks == 0 as std::os::raw::c_int {
        (*(*ctxt).sax).ignorableWhitespace =
            Some(xmlSAX2IgnorableWhitespace as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar, _: std::os::raw::c_int)
                         -> ());
        (*ctxt).options |= XML_PARSE_NOBLANKS as std::os::raw::c_int
    }
    (*ctxt).vctxt.finishDtd = 0xabcd1234 as std::os::raw::c_uint;
    (*ctxt).vctxt.userData = ctxt as *mut std::os::raw::c_void;
    (*ctxt).vctxt.error =
        Some(xmlParserValidityError as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    (*ctxt).vctxt.warning =
        Some(xmlParserValidityWarning as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    if (*ctxt).validate != 0 {
        if *__xmlGetWarningsDefaultValue() == 0 as std::os::raw::c_int {
            (*ctxt).vctxt.warning = None
        } else {
            (*ctxt).vctxt.warning =
                Some(xmlParserValidityWarning as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const std::os::raw::c_char, _: ...)
                             -> ())
        }
        (*ctxt).vctxt.nodeMax = 0 as std::os::raw::c_int;
        (*ctxt).options |= XML_PARSE_DTDVALID as std::os::raw::c_int
    }
    (*ctxt).replaceEntities = *__xmlSubstituteEntitiesDefaultValue();
    if (*ctxt).replaceEntities != 0 {
        (*ctxt).options |= XML_PARSE_NOENT as std::os::raw::c_int
    }
    (*ctxt).record_info = 0 as std::os::raw::c_int;
    (*ctxt).nbChars = 0 as std::os::raw::c_int as std::os::raw::c_long;
    (*ctxt).checkIndex = 0 as std::os::raw::c_int as std::os::raw::c_long;
    (*ctxt).inSubset = 0 as std::os::raw::c_int;
    (*ctxt).errNo = XML_ERR_OK as std::os::raw::c_int;
    (*ctxt).depth = 0 as std::os::raw::c_int;
    (*ctxt).charset = XML_CHAR_ENCODING_UTF8 as std::os::raw::c_int;
    (*ctxt).catalogs = 0 as *mut std::os::raw::c_void;
    (*ctxt).nbentities = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*ctxt).sizeentities = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*ctxt).sizeentcopy = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*ctxt).input_id = 1 as std::os::raw::c_int;
    xmlInitNodeInfoSeq(&mut (*ctxt).node_seq);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlFreeParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Free all the memory used by a parser context. However the parsed
 * document in ctxt->myDoc is not freed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeParserCtxt(mut ctxt: xmlParserCtxtPtr) {
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    if ctxt.is_null() { return }
    loop  {
        input = inputPop(ctxt);
        if input.is_null() { break ; }
        /* Non consuming */
        xmlFreeInputStream(input);
    }
    if !(*ctxt).spaceTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).spaceTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).nameTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nameTab as
                                                        *mut *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nodeTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).nodeInfoTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nodeInfoTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).inputTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).inputTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).version.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).version as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).encoding.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).encoding as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).extSubURI.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).extSubURI as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).extSubSystem.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).extSubSystem as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).sax.is_null() &&
           (*ctxt).sax != __xmlDefaultSAXHandler() as xmlSAXHandlerPtr {
        /* LIBXML_SAX1_ENABLED */
        xmlFree.expect("non-null function pointer")((*ctxt).sax as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).directory.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).directory as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).vctxt.nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).vctxt.nodeTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).atts.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).atts as
                                                        *mut *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).dict.is_null() { xmlDictFree((*ctxt).dict); }
    if !(*ctxt).nsTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nsTab as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).pushTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).pushTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).attallocs.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).attallocs as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).attsDefault.is_null() {
        xmlHashFree((*ctxt).attsDefault,
                    Some(xmlHashDefaultDeallocator as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()));
    }
    if !(*ctxt).attsSpecial.is_null() {
        xmlHashFree((*ctxt).attsSpecial, None);
    }
    if !(*ctxt).freeElems.is_null() {
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        let mut next: xmlNodePtr = 0 as *mut xmlNode;
        cur = (*ctxt).freeElems;
        while !cur.is_null() {
            next = (*cur).next;
            xmlFree.expect("non-null function pointer")(cur as
                                                            *mut std::os::raw::c_void);
            cur = next
        }
    }
    if !(*ctxt).freeAttrs.is_null() {
        let mut cur_0: xmlAttrPtr = 0 as *mut xmlAttr;
        let mut next_0: xmlAttrPtr = 0 as *mut xmlAttr;
        cur_0 = (*ctxt).freeAttrs;
        while !cur_0.is_null() {
            next_0 = (*cur_0).next;
            xmlFree.expect("non-null function pointer")(cur_0 as
                                                            *mut std::os::raw::c_void);
            cur_0 = next_0
        }
    }
    /*
     * cleanup the error strings
     */
    if !(*ctxt).lastError.message.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.message
                                                        as *mut std::os::raw::c_void);
    }
    if !(*ctxt).lastError.file.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.file as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).lastError.str1.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.str1 as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).lastError.str2.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.str2 as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).lastError.str3.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).lastError.str3 as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).catalogs.is_null() { xmlCatalogFreeLocal((*ctxt).catalogs); }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/* *
 * xmlNewParserCtxt:
 *
 * Allocate and initialize a new parser context.
 *
 * Returns the xmlParserCtxtPtr or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewParserCtxt() -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    ctxt =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlParserCtxtPtr;
    if ctxt.is_null() {
        xmlErrMemory(0 as xmlParserCtxtPtr,
                     b"cannot allocate parser context\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
        return 0 as xmlParserCtxtPtr
    }
    memset(ctxt as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlParserCtxt>() as std::os::raw::c_ulong);
    if xmlInitParserCtxt(ctxt) < 0 as std::os::raw::c_int {
        xmlFreeParserCtxt(ctxt);
        return 0 as xmlParserCtxtPtr
    }
    return ctxt;
}
/* ***********************************************************************
 *									*
 *		Handling of node informations				*
 *									*
 ************************************************************************/
/* *
 * xmlClearParserCtxt:
 * @ctxt:  an XML parser context
 *
 * Clear (release owned resources) and reinitialize a parser context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlClearParserCtxt(mut ctxt: xmlParserCtxtPtr) {
    if ctxt.is_null() { return }
    xmlClearNodeInfoSeq(&mut (*ctxt).node_seq);
    xmlCtxtReset(ctxt);
}
/* *
 * xmlParserFindNodeInfo:
 * @ctx:  an XML parser context
 * @node:  an XML node within the tree
 *
 * Find the parser node info struct for a given node
 *
 * Returns an xmlParserNodeInfo block pointer or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserFindNodeInfo(ctx: xmlParserCtxtPtr,
                                               node: xmlNodePtr)
 -> *const xmlParserNodeInfo {
    let mut pos: std::os::raw::c_ulong = 0;
    if ctx.is_null() || node.is_null() {
        return 0 as *const xmlParserNodeInfo
    }
    /* Find position where node should be at */
    pos = xmlParserFindNodeInfoIndex(&mut (*ctx).node_seq, node);
    if pos < (*ctx).node_seq.length &&
           (*(*ctx).node_seq.buffer.offset(pos as isize)).node ==
               node as *const _xmlNode {
        return &mut *(*ctx).node_seq.buffer.offset(pos as isize) as
                   *mut xmlParserNodeInfo
    } else { return 0 as *const xmlParserNodeInfo };
}
/* *
 * xmlInitNodeInfoSeq:
 * @seq:  a node info sequence pointer
 *
 * -- Initialize (set to initial state) node info sequence
 */
#[no_mangle]
pub unsafe extern "C" fn xmlInitNodeInfoSeq(mut seq:
                                                xmlParserNodeInfoSeqPtr) {
    if seq.is_null() { return }
    (*seq).length = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*seq).maximum = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    (*seq).buffer = 0 as *mut xmlParserNodeInfo;
}
/* *
 * xmlClearNodeInfoSeq:
 * @seq:  a node info sequence pointer
 *
 * -- Clear (release memory and reinitialize) node
 *   info sequence
 */
#[no_mangle]
pub unsafe extern "C" fn xmlClearNodeInfoSeq(mut seq:
                                                 xmlParserNodeInfoSeqPtr) {
    if seq.is_null() { return }
    if !(*seq).buffer.is_null() {
        xmlFree.expect("non-null function pointer")((*seq).buffer as
                                                        *mut std::os::raw::c_void);
    }
    xmlInitNodeInfoSeq(seq);
}
/* *
 * xmlParserFindNodeInfoIndex:
 * @seq:  a node info sequence pointer
 * @node:  an XML node pointer
 *
 *
 * xmlParserFindNodeInfoIndex : Find the index that the info record for
 *   the given node is or should be at in a sorted sequence
 *
 * Returns a long indicating the position of the record
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserFindNodeInfoIndex(seq:
                                                        xmlParserNodeInfoSeqPtr,
                                                    node: xmlNodePtr)
 -> std::os::raw::c_ulong {
    let mut upper: std::os::raw::c_ulong = 0;
    let mut lower: std::os::raw::c_ulong = 0;
    let mut middle: std::os::raw::c_ulong = 0;
    let mut found: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if seq.is_null() || node.is_null() {
        return -(1 as std::os::raw::c_int) as std::os::raw::c_ulong
    }
    /* Do a binary search for the key */
    lower = 1 as std::os::raw::c_int as std::os::raw::c_ulong;
    upper = (*seq).length;
    middle = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    while lower <= upper && found == 0 {
        middle =
            lower.wrapping_add(upper.wrapping_sub(lower).wrapping_div(2 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          std::os::raw::c_ulong));
        if node ==
               (*(*seq).buffer.offset(middle.wrapping_sub(1 as std::os::raw::c_int as
                                                              std::os::raw::c_ulong)
                                          as isize)).node as xmlNodePtr {
            found = 1 as std::os::raw::c_int
        } else if node <
                      (*(*seq).buffer.offset(middle.wrapping_sub(1 as
                                                                     std::os::raw::c_int
                                                                     as
                                                                     std::os::raw::c_ulong)
                                                 as isize)).node as xmlNodePtr
         {
            upper = middle.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        } else {
            lower = middle.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        }
    }
    /* Return position */
    if middle == 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           (*(*seq).buffer.offset(middle.wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                                      isize)).node < node as *const _xmlNode {
        return middle
    } else { return middle.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) };
}
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
/* *
 * xmlParserAddNodeInfo:
 * @ctxt:  an XML parser context
 * @info:  a node info sequence pointer
 *
 * Insert node info record into the sorted sequence
 */
#[no_mangle]
pub unsafe extern "C" fn xmlParserAddNodeInfo(mut ctxt: xmlParserCtxtPtr,
                                              info: xmlParserNodeInfoPtr) {
    let mut pos: std::os::raw::c_ulong = 0;
    if ctxt.is_null() || info.is_null() { return }
    /* Find pos and check to see if node is already in the sequence */
    pos =
        xmlParserFindNodeInfoIndex(&mut (*ctxt).node_seq,
                                   (*info).node as xmlNodePtr);
    if pos < (*ctxt).node_seq.length && !(*ctxt).node_seq.buffer.is_null() &&
           (*(*ctxt).node_seq.buffer.offset(pos as isize)).node ==
               (*info).node {
        *(*ctxt).node_seq.buffer.offset(pos as isize) = *info
    } else {
        /* Otherwise, we need to add new node to buffer */
        if (*ctxt).node_seq.length.wrapping_add(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong) >
               (*ctxt).node_seq.maximum || (*ctxt).node_seq.buffer.is_null() {
            let mut tmp_buffer: *mut xmlParserNodeInfo =
                0 as *mut xmlParserNodeInfo;
            let mut byte_size: std::os::raw::c_uint = 0;
            if (*ctxt).node_seq.maximum == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                (*ctxt).node_seq.maximum = 2 as std::os::raw::c_int as std::os::raw::c_ulong
            }
            byte_size =
                (::std::mem::size_of::<xmlParserNodeInfo>() as
                     std::os::raw::c_ulong).wrapping_mul((2 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong).wrapping_mul((*ctxt).node_seq.maximum))
                    as std::os::raw::c_uint;
            if (*ctxt).node_seq.buffer.is_null() {
                tmp_buffer =
                    xmlMalloc.expect("non-null function pointer")(byte_size as
                                                                      size_t)
                        as *mut xmlParserNodeInfo
            } else {
                tmp_buffer =
                    xmlRealloc.expect("non-null function pointer")((*ctxt).node_seq.buffer
                                                                       as
                                                                       *mut std::os::raw::c_void,
                                                                   byte_size
                                                                       as
                                                                       size_t)
                        as *mut xmlParserNodeInfo
            }
            if tmp_buffer.is_null() {
                xmlErrMemory(ctxt,
                             b"failed to allocate buffer\n\x00" as *const u8
                                 as *const std::os::raw::c_char);
                return
            }
            (*ctxt).node_seq.buffer = tmp_buffer;
            (*ctxt).node_seq.maximum =
                (*ctxt).node_seq.maximum.wrapping_mul(2 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong)
        }
        /* If position is not at end, move elements out of the way */
        if pos != (*ctxt).node_seq.length {
            let mut i: std::os::raw::c_ulong = 0;
            i = (*ctxt).node_seq.length;
            while i > pos {
                *(*ctxt).node_seq.buffer.offset(i as isize) =
                    *(*ctxt).node_seq.buffer.offset(i.wrapping_sub(1 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong)
                                                        as isize);
                i = i.wrapping_sub(1)
            }
        }
        /* Copy element and increase length */
        *(*ctxt).node_seq.buffer.offset(pos as isize) = *info;
        (*ctxt).node_seq.length = (*ctxt).node_seq.length.wrapping_add(1)
    };
}
/* ***********************************************************************
 *									*
 *		Defaults settings					*
 *									*
 ************************************************************************/
/* *
 * xmlPedanticParserDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for enabling pedantic warnings.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlPedanticParserDefault(mut val: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut old: std::os::raw::c_int = *__xmlPedanticParserDefaultValue();
    *__xmlPedanticParserDefaultValue() = val;
    return old;
}
/* *
 * xmlLineNumbersDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for enabling line numbers in elements
 * contents. This may break on old application and is turned off by default.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlLineNumbersDefault(mut val: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut old: std::os::raw::c_int = *__xmlLineNumbersDefaultValue();
    *__xmlLineNumbersDefaultValue() = val;
    return old;
}
/* *
 * xmlSubstituteEntitiesDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for default entity support.
 * Initially the parser always keep entity references instead of substituting
 * entity values in the output. This function has to be used to change the
 * default parser behavior
 * SAX::substituteEntities() has to be used for changing that on a file by
 * file basis.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSubstituteEntitiesDefault(mut val: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut old: std::os::raw::c_int = *__xmlSubstituteEntitiesDefaultValue();
    *__xmlSubstituteEntitiesDefaultValue() = val;
    return old;
}
/*
 * Input functions
 */
/*
 * Basic parsing Interfaces
 */
/* LIBXML_SAX1_ENABLED */
/* *
 * xmlKeepBlanksDefault:
 * @val:  int 0 or 1
 *
 * Set and return the previous value for default blanks text nodes support.
 * The 1.x version of the parser used an heuristic to try to detect
 * ignorable white spaces. As a result the SAX callback was generating
 * xmlSAX2IgnorableWhitespace() callbacks instead of characters() one, and when
 * using the DOM output text nodes containing those blanks were not generated.
 * The 2.x and later version will switch to the XML standard way and
 * ignorableWhitespace() are only generated when running the parser in
 * validating mode and when the current element doesn't allow CDATA or
 * mixed content.
 * This function is provided as a way to force the standard behavior
 * on 1.X libs and to switch back to the old mode for compatibility when
 * running 1.X client code on 2.X . Upgrade of 1.X code should be done
 * by using xmlIsBlankNode() commodity function to detect the "empty"
 * nodes generated.
 * This value also affect autogeneration of indentation when saving code
 * if blanks sections are kept, indentation is not generated.
 *
 * Returns the last value for 0 for no substitution, 1 for substitution.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlKeepBlanksDefault(mut val: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut old: std::os::raw::c_int = *__xmlKeepBlanksDefaultValue();
    *__xmlKeepBlanksDefaultValue() = val;
    if val == 0 { *__xmlIndentTreeOutput() = 1 as std::os::raw::c_int }
    return old;
}
/* __INCLUDE_ELFGCCHACK */
