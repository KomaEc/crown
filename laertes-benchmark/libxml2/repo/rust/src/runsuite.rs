
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
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGParserCtxt;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlXPathCompExpr;
    #[no_mangle]
    static mut stderr: *mut FILE;
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
    fn vfprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ::std::ffi::VaList)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                 _: *const std::os::raw::c_char, _: ::std::ffi::VaList)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn __xstat(__ver: std::os::raw::c_int, __filename: *const std::os::raw::c_char,
               __stat_buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufferCreate() -> xmlBufferPtr;
    #[no_mangle]
    fn xmlBufferFree(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlBufferAdd(buf: xmlBufferPtr, str: *const xmlChar, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufferEmpty(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* LIBXML_TREE_ENABLED */
    /*
 * Navigating.
 */
    #[no_mangle]
    fn xmlGetLineNo(node: *const xmlNode) -> std::os::raw::c_long;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeDump(buf: xmlBufferPtr, doc: xmlDocPtr, cur: xmlNodePtr,
                   level: std::os::raw::c_int, format: std::os::raw::c_int) -> std::os::raw::c_int;
    /*
 * Use the following function to reset the two global variables
 * xmlGenericError and xmlGenericErrorContext.
 */
    #[no_mangle]
    fn xmlSetGenericErrorFunc(ctx: *mut std::os::raw::c_void,
                              handler: xmlGenericErrorFunc);
    #[no_mangle]
    fn xmlResetLastError();
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
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
    fn xmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                     URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                     options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader);
    /*
 * A predefined entity loader disabling network accesses
 */
    #[no_mangle]
    fn xmlNoNetExternalEntityLoader(URL: *const std::os::raw::c_char,
                                    ID: *const std::os::raw::c_char,
                                    ctxt: xmlParserCtxtPtr)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlInitParser();
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlPedanticParserDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlGetWarningsDefaultValue() -> *mut std::os::raw::c_int;
    /* *
 * Input Streams.
 */
    #[no_mangle]
    fn xmlNewStringInputStream(ctxt: xmlParserCtxtPtr, buffer: *const xmlChar)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlSchemaNewParserCtxt(URL: *const std::os::raw::c_char)
     -> xmlSchemaParserCtxtPtr;
    #[no_mangle]
    fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
    #[no_mangle]
    fn xmlSchemaSetParserErrors(ctxt: xmlSchemaParserCtxtPtr,
                                err: xmlSchemaValidityErrorFunc,
                                warn: xmlSchemaValidityWarningFunc,
                                ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    #[no_mangle]
    fn xmlSchemaFree(schema: xmlSchemaPtr);
    #[no_mangle]
    fn xmlSchemaSetValidErrors(ctxt: xmlSchemaValidCtxtPtr,
                               err: xmlSchemaValidityErrorFunc,
                               warn: xmlSchemaValidityWarningFunc,
                               ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    #[no_mangle]
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    #[no_mangle]
    fn xmlSchemaValidateDoc(ctxt: xmlSchemaValidCtxtPtr, instance: xmlDocPtr)
     -> std::os::raw::c_int;
    /*
 * Summary: implementation of the Relax-NG validation
 * Description: implementation of the Relax-NG validation
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlRelaxNGValidityErrorFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of an error callback from a Relax-NG validation
 */
    /* *
 * xmlRelaxNGValidityWarningFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of a warning callback from a Relax-NG validation
 */
    /* *
 * A schemas validation context
 */
    /*
 * xmlRelaxNGValidErr:
 *
 * List of possible Relax NG validation errors
 */
    /*
 * xmlRelaxNGParserFlags:
 *
 * List of possible Relax NG Parser flags
 */
    /*
 * Interfaces for parsing.
 */
    #[no_mangle]
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    #[no_mangle]
    fn xmlRelaxNGNewMemParserCtxt(buffer: *const std::os::raw::c_char,
                                  size: std::os::raw::c_int)
     -> xmlRelaxNGParserCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGInitTypes() -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRelaxNGSetParserErrors(ctxt: xmlRelaxNGParserCtxtPtr,
                                 err: xmlRelaxNGValidityErrorFunc,
                                 warn: xmlRelaxNGValidityWarningFunc,
                                 ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGValidateDoc(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    #[no_mangle]
    fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr)
     -> xmlRelaxNGValidCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGSetValidErrors(ctxt: xmlRelaxNGValidCtxtPtr,
                                err: xmlRelaxNGValidityErrorFunc,
                                warn: xmlRelaxNGValidityWarningFunc,
                                ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
    #[no_mangle]
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    /* These macros may later turn into functions */
/* *
 * xmlXPathNodeSetGetLength:
 * @ns:  a node-set
 *
 * Implement a functionality similar to the DOM NodeList.length.
 *
 * Returns the number of nodes in the node-set.
 */
    /* *
 * xmlXPathNodeSetItem:
 * @ns:  a node-set
 * @index:  index of a node in the set
 *
 * Implements a functionality similar to the DOM NodeList.item().
 *
 * Returns the xmlNodePtr at the given @index in @ns or NULL if
 *         @index is out of range (0 to length-1)
 */
    /* *
 * xmlXPathNodeSetIsEmpty:
 * @ns: a node-set
 *
 * Checks whether @ns is empty or not.
 *
 * Returns %TRUE if @ns is an empty node-set.
 */
    #[no_mangle]
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    /* *
 * Context handling.
 */
    #[no_mangle]
    fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
    #[no_mangle]
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    #[no_mangle]
    fn xmlXPathContextSetCache(ctxt: xmlXPathContextPtr, active: std::os::raw::c_int,
                               value: std::os::raw::c_int, options: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /* *
 * Separate compilation/evaluation entry points.
 */
    #[no_mangle]
    fn xmlXPathCompile(str: *const xmlChar) -> xmlXPathCompExprPtr;
    #[no_mangle]
    fn xmlXPathCompiledEval(comp: xmlXPathCompExprPtr,
                            ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathFreeCompExpr(comp: xmlXPathCompExprPtr);
    /* *
 * Extending a context.
 */
    #[no_mangle]
    fn xmlXPathRegisterNs(ctxt: xmlXPathContextPtr, prefix: *const xmlChar,
                          ns_uri: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaInitTypes();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut std::os::raw::c_void,
    pub reg_save_area: *mut std::os::raw::c_void,
}
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type xmlNsPtr = *mut xmlNs;
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
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed = 1;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlRelaxNGValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
/*
* TODO: Actually all those flags used for the schema should sit
* on the schema parser context, since they are used only
* during parsing an XML schema document, and not available
* on the component level as per spec.
*/
/* *
 * XML_SCHEMAS_QUALIF_ELEM:
 *
 * Reflects elementFormDefault == qualified in
 * an XML schema document.
 */
/* *
 * XML_SCHEMAS_QUALIF_ATTR:
 *
 * Reflects attributeFormDefault == qualified in
 * an XML schema document.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_EXTENSION:
 *
 * the schema has "extension" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_RESTRICTION:
 *
 * the schema has "restriction" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_LIST:
 *
 * the cshema has "list" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_UNION:
 *
 * the schema has "union" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_BLOCK_DEFAULT_EXTENSION:
 *
 * the schema has "extension" in the set of blockDefault.
 */
/* *
 * XML_SCHEMAS_BLOCK_DEFAULT_RESTRICTION:
 *
 * the schema has "restriction" in the set of blockDefault.
 */
/* *
 * XML_SCHEMAS_BLOCK_DEFAULT_SUBSTITUTION:
 *
 * the schema has "substitution" in the set of blockDefault.
 */
/* *
 * XML_SCHEMAS_INCLUDING_CONVERT_NS:
 *
 * the schema is currently including an other schema with
 * no target namespace.
 */
/* *
 * _xmlSchema:
 *
 * A Schemas definition
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchema {
    pub name: *const xmlChar,
    pub targetNamespace: *const xmlChar,
    pub version: *const xmlChar,
    pub id: *const xmlChar,
    pub doc: xmlDocPtr,
    pub annot: xmlSchemaAnnotPtr,
    pub flags: std::os::raw::c_int,
    pub typeDecl: xmlHashTablePtr,
    pub attrDecl: xmlHashTablePtr,
    pub attrgrpDecl: xmlHashTablePtr,
    pub elemDecl: xmlHashTablePtr,
    pub notaDecl: xmlHashTablePtr,
    pub schemasImports: xmlHashTablePtr,
    pub _private: *mut std::os::raw::c_void,
    pub groupDecl: xmlHashTablePtr,
    pub dict: xmlDictPtr,
    pub includes: *mut std::os::raw::c_void,
    pub preserve: std::os::raw::c_int,
    pub counter: std::os::raw::c_int,
    pub idcDef: xmlHashTablePtr,
    pub volatiles: *mut std::os::raw::c_void,
}
pub type xmlSchemaAnnotPtr = *mut xmlSchemaAnnot;
/* *
 * Annotation
 */
pub type xmlSchemaAnnot = _xmlSchemaAnnot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAnnot {
    pub next: *mut _xmlSchemaAnnot,
    pub content: xmlNodePtr,
}
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlSchemaValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
/* *
 * xmlXPathFlags:
 * Flags for XPath engine compilation and runtime
 */
/* *
 * XML_XPATH_CHECKNS:
 *
 * check namespaces at compilation
 */
/* *
 * XML_XPATH_NOVAR:
 *
 * forbid variables in expression
 */
/* *
 * xmlXPathContext:
 *
 * Expression evaluation occurs with respect to a context.
 * he context consists of:
 *    - a node (the context node)
 *    - a node list (the context node list)
 *    - a set of variable bindings
 *    - a function library
 *    - the set of namespace declarations in scope for the expression
 * Following the switch to hash tables, this need to be trimmed up at
 * the next binary incompatible release.
 * The node may be modified when the context is passed to libxml2
 * for an XPath evaluation so you may need to initialize it again
 * before the next call.
 */
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
/* *
 * xmlXPathFuncLookupFunc:
 * @ctxt:  an XPath context
 * @name:  name of the function
 * @ns_uri:  the namespace name hosting this function
 *
 * Prototype for callbacks used to plug function lookup in the XPath
 * engine.
 *
 * Returns the XPath function or NULL if not found.
 */
pub type xmlXPathFuncLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathFunction>;
/* the search function */
/* *
 * xmlXPathFunction:
 * @ctxt:  the XPath interprestation context
 * @nargs:  the number of arguments
 *
 * An XPath function.
 * The arguments (if any) are popped out from the context stack
 * and the result is pushed on the stack.
 */
pub type xmlXPathFunction
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: std::os::raw::c_int)
               -> ()>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
/* *
 * xmlXPathParserContext:
 *
 * An XPath parser context. It contains pure parsing informations,
 * an xmlXPathContext, and the stack of objects.
 */
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
/*
 * The structure of a compiled expression form is not public.
 */
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
/*
 * A node-set (an unordered collection of nodes without duplicates).
 */
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
/*
 * Summary: XML Path Language implementation
 * Description: API for the XML Path Language implementation
 *
 * XML Path Language implementation
 * XPath is a language for addressing parts of an XML document,
 * designed to be used by both XSLT and XPointer
 *     http://www.w3.org/TR/xpath
 *
 * Implements
 * W3C Recommendation 16 November 1999
 *     http://www.w3.org/TR/1999/REC-xpath-19991116
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* LIBXML_XPATH_ENABLED */
/* LIBXML_XPATH_ENABLED or LIBXML_SCHEMAS_ENABLED */
pub type xmlXPathContext = _xmlXPathContext;
/*
 * Function and Variable Lookup.
 */
/* *
 * xmlXPathVariableLookupFunc:
 * @ctxt:  an XPath context
 * @name:  name of the variable
 * @ns_uri:  the namespace name hosting this variable
 *
 * Prototype for callbacks used to plug variable lookup in the XPath
 * engine.
 *
 * Returns the XPath object value or NULL if not found.
 */
pub type xmlXPathVariableLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathObjectPtr>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
/*
 * Extra axis: a name and an axis function.
 */
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
/* the evaluation function */
/* *
 * xmlXPathAxisFunc:
 * @ctxt:  the XPath interpreter context
 * @cur:  the previous node being explored on that axis
 *
 * An axis traversal function. To traverse an axis, the engine calls
 * the first time with cur == NULL and repeat until the function returns
 * NULL indicating the end of the axis traversal.
 *
 * Returns the next node in that axis or NULL if at the end of the axis.
 */
pub type xmlXPathAxisFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr,
                                _: xmlXPathObjectPtr) -> xmlXPathObjectPtr>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
/*
 * Extra type: a name and a conversion function.
 */
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
/* *
 * xmlXPathConvertFunc:
 * @obj:  an XPath object
 * @type:  the number of the target type
 *
 * A conversion function is associated to a type and used to cast
 * the new type to primitive values.
 *
 * Returns -1 in case of error, 0 otherwise
 */
pub type xmlXPathConvertFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathObjectPtr, _: std::os::raw::c_int)
               -> std::os::raw::c_int>;
#[inline]
unsafe extern "C" fn stat(mut __path: *const std::os::raw::c_char,
                          mut __statbuf: *mut stat) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut verbose: std::os::raw::c_int = 0 as std::os::raw::c_int;
/* ***********************************************************************
 *									*
 *		File name and path utilities				*
 *									*
 ************************************************************************/
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
unsafe extern "C" fn composeDir(mut dir: *const xmlChar,
                                mut path: *const xmlChar) -> *mut xmlChar {
    let mut buf: [std::os::raw::c_char; 500] = [0; 500];
    if dir.is_null() { return xmlStrdup(path) }
    if path.is_null() { return 0 as *mut xmlChar }
    snprintf(buf.as_mut_ptr(), 500 as std::os::raw::c_int as std::os::raw::c_ulong,
             b"%s/%s\x00" as *const u8 as *const std::os::raw::c_char,
             dir as *const std::os::raw::c_char, path as *const std::os::raw::c_char);
    return xmlStrdup(buf.as_mut_ptr() as *const xmlChar);
}
/* ***********************************************************************
 *									*
 *		Libxml2 specific routines				*
 *									*
 ************************************************************************/
static mut nb_tests: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_errors: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_internals: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_schematas: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_unimplemented: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut nb_leaks: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut extraMemoryFromResolver: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn fatalError() -> std::os::raw::c_int {
    fprintf(stderr,
            b"Exitting tests on fatal error\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    exit(1 as std::os::raw::c_int);
}
static mut testEntitiesName: [*mut std::os::raw::c_char; 20] =
    [0 as *const std::os::raw::c_char as *mut std::os::raw::c_char; 20];
static mut testEntitiesValue: [*mut std::os::raw::c_char; 20] =
    [0 as *const std::os::raw::c_char as *mut std::os::raw::c_char; 20];
static mut nb_entities: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn resetEntities() {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < nb_entities {
        if !testEntitiesName[i as usize].is_null() {
            xmlFree.expect("non-null function pointer")(testEntitiesName[i as
                                                                             usize]
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        if !testEntitiesValue[i as usize].is_null() {
            xmlFree.expect("non-null function pointer")(testEntitiesValue[i as
                                                                              usize]
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    }
    nb_entities = 0 as std::os::raw::c_int;
}
unsafe extern "C" fn addEntity(mut name: *mut std::os::raw::c_char,
                               mut content: *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    if nb_entities >= 20 as std::os::raw::c_int {
        fprintf(stderr,
                b"Too many entities defined\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    testEntitiesName[nb_entities as usize] = name;
    testEntitiesValue[nb_entities as usize] = content;
    nb_entities += 1;
    return 0 as std::os::raw::c_int;
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
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < nb_entities {
        if strcmp(testEntitiesName[i as usize], URL) == 0 {
            ret =
                xmlNewStringInputStream(ctxt,
                                        testEntitiesValue[i as usize] as
                                            *const xmlChar);
            if !ret.is_null() {
                (*ret).filename =
                    xmlStrdup(testEntitiesName[i as usize] as *mut xmlChar) as
                        *const std::os::raw::c_char
            }
            return ret
        }
        i += 1
    }
    if checkTestFile(URL) != 0 {
        ret = xmlNoNetExternalEntityLoader(URL, ID, ctxt)
    } else {
        let mut memused: std::os::raw::c_int = xmlMemUsed();
        ret = xmlNoNetExternalEntityLoader(URL, ID, ctxt);
        extraMemoryFromResolver += xmlMemUsed() - memused
    }
    return ret;
}
/*
 * Trapping the error messages at the generic level to grab the equivalent of
 * stderr messages on CLI tools.
 */
static mut testErrors: [std::os::raw::c_char; 32769] = [0; 32769];
static mut testErrorsSize: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe extern "C" fn test_log(mut msg: *const std::os::raw::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    if !logfile.is_null() {
        fprintf(logfile,
                b"\n------------\n\x00" as *const u8 as *const std::os::raw::c_char);
        args_0 = args.clone();
        vfprintf(logfile, msg, args_0.as_va_list());
        fprintf(logfile, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                testErrors.as_mut_ptr());
        testErrorsSize = 0 as std::os::raw::c_int;
        testErrors[0 as std::os::raw::c_int as usize] =
            0 as std::os::raw::c_int as std::os::raw::c_char
    }
    if verbose != 0 {
        args_0 = args.clone();
        vfprintf(stderr, msg, args_0.as_va_list());
    };
}
unsafe extern "C" fn testErrorHandler(mut ctx: *mut std::os::raw::c_void,
                                      mut msg: *const std::os::raw::c_char,
                                      mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut res: std::os::raw::c_int = 0;
    if testErrorsSize >= 32768 as std::os::raw::c_int { return }
    args_0 = args.clone();
    res =
        vsnprintf(&mut *testErrors.as_mut_ptr().offset(testErrorsSize as
                                                           isize),
                  (32768 as std::os::raw::c_int - testErrorsSize) as std::os::raw::c_ulong,
                  msg, args_0.as_va_list());
    if testErrorsSize + res >= 32768 as std::os::raw::c_int {
        /* buffer is full */
        testErrorsSize = 32768 as std::os::raw::c_int;
        testErrors[testErrorsSize as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char
    } else { testErrorsSize += res }
    testErrors[testErrorsSize as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
}
static mut ctxtXPath: xmlXPathContextPtr =
    0 as *const xmlXPathContext as *mut xmlXPathContext;
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
    ctxtXPath = xmlXPathNewContext(0 as xmlDocPtr);
    /*
    * Deactivate the cache if created; otherwise we have to create/free it
    * for every test, since it will confuse the memory leak detection.
    * Note that normally this need not be done, since the cache is not
    * created until set explicitely with xmlXPathContextSetCache();
    * but for test purposes it is sometimes usefull to activate the
    * cache by default for the whole library.
    */
    if !(*ctxtXPath).cache.is_null() {
        xmlXPathContextSetCache(ctxtXPath, 0 as std::os::raw::c_int,
                                -(1 as std::os::raw::c_int), 0 as std::os::raw::c_int);
    }
    /* used as default nanemspace in xstc tests */
    xmlXPathRegisterNs(ctxtXPath,
                       b"ts\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar,
                       b"TestSuite\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
    xmlXPathRegisterNs(ctxtXPath,
                       b"xlink\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar,
                       b"http://www.w3.org/1999/xlink\x00" as *const u8 as
                           *const std::os::raw::c_char as *mut xmlChar);
    xmlSetGenericErrorFunc(0 as *mut std::os::raw::c_void,
                           Some(testErrorHandler as
                                    unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                         _:
                                                             *const std::os::raw::c_char,
                                                         _: ...) -> ()));
    xmlSchemaInitTypes();
    xmlRelaxNGInitTypes();
}
unsafe extern "C" fn getNext(mut cur: xmlNodePtr,
                             mut xpath: *const std::os::raw::c_char) -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as xmlNodePtr;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    if cur.is_null() || (*cur).doc.is_null() || xpath.is_null() {
        return 0 as xmlNodePtr
    }
    (*ctxtXPath).doc = (*cur).doc;
    (*ctxtXPath).node = cur;
    comp = xmlXPathCompile(xpath as *mut xmlChar);
    if comp.is_null() {
        fprintf(stderr,
                b"Failed to compile %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, xpath);
        return 0 as xmlNodePtr
    }
    res = xmlXPathCompiledEval(comp, ctxtXPath);
    xmlXPathFreeCompExpr(comp);
    if res.is_null() { return 0 as xmlNodePtr }
    if (*res).type_0 as std::os::raw::c_uint ==
           XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*res).nodesetval.is_null() &&
           (*(*res).nodesetval).nodeNr > 0 as std::os::raw::c_int &&
           !(*(*res).nodesetval).nodeTab.is_null() {
        ret = *(*(*res).nodesetval).nodeTab.offset(0 as std::os::raw::c_int as isize)
    }
    xmlXPathFreeObject(res);
    return ret;
}
unsafe extern "C" fn getString(mut cur: xmlNodePtr,
                               mut xpath: *const std::os::raw::c_char)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    if cur.is_null() || (*cur).doc.is_null() || xpath.is_null() {
        return 0 as *mut xmlChar
    }
    (*ctxtXPath).doc = (*cur).doc;
    (*ctxtXPath).node = cur;
    comp = xmlXPathCompile(xpath as *mut xmlChar);
    if comp.is_null() {
        fprintf(stderr,
                b"Failed to compile %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, xpath);
        return 0 as *mut xmlChar
    }
    res = xmlXPathCompiledEval(comp, ctxtXPath);
    xmlXPathFreeCompExpr(comp);
    if res.is_null() { return 0 as *mut xmlChar }
    if (*res).type_0 as std::os::raw::c_uint ==
           XPATH_STRING as std::os::raw::c_int as std::os::raw::c_uint {
        ret = (*res).stringval;
        (*res).stringval = 0 as *mut xmlChar
    }
    xmlXPathFreeObject(res);
    return ret;
}
/* ***********************************************************************
 *									*
 *		Test test/xsdtest/xsdtestsuite.xml			*
 *									*
 ************************************************************************/
unsafe extern "C" fn xsdIncorectTestCase(mut cur: xmlNodePtr) -> std::os::raw::c_int {
    let mut test: xmlNodePtr = 0 as *mut xmlNode;
    let mut buf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut pctxt: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    let mut rng: xmlRelaxNGPtr = 0 as xmlRelaxNGPtr;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut memt: std::os::raw::c_int = 0;
    cur =
        getNext(cur,
                b"./incorrect[1]\x00" as *const u8 as *const std::os::raw::c_char);
    if cur.is_null() { return 0 as std::os::raw::c_int }
    test = getNext(cur, b"./*\x00" as *const u8 as *const std::os::raw::c_char);
    if test.is_null() {
        test_log(b"Failed to find test in correct line %ld\n\x00" as *const u8
                     as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode));
        return 1 as std::os::raw::c_int
    }
    memt = xmlMemUsed();
    extraMemoryFromResolver = 0 as std::os::raw::c_int;
    /*
     * dump the schemas to a buffer, then reparse it and compile the schemas
     */
    buf = xmlBufferCreate();
    if buf.is_null() {
        fprintf(stderr,
                b"out of memory !\n\x00" as *const u8 as *const std::os::raw::c_char);
        fatalError();
    }
    xmlNodeDump(buf, (*test).doc, test, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    pctxt =
        xmlRelaxNGNewMemParserCtxt((*buf).content as *const std::os::raw::c_char,
                                   (*buf).use_0 as std::os::raw::c_int);
    xmlRelaxNGSetParserErrors(pctxt,
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut std::os::raw::c_void,
                                                                                  _:
                                                                                      *const std::os::raw::c_char,
                                                                                  _:
                                                                                      ...)
                                                                 -> ()>,
                                                      xmlRelaxNGValidityErrorFunc>(Some(testErrorHandler
                                                                                            as
                                                                                            unsafe extern "C" fn(_:
                                                                                                                     *mut std::os::raw::c_void,
                                                                                                                 _:
                                                                                                                     *const std::os::raw::c_char,
                                                                                                                 _:
                                                                                                                     ...)
                                                                                                ->
                                                                                                    ())),
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut std::os::raw::c_void,
                                                                                  _:
                                                                                      *const std::os::raw::c_char,
                                                                                  _:
                                                                                      ...)
                                                                 -> ()>,
                                                      xmlRelaxNGValidityWarningFunc>(Some(testErrorHandler
                                                                                              as
                                                                                              unsafe extern "C" fn(_:
                                                                                                                       *mut std::os::raw::c_void,
                                                                                                                   _:
                                                                                                                       *const std::os::raw::c_char,
                                                                                                                   _:
                                                                                                                       ...)
                                                                                                  ->
                                                                                                      ())),
                              pctxt as *mut std::os::raw::c_void);
    rng = xmlRelaxNGParse(pctxt);
    xmlRelaxNGFreeParserCtxt(pctxt);
    if !rng.is_null() {
        test_log(b"Failed to detect incorect RNG line %ld\n\x00" as *const u8
                     as *const std::os::raw::c_char,
                 xmlGetLineNo(test as *const xmlNode));
        ret = 1 as std::os::raw::c_int
    }
    if !buf.is_null() { xmlBufferFree(buf); }
    if !rng.is_null() { xmlRelaxNGFree(rng); }
    xmlResetLastError();
    if memt < xmlMemUsed() && extraMemoryFromResolver == 0 as std::os::raw::c_int {
        test_log(b"Validation of tests starting line %ld leaked %d\n\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode), xmlMemUsed() - memt);
        nb_leaks += 1
    }
    return ret;
}
unsafe extern "C" fn installResources(mut tst: xmlNodePtr,
                                      mut base: *const xmlChar) {
    let mut test: xmlNodePtr = 0 as *mut xmlNode;
    let mut buf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut content: *mut xmlChar = 0 as *mut xmlChar;
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    buf = xmlBufferCreate();
    if buf.is_null() {
        fprintf(stderr,
                b"out of memory !\n\x00" as *const u8 as *const std::os::raw::c_char);
        fatalError();
    }
    xmlNodeDump(buf, (*tst).doc, tst, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    while !tst.is_null() {
        test = getNext(tst, b"./*\x00" as *const u8 as *const std::os::raw::c_char);
        if !test.is_null() {
            xmlBufferEmpty(buf);
            xmlNodeDump(buf, (*test).doc, test, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int);
            name =
                getString(tst,
                          b"string(@name)\x00" as *const u8 as
                              *const std::os::raw::c_char);
            content = xmlStrdup((*buf).content);
            if !name.is_null() && !content.is_null() {
                res = composeDir(base, name);
                xmlFree.expect("non-null function pointer")(name as
                                                                *mut std::os::raw::c_void);
                addEntity(res as *mut std::os::raw::c_char,
                          content as *mut std::os::raw::c_char);
            } else {
                if !name.is_null() {
                    xmlFree.expect("non-null function pointer")(name as
                                                                    *mut std::os::raw::c_void);
                }
                if !content.is_null() {
                    xmlFree.expect("non-null function pointer")(content as
                                                                    *mut std::os::raw::c_void);
                }
            }
        }
        tst =
            getNext(tst,
                    b"following-sibling::resource[1]\x00" as *const u8 as
                        *const std::os::raw::c_char)
    }
    if !buf.is_null() { xmlBufferFree(buf); };
}
unsafe extern "C" fn installDirs(mut tst: xmlNodePtr,
                                 mut base: *const xmlChar) {
    let mut test: xmlNodePtr = 0 as *mut xmlNode;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    name =
        getString(tst,
                  b"string(@name)\x00" as *const u8 as *const std::os::raw::c_char);
    if name.is_null() { return }
    res = composeDir(base, name);
    xmlFree.expect("non-null function pointer")(name as *mut std::os::raw::c_void);
    if res.is_null() { return }
    /* Now process resources and subdir recursively */
    test =
        getNext(tst,
                b"./resource[1]\x00" as *const u8 as *const std::os::raw::c_char);
    if !test.is_null() { installResources(test, res); }
    test = getNext(tst, b"./dir[1]\x00" as *const u8 as *const std::os::raw::c_char);
    while !test.is_null() {
        installDirs(test, res);
        test =
            getNext(test,
                    b"following-sibling::dir[1]\x00" as *const u8 as
                        *const std::os::raw::c_char)
    }
    xmlFree.expect("non-null function pointer")(res as *mut std::os::raw::c_void);
}
unsafe extern "C" fn xsdTestCase(mut tst: xmlNodePtr) -> std::os::raw::c_int {
    let mut test: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut buf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    let mut pctxt: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    let mut ctxt: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
    let mut rng: xmlRelaxNGPtr = 0 as xmlRelaxNGPtr;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut mem: std::os::raw::c_int = 0;
    let mut memt: std::os::raw::c_int = 0;
    let mut dtd: *mut xmlChar = 0 as *mut xmlChar;
    resetEntities();
    testErrorsSize = 0 as std::os::raw::c_int;
    testErrors[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    tmp = getNext(tst, b"./dir[1]\x00" as *const u8 as *const std::os::raw::c_char);
    if !tmp.is_null() { installDirs(tmp, 0 as *const xmlChar); }
    tmp =
        getNext(tst,
                b"./resource[1]\x00" as *const u8 as *const std::os::raw::c_char);
    if !tmp.is_null() { installResources(tmp, 0 as *const xmlChar); }
    cur =
        getNext(tst, b"./correct[1]\x00" as *const u8 as *const std::os::raw::c_char);
    if cur.is_null() { return xsdIncorectTestCase(tst) }
    test = getNext(cur, b"./*\x00" as *const u8 as *const std::os::raw::c_char);
    if test.is_null() {
        fprintf(stderr,
                b"Failed to find test in correct line %ld\n\x00" as *const u8
                    as *const std::os::raw::c_char,
                xmlGetLineNo(cur as *const xmlNode));
        return 1 as std::os::raw::c_int
    }
    memt = xmlMemUsed();
    extraMemoryFromResolver = 0 as std::os::raw::c_int;
    /*
     * dump the schemas to a buffer, then reparse it and compile the schemas
     */
    buf = xmlBufferCreate();
    if buf.is_null() {
        fprintf(stderr,
                b"out of memory !\n\x00" as *const u8 as *const std::os::raw::c_char);
        fatalError();
    }
    xmlNodeDump(buf, (*test).doc, test, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    pctxt =
        xmlRelaxNGNewMemParserCtxt((*buf).content as *const std::os::raw::c_char,
                                   (*buf).use_0 as std::os::raw::c_int);
    xmlRelaxNGSetParserErrors(pctxt,
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut std::os::raw::c_void,
                                                                                  _:
                                                                                      *const std::os::raw::c_char,
                                                                                  _:
                                                                                      ...)
                                                                 -> ()>,
                                                      xmlRelaxNGValidityErrorFunc>(Some(testErrorHandler
                                                                                            as
                                                                                            unsafe extern "C" fn(_:
                                                                                                                     *mut std::os::raw::c_void,
                                                                                                                 _:
                                                                                                                     *const std::os::raw::c_char,
                                                                                                                 _:
                                                                                                                     ...)
                                                                                                ->
                                                                                                    ())),
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut std::os::raw::c_void,
                                                                                  _:
                                                                                      *const std::os::raw::c_char,
                                                                                  _:
                                                                                      ...)
                                                                 -> ()>,
                                                      xmlRelaxNGValidityWarningFunc>(Some(testErrorHandler
                                                                                              as
                                                                                              unsafe extern "C" fn(_:
                                                                                                                       *mut std::os::raw::c_void,
                                                                                                                   _:
                                                                                                                       *const std::os::raw::c_char,
                                                                                                                   _:
                                                                                                                       ...)
                                                                                                  ->
                                                                                                      ())),
                              pctxt as *mut std::os::raw::c_void);
    rng = xmlRelaxNGParse(pctxt);
    xmlRelaxNGFreeParserCtxt(pctxt);
    if extraMemoryFromResolver != 0 { memt = 0 as std::os::raw::c_int }
    if rng.is_null() {
        test_log(b"Failed to parse RNGtest line %ld\n\x00" as *const u8 as
                     *const std::os::raw::c_char,
                 xmlGetLineNo(test as *const xmlNode));
        nb_errors += 1;
        ret = 1 as std::os::raw::c_int
    } else {
        /*
     * now scan all the siblings of correct to process the <valid> tests
     */
        tmp =
            getNext(cur,
                    b"following-sibling::valid[1]\x00" as *const u8 as
                        *const std::os::raw::c_char);
        while !tmp.is_null() {
            dtd =
                xmlGetProp(tmp as *const xmlNode,
                           b"dtd\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar);
            test =
                getNext(tmp, b"./*\x00" as *const u8 as *const std::os::raw::c_char);
            if test.is_null() {
                fprintf(stderr,
                        b"Failed to find test in <valid> line %ld\n\x00" as
                            *const u8 as *const std::os::raw::c_char,
                        xmlGetLineNo(tmp as *const xmlNode));
            } else {
                xmlBufferEmpty(buf);
                if !dtd.is_null() {
                    xmlBufferAdd(buf, dtd, -(1 as std::os::raw::c_int));
                }
                xmlNodeDump(buf, (*test).doc, test, 0 as std::os::raw::c_int,
                            0 as std::os::raw::c_int);
                /*
	     * We are ready to run the test
	     */
                mem = xmlMemUsed();
                extraMemoryFromResolver = 0 as std::os::raw::c_int;
                doc =
                    xmlReadMemory((*buf).content as *const std::os::raw::c_char,
                                  (*buf).use_0 as std::os::raw::c_int,
                                  b"test\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
                if doc.is_null() {
                    test_log(b"Failed to parse valid instance line %ld\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             xmlGetLineNo(tmp as *const xmlNode));
                    nb_errors += 1
                } else {
                    nb_tests += 1;
                    ctxt = xmlRelaxNGNewValidCtxt(rng);
                    xmlRelaxNGSetValidErrors(ctxt,
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlRelaxNGValidityErrorFunc>(Some(testErrorHandler
                                                                                                           as
                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                    *mut std::os::raw::c_void,
                                                                                                                                _:
                                                                                                                                    *const std::os::raw::c_char,
                                                                                                                                _:
                                                                                                                                    ...)
                                                                                                               ->
                                                                                                                   ())),
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlRelaxNGValidityWarningFunc>(Some(testErrorHandler
                                                                                                             as
                                                                                                             unsafe extern "C" fn(_:
                                                                                                                                      *mut std::os::raw::c_void,
                                                                                                                                  _:
                                                                                                                                      *const std::os::raw::c_char,
                                                                                                                                  _:
                                                                                                                                      ...)
                                                                                                                 ->
                                                                                                                     ())),
                                             ctxt as *mut std::os::raw::c_void);
                    ret = xmlRelaxNGValidateDoc(ctxt, doc);
                    xmlRelaxNGFreeValidCtxt(ctxt);
                    if ret > 0 as std::os::raw::c_int {
                        test_log(b"Failed to validate valid instance line %ld\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 xmlGetLineNo(tmp as *const xmlNode));
                        nb_errors += 1
                    } else if ret < 0 as std::os::raw::c_int {
                        test_log(b"Internal error validating instance line %ld\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 xmlGetLineNo(tmp as *const xmlNode));
                        nb_errors += 1
                    }
                    xmlFreeDoc(doc);
                }
                xmlResetLastError();
                if mem != xmlMemUsed() &&
                       extraMemoryFromResolver == 0 as std::os::raw::c_int {
                    test_log(b"Validation of instance line %ld leaked %d\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             xmlGetLineNo(tmp as *const xmlNode),
                             xmlMemUsed() - mem);
                    xmlMemoryDump();
                    nb_leaks += 1
                }
            }
            if !dtd.is_null() {
                xmlFree.expect("non-null function pointer")(dtd as
                                                                *mut std::os::raw::c_void);
            }
            tmp =
                getNext(tmp,
                        b"following-sibling::valid[1]\x00" as *const u8 as
                            *const std::os::raw::c_char)
        }
        /*
     * now scan all the siblings of correct to process the <invalid> tests
     */
        tmp =
            getNext(cur,
                    b"following-sibling::invalid[1]\x00" as *const u8 as
                        *const std::os::raw::c_char);
        while !tmp.is_null() {
            test =
                getNext(tmp, b"./*\x00" as *const u8 as *const std::os::raw::c_char);
            if test.is_null() {
                fprintf(stderr,
                        b"Failed to find test in <invalid> line %ld\n\x00" as
                            *const u8 as *const std::os::raw::c_char,
                        xmlGetLineNo(tmp as *const xmlNode));
            } else {
                xmlBufferEmpty(buf);
                xmlNodeDump(buf, (*test).doc, test, 0 as std::os::raw::c_int,
                            0 as std::os::raw::c_int);
                /*
	     * We are ready to run the test
	     */
                mem = xmlMemUsed();
                extraMemoryFromResolver = 0 as std::os::raw::c_int;
                doc =
                    xmlReadMemory((*buf).content as *const std::os::raw::c_char,
                                  (*buf).use_0 as std::os::raw::c_int,
                                  b"test\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
                if doc.is_null() {
                    test_log(b"Failed to parse valid instance line %ld\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             xmlGetLineNo(tmp as *const xmlNode));
                    nb_errors += 1
                } else {
                    nb_tests += 1;
                    ctxt = xmlRelaxNGNewValidCtxt(rng);
                    xmlRelaxNGSetValidErrors(ctxt,
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlRelaxNGValidityErrorFunc>(Some(testErrorHandler
                                                                                                           as
                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                    *mut std::os::raw::c_void,
                                                                                                                                _:
                                                                                                                                    *const std::os::raw::c_char,
                                                                                                                                _:
                                                                                                                                    ...)
                                                                                                               ->
                                                                                                                   ())),
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlRelaxNGValidityWarningFunc>(Some(testErrorHandler
                                                                                                             as
                                                                                                             unsafe extern "C" fn(_:
                                                                                                                                      *mut std::os::raw::c_void,
                                                                                                                                  _:
                                                                                                                                      *const std::os::raw::c_char,
                                                                                                                                  _:
                                                                                                                                      ...)
                                                                                                                 ->
                                                                                                                     ())),
                                             ctxt as *mut std::os::raw::c_void);
                    ret = xmlRelaxNGValidateDoc(ctxt, doc);
                    xmlRelaxNGFreeValidCtxt(ctxt);
                    if ret == 0 as std::os::raw::c_int {
                        test_log(b"Failed to detect invalid instance line %ld\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 xmlGetLineNo(tmp as *const xmlNode));
                        nb_errors += 1
                    } else if ret < 0 as std::os::raw::c_int {
                        test_log(b"Internal error validating instance line %ld\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 xmlGetLineNo(tmp as *const xmlNode));
                        nb_errors += 1
                    }
                    xmlFreeDoc(doc);
                }
                xmlResetLastError();
                if mem != xmlMemUsed() &&
                       extraMemoryFromResolver == 0 as std::os::raw::c_int {
                    test_log(b"Validation of instance line %ld leaked %d\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             xmlGetLineNo(tmp as *const xmlNode),
                             xmlMemUsed() - mem);
                    xmlMemoryDump();
                    nb_leaks += 1
                }
            }
            tmp =
                getNext(tmp,
                        b"following-sibling::invalid[1]\x00" as *const u8 as
                            *const std::os::raw::c_char)
        }
    }
    if !buf.is_null() { xmlBufferFree(buf); }
    if !rng.is_null() { xmlRelaxNGFree(rng); }
    xmlResetLastError();
    if memt != xmlMemUsed() && memt != 0 as std::os::raw::c_int {
        test_log(b"Validation of tests starting line %ld leaked %d\n\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode), xmlMemUsed() - memt);
        nb_leaks += 1
    }
    return ret;
}
unsafe extern "C" fn xsdTestSuite(mut cur: xmlNodePtr) -> std::os::raw::c_int {
    if verbose != 0 {
        let mut doc: *mut xmlChar =
            getString(cur,
                      b"string(documentation)\x00" as *const u8 as
                          *const std::os::raw::c_char);
        if !doc.is_null() {
            printf(b"Suite %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                   doc);
            xmlFree.expect("non-null function pointer")(doc as
                                                            *mut std::os::raw::c_void);
        }
    }
    cur =
        getNext(cur,
                b"./testCase[1]\x00" as *const u8 as *const std::os::raw::c_char);
    while !cur.is_null() {
        xsdTestCase(cur);
        cur =
            getNext(cur,
                    b"following-sibling::testCase[1]\x00" as *const u8 as
                        *const std::os::raw::c_char)
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn xsdTest() -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut filename: *const std::os::raw::c_char =
        b"test/xsdtest/xsdtestsuite.xml\x00" as *const u8 as
            *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    doc =
        xmlReadFile(filename, 0 as *const std::os::raw::c_char,
                    XML_PARSE_NOENT as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Failed to parse %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        return -(1 as std::os::raw::c_int)
    }
    printf(b"## XML Schemas datatypes test suite from James Clark\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if cur.is_null() ||
           xmlStrEqual((*cur).name,
                       b"testSuite\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
        fprintf(stderr,
                b"Unexpected format %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        ret = -(1 as std::os::raw::c_int)
    } else {
        cur =
            getNext(cur,
                    b"./testSuite[1]\x00" as *const u8 as
                        *const std::os::raw::c_char);
        if cur.is_null() ||
               xmlStrEqual((*cur).name,
                           b"testSuite\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut xmlChar) == 0 {
            fprintf(stderr,
                    b"Unexpected format %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            ret = -(1 as std::os::raw::c_int)
        } else {
            while !cur.is_null() {
                xsdTestSuite(cur);
                cur =
                    getNext(cur,
                            b"following-sibling::testSuite[1]\x00" as
                                *const u8 as *const std::os::raw::c_char)
            }
        }
    }
    if !doc.is_null() { xmlFreeDoc(doc); }
    return ret;
}
unsafe extern "C" fn rngTestSuite(mut cur: xmlNodePtr) -> std::os::raw::c_int {
    if verbose != 0 {
        let mut doc: *mut xmlChar =
            getString(cur,
                      b"string(documentation)\x00" as *const u8 as
                          *const std::os::raw::c_char);
        if !doc.is_null() {
            printf(b"Suite %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                   doc);
            xmlFree.expect("non-null function pointer")(doc as
                                                            *mut std::os::raw::c_void);
        } else {
            doc =
                getString(cur,
                          b"string(section)\x00" as *const u8 as
                              *const std::os::raw::c_char);
            if !doc.is_null() {
                printf(b"Section %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, doc);
                xmlFree.expect("non-null function pointer")(doc as
                                                                *mut std::os::raw::c_void);
            }
        }
    }
    cur =
        getNext(cur,
                b"./testSuite[1]\x00" as *const u8 as *const std::os::raw::c_char);
    while !cur.is_null() {
        xsdTestSuite(cur);
        cur =
            getNext(cur,
                    b"following-sibling::testSuite[1]\x00" as *const u8 as
                        *const std::os::raw::c_char)
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn rngTest1() -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut filename: *const std::os::raw::c_char =
        b"test/relaxng/OASIS/spectest.xml\x00" as *const u8 as
            *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    doc =
        xmlReadFile(filename, 0 as *const std::os::raw::c_char,
                    XML_PARSE_NOENT as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Failed to parse %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        return -(1 as std::os::raw::c_int)
    }
    printf(b"## Relax NG test suite from James Clark\n\x00" as *const u8 as
               *const std::os::raw::c_char);
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if cur.is_null() ||
           xmlStrEqual((*cur).name,
                       b"testSuite\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
        fprintf(stderr,
                b"Unexpected format %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        ret = -(1 as std::os::raw::c_int)
    } else {
        cur =
            getNext(cur,
                    b"./testSuite[1]\x00" as *const u8 as
                        *const std::os::raw::c_char);
        if cur.is_null() ||
               xmlStrEqual((*cur).name,
                           b"testSuite\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut xmlChar) == 0 {
            fprintf(stderr,
                    b"Unexpected format %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            ret = -(1 as std::os::raw::c_int)
        } else {
            while !cur.is_null() {
                rngTestSuite(cur);
                cur =
                    getNext(cur,
                            b"following-sibling::testSuite[1]\x00" as
                                *const u8 as *const std::os::raw::c_char)
            }
        }
    }
    if !doc.is_null() { xmlFreeDoc(doc); }
    return ret;
}
unsafe extern "C" fn rngTest2() -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut filename: *const std::os::raw::c_char =
        b"test/relaxng/testsuite.xml\x00" as *const u8 as *const std::os::raw::c_char;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    doc =
        xmlReadFile(filename, 0 as *const std::os::raw::c_char,
                    XML_PARSE_NOENT as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Failed to parse %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        return -(1 as std::os::raw::c_int)
    }
    printf(b"## Relax NG test suite for libxml2\n\x00" as *const u8 as
               *const std::os::raw::c_char);
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if cur.is_null() ||
           xmlStrEqual((*cur).name,
                       b"testSuite\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
        fprintf(stderr,
                b"Unexpected format %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        ret = -(1 as std::os::raw::c_int)
    } else {
        cur =
            getNext(cur,
                    b"./testSuite[1]\x00" as *const u8 as
                        *const std::os::raw::c_char);
        if cur.is_null() ||
               xmlStrEqual((*cur).name,
                           b"testSuite\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut xmlChar) == 0 {
            fprintf(stderr,
                    b"Unexpected format %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, filename);
            ret = -(1 as std::os::raw::c_int)
        } else {
            while !cur.is_null() {
                xsdTestSuite(cur);
                cur =
                    getNext(cur,
                            b"following-sibling::testSuite[1]\x00" as
                                *const u8 as *const std::os::raw::c_char)
            }
        }
    }
    if !doc.is_null() { xmlFreeDoc(doc); }
    return ret;
}
/* ***********************************************************************
 *									*
 *		Schemas test suites from W3C/NIST/MS/Sun		*
 *									*
 ************************************************************************/
unsafe extern "C" fn xstcTestInstance(mut cur: xmlNodePtr,
                                      mut schemas: xmlSchemaPtr,
                                      mut spath: *const xmlChar,
                                      mut base: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut path: *mut xmlChar = 0 as *mut xmlChar;
    let mut validity: *mut xmlChar = 0 as *mut xmlChar;
    let mut ctxt: xmlSchemaValidCtxtPtr = 0 as xmlSchemaValidCtxtPtr;
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut mem: std::os::raw::c_int = 0;
    xmlResetLastError();
    testErrorsSize = 0 as std::os::raw::c_int;
    testErrors[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    mem = xmlMemUsed();
    href =
        getString(cur,
                  b"string(ts:instanceDocument/@xlink:href)\x00" as *const u8
                      as *const std::os::raw::c_char);
    if href.is_null() ||
           *href.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        test_log(b"testGroup line %ld misses href for schemaDocument\n\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode));
        ret = -(1 as std::os::raw::c_int)
    } else {
        path = xmlBuildURI(href, base as *mut xmlChar);
        if path.is_null() {
            fprintf(stderr,
                    b"Failed to build path to schemas testGroup line %ld : %s\n\x00"
                        as *const u8 as *const std::os::raw::c_char,
                    xmlGetLineNo(cur as *const xmlNode), href);
            ret = -(1 as std::os::raw::c_int)
        } else if checkTestFile(path as *const std::os::raw::c_char) <=
                      0 as std::os::raw::c_int {
            test_log(b"schemas for testGroup line %ld is missing: %s\n\x00" as
                         *const u8 as *const std::os::raw::c_char,
                     xmlGetLineNo(cur as *const xmlNode), path);
            ret = -(1 as std::os::raw::c_int)
        } else {
            validity =
                getString(cur,
                          b"string(ts:expected/@validity)\x00" as *const u8 as
                              *const std::os::raw::c_char);
            if validity.is_null() {
                fprintf(stderr,
                        b"instanceDocument line %ld misses expected validity\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        xmlGetLineNo(cur as *const xmlNode));
                ret = -(1 as std::os::raw::c_int)
            } else {
                nb_tests += 1;
                doc =
                    xmlReadFile(path as *const std::os::raw::c_char,
                                0 as *const std::os::raw::c_char,
                                XML_PARSE_NOENT as std::os::raw::c_int);
                if doc.is_null() {
                    fprintf(stderr,
                            b"instance %s fails to parse\n\x00" as *const u8
                                as *const std::os::raw::c_char, path);
                    ret = -(1 as std::os::raw::c_int);
                    nb_errors += 1
                } else {
                    ctxt = xmlSchemaNewValidCtxt(schemas);
                    xmlSchemaSetValidErrors(ctxt,
                                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                    *mut std::os::raw::c_void,
                                                                                                _:
                                                                                                    *const std::os::raw::c_char,
                                                                                                _:
                                                                                                    ...)
                                                                               ->
                                                                                   ()>,
                                                                    xmlSchemaValidityErrorFunc>(Some(testErrorHandler
                                                                                                         as
                                                                                                         unsafe extern "C" fn(_:
                                                                                                                                  *mut std::os::raw::c_void,
                                                                                                                              _:
                                                                                                                                  *const std::os::raw::c_char,
                                                                                                                              _:
                                                                                                                                  ...)
                                                                                                             ->
                                                                                                                 ())),
                                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                    *mut std::os::raw::c_void,
                                                                                                _:
                                                                                                    *const std::os::raw::c_char,
                                                                                                _:
                                                                                                    ...)
                                                                               ->
                                                                                   ()>,
                                                                    xmlSchemaValidityWarningFunc>(Some(testErrorHandler
                                                                                                           as
                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                    *mut std::os::raw::c_void,
                                                                                                                                _:
                                                                                                                                    *const std::os::raw::c_char,
                                                                                                                                _:
                                                                                                                                    ...)
                                                                                                               ->
                                                                                                                   ())),
                                            ctxt as *mut std::os::raw::c_void);
                    ret = xmlSchemaValidateDoc(ctxt, doc);
                    if xmlStrEqual(validity,
                                   b"valid\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) !=
                           0 {
                        if ret > 0 as std::os::raw::c_int {
                            test_log(b"valid instance %s failed to validate against %s\n\x00"
                                         as *const u8 as *const std::os::raw::c_char,
                                     path, spath);
                            nb_errors += 1
                        } else if ret < 0 as std::os::raw::c_int {
                            test_log(b"valid instance %s got internal error validating %s\n\x00"
                                         as *const u8 as *const std::os::raw::c_char,
                                     path, spath);
                            nb_internals += 1;
                            nb_errors += 1
                        }
                    } else if xmlStrEqual(validity,
                                          b"invalid\x00" as *const u8 as
                                              *const std::os::raw::c_char as
                                              *mut xmlChar) != 0 {
                        if ret == 0 as std::os::raw::c_int {
                            test_log(b"Failed to detect invalid instance %s against %s\n\x00"
                                         as *const u8 as *const std::os::raw::c_char,
                                     path, spath);
                            nb_errors += 1
                        }
                    } else {
                        test_log(b"instanceDocument line %ld has unexpected validity value%s\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 xmlGetLineNo(cur as *const xmlNode),
                                 validity);
                        ret = -(1 as std::os::raw::c_int)
                    }
                }
            }
        }
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as
                                                        *mut std::os::raw::c_void);
    }
    if !path.is_null() {
        xmlFree.expect("non-null function pointer")(path as
                                                        *mut std::os::raw::c_void);
    }
    if !validity.is_null() {
        xmlFree.expect("non-null function pointer")(validity as
                                                        *mut std::os::raw::c_void);
    }
    if !ctxt.is_null() { xmlSchemaFreeValidCtxt(ctxt); }
    if !doc.is_null() { xmlFreeDoc(doc); }
    xmlResetLastError();
    if mem != xmlMemUsed() {
        test_log(b"Validation of tests starting line %ld leaked %d\n\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode), xmlMemUsed() - mem);
        nb_leaks += 1
    }
    return ret;
}
unsafe extern "C" fn xstcTestGroup(mut cur: xmlNodePtr,
                                   mut base: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut path: *mut xmlChar = 0 as *mut xmlChar;
    let mut validity: *mut xmlChar = 0 as *mut xmlChar;
    let mut schemas: xmlSchemaPtr = 0 as xmlSchemaPtr;
    let mut ctxt: xmlSchemaParserCtxtPtr = 0 as *mut xmlSchemaParserCtxt;
    let mut instance: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut mem: std::os::raw::c_int = 0;
    xmlResetLastError();
    testErrorsSize = 0 as std::os::raw::c_int;
    testErrors[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    mem = xmlMemUsed();
    href =
        getString(cur,
                  b"string(ts:schemaTest/ts:schemaDocument/@xlink:href)\x00"
                      as *const u8 as *const std::os::raw::c_char);
    if href.is_null() ||
           *href.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        test_log(b"testGroup line %ld misses href for schemaDocument\n\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode));
        ret = -(1 as std::os::raw::c_int)
    } else {
        path = xmlBuildURI(href, base as *mut xmlChar);
        if path.is_null() {
            test_log(b"Failed to build path to schemas testGroup line %ld : %s\n\x00"
                         as *const u8 as *const std::os::raw::c_char,
                     xmlGetLineNo(cur as *const xmlNode), href);
            ret = -(1 as std::os::raw::c_int)
        } else if checkTestFile(path as *const std::os::raw::c_char) <=
                      0 as std::os::raw::c_int {
            test_log(b"schemas for testGroup line %ld is missing: %s\n\x00" as
                         *const u8 as *const std::os::raw::c_char,
                     xmlGetLineNo(cur as *const xmlNode), path);
            ret = -(1 as std::os::raw::c_int)
        } else {
            validity =
                getString(cur,
                          b"string(ts:schemaTest/ts:expected/@validity)\x00"
                              as *const u8 as *const std::os::raw::c_char);
            if validity.is_null() {
                test_log(b"testGroup line %ld misses expected validity\n\x00"
                             as *const u8 as *const std::os::raw::c_char,
                         xmlGetLineNo(cur as *const xmlNode));
                ret = -(1 as std::os::raw::c_int)
            } else {
                nb_tests += 1;
                if xmlStrEqual(validity,
                               b"valid\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
                    nb_schematas += 1;
                    ctxt =
                        xmlSchemaNewParserCtxt(path as *const std::os::raw::c_char);
                    xmlSchemaSetParserErrors(ctxt,
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlSchemaValidityErrorFunc>(Some(testErrorHandler
                                                                                                          as
                                                                                                          unsafe extern "C" fn(_:
                                                                                                                                   *mut std::os::raw::c_void,
                                                                                                                               _:
                                                                                                                                   *const std::os::raw::c_char,
                                                                                                                               _:
                                                                                                                                   ...)
                                                                                                              ->
                                                                                                                  ())),
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlSchemaValidityWarningFunc>(Some(testErrorHandler
                                                                                                            as
                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                     *mut std::os::raw::c_void,
                                                                                                                                 _:
                                                                                                                                     *const std::os::raw::c_char,
                                                                                                                                 _:
                                                                                                                                     ...)
                                                                                                                ->
                                                                                                                    ())),
                                             ctxt as *mut std::os::raw::c_void);
                    schemas = xmlSchemaParse(ctxt);
                    xmlSchemaFreeParserCtxt(ctxt);
                    if schemas.is_null() {
                        test_log(b"valid schemas %s failed to parse\n\x00" as
                                     *const u8 as *const std::os::raw::c_char, path);
                        ret = 1 as std::os::raw::c_int;
                        nb_errors += 1
                    }
                    if ret == 0 as std::os::raw::c_int &&
                           !strstr(testErrors.as_mut_ptr(),
                                   b"nimplemented\x00" as *const u8 as
                                       *const std::os::raw::c_char).is_null() {
                        test_log(b"valid schemas %s hit an unimplemented block\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 path);
                        ret = 1 as std::os::raw::c_int;
                        nb_unimplemented += 1;
                        nb_errors += 1
                    }
                    instance =
                        getNext(cur,
                                b"./ts:instanceTest[1]\x00" as *const u8 as
                                    *const std::os::raw::c_char);
                    while !instance.is_null() {
                        if !schemas.is_null() {
                            xstcTestInstance(instance, schemas, path, base);
                        } else {
                            /*
		* We'll automatically mark the instances as failed
		* if the schema was broken.
		*/
                            nb_errors += 1
                        }
                        instance =
                            getNext(instance,
                                    b"following-sibling::ts:instanceTest[1]\x00"
                                        as *const u8 as *const std::os::raw::c_char)
                    }
                } else if xmlStrEqual(validity,
                                      b"invalid\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
                              != 0 {
                    nb_schematas += 1;
                    ctxt =
                        xmlSchemaNewParserCtxt(path as *const std::os::raw::c_char);
                    xmlSchemaSetParserErrors(ctxt,
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlSchemaValidityErrorFunc>(Some(testErrorHandler
                                                                                                          as
                                                                                                          unsafe extern "C" fn(_:
                                                                                                                                   *mut std::os::raw::c_void,
                                                                                                                               _:
                                                                                                                                   *const std::os::raw::c_char,
                                                                                                                               _:
                                                                                                                                   ...)
                                                                                                              ->
                                                                                                                  ())),
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut std::os::raw::c_void,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    ()>,
                                                                     xmlSchemaValidityWarningFunc>(Some(testErrorHandler
                                                                                                            as
                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                     *mut std::os::raw::c_void,
                                                                                                                                 _:
                                                                                                                                     *const std::os::raw::c_char,
                                                                                                                                 _:
                                                                                                                                     ...)
                                                                                                                ->
                                                                                                                    ())),
                                             ctxt as *mut std::os::raw::c_void);
                    schemas = xmlSchemaParse(ctxt);
                    xmlSchemaFreeParserCtxt(ctxt);
                    if !schemas.is_null() {
                        test_log(b"Failed to detect error in schemas %s\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 path);
                        nb_errors += 1;
                        ret = 1 as std::os::raw::c_int
                    }
                    if ret == 0 as std::os::raw::c_int &&
                           !strstr(testErrors.as_mut_ptr(),
                                   b"nimplemented\x00" as *const u8 as
                                       *const std::os::raw::c_char).is_null() {
                        nb_unimplemented += 1;
                        test_log(b"invalid schemas %s hit an unimplemented block\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 path);
                        ret = 1 as std::os::raw::c_int;
                        nb_errors += 1
                    }
                } else {
                    test_log(b"testGroup line %ld misses unexpected validity value%s\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             xmlGetLineNo(cur as *const xmlNode), validity);
                    ret = -(1 as std::os::raw::c_int)
                }
            }
        }
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as
                                                        *mut std::os::raw::c_void);
    }
    if !path.is_null() {
        xmlFree.expect("non-null function pointer")(path as
                                                        *mut std::os::raw::c_void);
    }
    if !validity.is_null() {
        xmlFree.expect("non-null function pointer")(validity as
                                                        *mut std::os::raw::c_void);
    }
    if !schemas.is_null() { xmlSchemaFree(schemas); }
    xmlResetLastError();
    if mem != xmlMemUsed() && extraMemoryFromResolver == 0 as std::os::raw::c_int {
        test_log(b"Processing test line %ld %s leaked %d\n\x00" as *const u8
                     as *const std::os::raw::c_char,
                 xmlGetLineNo(cur as *const xmlNode), path,
                 xmlMemUsed() - mem);
        nb_leaks += 1
    }
    return ret;
}
unsafe extern "C" fn xstcMetadata(mut metadata: *const std::os::raw::c_char,
                                  mut base: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut contributor: *mut xmlChar = 0 as *mut xmlChar;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    doc =
        xmlReadFile(metadata, 0 as *const std::os::raw::c_char,
                    XML_PARSE_NOENT as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Failed to parse %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, metadata);
        return -(1 as std::os::raw::c_int)
    }
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if cur.is_null() ||
           xmlStrEqual((*cur).name,
                       b"testSet\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
        fprintf(stderr,
                b"Unexpected format %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, metadata);
        return -(1 as std::os::raw::c_int)
    }
    contributor =
        xmlGetProp(cur as *const xmlNode,
                   b"contributor\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    if contributor.is_null() {
        contributor =
            xmlStrdup(b"Unknown\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar)
    }
    name =
        xmlGetProp(cur as *const xmlNode,
                   b"name\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    if name.is_null() {
        name =
            xmlStrdup(b"Unknown\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar)
    }
    printf(b"## %s test suite for Schemas version %s\n\x00" as *const u8 as
               *const std::os::raw::c_char, contributor, name);
    xmlFree.expect("non-null function pointer")(contributor as
                                                    *mut std::os::raw::c_void);
    xmlFree.expect("non-null function pointer")(name as *mut std::os::raw::c_void);
    cur =
        getNext(cur,
                b"./ts:testGroup[1]\x00" as *const u8 as *const std::os::raw::c_char);
    if cur.is_null() ||
           xmlStrEqual((*cur).name,
                       b"testGroup\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
        fprintf(stderr,
                b"Unexpected format %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, metadata);
        ret = -(1 as std::os::raw::c_int)
    } else {
        while !cur.is_null() {
            xstcTestGroup(cur, base);
            cur =
                getNext(cur,
                        b"following-sibling::ts:testGroup[1]\x00" as *const u8
                            as *const std::os::raw::c_char)
        }
    }
    xmlFreeDoc(doc);
    return ret;
}
/* ***********************************************************************
 *									*
 *		The driver for the tests				*
 *									*
 ************************************************************************/
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut old_errors: std::os::raw::c_int = 0;
    let mut old_tests: std::os::raw::c_int = 0;
    let mut old_leaks: std::os::raw::c_int = 0;
    logfile =
        fopen(b"runsuite.log\x00" as *const u8 as *const std::os::raw::c_char,
              b"w\x00" as *const u8 as *const std::os::raw::c_char);
    if logfile.is_null() {
        fprintf(stderr,
                b"Could not open the log file, running in verbose mode\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
        verbose = 1 as std::os::raw::c_int
    }
    initializeLibxml2();
    if argc >= 2 as std::os::raw::c_int &&
           strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                  b"-v\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        verbose = 1 as std::os::raw::c_int
    }
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    xsdTest();
    if nb_errors == old_errors && nb_leaks == old_leaks {
        printf(b"Ran %d tests, no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests);
    } else {
        printf(b"Ran %d tests, %d errors, %d leaks\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests,
               nb_errors - old_errors, nb_leaks - old_leaks);
    }
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    rngTest1();
    if nb_errors == old_errors && nb_leaks == old_leaks {
        printf(b"Ran %d tests, no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests);
    } else {
        printf(b"Ran %d tests, %d errors, %d leaks\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests,
               nb_errors - old_errors, nb_leaks - old_leaks);
    }
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    rngTest2();
    if nb_errors == old_errors && nb_leaks == old_leaks {
        printf(b"Ran %d tests, no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests);
    } else {
        printf(b"Ran %d tests, %d errors, %d leaks\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests,
               nb_errors - old_errors, nb_leaks - old_leaks);
    }
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    nb_internals = 0 as std::os::raw::c_int;
    nb_schematas = 0 as std::os::raw::c_int;
    xstcMetadata(b"xstc/Tests/Metadata/NISTXMLSchemaDatatypes.testSet\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 b"xstc/Tests/Metadata/\x00" as *const u8 as
                     *const std::os::raw::c_char);
    if nb_errors == old_errors && nb_leaks == old_leaks {
        printf(b"Ran %d tests (%d schemata), no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests, nb_schematas);
    } else {
        printf(b"Ran %d tests (%d schemata), %d errors (%d internals), %d leaks\n\x00"
                   as *const u8 as *const std::os::raw::c_char, nb_tests - old_tests,
               nb_schematas, nb_errors - old_errors, nb_internals,
               nb_leaks - old_leaks);
    }
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    nb_internals = 0 as std::os::raw::c_int;
    nb_schematas = 0 as std::os::raw::c_int;
    xstcMetadata(b"xstc/Tests/Metadata/SunXMLSchema1-0-20020116.testSet\x00"
                     as *const u8 as *const std::os::raw::c_char,
                 b"xstc/Tests/\x00" as *const u8 as *const std::os::raw::c_char);
    if nb_errors == old_errors && nb_leaks == old_leaks {
        printf(b"Ran %d tests (%d schemata), no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests, nb_schematas);
    } else {
        printf(b"Ran %d tests (%d schemata), %d errors (%d internals), %d leaks\n\x00"
                   as *const u8 as *const std::os::raw::c_char, nb_tests - old_tests,
               nb_schematas, nb_errors - old_errors, nb_internals,
               nb_leaks - old_leaks);
    }
    old_errors = nb_errors;
    old_tests = nb_tests;
    old_leaks = nb_leaks;
    nb_internals = 0 as std::os::raw::c_int;
    nb_schematas = 0 as std::os::raw::c_int;
    xstcMetadata(b"xstc/Tests/Metadata/MSXMLSchema1-0-20020116.testSet\x00" as
                     *const u8 as *const std::os::raw::c_char,
                 b"xstc/Tests/\x00" as *const u8 as *const std::os::raw::c_char);
    if nb_errors == old_errors && nb_leaks == old_leaks {
        printf(b"Ran %d tests (%d schemata), no errors\n\x00" as *const u8 as
                   *const std::os::raw::c_char, nb_tests - old_tests, nb_schematas);
    } else {
        printf(b"Ran %d tests (%d schemata), %d errors (%d internals), %d leaks\n\x00"
                   as *const u8 as *const std::os::raw::c_char, nb_tests - old_tests,
               nb_schematas, nb_errors - old_errors, nb_internals,
               nb_leaks - old_leaks);
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
    xmlXPathFreeContext(ctxtXPath);
    xmlCleanupParser();
    xmlMemoryDump();
    if !logfile.is_null() { fclose(logfile); }
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
/* !SCHEMAS */
