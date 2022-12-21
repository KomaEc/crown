
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
 * Summary: implementation of the Relax-NG validation
 * Description: implementation of the Relax-NG validation
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    pub type _xmlRelaxNG;
    /* *
 * A schemas validation context
 */
    pub type _xmlRelaxNGParserCtxt;
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
    /* *
 * A schemas validation context
 */
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
    #[no_mangle]
    static mut __xmlRegisterCallbacks: std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
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
    fn xmlDictFree(dict: xmlDictPtr);
    /*
 * Lookup of entry in the dictionary.
 */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlDictQLookup(dict: xmlDictPtr, prefix: *const xmlChar,
                      name: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufferCreate() -> xmlBufferPtr;
    #[no_mangle]
    fn xmlBufferFree(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlBufferCat(buf: xmlBufferPtr, str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeDtd(cur: xmlDtdPtr);
    #[no_mangle]
    fn xmlFreeNs(cur: xmlNsPtr);
    #[no_mangle]
    fn xmlFreeNsList(cur: xmlNsPtr);
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlCopyDtd(dtd: xmlDtdPtr) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar)
     -> xmlNodePtr;
    #[no_mangle]
    fn xmlDocCopyNode(node: xmlNodePtr, doc: xmlDocPtr,
                      recursive: std::os::raw::c_int) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED */
    /*
 * Navigating.
 */
    #[no_mangle]
    fn xmlGetLineNo(node: *const xmlNode) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlIsBlankNode(node: *const xmlNode) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlUnlinkNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlFreeNode(cur: xmlNodePtr);
    /*
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr,
                   nameSpace: *const xmlChar) -> xmlNsPtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_XINCLUDE_ENABLED) || \
	  defined(LIBXML_SCHEMAS_ENABLED) || defined(LIBXML_HTML_ENABLED) */
    #[no_mangle]
    fn xmlGetNoNsProp(node: *const xmlNode, name: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlGetNsProp(node: *const xmlNode, name: *const xmlChar,
                    nameSpace: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeListGetString(doc: xmlDocPtr, list: *const xmlNode,
                            inLine: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufGetNodeContent(buf: xmlBufPtr, cur: *const xmlNode)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeGetSpacePreserve(cur: *const xmlNode) -> std::os::raw::c_int;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeDump(buf: xmlBufferPtr, doc: xmlDocPtr, cur: xmlNodePtr,
                   level: std::os::raw::c_int, format: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    /*
 * Retrieve the userdata.
 */
    #[no_mangle]
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar)
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
    fn xmlParserValidityError(ctx: *mut std::os::raw::c_void,
                              msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlParserValidityWarning(ctx: *mut std::os::raw::c_void,
                                msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlIsID(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeRefTable(table: xmlRefTablePtr);
    #[no_mangle]
    fn xmlValidatePushElement(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                              elem: xmlNodePtr, qname: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidatePushCData(ctxt: xmlValidCtxtPtr, data: *const xmlChar,
                            len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidatePopElement(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                             elem: xmlNodePtr, qname: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFindCharEncodingHandler(name: *const std::os::raw::c_char)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateFilename(URI: *const std::os::raw::c_char,
                                          enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateFd(fd: std::os::raw::c_int, enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateStatic(mem: *const std::os::raw::c_char,
                                        size: std::os::raw::c_int,
                                        enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferCreateIO(ioread: xmlInputReadCallback,
                                    ioclose: xmlInputCloseCallback,
                                    ioctx: *mut std::os::raw::c_void,
                                    enc: xmlCharEncoding)
     -> xmlParserInputBufferPtr;
    #[no_mangle]
    fn xmlParserInputBufferRead(in_0: xmlParserInputBufferPtr,
                                len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    #[no_mangle]
    fn xmlParserGetDirectory(filename: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn xmlStopParser(ctxt: xmlParserCtxtPtr);
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
    /*
 * Index lookup, actually implemented in the encoding module
 */
    #[no_mangle]
    fn xmlByteConsumed(ctxt: xmlParserCtxtPtr) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: std::os::raw::c_int)
     -> std::os::raw::c_int;
    /* LIBXML_SAX1_ENABLED */
    #[no_mangle]
    fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: std::os::raw::c_int)
     -> std::os::raw::c_int;
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
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
    /*
 * Interfaces for parsing.
 */
    #[no_mangle]
    fn xmlRelaxNGNewParserCtxt(URL: *const std::os::raw::c_char)
     -> xmlRelaxNGParserCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    #[no_mangle]
    fn xmlRelaxNGSetParserErrors(ctxt: xmlRelaxNGParserCtxtPtr,
                                 err: xmlRelaxNGValidityErrorFunc,
                                 warn: xmlRelaxNGValidityWarningFunc,
                                 ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    #[no_mangle]
    fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
    /* LIBXML_OUTPUT_ENABLED */
    /*
 * Interfaces for validating
 */
    #[no_mangle]
    fn xmlRelaxNGSetValidErrors(ctxt: xmlRelaxNGValidCtxtPtr,
                                err: xmlRelaxNGValidityErrorFunc,
                                warn: xmlRelaxNGValidityWarningFunc,
                                ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGSetValidStructuredErrors(ctxt: xmlRelaxNGValidCtxtPtr,
                                          serror: xmlStructuredErrorFunc,
                                          ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr)
     -> xmlRelaxNGValidCtxtPtr;
    #[no_mangle]
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    /*
 * Interfaces for progressive validation when possible
 */
    #[no_mangle]
    fn xmlRelaxNGValidatePushElement(ctxt: xmlRelaxNGValidCtxtPtr,
                                     doc: xmlDocPtr, elem: xmlNodePtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRelaxNGValidatePushCData(ctxt: xmlRelaxNGValidCtxtPtr,
                                   data: *const xmlChar, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRelaxNGValidatePopElement(ctxt: xmlRelaxNGValidCtxtPtr,
                                    doc: xmlDocPtr, elem: xmlNodePtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRelaxNGValidateFullElement(ctxt: xmlRelaxNGValidCtxtPtr,
                                     doc: xmlDocPtr, elem: xmlNodePtr)
     -> std::os::raw::c_int;
    /*
 * Interfaces for parsing.
 */
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
    fn xmlSchemaIsValid(ctxt: xmlSchemaValidCtxtPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    #[no_mangle]
    fn xmlSchemaFree(schema: xmlSchemaPtr);
    /* LIBXML_OUTPUT_ENABLED */
    /*
 * Interfaces for validating
 */
    #[no_mangle]
    fn xmlSchemaSetValidErrors(ctxt: xmlSchemaValidCtxtPtr,
                               err: xmlSchemaValidityErrorFunc,
                               warn: xmlSchemaValidityWarningFunc,
                               ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSchemaSetValidStructuredErrors(ctxt: xmlSchemaValidCtxtPtr,
                                         serror: xmlStructuredErrorFunc,
                                         ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    #[no_mangle]
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    #[no_mangle]
    fn xmlSchemaSAXPlug(ctxt: xmlSchemaValidCtxtPtr,
                        sax: *mut xmlSAXHandlerPtr,
                        user_data: *mut *mut std::os::raw::c_void)
     -> xmlSchemaSAXPlugPtr;
    #[no_mangle]
    fn xmlSchemaSAXUnplug(plug: xmlSchemaSAXPlugPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaValidateSetLocator(vctxt: xmlSchemaValidCtxtPtr,
                                   f: xmlSchemaValidityLocatorFunc,
                                   ctxt: *mut std::os::raw::c_void);
    /* *
 * xmlPatternFlags:
 *
 * This is the set of options affecting the behaviour of pattern
 * matching with this module
 *
 */
    /* simple pattern match */
    /* standard XPath pattern */
    /* XPath subset for schema selector */
    /* XPath subset for schema field */
    #[no_mangle]
    fn xmlFreePattern(comp: xmlPatternPtr);
    /*
 * standalone processing
 */
    /*
 * contextual processing
 */
    #[no_mangle]
    fn xmlXIncludeFreeContext(ctxt: xmlXIncludeCtxtPtr);
    #[no_mangle]
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXIncludeProcessNode(ctxt: xmlXIncludeCtxtPtr, tree: xmlNodePtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXIncludeSetFlags(ctxt: xmlXIncludeCtxtPtr, flags: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXIncludeNewContext(doc: xmlDocPtr) -> xmlXIncludeCtxtPtr;
    #[no_mangle]
    fn xmlPatterncompile(pattern: *const xmlChar, dict: *mut xmlDict,
                         flags: std::os::raw::c_int, namespaces: *mut *const xmlChar)
     -> xmlPatternPtr;
    #[no_mangle]
    fn xmlTextReaderValidityWarningRelay(ctx: *mut std::os::raw::c_void,
                                         msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlTextReaderValidityErrorRelay(ctx: *mut std::os::raw::c_void,
                                       msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlTextReaderValidityWarning(ctxt: *mut std::os::raw::c_void,
                                    msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlTextReaderWarning(ctxt: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                            _: ...);
    #[no_mangle]
    fn xmlTextReaderValidityError(ctxt: *mut std::os::raw::c_void,
                                  msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn xmlTextReaderError(ctxt: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                          _: ...);
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    /* *
 * XML_SUBSTITUTE_NONE:
 *
 * If no entities need to be substituted.
 */
    /* *
 * XML_SUBSTITUTE_REF:
 *
 * Whether general entities need to be substituted.
 */
    /* *
 * XML_SUBSTITUTE_PEREF:
 *
 * Whether parameter entities need to be substituted.
 */
    /* *
 * XML_SUBSTITUTE_BOTH:
 *
 * Both general and parameter entities need to be substituted.
 */
    /*
 * Generated by MACROS on top of parser.c c.f. PUSH_AND_POP.
 */
    #[no_mangle]
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlSwitchToEncoding(ctxt: xmlParserCtxtPtr,
                           handler: xmlCharEncodingHandlerPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    #[no_mangle]
    fn xmlBufSetAllocationScheme(buf: xmlBufPtr,
                                 scheme: xmlBufferAllocationScheme)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufGetAllocationScheme(buf: xmlBufPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufFree(buf: xmlBufPtr);
    #[no_mangle]
    fn xmlBufEmpty(buf: xmlBufPtr);
    #[no_mangle]
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr)
     -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
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
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlID {
    pub next: *mut _xmlID,
    pub value: *const xmlChar,
    pub attr: xmlAttrPtr,
    pub name: *const xmlChar,
    pub lineno: std::os::raw::c_int,
    pub doc: *mut _xmlDoc,
}
pub type xmlID = _xmlID;
pub type xmlIDPtr = *mut xmlID;
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
pub type xmlGenericErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
/*
 * ALL IDs attributes are stored in a table.
 * There is one table per document.
 */
pub type xmlIDTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
/*
 * ALL Refs attributes are stored in a table.
 * There is one table per document.
 */
pub type xmlRefTable = _xmlHashTable;
pub type xmlRefTablePtr = *mut xmlRefTable;
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
pub type xmlCharEncoding = std::os::raw::c_int;
/* pure ASCII */
/* EUC-JP */
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
/* Shift_JIS */
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
/* ISO-2022-JP */
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
/* ISO-8859-9 */
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
/* ISO-8859-8 */
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
/* ISO-8859-7 */
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
/* ISO-8859-6 */
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
/* ISO-8859-5 */
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
/* ISO-8859-4 */
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
/* ISO-8859-3 */
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
/* ISO-8859-2 ISO Latin 2 */
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
/* ISO-8859-1 ISO Latin 1 */
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
/* UCS-2 */
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
/* UCS-4 unusual ordering */
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
/* UCS-4 unusual ordering */
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
/* EBCDIC uh! */
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
/* UCS-4 big endian */
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
/* UCS-4 little endian */
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
/* UTF-16 big endian */
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
/* UTF-16 little endian */
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
/* UTF-8 */
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
/* No char encoding detected */
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
/* No char encoding detected */
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
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
/* *
 * xmlDeregisterNodeFunc:
 * @node: the current node
 *
 * Signature for the deregistration callback of a discarded node
 */
pub type xmlDeregisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
/* *
 * xmlRelaxNGValidityErrorFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of an error callback from a Relax-NG validation
 */
pub type xmlRelaxNGValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * xmlRelaxNGValidityWarningFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of a warning callback from a Relax-NG validation
 */
pub type xmlRelaxNGValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
/* *
 * xmlSchemaValidityErrorFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of an error callback from an XSD validation
 */
pub type xmlSchemaValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * xmlSchemaValidityWarningFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of a warning callback from an XSD validation
 */
pub type xmlSchemaValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
/* *
 * xmlSchemaValidityLocatorFunc:
 * @ctx: user provided context
 * @file: returned file information
 * @line: returned line information
 *
 * A schemas validation locator, a callback called by the validator.
 * This is used when file or node informations are not available
 * to find out what file and line number are affected
 *
 * Returns: 0 in case of success and -1 in case of error
 */
pub type xmlSchemaValidityLocatorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                _: *mut *const std::os::raw::c_char,
                                _: *mut std::os::raw::c_ulong) -> std::os::raw::c_int>;
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
pub type xmlParserSeverities = std::os::raw::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_TEXTREADER_MODE_READING: C2RustUnnamed_0 = 5;
pub const XML_TEXTREADER_MODE_CLOSED: C2RustUnnamed_0 = 4;
pub const XML_TEXTREADER_MODE_EOF: C2RustUnnamed_0 = 3;
pub const XML_TEXTREADER_MODE_ERROR: C2RustUnnamed_0 = 2;
pub const XML_TEXTREADER_MODE_INTERACTIVE: C2RustUnnamed_0 = 1;
pub const XML_TEXTREADER_MODE_INITIAL: C2RustUnnamed_0 = 0;
pub type xmlParserProperties = std::os::raw::c_uint;
pub const XML_PARSER_SUBST_ENTITIES: xmlParserProperties = 4;
pub const XML_PARSER_VALIDATE: xmlParserProperties = 3;
pub const XML_PARSER_DEFAULTATTRS: xmlParserProperties = 2;
pub const XML_PARSER_LOADDTD: xmlParserProperties = 1;
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_1 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_1 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_1 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_1 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_1 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_1 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_1 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_1 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_1 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_1 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_1 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_1 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_1 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_1 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_1 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_1 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_1 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_1 = 0;
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
/* *
 * xmlFreeID:
 * @not:  A id
 *
 * Deallocate the memory used by an id definition
 */
unsafe extern "C" fn xmlFreeID(mut id: xmlIDPtr) {
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if id.is_null() { return }
    if !(*id).doc.is_null() { dict = (*(*id).doc).dict }
    if !(*id).value.is_null() {
        if !(*id).value.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*id).value) == 0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*id).value as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    xmlFree.expect("non-null function pointer")(id as *mut std::os::raw::c_void);
}
/* *
 * xmlTextReaderRemoveID:
 * @doc:  the document
 * @attr:  the attribute
 *
 * Remove the given attribute from the ID table maintained internally.
 *
 * Returns -1 if the lookup failed and 0 otherwise
 */
unsafe extern "C" fn xmlTextReaderRemoveID(mut doc: xmlDocPtr,
                                           mut attr: xmlAttrPtr)
 -> std::os::raw::c_int {
    let mut table: xmlIDTablePtr = 0 as *mut xmlIDTable;
    let mut id: xmlIDPtr = 0 as *mut xmlID;
    let mut ID: *mut xmlChar = 0 as *mut xmlChar;
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    if attr.is_null() { return -(1 as std::os::raw::c_int) }
    table = (*doc).ids as xmlIDTablePtr;
    if table.is_null() { return -(1 as std::os::raw::c_int) }
    ID = xmlNodeListGetString(doc, (*attr).children, 1 as std::os::raw::c_int);
    if ID.is_null() { return -(1 as std::os::raw::c_int) }
    id = xmlHashLookup(table, ID) as xmlIDPtr;
    xmlFree.expect("non-null function pointer")(ID as *mut std::os::raw::c_void);
    if id.is_null() || (*id).attr != attr { return -(1 as std::os::raw::c_int) }
    (*id).name = (*attr).name;
    (*id).attr = 0 as xmlAttrPtr;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderFreeProp:
 * @reader:  the xmlTextReaderPtr used
 * @cur:  the node
 *
 * Free a node.
 */
unsafe extern "C" fn xmlTextReaderFreeProp(mut reader: xmlTextReaderPtr,
                                           mut cur: xmlAttrPtr) {
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict
    } else { dict = 0 as xmlDictPtr }
    if cur.is_null() { return }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                     as
                                                                                     xmlNodePtr);
    }
    /* Check for ID removal -> leading to invalid references ! */
    if !(*cur).parent.is_null() && !(*(*cur).parent).doc.is_null() &&
           (!(*(*(*cur).parent).doc).intSubset.is_null() ||
                !(*(*(*cur).parent).doc).extSubset.is_null()) {
        if xmlIsID((*(*cur).parent).doc, (*cur).parent, cur) != 0 {
            xmlTextReaderRemoveID((*(*cur).parent).doc, cur);
        }
    }
    if !(*cur).children.is_null() {
        xmlTextReaderFreeNodeList(reader, (*cur).children);
    }
    if !(*cur).name.is_null() &&
           (dict.is_null() ||
                xmlDictOwns(dict, (*cur).name) == 0 as std::os::raw::c_int) {
        xmlFree.expect("non-null function pointer")((*cur).name as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !reader.is_null() && !(*reader).ctxt.is_null() &&
           (*(*reader).ctxt).freeAttrsNr < 100 as std::os::raw::c_int {
        (*cur).next = (*(*reader).ctxt).freeAttrs;
        (*(*reader).ctxt).freeAttrs = cur;
        (*(*reader).ctxt).freeAttrsNr += 1
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
    };
}
/* *
 * xmlTextReaderFreePropList:
 * @reader:  the xmlTextReaderPtr used
 * @cur:  the first property in the list
 *
 * Free a property and all its siblings, all the children are freed too.
 */
unsafe extern "C" fn xmlTextReaderFreePropList(mut reader: xmlTextReaderPtr,
                                               mut cur: xmlAttrPtr) {
    let mut next: xmlAttrPtr = 0 as *mut xmlAttr;
    while !cur.is_null() {
        next = (*cur).next;
        xmlTextReaderFreeProp(reader, cur);
        cur = next
    };
}
/* *
 * xmlTextReaderFreeNodeList:
 * @reader:  the xmlTextReaderPtr used
 * @cur:  the first node in the list
 *
 * Free a node and all its siblings, this is a recursive behaviour, all
 * the children are freed too.
 */
unsafe extern "C" fn xmlTextReaderFreeNodeList(mut reader: xmlTextReaderPtr,
                                               mut cur: xmlNodePtr) {
    let mut next: xmlNodePtr = 0 as *mut xmlNode;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict
    } else { dict = 0 as xmlDictPtr }
    if cur.is_null() { return }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeNsList(cur as xmlNsPtr);
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeDoc(cur as xmlDocPtr);
        return
    }
    while !cur.is_null() {
        next = (*cur).next;
        /* unroll to speed up freeing the document */
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if !(*cur).children.is_null() &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                if (*(*cur).children).parent == cur {
                    xmlTextReaderFreeNodeList(reader, (*cur).children);
                }
                (*cur).children = 0 as *mut _xmlNode
            }
            if __xmlRegisterCallbacks != 0 &&
                   (*__xmlDeregisterNodeDefaultValue()).is_some() {
                (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
            }
            if ((*cur).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
                   !(*cur).properties.is_null() {
                xmlTextReaderFreePropList(reader, (*cur).properties);
            }
            if (*cur).content !=
                   &mut (*cur).properties as *mut *mut _xmlAttr as
                       *mut xmlChar &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                if !(*cur).content.is_null() &&
                       (dict.is_null() ||
                            xmlDictOwns(dict,
                                        (*cur).content as *const xmlChar) ==
                                0 as std::os::raw::c_int) {
                    xmlFree.expect("non-null function pointer")((*cur).content
                                                                    as
                                                                    *mut std::os::raw::c_char
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            if ((*cur).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
                   !(*cur).nsDef.is_null() {
                xmlFreeNsList((*cur).nsDef);
            }
            /*
	     * we don't free element names here they are interned now
	     */
            if (*cur).type_0 as std::os::raw::c_uint !=
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*cur).type_0 as std::os::raw::c_uint !=
                       XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                if !(*cur).name.is_null() &&
                       (dict.is_null() ||
                            xmlDictOwns(dict, (*cur).name) ==
                                0 as std::os::raw::c_int) {
                    xmlFree.expect("non-null function pointer")((*cur).name as
                                                                    *mut std::os::raw::c_char
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            if ((*cur).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*cur).type_0 as std::os::raw::c_uint ==
                        XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint) &&
                   !reader.is_null() && !(*reader).ctxt.is_null() &&
                   (*(*reader).ctxt).freeElemsNr < 100 as std::os::raw::c_int {
                (*cur).next = (*(*reader).ctxt).freeElems;
                (*(*reader).ctxt).freeElems = cur;
                (*(*reader).ctxt).freeElemsNr += 1
            } else {
                xmlFree.expect("non-null function pointer")(cur as
                                                                *mut std::os::raw::c_void);
            }
        }
        cur = next
    };
}
/* ***********************************************************************
 *									*
 *	Our own version of the freeing routines as we recycle nodes	*
 *									*
 ************************************************************************/
/* *
 * DICT_FREE:
 * @str:  a string
 *
 * Free a string if it is not owned by the "dict" dictionary in the
 * current scope
 */
/* *
 * xmlTextReaderFreeNode:
 * @reader:  the xmlTextReaderPtr used
 * @cur:  the node
 *
 * Free a node, this is a recursive behaviour, all the children are freed too.
 * This doesn't unlink the child from the list, use xmlUnlinkNode() first.
 */
unsafe extern "C" fn xmlTextReaderFreeNode(mut reader: xmlTextReaderPtr,
                                           mut cur: xmlNodePtr) {
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict
    } else { dict = 0 as xmlDictPtr }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeDtd(cur as xmlDtdPtr);
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        xmlFreeNs(cur as xmlNsPtr);
        return
    }
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlTextReaderFreeProp(reader, cur as xmlAttrPtr);
        return
    }
    if !(*cur).children.is_null() &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*(*cur).children).parent == cur {
            xmlTextReaderFreeNodeList(reader, (*cur).children);
        }
        (*cur).children = 0 as *mut _xmlNode
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    if ((*cur).type_0 as std::os::raw::c_uint ==
            XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
           !(*cur).properties.is_null() {
        xmlTextReaderFreePropList(reader, (*cur).properties);
    }
    if (*cur).content !=
           &mut (*cur).properties as *mut *mut _xmlAttr as *mut xmlChar &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*cur).content.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*cur).content as *const xmlChar) ==
                        0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*cur).content as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    if ((*cur).type_0 as std::os::raw::c_uint ==
            XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) &&
           !(*cur).nsDef.is_null() {
        xmlFreeNsList((*cur).nsDef);
    }
    /*
     * we don't free names here they are interned now
     */
    if (*cur).type_0 as std::os::raw::c_uint !=
           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*cur).name.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*cur).name) == 0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*cur).name as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    if ((*cur).type_0 as std::os::raw::c_uint ==
            XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
            (*cur).type_0 as std::os::raw::c_uint ==
                XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint) &&
           !reader.is_null() && !(*reader).ctxt.is_null() &&
           (*(*reader).ctxt).freeElemsNr < 100 as std::os::raw::c_int {
        (*cur).next = (*(*reader).ctxt).freeElems;
        (*(*reader).ctxt).freeElems = cur;
        (*(*reader).ctxt).freeElemsNr += 1
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
    };
}
unsafe extern "C" fn xmlTextReaderFreeIDTableEntry(mut id: *mut std::os::raw::c_void,
                                                   mut name: *const xmlChar) {
    xmlFreeID(id as xmlIDPtr);
}
/* *
 * xmlTextReaderFreeIDTable:
 * @table:  An id table
 *
 * Deallocate the memory used by an ID hash table.
 */
unsafe extern "C" fn xmlTextReaderFreeIDTable(mut table: xmlIDTablePtr) {
    xmlHashFree(table,
                Some(xmlTextReaderFreeIDTableEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
}
/* *
 * xmlTextReaderFreeDoc:
 * @reader:  the xmlTextReaderPtr used
 * @cur:  pointer to the document
 *
 * Free up all the structures used by a document, tree included.
 */
unsafe extern "C" fn xmlTextReaderFreeDoc(mut reader: xmlTextReaderPtr,
                                          mut cur: xmlDocPtr) {
    let mut extSubset: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut intSubset: xmlDtdPtr = 0 as *mut xmlDtd;
    if cur.is_null() { return }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur
                                                                                     as
                                                                                     xmlNodePtr);
    }
    /*
     * Do this before freeing the children list to avoid ID lookups
     */
    if !(*cur).ids.is_null() {
        xmlTextReaderFreeIDTable((*cur).ids as xmlIDTablePtr);
    }
    (*cur).ids = 0 as *mut std::os::raw::c_void;
    if !(*cur).refs.is_null() {
        xmlFreeRefTable((*cur).refs as xmlRefTablePtr);
    }
    (*cur).refs = 0 as *mut std::os::raw::c_void;
    extSubset = (*cur).extSubset;
    intSubset = (*cur).intSubset;
    if intSubset == extSubset { extSubset = 0 as xmlDtdPtr }
    if !extSubset.is_null() {
        xmlUnlinkNode((*cur).extSubset as xmlNodePtr);
        (*cur).extSubset = 0 as *mut _xmlDtd;
        xmlFreeDtd(extSubset);
    }
    if !intSubset.is_null() {
        xmlUnlinkNode((*cur).intSubset as xmlNodePtr);
        (*cur).intSubset = 0 as *mut _xmlDtd;
        xmlFreeDtd(intSubset);
    }
    if !(*cur).children.is_null() {
        xmlTextReaderFreeNodeList(reader, (*cur).children);
    }
    if !(*cur).version.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).version as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).name.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).name as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).encoding.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).encoding as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).oldNs.is_null() { xmlFreeNsList((*cur).oldNs); }
    if !(*cur).URL.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).URL as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).dict.is_null() { xmlDictFree((*cur).dict); }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* ***********************************************************************
 *									*
 *			The reader core parser				*
 *									*
 ************************************************************************/
/* *
 * xmlTextReaderEntPush:
 * @reader:  the xmlTextReaderPtr used
 * @value:  the entity reference node
 *
 * Pushes a new entity reference node on top of the entities stack
 *
 * Returns 0 in case of error, the index in the stack otherwise
 */
unsafe extern "C" fn xmlTextReaderEntPush(mut reader: xmlTextReaderPtr,
                                          mut value: xmlNodePtr)
 -> std::os::raw::c_int {
    if (*reader).entMax <= 0 as std::os::raw::c_int {
        (*reader).entMax = 10 as std::os::raw::c_int;
        (*reader).entTab =
            xmlMalloc.expect("non-null function pointer")(((*reader).entMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if (*reader).entTab.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"xmlMalloc failed !\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    if (*reader).entNr >= (*reader).entMax {
        (*reader).entMax *= 2 as std::os::raw::c_int;
        (*reader).entTab =
            xmlRealloc.expect("non-null function pointer")((*reader).entTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*reader).entMax
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if (*reader).entTab.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"xmlRealloc failed !\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    let ref mut fresh0 = *(*reader).entTab.offset((*reader).entNr as isize);
    *fresh0 = value;
    (*reader).ent = value;
    let fresh1 = (*reader).entNr;
    (*reader).entNr = (*reader).entNr + 1;
    return fresh1;
}
/* *
 * xmlTextReaderEntPop:
 * @reader:  the xmlTextReaderPtr used
 *
 * Pops the top element entity from the entities stack
 *
 * Returns the entity just removed
 */
unsafe extern "C" fn xmlTextReaderEntPop(mut reader: xmlTextReaderPtr)
 -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    if (*reader).entNr <= 0 as std::os::raw::c_int { return 0 as xmlNodePtr }
    (*reader).entNr -= 1;
    if (*reader).entNr > 0 as std::os::raw::c_int {
        (*reader).ent =
            *(*reader).entTab.offset(((*reader).entNr - 1 as std::os::raw::c_int) as
                                         isize)
    } else { (*reader).ent = 0 as xmlNodePtr }
    ret = *(*reader).entTab.offset((*reader).entNr as isize);
    let ref mut fresh2 = *(*reader).entTab.offset((*reader).entNr as isize);
    *fresh2 = 0 as xmlNodePtr;
    return ret;
}
/* *
 * xmlTextReaderStartElement:
 * @ctx: the user data (XML parser context)
 * @fullname:  The element name, including namespace prefix
 * @atts:  An array of name/value attributes pairs, NULL terminated
 *
 * called when an opening tag has been processed.
 */
unsafe extern "C" fn xmlTextReaderStartElement(mut ctx: *mut std::os::raw::c_void,
                                               mut fullname: *const xmlChar,
                                               mut atts:
                                                   *mut *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).startElement.is_some() {
        (*reader).startElement.expect("non-null function pointer")(ctx,
                                                                   fullname,
                                                                   atts);
        if !(*ctxt).node.is_null() && !(*ctxt).input.is_null() &&
               !(*(*ctxt).input).cur.is_null() &&
               *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == '/' as i32 &&
               *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == '>' as i32 {
            (*(*ctxt).node).extra = 0x1 as std::os::raw::c_int as std::os::raw::c_ushort
        }
    }
    if !reader.is_null() { (*reader).state = XML_TEXTREADER_ELEMENT };
}
/* *
 * xmlTextReaderEndElement:
 * @ctx: the user data (XML parser context)
 * @fullname:  The element name, including namespace prefix
 *
 * called when an ending tag has been processed.
 */
unsafe extern "C" fn xmlTextReaderEndElement(mut ctx: *mut std::os::raw::c_void,
                                             mut fullname: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).endElement.is_some() {
        (*reader).endElement.expect("non-null function pointer")(ctx,
                                                                 fullname);
    };
}
/* *
 * xmlTextReaderStartElementNs:
 * @ctx: the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 * @nb_namespaces:  number of namespace definitions on that node
 * @namespaces:  pointer to the array of prefix/URI pairs namespace definitions
 * @nb_attributes:  the number of attributes on that node
 * nb_defaulted:  the number of defaulted attributes.
 * @attributes:  pointer to the array of (localname/prefix/URI/value/end)
 *               attribute values.
 *
 * called when an opening tag has been processed.
 */
unsafe extern "C" fn xmlTextReaderStartElementNs(mut ctx: *mut std::os::raw::c_void,
                                                 mut localname:
                                                     *const xmlChar,
                                                 mut prefix: *const xmlChar,
                                                 mut URI: *const xmlChar,
                                                 mut nb_namespaces:
                                                     std::os::raw::c_int,
                                                 mut namespaces:
                                                     *mut *const xmlChar,
                                                 mut nb_attributes:
                                                     std::os::raw::c_int,
                                                 mut nb_defaulted:
                                                     std::os::raw::c_int,
                                                 mut attributes:
                                                     *mut *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).startElementNs.is_some() {
        (*reader).startElementNs.expect("non-null function pointer")(ctx,
                                                                     localname,
                                                                     prefix,
                                                                     URI,
                                                                     nb_namespaces,
                                                                     namespaces,
                                                                     nb_attributes,
                                                                     nb_defaulted,
                                                                     attributes);
        if !(*ctxt).node.is_null() && !(*ctxt).input.is_null() &&
               !(*(*ctxt).input).cur.is_null() &&
               *(*(*ctxt).input).cur.offset(0 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == '/' as i32 &&
               *(*(*ctxt).input).cur.offset(1 as std::os::raw::c_int as isize) as
                   std::os::raw::c_int == '>' as i32 {
            (*(*ctxt).node).extra = 0x1 as std::os::raw::c_int as std::os::raw::c_ushort
        }
    }
    if !reader.is_null() { (*reader).state = XML_TEXTREADER_ELEMENT };
}
/* *
 * xmlTextReaderEndElementNs:
 * @ctx: the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 *
 * called when an ending tag has been processed.
 */
unsafe extern "C" fn xmlTextReaderEndElementNs(mut ctx: *mut std::os::raw::c_void,
                                               mut localname: *const xmlChar,
                                               mut prefix: *const xmlChar,
                                               mut URI: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).endElementNs.is_some() {
        (*reader).endElementNs.expect("non-null function pointer")(ctx,
                                                                   localname,
                                                                   prefix,
                                                                   URI);
    };
}
/* *
 * xmlTextReaderCharacters:
 * @ctx: the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * receiving some chars from the parser.
 */
unsafe extern "C" fn xmlTextReaderCharacters(mut ctx: *mut std::os::raw::c_void,
                                             mut ch: *const xmlChar,
                                             mut len: std::os::raw::c_int) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).characters.is_some() {
        (*reader).characters.expect("non-null function pointer")(ctx, ch,
                                                                 len);
    };
}
/* *
 * xmlTextReaderCDataBlock:
 * @ctx: the user data (XML parser context)
 * @value:  The pcdata content
 * @len:  the block length
 *
 * called when a pcdata block has been parsed
 */
unsafe extern "C" fn xmlTextReaderCDataBlock(mut ctx: *mut std::os::raw::c_void,
                                             mut ch: *const xmlChar,
                                             mut len: std::os::raw::c_int) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).cdataBlock.is_some() {
        (*reader).cdataBlock.expect("non-null function pointer")(ctx, ch,
                                                                 len);
    };
}
/* *
 * xmlTextReaderPushData:
 * @reader:  the xmlTextReaderPtr used
 *
 * Push data down the progressive parser until a significant callback
 * got raised.
 *
 * Returns -1 in case of failure, 0 otherwise
 */
unsafe extern "C" fn xmlTextReaderPushData(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut inbuf: xmlBufPtr = 0 as *mut xmlBuf;
    let mut val: std::os::raw::c_int = 0;
    let mut s: std::os::raw::c_int = 0;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    let mut alloc: std::os::raw::c_int = 0;
    if (*reader).input.is_null() || (*(*reader).input).buffer.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    oldstate = (*reader).state;
    (*reader).state = XML_TEXTREADER_NONE;
    inbuf = (*(*reader).input).buffer;
    alloc = xmlBufGetAllocationScheme(inbuf);
    while (*reader).state as std::os::raw::c_int == XML_TEXTREADER_NONE as std::os::raw::c_int
          {
        if xmlBufUse(inbuf) <
               (*reader).cur.wrapping_add(512 as std::os::raw::c_int as std::os::raw::c_uint)
                   as std::os::raw::c_ulong {
            /*
	     * Refill the buffer unless we are at the end of the stream
	     */
            if !((*reader).mode != XML_TEXTREADER_MODE_EOF as std::os::raw::c_int) {
                break ;
            }
            val =
                xmlParserInputBufferRead((*reader).input,
                                         4096 as std::os::raw::c_int);
            if val == 0 as std::os::raw::c_int &&
                   alloc == XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int {
                if xmlBufUse(inbuf) == (*reader).cur as std::os::raw::c_ulong {
                    (*reader).mode = XML_TEXTREADER_MODE_EOF as std::os::raw::c_int;
                    (*reader).state = oldstate
                }
            } else if val < 0 as std::os::raw::c_int {
                (*reader).mode = XML_TEXTREADER_MODE_EOF as std::os::raw::c_int;
                (*reader).state = oldstate;
                if oldstate as std::os::raw::c_int !=
                       XML_TEXTREADER_START as std::os::raw::c_int ||
                       !(*(*reader).ctxt).myDoc.is_null() {
                    return val
                }
            } else if val == 0 as std::os::raw::c_int {
                /* mark the end of the stream and process the remains */
                (*reader).mode = XML_TEXTREADER_MODE_EOF as std::os::raw::c_int;
                break ;
            }
        }
        /*
	 * parse by block of CHUNK_SIZE bytes, various tests show that
	 * it's the best tradeoff at least on a 1.2GH Duron
	 */
        if xmlBufUse(inbuf) >=
               (*reader).cur.wrapping_add(512 as std::os::raw::c_int as std::os::raw::c_uint)
                   as std::os::raw::c_ulong {
            val =
                xmlParseChunk((*reader).ctxt,
                              (xmlBufContent(inbuf as *const xmlBuf) as
                                   *const std::os::raw::c_char).offset((*reader).cur
                                                                   as isize),
                              512 as std::os::raw::c_int, 0 as std::os::raw::c_int);
            (*reader).cur =
                (*reader).cur.wrapping_add(512 as std::os::raw::c_int as
                                               std::os::raw::c_uint);
            if val != 0 as std::os::raw::c_int {
                (*(*reader).ctxt).wellFormed = 0 as std::os::raw::c_int
            }
            if (*(*reader).ctxt).wellFormed == 0 as std::os::raw::c_int { break ; }
        } else {
            s =
                xmlBufUse(inbuf).wrapping_sub((*reader).cur as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
            val =
                xmlParseChunk((*reader).ctxt,
                              (xmlBufContent(inbuf as *const xmlBuf) as
                                   *const std::os::raw::c_char).offset((*reader).cur
                                                                   as isize),
                              s, 0 as std::os::raw::c_int);
            (*reader).cur = (*reader).cur.wrapping_add(s as std::os::raw::c_uint);
            if val != 0 as std::os::raw::c_int {
                (*(*reader).ctxt).wellFormed = 0 as std::os::raw::c_int
            }
            break ;
        }
    }
    /*
     * Discard the consumed input when needed and possible
     */
    if (*reader).mode == XML_TEXTREADER_MODE_INTERACTIVE as std::os::raw::c_int {
        if alloc != XML_BUFFER_ALLOC_IMMUTABLE as std::os::raw::c_int {
            if (*reader).cur >= 4096 as std::os::raw::c_int as std::os::raw::c_uint &&
                   xmlBufUse(inbuf).wrapping_sub((*reader).cur as
                                                     std::os::raw::c_ulong) <=
                       512 as std::os::raw::c_int as std::os::raw::c_ulong {
                val =
                    xmlBufShrink(inbuf, (*reader).cur as size_t) as
                        std::os::raw::c_int;
                if val >= 0 as std::os::raw::c_int {
                    (*reader).cur =
                        (*reader).cur.wrapping_sub(val as std::os::raw::c_uint)
                }
            }
        }
    } else if (*reader).mode == XML_TEXTREADER_MODE_EOF as std::os::raw::c_int {
        if (*reader).state as std::os::raw::c_int !=
               XML_TEXTREADER_DONE as std::os::raw::c_int {
            s =
                xmlBufUse(inbuf).wrapping_sub((*reader).cur as std::os::raw::c_ulong)
                    as std::os::raw::c_int;
            val =
                xmlParseChunk((*reader).ctxt,
                              (xmlBufContent(inbuf as *const xmlBuf) as
                                   *const std::os::raw::c_char).offset((*reader).cur
                                                                   as isize),
                              s, 1 as std::os::raw::c_int);
            (*reader).cur = xmlBufUse(inbuf) as std::os::raw::c_uint;
            (*reader).state = XML_TEXTREADER_DONE;
            if val != 0 as std::os::raw::c_int {
                if (*(*reader).ctxt).wellFormed != 0 {
                    (*(*reader).ctxt).wellFormed = 0 as std::os::raw::c_int
                } else { return -(1 as std::os::raw::c_int) }
            }
        }
    }
    (*reader).state = oldstate;
    if (*(*reader).ctxt).wellFormed == 0 as std::os::raw::c_int {
        (*reader).mode = XML_TEXTREADER_MODE_EOF as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
/*
     * At the end of the stream signal that the work is done to the Push
     * parser.
     */
/* *
 * xmlTextReaderValidatePush:
 * @reader:  the xmlTextReaderPtr used
 *
 * Push the current node for validation
 */
unsafe extern "C" fn xmlTextReaderValidatePush(mut reader: xmlTextReaderPtr) {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_DTD as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*reader).ctxt.is_null() &&
           (*(*reader).ctxt).validate == 1 as std::os::raw::c_int {
        if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
            (*(*reader).ctxt).valid &=
                xmlValidatePushElement(&mut (*(*reader).ctxt).vctxt,
                                       (*(*reader).ctxt).myDoc, node,
                                       (*node).name)
        } else {
            /* TODO use the BuildQName interface */
            let mut qname: *mut xmlChar = 0 as *mut xmlChar;
            qname = xmlStrdup((*(*node).ns).prefix);
            qname =
                xmlStrcat(qname,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid &=
                xmlValidatePushElement(&mut (*(*reader).ctxt).vctxt,
                                       (*(*reader).ctxt).myDoc, node, qname);
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as
                                                                *mut std::os::raw::c_void);
            }
        }
    }
    /* LIBXML_VALID_ENABLED */
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_RNG as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*reader).rngValidCtxt.is_null() {
        let mut ret: std::os::raw::c_int = 0;
        if !(*reader).rngFullNode.is_null() { return }
        ret =
            xmlRelaxNGValidatePushElement((*reader).rngValidCtxt,
                                          (*(*reader).ctxt).myDoc, node);
        if ret == 0 as std::os::raw::c_int {
            /*
	     * this element requires a full tree
	     */
            node = xmlTextReaderExpand(reader);
            if node.is_null() {
                printf(b"Expand failed !\n\x00" as *const u8 as
                           *const std::os::raw::c_char);
                ret = -(1 as std::os::raw::c_int)
            } else {
                ret =
                    xmlRelaxNGValidateFullElement((*reader).rngValidCtxt,
                                                  (*(*reader).ctxt).myDoc,
                                                  node);
                (*reader).rngFullNode = node
            }
        }
        if ret != 1 as std::os::raw::c_int { (*reader).rngValidErrors += 1 }
    };
}
/* *
 * xmlTextReaderValidateCData:
 * @reader:  the xmlTextReaderPtr used
 * @data:  pointer to the CData
 * @len:  length of the CData block in bytes.
 *
 * Push some CData for validation
 */
unsafe extern "C" fn xmlTextReaderValidateCData(mut reader: xmlTextReaderPtr,
                                                mut data: *const xmlChar,
                                                mut len: std::os::raw::c_int) {
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_DTD as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*reader).ctxt.is_null() &&
           (*(*reader).ctxt).validate == 1 as std::os::raw::c_int {
        (*(*reader).ctxt).valid &=
            xmlValidatePushCData(&mut (*(*reader).ctxt).vctxt, data, len)
    }
    /* LIBXML_VALID_ENABLED */
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_RNG as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*reader).rngValidCtxt.is_null() {
        let mut ret: std::os::raw::c_int = 0;
        if !(*reader).rngFullNode.is_null() { return }
        ret = xmlRelaxNGValidatePushCData((*reader).rngValidCtxt, data, len);
        if ret != 1 as std::os::raw::c_int { (*reader).rngValidErrors += 1 }
    };
}
/* *
 * xmlTextReaderValidatePop:
 * @reader:  the xmlTextReaderPtr used
 *
 * Pop the current node from validation
 */
unsafe extern "C" fn xmlTextReaderValidatePop(mut reader: xmlTextReaderPtr) {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_DTD as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*reader).ctxt.is_null() &&
           (*(*reader).ctxt).validate == 1 as std::os::raw::c_int {
        if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
            (*(*reader).ctxt).valid &=
                xmlValidatePopElement(&mut (*(*reader).ctxt).vctxt,
                                      (*(*reader).ctxt).myDoc, node,
                                      (*node).name)
        } else {
            /* TODO use the BuildQName interface */
            let mut qname: *mut xmlChar = 0 as *mut xmlChar;
            qname = xmlStrdup((*(*node).ns).prefix);
            qname =
                xmlStrcat(qname,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid &=
                xmlValidatePopElement(&mut (*(*reader).ctxt).vctxt,
                                      (*(*reader).ctxt).myDoc, node, qname);
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as
                                                                *mut std::os::raw::c_void);
            }
        }
    }
    /* LIBXML_VALID_ENABLED */
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_RNG as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*reader).rngValidCtxt.is_null() {
        let mut ret: std::os::raw::c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            if node == (*reader).rngFullNode {
                (*reader).rngFullNode = 0 as xmlNodePtr
            }
            return
        }
        ret =
            xmlRelaxNGValidatePopElement((*reader).rngValidCtxt,
                                         (*(*reader).ctxt).myDoc, node);
        if ret != 1 as std::os::raw::c_int { (*reader).rngValidErrors += 1 }
    };
}
/* *
 * xmlTextReaderValidateEntity:
 * @reader:  the xmlTextReaderPtr used
 *
 * Handle the validation when an entity reference is encountered and
 * entity substitution is not activated. As a result the parser interface
 * must walk through the entity and do the validation calls
 */
unsafe extern "C" fn xmlTextReaderValidateEntity(mut reader:
                                                     xmlTextReaderPtr) {
    let mut oldnode: xmlNodePtr = (*reader).node;
    let mut node: xmlNodePtr = (*reader).node;
    let mut ctxt: xmlParserCtxtPtr = (*reader).ctxt;
    let mut current_block_33: u64;
    loop  {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /*
	     * Case where the underlying tree is not availble, lookup the entity
	     * and walk it.
	     */
            if (*node).children.is_null() && !(*ctxt).sax.is_null() &&
                   (*(*ctxt).sax).getEntity.is_some() {
                (*node).children =
                    (*(*ctxt).sax).getEntity.expect("non-null function pointer")(ctxt
                                                                                     as
                                                                                     *mut std::os::raw::c_void,
                                                                                 (*node).name)
                        as xmlNodePtr
            }
            if !(*node).children.is_null() &&
                   (*(*node).children).type_0 as std::os::raw::c_uint ==
                       XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
                   !(*(*node).children).children.is_null() {
                xmlTextReaderEntPush(reader, node);
                node = (*(*node).children).children;
                current_block_33 = 8258075665625361029;
            } else {
                /*
		 * The error has probably be raised already.
		 */
                if node == oldnode { break ; }
                node = (*node).next;
                current_block_33 = 12124785117276362961;
            }
        } else {
            if (*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                (*reader).node = node;
                xmlTextReaderValidatePush(reader);
            } else if (*node).type_0 as std::os::raw::c_uint ==
                          XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                          (*node).type_0 as std::os::raw::c_uint ==
                              XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                xmlTextReaderValidateCData(reader, (*node).content,
                                           xmlStrlen((*node).content));
            }
            current_block_33 = 12124785117276362961;
        }
        match current_block_33 {
            12124785117276362961 =>
            /*
	 * go to next node
	 */
            {
                if !(*node).children.is_null() {
                    node = (*node).children
                } else {
                    if (*node).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                        xmlTextReaderValidatePop(reader);
                    }
                    if !(*node).next.is_null() {
                        node = (*node).next
                    } else {
                        loop  {
                            node = (*node).parent;
                            if (*node).type_0 as std::os::raw::c_uint ==
                                   XML_ELEMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                                let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                                if (*reader).entNr == 0 as std::os::raw::c_int {
                                    loop  {
                                        tmp = (*node).last;
                                        if tmp.is_null() { break ; }
                                        if !((*tmp).extra as std::os::raw::c_int &
                                                 0x2 as std::os::raw::c_int ==
                                                 0 as std::os::raw::c_int) {
                                            break ;
                                        }
                                        xmlUnlinkNode(tmp);
                                        xmlTextReaderFreeNode(reader, tmp);
                                    }
                                }
                                (*reader).node = node;
                                xmlTextReaderValidatePop(reader);
                            }
                            if (*node).type_0 as std::os::raw::c_uint ==
                                   XML_ENTITY_DECL as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                                   !(*reader).ent.is_null() &&
                                   (*(*reader).ent).children == node {
                                node = xmlTextReaderEntPop(reader)
                            }
                            if node == oldnode { break ; }
                            if !(*node).next.is_null() {
                                node = (*node).next;
                                break ;
                            } else if !(!node.is_null() && node != oldnode) {
                                break ;
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        if !(!node.is_null() && node != oldnode) { break ; }
    }
    (*reader).node = oldnode;
}
/* LIBXML_REGEXP_ENABLED */
/* *
 * xmlTextReaderGetSuccessor:
 * @cur:  the current node
 *
 * Get the successor of a node if available.
 *
 * Returns the successor node or NULL
 */
unsafe extern "C" fn xmlTextReaderGetSuccessor(mut cur: xmlNodePtr)
 -> xmlNodePtr {
    if cur.is_null() { return 0 as xmlNodePtr } /* ERROR */
    if !(*cur).next.is_null() { return (*cur).next }
    loop  {
        cur = (*cur).parent;
        if cur.is_null() { break ; }
        if !(*cur).next.is_null() { return (*cur).next }
        if cur.is_null() { break ; }
    }
    return cur;
}
/* *
 * xmlTextReaderDoExpand:
 * @reader:  the xmlTextReaderPtr used
 *
 * Makes sure that the current node is fully read as well as all its
 * descendant. It means the full DOM subtree must be available at the
 * end of the call.
 *
 * Returns 1 if the node was expanded successfully, 0 if there is no more
 *          nodes to read, or -1 in case of error
 */
unsafe extern "C" fn xmlTextReaderDoExpand(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut val: std::os::raw::c_int = 0;
    if reader.is_null() || (*reader).node.is_null() ||
           (*reader).ctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    loop  {
        if (*(*reader).ctxt).instate as std::os::raw::c_int ==
               XML_PARSER_EOF as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
        if !xmlTextReaderGetSuccessor((*reader).node).is_null() {
            return 1 as std::os::raw::c_int
        }
        if (*(*reader).ctxt).nodeNr < (*reader).depth {
            return 1 as std::os::raw::c_int
        }
        if (*reader).mode == XML_TEXTREADER_MODE_EOF as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
        val = xmlTextReaderPushData(reader);
        if val < 0 as std::os::raw::c_int {
            (*reader).mode = XML_TEXTREADER_MODE_ERROR as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        if !((*reader).mode != XML_TEXTREADER_MODE_EOF as std::os::raw::c_int) {
            break ;
        }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderCollectSiblings:
 * @node:    the first child
 *
 *  Traverse depth-first through all sibling nodes and their children
 *  nodes and concatenate their content. This is an auxiliary function
 *  to xmlTextReaderReadString.
 *
 *  Returns a string containing the content, or NULL in case of error.
 */
unsafe extern "C" fn xmlTextReaderCollectSiblings(mut node: xmlNodePtr)
 -> *mut xmlChar {
    let mut buffer: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    buffer = xmlBufferCreate();
    if buffer.is_null() { return 0 as *mut xmlChar }
    while !node.is_null() {
        match (*node).type_0 as std::os::raw::c_uint {
            3 | 4 => { xmlBufferCat(buffer, (*node).content); }
            1 => {
                let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                tmp = xmlTextReaderCollectSiblings((*node).children);
                xmlBufferCat(buffer, tmp);
                xmlFree.expect("non-null function pointer")(tmp as
                                                                *mut std::os::raw::c_void);
            }
            _ => { }
        }
        node = (*node).next
    }
    ret = (*buffer).content;
    (*buffer).content = 0 as *mut xmlChar;
    xmlBufferFree(buffer);
    return ret;
}
/* *
 * xmlTextReaderRead:
 * @reader:  the xmlTextReaderPtr used
 *
 *  Moves the position of the current instance to the next node in
 *  the stream, exposing its properties.
 *
 *  Returns 1 if the node was read successfully, 0 if there is no more
 *          nodes to read, or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRead(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut val: std::os::raw::c_int = 0;
    let mut olddepth: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    let mut oldnode: xmlNodePtr = 0 as xmlNodePtr;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    (*reader).curnode = 0 as xmlNodePtr;
    if !(*reader).doc.is_null() { return xmlTextReaderReadTree(reader) }
    if (*reader).ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).mode == XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int {
        (*reader).mode = XML_TEXTREADER_MODE_INTERACTIVE as std::os::raw::c_int;
        loop 
             /*
	 * Initial state
	 */
             {
            val = xmlTextReaderPushData(reader);
            if val < 0 as std::os::raw::c_int {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as std::os::raw::c_int;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as std::os::raw::c_int)
            }
            if !((*(*reader).ctxt).node.is_null() &&
                     ((*reader).mode != XML_TEXTREADER_MODE_EOF as std::os::raw::c_int
                          &&
                          (*reader).state as std::os::raw::c_int !=
                              XML_TEXTREADER_DONE as std::os::raw::c_int)) {
                break ;
            }
        }
        if (*(*reader).ctxt).node.is_null() {
            if !(*(*reader).ctxt).myDoc.is_null() {
                (*reader).node = (*(*(*reader).ctxt).myDoc).children
            }
            if (*reader).node.is_null() {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as std::os::raw::c_int;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as std::os::raw::c_int)
            }
            (*reader).state = XML_TEXTREADER_ELEMENT
        } else {
            if !(*(*reader).ctxt).myDoc.is_null() {
                (*reader).node = (*(*(*reader).ctxt).myDoc).children
            }
            if (*reader).node.is_null() {
                (*reader).node =
                    *(*(*reader).ctxt).nodeTab.offset(0 as std::os::raw::c_int as
                                                          isize)
            }
            (*reader).state = XML_TEXTREADER_ELEMENT
        }
        (*reader).depth = 0 as std::os::raw::c_int;
        (*(*reader).ctxt).parseMode = XML_PARSE_READER;
        current_block = 5364280785746004627;
    } else {
        oldstate = (*reader).state;
        olddepth = (*(*reader).ctxt).nodeNr;
        oldnode = (*reader).node;
        current_block = 2045240393593748049;
    }
    'c_11966:
        loop  {
            match current_block {
                2045240393593748049 => {
                    if (*reader).node.is_null() {
                        if (*reader).mode ==
                               XML_TEXTREADER_MODE_EOF as std::os::raw::c_int {
                            return 0 as std::os::raw::c_int
                        } else { return -(1 as std::os::raw::c_int) }
                    }
                    /*
     * If we are not backtracking on ancestors or examined nodes,
     * that the parser didn't finished or that we arent at the end
     * of stream, continue processing.
     */
                    while !(*reader).node.is_null() &&
                              (*(*reader).node).next.is_null() &&
                              (*(*reader).ctxt).nodeNr == olddepth &&
                              (oldstate as std::os::raw::c_int ==
                                   XML_TEXTREADER_BACKTRACK as std::os::raw::c_int ||
                                   (*(*reader).node).children.is_null() ||
                                   (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                       XML_ENTITY_REF_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint ||
                                   !(*(*reader).node).children.is_null() &&
                                       (*(*(*reader).node).children).type_0 as
                                           std::os::raw::c_uint ==
                                           XML_TEXT_NODE as std::os::raw::c_int as
                                               std::os::raw::c_uint &&
                                       (*(*(*reader).node).children).next.is_null()
                                   ||
                                   (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                       XML_DTD_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint ||
                                   (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                       XML_DOCUMENT_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint ||
                                   (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                       XML_HTML_DOCUMENT_NODE as std::os::raw::c_int
                                           as std::os::raw::c_uint) &&
                              ((*(*reader).ctxt).node.is_null() ||
                                   (*(*reader).ctxt).node == (*reader).node ||
                                   (*(*reader).ctxt).node ==
                                       (*(*reader).node).parent) &&
                              (*(*reader).ctxt).instate as std::os::raw::c_int !=
                                  XML_PARSER_EOF as std::os::raw::c_int {
                        val = xmlTextReaderPushData(reader);
                        if val < 0 as std::os::raw::c_int {
                            (*reader).mode =
                                XML_TEXTREADER_MODE_ERROR as std::os::raw::c_int;
                            (*reader).state = XML_TEXTREADER_ERROR;
                            return -(1 as std::os::raw::c_int)
                        }
                        if (*reader).node.is_null() { break 'c_11966 ; }
                    }
                    if oldstate as std::os::raw::c_int !=
                           XML_TEXTREADER_BACKTRACK as std::os::raw::c_int {
                        if !(*(*reader).node).children.is_null() &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint !=
                                   XML_ENTITY_REF_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint !=
                                   XML_XINCLUDE_START as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint !=
                                   XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint
                           {
                            (*reader).node = (*(*reader).node).children;
                            (*reader).depth += 1;
                            (*reader).state = XML_TEXTREADER_ELEMENT;
                            current_block = 5364280785746004627;
                            continue ;
                        }
                    }
                    if !(*(*reader).node).next.is_null() {
                        if oldstate as std::os::raw::c_int ==
                               XML_TEXTREADER_ELEMENT as std::os::raw::c_int &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_ELEMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                               (*(*reader).node).children.is_null() &&
                               (*(*reader).node).extra as std::os::raw::c_int &
                                   0x1 as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                               (*reader).in_xinclude <= 0 as std::os::raw::c_int {
                            (*reader).state = XML_TEXTREADER_END;
                            current_block = 5364280785746004627;
                        } else {
                            if (*reader).validate as std::os::raw::c_uint != 0 &&
                                   (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                       XML_ELEMENT_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint {
                                xmlTextReaderValidatePop(reader);
                            }
                            /* LIBXML_REGEXP_ENABLED */
                            if (*reader).preserves > 0 as std::os::raw::c_int &&
                                   (*(*reader).node).extra as std::os::raw::c_int &
                                       0x4 as std::os::raw::c_int != 0 {
                                (*reader).preserves -= 1
                            }
                            (*reader).node = (*(*reader).node).next;
                            (*reader).state = XML_TEXTREADER_ELEMENT;
                            /*
	 * Cleanup of the old node
	 */
                            if (*reader).preserves == 0 as std::os::raw::c_int &&
                                   (*reader).in_xinclude == 0 as std::os::raw::c_int
                                   && (*reader).entNr == 0 as std::os::raw::c_int &&
                                   !(*(*reader).node).prev.is_null() &&
                                   (*(*(*reader).node).prev).type_0 as
                                       std::os::raw::c_uint !=
                                       XML_DTD_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint {
                                let mut tmp: xmlNodePtr =
                                    (*(*reader).node).prev;
                                if (*tmp).extra as std::os::raw::c_int &
                                       0x2 as std::os::raw::c_int == 0 as std::os::raw::c_int
                                   {
                                    xmlUnlinkNode(tmp);
                                    xmlTextReaderFreeNode(reader, tmp);
                                }
                            }
                            current_block = 5364280785746004627;
                        }
                    } else if oldstate as std::os::raw::c_int ==
                                  XML_TEXTREADER_ELEMENT as std::os::raw::c_int &&
                                  (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                      XML_ELEMENT_NODE as std::os::raw::c_int as
                                          std::os::raw::c_uint &&
                                  (*(*reader).node).children.is_null() &&
                                  (*(*reader).node).extra as std::os::raw::c_int &
                                      0x1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        (*reader).state = XML_TEXTREADER_END;
                        current_block = 5364280785746004627;
                    } else {
                        if (*reader).validate as std::os::raw::c_uint !=
                               XML_TEXTREADER_NOT_VALIDATE as std::os::raw::c_int as
                                   std::os::raw::c_uint &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_ELEMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                            xmlTextReaderValidatePop(reader);
                        }
                        /* LIBXML_REGEXP_ENABLED */
                        if (*reader).preserves > 0 as std::os::raw::c_int &&
                               (*(*reader).node).extra as std::os::raw::c_int &
                                   0x4 as std::os::raw::c_int != 0 {
                            (*reader).preserves -= 1
                        }
                        (*reader).node = (*(*reader).node).parent;
                        if (*reader).node.is_null() ||
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_DOCUMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint ||
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_DOCB_DOCUMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint ||
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                            if (*reader).mode !=
                                   XML_TEXTREADER_MODE_EOF as std::os::raw::c_int {
                                val =
                                    xmlParseChunk((*reader).ctxt,
                                                  b"\x00" as *const u8 as
                                                      *const std::os::raw::c_char,
                                                  0 as std::os::raw::c_int,
                                                  1 as std::os::raw::c_int);
                                (*reader).state = XML_TEXTREADER_DONE;
                                if val != 0 as std::os::raw::c_int {
                                    return -(1 as std::os::raw::c_int)
                                }
                            }
                            (*reader).node = 0 as xmlNodePtr;
                            (*reader).depth = -(1 as std::os::raw::c_int);
                            /*
	 * Cleanup of the old node
	 */
                            if !oldnode.is_null() &&
                                   (*reader).preserves == 0 as std::os::raw::c_int &&
                                   (*reader).in_xinclude == 0 as std::os::raw::c_int
                                   && (*reader).entNr == 0 as std::os::raw::c_int &&
                                   (*oldnode).type_0 as std::os::raw::c_uint !=
                                       XML_DTD_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint &&
                                   (*oldnode).extra as std::os::raw::c_int &
                                       0x2 as std::os::raw::c_int == 0 as std::os::raw::c_int
                               {
                                xmlUnlinkNode(oldnode);
                                xmlTextReaderFreeNode(reader, oldnode);
                            }
                            break ;
                        } else {
                            if (*reader).preserves == 0 as std::os::raw::c_int &&
                                   (*reader).in_xinclude == 0 as std::os::raw::c_int
                                   && (*reader).entNr == 0 as std::os::raw::c_int &&
                                   !(*(*reader).node).last.is_null() &&
                                   (*(*(*reader).node).last).extra as
                                       std::os::raw::c_int & 0x2 as std::os::raw::c_int ==
                                       0 as std::os::raw::c_int {
                                let mut tmp_0: xmlNodePtr =
                                    (*(*reader).node).last;
                                xmlUnlinkNode(tmp_0);
                                xmlTextReaderFreeNode(reader, tmp_0);
                            }
                            (*reader).depth -= 1;
                            (*reader).state = XML_TEXTREADER_BACKTRACK;
                            current_block = 5364280785746004627;
                        }
                    }
                }
                _ => {
                    /*
     * If we are in the middle of a piece of CDATA make sure it's finished
     */
                    if !(*reader).node.is_null() &&
                           (*(*reader).node).next.is_null() &&
                           ((*(*reader).node).type_0 as std::os::raw::c_uint ==
                                XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                                ||
                                (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                    XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                        std::os::raw::c_uint) {
                        if xmlTextReaderExpand(reader).is_null() {
                            return -(1 as std::os::raw::c_int)
                        }
                    }
                    /*
     * Handle XInclude if asked for
     */
                    if (*reader).xinclude != 0 && !(*reader).node.is_null() &&
                           (*(*reader).node).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                           && !(*(*reader).node).ns.is_null() &&
                           (xmlStrEqual((*(*(*reader).node).ns).href,
                                        b"http://www.w3.org/2003/XInclude\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char as
                                            *const xmlChar) != 0 ||
                                xmlStrEqual((*(*(*reader).node).ns).href,
                                            b"http://www.w3.org/2001/XInclude\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char as
                                                *const xmlChar) != 0) {
                        if (*reader).xincctxt.is_null() {
                            (*reader).xincctxt =
                                xmlXIncludeNewContext((*(*reader).ctxt).myDoc);
                            xmlXIncludeSetFlags((*reader).xincctxt,
                                                (*reader).parserFlags &
                                                    !(XML_PARSE_NOXINCNODE as
                                                          std::os::raw::c_int));
                        }
                        /*
	 * expand that node and process it
	 */
                        if xmlTextReaderExpand(reader).is_null() {
                            return -(1 as std::os::raw::c_int)
                        }
                        xmlXIncludeProcessNode((*reader).xincctxt,
                                               (*reader).node);
                    }
                    if !(*reader).node.is_null() &&
                           (*(*reader).node).type_0 as std::os::raw::c_uint ==
                               XML_XINCLUDE_START as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        (*reader).in_xinclude += 1;
                        current_block = 2045240393593748049;
                    } else if !(*reader).node.is_null() &&
                                  (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                      XML_XINCLUDE_END as std::os::raw::c_int as
                                          std::os::raw::c_uint {
                        (*reader).in_xinclude -= 1;
                        current_block = 2045240393593748049;
                    } else {
                        /*
     * Handle entities enter and exit when in entity replacement mode
     */
                        if !(*reader).node.is_null() &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_ENTITY_REF_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                               !(*reader).ctxt.is_null() &&
                               (*(*reader).ctxt).replaceEntities ==
                                   1 as std::os::raw::c_int {
                            /*
	 * Case where the underlying tree is not availble, lookup the entity
	 * and walk it.
	 */
                            if (*(*reader).node).children.is_null() &&
                                   !(*(*reader).ctxt).sax.is_null() &&
                                   (*(*(*reader).ctxt).sax).getEntity.is_some()
                               {
                                (*(*reader).node).children =
                                    (*(*(*reader).ctxt).sax).getEntity.expect("non-null function pointer")((*reader).ctxt
                                                                                                               as
                                                                                                               *mut std::os::raw::c_void,
                                                                                                           (*(*reader).node).name)
                                        as xmlNodePtr
                            }
                            if !(*(*reader).node).children.is_null() &&
                                   (*(*(*reader).node).children).type_0 as
                                       std::os::raw::c_uint ==
                                       XML_ENTITY_DECL as std::os::raw::c_int as
                                           std::os::raw::c_uint &&
                                   !(*(*(*reader).node).children).children.is_null()
                               {
                                xmlTextReaderEntPush(reader, (*reader).node);
                                (*reader).node =
                                    (*(*(*reader).node).children).children
                            }
                        } else if !(*reader).node.is_null() &&
                                      (*(*reader).node).type_0 as std::os::raw::c_uint
                                          ==
                                          XML_ENTITY_REF_NODE as std::os::raw::c_int
                                              as std::os::raw::c_uint &&
                                      !(*reader).ctxt.is_null() &&
                                      (*reader).validate as std::os::raw::c_uint != 0
                         {
                            xmlTextReaderValidateEntity(reader);
                            /* LIBXML_REGEXP_ENABLED */
                        }
                        if !(*reader).node.is_null() &&
                               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                                   XML_ENTITY_DECL as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                               !(*reader).ent.is_null() &&
                               (*(*reader).ent).children == (*reader).node {
                            (*reader).node = xmlTextReaderEntPop(reader);
                            (*reader).depth += 1;
                            current_block = 2045240393593748049;
                        } else {
                            if (*reader).validate as std::os::raw::c_uint !=
                                   XML_TEXTREADER_NOT_VALIDATE as std::os::raw::c_int
                                       as std::os::raw::c_uint &&
                                   !(*reader).node.is_null() {
                                let mut node: xmlNodePtr = (*reader).node;
                                if (*node).type_0 as std::os::raw::c_uint ==
                                       XML_ELEMENT_NODE as std::os::raw::c_int as
                                           std::os::raw::c_uint &&
                                       ((*reader).state as std::os::raw::c_int !=
                                            XML_TEXTREADER_END as std::os::raw::c_int
                                            &&
                                            (*reader).state as std::os::raw::c_int !=
                                                XML_TEXTREADER_BACKTRACK as
                                                    std::os::raw::c_int) {
                                    xmlTextReaderValidatePush(reader);
                                } else if (*node).type_0 as std::os::raw::c_uint ==
                                              XML_TEXT_NODE as std::os::raw::c_int as
                                                  std::os::raw::c_uint ||
                                              (*node).type_0 as std::os::raw::c_uint
                                                  ==
                                                  XML_CDATA_SECTION_NODE as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint {
                                    xmlTextReaderValidateCData(reader,
                                                               (*node).content,
                                                               xmlStrlen((*node).content));
                                }
                            }
                            /* LIBXML_REGEXP_ENABLED */
                            if (*reader).patternNr > 0 as std::os::raw::c_int &&
                                   (*reader).state as std::os::raw::c_int !=
                                       XML_TEXTREADER_END as std::os::raw::c_int &&
                                   (*reader).state as std::os::raw::c_int !=
                                       XML_TEXTREADER_BACKTRACK as std::os::raw::c_int
                               {
                                let mut i: std::os::raw::c_int = 0;
                                i = 0 as std::os::raw::c_int;
                                while i < (*reader).patternNr {
                                    if xmlPatternMatch(*(*reader).patternTab.offset(i
                                                                                        as
                                                                                        isize),
                                                       (*reader).node) ==
                                           1 as std::os::raw::c_int {
                                        xmlTextReaderPreserve(reader);
                                        break ;
                                    } else { i += 1 }
                                }
                            }
                            /* LIBXML_PATTERN_ENABLED */
                            if (*reader).validate as std::os::raw::c_uint ==
                                   XML_TEXTREADER_VALIDATE_XSD as std::os::raw::c_int
                                       as std::os::raw::c_uint &&
                                   (*reader).xsdValidErrors ==
                                       0 as std::os::raw::c_int &&
                                   !(*reader).xsdValidCtxt.is_null() {
                                (*reader).xsdValidErrors =
                                    (xmlSchemaIsValid((*reader).xsdValidCtxt)
                                         == 0) as std::os::raw::c_int
                            }
                            /* LIBXML_PATTERN_ENABLED */
                            return 1 as std::os::raw::c_int
                        }
                    }
                }
            }
        }
    (*reader).state = XML_TEXTREADER_DONE;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderReadState:
 * @reader:  the xmlTextReaderPtr used
 *
 * Gets the read state of the reader.
 *
 * Returns the state value, or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadState(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    return (*reader).mode;
}
/* *
 * xmlTextReaderExpand:
 * @reader:  the xmlTextReaderPtr used
 *
 * Reads the contents of the current node and the full subtree. It then makes
 * the subtree available until the next xmlTextReaderRead() call
 *
 * Returns a node pointer valid until the next xmlTextReaderRead() call
 *         or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderExpand(mut reader: xmlTextReaderPtr)
 -> xmlNodePtr {
    if reader.is_null() || (*reader).node.is_null() { return 0 as xmlNodePtr }
    if !(*reader).doc.is_null() { return (*reader).node }
    if (*reader).ctxt.is_null() { return 0 as xmlNodePtr }
    if xmlTextReaderDoExpand(reader) < 0 as std::os::raw::c_int {
        return 0 as xmlNodePtr
    }
    return (*reader).node;
}
/* *
 * xmlTextReaderNext:
 * @reader:  the xmlTextReaderPtr used
 *
 * Skip to the node following the current one in document order while
 * avoiding the subtree if any.
 *
 * Returns 1 if the node was read successfully, 0 if there is no more
 *          nodes to read, or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNext(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*reader).doc.is_null() { return xmlTextReaderNextTree(reader) }
    cur = (*reader).node;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlTextReaderRead(reader)
    }
    if (*reader).state as std::os::raw::c_int == XML_TEXTREADER_END as std::os::raw::c_int ||
           (*reader).state as std::os::raw::c_int ==
               XML_TEXTREADER_BACKTRACK as std::os::raw::c_int {
        return xmlTextReaderRead(reader)
    }
    if (*cur).extra as std::os::raw::c_int & 0x1 as std::os::raw::c_int != 0 {
        return xmlTextReaderRead(reader)
    }
    loop  {
        ret = xmlTextReaderRead(reader);
        if ret != 1 as std::os::raw::c_int { return ret }
        if !((*reader).node != cur) { break ; }
    }
    return xmlTextReaderRead(reader);
}
/* *
 * xmlTextReaderReadInnerXml:
 * @reader:  the xmlTextReaderPtr used
 *
 * Reads the contents of the current node, including child nodes and markup.
 *
 * Returns a string containing the XML content, or NULL if the current node
 *         is neither an element nor attribute, or has no child nodes. The
 *         string must be deallocated by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadInnerXml(mut reader:
                                                       xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut resbuf: *mut xmlChar = 0 as *mut xmlChar;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur_node: xmlNodePtr = 0 as *mut xmlNode;
    let mut buff: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut buff2: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    if xmlTextReaderExpand(reader).is_null() { return 0 as *mut xmlChar }
    doc = (*reader).doc;
    buff = xmlBufferCreate();
    cur_node = (*(*reader).node).children;
    while !cur_node.is_null() {
        node = xmlDocCopyNode(cur_node, doc, 1 as std::os::raw::c_int);
        buff2 = xmlBufferCreate();
        if xmlNodeDump(buff2, doc, node, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int)
               == -(1 as std::os::raw::c_int) {
            xmlFreeNode(node);
            xmlBufferFree(buff2);
            xmlBufferFree(buff);
            return 0 as *mut xmlChar
        }
        xmlBufferCat(buff, (*buff2).content);
        xmlFreeNode(node);
        xmlBufferFree(buff2);
        cur_node = (*cur_node).next
    }
    resbuf = (*buff).content;
    (*buff).content = 0 as *mut xmlChar;
    xmlBufferFree(buff);
    return resbuf;
}
/* *
 * xmlTextReaderReadOuterXml:
 * @reader:  the xmlTextReaderPtr used
 *
 * Reads the contents of the current node, including child nodes and markup.
 *
 * Returns a string containing the node and any XML content, or NULL if the
 *         current node cannot be serialized. The string must be deallocated
 *         by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadOuterXml(mut reader:
                                                       xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut resbuf: *mut xmlChar = 0 as *mut xmlChar;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut buff: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    node = (*reader).node;
    doc = (*reader).doc;
    if xmlTextReaderExpand(reader).is_null() { return 0 as *mut xmlChar }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        node = xmlCopyDtd(node as xmlDtdPtr) as xmlNodePtr
    } else { node = xmlDocCopyNode(node, doc, 1 as std::os::raw::c_int) }
    buff = xmlBufferCreate();
    if xmlNodeDump(buff, doc, node, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int) ==
           -(1 as std::os::raw::c_int) {
        xmlFreeNode(node);
        xmlBufferFree(buff);
        return 0 as *mut xmlChar
    }
    resbuf = (*buff).content;
    (*buff).content = 0 as *mut xmlChar;
    xmlFreeNode(node);
    xmlBufferFree(buff);
    return resbuf;
}
/* *
 * xmlTextReaderReadString:
 * @reader:  the xmlTextReaderPtr used
 *
 * Reads the contents of an element or a text node as a string.
 *
 * Returns a string containing the contents of the Element or Text node,
 *         or NULL if the reader is positioned on any other type of node.
 *         The string must be deallocated by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadString(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *mut xmlChar
    }
    node =
        if !(*reader).curnode.is_null() {
            (*reader).curnode
        } else { (*reader).node };
    match (*node).type_0 as std::os::raw::c_uint {
        3 => {
            if !(*node).content.is_null() {
                return xmlStrdup((*node).content)
            }
        }
        1 => {
            if xmlTextReaderDoExpand(reader) != -(1 as std::os::raw::c_int) {
                return xmlTextReaderCollectSiblings((*node).children)
            }
        }
        2 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"xmlreader.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       1814 as
                                                                           std::os::raw::c_int);
        }
        _ => { }
    }
    return 0 as *mut xmlChar;
}
/* ***********************************************************************
 *									*
 *			Operating on a preparsed tree			*
 *									*
 ************************************************************************/
unsafe extern "C" fn xmlTextReaderNextTree(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).state as std::os::raw::c_int == XML_TEXTREADER_END as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    if (*reader).node.is_null() {
        if (*(*reader).doc).children.is_null() {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as std::os::raw::c_int
        }
        (*reader).node = (*(*reader).doc).children;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as std::os::raw::c_int
    }
    if (*reader).state as std::os::raw::c_int !=
           XML_TEXTREADER_BACKTRACK as std::os::raw::c_int {
        /* Here removed traversal to child, because we want to skip the subtree,
	replace with traversal to sibling to skip subtree */
        if !(*(*reader).node).next.is_null() {
            /* Move to sibling if present,skipping sub-tree */
            (*reader).node = (*(*reader).node).next;
            (*reader).state = XML_TEXTREADER_START;
            return 1 as std::os::raw::c_int
        }
        /* if reader->node->next is NULL mean no subtree for current node,
	so need to move to sibling of parent node if present */
        if (*(*reader).node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*(*reader).node).type_0 as std::os::raw::c_uint ==
                   XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            (*reader).state = XML_TEXTREADER_BACKTRACK;
            /* This will move to parent if present */
            xmlTextReaderRead(reader);
        }
    }
    if !(*(*reader).node).next.is_null() {
        (*reader).node = (*(*reader).node).next;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as std::os::raw::c_int
    }
    if !(*(*reader).node).parent.is_null() {
        if (*(*(*reader).node).parent).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as std::os::raw::c_int
        }
        (*reader).node = (*(*reader).node).parent;
        (*reader).depth -= 1;
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        /* Repeat process to move to sibling of parent node if present */
        xmlTextReaderNextTree(reader);
    }
    (*reader).state = XML_TEXTREADER_END;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderReadTree:
 * @reader:  the xmlTextReaderPtr used
 *
 *  Moves the position of the current instance to the next node in
 *  the stream, exposing its properties.
 *
 *  Returns 1 if the node was read successfully, 0 if there is no more
 *          nodes to read, or -1 in case of error
 */
unsafe extern "C" fn xmlTextReaderReadTree(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    if (*reader).state as std::os::raw::c_int == XML_TEXTREADER_END as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    loop  {
        if (*reader).node.is_null() {
            if (*(*reader).doc).children.is_null() {
                (*reader).state = XML_TEXTREADER_END;
                return 0 as std::os::raw::c_int
            }
            (*reader).node = (*(*reader).doc).children;
            (*reader).state = XML_TEXTREADER_START
        } else {
            if (*reader).state as std::os::raw::c_int !=
                   XML_TEXTREADER_BACKTRACK as std::os::raw::c_int &&
                   (*(*reader).node).type_0 as std::os::raw::c_uint !=
                       XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*(*reader).node).type_0 as std::os::raw::c_uint !=
                       XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*(*reader).node).type_0 as std::os::raw::c_uint !=
                       XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                if !(*(*reader).node).children.is_null() {
                    (*reader).node = (*(*reader).node).children;
                    (*reader).depth += 1;
                    (*reader).state = XML_TEXTREADER_START;
                    current_block = 18391292360981809970;
                } else if (*(*reader).node).type_0 as std::os::raw::c_uint ==
                              XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                    (*reader).state = XML_TEXTREADER_BACKTRACK;
                    current_block = 18391292360981809970;
                } else { current_block = 5143058163439228106; }
            } else { current_block = 5143058163439228106; }
            match current_block {
                18391292360981809970 => { }
                _ => {
                    if !(*(*reader).node).next.is_null() {
                        (*reader).node = (*(*reader).node).next;
                        (*reader).state = XML_TEXTREADER_START
                    } else if !(*(*reader).node).parent.is_null() {
                        if (*(*(*reader).node).parent).type_0 as std::os::raw::c_uint
                               ==
                               XML_DOCUMENT_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint ||
                               (*(*(*reader).node).parent).type_0 as
                                   std::os::raw::c_uint ==
                                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                            (*reader).state = XML_TEXTREADER_END;
                            return 0 as std::os::raw::c_int
                        }
                        (*reader).node = (*(*reader).node).parent;
                        (*reader).depth -= 1;
                        (*reader).state = XML_TEXTREADER_BACKTRACK
                    } else { (*reader).state = XML_TEXTREADER_END }
                }
            }
        }
        if !((*(*reader).node).type_0 as std::os::raw::c_uint ==
                 XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
                 (*(*reader).node).type_0 as std::os::raw::c_uint ==
                     XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint) {
            break ;
        }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderNextSibling:
 * @reader:  the xmlTextReaderPtr used
 *
 * Skip to the node following the current one in document order while
 * avoiding the subtree if any.
 * Currently implemented only for Readers built on a document
 *
 * Returns 1 if the node was read successfully, 0 if there is no more
 *          nodes to read, or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNextSibling(mut reader:
                                                      xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).doc.is_null() {
        /* TODO */
        return -(1 as std::os::raw::c_int)
    }
    if (*reader).state as std::os::raw::c_int == XML_TEXTREADER_END as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    if (*reader).node.is_null() { return xmlTextReaderNextTree(reader) }
    if !(*(*reader).node).next.is_null() {
        (*reader).node = (*(*reader).node).next;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *			Constructor and destructors			*
 *									*
 ************************************************************************/
/* *
 * xmlNewTextReader:
 * @input: the xmlParserInputBufferPtr used to read data
 * @URI: the URI information for the source if available
 *
 * Create an xmlTextReader structure fed with @input
 *
 * Returns the new xmlTextReaderPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextReader(mut input: xmlParserInputBufferPtr,
                                          mut URI: *const std::os::raw::c_char)
 -> xmlTextReaderPtr {
    let mut ret: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    if input.is_null() { return 0 as xmlTextReaderPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextReader>()
                                                          as std::os::raw::c_ulong) as
            xmlTextReaderPtr;
    if ret.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlNewTextReader : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return 0 as xmlTextReaderPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlTextReader>() as std::os::raw::c_ulong);
    (*ret).doc = 0 as xmlDocPtr;
    (*ret).entTab = 0 as *mut xmlNodePtr;
    (*ret).entMax = 0 as std::os::raw::c_int;
    (*ret).entNr = 0 as std::os::raw::c_int;
    (*ret).input = input;
    (*ret).buffer = xmlBufCreateSize(100 as std::os::raw::c_int as size_t);
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlNewTextReader : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return 0 as xmlTextReaderPtr
    }
    /* no operation on a reader should require a huge buffer */
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_BOUNDED);
    (*ret).sax =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSAXHandler>()
                                                          as std::os::raw::c_ulong) as
            *mut xmlSAXHandler;
    if (*ret).sax.is_null() {
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlNewTextReader : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return 0 as xmlTextReaderPtr
    }
    xmlSAXVersion((*ret).sax, 2 as std::os::raw::c_int);
    (*ret).startElement = (*(*ret).sax).startElement;
    (*(*ret).sax).startElement =
        Some(xmlTextReaderStartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    (*ret).endElement = (*(*ret).sax).endElement;
    (*(*ret).sax).endElement =
        Some(xmlTextReaderEndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    if (*(*ret).sax).initialized == 0xdeedbeaf as std::os::raw::c_uint {
        /* LIBXML_SAX1_ENABLED */
        (*ret).startElementNs = (*(*ret).sax).startElementNs;
        (*(*ret).sax).startElementNs =
            Some(xmlTextReaderStartElementNs as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *const xmlChar,
                                          _: *const xmlChar, _: std::os::raw::c_int,
                                          _: *mut *const xmlChar,
                                          _: std::os::raw::c_int, _: std::os::raw::c_int,
                                          _: *mut *const xmlChar) -> ());
        (*ret).endElementNs = (*(*ret).sax).endElementNs;
        (*(*ret).sax).endElementNs =
            Some(xmlTextReaderEndElementNs as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *const xmlChar,
                                          _: *const xmlChar) -> ())
    } else { (*ret).startElementNs = None; (*ret).endElementNs = None }
    /* LIBXML_SAX1_ENABLED */
    (*ret).characters = (*(*ret).sax).characters;
    (*(*ret).sax).characters =
        Some(xmlTextReaderCharacters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*(*ret).sax).ignorableWhitespace =
        Some(xmlTextReaderCharacters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*ret).cdataBlock = (*(*ret).sax).cdataBlock;
    (*(*ret).sax).cdataBlock =
        Some(xmlTextReaderCDataBlock as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int;
    (*ret).node = 0 as xmlNodePtr;
    (*ret).curnode = 0 as xmlNodePtr;
    if xmlBufUse((*(*ret).input).buffer) < 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        xmlParserInputBufferRead(input, 4 as std::os::raw::c_int);
    }
    if xmlBufUse((*(*ret).input).buffer) >= 4 as std::os::raw::c_int as std::os::raw::c_ulong
       {
        (*ret).ctxt =
            xmlCreatePushParserCtxt((*ret).sax, 0 as *mut std::os::raw::c_void,
                                    xmlBufContent((*(*ret).input).buffer as
                                                      *const xmlBuf) as
                                        *const std::os::raw::c_char, 4 as std::os::raw::c_int,
                                    URI);
        (*ret).base = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        (*ret).cur = 4 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        (*ret).ctxt =
            xmlCreatePushParserCtxt((*ret).sax, 0 as *mut std::os::raw::c_void,
                                    0 as *const std::os::raw::c_char,
                                    0 as std::os::raw::c_int, URI);
        (*ret).base = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        (*ret).cur = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if (*ret).ctxt.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlNewTextReader : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")((*ret).sax as
                                                        *mut std::os::raw::c_void);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlTextReaderPtr
    }
    (*(*ret).ctxt).parseMode = XML_PARSE_READER;
    (*(*ret).ctxt)._private = ret as *mut std::os::raw::c_void;
    (*(*ret).ctxt).linenumbers = 1 as std::os::raw::c_int;
    (*(*ret).ctxt).dictNames = 1 as std::os::raw::c_int;
    (*ret).allocs = 2 as std::os::raw::c_int;
    /*
     * use the parser dictionary to allocate all elements and attributes names
     */
    (*(*ret).ctxt).docdict = 1 as std::os::raw::c_int;
    (*ret).dict = (*(*ret).ctxt).dict;
    (*ret).xinclude = 0 as std::os::raw::c_int;
    (*ret).patternMax = 0 as std::os::raw::c_int;
    (*ret).patternTab = 0 as *mut xmlPatternPtr;
    return ret;
}
/* *
 * xmlNewTextReaderFilename:
 * @URI: the URI of the resource to process
 *
 * Create an xmlTextReader structure fed with the resource at @URI
 *
 * Returns the new xmlTextReaderPtr or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextReaderFilename(mut URI:
                                                      *const std::os::raw::c_char)
 -> xmlTextReaderPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut ret: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut directory: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    input = xmlParserInputBufferCreateFilename(URI, XML_CHAR_ENCODING_NONE);
    if input.is_null() { return 0 as xmlTextReaderPtr }
    ret = xmlNewTextReader(input, URI);
    if ret.is_null() {
        xmlFreeParserInputBuffer(input);
        return 0 as xmlTextReaderPtr
    }
    (*ret).allocs |= 1 as std::os::raw::c_int;
    if (*(*ret).ctxt).directory.is_null() {
        directory = xmlParserGetDirectory(URI)
    }
    if (*(*ret).ctxt).directory.is_null() && !directory.is_null() {
        (*(*ret).ctxt).directory =
            xmlStrdup(directory as *mut xmlChar) as *mut std::os::raw::c_char
    }
    if !directory.is_null() {
        xmlFree.expect("non-null function pointer")(directory as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
/* *
 * xmlFreeTextReader:
 * @reader:  the xmlTextReaderPtr
 *
 * Deallocate all the resources associated to the reader
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeTextReader(mut reader: xmlTextReaderPtr) {
    if reader.is_null() { return }
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = 0 as xmlRelaxNGPtr
    }
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = 0 as xmlRelaxNGValidCtxtPtr
    }
    if !(*reader).xsdPlug.is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        (*reader).xsdPlug = 0 as xmlSchemaSAXPlugPtr
    }
    if !(*reader).xsdValidCtxt.is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr
    }
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = 0 as xmlSchemaPtr
    }
    if !(*reader).xincctxt.is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
    }
    if !(*reader).patternTab.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < (*reader).patternNr {
            if !(*(*reader).patternTab.offset(i as isize)).is_null() {
                xmlFreePattern(*(*reader).patternTab.offset(i as isize));
            }
            i += 1
        }
        xmlFree.expect("non-null function pointer")((*reader).patternTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*reader).faketext.is_null() { xmlFreeNode((*reader).faketext); }
    if !(*reader).ctxt.is_null() {
        if (*reader).dict == (*(*reader).ctxt).dict {
            (*reader).dict = 0 as xmlDictPtr
        }
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as std::os::raw::c_int {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            (*(*reader).ctxt).myDoc = 0 as xmlDocPtr
        }
        if !(*(*reader).ctxt).vctxt.vstateTab.is_null() &&
               (*(*reader).ctxt).vctxt.vstateMax > 0 as std::os::raw::c_int {
            xmlFree.expect("non-null function pointer")((*(*reader).ctxt).vctxt.vstateTab
                                                            as
                                                            *mut std::os::raw::c_void);
            (*(*reader).ctxt).vctxt.vstateTab = 0 as *mut xmlValidState;
            (*(*reader).ctxt).vctxt.vstateMax = 0 as std::os::raw::c_int
        }
        if (*reader).allocs & 2 as std::os::raw::c_int != 0 {
            xmlFreeParserCtxt((*reader).ctxt);
        }
    }
    if !(*reader).sax.is_null() {
        xmlFree.expect("non-null function pointer")((*reader).sax as
                                                        *mut std::os::raw::c_void);
    }
    if !(*reader).input.is_null() && (*reader).allocs & 1 as std::os::raw::c_int != 0
       {
        xmlFreeParserInputBuffer((*reader).input);
    }
    if !(*reader).buffer.is_null() { xmlBufFree((*reader).buffer); }
    if !(*reader).entTab.is_null() {
        xmlFree.expect("non-null function pointer")((*reader).entTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*reader).dict.is_null() { xmlDictFree((*reader).dict); }
    xmlFree.expect("non-null function pointer")(reader as *mut std::os::raw::c_void);
}
/* ***********************************************************************
 *									*
 *			Methods for XmlTextReader			*
 *									*
 ************************************************************************/
/* *
 * xmlTextReaderClose:
 * @reader:  the xmlTextReaderPtr used
 *
 * This method releases any resources allocated by the current instance
 * changes the state to Closed and close any underlying input.
 *
 * Returns 0 or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderClose(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    (*reader).node = 0 as xmlNodePtr;
    (*reader).curnode = 0 as xmlNodePtr;
    (*reader).mode = XML_TEXTREADER_MODE_CLOSED as std::os::raw::c_int;
    if !(*reader).ctxt.is_null() {
        xmlStopParser((*reader).ctxt);
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as std::os::raw::c_int {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            (*(*reader).ctxt).myDoc = 0 as xmlDocPtr
        }
    }
    if !(*reader).input.is_null() && (*reader).allocs & 1 as std::os::raw::c_int != 0
       {
        xmlFreeParserInputBuffer((*reader).input);
        (*reader).allocs -= 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderGetAttributeNo:
 * @reader:  the xmlTextReaderPtr used
 * @no: the zero-based index of the attribute relative to the containing element
 *
 * Provides the value of the attribute with the specified index relative
 * to the containing element.
 *
 * Returns a string containing the value of the specified attribute, or NULL
 *    in case of error. The string must be deallocated by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttributeNo(mut reader:
                                                         xmlTextReaderPtr,
                                                     mut no: std::os::raw::c_int)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: std::os::raw::c_int = 0;
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() { return 0 as *mut xmlChar }
    if (*reader).node.is_null() { return 0 as *mut xmlChar }
    if !(*reader).curnode.is_null() { return 0 as *mut xmlChar }
    /* TODO: handle the xmlDecl */
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    ns = (*(*reader).node).nsDef;
    i = 0 as std::os::raw::c_int;
    while i < no && !ns.is_null() { ns = (*ns).next; i += 1 }
    if !ns.is_null() { return xmlStrdup((*ns).href) }
    cur = (*(*reader).node).properties;
    if cur.is_null() { return 0 as *mut xmlChar }
    while i < no {
        cur = (*cur).next;
        if cur.is_null() { return 0 as *mut xmlChar }
        i += 1
    }
    /* TODO walk the DTD if present */
    ret =
        xmlNodeListGetString((*(*reader).node).doc, (*cur).children,
                             1 as std::os::raw::c_int);
    if ret.is_null() {
        return xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar)
    }
    return ret;
}
/* *
 * xmlTextReaderGetAttribute:
 * @reader:  the xmlTextReaderPtr used
 * @name: the qualified name of the attribute.
 *
 * Provides the value of the attribute with the specified qualified name.
 *
 * Returns a string containing the value of the specified attribute, or NULL
 *    in case of error. The string must be deallocated by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttribute(mut reader:
                                                       xmlTextReaderPtr,
                                                   mut name: *const xmlChar)
 -> *mut xmlChar {
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut localname: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if reader.is_null() || name.is_null() { return 0 as *mut xmlChar }
    if (*reader).node.is_null() { return 0 as *mut xmlChar }
    if !(*reader).curnode.is_null() { return 0 as *mut xmlChar }
    /* TODO: handle the xmlDecl */
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    localname = xmlSplitQName2(name, &mut prefix);
    if localname.is_null() {
        /*
		 * Namespace default decl
		 */
        if xmlStrEqual(name,
                       b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
            ns = (*(*reader).node).nsDef;
            while !ns.is_null() {
                if (*ns).prefix.is_null() { return xmlStrdup((*ns).href) }
                ns = (*ns).next
            }
            return 0 as *mut xmlChar
        }
        return xmlGetNoNsProp((*reader).node as *const xmlNode, name)
    }
    /*
     * Namespace default decl
     */
    if xmlStrEqual(prefix,
                   b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        ns = (*(*reader).node).nsDef;
        while !ns.is_null() {
            if !(*ns).prefix.is_null() &&
                   xmlStrEqual((*ns).prefix, localname) != 0 {
                ret = xmlStrdup((*ns).href);
                break ;
            } else { ns = (*ns).next }
        }
    } else {
        ns = xmlSearchNs((*(*reader).node).doc, (*reader).node, prefix);
        if !ns.is_null() {
            ret =
                xmlGetNsProp((*reader).node as *const xmlNode, localname,
                             (*ns).href)
        }
    }
    xmlFree.expect("non-null function pointer")(localname as
                                                    *mut std::os::raw::c_void);
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
/* *
 * xmlTextReaderGetAttributeNs:
 * @reader:  the xmlTextReaderPtr used
 * @localName: the local name of the attribute.
 * @namespaceURI: the namespace URI of the attribute.
 *
 * Provides the value of the specified attribute
 *
 * Returns a string containing the value of the specified attribute, or NULL
 *    in case of error. The string must be deallocated by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttributeNs(mut reader:
                                                         xmlTextReaderPtr,
                                                     mut localName:
                                                         *const xmlChar,
                                                     mut namespaceURI:
                                                         *const xmlChar)
 -> *mut xmlChar {
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() || localName.is_null() { return 0 as *mut xmlChar }
    if (*reader).node.is_null() { return 0 as *mut xmlChar }
    if !(*reader).curnode.is_null() { return 0 as *mut xmlChar }
    /* TODO: handle the xmlDecl */
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    if xmlStrEqual(namespaceURI,
                   b"http://www.w3.org/2000/xmlns/\x00" as *const u8 as
                       *const std::os::raw::c_char as *mut xmlChar) != 0 {
        if xmlStrEqual(localName,
                       b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
            prefix = localName as *mut xmlChar
        }
        ns = (*(*reader).node).nsDef;
        while !ns.is_null() {
            if prefix.is_null() && (*ns).prefix.is_null() ||
                   !(*ns).prefix.is_null() &&
                       xmlStrEqual((*ns).prefix, localName) != 0 {
                return xmlStrdup((*ns).href)
            }
            ns = (*ns).next
        }
        return 0 as *mut xmlChar
    }
    return xmlGetNsProp((*reader).node as *const xmlNode, localName,
                        namespaceURI);
}
/* *
 * xmlTextReaderGetRemainder:
 * @reader:  the xmlTextReaderPtr used
 *
 * Method to get the remainder of the buffered XML. this method stops the
 * parser, set its state to End Of File and return the input stream with
 * what is left that the parser did not use.
 *
 * The implementation is not good, the parser certainly procgressed past
 * what's left in reader->input, and there is an allocation problem. Best
 * would be to rewrite it differently.
 *
 * Returns the xmlParserInputBufferPtr attached to the XML or NULL
 *    in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetRemainder(mut reader:
                                                       xmlTextReaderPtr)
 -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as xmlParserInputBufferPtr;
    if reader.is_null() { return 0 as xmlParserInputBufferPtr }
    if (*reader).node.is_null() { return 0 as xmlParserInputBufferPtr }
    (*reader).node = 0 as xmlNodePtr;
    (*reader).curnode = 0 as xmlNodePtr;
    (*reader).mode = XML_TEXTREADER_MODE_EOF as std::os::raw::c_int;
    if !(*reader).ctxt.is_null() {
        xmlStopParser((*reader).ctxt);
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as std::os::raw::c_int {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            (*(*reader).ctxt).myDoc = 0 as xmlDocPtr
        }
    }
    if (*reader).allocs & 1 as std::os::raw::c_int != 0 {
        ret = (*reader).input;
        (*reader).input = 0 as xmlParserInputBufferPtr;
        (*reader).allocs -= 1 as std::os::raw::c_int
    } else {
        /*
	 * Hum, one may need to duplicate the data structure because
	 * without reference counting the input may be freed twice:
	 *   - by the layer which allocated it.
	 *   - by the layer to which would have been returned to.
	 */
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"xmlreader.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   2543 as
                                                                       std::os::raw::c_int);
        return 0 as xmlParserInputBufferPtr
    }
    return ret;
}
/* *
 * xmlTextReaderLookupNamespace:
 * @reader:  the xmlTextReaderPtr used
 * @prefix: the prefix whose namespace URI is to be resolved. To return
 *          the default namespace, specify NULL
 *
 * Resolves a namespace prefix in the scope of the current element.
 *
 * Returns a string containing the namespace URI to which the prefix maps
 *    or NULL in case of error. The string must be deallocated by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLookupNamespace(mut reader:
                                                          xmlTextReaderPtr,
                                                      mut prefix:
                                                          *const xmlChar)
 -> *mut xmlChar {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() { return 0 as *mut xmlChar }
    if (*reader).node.is_null() { return 0 as *mut xmlChar }
    ns = xmlSearchNs((*(*reader).node).doc, (*reader).node, prefix);
    if ns.is_null() { return 0 as *mut xmlChar }
    return xmlStrdup((*ns).href);
}
/* *
 * xmlTextReaderMoveToAttributeNo:
 * @reader:  the xmlTextReaderPtr used
 * @no: the zero-based index of the attribute relative to the containing
 *      element.
 *
 * Moves the position of the current instance to the attribute with
 * the specified index relative to the containing element.
 *
 * Returns 1 in case of success, -1 in case of error, 0 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttributeNo(mut reader:
                                                            xmlTextReaderPtr,
                                                        mut no: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    /* TODO: handle the xmlDecl */
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    (*reader).curnode = 0 as xmlNodePtr;
    ns = (*(*reader).node).nsDef;
    i = 0 as std::os::raw::c_int;
    while i < no && !ns.is_null() { ns = (*ns).next; i += 1 }
    if !ns.is_null() {
        (*reader).curnode = ns as xmlNodePtr;
        return 1 as std::os::raw::c_int
    }
    cur = (*(*reader).node).properties;
    if cur.is_null() { return 0 as std::os::raw::c_int }
    while i < no {
        cur = (*cur).next;
        if cur.is_null() { return 0 as std::os::raw::c_int }
        i += 1
    }
    /* TODO walk the DTD if present */
    (*reader).curnode = cur as xmlNodePtr;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderMoveToAttribute:
 * @reader:  the xmlTextReaderPtr used
 * @name: the qualified name of the attribute.
 *
 * Moves the position of the current instance to the attribute with
 * the specified qualified name.
 *
 * Returns 1 in case of success, -1 in case of error, 0 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttribute(mut reader:
                                                          xmlTextReaderPtr,
                                                      mut name:
                                                          *const xmlChar)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut localname: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    if reader.is_null() || name.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    /* TODO: handle the xmlDecl */
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    localname = xmlSplitQName2(name, &mut prefix);
    if localname.is_null() {
        /*
	 * Namespace default decl
	 */
        if xmlStrEqual(name,
                       b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
            ns = (*(*reader).node).nsDef;
            while !ns.is_null() {
                if (*ns).prefix.is_null() {
                    (*reader).curnode = ns as xmlNodePtr;
                    return 1 as std::os::raw::c_int
                }
                ns = (*ns).next
            }
            return 0 as std::os::raw::c_int
        }
        prop = (*(*reader).node).properties;
        while !prop.is_null() {
            /*
	     * One need to have
	     *   - same attribute names
	     *   - and the attribute carrying that namespace
	     */
            if xmlStrEqual((*prop).name, name) != 0 &&
                   ((*prop).ns.is_null() || (*(*prop).ns).prefix.is_null()) {
                (*reader).curnode = prop as xmlNodePtr;
                return 1 as std::os::raw::c_int
            }
            prop = (*prop).next
        }
        return 0 as std::os::raw::c_int
    }
    /*
     * Namespace default decl
     */
    if xmlStrEqual(prefix,
                   b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        ns = (*(*reader).node).nsDef;
        loop  {
            if ns.is_null() { current_block = 595459606561049378; break ; }
            if !(*ns).prefix.is_null() &&
                   xmlStrEqual((*ns).prefix, localname) != 0 {
                (*reader).curnode = ns as xmlNodePtr;
                current_block = 12558844995300341259;
                break ;
            } else { ns = (*ns).next }
        }
    } else {
        prop = (*(*reader).node).properties;
        loop  {
            if prop.is_null() { current_block = 595459606561049378; break ; }
            /*
	 * One need to have
	 *   - same attribute names
	 *   - and the attribute carrying that namespace
	 */
            if xmlStrEqual((*prop).name, localname) != 0 &&
                   !(*prop).ns.is_null() &&
                   xmlStrEqual((*(*prop).ns).prefix, prefix) != 0 {
                (*reader).curnode = prop as xmlNodePtr;
                current_block = 12558844995300341259;
                break ;
            } else { prop = (*prop).next }
        }
    }
    match current_block {
        12558844995300341259 => {
            if !localname.is_null() {
                xmlFree.expect("non-null function pointer")(localname as
                                                                *mut std::os::raw::c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as
                                                                *mut std::os::raw::c_void);
            }
            return 1 as std::os::raw::c_int
        }
        _ => {
            if !localname.is_null() {
                xmlFree.expect("non-null function pointer")(localname as
                                                                *mut std::os::raw::c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as std::os::raw::c_int
        }
    };
}
/* *
 * xmlTextReaderMoveToAttributeNs:
 * @reader:  the xmlTextReaderPtr used
 * @localName:  the local name of the attribute.
 * @namespaceURI:  the namespace URI of the attribute.
 *
 * Moves the position of the current instance to the attribute with the
 * specified local name and namespace URI.
 *
 * Returns 1 in case of success, -1 in case of error, 0 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttributeNs(mut reader:
                                                            xmlTextReaderPtr,
                                                        mut localName:
                                                            *const xmlChar,
                                                        mut namespaceURI:
                                                            *const xmlChar)
 -> std::os::raw::c_int {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    if reader.is_null() || localName.is_null() || namespaceURI.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    node = (*reader).node;
    if xmlStrEqual(namespaceURI,
                   b"http://www.w3.org/2000/xmlns/\x00" as *const u8 as
                       *const std::os::raw::c_char as *mut xmlChar) != 0 {
        if xmlStrEqual(localName,
                       b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
            prefix = localName as *mut xmlChar
        }
        ns = (*(*reader).node).nsDef;
        while !ns.is_null() {
            if prefix.is_null() && (*ns).prefix.is_null() ||
                   !(*ns).prefix.is_null() &&
                       xmlStrEqual((*ns).prefix, localName) != 0 {
                (*reader).curnode = ns as xmlNodePtr;
                return 1 as std::os::raw::c_int
            }
            ns = (*ns).next
        }
        return 0 as std::os::raw::c_int
    }
    prop = (*node).properties;
    while !prop.is_null() {
        /*
	 * One need to have
	 *   - same attribute names
	 *   - and the attribute carrying that namespace
	 */
        if xmlStrEqual((*prop).name, localName) != 0 &&
               (!(*prop).ns.is_null() &&
                    xmlStrEqual((*(*prop).ns).href, namespaceURI) != 0) {
            (*reader).curnode = prop as xmlNodePtr;
            return 1 as std::os::raw::c_int
        }
        prop = (*prop).next
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderMoveToFirstAttribute:
 * @reader:  the xmlTextReaderPtr used
 *
 * Moves the position of the current instance to the first attribute
 * associated with the current node.
 *
 * Returns 1 in case of success, -1 in case of error, 0 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToFirstAttribute(mut reader:
                                                               xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if !(*(*reader).node).nsDef.is_null() {
        (*reader).curnode = (*(*reader).node).nsDef as xmlNodePtr;
        return 1 as std::os::raw::c_int
    }
    if !(*(*reader).node).properties.is_null() {
        (*reader).curnode = (*(*reader).node).properties as xmlNodePtr;
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderMoveToNextAttribute:
 * @reader:  the xmlTextReaderPtr used
 *
 * Moves the position of the current instance to the next attribute
 * associated with the current node.
 *
 * Returns 1 in case of success, -1 in case of error, 0 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToNextAttribute(mut reader:
                                                              xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*reader).curnode.is_null() {
        return xmlTextReaderMoveToFirstAttribute(reader)
    }
    if (*(*reader).curnode).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if !(*ns).next.is_null() {
            (*reader).curnode = (*ns).next as xmlNodePtr;
            return 1 as std::os::raw::c_int
        }
        if !(*(*reader).node).properties.is_null() {
            (*reader).curnode = (*(*reader).node).properties as xmlNodePtr;
            return 1 as std::os::raw::c_int
        }
        return 0 as std::os::raw::c_int
    } else {
        if (*(*reader).curnode).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*(*reader).curnode).next.is_null() {
            (*reader).curnode = (*(*reader).curnode).next;
            return 1 as std::os::raw::c_int
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderMoveToElement:
 * @reader:  the xmlTextReaderPtr used
 *
 * Moves the position of the current instance to the node that
 * contains the current Attribute  node.
 *
 * Returns 1 in case of success, -1 in case of error, 0 if not moved
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToElement(mut reader:
                                                        xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if !(*reader).curnode.is_null() {
        (*reader).curnode = 0 as xmlNodePtr;
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderReadAttributeValue:
 * @reader:  the xmlTextReaderPtr used
 *
 * Parses an attribute value into one or more Text and EntityReference nodes.
 *
 * Returns 1 in case of success, 0 if the reader was not positionned on an
 *         ttribute node or all the attribute values have been read, or -1
 *         in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadAttributeValue(mut reader:
                                                             xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).curnode.is_null() { return 0 as std::os::raw::c_int }
    if (*(*reader).curnode).type_0 as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if (*(*reader).curnode).children.is_null() { return 0 as std::os::raw::c_int }
        (*reader).curnode = (*(*reader).curnode).children
    } else if (*(*reader).curnode).type_0 as std::os::raw::c_uint ==
                  XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if (*reader).faketext.is_null() {
            (*reader).faketext =
                xmlNewDocText((*(*reader).node).doc, (*ns).href)
        } else {
            if !(*(*reader).faketext).content.is_null() &&
                   (*(*reader).faketext).content !=
                       &mut (*(*reader).faketext).properties as
                           *mut *mut _xmlAttr as *mut xmlChar {
                xmlFree.expect("non-null function pointer")((*(*reader).faketext).content
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            (*(*reader).faketext).content = xmlStrdup((*ns).href)
        }
        (*reader).curnode = (*reader).faketext
    } else {
        if (*(*reader).curnode).next.is_null() { return 0 as std::os::raw::c_int }
        (*reader).curnode = (*(*reader).curnode).next
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderConstEncoding:
 * @reader:  the xmlTextReaderPtr used
 *
 * Determine the encoding of the document being read.
 *
 * Returns a string containing the encoding of the document or NULL in
 * case of error.  The string is deallocated with the reader.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstEncoding(mut reader:
                                                        xmlTextReaderPtr)
 -> *const xmlChar {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if reader.is_null() { return 0 as *const xmlChar }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc
    } else if !(*reader).ctxt.is_null() { doc = (*(*reader).ctxt).myDoc }
    if doc.is_null() { return 0 as *const xmlChar }
    if (*doc).encoding.is_null() {
        return 0 as *const xmlChar
    } else {
        return xmlDictLookup((*reader).dict, (*doc).encoding,
                             -(1 as std::os::raw::c_int))
    };
}
/* ***********************************************************************
 *									*
 *			Acces API to the current node			*
 *									*
 ************************************************************************/
/* *
 * xmlTextReaderAttributeCount:
 * @reader:  the xmlTextReaderPtr used
 *
 * Provides the number of attributes of the current node
 *
 * Returns 0 i no attributes, -1 in case of error or the attribute count
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderAttributeCount(mut reader:
                                                         xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return 0 as std::os::raw::c_int }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*reader).state as std::os::raw::c_int == XML_TEXTREADER_END as std::os::raw::c_int ||
           (*reader).state as std::os::raw::c_int ==
               XML_TEXTREADER_BACKTRACK as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    ret = 0 as std::os::raw::c_int;
    attr = (*node).properties;
    while !attr.is_null() { ret += 1; attr = (*attr).next }
    ns = (*node).nsDef;
    while !ns.is_null() { ret += 1; ns = (*ns).next }
    return ret;
}
/* *
 * xmlTextReaderNodeType:
 * @reader:  the xmlTextReaderPtr used
 *
 * Get the node type of the current node
 * Reference:
 * http://www.gnu.org/software/dotgnu/pnetlib-doc/System/Xml/XmlNodeType.html
 *
 * Returns the xmlNodeType of the current node or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNodeType(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return XML_READER_TYPE_NONE as std::os::raw::c_int }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    match (*node).type_0 as std::os::raw::c_uint {
        1 => {
            if (*reader).state as std::os::raw::c_int ==
                   XML_TEXTREADER_END as std::os::raw::c_int ||
                   (*reader).state as std::os::raw::c_int ==
                       XML_TEXTREADER_BACKTRACK as std::os::raw::c_int {
                return XML_READER_TYPE_END_ELEMENT as std::os::raw::c_int
            }
            return XML_READER_TYPE_ELEMENT as std::os::raw::c_int
        }
        18 | 2 => { return XML_READER_TYPE_ATTRIBUTE as std::os::raw::c_int }
        3 => {
            if xmlIsBlankNode((*reader).node as *const xmlNode) != 0 {
                if xmlNodeGetSpacePreserve((*reader).node as *const xmlNode)
                       != 0 {
                    return XML_READER_TYPE_SIGNIFICANT_WHITESPACE as
                               std::os::raw::c_int
                } else { return XML_READER_TYPE_WHITESPACE as std::os::raw::c_int }
            } else { return XML_READER_TYPE_TEXT as std::os::raw::c_int }
        }
        4 => { return XML_READER_TYPE_CDATA as std::os::raw::c_int }
        5 => { return XML_READER_TYPE_ENTITY_REFERENCE as std::os::raw::c_int }
        6 => { return XML_READER_TYPE_ENTITY as std::os::raw::c_int }
        7 => { return XML_READER_TYPE_PROCESSING_INSTRUCTION as std::os::raw::c_int }
        8 => { return XML_READER_TYPE_COMMENT as std::os::raw::c_int }
        9 | 13 | 21 => { return XML_READER_TYPE_DOCUMENT as std::os::raw::c_int }
        11 => { return XML_READER_TYPE_DOCUMENT_FRAGMENT as std::os::raw::c_int }
        12 => { return XML_READER_TYPE_NOTATION as std::os::raw::c_int }
        10 | 14 => { return XML_READER_TYPE_DOCUMENT_TYPE as std::os::raw::c_int }
        15 | 16 | 17 | 19 | 20 => {
            return XML_READER_TYPE_NONE as std::os::raw::c_int
        }
        _ => { }
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlTextReaderIsEmptyElement:
 * @reader:  the xmlTextReaderPtr used
 *
 * Check if the current node is empty
 *
 * Returns 1 if empty, 0 if not and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsEmptyElement(mut reader:
                                                         xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() || (*reader).node.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if (*(*reader).node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if !(*reader).curnode.is_null() { return 0 as std::os::raw::c_int }
    if !(*(*reader).node).children.is_null() { return 0 as std::os::raw::c_int }
    if (*reader).state as std::os::raw::c_int == XML_TEXTREADER_END as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    if !(*reader).doc.is_null() { return 1 as std::os::raw::c_int }
    if (*reader).in_xinclude > 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    return ((*(*reader).node).extra as std::os::raw::c_int & 0x1 as std::os::raw::c_int !=
                0 as std::os::raw::c_int) as std::os::raw::c_int;
}
/* *
 * xmlTextReaderLocalName:
 * @reader:  the xmlTextReaderPtr used
 *
 * The local name of the node.
 *
 * Returns the local name or NULL if not available,
 *   if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocalName(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *mut xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return xmlStrdup(b"xmlns\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar)
        } else { return xmlStrdup((*ns).prefix) }
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlTextReaderName(reader)
    }
    return xmlStrdup((*node).name);
}
/* *
 * xmlTextReaderConstLocalName:
 * @reader:  the xmlTextReaderPtr used
 *
 * The local name of the node.
 *
 * Returns the local name or NULL if not available, the
 *         string will be deallocated with the reader.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstLocalName(mut reader:
                                                         xmlTextReaderPtr)
 -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *const xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return xmlDictLookup((*reader).dict,
                                 b"xmlns\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 -(1 as std::os::raw::c_int))
        } else { return (*ns).prefix }
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlTextReaderConstName(reader)
    }
    return (*node).name;
}
/* *
 * xmlTextReaderName:
 * @reader:  the xmlTextReaderPtr used
 *
 * The qualified name of the node, equal to Prefix :LocalName.
 *
 * Returns the local name or NULL if not available,
 *   if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderName(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *mut xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    match (*node).type_0 as std::os::raw::c_uint {
        1 | 2 => {
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                return xmlStrdup((*node).name)
            }
            ret = xmlStrdup((*(*node).ns).prefix);
            ret =
                xmlStrcat(ret,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            ret = xmlStrcat(ret, (*node).name);
            return ret
        }
        3 => {
            return xmlStrdup(b"#text\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar)
        }
        4 => {
            return xmlStrdup(b"#cdata-section\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar)
        }
        6 | 5 => { return xmlStrdup((*node).name) }
        7 => { return xmlStrdup((*node).name) }
        8 => {
            return xmlStrdup(b"#comment\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar)
        }
        9 | 13 | 21 => {
            return xmlStrdup(b"#document\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar)
        }
        11 => {
            return xmlStrdup(b"#document-fragment\x00" as *const u8 as
                                 *const std::os::raw::c_char as *mut xmlChar)
        }
        12 => { return xmlStrdup((*node).name) }
        10 | 14 => { return xmlStrdup((*node).name) }
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            ret =
                xmlStrdup(b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            if (*ns).prefix.is_null() { return ret }
            ret =
                xmlStrcat(ret,
                          b":\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            ret = xmlStrcat(ret, (*ns).prefix);
            return ret
        }
        15 | 16 | 17 | 19 | 20 => { return 0 as *mut xmlChar }
        _ => { }
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlTextReaderConstName:
 * @reader:  the xmlTextReaderPtr used
 *
 * The qualified name of the node, equal to Prefix :LocalName.
 *
 * Returns the local name or NULL if not available, the string is
 *         deallocated with the reader.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstName(mut reader: xmlTextReaderPtr)
 -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *const xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    match (*node).type_0 as std::os::raw::c_uint {
        1 | 2 => {
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                return (*node).name
            }
            return xmlDictQLookup((*reader).dict, (*(*node).ns).prefix,
                                  (*node).name)
        }
        3 => {
            return xmlDictLookup((*reader).dict,
                                 b"#text\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 -(1 as std::os::raw::c_int))
        }
        4 => {
            return xmlDictLookup((*reader).dict,
                                 b"#cdata-section\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 -(1 as std::os::raw::c_int))
        }
        6 | 5 => {
            return xmlDictLookup((*reader).dict, (*node).name,
                                 -(1 as std::os::raw::c_int))
        }
        7 => {
            return xmlDictLookup((*reader).dict, (*node).name,
                                 -(1 as std::os::raw::c_int))
        }
        8 => {
            return xmlDictLookup((*reader).dict,
                                 b"#comment\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 -(1 as std::os::raw::c_int))
        }
        9 | 13 | 21 => {
            return xmlDictLookup((*reader).dict,
                                 b"#document\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 -(1 as std::os::raw::c_int))
        }
        11 => {
            return xmlDictLookup((*reader).dict,
                                 b"#document-fragment\x00" as *const u8 as
                                     *const std::os::raw::c_char as *mut xmlChar,
                                 -(1 as std::os::raw::c_int))
        }
        12 => {
            return xmlDictLookup((*reader).dict, (*node).name,
                                 -(1 as std::os::raw::c_int))
        }
        10 | 14 => {
            return xmlDictLookup((*reader).dict, (*node).name,
                                 -(1 as std::os::raw::c_int))
        }
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            if (*ns).prefix.is_null() {
                return xmlDictLookup((*reader).dict,
                                     b"xmlns\x00" as *const u8 as
                                         *const std::os::raw::c_char as *mut xmlChar,
                                     -(1 as std::os::raw::c_int))
            }
            return xmlDictQLookup((*reader).dict,
                                  b"xmlns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  (*ns).prefix)
        }
        15 | 16 | 17 | 19 | 20 => { return 0 as *const xmlChar }
        _ => { }
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlTextReaderPrefix:
 * @reader:  the xmlTextReaderPtr used
 *
 * A shorthand reference to the namespace associated with the node.
 *
 * Returns the prefix or NULL if not available,
 *    if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPrefix(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *mut xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() { return 0 as *mut xmlChar }
        return xmlStrdup(b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar)
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
        return xmlStrdup((*(*node).ns).prefix)
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlTextReaderConstPrefix:
 * @reader:  the xmlTextReaderPtr used
 *
 * A shorthand reference to the namespace associated with the node.
 *
 * Returns the prefix or NULL if not available, the string is deallocated
 *         with the reader.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstPrefix(mut reader:
                                                      xmlTextReaderPtr)
 -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *const xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() { return 0 as *const xmlChar }
        return xmlDictLookup((*reader).dict,
                             b"xmlns\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar, -(1 as std::os::raw::c_int))
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *const xmlChar
    }
    if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
        return xmlDictLookup((*reader).dict, (*(*node).ns).prefix,
                             -(1 as std::os::raw::c_int))
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlTextReaderNamespaceUri:
 * @reader:  the xmlTextReaderPtr used
 *
 * The URI defining the namespace associated with the node.
 *
 * Returns the namespace URI or NULL if not available,
 *    if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNamespaceUri(mut reader:
                                                       xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *mut xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlStrdup(b"http://www.w3.org/2000/xmlns/\x00" as *const u8 as
                             *const std::os::raw::c_char as *mut xmlChar)
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    if !(*node).ns.is_null() { return xmlStrdup((*(*node).ns).href) }
    return 0 as *mut xmlChar;
}
/* *
 * xmlTextReaderConstNamespaceUri:
 * @reader:  the xmlTextReaderPtr used
 *
 * The URI defining the namespace associated with the node.
 *
 * Returns the namespace URI or NULL if not available, the string
 *         will be deallocated with the reader
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstNamespaceUri(mut reader:
                                                            xmlTextReaderPtr)
 -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *const xmlChar
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return xmlDictLookup((*reader).dict,
                             b"http://www.w3.org/2000/xmlns/\x00" as *const u8
                                 as *const std::os::raw::c_char as *mut xmlChar,
                             -(1 as std::os::raw::c_int))
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *const xmlChar
    }
    if !(*node).ns.is_null() {
        return xmlDictLookup((*reader).dict, (*(*node).ns).href,
                             -(1 as std::os::raw::c_int))
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlTextReaderBaseUri:
 * @reader:  the xmlTextReaderPtr used
 *
 * The base URI of the node.
 *
 * Returns the base URI or NULL if not available,
 *    if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderBaseUri(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *mut xmlChar
    }
    return xmlNodeGetBase(0 as *const xmlDoc,
                          (*reader).node as *const xmlNode);
}
/* *
 * xmlTextReaderConstBaseUri:
 * @reader:  the xmlTextReaderPtr used
 *
 * The base URI of the node.
 *
 * Returns the base URI or NULL if not available, the string
 *         will be deallocated with the reader
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstBaseUri(mut reader:
                                                       xmlTextReaderPtr)
 -> *const xmlChar {
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if reader.is_null() || (*reader).node.is_null() {
        return 0 as *const xmlChar
    }
    tmp =
        xmlNodeGetBase(0 as *const xmlDoc, (*reader).node as *const xmlNode);
    if tmp.is_null() { return 0 as *const xmlChar }
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as std::os::raw::c_int));
    xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    return ret;
}
/* *
 * xmlTextReaderDepth:
 * @reader:  the xmlTextReaderPtr used
 *
 * The depth of the node in the tree.
 *
 * Returns the depth or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderDepth(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return 0 as std::os::raw::c_int }
    if !(*reader).curnode.is_null() {
        if (*(*reader).curnode).type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*(*reader).curnode).type_0 as std::os::raw::c_uint ==
                   XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            return (*reader).depth + 1 as std::os::raw::c_int
        }
        return (*reader).depth + 2 as std::os::raw::c_int
    }
    return (*reader).depth;
}
/* *
 * xmlTextReaderHasAttributes:
 * @reader:  the xmlTextReaderPtr used
 *
 * Whether the node has attributes.
 *
 * Returns 1 if true, 0 if false, and -1 in case or error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderHasAttributes(mut reader:
                                                        xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return 0 as std::os::raw::c_int }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (!(*node).properties.is_null() || !(*node).nsDef.is_null()) {
        return 1 as std::os::raw::c_int
    }
    /* TODO: handle the xmlDecl */
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderHasValue:
 * @reader:  the xmlTextReaderPtr used
 *
 * Whether the node can have a text value.
 *
 * Returns 1 if true, 0 if false, and -1 in case or error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderHasValue(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return 0 as std::os::raw::c_int }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    match (*node).type_0 as std::os::raw::c_uint {
        2 | 3 | 4 | 7 | 8 | 18 => { return 1 as std::os::raw::c_int }
        _ => { }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderValue:
 * @reader:  the xmlTextReaderPtr used
 *
 * Provides the text value of the node if present
 *
 * Returns the string or NULL if not available. The result must be deallocated
 *     with xmlFree()
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValue(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return 0 as *mut xmlChar }
    if (*reader).node.is_null() { return 0 as *mut xmlChar }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    match (*node).type_0 as std::os::raw::c_uint {
        18 => { return xmlStrdup((*(node as xmlNsPtr)).href) }
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            if !(*attr).parent.is_null() {
                return xmlNodeListGetString((*(*attr).parent).doc,
                                            (*attr).children,
                                            1 as std::os::raw::c_int)
            } else {
                return xmlNodeListGetString(0 as xmlDocPtr, (*attr).children,
                                            1 as std::os::raw::c_int)
            }
        }
        3 | 4 | 7 | 8 => {
            if !(*node).content.is_null() {
                return xmlStrdup((*node).content)
            }
        }
        _ => { }
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlTextReaderConstValue:
 * @reader:  the xmlTextReaderPtr used
 *
 * Provides the text value of the node if present
 *
 * Returns the string or NULL if not available. The result will be
 *     deallocated on the next Read() operation.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstValue(mut reader: xmlTextReaderPtr)
 -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return 0 as *const xmlChar }
    if (*reader).node.is_null() { return 0 as *const xmlChar }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    match (*node).type_0 as std::os::raw::c_uint {
        18 => { return (*(node as xmlNsPtr)).href }
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            let mut ret: *const xmlChar = 0 as *const xmlChar;
            if !(*attr).children.is_null() &&
                   (*(*attr).children).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   (*(*attr).children).next.is_null() {
                return (*(*attr).children).content
            } else {
                if (*reader).buffer.is_null() {
                    (*reader).buffer =
                        xmlBufCreateSize(100 as std::os::raw::c_int as size_t);
                    if (*reader).buffer.is_null() {
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"xmlTextReaderSetup : malloc failed\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char);
                        return 0 as *const xmlChar
                    }
                    xmlBufSetAllocationScheme((*reader).buffer,
                                              XML_BUFFER_ALLOC_BOUNDED);
                } else { xmlBufEmpty((*reader).buffer); }
                xmlBufGetNodeContent((*reader).buffer,
                                     node as *const xmlNode);
                ret = xmlBufContent((*reader).buffer as *const xmlBuf);
                if ret.is_null() {
                    /* error on the buffer best to reallocate */
                    xmlBufFree((*reader).buffer);
                    (*reader).buffer =
                        xmlBufCreateSize(100 as std::os::raw::c_int as size_t);
                    xmlBufSetAllocationScheme((*reader).buffer,
                                              XML_BUFFER_ALLOC_BOUNDED);
                    ret =
                        b"\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut xmlChar
                }
                return ret
            }
        }
        3 | 4 | 7 | 8 => { return (*node).content }
        _ => { }
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlTextReaderIsDefault:
 * @reader:  the xmlTextReaderPtr used
 *
 * Whether an Attribute  node was generated from the default value
 * defined in the DTD or schema.
 *
 * Returns 0 if not defaulted, 1 if defaulted, and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsDefault(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderQuoteChar:
 * @reader:  the xmlTextReaderPtr used
 *
 * The quotation mark character used to enclose the value of an attribute.
 *
 * Returns " or ' and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderQuoteChar(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    /* TODO maybe lookup the attribute value for " first */
    return '\"' as i32;
}
/* *
 * xmlTextReaderXmlLang:
 * @reader:  the xmlTextReaderPtr used
 *
 * The xml:lang scope within which the node resides.
 *
 * Returns the xml:lang value or NULL if none exists.,
 *    if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderXmlLang(mut reader: xmlTextReaderPtr)
 -> *mut xmlChar {
    if reader.is_null() { return 0 as *mut xmlChar }
    if (*reader).node.is_null() { return 0 as *mut xmlChar }
    return xmlNodeGetLang((*reader).node as *const xmlNode);
}
/* *
 * xmlTextReaderConstXmlLang:
 * @reader:  the xmlTextReaderPtr used
 *
 * The xml:lang scope within which the node resides.
 *
 * Returns the xml:lang value or NULL if none exists.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstXmlLang(mut reader:
                                                       xmlTextReaderPtr)
 -> *const xmlChar {
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if reader.is_null() { return 0 as *const xmlChar }
    if (*reader).node.is_null() { return 0 as *const xmlChar }
    tmp = xmlNodeGetLang((*reader).node as *const xmlNode);
    if tmp.is_null() { return 0 as *const xmlChar }
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as std::os::raw::c_int));
    xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    return ret;
}
/* *
 * xmlTextReaderConstString:
 * @reader:  the xmlTextReaderPtr used
 * @str:  the string to intern.
 *
 * Get an interned string from the reader, allows for example to
 * speedup string name comparisons
 *
 * Returns an interned copy of the string or NULL in case of error. The
 *         string will be deallocated with the reader.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstString(mut reader:
                                                      xmlTextReaderPtr,
                                                  mut str: *const xmlChar)
 -> *const xmlChar {
    if reader.is_null() { return 0 as *const xmlChar }
    return xmlDictLookup((*reader).dict, str, -(1 as std::os::raw::c_int));
}
/* *
 * xmlTextReaderNormalization:
 * @reader:  the xmlTextReaderPtr used
 *
 * The value indicating whether to normalize white space and attribute values.
 * Since attribute value and end of line normalizations are a MUST in the XML
 * specification only the value true is accepted. The broken bahaviour of
 * accepting out of range character entities like &#0; is of course not
 * supported either.
 *
 * Returns 1 or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNormalization(mut reader:
                                                        xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    return 1 as std::os::raw::c_int;
}
/* ***********************************************************************
 *									*
 *			Extensions to the base APIs			*
 *									*
 ************************************************************************/
/* *
 * xmlTextReaderSetParserProp:
 * @reader:  the xmlTextReaderPtr used
 * @prop:  the xmlParserProperties to set
 * @value:  usually 0 or 1 to (de)activate it
 *
 * Change the parser processing behaviour by changing some of its internal
 * properties. Note that some properties can only be changed before any
 * read has been done.
 *
 * Returns 0 if the call was successful, or -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetParserProp(mut reader:
                                                        xmlTextReaderPtr,
                                                    mut prop: std::os::raw::c_int,
                                                    mut value: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    ctxt = (*reader).ctxt;
    match p as std::os::raw::c_uint {
        1 => {
            if value != 0 as std::os::raw::c_int {
                if (*ctxt).loadsubset == 0 as std::os::raw::c_int {
                    if (*reader).mode !=
                           XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int {
                        return -(1 as std::os::raw::c_int)
                    }
                    (*ctxt).loadsubset = 2 as std::os::raw::c_int
                }
            } else { (*ctxt).loadsubset = 0 as std::os::raw::c_int }
            return 0 as std::os::raw::c_int
        }
        2 => {
            if value != 0 as std::os::raw::c_int {
                (*ctxt).loadsubset |= 4 as std::os::raw::c_int
            } else if (*ctxt).loadsubset & 4 as std::os::raw::c_int != 0 {
                (*ctxt).loadsubset -= 4 as std::os::raw::c_int
            }
            return 0 as std::os::raw::c_int
        }
        3 => {
            if value != 0 as std::os::raw::c_int {
                (*ctxt).validate = 1 as std::os::raw::c_int;
                (*reader).validate = XML_TEXTREADER_VALIDATE_DTD
            } else { (*ctxt).validate = 0 as std::os::raw::c_int }
            return 0 as std::os::raw::c_int
        }
        4 => {
            if value != 0 as std::os::raw::c_int {
                (*ctxt).replaceEntities = 1 as std::os::raw::c_int
            } else { (*ctxt).replaceEntities = 0 as std::os::raw::c_int }
            return 0 as std::os::raw::c_int
        }
        _ => { }
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlTextReaderGetParserProp:
 * @reader:  the xmlTextReaderPtr used
 * @prop:  the xmlParserProperties to get
 *
 * Read the parser internal property.
 *
 * Returns the value, usually 0 or 1, or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserProp(mut reader:
                                                        xmlTextReaderPtr,
                                                    mut prop: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    ctxt = (*reader).ctxt;
    match p as std::os::raw::c_uint {
        1 => {
            if (*ctxt).loadsubset != 0 as std::os::raw::c_int ||
                   (*ctxt).validate != 0 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int
            }
            return 0 as std::os::raw::c_int
        }
        2 => {
            if (*ctxt).loadsubset & 4 as std::os::raw::c_int != 0 {
                return 1 as std::os::raw::c_int
            }
            return 0 as std::os::raw::c_int
        }
        3 => { return (*reader).validate as std::os::raw::c_int }
        4 => { return (*ctxt).replaceEntities }
        _ => { }
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlTextReaderGetParserLineNumber:
 * @reader: the user data (XML reader context)
 *
 * Provide the line number of the current parsing point.
 *
 * Returns an int or 0 if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserLineNumber(mut reader:
                                                              xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() || (*reader).ctxt.is_null() ||
           (*(*reader).ctxt).input.is_null() {
        return 0 as std::os::raw::c_int
    }
    return (*(*(*reader).ctxt).input).line;
}
/* *
 * xmlTextReaderGetParserColumnNumber:
 * @reader: the user data (XML reader context)
 *
 * Provide the column number of the current parsing point.
 *
 * Returns an int or 0 if not available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserColumnNumber(mut reader:
                                                                xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() || (*reader).ctxt.is_null() ||
           (*(*reader).ctxt).input.is_null() {
        return 0 as std::os::raw::c_int
    }
    return (*(*(*reader).ctxt).input).col;
}
/* *
 * xmlTextReaderCurrentNode:
 * @reader:  the xmlTextReaderPtr used
 *
 * Hacking interface allowing to get the xmlNodePtr correponding to the
 * current node being accessed by the xmlTextReader. This is dangerous
 * because the underlying node may be destroyed on the next Reads.
 *
 * Returns the xmlNodePtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderCurrentNode(mut reader:
                                                      xmlTextReaderPtr)
 -> xmlNodePtr {
    if reader.is_null() { return 0 as xmlNodePtr }
    if !(*reader).curnode.is_null() { return (*reader).curnode }
    return (*reader).node;
}
/* *
 * xmlTextReaderPreserve:
 * @reader:  the xmlTextReaderPtr used
 *
 * This tells the XML Reader to preserve the current node.
 * The caller must also use xmlTextReaderCurrentDoc() to
 * keep an handle on the resulting document once parsing has finished
 *
 * Returns the xmlNodePtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPreserve(mut reader: xmlTextReaderPtr)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return 0 as xmlNodePtr }
    if !(*reader).curnode.is_null() {
        cur = (*reader).curnode
    } else { cur = (*reader).node }
    if cur.is_null() { return 0 as xmlNodePtr }
    if (*cur).type_0 as std::os::raw::c_uint !=
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           (*cur).type_0 as std::os::raw::c_uint !=
               XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        (*cur).extra =
            ((*cur).extra as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as
                std::os::raw::c_ushort;
        (*cur).extra =
            ((*cur).extra as std::os::raw::c_int | 0x4 as std::os::raw::c_int) as
                std::os::raw::c_ushort
    }
    (*reader).preserves += 1;
    parent = (*cur).parent;
    while !parent.is_null() {
        if (*parent).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            (*parent).extra =
                ((*parent).extra as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as
                    std::os::raw::c_ushort
        }
        parent = (*parent).parent
    }
    return cur;
}
/* *
 * xmlTextReaderPreservePattern:
 * @reader:  the xmlTextReaderPtr used
 * @pattern:  an XPath subset pattern
 * @namespaces: the prefix definitions, array of [URI, prefix] or NULL
 *
 * This tells the XML Reader to preserve all nodes matched by the
 * pattern. The caller must also use xmlTextReaderCurrentDoc() to
 * keep an handle on the resulting document once parsing has finished
 *
 * Returns a non-negative number in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPreservePattern(mut reader:
                                                          xmlTextReaderPtr,
                                                      mut pattern:
                                                          *const xmlChar,
                                                      mut namespaces:
                                                          *mut *const xmlChar)
 -> std::os::raw::c_int {
    let mut comp: xmlPatternPtr = 0 as *mut xmlPattern;
    if reader.is_null() || pattern.is_null() { return -(1 as std::os::raw::c_int) }
    comp =
        xmlPatterncompile(pattern, (*reader).dict, 0 as std::os::raw::c_int,
                          namespaces);
    if comp.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).patternMax <= 0 as std::os::raw::c_int {
        (*reader).patternMax = 4 as std::os::raw::c_int;
        (*reader).patternTab =
            xmlMalloc.expect("non-null function pointer")(((*reader).patternMax
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlPatternPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlPatternPtr;
        if (*reader).patternTab.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"xmlMalloc failed !\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if (*reader).patternNr >= (*reader).patternMax {
        let mut tmp: *mut xmlPatternPtr = 0 as *mut xmlPatternPtr;
        (*reader).patternMax *= 2 as std::os::raw::c_int;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*reader).patternTab
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           ((*reader).patternMax
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlPatternPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlPatternPtr;
        if tmp.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"xmlRealloc failed !\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            (*reader).patternMax /= 2 as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        (*reader).patternTab = tmp
    }
    let ref mut fresh3 =
        *(*reader).patternTab.offset((*reader).patternNr as isize);
    *fresh3 = comp;
    let fresh4 = (*reader).patternNr;
    (*reader).patternNr = (*reader).patternNr + 1;
    return fresh4;
}
/* *
 * xmlTextReaderCurrentDoc:
 * @reader:  the xmlTextReaderPtr used
 *
 * Hacking interface allowing to get the xmlDocPtr correponding to the
 * current document being accessed by the xmlTextReader.
 * NOTE: as a result of this call, the reader will not destroy the
 *       associated XML document and calling xmlFreeDoc() on the result
 *       is needed once the reader parsing has finished.
 *
 * Returns the xmlDocPtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderCurrentDoc(mut reader: xmlTextReaderPtr)
 -> xmlDocPtr {
    if reader.is_null() { return 0 as xmlDocPtr }
    if !(*reader).doc.is_null() { return (*reader).doc }
    if (*reader).ctxt.is_null() || (*(*reader).ctxt).myDoc.is_null() {
        return 0 as xmlDocPtr
    }
    (*reader).preserve = 1 as std::os::raw::c_int;
    return (*(*reader).ctxt).myDoc;
}
unsafe extern "C" fn xmlTextReaderValidityStructuredRelay(mut userData:
                                                              *mut std::os::raw::c_void,
                                                          mut error:
                                                              xmlErrorPtr) {
    let mut reader: xmlTextReaderPtr = userData as xmlTextReaderPtr;
    if (*reader).sErrorFunc.is_some() {
        (*reader).sErrorFunc.expect("non-null function pointer")((*reader).errorFuncArg,
                                                                 error);
    } else {
        xmlTextReaderStructuredError(reader as *mut std::os::raw::c_void, error);
    };
}
/* *
 * xmlTextReaderRelaxNGSetSchema:
 * @reader:  the xmlTextReaderPtr used
 * @schema:  a precompiled RelaxNG schema
 *
 * Use RelaxNG to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * if @schema is NULL, then RelaxNG validation is desactivated.
 @ The @schema should not be freed until the reader is deallocated
 * or its use has been deactivated.
 *
 * Returns 0 in case the RelaxNG validation could be (des)activated and
 *         -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGSetSchema(mut reader:
                                                           xmlTextReaderPtr,
                                                       mut schema:
                                                           xmlRelaxNGPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if schema.is_null() {
        if !(*reader).rngSchemas.is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            (*reader).rngSchemas = 0 as xmlRelaxNGPtr
        }
        if !(*reader).rngValidCtxt.is_null() {
            if (*reader).rngPreserveCtxt == 0 {
                xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
            }
            (*reader).rngValidCtxt = 0 as xmlRelaxNGValidCtxtPtr
        }
        (*reader).rngPreserveCtxt = 0 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = 0 as xmlRelaxNGPtr
    }
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = 0 as xmlRelaxNGValidCtxtPtr
    }
    (*reader).rngPreserveCtxt = 0 as std::os::raw::c_int;
    (*reader).rngValidCtxt = xmlRelaxNGNewValidCtxt(schema);
    if (*reader).rngValidCtxt.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).errorFunc.is_some() {
        xmlRelaxNGSetValidErrors((*reader).rngValidCtxt,
                                 Some(xmlTextReaderValidityErrorRelay as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void,
                                                               _:
                                                                   *const std::os::raw::c_char,
                                                               _: ...) -> ()),
                                 Some(xmlTextReaderValidityWarningRelay as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void,
                                                               _:
                                                                   *const std::os::raw::c_char,
                                                               _: ...) -> ()),
                                 reader as *mut std::os::raw::c_void);
    }
    if (*reader).sErrorFunc.is_some() {
        xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt,
                                           Some(xmlTextReaderValidityStructuredRelay
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut std::os::raw::c_void,
                                                                         _:
                                                                             xmlErrorPtr)
                                                        -> ()),
                                           reader as *mut std::os::raw::c_void);
    }
    (*reader).rngValidErrors = 0 as std::os::raw::c_int;
    (*reader).rngFullNode = 0 as xmlNodePtr;
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderLocator:
 * @ctx: the xmlTextReaderPtr used
 * @file: returned file information
 * @line: returned line information
 *
 * Internal locator function for the readers
 *
 * Returns 0 in case the Schema validation could be (des)activated and
 *         -1 in case of error.
 */
unsafe extern "C" fn xmlTextReaderLocator(mut ctx: *mut std::os::raw::c_void,
                                          mut file: *mut *const std::os::raw::c_char,
                                          mut line: *mut std::os::raw::c_ulong)
 -> std::os::raw::c_int {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    if ctx.is_null() || file.is_null() && line.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if !file.is_null() { *file = 0 as *const std::os::raw::c_char }
    if !line.is_null() { *line = 0 as std::os::raw::c_int as std::os::raw::c_ulong }
    reader = ctx as xmlTextReaderPtr;
    if !(*reader).ctxt.is_null() && !(*(*reader).ctxt).input.is_null() {
        if !file.is_null() { *file = (*(*(*reader).ctxt).input).filename }
        if !line.is_null() {
            *line = (*(*(*reader).ctxt).input).line as std::os::raw::c_ulong
        }
        return 0 as std::os::raw::c_int
    }
    if !(*reader).node.is_null() {
        let mut res: std::os::raw::c_long = 0;
        let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
        if !line.is_null() {
            res = xmlGetLineNo((*reader).node as *const xmlNode);
            if res > 0 as std::os::raw::c_int as std::os::raw::c_long {
                *line = res as std::os::raw::c_ulong
            } else { ret = -(1 as std::os::raw::c_int) }
        }
        if !file.is_null() {
            let mut doc: xmlDocPtr = (*(*reader).node).doc;
            if !doc.is_null() && !(*doc).URL.is_null() {
                *file = (*doc).URL as *const std::os::raw::c_char
            } else { ret = -(1 as std::os::raw::c_int) }
        }
        return ret
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlTextReaderSetSchema:
 * @reader:  the xmlTextReaderPtr used
 * @schema:  a precompiled Schema schema
 *
 * Use XSD Schema to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * if @schema is NULL, then Schema validation is desactivated.
 @ The @schema should not be freed until the reader is deallocated
 * or its use has been deactivated.
 *
 * Returns 0 in case the Schema validation could be (des)activated and
 *         -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetSchema(mut reader: xmlTextReaderPtr,
                                                mut schema: xmlSchemaPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if schema.is_null() {
        if !(*reader).xsdPlug.is_null() {
            xmlSchemaSAXUnplug((*reader).xsdPlug);
            (*reader).xsdPlug = 0 as xmlSchemaSAXPlugPtr
        }
        if !(*reader).xsdValidCtxt.is_null() {
            if (*reader).xsdPreserveCtxt == 0 {
                xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
            }
            (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr
        }
        (*reader).xsdPreserveCtxt = 0 as std::os::raw::c_int;
        if !(*reader).xsdSchemas.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = 0 as xmlSchemaPtr
        }
        return 0 as std::os::raw::c_int
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if !(*reader).xsdPlug.is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        (*reader).xsdPlug = 0 as xmlSchemaSAXPlugPtr
    }
    if !(*reader).xsdValidCtxt.is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr
    }
    (*reader).xsdPreserveCtxt = 0 as std::os::raw::c_int;
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = 0 as xmlSchemaPtr
    }
    (*reader).xsdValidCtxt = xmlSchemaNewValidCtxt(schema);
    if (*reader).xsdValidCtxt.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = 0 as xmlSchemaPtr;
        return -(1 as std::os::raw::c_int)
    }
    (*reader).xsdPlug =
        xmlSchemaSAXPlug((*reader).xsdValidCtxt, &mut (*(*reader).ctxt).sax,
                         &mut (*(*reader).ctxt).userData);
    if (*reader).xsdPlug.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = 0 as xmlSchemaPtr;
        xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr;
        return -(1 as std::os::raw::c_int)
    }
    xmlSchemaValidateSetLocator((*reader).xsdValidCtxt,
                                Some(xmlTextReaderLocator as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *mut *const std::os::raw::c_char,
                                                              _:
                                                                  *mut std::os::raw::c_ulong)
                                             -> std::os::raw::c_int),
                                reader as *mut std::os::raw::c_void);
    if (*reader).errorFunc.is_some() {
        xmlSchemaSetValidErrors((*reader).xsdValidCtxt,
                                Some(xmlTextReaderValidityErrorRelay as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *const std::os::raw::c_char,
                                                              _: ...) -> ()),
                                Some(xmlTextReaderValidityWarningRelay as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *const std::os::raw::c_char,
                                                              _: ...) -> ()),
                                reader as *mut std::os::raw::c_void);
    }
    if (*reader).sErrorFunc.is_some() {
        xmlSchemaSetValidStructuredErrors((*reader).xsdValidCtxt,
                                          Some(xmlTextReaderValidityStructuredRelay
                                                   as
                                                   unsafe extern "C" fn(_:
                                                                            *mut std::os::raw::c_void,
                                                                        _:
                                                                            xmlErrorPtr)
                                                       -> ()),
                                          reader as *mut std::os::raw::c_void);
    }
    (*reader).xsdValidErrors = 0 as std::os::raw::c_int;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderRelaxNGValidateInternal:
 * @reader:  the xmlTextReaderPtr used
 * @rng:  the path to a RelaxNG schema or NULL
 * @ctxt: the RelaxNG schema validation context or NULL
 * @options: options (not yet used)
 *
 * Use RelaxNG to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * If both @rng and @ctxt are NULL, then RelaxNG validation is deactivated.
 *
 * Returns 0 in case the RelaxNG validation could be (de)activated and
 *	   -1 in case of error.
 */
unsafe extern "C" fn xmlTextReaderRelaxNGValidateInternal(mut reader:
                                                              xmlTextReaderPtr,
                                                          mut rng:
                                                              *const std::os::raw::c_char,
                                                          mut ctxt:
                                                              xmlRelaxNGValidCtxtPtr,
                                                          mut options:
                                                              std::os::raw::c_int)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if !rng.is_null() && !ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if (!rng.is_null() || !ctxt.is_null()) &&
           ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int ||
                (*reader).ctxt.is_null()) {
        return -(1 as std::os::raw::c_int)
    }
    /* Cleanup previous validation stuff. */
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = 0 as xmlRelaxNGValidCtxtPtr
    }
    (*reader).rngPreserveCtxt = 0 as std::os::raw::c_int;
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = 0 as xmlRelaxNGPtr
    }
    if rng.is_null() && ctxt.is_null() {
        /* We just want to deactivate the validation, so get out. */
        return 0 as std::os::raw::c_int
    }
    if !rng.is_null() {
        let mut pctxt: xmlRelaxNGParserCtxtPtr =
            0 as *mut xmlRelaxNGParserCtxt;
        /* Parse the schema and create validation environment. */
        pctxt = xmlRelaxNGNewParserCtxt(rng);
        if (*reader).errorFunc.is_some() {
            xmlRelaxNGSetParserErrors(pctxt,
                                      Some(xmlTextReaderValidityErrorRelay as
                                               unsafe extern "C" fn(_:
                                                                        *mut std::os::raw::c_void,
                                                                    _:
                                                                        *const std::os::raw::c_char,
                                                                    _: ...)
                                                   -> ()),
                                      Some(xmlTextReaderValidityWarningRelay
                                               as
                                               unsafe extern "C" fn(_:
                                                                        *mut std::os::raw::c_void,
                                                                    _:
                                                                        *const std::os::raw::c_char,
                                                                    _: ...)
                                                   -> ()),
                                      reader as *mut std::os::raw::c_void);
        }
        if (*reader).sErrorFunc.is_some() {
            xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt,
                                               Some(xmlTextReaderValidityStructuredRelay
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut std::os::raw::c_void,
                                                                             _:
                                                                                 xmlErrorPtr)
                                                            -> ()),
                                               reader as *mut std::os::raw::c_void);
        }
        (*reader).rngSchemas = xmlRelaxNGParse(pctxt);
        xmlRelaxNGFreeParserCtxt(pctxt);
        if (*reader).rngSchemas.is_null() { return -(1 as std::os::raw::c_int) }
        (*reader).rngValidCtxt = xmlRelaxNGNewValidCtxt((*reader).rngSchemas);
        if (*reader).rngValidCtxt.is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            (*reader).rngSchemas = 0 as xmlRelaxNGPtr;
            return -(1 as std::os::raw::c_int)
        }
    } else {
        /* Use the given validation context. */
        (*reader).rngValidCtxt = ctxt;
        (*reader).rngPreserveCtxt = 1 as std::os::raw::c_int
    }
    /*
    * Redirect the validation context's error channels to use
    * the reader channels.
    * TODO: In case the user provides the validation context we
    *	could make this redirection optional.
    */
    if (*reader).errorFunc.is_some() {
        xmlRelaxNGSetValidErrors((*reader).rngValidCtxt,
                                 Some(xmlTextReaderValidityErrorRelay as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void,
                                                               _:
                                                                   *const std::os::raw::c_char,
                                                               _: ...) -> ()),
                                 Some(xmlTextReaderValidityWarningRelay as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void,
                                                               _:
                                                                   *const std::os::raw::c_char,
                                                               _: ...) -> ()),
                                 reader as *mut std::os::raw::c_void);
    }
    if (*reader).sErrorFunc.is_some() {
        xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt,
                                           Some(xmlTextReaderValidityStructuredRelay
                                                    as
                                                    unsafe extern "C" fn(_:
                                                                             *mut std::os::raw::c_void,
                                                                         _:
                                                                             xmlErrorPtr)
                                                        -> ()),
                                           reader as *mut std::os::raw::c_void);
    }
    (*reader).rngValidErrors = 0 as std::os::raw::c_int;
    (*reader).rngFullNode = 0 as xmlNodePtr;
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderSchemaValidateInternal:
 * @reader:  the xmlTextReaderPtr used
 * @xsd:  the path to a W3C XSD schema or NULL
 * @ctxt: the XML Schema validation context or NULL
 * @options: options (not used yet)
 *
 * Validate the document as it is processed using XML Schema.
 * Activation is only possible before the first Read().
 * If both @xsd and @ctxt are NULL then XML Schema validation is deactivated.
 *
 * Returns 0 in case the schemas validation could be (de)activated and
 *         -1 in case of error.
 */
unsafe extern "C" fn xmlTextReaderSchemaValidateInternal(mut reader:
                                                             xmlTextReaderPtr,
                                                         mut xsd:
                                                             *const std::os::raw::c_char,
                                                         mut ctxt:
                                                             xmlSchemaValidCtxtPtr,
                                                         mut options:
                                                             std::os::raw::c_int)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if !xsd.is_null() && !ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if (!xsd.is_null() || !ctxt.is_null()) &&
           ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int ||
                (*reader).ctxt.is_null()) {
        return -(1 as std::os::raw::c_int)
    }
    /* Cleanup previous validation stuff. */
    if !(*reader).xsdPlug.is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        (*reader).xsdPlug = 0 as xmlSchemaSAXPlugPtr
    }
    if !(*reader).xsdValidCtxt.is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr
    }
    (*reader).xsdPreserveCtxt = 0 as std::os::raw::c_int;
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = 0 as xmlSchemaPtr
    }
    if xsd.is_null() && ctxt.is_null() {
        /* We just want to deactivate the validation, so get out. */
        return 0 as std::os::raw::c_int
    }
    if !xsd.is_null() {
        let mut pctxt: xmlSchemaParserCtxtPtr = 0 as *mut xmlSchemaParserCtxt;
        /* Parse the schema and create validation environment. */
        pctxt = xmlSchemaNewParserCtxt(xsd);
        if (*reader).errorFunc.is_some() {
            xmlSchemaSetParserErrors(pctxt,
                                     Some(xmlTextReaderValidityErrorRelay as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                     Some(xmlTextReaderValidityWarningRelay as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                     reader as *mut std::os::raw::c_void);
        }
        (*reader).xsdSchemas = xmlSchemaParse(pctxt);
        xmlSchemaFreeParserCtxt(pctxt);
        if (*reader).xsdSchemas.is_null() { return -(1 as std::os::raw::c_int) }
        (*reader).xsdValidCtxt = xmlSchemaNewValidCtxt((*reader).xsdSchemas);
        if (*reader).xsdValidCtxt.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = 0 as xmlSchemaPtr;
            return -(1 as std::os::raw::c_int)
        }
        (*reader).xsdPlug =
            xmlSchemaSAXPlug((*reader).xsdValidCtxt,
                             &mut (*(*reader).ctxt).sax,
                             &mut (*(*reader).ctxt).userData);
        if (*reader).xsdPlug.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = 0 as xmlSchemaPtr;
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
            (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr;
            return -(1 as std::os::raw::c_int)
        }
    } else {
        /* Use the given validation context. */
        (*reader).xsdValidCtxt = ctxt;
        (*reader).xsdPreserveCtxt = 1 as std::os::raw::c_int;
        (*reader).xsdPlug =
            xmlSchemaSAXPlug((*reader).xsdValidCtxt,
                             &mut (*(*reader).ctxt).sax,
                             &mut (*(*reader).ctxt).userData);
        if (*reader).xsdPlug.is_null() {
            (*reader).xsdValidCtxt = 0 as xmlSchemaValidCtxtPtr;
            (*reader).xsdPreserveCtxt = 0 as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
    }
    xmlSchemaValidateSetLocator((*reader).xsdValidCtxt,
                                Some(xmlTextReaderLocator as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *mut *const std::os::raw::c_char,
                                                              _:
                                                                  *mut std::os::raw::c_ulong)
                                             -> std::os::raw::c_int),
                                reader as *mut std::os::raw::c_void);
    /*
    * Redirect the validation context's error channels to use
    * the reader channels.
    * TODO: In case the user provides the validation context we
    *   could make this redirection optional.
    */
    if (*reader).errorFunc.is_some() {
        xmlSchemaSetValidErrors((*reader).xsdValidCtxt,
                                Some(xmlTextReaderValidityErrorRelay as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *const std::os::raw::c_char,
                                                              _: ...) -> ()),
                                Some(xmlTextReaderValidityWarningRelay as
                                         unsafe extern "C" fn(_:
                                                                  *mut std::os::raw::c_void,
                                                              _:
                                                                  *const std::os::raw::c_char,
                                                              _: ...) -> ()),
                                reader as *mut std::os::raw::c_void);
    }
    if (*reader).sErrorFunc.is_some() {
        xmlSchemaSetValidStructuredErrors((*reader).xsdValidCtxt,
                                          Some(xmlTextReaderValidityStructuredRelay
                                                   as
                                                   unsafe extern "C" fn(_:
                                                                            *mut std::os::raw::c_void,
                                                                        _:
                                                                            xmlErrorPtr)
                                                       -> ()),
                                          reader as *mut std::os::raw::c_void);
    }
    (*reader).xsdValidErrors = 0 as std::os::raw::c_int;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderSchemaValidateCtxt:
 * @reader:  the xmlTextReaderPtr used
 * @ctxt: the XML Schema validation context or NULL
 * @options: options (not used yet)
 *
 * Use W3C XSD schema context to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * If @ctxt is NULL, then XML Schema validation is deactivated.
 *
 * Returns 0 in case the schemas validation could be (de)activated and
 *         -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSchemaValidateCtxt(mut reader:
                                                             xmlTextReaderPtr,
                                                         mut ctxt:
                                                             xmlSchemaValidCtxtPtr,
                                                         mut options:
                                                             std::os::raw::c_int)
 -> std::os::raw::c_int {
    return xmlTextReaderSchemaValidateInternal(reader,
                                               0 as *const std::os::raw::c_char, ctxt,
                                               options);
}
/* *
 * xmlTextReaderSchemaValidate:
 * @reader:  the xmlTextReaderPtr used
 * @xsd:  the path to a W3C XSD schema or NULL
 *
 * Use W3C XSD schema to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * If @xsd is NULL, then XML Schema validation is deactivated.
 *
 * Returns 0 in case the schemas validation could be (de)activated and
 *         -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSchemaValidate(mut reader:
                                                         xmlTextReaderPtr,
                                                     mut xsd:
                                                         *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return xmlTextReaderSchemaValidateInternal(reader, xsd,
                                               0 as xmlSchemaValidCtxtPtr,
                                               0 as std::os::raw::c_int);
}
/* *
 * xmlTextReaderRelaxNGValidateCtxt:
 * @reader:  the xmlTextReaderPtr used
 * @ctxt: the RelaxNG schema validation context or NULL
 * @options: options (not used yet)
 *
 * Use RelaxNG schema context to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * If @ctxt is NULL, then RelaxNG schema validation is deactivated.
 *
 * Returns 0 in case the schemas validation could be (de)activated and
 *         -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGValidateCtxt(mut reader:
                                                              xmlTextReaderPtr,
                                                          mut ctxt:
                                                              xmlRelaxNGValidCtxtPtr,
                                                          mut options:
                                                              std::os::raw::c_int)
 -> std::os::raw::c_int {
    return xmlTextReaderRelaxNGValidateInternal(reader,
                                                0 as *const std::os::raw::c_char,
                                                ctxt, options);
}
/* *
 * xmlTextReaderRelaxNGValidate:
 * @reader:  the xmlTextReaderPtr used
 * @rng:  the path to a RelaxNG schema or NULL
 *
 * Use RelaxNG schema to validate the document as it is processed.
 * Activation is only possible before the first Read().
 * If @rng is NULL, then RelaxNG schema validation is deactivated.
 *
 * Returns 0 in case the schemas validation could be (de)activated and
 *         -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGValidate(mut reader:
                                                          xmlTextReaderPtr,
                                                      mut rng:
                                                          *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return xmlTextReaderRelaxNGValidateInternal(reader, rng,
                                                0 as xmlRelaxNGValidCtxtPtr,
                                                0 as std::os::raw::c_int);
}
/* *
 * xmlTextReaderIsNamespaceDecl:
 * @reader: the xmlTextReaderPtr used
 *
 * Determine whether the current node is a namespace declaration
 * rather than a regular attribute.
 *
 * Returns 1 if the current node is a namespace declaration, 0 if it
 * is a regular attribute or other type of node, or -1 in case of
 * error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsNamespaceDecl(mut reader:
                                                          xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).node.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode
    } else { node = (*reader).node }
    if XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ==
           (*node).type_0 as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    } else { return 0 as std::os::raw::c_int };
}
/* *
 * xmlTextReaderConstXmlVersion:
 * @reader:  the xmlTextReaderPtr used
 *
 * Determine the XML version of the document being read.
 *
 * Returns a string containing the XML version of the document or NULL
 * in case of error.  The string is deallocated with the reader.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstXmlVersion(mut reader:
                                                          xmlTextReaderPtr)
 -> *const xmlChar {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if reader.is_null() { return 0 as *const xmlChar }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc
    } else if !(*reader).ctxt.is_null() { doc = (*(*reader).ctxt).myDoc }
    if doc.is_null() { return 0 as *const xmlChar }
    if (*doc).version.is_null() {
        return 0 as *const xmlChar
    } else {
        return xmlDictLookup((*reader).dict, (*doc).version,
                             -(1 as std::os::raw::c_int))
    };
}
/* *
 * xmlTextReaderStandalone:
 * @reader:  the xmlTextReaderPtr used
 *
 * Determine the standalone status of the document being read.
 *
 * Returns 1 if the document was declared to be standalone, 0 if it
 * was declared to be not standalone, or -1 if the document did not
 * specify its standalone status or in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderStandalone(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc
    } else if !(*reader).ctxt.is_null() { doc = (*(*reader).ctxt).myDoc }
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    return (*doc).standalone;
}
/* ***********************************************************************
 *									*
 *			Error Handling Extensions                       *
 *									*
 ************************************************************************/
/* helper to build a xmlMalloc'ed string from a format and va_list */
// char *
// xmlTextReaderBuildMessage(const char *msg, va_list ap) {
//     int size = 0;
//     int chars;
//     char *larger;
//     char *str = NULL;
//     va_list aq;
//     while (1) {
//         VA_COPY(aq, ap);
//         chars = vsnprintf(str, size, msg, aq);
//         va_end(aq);
//         if (chars < 0) {
// 	    xmlGenericError(xmlGenericErrorContext, "vsnprintf failed !\n");
// 	    if (str)
// 		xmlFree(str);
// 	    return NULL;
// 	}
// 	if ((chars < size) || (size == MAX_ERR_MSG_SIZE))
//             break;
//         if (chars < MAX_ERR_MSG_SIZE)
// 	size = chars + 1;
// 	else
// 		size = MAX_ERR_MSG_SIZE;
//         if ((larger = (char *) xmlRealloc(str, size)) == NULL) {
// 	    xmlGenericError(xmlGenericErrorContext, "xmlRealloc failed !\n");
// 	    if (str)
//                 xmlFree(str);
//             return NULL;
//         }
//         str = larger;
//     }
//     return str;
// }
/* *
 * xmlTextReaderLocatorLineNumber:
 * @locator: the xmlTextReaderLocatorPtr used
 *
 * Obtain the line number for the given locator.
 *
 * Returns the line number or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocatorLineNumber(mut locator:
                                                            xmlTextReaderLocatorPtr)
 -> std::os::raw::c_int {
    /* we know that locator is a xmlParserCtxtPtr */
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if locator.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*ctx).node.is_null() {
        ret = xmlGetLineNo((*ctx).node as *const xmlNode) as std::os::raw::c_int
    } else {
        /* inspired from error.c */
        let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
        input = (*ctx).input;
        if (*input).filename.is_null() && (*ctx).inputNr > 1 as std::os::raw::c_int {
            input =
                *(*ctx).inputTab.offset(((*ctx).inputNr - 2 as std::os::raw::c_int) as
                                            isize)
        }
        if !input.is_null() {
            ret = (*input).line
        } else { ret = -(1 as std::os::raw::c_int) }
    }
    return ret;
}
/* *
 * xmlTextReaderLocatorBaseURI:
 * @locator: the xmlTextReaderLocatorPtr used
 *
 * Obtain the base URI for the given locator.
 *
 * Returns the base URI or NULL in case of error,
 *    if non NULL it need to be freed by the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocatorBaseURI(mut locator:
                                                         xmlTextReaderLocatorPtr)
 -> *mut xmlChar {
    /* we know that locator is a xmlParserCtxtPtr */
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if locator.is_null() { return 0 as *mut xmlChar }
    if !(*ctx).node.is_null() {
        ret =
            xmlNodeGetBase(0 as *const xmlDoc, (*ctx).node as *const xmlNode)
    } else {
        /* inspired from error.c */
        let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
        input = (*ctx).input;
        if (*input).filename.is_null() && (*ctx).inputNr > 1 as std::os::raw::c_int {
            input =
                *(*ctx).inputTab.offset(((*ctx).inputNr - 2 as std::os::raw::c_int) as
                                            isize)
        }
        if !input.is_null() {
            ret = xmlStrdup((*input).filename as *mut xmlChar)
        } else { ret = 0 as *mut xmlChar }
    }
    return ret;
}
// from nanoftp.c:
/* the protocol name */
/* the host name */
/* the port */
/* the path within the URL */
/* user string */
/* passwd string */
/* this is large enough to hold IPv6 address*/
/* currently we support only passive !!! */
/* the file descriptor for the control socket */
/* the file descriptor for the data socket */
/* WRITE / READ / CLOSED */
/* the protocol return value */
/* buffer for data received from the control connection */
// from error.c:
// already in include/libxml2/xmlerror.h?:
// void XMLCDECL __xmlRaiseError(xmlStructuredErrorFunc schannel, xmlGenericErrorFunc channel, void *data, void *ctx,
//                               void *nod, int domain, int code, xmlErrorLevel level, const char *file, int line,
//                               const char *str1, const char *str2, const char *str3, int int1, int col, const char *msg, ...);
// void XMLCDECL xmlParserError(void *ctx, const char *msg, ...);
// void XMLCDECL xmlParserWarning(void *ctx, const char *msg, ...);
// void XMLCDECL xmlParserValidityError(void *ctx, const char *msg, ...);
// void XMLCDECL xmlParserValidityWarning(void *ctx, const char *msg, ...);
// from valid.c:
// from xmlreader.c:
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGenericError(mut ctxt:
                                                       *mut std::os::raw::c_void,
                                                   mut severity:
                                                       xmlParserSeverities,
                                                   mut str:
                                                       *mut std::os::raw::c_char) {
    let mut ctx: xmlParserCtxtPtr = ctxt as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctx)._private as xmlTextReaderPtr;
    if !str.is_null() {
        if (*reader).errorFunc.is_some() {
            (*reader).errorFunc.expect("non-null function pointer")((*reader).errorFuncArg,
                                                                    str,
                                                                    severity,
                                                                    ctx as
                                                                        xmlTextReaderLocatorPtr);
        }
        xmlFree.expect("non-null function pointer")(str as *mut std::os::raw::c_void);
    };
}
// static char *xmlTextReaderBuildMessage(const char *msg, va_list ap) LIBXML_ATTR_FORMAT(1,0);
// static void XMLCDECL
// xmlTextReaderValidityError(void *ctxt, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
// static void XMLCDECL
// xmlTextReaderValidityWarning(void *ctxt, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
// static void XMLCDECL
// xmlTextReaderValidityErrorRelay(void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
// static void XMLCDECL
// xmlTextReaderValidityWarningRelay(void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
// static void XMLCDECL
// xmlTextReaderValidityErrorRelay(void *ctx, const char *msg, ...)
// {
//     xmlTextReaderPtr reader = (xmlTextReaderPtr) ctx;
//     char *str;
//     va_list ap;
//     va_start(ap, msg);
//     str = xmlTextReaderBuildMessage(msg, ap);
//     if (!reader->errorFunc) {
//         xmlTextReaderValidityError(ctx, "%s", str);
//     } else {
//         reader->errorFunc(reader->errorFuncArg, str,
//                           XML_PARSER_SEVERITY_VALIDITY_ERROR,
//                           NULL /* locator */ );
//     }
//     if (str != NULL)
//         xmlFree(str);
//     va_end(ap);
// }
// static void XMLCDECL
// xmlTextReaderValidityWarningRelay(void *ctx, const char *msg, ...)
// {
//     xmlTextReaderPtr reader = (xmlTextReaderPtr) ctx;
//     char *str;
//     va_list ap;
//     va_start(ap, msg);
//     str = xmlTextReaderBuildMessage(msg, ap);
//     if (!reader->errorFunc) {
//         xmlTextReaderValidityWarning(ctx, "%s", str);
//     } else {
//         reader->errorFunc(reader->errorFuncArg, str,
//                           XML_PARSER_SEVERITY_VALIDITY_WARNING,
//                           NULL /* locator */ );
//     }
//     if (str != NULL)
//         xmlFree(str);
//     va_end(ap);
// }
unsafe extern "C" fn xmlTextReaderStructuredError(mut ctxt: *mut std::os::raw::c_void,
                                                  mut error: xmlErrorPtr) {
    let mut ctx: xmlParserCtxtPtr = ctxt as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctx)._private as xmlTextReaderPtr;
    if !error.is_null() && (*reader).sErrorFunc.is_some() {
        (*reader).sErrorFunc.expect("non-null function pointer")((*reader).errorFuncArg,
                                                                 error);
    };
}
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// xmlTextReaderError(void *ctxt, const char *msg, ...)
// {
//     va_list ap;
//     va_start(ap, msg);
//     xmlTextReaderGenericError(ctxt,
//                               XML_PARSER_SEVERITY_ERROR,
//                               xmlTextReaderBuildMessage(msg, ap));
//     va_end(ap);
// }
// static void XMLCDECL LIBXML_ATTR_FORMAT(2,3)
// xmlTextReaderWarning(void *ctxt, const char *msg, ...)
// {
//     va_list ap;
//     va_start(ap, msg);
//     xmlTextReaderGenericError(ctxt,
//                               XML_PARSER_SEVERITY_WARNING,
//                               xmlTextReaderBuildMessage(msg, ap));
//     va_end(ap);
// }
// static void XMLCDECL
// xmlTextReaderValidityError(void *ctxt, const char *msg, ...)
// {
//     va_list ap;
//     int len = xmlStrlen((const xmlChar *) msg);
//     if ((len > 1) && (msg[len - 2] != ':')) {
//         /*
//          * some callbacks only report locator information:
//          * skip them (mimicking behaviour in error.c)
//          */
//         va_start(ap, msg);
//         xmlTextReaderGenericError(ctxt,
//                                   XML_PARSER_SEVERITY_VALIDITY_ERROR,
//                                   xmlTextReaderBuildMessage(msg, ap));
//         va_end(ap);
//     }
// }
// static void XMLCDECL
// xmlTextReaderValidityWarning(void *ctxt, const char *msg, ...)
// {
//     va_list ap;
//     int len = xmlStrlen((const xmlChar *) msg);
//     if ((len != 0) && (msg[len - 1] != ':')) {
//         /*
//          * some callbacks only report locator information:
//          * skip them (mimicking behaviour in error.c)
//          */
//         va_start(ap, msg);
//         xmlTextReaderGenericError(ctxt,
//                                   XML_PARSER_SEVERITY_VALIDITY_WARNING,
//                                   xmlTextReaderBuildMessage(msg, ap));
//         va_end(ap);
//     }
// }
/* *
 * xmlTextReaderSetErrorHandler:
 * @reader:  the xmlTextReaderPtr used
 * @f:	the callback function to call on error and warnings
 * @arg:    a user argument to pass to the callback function
 *
 * Register a callback function that will be called on error and warnings.
 *
 * If @f is NULL, the default error and warning handlers are restored.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetErrorHandler(mut reader:
                                                          xmlTextReaderPtr,
                                                      mut f:
                                                          xmlTextReaderErrorFunc,
                                                      mut arg:
                                                          *mut std::os::raw::c_void) {
    if f.is_some() {
        (*(*(*reader).ctxt).sax).error =
            Some(xmlTextReaderError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*(*reader).ctxt).sax).serror = None;
        (*(*reader).ctxt).vctxt.error =
            Some(xmlTextReaderValidityError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*(*reader).ctxt).sax).warning =
            Some(xmlTextReaderWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*reader).ctxt).vctxt.warning =
            Some(xmlTextReaderValidityWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*reader).errorFunc = f;
        (*reader).sErrorFunc = None;
        (*reader).errorFuncArg = arg;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors((*reader).rngValidCtxt,
                                     Some(xmlTextReaderValidityErrorRelay as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                     Some(xmlTextReaderValidityWarningRelay as
                                              unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _:
                                                                       *const std::os::raw::c_char,
                                                                   _: ...)
                                                  -> ()),
                                     reader as *mut std::os::raw::c_void);
            xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt, None,
                                               reader as *mut std::os::raw::c_void);
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors((*reader).xsdValidCtxt,
                                    Some(xmlTextReaderValidityErrorRelay as
                                             unsafe extern "C" fn(_:
                                                                      *mut std::os::raw::c_void,
                                                                  _:
                                                                      *const std::os::raw::c_char,
                                                                  _: ...)
                                                 -> ()),
                                    Some(xmlTextReaderValidityWarningRelay as
                                             unsafe extern "C" fn(_:
                                                                      *mut std::os::raw::c_void,
                                                                  _:
                                                                      *const std::os::raw::c_char,
                                                                  _: ...)
                                                 -> ()),
                                    reader as *mut std::os::raw::c_void);
            xmlSchemaSetValidStructuredErrors((*reader).xsdValidCtxt, None,
                                              reader as *mut std::os::raw::c_void);
        }
    } else {
        /* restore defaults */
        (*(*(*reader).ctxt).sax).error =
            Some(xmlParserError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*reader).ctxt).vctxt.error =
            Some(xmlParserValidityError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*(*reader).ctxt).sax).warning =
            Some(xmlParserWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*reader).ctxt).vctxt.warning =
            Some(xmlParserValidityWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*reader).errorFunc = None;
        (*reader).sErrorFunc = None;
        (*reader).errorFuncArg = 0 as *mut std::os::raw::c_void;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors((*reader).rngValidCtxt, None, None,
                                     reader as *mut std::os::raw::c_void);
            xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt, None,
                                               reader as *mut std::os::raw::c_void);
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors((*reader).xsdValidCtxt, None, None,
                                    reader as *mut std::os::raw::c_void);
            xmlSchemaSetValidStructuredErrors((*reader).xsdValidCtxt, None,
                                              reader as *mut std::os::raw::c_void);
        }
    };
}
/* *
* xmlTextReaderSetStructuredErrorHandler:
 * @reader:  the xmlTextReaderPtr used
 * @f:	the callback function to call on error and warnings
 * @arg:    a user argument to pass to the callback function
 *
 * Register a callback function that will be called on error and warnings.
 *
 * If @f is NULL, the default error and warning handlers are restored.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetStructuredErrorHandler(mut reader:
                                                                    xmlTextReaderPtr,
                                                                mut f:
                                                                    xmlStructuredErrorFunc,
                                                                mut arg:
                                                                    *mut std::os::raw::c_void) {
    if f.is_some() {
        (*(*(*reader).ctxt).sax).error = None;
        (*(*(*reader).ctxt).sax).serror =
            Some(xmlTextReaderStructuredError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: xmlErrorPtr) -> ());
        (*(*reader).ctxt).vctxt.error =
            Some(xmlTextReaderValidityError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*(*reader).ctxt).sax).warning =
            Some(xmlTextReaderWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*reader).ctxt).vctxt.warning =
            Some(xmlTextReaderValidityWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*reader).sErrorFunc = f;
        (*reader).errorFunc = None;
        (*reader).errorFuncArg = arg;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors((*reader).rngValidCtxt, None, None,
                                     reader as *mut std::os::raw::c_void);
            xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt,
                                               Some(xmlTextReaderValidityStructuredRelay
                                                        as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut std::os::raw::c_void,
                                                                             _:
                                                                                 xmlErrorPtr)
                                                            -> ()),
                                               reader as *mut std::os::raw::c_void);
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors((*reader).xsdValidCtxt, None, None,
                                    reader as *mut std::os::raw::c_void);
            xmlSchemaSetValidStructuredErrors((*reader).xsdValidCtxt,
                                              Some(xmlTextReaderValidityStructuredRelay
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *mut std::os::raw::c_void,
                                                                            _:
                                                                                xmlErrorPtr)
                                                           -> ()),
                                              reader as *mut std::os::raw::c_void);
        }
    } else {
        /* restore defaults */
        (*(*(*reader).ctxt).sax).error =
            Some(xmlParserError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*(*reader).ctxt).sax).serror = None;
        (*(*reader).ctxt).vctxt.error =
            Some(xmlParserValidityError as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*(*reader).ctxt).sax).warning =
            Some(xmlParserWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*(*reader).ctxt).vctxt.warning =
            Some(xmlParserValidityWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ());
        (*reader).errorFunc = None;
        (*reader).sErrorFunc = None;
        (*reader).errorFuncArg = 0 as *mut std::os::raw::c_void;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors((*reader).rngValidCtxt, None, None,
                                     reader as *mut std::os::raw::c_void);
            xmlRelaxNGSetValidStructuredErrors((*reader).rngValidCtxt, None,
                                               reader as *mut std::os::raw::c_void);
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors((*reader).xsdValidCtxt, None, None,
                                    reader as *mut std::os::raw::c_void);
            xmlSchemaSetValidStructuredErrors((*reader).xsdValidCtxt, None,
                                              reader as *mut std::os::raw::c_void);
        }
    };
}
/* *
 * xmlTextReaderIsValid:
 * @reader:  the xmlTextReaderPtr used
 *
 * Retrieve the validity status from the parser context
 *
 * Returns the flag value 1 if valid, 0 if no, and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsValid(mut reader: xmlTextReaderPtr)
 -> std::os::raw::c_int {
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_RNG as std::os::raw::c_int as std::os::raw::c_uint {
        return ((*reader).rngValidErrors == 0 as std::os::raw::c_int) as std::os::raw::c_int
    }
    if (*reader).validate as std::os::raw::c_uint ==
           XML_TEXTREADER_VALIDATE_XSD as std::os::raw::c_int as std::os::raw::c_uint {
        return ((*reader).xsdValidErrors == 0 as std::os::raw::c_int) as std::os::raw::c_int
    }
    if !(*reader).ctxt.is_null() &&
           (*(*reader).ctxt).validate == 1 as std::os::raw::c_int {
        return (*(*reader).ctxt).valid
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderGetErrorHandler:
 * @reader:  the xmlTextReaderPtr used
 * @f:	the callback function or NULL is no callback has been registered
 * @arg:    a user argument
 *
 * Retrieve the error callback function and user argument.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetErrorHandler(mut reader:
                                                          xmlTextReaderPtr,
                                                      mut f:
                                                          *mut xmlTextReaderErrorFunc,
                                                      mut arg:
                                                          *mut *mut std::os::raw::c_void) {
    if !f.is_null() { *f = (*reader).errorFunc }
    if !arg.is_null() { *arg = (*reader).errorFuncArg };
}
/* ***********************************************************************
 *									*
 *	New set (2.6.0) of simpler and more flexible APIs		*
 *									*
 ************************************************************************/
/* *
 * xmlTextReaderSetup:
 * @reader:  an XML reader
 * @input: xmlParserInputBufferPtr used to feed the reader, will
 *         be destroyed with it.
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Setup an XML reader with new options
 *
 * Returns 0 in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetup(mut reader: xmlTextReaderPtr,
                                            mut input:
                                                xmlParserInputBufferPtr,
                                            mut URL: *const std::os::raw::c_char,
                                            mut encoding: *const std::os::raw::c_char,
                                            mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if reader.is_null() {
        if !input.is_null() { xmlFreeParserInputBuffer(input); }
        return -(1 as std::os::raw::c_int)
    }
    /*
     * we force the generation of compact text nodes on the reader
     * since usr applications should never modify the tree
     */
    options |= XML_PARSE_COMPACT as std::os::raw::c_int;
    (*reader).doc = 0 as xmlDocPtr;
    (*reader).entNr = 0 as std::os::raw::c_int;
    (*reader).parserFlags = options;
    (*reader).validate = XML_TEXTREADER_NOT_VALIDATE;
    if !input.is_null() && !(*reader).input.is_null() &&
           (*reader).allocs & 1 as std::os::raw::c_int != 0 {
        xmlFreeParserInputBuffer((*reader).input);
        (*reader).input = 0 as xmlParserInputBufferPtr;
        (*reader).allocs -= 1 as std::os::raw::c_int
    }
    if !input.is_null() {
        (*reader).input = input;
        (*reader).allocs |= 1 as std::os::raw::c_int
    }
    if (*reader).buffer.is_null() {
        (*reader).buffer = xmlBufCreateSize(100 as std::os::raw::c_int as size_t)
    }
    if (*reader).buffer.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlTextReaderSetup : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    /* no operation on a reader should require a huge buffer */
    xmlBufSetAllocationScheme((*reader).buffer, XML_BUFFER_ALLOC_BOUNDED);
    if (*reader).sax.is_null() {
        (*reader).sax =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSAXHandler>()
                                                              as
                                                              std::os::raw::c_ulong)
                as *mut xmlSAXHandler
    }
    if (*reader).sax.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlTextReaderSetup : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    xmlSAXVersion((*reader).sax, 2 as std::os::raw::c_int);
    (*reader).startElement = (*(*reader).sax).startElement;
    (*(*reader).sax).startElement =
        Some(xmlTextReaderStartElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: *mut *const xmlChar) -> ());
    (*reader).endElement = (*(*reader).sax).endElement;
    (*(*reader).sax).endElement =
        Some(xmlTextReaderEndElement as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
                     -> ());
    if (*(*reader).sax).initialized == 0xdeedbeaf as std::os::raw::c_uint {
        /* LIBXML_SAX1_ENABLED */
        (*reader).startElementNs = (*(*reader).sax).startElementNs;
        (*(*reader).sax).startElementNs =
            Some(xmlTextReaderStartElementNs as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *const xmlChar,
                                          _: *const xmlChar, _: std::os::raw::c_int,
                                          _: *mut *const xmlChar,
                                          _: std::os::raw::c_int, _: std::os::raw::c_int,
                                          _: *mut *const xmlChar) -> ());
        (*reader).endElementNs = (*(*reader).sax).endElementNs;
        (*(*reader).sax).endElementNs =
            Some(xmlTextReaderEndElementNs as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *const xmlChar,
                                          _: *const xmlChar) -> ())
    } else { (*reader).startElementNs = None; (*reader).endElementNs = None }
    /* LIBXML_SAX1_ENABLED */
    (*reader).characters = (*(*reader).sax).characters;
    (*(*reader).sax).characters =
        Some(xmlTextReaderCharacters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*(*reader).sax).ignorableWhitespace =
        Some(xmlTextReaderCharacters as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*reader).cdataBlock = (*(*reader).sax).cdataBlock;
    (*(*reader).sax).cdataBlock =
        Some(xmlTextReaderCDataBlock as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                      _: std::os::raw::c_int) -> ());
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int;
    (*reader).node = 0 as xmlNodePtr;
    (*reader).curnode = 0 as xmlNodePtr;
    if !input.is_null() {
        if xmlBufUse((*(*reader).input).buffer) <
               4 as std::os::raw::c_int as std::os::raw::c_ulong {
            xmlParserInputBufferRead(input, 4 as std::os::raw::c_int);
        }
        if (*reader).ctxt.is_null() {
            if xmlBufUse((*(*reader).input).buffer) >=
                   4 as std::os::raw::c_int as std::os::raw::c_ulong {
                (*reader).ctxt =
                    xmlCreatePushParserCtxt((*reader).sax,
                                            0 as *mut std::os::raw::c_void,
                                            xmlBufContent((*(*reader).input).buffer
                                                              as
                                                              *const xmlBuf)
                                                as *const std::os::raw::c_char,
                                            4 as std::os::raw::c_int, URL);
                (*reader).base = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                (*reader).cur = 4 as std::os::raw::c_int as std::os::raw::c_uint
            } else {
                (*reader).ctxt =
                    xmlCreatePushParserCtxt((*reader).sax,
                                            0 as *mut std::os::raw::c_void,
                                            0 as *const std::os::raw::c_char,
                                            0 as std::os::raw::c_int, URL);
                (*reader).base = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                (*reader).cur = 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
        } else {
            let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
            let mut buf: xmlParserInputBufferPtr =
                0 as *mut xmlParserInputBuffer;
            let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
            xmlCtxtReset((*reader).ctxt);
            buf = xmlAllocParserInputBuffer(enc);
            if buf.is_null() { return -(1 as std::os::raw::c_int) }
            inputStream = xmlNewInputStream((*reader).ctxt);
            if inputStream.is_null() {
                xmlFreeParserInputBuffer(buf);
                return -(1 as std::os::raw::c_int)
            }
            if URL.is_null() {
                (*inputStream).filename = 0 as *const std::os::raw::c_char
            } else {
                (*inputStream).filename =
                    xmlCanonicPath(URL as *const xmlChar) as *mut std::os::raw::c_char
            }
            (*inputStream).buf = buf;
            xmlBufResetInput((*buf).buffer, inputStream);
            inputPush((*reader).ctxt, inputStream);
            (*reader).cur = 0 as std::os::raw::c_int as std::os::raw::c_uint
        }
        if (*reader).ctxt.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"xmlTextReaderSetup : malloc failed\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if !(*reader).dict.is_null() {
        if !(*(*reader).ctxt).dict.is_null() {
            if (*reader).dict != (*(*reader).ctxt).dict {
                xmlDictFree((*reader).dict);
                (*reader).dict = (*(*reader).ctxt).dict
            }
        } else { (*(*reader).ctxt).dict = (*reader).dict }
    } else {
        if (*(*reader).ctxt).dict.is_null() {
            (*(*reader).ctxt).dict = xmlDictCreate()
        }
        (*reader).dict = (*(*reader).ctxt).dict
    }
    (*(*reader).ctxt)._private = reader as *mut std::os::raw::c_void;
    (*(*reader).ctxt).linenumbers = 1 as std::os::raw::c_int;
    (*(*reader).ctxt).dictNames = 1 as std::os::raw::c_int;
    /*
     * use the parser dictionary to allocate all elements and attributes names
     */
    (*(*reader).ctxt).docdict = 1 as std::os::raw::c_int;
    (*(*reader).ctxt).parseMode = XML_PARSE_READER;
    if !(*reader).xincctxt.is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
        (*reader).xincctxt = 0 as xmlXIncludeCtxtPtr
    }
    if options & XML_PARSE_XINCLUDE as std::os::raw::c_int != 0 {
        (*reader).xinclude = 1 as std::os::raw::c_int;
        (*reader).xinclude_name =
            xmlDictLookup((*reader).dict,
                          b"include\x00" as *const u8 as *const std::os::raw::c_char
                              as *const xmlChar, -(1 as std::os::raw::c_int));
        options -= XML_PARSE_XINCLUDE as std::os::raw::c_int
    } else { (*reader).xinclude = 0 as std::os::raw::c_int }
    (*reader).in_xinclude = 0 as std::os::raw::c_int;
    if (*reader).patternTab.is_null() {
        (*reader).patternNr = 0 as std::os::raw::c_int;
        (*reader).patternMax = 0 as std::os::raw::c_int
    }
    while (*reader).patternNr > 0 as std::os::raw::c_int {
        (*reader).patternNr -= 1;
        if !(*(*reader).patternTab.offset((*reader).patternNr as
                                              isize)).is_null() {
            xmlFreePattern(*(*reader).patternTab.offset((*reader).patternNr as
                                                            isize));
            let ref mut fresh5 =
                *(*reader).patternTab.offset((*reader).patternNr as isize);
            *fresh5 = 0 as xmlPatternPtr
        }
    }
    if options & XML_PARSE_DTDVALID as std::os::raw::c_int != 0 {
        (*reader).validate = XML_TEXTREADER_VALIDATE_DTD
    }
    xmlCtxtUseOptions((*reader).ctxt, options);
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr =
            0 as *mut xmlCharEncodingHandler;
        hdlr = xmlFindCharEncodingHandler(encoding);
        if !hdlr.is_null() { xmlSwitchToEncoding((*reader).ctxt, hdlr); }
    }
    if !URL.is_null() && !(*(*reader).ctxt).input.is_null() &&
           (*(*(*reader).ctxt).input).filename.is_null() {
        (*(*(*reader).ctxt).input).filename =
            xmlStrdup(URL as *const xmlChar) as *mut std::os::raw::c_char
    }
    (*reader).doc = 0 as xmlDocPtr;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlTextReaderByteConsumed:
 * @reader: an XML reader
 *
 * This function provides the current index of the parser used
 * by the reader, relative to the start of the current entity.
 * This function actually just wraps a call to xmlBytesConsumed()
 * for the parser context associated with the reader.
 * See xmlBytesConsumed() for more information.
 *
 * Returns the index in bytes from the beginning of the entity or -1
 *         in case the index could not be computed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderByteConsumed(mut reader:
                                                       xmlTextReaderPtr)
 -> std::os::raw::c_long {
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as std::os::raw::c_int) as std::os::raw::c_long
    }
    return xmlByteConsumed((*reader).ctxt);
}
/* *
 * xmlReaderWalker:
 * @doc:  a preparsed document
 *
 * Create an xmltextReader for a preparsed document.
 *
 * Returns the new reader or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderWalker(mut doc: xmlDocPtr)
 -> xmlTextReaderPtr {
    let mut ret: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    if doc.is_null() { return 0 as xmlTextReaderPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlTextReader>()
                                                          as std::os::raw::c_ulong) as
            xmlTextReaderPtr;
    if ret.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"xmlNewTextReader : malloc failed\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return 0 as xmlTextReaderPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlTextReader>() as std::os::raw::c_ulong);
    (*ret).entNr = 0 as std::os::raw::c_int;
    (*ret).input = 0 as xmlParserInputBufferPtr;
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int;
    (*ret).node = 0 as xmlNodePtr;
    (*ret).curnode = 0 as xmlNodePtr;
    (*ret).base = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*ret).cur = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*ret).allocs = 2 as std::os::raw::c_int;
    (*ret).doc = doc;
    (*ret).state = XML_TEXTREADER_START;
    (*ret).dict = xmlDictCreate();
    return ret;
}
/* *
 * xmlReaderForDoc:
 * @cur:  a pointer to a zero terminated string
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Create an xmltextReader for an XML in-memory document.
 * The parsing flags @options are a combination of xmlParserOption.
 *
 * Returns the new reader or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForDoc(mut cur: *const xmlChar,
                                         mut URL: *const std::os::raw::c_char,
                                         mut encoding: *const std::os::raw::c_char,
                                         mut options: std::os::raw::c_int)
 -> xmlTextReaderPtr {
    let mut len: std::os::raw::c_int = 0;
    if cur.is_null() { return 0 as xmlTextReaderPtr }
    len = xmlStrlen(cur);
    return xmlReaderForMemory(cur as *const std::os::raw::c_char, len, URL, encoding,
                              options);
}
/* *
 * xmlReaderForFile:
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML file from the filesystem or the network.
 * The parsing flags @options are a combination of xmlParserOption.
 *
 * Returns the new reader or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForFile(mut filename: *const std::os::raw::c_char,
                                          mut encoding: *const std::os::raw::c_char,
                                          mut options: std::os::raw::c_int)
 -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    reader = xmlNewTextReaderFilename(filename);
    if reader.is_null() { return 0 as xmlTextReaderPtr }
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr,
                       0 as *const std::os::raw::c_char, encoding, options);
    return reader;
}
/* *
 * xmlReaderForMemory:
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Create an xmltextReader for an XML in-memory document.
 * The parsing flags @options are a combination of xmlParserOption.
 *
 * Returns the new reader or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForMemory(mut buffer: *const std::os::raw::c_char,
                                            mut size: std::os::raw::c_int,
                                            mut URL: *const std::os::raw::c_char,
                                            mut encoding: *const std::os::raw::c_char,
                                            mut options: std::os::raw::c_int)
 -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    buf =
        xmlParserInputBufferCreateStatic(buffer, size,
                                         XML_CHAR_ENCODING_NONE);
    if buf.is_null() { return 0 as xmlTextReaderPtr }
    reader = xmlNewTextReader(buf, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(buf);
        return 0 as xmlTextReaderPtr
    }
    (*reader).allocs |= 1 as std::os::raw::c_int;
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr, URL, encoding,
                       options);
    return reader;
}
/* *
 * xmlReaderForFd:
 * @fd:  an open file descriptor
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Create an xmltextReader for an XML from a file descriptor.
 * The parsing flags @options are a combination of xmlParserOption.
 * NOTE that the file descriptor will not be closed when the
 *      reader is closed or reset.
 *
 * Returns the new reader or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForFd(mut fd: std::os::raw::c_int,
                                        mut URL: *const std::os::raw::c_char,
                                        mut encoding: *const std::os::raw::c_char,
                                        mut options: std::os::raw::c_int)
 -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if fd < 0 as std::os::raw::c_int { return 0 as xmlTextReaderPtr }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() { return 0 as xmlTextReaderPtr }
    (*input).closecallback = None;
    reader = xmlNewTextReader(input, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(input);
        return 0 as xmlTextReaderPtr
    }
    (*reader).allocs |= 1 as std::os::raw::c_int;
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr, URL, encoding,
                       options);
    return reader;
}
/* *
 * xmlReaderForIO:
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Create an xmltextReader for an XML document from I/O functions and source.
 * The parsing flags @options are a combination of xmlParserOption.
 *
 * Returns the new reader or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForIO(mut ioread: xmlInputReadCallback,
                                        mut ioclose: xmlInputCloseCallback,
                                        mut ioctx: *mut std::os::raw::c_void,
                                        mut URL: *const std::os::raw::c_char,
                                        mut encoding: *const std::os::raw::c_char,
                                        mut options: std::os::raw::c_int)
 -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if ioread.is_none() { return 0 as xmlTextReaderPtr }
    input =
        xmlParserInputBufferCreateIO(ioread, ioclose, ioctx,
                                     XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return 0 as xmlTextReaderPtr
    }
    reader = xmlNewTextReader(input, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(input);
        return 0 as xmlTextReaderPtr
    }
    (*reader).allocs |= 1 as std::os::raw::c_int;
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr, URL, encoding,
                       options);
    return reader;
}
/* *
 * xmlReaderNewWalker:
 * @reader:  an XML reader
 * @doc:  a preparsed document
 *
 * Setup an xmltextReader to parse a preparsed XML document.
 * This reuses the existing @reader xmlTextReader.
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewWalker(mut reader: xmlTextReaderPtr,
                                            mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*reader).input.is_null() {
        xmlFreeParserInputBuffer((*reader).input);
    }
    if !(*reader).ctxt.is_null() { xmlCtxtReset((*reader).ctxt); }
    (*reader).entNr = 0 as std::os::raw::c_int;
    (*reader).input = 0 as xmlParserInputBufferPtr;
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as std::os::raw::c_int;
    (*reader).node = 0 as xmlNodePtr;
    (*reader).curnode = 0 as xmlNodePtr;
    (*reader).base = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*reader).cur = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*reader).allocs = 2 as std::os::raw::c_int;
    (*reader).doc = doc;
    (*reader).state = XML_TEXTREADER_START;
    if (*reader).dict.is_null() {
        if !(*reader).ctxt.is_null() && !(*(*reader).ctxt).dict.is_null() {
            (*reader).dict = (*(*reader).ctxt).dict
        } else { (*reader).dict = xmlDictCreate() }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlReaderNewDoc:
 * @reader:  an XML reader
 * @cur:  a pointer to a zero terminated string
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Setup an xmltextReader to parse an XML in-memory document.
 * The parsing flags @options are a combination of xmlParserOption.
 * This reuses the existing @reader xmlTextReader.
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewDoc(mut reader: xmlTextReaderPtr,
                                         mut cur: *const xmlChar,
                                         mut URL: *const std::os::raw::c_char,
                                         mut encoding: *const std::os::raw::c_char,
                                         mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut len: std::os::raw::c_int = 0;
    if cur.is_null() { return -(1 as std::os::raw::c_int) }
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    len = xmlStrlen(cur);
    return xmlReaderNewMemory(reader, cur as *const std::os::raw::c_char, len, URL,
                              encoding, options);
}
/* *
 * xmlReaderNewFile:
 * @reader:  an XML reader
 * @filename:  a file or URL
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * parse an XML file from the filesystem or the network.
 * The parsing flags @options are a combination of xmlParserOption.
 * This reuses the existing @reader xmlTextReader.
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewFile(mut reader: xmlTextReaderPtr,
                                          mut filename: *const std::os::raw::c_char,
                                          mut encoding: *const std::os::raw::c_char,
                                          mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if filename.is_null() { return -(1 as std::os::raw::c_int) }
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    input =
        xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if input.is_null() { return -(1 as std::os::raw::c_int) }
    return xmlTextReaderSetup(reader, input, filename, encoding, options);
}
/* *
 * xmlReaderNewMemory:
 * @reader:  an XML reader
 * @buffer:  a pointer to a char array
 * @size:  the size of the array
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Setup an xmltextReader to parse an XML in-memory document.
 * The parsing flags @options are a combination of xmlParserOption.
 * This reuses the existing @reader xmlTextReader.
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewMemory(mut reader: xmlTextReaderPtr,
                                            mut buffer: *const std::os::raw::c_char,
                                            mut size: std::os::raw::c_int,
                                            mut URL: *const std::os::raw::c_char,
                                            mut encoding: *const std::os::raw::c_char,
                                            mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    if buffer.is_null() { return -(1 as std::os::raw::c_int) }
    input =
        xmlParserInputBufferCreateStatic(buffer, size,
                                         XML_CHAR_ENCODING_NONE);
    if input.is_null() { return -(1 as std::os::raw::c_int) }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
/* *
 * xmlReaderNewFd:
 * @reader:  an XML reader
 * @fd:  an open file descriptor
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Setup an xmltextReader to parse an XML from a file descriptor.
 * NOTE that the file descriptor will not be closed when the
 *      reader is closed or reset.
 * The parsing flags @options are a combination of xmlParserOption.
 * This reuses the existing @reader xmlTextReader.
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewFd(mut reader: xmlTextReaderPtr,
                                        mut fd: std::os::raw::c_int,
                                        mut URL: *const std::os::raw::c_char,
                                        mut encoding: *const std::os::raw::c_char,
                                        mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if fd < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() { return -(1 as std::os::raw::c_int) }
    (*input).closecallback = None;
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
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
/* *
 * xmlTextReaderMode:
 *
 * Internal state values for the reader.
 */
/* *
 * xmlParserProperties:
 *
 * Some common options to use with xmlTextReaderSetParserProp, but it
 * is better to use xmlParserOption and the xmlReaderNewxxx and
 * xmlReaderForxxx APIs now.
 */
/* *
 * xmlReaderTypes:
 *
 * Predefined constants for the different types of nodes.
 */
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
/* *
 * xmlReaderNewIO:
 * @reader:  an XML reader
 * @ioread:  an I/O read function
 * @ioclose:  an I/O close function
 * @ioctx:  an I/O handler
 * @URL:  the base URL to use for the document
 * @encoding:  the document encoding, or NULL
 * @options:  a combination of xmlParserOption
 *
 * Setup an xmltextReader to parse an XML document from I/O functions
 * and source.
 * The parsing flags @options are a combination of xmlParserOption.
 * This reuses the existing @reader xmlTextReader.
 *
 * Returns 0 in case of success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewIO(mut reader: xmlTextReaderPtr,
                                        mut ioread: xmlInputReadCallback,
                                        mut ioclose: xmlInputCloseCallback,
                                        mut ioctx: *mut std::os::raw::c_void,
                                        mut URL: *const std::os::raw::c_char,
                                        mut encoding: *const std::os::raw::c_char,
                                        mut options: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if ioread.is_none() { return -(1 as std::os::raw::c_int) }
    if reader.is_null() { return -(1 as std::os::raw::c_int) }
    input =
        xmlParserInputBufferCreateIO(ioread, ioclose, ioctx,
                                     XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return -(1 as std::os::raw::c_int)
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
/* LIBXML_READER_ENABLED */
/* __INCLUDE_ELFGCCHACK */
/* NOT_USED_YET */
/* ***********************************************************************
 *									*
 *			Utilities					*
 *									*
 ************************************************************************/
