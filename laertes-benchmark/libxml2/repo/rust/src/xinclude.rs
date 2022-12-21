
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
    pub type _xmlXPathCompExpr;
    /* *
 * BAD_CAST:
 *
 * Macro to cast a string to an xmlChar * when one know its safe.
 */
    /*
 * xmlChar handling
 */
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    #[no_mangle]
    fn xmlDictReference(dict: xmlDictPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlDictFree(dict: xmlDictPtr);
    /*
 * Creating/freeing new structures.
 */
    #[no_mangle]
    fn xmlCreateIntSubset(doc: xmlDocPtr, name: *const xmlChar,
                          ExternalID: *const xmlChar,
                          SystemID: *const xmlChar) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewTextLen(content: *const xmlChar, len: std::os::raw::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlCopyNode(node: xmlNodePtr, recursive: std::os::raw::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlDocCopyNode(node: xmlNodePtr, doc: xmlDocPtr,
                      recursive: std::os::raw::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlDocCopyNodeList(doc: xmlDocPtr, node: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlCopyNodeList(node: xmlNodePtr) -> xmlNodePtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_WRITER_ENABLED) */
    #[no_mangle]
    fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlAddNextSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlUnlinkNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlFreeNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlGetNsProp(node: *const xmlNode, name: *const xmlChar,
                    nameSpace: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeAddContentLen(cur: xmlNodePtr, content: *const xmlChar,
                            len: std::os::raw::c_int);
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar);
    #[no_mangle]
    fn xmlHashScan(table: xmlHashTablePtr, f: xmlHashScanner,
                   data: *mut std::os::raw::c_void);
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
    fn xmlAddDocEntity(doc: xmlDocPtr, name: *const xmlChar,
                       type_0: std::os::raw::c_int, ExternalID: *const xmlChar,
                       SystemID: *const xmlChar, content: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlParseCharEncoding(name: *const std::os::raw::c_char) -> xmlCharEncoding;
    #[no_mangle]
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParserInputBufferRead(in_0: xmlParserInputBufferPtr,
                                len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    #[no_mangle]
    fn xmlParserGetDirectory(filename: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    /*
 * Init/Cleanup
 */
    #[no_mangle]
    fn xmlInitParser();
    /* LIBXML_SAX1_ENABLED */
    /*
 * Less common routines and SAX interfaces
 */
    #[no_mangle]
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> std::os::raw::c_int;
    /*
 * Parser contexts handling.
 */
    #[no_mangle]
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    #[no_mangle]
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    #[no_mangle]
    fn xmlLoadExternalEntity(URL: *const std::os::raw::c_char,
                             ID: *const std::os::raw::c_char, ctxt: xmlParserCtxtPtr)
     -> xmlParserInputPtr;
    #[no_mangle]
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlBuildRelativeURI(URI: *const xmlChar, base: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlURIEscape(str: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
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
    #[no_mangle]
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    /*
 * Functions.
 */
    #[no_mangle]
    fn xmlXPtrNewContext(doc: xmlDocPtr, here: xmlNodePtr, origin: xmlNodePtr)
     -> xmlXPathContextPtr;
    #[no_mangle]
    fn xmlXPtrEval(str: *const xmlChar, ctx: xmlXPathContextPtr)
     -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    #[no_mangle]
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStringCurrentChar(ctxt: xmlParserCtxtPtr, cur: *const xmlChar,
                            len: *mut std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBufLength(buf: xmlBufPtr) -> size_t;
    #[no_mangle]
    fn xmlXPtrAdvanceNode(cur: xmlNodePtr, level: *mut std::os::raw::c_int)
     -> xmlNodePtr;
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
pub type xmlParserInputDeallocate
    =
    Option<unsafe extern "C" fn(_: *mut xmlChar) -> ()>;
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
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
/* *
 * getParameterEntitySAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The entity name
 *
 * Get a parameter entity by name.
 *
 * Returns the xmlEntityPtr if found.
 */
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
/* *
 * errorSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format an error messages, callback.
 */
pub type errorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * warningSAXFunc:
 * @ctx:  an XML parser context
 * @msg:  the message to display/transmit
 * @...:  extra parameters for the message display
 *
 * Display and format a warning messages, callback.
 */
pub type warningSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * commentSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @value:  the comment content
 *
 * A comment has been parsed.
 */
pub type commentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
/* *
 * processingInstructionSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @target:  the target name
 * @data: the PI data's
 *
 * A processing instruction has been parsed.
 */
pub type processingInstructionSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
/* *
 * ignorableWhitespaceSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * Receiving some ignorable whitespaces from the parser.
 * UNUSED: by default the DOM building will use characters.
 */
pub type ignorableWhitespaceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
/* *
 * charactersSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * Receiving some chars from the parser.
 */
pub type charactersSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int) -> ()>;
/* *
 * referenceSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The entity name
 *
 * Called when an entity reference is detected.
 */
pub type referenceSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
/* *
 * endElementSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The element name
 *
 * Called when the end of an element has been detected.
 */
pub type endElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> ()>;
/* *
 * startElementSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  The element name, including namespace prefix
 * @atts:  An array of name/value attributes pairs, NULL terminated
 *
 * Called when an opening tag has been processed.
 */
pub type startElementSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *mut *const xmlChar) -> ()>;
/* *
 * endDocumentSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Called when the document end has been detected.
 */
pub type endDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
/* *
 * startDocumentSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Called when the document start being processed.
 */
pub type startDocumentSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
/* *
 * setDocumentLocatorSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @loc: A SAX Locator
 *
 * Receive the document locator at startup, actually xmlDefaultSAXLocator.
 * Everything is available on the context, so this is useless in our case.
 */
pub type setDocumentLocatorSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlSAXLocatorPtr)
               -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
/* volume of entity copy */
/* *
 * xmlSAXLocator:
 *
 * A SAX Locator.
 */
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
pub type unparsedEntityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: *const xmlChar) -> ()>;
/* *
 * elementDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the element name
 * @type:  the element type
 * @content: the element value tree
 *
 * An element definition has been parsed.
 */
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
/* *
 * notationDeclSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name: The name of the notation
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * What to do when a notation declaration has been parsed.
 */
pub type notationDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
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
pub type entityDeclSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: std::os::raw::c_int, _: *const xmlChar,
                                _: *const xmlChar, _: *mut xmlChar) -> ()>;
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
pub type resolveEntitySAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlParserInputPtr>;
pub type hasExternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
/* *
 * hasInternalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Does this document has an internal subset.
 *
 * Returns 1 if true
 */
pub type hasInternalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
/* *
 * isStandaloneSAXFunc:
 * @ctx:  the user data (XML parser context)
 *
 * Is this document tagged standalone?
 *
 * Returns 1 if true
 */
pub type isStandaloneSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
/* *
 * internalSubsetSAXFunc:
 * @ctx:  the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on internal subset declaration.
 */
pub type internalSubsetSAXFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
/* *
 * xmlHashScanner:
 * @payload:  the data in the hash
 * @data:  extra scannner data
 * @name:  the name associated
 *
 * Callback when scanning data in a hash with the simple scanner.
 */
pub type xmlHashScanner
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_void,
                                _: *const xmlChar) -> ()>;
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
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 =
    3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 =
    3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
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
pub struct _xmlURI {
    pub scheme: *mut std::os::raw::c_char,
    pub opaque: *mut std::os::raw::c_char,
    pub authority: *mut std::os::raw::c_char,
    pub server: *mut std::os::raw::c_char,
    pub user: *mut std::os::raw::c_char,
    pub port: std::os::raw::c_int,
    pub path: *mut std::os::raw::c_char,
    pub query: *mut std::os::raw::c_char,
    pub fragment: *mut std::os::raw::c_char,
    pub cleanup: std::os::raw::c_int,
    pub query_raw: *mut std::os::raw::c_char,
}
/* *
 * Summary: library of generic URI related routines
 * Description: library of generic URI related routines
 *              Implements RFC 2396
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlURI:
 *
 * A parsed URI reference. This is a struct containing the various fields
 * as described in RFC 2396 but separated for further processing.
 *
 * Note: query is a deprecated field which is incorrectly unescaped.
 * query_raw takes precedence over query if the former is set.
 * See: http://mail.gnome.org/archives/xml/2007-April/thread.html#00127
 */
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLocationSet {
    pub locNr: std::os::raw::c_int,
    pub locMax: std::os::raw::c_int,
    pub locTab: *mut xmlXPathObjectPtr,
}
/*
 * Summary: API to handle XML Pointers
 * Description: API to handle XML Pointers
 * Base implementation was made accordingly to
 * W3C Candidate Recommendation 7 June 2000
 * http://www.w3.org/TR/2000/CR-xptr-20000607
 *
 * Added support for the element() scheme described in:
 * W3C Proposed Recommendation 13 November 2002
 * http://www.w3.org/TR/2002/PR-xptr-element-20021113/
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * A Location Set
 */
pub type xmlLocationSet = _xmlLocationSet;
pub type xmlLocationSetPtr = *mut xmlLocationSet;
/* flag to show fallback empty */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeCtxt {
    pub doc: xmlDocPtr,
    pub incBase: std::os::raw::c_int,
    pub incNr: std::os::raw::c_int,
    pub incMax: std::os::raw::c_int,
    pub incTab: *mut xmlXIncludeRefPtr,
    pub txtNr: std::os::raw::c_int,
    pub txtMax: std::os::raw::c_int,
    pub txtTab: *mut xmlNodePtr,
    pub txturlTab: *mut xmlURL,
    pub url: *mut xmlChar,
    pub urlNr: std::os::raw::c_int,
    pub urlMax: std::os::raw::c_int,
    pub urlTab: *mut *mut xmlChar,
    pub nbErrors: std::os::raw::c_int,
    pub legacy: std::os::raw::c_int,
    pub parseFlags: std::os::raw::c_int,
    pub base: *mut xmlChar,
    pub _private: *mut std::os::raw::c_void,
}
/* #define DEBUG_XINCLUDE */
/* ***********************************************************************
 *									*
 *			XInclude context handling			*
 *									*
 ************************************************************************/
/*
 * An XInclude context
 */
pub type xmlURL = *mut xmlChar;
pub type xmlXIncludeRefPtr = *mut xmlXIncludeRef;
pub type xmlXIncludeRef = _xmlXIncludeRef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeRef {
    pub URI: *mut xmlChar,
    pub fragment: *mut xmlChar,
    pub doc: xmlDocPtr,
    pub ref_0: xmlNodePtr,
    pub inc: xmlNodePtr,
    pub xml: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub xptr: xmlXPathObjectPtr,
    pub emptyFb: std::os::raw::c_int,
}
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
/* ***********************************************************************
 *									*
 *			XInclude I/O handling				*
 *									*
 ************************************************************************/
pub type xmlXIncludeMergeData = _xmlXIncludeMergeData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeMergeData {
    pub doc: xmlDocPtr,
    pub ctxt: xmlXIncludeCtxtPtr,
}
pub type xmlXIncludeMergeDataPtr = *mut xmlXIncludeMergeData;
/* ***********************************************************************
 *									*
 *			XInclude error handler				*
 *									*
 ************************************************************************/
/* *
 * xmlXIncludeErrMemory:
 * @extra:  extra information
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlXIncludeErrMemory(mut ctxt: xmlXIncludeCtxtPtr,
                                          mut node: xmlNodePtr,
                                          mut extra: *const std::os::raw::c_char) {
    if !ctxt.is_null() { (*ctxt).nbErrors += 1 }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, node as *mut std::os::raw::c_void,
                    XML_FROM_XINCLUDE as std::os::raw::c_int,
                    XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Memory allocation failed : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, extra);
}
/* *
 * xmlXIncludeErr:
 * @ctxt: the XInclude context
 * @node: the context node
 * @msg:  the error message
 * @extra:  extra information
 *
 * Handle an XInclude error
 */
unsafe extern "C" fn xmlXIncludeErr(mut ctxt: xmlXIncludeCtxtPtr,
                                    mut node: xmlNodePtr,
                                    mut error: std::os::raw::c_int,
                                    mut msg: *const std::os::raw::c_char,
                                    mut extra: *const xmlChar) {
    if !ctxt.is_null() { (*ctxt).nbErrors += 1 }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, node as *mut std::os::raw::c_void,
                    XML_FROM_XINCLUDE as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    extra as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, extra as *const std::os::raw::c_char);
}
/* *
 * xmlXIncludeGetProp:
 * @ctxt:  the XInclude context
 * @cur:  the node
 * @name:  the attribute name
 *
 * Get an XInclude attribute
 *
 * Returns the value (to be freed) or NULL if not found
 */
unsafe extern "C" fn xmlXIncludeGetProp(mut ctxt: xmlXIncludeCtxtPtr,
                                        mut cur: xmlNodePtr,
                                        mut name: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    ret =
        xmlGetNsProp(cur as *const xmlNode,
                     b"http://www.w3.org/2003/XInclude\x00" as *const u8 as
                         *const std::os::raw::c_char as *const xmlChar, name);
    if !ret.is_null() { return ret }
    if (*ctxt).legacy != 0 as std::os::raw::c_int {
        ret =
            xmlGetNsProp(cur as *const xmlNode,
                         b"http://www.w3.org/2001/XInclude\x00" as *const u8
                             as *const std::os::raw::c_char as *const xmlChar, name);
        if !ret.is_null() { return ret }
    }
    ret = xmlGetProp(cur as *const xmlNode, name);
    return ret;
}
/* *
 * xmlXIncludeFreeRef:
 * @ref: the XInclude reference
 *
 * Free an XInclude reference
 */
unsafe extern "C" fn xmlXIncludeFreeRef(mut ref_0: xmlXIncludeRefPtr) {
    if ref_0.is_null() { return }
    if !(*ref_0).doc.is_null() { xmlFreeDoc((*ref_0).doc); }
    if !(*ref_0).URI.is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).URI as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ref_0).fragment.is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).fragment as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ref_0).xptr.is_null() { xmlXPathFreeObject((*ref_0).xptr); }
    xmlFree.expect("non-null function pointer")(ref_0 as *mut std::os::raw::c_void);
}
/* *
 * xmlXIncludeNewRef:
 * @ctxt: the XInclude context
 * @URI:  the resource URI
 *
 * Creates a new reference within an XInclude context
 *
 * Returns the new set
 */
unsafe extern "C" fn xmlXIncludeNewRef(mut ctxt: xmlXIncludeCtxtPtr,
                                       mut URI: *const xmlChar,
                                       mut ref_0: xmlNodePtr)
 -> xmlXIncludeRefPtr {
    let mut ret: xmlXIncludeRefPtr = 0 as *mut xmlXIncludeRef;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXIncludeRef>()
                                                          as std::os::raw::c_ulong) as
            xmlXIncludeRefPtr;
    if ret.is_null() {
        xmlXIncludeErrMemory(ctxt, ref_0,
                             b"growing XInclude context\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        return 0 as xmlXIncludeRefPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXIncludeRef>() as std::os::raw::c_ulong);
    if URI.is_null() {
        (*ret).URI = 0 as *mut xmlChar
    } else { (*ret).URI = xmlStrdup(URI) }
    (*ret).fragment = 0 as *mut xmlChar;
    (*ret).ref_0 = ref_0;
    (*ret).doc = 0 as xmlDocPtr;
    (*ret).count = 0 as std::os::raw::c_int;
    (*ret).xml = 0 as std::os::raw::c_int;
    (*ret).inc = 0 as xmlNodePtr;
    if (*ctxt).incMax == 0 as std::os::raw::c_int {
        (*ctxt).incMax = 4 as std::os::raw::c_int;
        (*ctxt).incTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).incMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXIncludeRefPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlXIncludeRefPtr;
        if (*ctxt).incTab.is_null() {
            xmlXIncludeErrMemory(ctxt, ref_0,
                                 b"growing XInclude context\x00" as *const u8
                                     as *const std::os::raw::c_char);
            xmlXIncludeFreeRef(ret);
            return 0 as xmlXIncludeRefPtr
        }
    }
    if (*ctxt).incNr >= (*ctxt).incMax {
        (*ctxt).incMax *= 2 as std::os::raw::c_int;
        (*ctxt).incTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).incTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).incMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXIncludeRefPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlXIncludeRefPtr;
        if (*ctxt).incTab.is_null() {
            xmlXIncludeErrMemory(ctxt, ref_0,
                                 b"growing XInclude context\x00" as *const u8
                                     as *const std::os::raw::c_char);
            xmlXIncludeFreeRef(ret);
            return 0 as xmlXIncludeRefPtr
        }
    }
    let fresh0 = (*ctxt).incNr;
    (*ctxt).incNr = (*ctxt).incNr + 1;
    let ref mut fresh1 = *(*ctxt).incTab.offset(fresh0 as isize);
    *fresh1 = ret;
    return ret;
}
/*
 * contextual processing
 */
/* *
 * xmlXIncludeNewContext:
 * @doc:  an XML Document
 *
 * Creates a new XInclude context
 *
 * Returns the new set
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeNewContext(mut doc: xmlDocPtr)
 -> xmlXIncludeCtxtPtr {
    let mut ret: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    if doc.is_null() { return 0 as xmlXIncludeCtxtPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXIncludeCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlXIncludeCtxtPtr;
    if ret.is_null() {
        xmlXIncludeErrMemory(0 as xmlXIncludeCtxtPtr, doc as xmlNodePtr,
                             b"creating XInclude context\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        return 0 as xmlXIncludeCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXIncludeCtxt>() as std::os::raw::c_ulong);
    (*ret).doc = doc;
    (*ret).incNr = 0 as std::os::raw::c_int;
    (*ret).incBase = 0 as std::os::raw::c_int;
    (*ret).incMax = 0 as std::os::raw::c_int;
    (*ret).incTab = 0 as *mut xmlXIncludeRefPtr;
    (*ret).nbErrors = 0 as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlXIncludeURLPush:
 * @ctxt:  the parser context
 * @value:  the url
 *
 * Pushes a new url on top of the url stack
 *
 * Returns -1 in case of error, the index in the stack otherwise
 */
unsafe extern "C" fn xmlXIncludeURLPush(mut ctxt: xmlXIncludeCtxtPtr,
                                        mut value: *const xmlChar)
 -> std::os::raw::c_int {
    if (*ctxt).urlNr > 40 as std::os::raw::c_int {
        xmlXIncludeErr(ctxt, 0 as xmlNodePtr,
                       XML_XINCLUDE_RECURSION as std::os::raw::c_int,
                       b"detected a recursion in %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, value);
        return -(1 as std::os::raw::c_int)
    }
    if (*ctxt).urlTab.is_null() {
        (*ctxt).urlMax = 4 as std::os::raw::c_int;
        (*ctxt).urlNr = 0 as std::os::raw::c_int;
        (*ctxt).urlTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).urlMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut *mut xmlChar;
        if (*ctxt).urlTab.is_null() {
            xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                                 b"adding URL\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if (*ctxt).urlNr >= (*ctxt).urlMax {
        (*ctxt).urlMax *= 2 as std::os::raw::c_int;
        (*ctxt).urlTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).urlTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).urlMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut *mut xmlChar;
        if (*ctxt).urlTab.is_null() {
            xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                                 b"adding URL\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    let ref mut fresh2 = *(*ctxt).urlTab.offset((*ctxt).urlNr as isize);
    *fresh2 = xmlStrdup(value);
    (*ctxt).url = *fresh2;
    let fresh3 = (*ctxt).urlNr;
    (*ctxt).urlNr = (*ctxt).urlNr + 1;
    return fresh3;
}
/* *
 * xmlXIncludeURLPop:
 * @ctxt: the parser context
 *
 * Pops the top URL from the URL stack
 */
unsafe extern "C" fn xmlXIncludeURLPop(mut ctxt: xmlXIncludeCtxtPtr) {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if (*ctxt).urlNr <= 0 as std::os::raw::c_int { return }
    (*ctxt).urlNr -= 1;
    if (*ctxt).urlNr > 0 as std::os::raw::c_int {
        (*ctxt).url =
            *(*ctxt).urlTab.offset(((*ctxt).urlNr - 1 as std::os::raw::c_int) as
                                       isize)
    } else { (*ctxt).url = 0 as *mut xmlChar }
    ret = *(*ctxt).urlTab.offset((*ctxt).urlNr as isize);
    let ref mut fresh4 = *(*ctxt).urlTab.offset((*ctxt).urlNr as isize);
    *fresh4 = 0 as *mut xmlChar;
    if !ret.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
    };
}
/* *
 * xmlXIncludeFreeContext:
 * @ctxt: the XInclude context
 *
 * Free an XInclude context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeFreeContext(mut ctxt:
                                                    xmlXIncludeCtxtPtr) {
    let mut i: std::os::raw::c_int = 0;
    if ctxt.is_null() { return }
    while (*ctxt).urlNr > 0 as std::os::raw::c_int { xmlXIncludeURLPop(ctxt); }
    if !(*ctxt).urlTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).urlTab as
                                                        *mut std::os::raw::c_void);
    }
    i = 0 as std::os::raw::c_int;
    while i < (*ctxt).incNr {
        if !(*(*ctxt).incTab.offset(i as isize)).is_null() {
            xmlXIncludeFreeRef(*(*ctxt).incTab.offset(i as isize));
        }
        i += 1
    }
    if !(*ctxt).txturlTab.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < (*ctxt).txtNr {
            if !(*(*ctxt).txturlTab.offset(i as isize)).is_null() {
                xmlFree.expect("non-null function pointer")(*(*ctxt).txturlTab.offset(i
                                                                                          as
                                                                                          isize)
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            i += 1
        }
    }
    if !(*ctxt).incTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).incTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).txtTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).txtTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).txturlTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).txturlTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).base.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).base as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/* *
 * xmlXIncludeParseFile:
 * @ctxt:  the XInclude context
 * @URL:  the URL or file path
 *
 * parse a document for XInclude
 */
unsafe extern "C" fn xmlXIncludeParseFile(mut ctxt: xmlXIncludeCtxtPtr,
                                          mut URL: *const std::os::raw::c_char)
 -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut pctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    xmlInitParser();
    pctxt = xmlNewParserCtxt();
    if pctxt.is_null() {
        xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                             b"cannot allocate parser context\x00" as
                                 *const u8 as *const std::os::raw::c_char);
        return 0 as xmlDocPtr
    }
    /*
     * pass in the application data to the parser context.
     */
    (*pctxt)._private = (*ctxt)._private;
    /*
     * try to ensure that new documents included are actually
     * built with the same dictionary as the including document.
     */
    if !(*ctxt).doc.is_null() && !(*(*ctxt).doc).dict.is_null() {
        if !(*pctxt).dict.is_null() { xmlDictFree((*pctxt).dict); }
        (*pctxt).dict = (*(*ctxt).doc).dict;
        xmlDictReference((*pctxt).dict);
    }
    xmlCtxtUseOptions(pctxt,
                      (*ctxt).parseFlags | XML_PARSE_DTDLOAD as std::os::raw::c_int);
    inputStream = xmlLoadExternalEntity(URL, 0 as *const std::os::raw::c_char, pctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt(pctxt);
        return 0 as xmlDocPtr
    }
    inputPush(pctxt, inputStream);
    if (*pctxt).directory.is_null() {
        (*pctxt).directory = xmlParserGetDirectory(URL)
    }
    (*pctxt).loadsubset |= 2 as std::os::raw::c_int;
    xmlParseDocument(pctxt);
    if (*pctxt).wellFormed != 0 {
        ret = (*pctxt).myDoc
    } else {
        ret = 0 as xmlDocPtr;
        if !(*pctxt).myDoc.is_null() { xmlFreeDoc((*pctxt).myDoc); }
        (*pctxt).myDoc = 0 as xmlDocPtr
    }
    xmlFreeParserCtxt(pctxt);
    return ret;
}
/* *
 * xmlXIncludeAddNode:
 * @ctxt:  the XInclude context
 * @cur:  the new node
 *
 * Add a new node to process to an XInclude context
 */
unsafe extern "C" fn xmlXIncludeAddNode(mut ctxt: xmlXIncludeCtxtPtr,
                                        mut cur: xmlNodePtr) -> std::os::raw::c_int {
    let mut ref_0: xmlXIncludeRefPtr =
        0 as *mut xmlXIncludeRef; /* default Issue 64 */
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut fragment: *mut xmlChar = 0 as *mut xmlChar;
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut parse: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut xml: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut local: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if cur.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * read the attributes
     */
    href =
        xmlXIncludeGetProp(ctxt, cur,
                           b"href\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar); /* @@@@ href is now optional */
    if href.is_null() {
        href =
            xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        if href.is_null() { return -(1 as std::os::raw::c_int) }
    }
    if *href.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '#' as i32 ||
           *href.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        local = 1 as std::os::raw::c_int
    }
    parse =
        xmlXIncludeGetProp(ctxt, cur,
                           b"parse\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar);
    if !parse.is_null() {
        if xmlStrEqual(parse,
                       b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 {
            xml = 1 as std::os::raw::c_int
        } else if xmlStrEqual(parse,
                              b"text\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 {
            xml = 0 as std::os::raw::c_int
        } else {
            xmlXIncludeErr(ctxt, cur, XML_XINCLUDE_PARSE_VALUE as std::os::raw::c_int,
                           b"invalid value %s for \'parse\'\n\x00" as
                               *const u8 as *const std::os::raw::c_char, parse);
            if !href.is_null() {
                xmlFree.expect("non-null function pointer")(href as
                                                                *mut std::os::raw::c_void);
            }
            if !parse.is_null() {
                xmlFree.expect("non-null function pointer")(parse as
                                                                *mut std::os::raw::c_void);
            }
            return -(1 as std::os::raw::c_int)
        }
    }
    /*
     * compute the URI
     */
    base =
        xmlNodeGetBase((*ctxt).doc as *const xmlDoc, cur as *const xmlNode);
    if base.is_null() {
        URI = xmlBuildURI(href, (*(*ctxt).doc).URL)
    } else { URI = xmlBuildURI(href, base) }
    if URI.is_null() {
        let mut escbase: *mut xmlChar = 0 as *mut xmlChar;
        let mut eschref: *mut xmlChar = 0 as *mut xmlChar;
        /*
	 * Some escaping may be needed
	 */
        escbase = xmlURIEscape(base);
        eschref = xmlURIEscape(href);
        URI = xmlBuildURI(eschref, escbase);
        if !escbase.is_null() {
            xmlFree.expect("non-null function pointer")(escbase as
                                                            *mut std::os::raw::c_void);
        }
        if !eschref.is_null() {
            xmlFree.expect("non-null function pointer")(eschref as
                                                            *mut std::os::raw::c_void);
        }
    }
    if !parse.is_null() {
        xmlFree.expect("non-null function pointer")(parse as
                                                        *mut std::os::raw::c_void);
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as
                                                        *mut std::os::raw::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as
                                                        *mut std::os::raw::c_void);
    }
    if URI.is_null() {
        xmlXIncludeErr(ctxt, cur, XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"failed build URL\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    fragment =
        xmlXIncludeGetProp(ctxt, cur,
                           b"xpointer\x00" as *const u8 as *const std::os::raw::c_char
                               as *const xmlChar);
    /*
     * Check the URL and remove any fragment identifier
     */
    uri = xmlParseURI(URI as *const std::os::raw::c_char);
    if uri.is_null() {
        xmlXIncludeErr(ctxt, cur, XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"invalid value URI %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, URI);
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
        }
        xmlFree.expect("non-null function pointer")(URI as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    if !(*uri).fragment.is_null() {
        if (*ctxt).legacy != 0 as std::os::raw::c_int {
            if fragment.is_null() {
                fragment = (*uri).fragment as *mut xmlChar
            } else {
                xmlFree.expect("non-null function pointer")((*uri).fragment as
                                                                *mut std::os::raw::c_void);
            }
        } else {
            xmlXIncludeErr(ctxt, cur, XML_XINCLUDE_FRAGMENT_ID as std::os::raw::c_int,
                           b"Invalid fragment identifier in URI %s use the xpointer attribute\n\x00"
                               as *const u8 as *const std::os::raw::c_char, URI);
            if !fragment.is_null() {
                xmlFree.expect("non-null function pointer")(fragment as
                                                                *mut std::os::raw::c_void);
            }
            xmlFreeURI(uri);
            xmlFree.expect("non-null function pointer")(URI as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        (*uri).fragment = 0 as *mut std::os::raw::c_char
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    xmlFree.expect("non-null function pointer")(URI as *mut std::os::raw::c_void);
    if URL.is_null() {
        xmlXIncludeErr(ctxt, cur, XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"invalid value URI %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, URI);
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
        }
        return -(1 as std::os::raw::c_int)
    }
    /*
     * If local and xml then we need a fragment
     */
    if local == 1 as std::os::raw::c_int && xml == 1 as std::os::raw::c_int &&
           (fragment.is_null() ||
                *fragment.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                    0 as std::os::raw::c_int) {
        xmlXIncludeErr(ctxt, cur, XML_XINCLUDE_RECURSION as std::os::raw::c_int,
                       b"detected a local recursion with no xpointer in %s\n\x00"
                           as *const u8 as *const std::os::raw::c_char, URL);
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
        }
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Check the URL against the stack for recursions
     */
    if local == 0 && xml == 1 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int;
        while i < (*ctxt).urlNr {
            if xmlStrEqual(URL, *(*ctxt).urlTab.offset(i as isize)) != 0 {
                xmlXIncludeErr(ctxt, cur,
                               XML_XINCLUDE_RECURSION as std::os::raw::c_int,
                               b"detected a recursion in %s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char, URL);
                return -(1 as std::os::raw::c_int)
            }
            i += 1
        }
    }
    ref_0 = xmlXIncludeNewRef(ctxt, URL, cur);
    if ref_0.is_null() { return -(1 as std::os::raw::c_int) }
    (*ref_0).fragment = fragment;
    (*ref_0).doc = 0 as xmlDocPtr;
    (*ref_0).xml = xml;
    (*ref_0).count = 1 as std::os::raw::c_int;
    xmlFree.expect("non-null function pointer")(URL as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeRecurseDoc:
 * @ctxt:  the XInclude context
 * @doc:  the new document
 * @url:  the associated URL
 *
 * The XInclude recursive nature is handled at this point.
 */
unsafe extern "C" fn xmlXIncludeRecurseDoc(mut ctxt: xmlXIncludeCtxtPtr,
                                           mut doc: xmlDocPtr, url: xmlURL) {
    let mut newctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut i: std::os::raw::c_int = 0;
    /*
     * Avoid recursion in already substitued resources
    for (i = 0;i < ctxt->urlNr;i++) {
	if (xmlStrEqual(doc->URL, ctxt->urlTab[i]))
	    return;
    }
     */
    /*
     * Handle recursion here.
     */
    newctxt = xmlXIncludeNewContext(doc);
    if !newctxt.is_null() {
        /*
	 * Copy the private user data
	 */
        (*newctxt)._private = (*ctxt)._private;
        /*
	 * Copy the existing document set
	 */
        (*newctxt).incMax = (*ctxt).incMax;
        (*newctxt).incNr = (*ctxt).incNr;
        (*newctxt).incTab =
            xmlMalloc.expect("non-null function pointer")(((*newctxt).incMax
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXIncludeRefPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlXIncludeRefPtr;
        if (*newctxt).incTab.is_null() {
            xmlXIncludeErrMemory(ctxt, doc as xmlNodePtr,
                                 b"processing doc\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(newctxt as
                                                            *mut std::os::raw::c_void);
            return
        }
        /*
	 * copy the urlTab
	 */
        (*newctxt).urlMax = (*ctxt).urlMax;
        (*newctxt).urlNr = (*ctxt).urlNr;
        (*newctxt).urlTab = (*ctxt).urlTab;
        /*
	 * Inherit the existing base
	 */
        (*newctxt).base = xmlStrdup((*ctxt).base);
        /*
	 * Inherit the documents already in use by other includes
	 */
        (*newctxt).incBase = (*ctxt).incNr;
        i = 0 as std::os::raw::c_int;
        while i < (*ctxt).incNr {
            let ref mut fresh5 = *(*newctxt).incTab.offset(i as isize);
            *fresh5 = *(*ctxt).incTab.offset(i as isize);
            let ref mut fresh6 =
                (**(*newctxt).incTab.offset(i as isize)).count;
            *fresh6 += 1;
            i += 1
            /* prevent the recursion from
					    freeing it */
        }
        /*
	 * The new context should also inherit the Parse Flags
	 * (bug 132597)
	 */
        (*newctxt).parseFlags = (*ctxt).parseFlags;
        xmlXIncludeDoProcess(newctxt, doc,
                             xmlDocGetRootElement(doc as *const xmlDoc));
        i = 0 as std::os::raw::c_int;
        while i < (*ctxt).incNr {
            let ref mut fresh7 =
                (**(*newctxt).incTab.offset(i as isize)).count;
            *fresh7 -= 1;
            let ref mut fresh8 = *(*newctxt).incTab.offset(i as isize);
            *fresh8 = 0 as xmlXIncludeRefPtr;
            i += 1
        }
        /* urlTab may have been reallocated */
        (*ctxt).urlTab = (*newctxt).urlTab;
        (*ctxt).urlMax = (*newctxt).urlMax;
        (*newctxt).urlMax = 0 as std::os::raw::c_int;
        (*newctxt).urlNr = 0 as std::os::raw::c_int;
        (*newctxt).urlTab = 0 as *mut *mut xmlChar;
        xmlXIncludeFreeContext(newctxt);
    };
}
/* *
 * xmlXIncludeAddTxt:
 * @ctxt:  the XInclude context
 * @txt:  the new text node
 * @url:  the associated URL
 *
 * Add a new txtument to the list
 */
unsafe extern "C" fn xmlXIncludeAddTxt(mut ctxt: xmlXIncludeCtxtPtr,
                                       mut txt: xmlNodePtr, url: xmlURL) {
    if (*ctxt).txtMax == 0 as std::os::raw::c_int {
        (*ctxt).txtMax = 4 as std::os::raw::c_int;
        (*ctxt).txtTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).txtMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if (*ctxt).txtTab.is_null() {
            xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                                 b"processing text\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            return
        }
        (*ctxt).txturlTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).txtMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlURL>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlURL;
        if (*ctxt).txturlTab.is_null() {
            xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                                 b"processing text\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            return
        }
    }
    if (*ctxt).txtNr >= (*ctxt).txtMax {
        (*ctxt).txtMax *= 2 as std::os::raw::c_int;
        (*ctxt).txtTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).txtTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).txtMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if (*ctxt).txtTab.is_null() {
            xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                                 b"processing text\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            return
        }
        (*ctxt).txturlTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).txturlTab
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).txtMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlURL>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlURL;
        if (*ctxt).txturlTab.is_null() {
            xmlXIncludeErrMemory(ctxt, 0 as xmlNodePtr,
                                 b"processing text\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            return
        }
    }
    let ref mut fresh9 = *(*ctxt).txtTab.offset((*ctxt).txtNr as isize);
    *fresh9 = txt;
    let ref mut fresh10 = *(*ctxt).txturlTab.offset((*ctxt).txtNr as isize);
    *fresh10 = xmlStrdup(url as *const xmlChar);
    (*ctxt).txtNr += 1;
}
/* *
 * xmlXIncludeCopyNode:
 * @ctxt:  the XInclude context
 * @target:  the document target
 * @source:  the document source
 * @elem:  the element
 *
 * Make a copy of the node while preserving the XInclude semantic
 * of the Infoset copy
 */
unsafe extern "C" fn xmlXIncludeCopyNode(mut ctxt: xmlXIncludeCtxtPtr,
                                         mut target: xmlDocPtr,
                                         mut source: xmlDocPtr,
                                         mut elem: xmlNodePtr) -> xmlNodePtr {
    let mut result: xmlNodePtr = 0 as xmlNodePtr;
    if ctxt.is_null() || target.is_null() || source.is_null() ||
           elem.is_null() {
        return 0 as xmlNodePtr
    }
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        result =
            xmlXIncludeCopyNodeList(ctxt, target, source, (*elem).children)
    } else { result = xmlDocCopyNode(elem, target, 1 as std::os::raw::c_int) }
    return result;
}
/* ***********************************************************************
 *									*
 *			Node copy with specific semantic		*
 *									*
 ************************************************************************/
/* *
 * xmlXIncludeCopyNodeList:
 * @ctxt:  the XInclude context
 * @target:  the document target
 * @source:  the document source
 * @elem:  the element list
 *
 * Make a copy of the node list while preserving the XInclude semantic
 * of the Infoset copy
 */
unsafe extern "C" fn xmlXIncludeCopyNodeList(mut ctxt: xmlXIncludeCtxtPtr,
                                             mut target: xmlDocPtr,
                                             mut source: xmlDocPtr,
                                             mut elem: xmlNodePtr)
 -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    let mut result: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    if ctxt.is_null() || target.is_null() || source.is_null() ||
           elem.is_null() {
        return 0 as xmlNodePtr
    }
    cur = elem;
    while !cur.is_null() {
        res = xmlXIncludeCopyNode(ctxt, target, source, cur);
        if !res.is_null() {
            if result.is_null() {
                last = res;
                result = last
            } else { (*last).next = res; (*res).prev = last; last = res }
        }
        cur = (*cur).next
    }
    return result;
}
/* *
 * xmlXIncludeGetNthChild:
 * @cur:  the node
 * @no:  the child number
 *
 * Returns the @n'th element child of @cur or NULL
 */
unsafe extern "C" fn xmlXIncludeGetNthChild(mut cur: xmlNodePtr,
                                            mut no: std::os::raw::c_int)
 -> xmlNodePtr {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    cur = (*cur).children;
    i = 0 as std::os::raw::c_int;
    while i <= no {
        if cur.is_null() { return cur }
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            i += 1;
            if i == no { break ; }
        }
        cur = (*cur).next
    }
    return cur;
}
/* in xpointer.c */
/* *
 * xmlXIncludeCopyRange:
 * @ctxt:  the XInclude context
 * @target:  the document target
 * @source:  the document source
 * @obj:  the XPointer result from the evaluation.
 *
 * Build a node list tree copy of the XPointer result.
 *
 * Returns an xmlNodePtr list or NULL.
 *         The caller has to free the node tree.
 */
unsafe extern "C" fn xmlXIncludeCopyRange(mut ctxt: xmlXIncludeCtxtPtr,
                                          mut target: xmlDocPtr,
                                          mut source: xmlDocPtr,
                                          mut range: xmlXPathObjectPtr)
 -> xmlNodePtr {
    /* pointers to generated nodes */
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut listParent: xmlNodePtr = 0 as xmlNodePtr;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp2: xmlNodePtr = 0 as *mut xmlNode;
    /* pointers to traversal nodes */
    let mut start: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut end: xmlNodePtr = 0 as *mut xmlNode;
    let mut index1: std::os::raw::c_int = 0;
    let mut index2: std::os::raw::c_int = 0;
    let mut level: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut lastLevel: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut endLevel: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut endFlag: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctxt.is_null() || target.is_null() || source.is_null() ||
           range.is_null() {
        return 0 as xmlNodePtr
    }
    if (*range).type_0 as std::os::raw::c_uint !=
           XPATH_RANGE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    start = (*range).user as xmlNodePtr;
    if start.is_null() ||
           (*start).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    end = (*range).user2 as xmlNodePtr;
    if end.is_null() {
        return xmlDocCopyNode(start, target, 1 as std::os::raw::c_int)
    }
    if (*end).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    cur = start;
    index1 = (*range).index;
    index2 = (*range).index2;
    /*
     * level is depth of the current node under consideration
     * list is the pointer to the root of the output tree
     * listParent is a pointer to the parent of output tree (within
       the included file) in case we need to add another level
     * last is a pointer to the last node added to the output tree
     * lastLevel is the depth of last (relative to the root)
     */
    while !cur.is_null() {
        /*
	 * Check if our output tree needs a parent
	 */
        if level < 0 as std::os::raw::c_int {
            while level < 0 as std::os::raw::c_int {
                /* copy must include namespaces and properties */
                tmp2 = xmlDocCopyNode(listParent, target, 2 as std::os::raw::c_int);
                xmlAddChild(tmp2, list);
                list = tmp2;
                listParent = (*listParent).parent;
                level += 1
            }
            last = list;
            lastLevel = 0 as std::os::raw::c_int
        }
        /*
	 * Check whether we need to change our insertion point
	 */
        while level < lastLevel { last = (*last).parent; lastLevel -= 1 }
        if cur == end {
            /* Are we at the end of the range? */
            if (*cur).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                let mut content: *const xmlChar =
                    (*cur).content; /* ending node not a text node */
                let mut len: std::os::raw::c_int = 0;
                if content.is_null() {
                    tmp = xmlNewTextLen(0 as *const xmlChar, 0 as std::os::raw::c_int)
                } else {
                    len = index2;
                    if cur == start && index1 > 1 as std::os::raw::c_int {
                        content =
                            content.offset((index1 - 1 as std::os::raw::c_int) as
                                               isize);
                        len -= index1 - 1 as std::os::raw::c_int
                    } else { len = index2 }
                    tmp = xmlNewTextLen(content, len)
                }
                /* single sub text node selection */
                if list.is_null() { return tmp }
                /* prune and return full set */
                if level == lastLevel {
                    xmlAddNextSibling(last,
                                      tmp); /* remember the level of the end node */
                } else { xmlAddChild(last, tmp); }
                return list
            } else {
                endLevel = level;
                endFlag = 1 as std::os::raw::c_int;
                /* while */
                /* last node - need to take care of properties + namespaces */
                tmp =
                    xmlDocCopyNode(cur, target,
                                   2 as
                                       std::os::raw::c_int); /* increment level to show change */
                if list.is_null() {
                    list = tmp;
                    listParent = (*cur).parent
                } else if level == lastLevel {
                    xmlAddNextSibling(last, tmp);
                } else { xmlAddChild(last, tmp); lastLevel = level }
                last = tmp;
                if index2 > 1 as std::os::raw::c_int {
                    end =
                        xmlXIncludeGetNthChild(cur,
                                               index2 - 1 as std::os::raw::c_int);
                    index2 = 0 as std::os::raw::c_int
                }
                if cur == start && index1 > 1 as std::os::raw::c_int {
                    cur =
                        xmlXIncludeGetNthChild(cur,
                                               index1 - 1 as std::os::raw::c_int);
                    index1 = 0 as std::os::raw::c_int
                } else { cur = (*cur).children }
                level += 1
            }
        } else {
            if cur == start {
                /* Not at the end, are we at start? */
                if (*cur).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                    let mut content_0: *const xmlChar =
                        (*cur).content; /* Not text node */
                    if content_0.is_null() {
                        tmp =
                            xmlNewTextLen(0 as *const xmlChar,
                                          0 as std::os::raw::c_int)
                    } else {
                        if index1 > 1 as std::os::raw::c_int {
                            content_0 =
                                content_0.offset((index1 - 1 as std::os::raw::c_int)
                                                     as isize);
                            index1 = 0 as std::os::raw::c_int
                        }
                        tmp = xmlNewText(content_0)
                    }
                    list = tmp;
                    last = list;
                    listParent = (*cur).parent
                } else {
                    /*
		 * start of the range - need to take care of
		 * properties and namespaces
		 */
                    tmp = xmlDocCopyNode(cur, target, 2 as std::os::raw::c_int);
                    last = tmp;
                    list = last;
                    listParent = (*cur).parent;
                    if index1 > 1 as std::os::raw::c_int {
                        /* Do we need to position? */
                        cur =
                            xmlXIncludeGetNthChild(cur,
                                                   index1 - 1 as std::os::raw::c_int);
                        lastLevel = 1 as std::os::raw::c_int;
                        level = lastLevel;
                        index1 = 0 as std::os::raw::c_int;
                        /* while */
                        continue ;
                    }
                }
            } else {
                tmp = 0 as xmlNodePtr;
                match (*cur).type_0 as std::os::raw::c_uint {
                    14 | 15 | 16 | 6 => { }
                    17 => { }
                    19 | 20 => { }
                    2 => { }
                    _ => {
                        /*
		     * Now gather the remaining nodes from cur to end
		     */
                        /*
		     * Middle of the range - need to take care of
		     * properties and namespaces
		     */
                        tmp = xmlDocCopyNode(cur, target, 2 as std::os::raw::c_int)
                    }
                }
                if !tmp.is_null() {
                    if level == lastLevel {
                        xmlAddNextSibling(last, tmp);
                    } else { xmlAddChild(last, tmp); lastLevel = level }
                    last = tmp
                }
            }
            /*
	 * Skip to next node in document order
	 */
            cur = xmlXPtrAdvanceNode(cur, &mut level);
            if endFlag != 0 && level >= endLevel { break ; }
        }
    }
    return list;
}
/* *
 * xmlXIncludeBuildNodeList:
 * @ctxt:  the XInclude context
 * @target:  the document target
 * @source:  the document source
 * @obj:  the XPointer result from the evaluation.
 *
 * Build a node list tree copy of the XPointer result.
 * This will drop Attributes and Namespace declarations.
 *
 * Returns an xmlNodePtr list or NULL.
 *         the caller has to free the node tree.
 */
unsafe extern "C" fn xmlXIncludeCopyXPointer(mut ctxt: xmlXIncludeCtxtPtr,
                                             mut target: xmlDocPtr,
                                             mut source: xmlDocPtr,
                                             mut obj: xmlXPathObjectPtr)
 -> xmlNodePtr {
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut i: std::os::raw::c_int = 0;
    if source.is_null() { source = (*ctxt).doc }
    if ctxt.is_null() || target.is_null() || source.is_null() || obj.is_null()
       {
        return 0 as xmlNodePtr
    }
    match (*obj).type_0 as std::os::raw::c_uint {
        1 => {
            let mut set: xmlNodeSetPtr = (*obj).nodesetval;
            if set.is_null() { return 0 as xmlNodePtr }
            let mut current_block_22: u64;
            i = 0 as std::os::raw::c_int;
            while i < (*set).nodeNr {
                if !(*(*set).nodeTab.offset(i as isize)).is_null() {
                    match (**(*set).nodeTab.offset(i as isize)).type_0 as
                              std::os::raw::c_uint {
                        19 => {
                            current_block_22 = 8236137900636309791;
                            match current_block_22 {
                                18153031941552419006 => {
                                    if last.is_null() {
                                        last =
                                            xmlXIncludeCopyNode(ctxt, target,
                                                                source,
                                                                *(*set).nodeTab.offset(i
                                                                                           as
                                                                                           isize));
                                        list = last
                                    } else {
                                        xmlAddNextSibling(last,
                                                          xmlXIncludeCopyNode(ctxt,
                                                                              target,
                                                                              source,
                                                                              *(*set).nodeTab.offset(i
                                                                                                         as
                                                                                                         isize)));
                                        if !(*last).next.is_null() {
                                            last = (*last).next
                                        }
                                    }
                                    /* for */
                                }
                                _ => {
                                    let mut tmp: xmlNodePtr =
                                        0 as *mut xmlNode;
                                    let mut cur: xmlNodePtr =
                                        *(*set).nodeTab.offset(i as isize);
                                    cur = (*cur).next;
                                    while !cur.is_null() {
                                        match (*cur).type_0 as std::os::raw::c_uint {
                                            3 | 4 | 1 | 5 | 6 | 7 | 8 => { }
                                            _ => { break ; }
                                        }
                                        tmp =
                                            xmlXIncludeCopyNode(ctxt, target,
                                                                source, cur);
                                        if last.is_null() {
                                            last = tmp;
                                            list = last
                                        } else {
                                            xmlAddNextSibling(last, tmp);
                                            last = tmp
                                        }
                                        cur = (*cur).next
                                    }
                                }
                            }
                        }
                        2 | 18 | 10 | 11 | 12 | 14 | 15 | 16 | 17 => { }
                        3 | 4 | 1 | 5 | 6 | 7 | 8 | 9 | 13 | 21 | 20 | _ => {
                            current_block_22 = 18153031941552419006;
                            match current_block_22 {
                                18153031941552419006 => {
                                    if last.is_null() {
                                        last =
                                            xmlXIncludeCopyNode(ctxt, target,
                                                                source,
                                                                *(*set).nodeTab.offset(i
                                                                                           as
                                                                                           isize));
                                        list = last
                                    } else {
                                        xmlAddNextSibling(last,
                                                          xmlXIncludeCopyNode(ctxt,
                                                                              target,
                                                                              source,
                                                                              *(*set).nodeTab.offset(i
                                                                                                         as
                                                                                                         isize)));
                                        if !(*last).next.is_null() {
                                            last = (*last).next
                                        }
                                    }
                                }
                                _ => {
                                    let mut tmp: xmlNodePtr =
                                        0 as *mut xmlNode;
                                    let mut cur: xmlNodePtr =
                                        *(*set).nodeTab.offset(i as isize);
                                    cur = (*cur).next;
                                    while !cur.is_null() {
                                        match (*cur).type_0 as std::os::raw::c_uint {
                                            3 | 4 | 1 | 5 | 6 | 7 | 8 => { }
                                            _ => { break ; }
                                        }
                                        tmp =
                                            xmlXIncludeCopyNode(ctxt, target,
                                                                source, cur);
                                        if last.is_null() {
                                            last = tmp;
                                            list = last
                                        } else {
                                            xmlAddNextSibling(last, tmp);
                                            last = tmp
                                        }
                                        cur = (*cur).next
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1
            }
        }
        7 => {
            let mut set_0: xmlLocationSetPtr =
                (*obj).user as xmlLocationSetPtr;
            if set_0.is_null() { return 0 as xmlNodePtr }
            i = 0 as std::os::raw::c_int;
            while i < (*set_0).locNr {
                if last.is_null() {
                    last =
                        xmlXIncludeCopyXPointer(ctxt, target, source,
                                                *(*set_0).locTab.offset(i as
                                                                            isize));
                    list = last
                } else {
                    xmlAddNextSibling(last,
                                      xmlXIncludeCopyXPointer(ctxt, target,
                                                              source,
                                                              *(*set_0).locTab.offset(i
                                                                                          as
                                                                                          isize)));
                }
                if !last.is_null() {
                    while !(*last).next.is_null() { last = (*last).next }
                }
                i += 1
            }
        }
        6 => { return xmlXIncludeCopyRange(ctxt, target, source, obj) }
        5 | _ => { }
    }
    return list;
}
/* *
 * xmlXIncludeMergeOneEntity:
 * @ent: the entity
 * @doc:  the including doc
 * @nr: the entity name
 *
 * Inplements the merge of one entity
 */
unsafe extern "C" fn xmlXIncludeMergeEntity(mut payload: *mut std::os::raw::c_void,
                                            mut vdata: *mut std::os::raw::c_void,
                                            mut name: *const xmlChar) {
    let mut current_block: u64;
    let mut ent: xmlEntityPtr = payload as xmlEntityPtr;
    let mut data: xmlXIncludeMergeDataPtr = vdata as xmlXIncludeMergeDataPtr;
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut prev: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    if ent.is_null() || data.is_null() { return }
    ctxt = (*data).ctxt;
    doc = (*data).doc;
    if ctxt.is_null() || doc.is_null() { return }
    match (*ent).etype as std::os::raw::c_uint {
        4 | 5 | 6 => { return }
        1 | 2 | 3 | _ => { }
    }
    ret =
        xmlAddDocEntity(doc, (*ent).name, (*ent).etype as std::os::raw::c_int,
                        (*ent).ExternalID, (*ent).SystemID, (*ent).content);
    if !ret.is_null() {
        if !(*ent).URI.is_null() { (*ret).URI = xmlStrdup((*ent).URI) }
    } else {
        prev = xmlGetDocEntity(doc as *const xmlDoc, (*ent).name);
        if !prev.is_null() {
            if (*ent).etype as std::os::raw::c_uint != (*prev).etype as std::os::raw::c_uint {
                current_block = 7179440311677010569;
            } else if !(*ent).SystemID.is_null() &&
                          !(*prev).SystemID.is_null() {
                if xmlStrEqual((*ent).SystemID, (*prev).SystemID) == 0 {
                    current_block = 7179440311677010569;
                } else { current_block = 4488286894823169796; }
            } else if !(*ent).ExternalID.is_null() &&
                          !(*prev).ExternalID.is_null() {
                if xmlStrEqual((*ent).ExternalID, (*prev).ExternalID) == 0 {
                    current_block = 7179440311677010569;
                } else { current_block = 4488286894823169796; }
            } else if !(*ent).content.is_null() && !(*prev).content.is_null()
             {
                if xmlStrEqual((*ent).content, (*prev).content) == 0 {
                    current_block = 7179440311677010569;
                } else { current_block = 4488286894823169796; }
            } else { current_block = 7179440311677010569; }
            match current_block {
                4488286894823169796 => { }
                _ => {
                    match (*ent).etype as std::os::raw::c_uint {
                        4 | 5 | 6 | 1 | 2 => { return }
                        3 | _ => { }
                    }
                    xmlXIncludeErr(ctxt, ent as xmlNodePtr,
                                   XML_XINCLUDE_ENTITY_DEF_MISMATCH as
                                       std::os::raw::c_int,
                                   b"mismatch in redefinition of entity %s\n\x00"
                                       as *const u8 as *const std::os::raw::c_char,
                                   (*ent).name);
                    return;
                }
            }
        }
    };
}
/* *
 * xmlXIncludeMergeEntities:
 * @ctxt: an XInclude context
 * @doc:  the including doc
 * @from:  the included doc
 *
 * Inplements the entity merge
 *
 * Returns 0 if merge succeeded, -1 if some processing failed
 */
unsafe extern "C" fn xmlXIncludeMergeEntities(mut ctxt: xmlXIncludeCtxtPtr,
                                              mut doc: xmlDocPtr,
                                              mut from: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut target: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut source: xmlDtdPtr = 0 as *mut xmlDtd;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if from.is_null() || (*from).intSubset.is_null() {
        return 0 as std::os::raw::c_int
    }
    target = (*doc).intSubset;
    if target.is_null() {
        cur = xmlDocGetRootElement(doc as *const xmlDoc);
        if cur.is_null() { return -(1 as std::os::raw::c_int) }
        target =
            xmlCreateIntSubset(doc, (*cur).name, 0 as *const xmlChar,
                               0 as *const xmlChar);
        if target.is_null() { return -(1 as std::os::raw::c_int) }
    }
    source = (*from).intSubset;
    if !source.is_null() && !(*source).entities.is_null() {
        let mut data: xmlXIncludeMergeData =
            xmlXIncludeMergeData{doc: 0 as *mut xmlDoc,
                                 ctxt: 0 as *mut xmlXIncludeCtxt,};
        data.ctxt = ctxt;
        data.doc = doc;
        xmlHashScan((*source).entities as xmlHashTablePtr,
                    Some(xmlXIncludeMergeEntity as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    &mut data as *mut xmlXIncludeMergeData as
                        *mut std::os::raw::c_void);
    }
    source = (*from).extSubset;
    if !source.is_null() && !(*source).entities.is_null() {
        let mut data_0: xmlXIncludeMergeData =
            xmlXIncludeMergeData{doc: 0 as *mut xmlDoc,
                                 ctxt: 0 as *mut xmlXIncludeCtxt,};
        data_0.ctxt = ctxt;
        data_0.doc = doc;
        /*
	 * don't duplicate existing stuff when external subsets are the same
	 */
        if xmlStrEqual((*target).ExternalID, (*source).ExternalID) == 0 &&
               xmlStrEqual((*target).SystemID, (*source).SystemID) == 0 {
            xmlHashScan((*source).entities as xmlHashTablePtr,
                        Some(xmlXIncludeMergeEntity as
                                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                      _: *mut std::os::raw::c_void,
                                                      _: *const xmlChar)
                                     -> ()),
                        &mut data_0 as *mut xmlXIncludeMergeData as
                            *mut std::os::raw::c_void);
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeLoadDoc:
 * @ctxt:  the XInclude context
 * @url:  the associated URL
 * @nr:  the xinclude node number
 *
 * Load the document, and store the result in the XInclude context
 *
 * Returns 0 in case of success, -1 in case of failure
 */
unsafe extern "C" fn xmlXIncludeLoadDoc(mut ctxt: xmlXIncludeCtxtPtr,
                                        mut url: *const xmlChar,
                                        mut nr: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut fragment: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut saveFlags: std::os::raw::c_int = 0;
    /*
     * Check the URL and remove any fragment identifier
     */
    uri = xmlParseURI(url as *const std::os::raw::c_char);
    if uri.is_null() {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"invalid value URI %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, url);
        return -(1 as std::os::raw::c_int)
    }
    if !(*uri).fragment.is_null() {
        fragment = (*uri).fragment as *mut xmlChar;
        (*uri).fragment = 0 as *mut std::os::raw::c_char
    }
    if !(*ctxt).incTab.is_null() &&
           !(*(*ctxt).incTab.offset(nr as isize)).is_null() &&
           !(**(*ctxt).incTab.offset(nr as isize)).fragment.is_null() {
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
        }
        fragment = xmlStrdup((**(*ctxt).incTab.offset(nr as isize)).fragment)
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    if URL.is_null() {
        if !(*ctxt).incTab.is_null() {
            xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                           XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                           b"invalid value URI %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, url);
        } else {
            xmlXIncludeErr(ctxt, 0 as xmlNodePtr,
                           XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                           b"invalid value URI %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, url);
        }
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
        }
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Handling of references to the local document are done
     * directly through ctxt->doc.
     */
    if *URL.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0 as std::os::raw::c_int ||
           *URL.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '#' as i32
           ||
           !(*ctxt).doc.is_null() && xmlStrEqual(URL, (*(*ctxt).doc).URL) != 0
       {
        doc = 0 as xmlDocPtr
    } else {
        /*
     * Prevent reloading twice the document.
     */
        i = 0 as std::os::raw::c_int;
        loop  {
            if !(i < (*ctxt).incNr) {
                current_block = 2891135413264362348;
                break ;
            }
            if xmlStrEqual(URL, (**(*ctxt).incTab.offset(i as isize)).URI) !=
                   0 && !(**(*ctxt).incTab.offset(i as isize)).doc.is_null() {
                doc = (**(*ctxt).incTab.offset(i as isize)).doc;
                current_block = 17990283814522870977;
                break ;
            } else { i += 1 }
        }
        match current_block {
            17990283814522870977 => { }
            _ => {
                /*
     * Load it.
     */
                /*
     * If this is an XPointer evaluation, we want to assure that
     * all entities have been resolved prior to processing the
     * referenced document
     */
                saveFlags = (*ctxt).parseFlags;
                if !fragment.is_null() {
                    /* if this is an XPointer eval */
                    (*ctxt).parseFlags |= XML_PARSE_NOENT as std::os::raw::c_int
                }
                doc = xmlXIncludeParseFile(ctxt, URL as *const std::os::raw::c_char);
                (*ctxt).parseFlags = saveFlags;
                if doc.is_null() {
                    xmlFree.expect("non-null function pointer")(URL as
                                                                    *mut std::os::raw::c_void);
                    if !fragment.is_null() {
                        xmlFree.expect("non-null function pointer")(fragment
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    }
                    return -(1 as std::os::raw::c_int)
                }
                let ref mut fresh11 =
                    (**(*ctxt).incTab.offset(nr as isize)).doc;
                *fresh11 = doc;
                /*
     * It's possible that the requested URL has been mapped to a
     * completely different location (e.g. through a catalog entry).
     * To check for this, we compare the URL with that of the doc
     * and change it if they disagree (bug 146988).
     */
                if xmlStrEqual(URL, (*doc).URL) == 0 {
                    xmlFree.expect("non-null function pointer")(URL as
                                                                    *mut std::os::raw::c_void);
                    URL = xmlStrdup((*doc).URL)
                }
                i = nr + 1 as std::os::raw::c_int;
                while i < (*ctxt).incNr {
                    if xmlStrEqual(URL,
                                   (**(*ctxt).incTab.offset(i as isize)).URI)
                           != 0 {
                        let ref mut fresh12 =
                            (**(*ctxt).incTab.offset(nr as isize)).count;
                        *fresh12 += 1;
                        break ;
                    } else { i += 1 }
                }
                /*
     * Make sure we have all entities fixed up
     */
                xmlXIncludeMergeEntities(ctxt, (*ctxt).doc, doc);
                /*
     * We don't need the DTD anymore, free up space
    if (doc->intSubset != NULL) {
	xmlUnlinkNode((xmlNodePtr) doc->intSubset);
	xmlFreeNode((xmlNodePtr) doc->intSubset);
	doc->intSubset = NULL;
    }
    if (doc->extSubset != NULL) {
	xmlUnlinkNode((xmlNodePtr) doc->extSubset);
	xmlFreeNode((xmlNodePtr) doc->extSubset);
	doc->extSubset = NULL;
    }
     */
                xmlXIncludeRecurseDoc(ctxt, doc, URL);
            }
        }
    }
    if fragment.is_null() {
        /*
	 * Add the top children list as the replacement copy.
	 */
        if doc.is_null() {
            /* Hopefully a DTD declaration won't be copied from
	     * the same document */
            let ref mut fresh13 = (**(*ctxt).incTab.offset(nr as isize)).inc;
            *fresh13 = xmlCopyNodeList((*(*ctxt).doc).children)
        } else {
            let ref mut fresh14 = (**(*ctxt).incTab.offset(nr as isize)).inc;
            *fresh14 =
                xmlXIncludeCopyNodeList(ctxt, (*ctxt).doc, doc,
                                        (*doc).children)
        }
    } else {
        /*
	 * Computes the XPointer expression and make a copy used
	 * as the replacement copy.
	 */
        let mut xptr: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        let mut xptrctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
        let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        if doc.is_null() {
            xptrctxt =
                xmlXPtrNewContext((*ctxt).doc,
                                  (**(*ctxt).incTab.offset(nr as
                                                               isize)).ref_0,
                                  0 as xmlNodePtr)
        } else {
            xptrctxt =
                xmlXPtrNewContext(doc, 0 as xmlNodePtr, 0 as xmlNodePtr)
        }
        if xptrctxt.is_null() {
            xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                           XML_XINCLUDE_XPTR_FAILED as std::os::raw::c_int,
                           b"could not create XPointer context\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar);
            xmlFree.expect("non-null function pointer")(URL as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        xptr = xmlXPtrEval(fragment, xptrctxt);
        if xptr.is_null() {
            xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                           XML_XINCLUDE_XPTR_FAILED as std::os::raw::c_int,
                           b"XPointer evaluation failed: #%s\n\x00" as
                               *const u8 as *const std::os::raw::c_char, fragment);
            xmlXPathFreeContext(xptrctxt);
            xmlFree.expect("non-null function pointer")(URL as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(fragment as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        match (*xptr).type_0 as std::os::raw::c_uint {
            0 | 2 | 3 | 4 | 5 | 8 | 9 => {
                xmlXIncludeErr(ctxt,
                               (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                               XML_XINCLUDE_XPTR_RESULT as std::os::raw::c_int,
                               b"XPointer is not a range: #%s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               fragment);
                xmlXPathFreeContext(xptrctxt);
                xmlFree.expect("non-null function pointer")(URL as
                                                                *mut std::os::raw::c_void);
                xmlFree.expect("non-null function pointer")(fragment as
                                                                *mut std::os::raw::c_void);
                return -(1 as std::os::raw::c_int)
            }
            1 => {
                if (*xptr).nodesetval.is_null() ||
                       (*(*xptr).nodesetval).nodeNr <= 0 as std::os::raw::c_int {
                    xmlXPathFreeContext(xptrctxt);
                    xmlFree.expect("non-null function pointer")(URL as
                                                                    *mut std::os::raw::c_void);
                    xmlFree.expect("non-null function pointer")(fragment as
                                                                    *mut std::os::raw::c_void);
                    return -(1 as std::os::raw::c_int)
                }
            }
            6 | 7 | _ => { }
        }
        set = (*xptr).nodesetval;
        if !set.is_null() {
            let mut current_block_94: u64;
            i = 0 as std::os::raw::c_int;
            while i < (*set).nodeNr {
                if !(*(*set).nodeTab.offset(i as isize)).is_null() {
                    match (**(*set).nodeTab.offset(i as isize)).type_0 as
                              std::os::raw::c_uint {
                        2 => {
                            current_block_94 = 12530596945886496894;
                            match current_block_94 {
                                12530596945886496894 => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects an attribute: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh15 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh15 = 0 as xmlNodePtr
                                }
                                1910542799034262194 => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects a namespace: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh16 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh16 = 0 as xmlNodePtr
                                }
                                _ => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects unexpected nodes: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh17 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh17 = 0 as xmlNodePtr;
                                    let ref mut fresh18 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh18 = 0 as xmlNodePtr
                                }
                            }
                        }
                        18 => {
                            current_block_94 = 1910542799034262194;
                            match current_block_94 {
                                12530596945886496894 => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects an attribute: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh15 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh15 = 0 as xmlNodePtr
                                }
                                1910542799034262194 => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects a namespace: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh16 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh16 = 0 as xmlNodePtr
                                }
                                _ => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects unexpected nodes: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh17 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh17 = 0 as xmlNodePtr;
                                    let ref mut fresh18 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh18 = 0 as xmlNodePtr
                                }
                            }
                        }
                        10 | 11 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
                            current_block_94 = 17032209247604446099;
                            match current_block_94 {
                                12530596945886496894 => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects an attribute: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh15 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh15 = 0 as xmlNodePtr
                                }
                                1910542799034262194 => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects a namespace: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh16 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh16 = 0 as xmlNodePtr
                                }
                                _ => {
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_XPTR_RESULT as
                                                       std::os::raw::c_int,
                                                   b"XPointer selects unexpected nodes: #%s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   fragment);
                                    let ref mut fresh17 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh17 = 0 as xmlNodePtr;
                                    let ref mut fresh18 =
                                        *(*set).nodeTab.offset(i as isize);
                                    *fresh18 = 0 as xmlNodePtr
                                }
                            }
                        }
                        1 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 13 | 21 | _ => { }
                    }
                }
                i += 1
                /* for */
            }
        }
        if doc.is_null() {
            let ref mut fresh19 = (**(*ctxt).incTab.offset(nr as isize)).xptr;
            *fresh19 = xptr;
            let ref mut fresh20 = (**(*ctxt).incTab.offset(nr as isize)).inc;
            *fresh20 = 0 as xmlNodePtr
        } else {
            let ref mut fresh21 = (**(*ctxt).incTab.offset(nr as isize)).inc;
            *fresh21 = xmlXIncludeCopyXPointer(ctxt, (*ctxt).doc, doc, xptr);
            xmlXPathFreeObject(xptr);
        }
        xmlXPathFreeContext(xptrctxt);
        xmlFree.expect("non-null function pointer")(fragment as
                                                        *mut std::os::raw::c_void);
    }
    /*
     * Do the xml:base fixup if needed
     */
    if !doc.is_null() && !URL.is_null() &&
           (*ctxt).parseFlags & XML_PARSE_NOBASEFIX as std::os::raw::c_int == 0 &&
           (*doc).parseFlags & XML_PARSE_NOBASEFIX as std::os::raw::c_int == 0 {
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        let mut base: *mut xmlChar = 0 as *mut xmlChar;
        let mut curBase: *mut xmlChar = 0 as *mut xmlChar;
        /*
	 * The base is only adjusted if "necessary", i.e. if the xinclude node
	 * has a base specified, or the URL is relative
	 */
        base =
            xmlGetNsProp((**(*ctxt).incTab.offset(nr as isize)).ref_0 as
                             *const xmlNode,
                         b"base\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         b"http://www.w3.org/XML/1998/namespace\x00" as
                             *const u8 as *const std::os::raw::c_char as
                             *const xmlChar);
        if base.is_null() {
            /*
	     * No xml:base on the xinclude node, so we check whether the
	     * URI base is different than (relative to) the context base
	     */
            curBase = xmlBuildRelativeURI(URL, (*ctxt).base);
            if curBase.is_null() {
                /* Error return */
                xmlXIncludeErr(ctxt,
                               (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                               XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                               b"trying to build relative URI from %s\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, URL);
            } else if xmlStrchr(curBase, '/' as i32 as xmlChar).is_null() {
                xmlFree.expect("non-null function pointer")(curBase as
                                                                *mut std::os::raw::c_void);
            } else { base = curBase }
        }
        if !base.is_null() {
            /* If the URI doesn't contain a slash, it's not relative */
            /* Adjustment may be needed */
            node = (**(*ctxt).incTab.offset(nr as isize)).inc;
            while !node.is_null() {
                /* Only work on element nodes */
                if (*node).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    curBase =
                        xmlNodeGetBase((*node).doc, node as *const xmlNode);
                    /* If no current base, set it */
                    if curBase.is_null() {
                        xmlNodeSetBase(node, base);
                    } else {
                        /*
			 * If the current base is the same as the
			 * URL of the document, then reset it to be
			 * the specified xml:base or the relative URI
			 */
                        if xmlStrEqual(curBase, (*(*node).doc).URL) != 0 {
                            xmlNodeSetBase(node, base);
                        } else {
                            /*
			     * If the element already has an xml:base
			     * set, then relativise it if necessary
			     */
                            let mut xmlBase: *mut xmlChar = 0 as *mut xmlChar;
                            xmlBase =
                                xmlGetNsProp(node as *const xmlNode,
                                             b"base\x00" as *const u8 as
                                                 *const std::os::raw::c_char as
                                                 *mut xmlChar,
                                             b"http://www.w3.org/XML/1998/namespace\x00"
                                                 as *const u8 as
                                                 *const std::os::raw::c_char as
                                                 *const xmlChar);
                            if !xmlBase.is_null() {
                                let mut relBase: *mut xmlChar =
                                    0 as *mut xmlChar;
                                relBase = xmlBuildURI(xmlBase, base);
                                if relBase.is_null() {
                                    /* error */
                                    xmlXIncludeErr(ctxt,
                                                   (**(*ctxt).incTab.offset(nr
                                                                                as
                                                                                isize)).ref_0,
                                                   XML_XINCLUDE_HREF_URI as
                                                       std::os::raw::c_int,
                                                   b"trying to rebuild base from %s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   xmlBase);
                                } else {
                                    xmlNodeSetBase(node, relBase);
                                    xmlFree.expect("non-null function pointer")(relBase
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                xmlFree.expect("non-null function pointer")(xmlBase
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                        }
                        xmlFree.expect("non-null function pointer")(curBase as
                                                                        *mut std::os::raw::c_void);
                    }
                }
                node = (*node).next
            }
            xmlFree.expect("non-null function pointer")(base as
                                                            *mut std::os::raw::c_void);
        }
    }
    if nr < (*ctxt).incNr &&
           !(**(*ctxt).incTab.offset(nr as isize)).doc.is_null() &&
           (**(*ctxt).incTab.offset(nr as isize)).count <= 1 as std::os::raw::c_int {
        xmlFreeDoc((**(*ctxt).incTab.offset(nr as isize)).doc);
        let ref mut fresh22 = (**(*ctxt).incTab.offset(nr as isize)).doc;
        *fresh22 = 0 as xmlDocPtr
    }
    xmlFree.expect("non-null function pointer")(URL as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeLoadTxt:
 * @ctxt:  the XInclude context
 * @url:  the associated URL
 * @nr:  the xinclude node number
 *
 * Load the content, and store the result in the XInclude context
 *
 * Returns 0 in case of success, -1 in case of failure
 */
unsafe extern "C" fn xmlXIncludeLoadTxt(mut ctxt: xmlXIncludeCtxtPtr,
                                        mut url: *const xmlChar,
                                        mut nr: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: std::os::raw::c_int = 0;
    let mut encoding: *mut xmlChar = 0 as *mut xmlChar;
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut pctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut xinclude_multibyte_fallback_used: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
     * Check the URL and remove any fragment identifier
     */
    uri = xmlParseURI(url as *const std::os::raw::c_char);
    if uri.is_null() {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"invalid value URI %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, url);
        return -(1 as std::os::raw::c_int)
    }
    if !(*uri).fragment.is_null() {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_TEXT_FRAGMENT as std::os::raw::c_int,
                       b"fragment identifier forbidden for text: %s\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       (*uri).fragment as *const xmlChar);
        xmlFreeURI(uri);
        return -(1 as std::os::raw::c_int)
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    if URL.is_null() {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"invalid value URI %s\n\x00" as *const u8 as
                           *const std::os::raw::c_char, url);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Handling of references to the local document are done
     * directly through ctxt->doc.
     */
    if *URL.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0 as std::os::raw::c_int {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_TEXT_DOCUMENT as std::os::raw::c_int,
                       b"text serialization of document not available\n\x00"
                           as *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar);
        xmlFree.expect("non-null function pointer")(URL as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Prevent reloading twice the document.
     */
    i = 0 as std::os::raw::c_int;
    loop  {
        if !(i < (*ctxt).txtNr) {
            current_block = 15345278821338558188;
            break ;
        }
        if xmlStrEqual(URL,
                       *(*ctxt).txturlTab.offset(i as isize) as
                           *const xmlChar) != 0 {
            node =
                xmlCopyNode(*(*ctxt).txtTab.offset(i as isize),
                            1 as std::os::raw::c_int);
            current_block = 16492625684971878027;
            break ;
        } else { i += 1 }
    }
    match current_block {
        15345278821338558188 => {
            /*
     * Try to get the encoding if available
     */
            if !(*(*ctxt).incTab.offset(nr as isize)).is_null() &&
                   !(**(*ctxt).incTab.offset(nr as isize)).ref_0.is_null() {
                encoding =
                    xmlGetProp((**(*ctxt).incTab.offset(nr as isize)).ref_0 as
                                   *const xmlNode,
                               b"encoding\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar)
            }
            if !encoding.is_null() {
                /*
	 * TODO: we should not have to remap to the xmlCharEncoding
	 *       predefined set, a better interface than
	 *       xmlParserInputBufferCreateFilename should allow any
	 *       encoding supported by iconv
	 */
                enc = xmlParseCharEncoding(encoding as *const std::os::raw::c_char);
                if enc as std::os::raw::c_int ==
                       XML_CHAR_ENCODING_ERROR as std::os::raw::c_int {
                    xmlXIncludeErr(ctxt,
                                   (**(*ctxt).incTab.offset(nr as
                                                                isize)).ref_0,
                                   XML_XINCLUDE_UNKNOWN_ENCODING as
                                       std::os::raw::c_int,
                                   b"encoding %s not supported\n\x00" as
                                       *const u8 as *const std::os::raw::c_char,
                                   encoding);
                    xmlFree.expect("non-null function pointer")(encoding as
                                                                    *mut std::os::raw::c_void);
                    xmlFree.expect("non-null function pointer")(URL as
                                                                    *mut std::os::raw::c_void);
                    return -(1 as std::os::raw::c_int)
                }
                xmlFree.expect("non-null function pointer")(encoding as
                                                                *mut std::os::raw::c_void);
            }
            /*
     * Load it.
     */
            pctxt = xmlNewParserCtxt();
            inputStream =
                xmlLoadExternalEntity(URL as *const std::os::raw::c_char,
                                      0 as *const std::os::raw::c_char, pctxt);
            if inputStream.is_null() {
                xmlFreeParserCtxt(pctxt);
                xmlFree.expect("non-null function pointer")(URL as
                                                                *mut std::os::raw::c_void);
                return -(1 as std::os::raw::c_int)
            }
            buf = (*inputStream).buf;
            if buf.is_null() {
                xmlFreeInputStream(inputStream);
                xmlFreeParserCtxt(pctxt);
                xmlFree.expect("non-null function pointer")(URL as
                                                                *mut std::os::raw::c_void);
                return -(1 as std::os::raw::c_int)
            }
            if !(*buf).encoder.is_null() {
                xmlCharEncCloseFunc((*buf).encoder);
            }
            (*buf).encoder = xmlGetCharEncodingHandler(enc);
            node = xmlNewText(0 as *const xmlChar);
            /*
     * Scan all chars from the resource and add the to the node
     */
            's_272:
                while xmlParserInputBufferRead(buf, 128 as std::os::raw::c_int) >
                          0 as std::os::raw::c_int {
                    let mut len: std::os::raw::c_int = 0;
                    let mut content: *const xmlChar = 0 as *const xmlChar;
                    content = xmlBufContent((*buf).buffer as *const xmlBuf);
                    len = xmlBufLength((*buf).buffer) as std::os::raw::c_int;
                    i = 0 as std::os::raw::c_int;
                    while i < len {
                        let mut cur: std::os::raw::c_int = 0;
                        let mut l: std::os::raw::c_int = 0;
                        cur =
                            xmlStringCurrentChar(0 as xmlParserCtxtPtr,
                                                 &*content.offset(i as isize),
                                                 &mut l);
                        if if cur < 0x100 as std::os::raw::c_int {
                               (0x9 as std::os::raw::c_int <= cur &&
                                    cur <= 0xa as std::os::raw::c_int ||
                                    cur == 0xd as std::os::raw::c_int ||
                                    0x20 as std::os::raw::c_int <= cur) as std::os::raw::c_int
                           } else {
                               (0x100 as std::os::raw::c_int <= cur &&
                                    cur <= 0xd7ff as std::os::raw::c_int ||
                                    0xe000 as std::os::raw::c_int <= cur &&
                                        cur <= 0xfffd as std::os::raw::c_int ||
                                    0x10000 as std::os::raw::c_int <= cur &&
                                        cur <= 0x10ffff as std::os::raw::c_int) as
                                   std::os::raw::c_int
                           } == 0 {
                            /* Handle splitted multibyte char at buffer boundary */
                            if len - i < 4 as std::os::raw::c_int &&
                                   xinclude_multibyte_fallback_used == 0 {
                                xinclude_multibyte_fallback_used =
                                    1 as std::os::raw::c_int;
                                xmlBufShrink((*buf).buffer, i as size_t);
                                continue 's_272 ;
                            } else {
                                xmlXIncludeErr(ctxt,
                                               (**(*ctxt).incTab.offset(nr as
                                                                            isize)).ref_0,
                                               XML_XINCLUDE_INVALID_CHAR as
                                                   std::os::raw::c_int,
                                               b"%s contains invalid char\n\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char, URL);
                                xmlFreeParserInputBuffer(buf);
                                xmlFree.expect("non-null function pointer")(URL
                                                                                as
                                                                                *mut std::os::raw::c_void);
                                return -(1 as std::os::raw::c_int)
                            }
                        } else {
                            xinclude_multibyte_fallback_used =
                                0 as std::os::raw::c_int;
                            xmlNodeAddContentLen(node,
                                                 &*content.offset(i as isize),
                                                 l);
                            i += l
                        }
                    }
                    xmlBufShrink((*buf).buffer, len as size_t);
                }
            xmlFreeParserCtxt(pctxt);
            xmlXIncludeAddTxt(ctxt, node, URL);
            xmlFreeInputStream(inputStream);
        }
        _ => { }
    }
    /*
     * Add the element as the replacement copy.
     */
    let ref mut fresh23 = (**(*ctxt).incTab.offset(nr as isize)).inc;
    *fresh23 = node;
    xmlFree.expect("non-null function pointer")(URL as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeLoadFallback:
 * @ctxt:  the XInclude context
 * @fallback:  the fallback node
 * @nr:  the xinclude node number
 *
 * Load the content of the fallback node, and store the result
 * in the XInclude context
 *
 * Returns 0 in case of success, -1 in case of failure
 */
unsafe extern "C" fn xmlXIncludeLoadFallback(mut ctxt: xmlXIncludeCtxtPtr,
                                             mut fallback: xmlNodePtr,
                                             mut nr: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut newctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if fallback.is_null() ||
           (*fallback).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           ctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if !(*fallback).children.is_null() {
        /*
	 * It's possible that the fallback also has 'includes'
	 * (Bug 129969), so we re-process the fallback just in case
	 */
        newctxt =
            xmlXIncludeNewContext((*ctxt).doc); /* Inherit the base from the existing context */
        if newctxt.is_null() {
            return -(1 as std::os::raw::c_int)
        } /* xmlXIncludeDoProcess can return +ve number */
        (*newctxt)._private = (*ctxt)._private;
        (*newctxt).base = xmlStrdup((*ctxt).base);
        xmlXIncludeSetFlags(newctxt, (*ctxt).parseFlags);
        ret =
            xmlXIncludeDoProcess(newctxt, (*ctxt).doc, (*fallback).children);
        if (*ctxt).nbErrors > 0 as std::os::raw::c_int {
            ret = -(1 as std::os::raw::c_int)
        } else if ret > 0 as std::os::raw::c_int { ret = 0 as std::os::raw::c_int }
        xmlXIncludeFreeContext(newctxt);
        let ref mut fresh24 = (**(*ctxt).incTab.offset(nr as isize)).inc;
        *fresh24 = xmlDocCopyNodeList((*ctxt).doc, (*fallback).children)
    } else {
        let ref mut fresh25 = (**(*ctxt).incTab.offset(nr as isize)).inc;
        *fresh25 = 0 as xmlNodePtr;
        (**(*ctxt).incTab.offset(nr as isize)).emptyFb = 1 as std::os::raw::c_int
        /* flag empty callback */
    }
    return ret;
}
/* ***********************************************************************
 *									*
 *			XInclude Processing				*
 *									*
 ************************************************************************/
/* *
 * xmlXIncludePreProcessNode:
 * @ctxt: an XInclude context
 * @node: an XInclude node
 *
 * Implement the XInclude preprocessing, currently just adding the element
 * for further processing.
 *
 * Returns the result list or NULL in case of error
 */
unsafe extern "C" fn xmlXIncludePreProcessNode(mut ctxt: xmlXIncludeCtxtPtr,
                                               mut node: xmlNodePtr)
 -> xmlNodePtr {
    xmlXIncludeAddNode(ctxt, node);
    return 0 as xmlNodePtr;
}
/* *
 * xmlXIncludeLoadNode:
 * @ctxt: an XInclude context
 * @nr: the node number
 *
 * Find and load the infoset replacement for the given node.
 *
 * Returns 0 if substitution succeeded, -1 if some processing failed
 */
unsafe extern "C" fn xmlXIncludeLoadNode(mut ctxt: xmlXIncludeCtxtPtr,
                                         mut nr: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode; /* default Issue 64 */
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut parse: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut oldBase: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut xml: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut ret: std::os::raw::c_int = 0;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if nr < 0 as std::os::raw::c_int || nr >= (*ctxt).incNr {
        return -(1 as std::os::raw::c_int)
    }
    cur = (**(*ctxt).incTab.offset(nr as isize)).ref_0;
    if cur.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * read the attributes
     */
    href =
        xmlXIncludeGetProp(ctxt, cur,
                           b"href\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar); /* @@@@ href is now optional */
    if href.is_null() {
        href =
            xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar);
        if href.is_null() { return -(1 as std::os::raw::c_int) }
    }
    parse =
        xmlXIncludeGetProp(ctxt, cur,
                           b"parse\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar);
    if !parse.is_null() {
        if xmlStrEqual(parse,
                       b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 {
            xml = 1 as std::os::raw::c_int
        } else if xmlStrEqual(parse,
                              b"text\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 {
            xml = 0 as std::os::raw::c_int
        } else {
            xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                           XML_XINCLUDE_PARSE_VALUE as std::os::raw::c_int,
                           b"invalid value %s for \'parse\'\n\x00" as
                               *const u8 as *const std::os::raw::c_char, parse);
            if !href.is_null() {
                xmlFree.expect("non-null function pointer")(href as
                                                                *mut std::os::raw::c_void);
            }
            if !parse.is_null() {
                xmlFree.expect("non-null function pointer")(parse as
                                                                *mut std::os::raw::c_void);
            }
            return -(1 as std::os::raw::c_int)
        }
    }
    /*
     * compute the URI
     */
    base =
        xmlNodeGetBase((*ctxt).doc as *const xmlDoc, cur as *const xmlNode);
    if base.is_null() {
        URI = xmlBuildURI(href, (*(*ctxt).doc).URL)
    } else { URI = xmlBuildURI(href, base) }
    if URI.is_null() {
        let mut escbase: *mut xmlChar = 0 as *mut xmlChar;
        let mut eschref: *mut xmlChar = 0 as *mut xmlChar;
        /*
	 * Some escaping may be needed
	 */
        escbase = xmlURIEscape(base);
        eschref = xmlURIEscape(href);
        URI = xmlBuildURI(eschref, escbase);
        if !escbase.is_null() {
            xmlFree.expect("non-null function pointer")(escbase as
                                                            *mut std::os::raw::c_void);
        }
        if !eschref.is_null() {
            xmlFree.expect("non-null function pointer")(eschref as
                                                            *mut std::os::raw::c_void);
        }
    }
    if URI.is_null() {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_HREF_URI as std::os::raw::c_int,
                       b"failed build URL\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar);
        if !parse.is_null() {
            xmlFree.expect("non-null function pointer")(parse as
                                                            *mut std::os::raw::c_void);
        }
        if !href.is_null() {
            xmlFree.expect("non-null function pointer")(href as
                                                            *mut std::os::raw::c_void);
        }
        if !base.is_null() {
            xmlFree.expect("non-null function pointer")(base as
                                                            *mut std::os::raw::c_void);
        }
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Save the base for this include (saving the current one)
     */
    oldBase = (*ctxt).base;
    (*ctxt).base = base;
    if xml != 0 {
        ret = xmlXIncludeLoadDoc(ctxt, URI, nr)
        /* xmlXIncludeGetFragment(ctxt, cur, URI); */
    } else { ret = xmlXIncludeLoadTxt(ctxt, URI, nr) }
    /*
     * Restore the original base before checking for fallback
     */
    (*ctxt).base = oldBase;
    if ret < 0 as std::os::raw::c_int {
        let mut children: xmlNodePtr = 0 as *mut xmlNode;
        /*
	 * Time to try a fallback if availble
	 */
        children = (*cur).children;
        while !children.is_null() {
            if (*children).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   !(*children).ns.is_null() &&
                   xmlStrEqual((*children).name,
                               b"fallback\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar) != 0
                   &&
                   (xmlStrEqual((*(*children).ns).href,
                                b"http://www.w3.org/2003/XInclude\x00" as
                                    *const u8 as *const std::os::raw::c_char as
                                    *const xmlChar) != 0 ||
                        xmlStrEqual((*(*children).ns).href,
                                    b"http://www.w3.org/2001/XInclude\x00" as
                                        *const u8 as *const std::os::raw::c_char as
                                        *const xmlChar) != 0) {
                ret = xmlXIncludeLoadFallback(ctxt, children, nr);
                if ret == 0 as std::os::raw::c_int { break ; }
            }
            children = (*children).next
        }
    }
    if ret < 0 as std::os::raw::c_int {
        xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                       XML_XINCLUDE_NO_FALLBACK as std::os::raw::c_int,
                       b"could not load %s, and no fallback was found\n\x00"
                           as *const u8 as *const std::os::raw::c_char, URI);
    }
    /*
     * Cleanup
     */
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut std::os::raw::c_void);
    }
    if !parse.is_null() {
        xmlFree.expect("non-null function pointer")(parse as
                                                        *mut std::os::raw::c_void);
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as
                                                        *mut std::os::raw::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as
                                                        *mut std::os::raw::c_void);
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeIncludeNode:
 * @ctxt: an XInclude context
 * @nr: the node number
 *
 * Inplement the infoset replacement for the given node
 *
 * Returns 0 if substitution succeeded, -1 if some processing failed
 */
unsafe extern "C" fn xmlXIncludeIncludeNode(mut ctxt: xmlXIncludeCtxtPtr,
                                            mut nr: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut end: xmlNodePtr = 0 as *mut xmlNode;
    let mut list: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if nr < 0 as std::os::raw::c_int || nr >= (*ctxt).incNr {
        return -(1 as std::os::raw::c_int)
    }
    cur = (**(*ctxt).incTab.offset(nr as isize)).ref_0;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    /*
     * If we stored an XPointer a late computation may be needed
     */
    if (**(*ctxt).incTab.offset(nr as isize)).inc.is_null() &&
           !(**(*ctxt).incTab.offset(nr as isize)).xptr.is_null() {
        let ref mut fresh26 = (**(*ctxt).incTab.offset(nr as isize)).inc;
        *fresh26 =
            xmlXIncludeCopyXPointer(ctxt, (*ctxt).doc, (*ctxt).doc,
                                    (**(*ctxt).incTab.offset(nr as
                                                                 isize)).xptr);
        xmlXPathFreeObject((**(*ctxt).incTab.offset(nr as isize)).xptr);
        let ref mut fresh27 = (**(*ctxt).incTab.offset(nr as isize)).xptr;
        *fresh27 = 0 as xmlXPathObjectPtr
    }
    list = (**(*ctxt).incTab.offset(nr as isize)).inc;
    let ref mut fresh28 = (**(*ctxt).incTab.offset(nr as isize)).inc;
    *fresh28 = 0 as xmlNodePtr;
    /*
     * Check against the risk of generating a multi-rooted document
     */
    if !(*cur).parent.is_null() &&
           (*(*cur).parent).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        let mut nb_elem: std::os::raw::c_int = 0 as std::os::raw::c_int;
        tmp = list;
        while !tmp.is_null() {
            if (*tmp).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                nb_elem += 1
            }
            tmp = (*tmp).next
        }
        if nb_elem > 1 as std::os::raw::c_int {
            xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                           XML_XINCLUDE_MULTIPLE_ROOT as std::os::raw::c_int,
                           b"XInclude error: would result in multiple root nodes\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar);
            return -(1 as std::os::raw::c_int)
        }
    }
    if (*ctxt).parseFlags & XML_PARSE_NOXINCNODE as std::os::raw::c_int != 0 {
        /*
	 * Add the list of nodes
	 */
        while !list.is_null() {
            end = list;
            list = (*list).next;
            xmlAddPrevSibling(cur, end);
        }
        xmlUnlinkNode(cur);
        xmlFreeNode(cur);
    } else {
        /*
	 * Change the current node as an XInclude start one, and add an
	 * XInclude end one
	 */
        (*cur).type_0 = XML_XINCLUDE_START;
        end =
            xmlNewDocNode((*cur).doc, (*cur).ns, (*cur).name,
                          0 as *const xmlChar);
        if end.is_null() {
            xmlXIncludeErr(ctxt, (**(*ctxt).incTab.offset(nr as isize)).ref_0,
                           XML_XINCLUDE_BUILD_FAILED as std::os::raw::c_int,
                           b"failed to build node\n\x00" as *const u8 as
                               *const std::os::raw::c_char, 0 as *const xmlChar);
            return -(1 as std::os::raw::c_int)
        }
        (*end).type_0 = XML_XINCLUDE_END;
        xmlAddNextSibling(cur, end);
        /*
	 * Add the list of nodes
	 */
        while !list.is_null() {
            cur = list;
            list = (*list).next;
            xmlAddPrevSibling(end, cur);
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeTestNode:
 * @ctxt: the XInclude processing context
 * @node: an XInclude node
 *
 * test if the node is an XInclude node
 *
 * Returns 1 true, 0 otherwise
 */
unsafe extern "C" fn xmlXIncludeTestNode(mut ctxt: xmlXIncludeCtxtPtr,
                                         mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    if node.is_null() { return 0 as std::os::raw::c_int }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*node).ns.is_null() { return 0 as std::os::raw::c_int }
    if xmlStrEqual((*(*node).ns).href,
                   b"http://www.w3.org/2003/XInclude\x00" as *const u8 as
                       *const std::os::raw::c_char as *const xmlChar) != 0 ||
           xmlStrEqual((*(*node).ns).href,
                       b"http://www.w3.org/2001/XInclude\x00" as *const u8 as
                           *const std::os::raw::c_char as *const xmlChar) != 0 {
        if xmlStrEqual((*(*node).ns).href,
                       b"http://www.w3.org/2001/XInclude\x00" as *const u8 as
                           *const std::os::raw::c_char as *const xmlChar) != 0 {
            if (*ctxt).legacy == 0 as std::os::raw::c_int {
                /* wait for the XML Core Working Group to get something stable ! */
                (*ctxt).legacy = 1 as std::os::raw::c_int
            }
        }
        if xmlStrEqual((*node).name,
                       b"include\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 {
            let mut child: xmlNodePtr = (*node).children;
            let mut nb_fallback: std::os::raw::c_int = 0 as std::os::raw::c_int;
            while !child.is_null() {
                if (*child).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                       !(*child).ns.is_null() &&
                       (xmlStrEqual((*(*child).ns).href,
                                    b"http://www.w3.org/2003/XInclude\x00" as
                                        *const u8 as *const std::os::raw::c_char as
                                        *const xmlChar) != 0 ||
                            xmlStrEqual((*(*child).ns).href,
                                        b"http://www.w3.org/2001/XInclude\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char as
                                            *const xmlChar) != 0) {
                    if xmlStrEqual((*child).name,
                                   b"include\x00" as *const u8 as
                                       *const std::os::raw::c_char as *const xmlChar)
                           != 0 {
                        xmlXIncludeErr(ctxt, node,
                                       XML_XINCLUDE_INCLUDE_IN_INCLUDE as
                                           std::os::raw::c_int,
                                       b"%s has an \'include\' child\n\x00" as
                                           *const u8 as *const std::os::raw::c_char,
                                       b"include\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *const xmlChar);
                        return 0 as std::os::raw::c_int
                    }
                    if xmlStrEqual((*child).name,
                                   b"fallback\x00" as *const u8 as
                                       *const std::os::raw::c_char as *const xmlChar)
                           != 0 {
                        nb_fallback += 1
                    }
                }
                child = (*child).next
            }
            if nb_fallback > 1 as std::os::raw::c_int {
                xmlXIncludeErr(ctxt, node,
                               XML_XINCLUDE_FALLBACKS_IN_INCLUDE as
                                   std::os::raw::c_int,
                               b"%s has multiple fallback children\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               b"include\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar);
                return 0 as std::os::raw::c_int
            }
            return 1 as std::os::raw::c_int
        }
        if xmlStrEqual((*node).name,
                       b"fallback\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 {
            if (*node).parent.is_null() ||
                   (*(*node).parent).type_0 as std::os::raw::c_uint !=
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*(*node).parent).ns.is_null() ||
                   xmlStrEqual((*(*(*node).parent).ns).href,
                               b"http://www.w3.org/2003/XInclude\x00" as
                                   *const u8 as *const std::os::raw::c_char as
                                   *const xmlChar) == 0 &&
                       xmlStrEqual((*(*(*node).parent).ns).href,
                                   b"http://www.w3.org/2001/XInclude\x00" as
                                       *const u8 as *const std::os::raw::c_char as
                                       *const xmlChar) == 0 ||
                   xmlStrEqual((*(*node).parent).name,
                               b"include\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar) == 0
               {
                xmlXIncludeErr(ctxt, node,
                               XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE as
                                   std::os::raw::c_int,
                               b"%s is not the child of an \'include\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               b"fallback\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar);
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* application data */
/* *
 * xmlXIncludeDoProcess:
 * @ctxt: the XInclude processing context
 * @doc: an XML document
 * @tree: the top of the tree to process
 *
 * Implement the XInclude substitution on the XML document @doc
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
unsafe extern "C" fn xmlXIncludeDoProcess(mut ctxt: xmlXIncludeCtxtPtr,
                                          mut doc: xmlDocPtr,
                                          mut tree: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut start: std::os::raw::c_int = 0;
    if doc.is_null() || tree.is_null() ||
           (*tree).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if !(*doc).URL.is_null() {
        ret = xmlXIncludeURLPush(ctxt, (*doc).URL);
        if ret < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    }
    start = (*ctxt).incNr;
    /*
     * First phase: lookup the elements in the document
     */
    cur = tree;
    if xmlXIncludeTestNode(ctxt, cur) == 1 as std::os::raw::c_int {
        xmlXIncludePreProcessNode(ctxt, cur);
    }
    while !cur.is_null() && cur != (*tree).parent {
        /* TODO: need to work on entities -> stack */
        if !(*cur).children.is_null() &&
               (*(*cur).children).type_0 as std::os::raw::c_uint !=
                   XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*cur).children).type_0 as std::os::raw::c_uint !=
                   XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*cur).children).type_0 as std::os::raw::c_uint !=
                   XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint {
            cur = (*cur).children; /* do */
            if xmlXIncludeTestNode(ctxt, cur) != 0 {
                xmlXIncludePreProcessNode(ctxt, cur);
            }
        } else if !(*cur).next.is_null() {
            cur = (*cur).next;
            if xmlXIncludeTestNode(ctxt, cur) != 0 {
                xmlXIncludePreProcessNode(ctxt, cur);
            }
        } else {
            if cur == tree { break ; }
            loop  {
                cur = (*cur).parent;
                if cur.is_null() || cur == (*tree).parent { break ; }
                if !(*cur).next.is_null() {
                    cur = (*cur).next;
                    if xmlXIncludeTestNode(ctxt, cur) != 0 {
                        xmlXIncludePreProcessNode(ctxt, cur);
                    }
                    break ;
                    /* do */
                } else if cur.is_null() { break ; }
            }
        }
    }
    /*
     * Second Phase : collect the infosets fragments
     */
    i = start;
    while i < (*ctxt).incNr { xmlXIncludeLoadNode(ctxt, i); ret += 1; i += 1 }
    /*
     * Third phase: extend the original document infoset.
     *
     * Originally we bypassed the inclusion if there were any errors
     * encountered on any of the XIncludes.  A bug was raised (bug
     * 132588) requesting that we output the XIncludes without error,
     * so the check for inc!=NULL || xptr!=NULL was put in.  This may
     * give some other problems in the future, but for now it seems to
     * work ok.
     *
     */
    i = (*ctxt).incBase;
    while i < (*ctxt).incNr {
        if !(**(*ctxt).incTab.offset(i as isize)).inc.is_null() ||
               !(**(*ctxt).incTab.offset(i as isize)).xptr.is_null() ||
               (**(*ctxt).incTab.offset(i as isize)).emptyFb !=
                   0 as std::os::raw::c_int {
            /* (empty fallback) */
            xmlXIncludeIncludeNode(ctxt, i);
        }
        i += 1
    }
    if !(*doc).URL.is_null() { xmlXIncludeURLPop(ctxt); }
    return ret;
}
/* *
 * xmlXIncludeSetFlags:
 * @ctxt:  an XInclude processing context
 * @flags: a set of xmlParserOption used for parsing XML includes
 *
 * Set the flags used for further processing of XML resources.
 *
 * Returns 0 in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeSetFlags(mut ctxt: xmlXIncludeCtxtPtr,
                                             mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    (*ctxt).parseFlags = flags;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXIncludeProcessTreeFlagsData:
 * @tree: an XML node
 * @flags: a set of xmlParserOption used for parsing XML includes
 * @data: application data that will be passed to the parser context
 *        in the _private field of the parser context(s)
 *
 * Implement the XInclude substitution on the XML node @tree
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTreeFlagsData(mut tree: xmlNodePtr,
                                                         mut flags:
                                                             std::os::raw::c_int,
                                                         mut data:
                                                             *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if tree.is_null() ||
           (*tree).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           (*tree).doc.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    ctxt = xmlXIncludeNewContext((*tree).doc);
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    (*ctxt)._private = data;
    (*ctxt).base = xmlStrdup((*(*tree).doc).URL as *mut xmlChar);
    xmlXIncludeSetFlags(ctxt, flags);
    ret = xmlXIncludeDoProcess(ctxt, (*tree).doc, tree);
    if ret >= 0 as std::os::raw::c_int && (*ctxt).nbErrors > 0 as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    }
    xmlXIncludeFreeContext(ctxt);
    return ret;
}
/* *
 * xmlXIncludeProcessFlagsData:
 * @doc: an XML document
 * @flags: a set of xmlParserOption used for parsing XML includes
 * @data: application data that will be passed to the parser context
 *        in the _private field of the parser context(s)
 *
 * Implement the XInclude substitution on the XML document @doc
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessFlagsData(mut doc: xmlDocPtr,
                                                     mut flags: std::os::raw::c_int,
                                                     mut data:
                                                         *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut tree: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    tree = xmlDocGetRootElement(doc as *const xmlDoc);
    if tree.is_null() { return -(1 as std::os::raw::c_int) }
    return xmlXIncludeProcessTreeFlagsData(tree, flags, data);
}
/* *
 * xmlXIncludeProcessFlags:
 * @doc: an XML document
 * @flags: a set of xmlParserOption used for parsing XML includes
 *
 * Implement the XInclude substitution on the XML document @doc
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessFlags(mut doc: xmlDocPtr,
                                                 mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    return xmlXIncludeProcessFlagsData(doc, flags, 0 as *mut std::os::raw::c_void);
}
/*
 * standalone processing
 */
/* *
 * xmlXIncludeProcess:
 * @doc: an XML document
 *
 * Implement the XInclude substitution on the XML document @doc
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcess(mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    return xmlXIncludeProcessFlags(doc, 0 as std::os::raw::c_int);
}
/* *
 * xmlXIncludeProcessTreeFlags:
 * @tree: a node in an XML document
 * @flags: a set of xmlParserOption used for parsing XML includes
 *
 * Implement the XInclude substitution for the given subtree
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTreeFlags(mut tree: xmlNodePtr,
                                                     mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if tree.is_null() ||
           (*tree).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           (*tree).doc.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    ctxt = xmlXIncludeNewContext((*tree).doc);
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    (*ctxt).base = xmlNodeGetBase((*tree).doc, tree as *const xmlNode);
    xmlXIncludeSetFlags(ctxt, flags);
    ret = xmlXIncludeDoProcess(ctxt, (*tree).doc, tree);
    if ret >= 0 as std::os::raw::c_int && (*ctxt).nbErrors > 0 as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    }
    xmlXIncludeFreeContext(ctxt);
    return ret;
}
/* *
 * xmlXIncludeProcessTree:
 * @tree: a node in an XML document
 *
 * Implement the XInclude substitution for the given subtree
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTree(mut tree: xmlNodePtr)
 -> std::os::raw::c_int {
    return xmlXIncludeProcessTreeFlags(tree, 0 as std::os::raw::c_int);
}
/* *
 * xmlXIncludeProcessNode:
 * @ctxt: an existing XInclude context
 * @node: a node in an XML document
 *
 * Implement the XInclude substitution for the given subtree reusing
 * the informations and data coming from the given context.
 *
 * Returns 0 if no substitution were done, -1 if some processing failed
 *    or the number of substitutions done.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessNode(mut ctxt: xmlXIncludeCtxtPtr,
                                                mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           (*node).doc.is_null() || ctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    ret = xmlXIncludeDoProcess(ctxt, (*node).doc, node);
    if ret >= 0 as std::os::raw::c_int && (*ctxt).nbErrors > 0 as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    }
    return ret;
}
/* __INCLUDE_ELFGCCHACK */
/* !LIBXML_XINCLUDE_ENABLED */
