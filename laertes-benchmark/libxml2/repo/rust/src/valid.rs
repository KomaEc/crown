
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlRegExecCtxt;
    pub type _xmlRegexp;
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
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrncmp(str1: *const xmlChar, str2: *const xmlChar,
                  len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar)
     -> xmlDtdPtr;
    /*
 * Lookup of entry in the dictionary.
 */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlRegFreeRegexp(regexp: xmlRegexpPtr);
    #[no_mangle]
    fn xmlRegexpIsDeterminist(comp: xmlRegexpPtr) -> std::os::raw::c_int;
    /*
 * The progressive API
 */
    #[no_mangle]
    fn xmlRegNewExecCtxt(comp: xmlRegexpPtr, callback: xmlRegExecCallbacks,
                         data: *mut std::os::raw::c_void) -> xmlRegExecCtxtPtr;
    #[no_mangle]
    fn xmlRegFreeExecCtxt(exec: xmlRegExecCtxtPtr);
    #[no_mangle]
    fn xmlRegExecPushString(exec: xmlRegExecCtxtPtr, value: *const xmlChar,
                            data: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBuildQName(ncname: *const xmlChar, prefix: *const xmlChar,
                     memory: *mut xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlSplitQName3(name: *const xmlChar, len: *mut std::os::raw::c_int)
     -> *const xmlChar;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
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
    fn xmlIsBlankNode(node: *const xmlNode) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlUnlinkNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlFreeNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlNodeListGetString(doc: xmlDocPtr, list: *const xmlNode,
                            inLine: std::os::raw::c_int) -> *mut xmlChar;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Internal, don't use.
 */
    #[no_mangle]
    fn xmlBufferWriteCHAR(buf: xmlBufferPtr, string: *const xmlChar);
    #[no_mangle]
    fn xmlBufferWriteChar(buf: xmlBufferPtr, string: *const std::os::raw::c_char);
    #[no_mangle]
    fn xmlBufferWriteQuotedString(buf: xmlBufferPtr, string: *const xmlChar);
    #[no_mangle]
    fn xmlHashCreateDict(size: std::os::raw::c_int, dict: xmlDictPtr)
     -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    /*
 * Add a new entry to the hash table.
 */
    #[no_mangle]
    fn xmlHashAddEntry(table: xmlHashTablePtr, name: *const xmlChar,
                       userdata: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashUpdateEntry(table: xmlHashTablePtr, name: *const xmlChar,
                          userdata: *mut std::os::raw::c_void, f: xmlHashDeallocator)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashAddEntry2(table: xmlHashTablePtr, name: *const xmlChar,
                        name2: *const xmlChar, userdata: *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashAddEntry3(table: xmlHashTablePtr, name: *const xmlChar,
                        name2: *const xmlChar, name3: *const xmlChar,
                        userdata: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    /*
 * Remove an entry from the hash table.
 */
    #[no_mangle]
    fn xmlHashRemoveEntry(table: xmlHashTablePtr, name: *const xmlChar,
                          f: xmlHashDeallocator) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashRemoveEntry2(table: xmlHashTablePtr, name: *const xmlChar,
                           name2: *const xmlChar, f: xmlHashDeallocator)
     -> std::os::raw::c_int;
    /*
 * Retrieve the userdata.
 */
    #[no_mangle]
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlHashLookup2(table: xmlHashTablePtr, name: *const xmlChar,
                      name2: *const xmlChar) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlHashLookup3(table: xmlHashTablePtr, name: *const xmlChar,
                      name2: *const xmlChar, name3: *const xmlChar)
     -> *mut std::os::raw::c_void;
    /*
 * Helpers.
 */
    #[no_mangle]
    fn xmlHashCopy(table: xmlHashTablePtr, f: xmlHashCopier)
     -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashScan(table: xmlHashTablePtr, f: xmlHashScanner,
                   data: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlHashScan3(table: xmlHashTablePtr, name: *const xmlChar,
                    name2: *const xmlChar, name3: *const xmlChar,
                    f: xmlHashScanner, data: *mut std::os::raw::c_void);
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
    #[no_mangle]
    fn xmlListAppend(l: xmlListPtr, data: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlListRemoveFirst(l: xmlListPtr, data: *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlListEmpty(l: xmlListPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlListWalk(l: xmlListPtr, walker: xmlListWalker,
                   user: *mut std::os::raw::c_void);
    /* Link operators */
    #[no_mangle]
    fn xmlLinkGetData(lk: xmlLinkPtr) -> *mut std::os::raw::c_void;
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
    /*
 * Building API
 */
    #[no_mangle]
    fn xmlNewAutomata() -> xmlAutomataPtr;
    #[no_mangle]
    fn xmlFreeAutomata(am: xmlAutomataPtr);
    #[no_mangle]
    fn xmlAutomataGetInitState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataSetFinalState(am: xmlAutomataPtr,
                                state: xmlAutomataStatePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlAutomataNewState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewTransition(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                                to: xmlAutomataStatePtr,
                                token: *const xmlChar,
                                data: *mut std::os::raw::c_void)
     -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewEpsilon(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                             to: xmlAutomataStatePtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataCompile(am: xmlAutomataPtr) -> xmlRegexpPtr;
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
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    /* was the entity content checked */
    /* this is also used to count entities
					 * references done from that entity
					 * and if it contains '<' */
    /*
 * All entities are stored in an hash table.
 * There is 2 separate hash tables for global and parameter entities.
 */
    /*
 * External functions:
 */
    /* LIBXML_LEGACY_ENABLED */
    #[no_mangle]
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    fn xmlNoValidityErr(ctx: *mut std::os::raw::c_void, msg: *const std::os::raw::c_char,
                        _: ...);
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStringCurrentChar(ctxt: xmlParserCtxtPtr, cur: *const xmlChar,
                            len: *mut std::os::raw::c_int) -> std::os::raw::c_int;
    /* *
 * xmlIsChar_ch:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsCharQ:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsCombiningQ:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsDigit_ch:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsDigitQ:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsExtender_ch:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    /* *
 * xmlIsExtenderQ:
 * @c: char to validate
 *
 * Automatically generated by genChRanges.py
 */
    #[no_mangle]
    static xmlIsExtenderGroup: xmlChRangeGroup;
    #[no_mangle]
    fn xmlCharInRange(val: std::os::raw::c_uint, group: *const xmlChRangeGroup)
     -> std::os::raw::c_int;
    #[no_mangle]
    static xmlIsCombiningGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsDigitGroup: xmlChRangeGroup;
    #[no_mangle]
    static xmlIsBaseCharGroup: xmlChRangeGroup;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidState {
    pub elemDecl: xmlElementPtr,
    pub node: xmlNodePtr,
    pub exec: xmlRegExecCtxtPtr,
}
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlElementPtr = *mut xmlElement;
pub type xmlElement = _xmlElement;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElement {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub etype: xmlElementTypeVal,
    pub content: xmlElementContentPtr,
    pub attributes: xmlAttributePtr,
    pub prefix: *const xmlChar,
    pub contModel: xmlRegexpPtr,
}
pub type xmlRegexpPtr = *mut xmlRegexp;
pub type xmlRegexp = _xmlRegexp;
pub type xmlAttributePtr = *mut xmlAttribute;
pub type xmlAttribute = _xmlAttribute;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttribute {
    pub _private: *mut std::os::raw::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub nexth: *mut _xmlAttribute,
    pub atype: xmlAttributeType,
    pub def: xmlAttributeDefault,
    pub defaultValue: *const xmlChar,
    pub tree: xmlEnumerationPtr,
    pub prefix: *const xmlChar,
    pub elem: *const xmlChar,
}
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type xmlAttributeDefault = std::os::raw::c_uint;
pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
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
pub type xmlElementContentType = std::os::raw::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type xmlElementTypeVal = std::os::raw::c_uint;
pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
pub type xmlDocPtr = *mut xmlDoc;
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
/* next Ns link for this node  */
/* global or local */
/* URL for the namespace */
/* prefix for the namespace */
/* application data */
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
/* application data */
/* XML_ATTRIBUTE_NODE, must be second ! */
/* the name of the property */
/* the value of the property */
/* NULL */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* pointer to the associated namespace */
/* the attribute type if validating */
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
pub type xmlDoc = _xmlDoc;
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
pub type xmlErrorPtr = *mut xmlError;
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
pub type endElementNsSAX2Func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNotation {
    pub name: *const xmlChar,
    pub PublicID: *const xmlChar,
    pub SystemID: *const xmlChar,
}
pub type xmlNotation = _xmlNotation;
pub type xmlNotationPtr = *mut xmlNotation;
/* *
 * xmlRegExecCallbacks:
 * @exec: the regular expression context
 * @token: the current token string
 * @transdata: transition data
 * @inputdata: input data
 *
 * Callback function when doing a transition in the automata
 */
pub type xmlRegExecCallbacks
    =
    Option<unsafe extern "C" fn(_: xmlRegExecCtxtPtr, _: *const xmlChar,
                                _: *mut std::os::raw::c_void, _: *mut std::os::raw::c_void)
               -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRef {
    pub next: *mut _xmlRef,
    pub value: *const xmlChar,
    pub attr: xmlAttrPtr,
    pub name: *const xmlChar,
    pub lineno: std::os::raw::c_int,
}
pub type xmlRef = _xmlRef;
pub type xmlRefPtr = *mut xmlRef;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const XML_DOC_HTML: C2RustUnnamed = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed = 8;
pub const XML_DOC_OLD10: C2RustUnnamed = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed = 1;
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
/* *
 * xmlHashCopier:
 * @payload:  the data in the hash
 * @name:  the name associated
 *
 * Callback to copy data from a hash.
 *
 * Returns a copy of the data or NULL in case of error.
 */
pub type xmlHashCopier
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> *mut std::os::raw::c_void>;
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
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_FROM_URI: C2RustUnnamed_0 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_0 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_0 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_0 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_0 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_0 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_0 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_0 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_0 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_0 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_0 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_0 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_0 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_0 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_0 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_0 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_0 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_0 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_0 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_0 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_0 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_0 = 9;
pub const XML_FROM_IO: C2RustUnnamed_0 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_0 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_0 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_0 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_0 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_0 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_0 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_0 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_0 = 0;
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
/* *
 * xmlListWalker:
 * @data: the data found in the list
 * @user: extra user provided data to the walker
 *
 * Callback function used when walking a list with xmlListWalk().
 *
 * Returns 0 to stop walking the list, 1 otherwise.
 */
pub type xmlListWalker
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_void, _: *mut std::os::raw::c_void)
               -> std::os::raw::c_int>;
pub type xmlValidStatePtr = *mut xmlValidState;
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
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
/* user specific data block */
/* the callback in case of errors */
/* the callback in case of warning */
/* Node analysis stack used when validating within entities */
/* Current parsed Node */
/* Depth of the parsing stack */
/* Max depth of the parsing stack */
/* array of nodes */
/* finished validating the Dtd ? */
/* the document */
/* temporary validity check result */
/* state state used for non-determinist content validation */
/* current state */
/* Depth of the validation stack */
/* Max depth of the validation stack */
/* array of validation states */
/* the automata */
/* used to build the automata */
/*
 * ALL notation declarations are stored in a table.
 * There is one table per DTD.
 */
pub type xmlNotationTable = _xmlHashTable;
pub type xmlNotationTablePtr = *mut xmlNotationTable;
/*
 * ALL element declarations are stored in a table.
 * There is one table per DTD.
 */
pub type xmlElementTable = _xmlHashTable;
pub type xmlElementTablePtr = *mut xmlElementTable;
/*
 * ALL attribute declarations are stored in a table.
 * There is one table per DTD.
 */
pub type xmlAttributeTable = _xmlHashTable;
pub type xmlAttributeTablePtr = *mut xmlAttributeTable;
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
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: std::os::raw::c_int,
    pub nbLongRange: std::os::raw::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: std::os::raw::c_uint,
    pub high: std::os::raw::c_uint,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: std::os::raw::c_ushort,
    pub high: std::os::raw::c_ushort,
}
/* ***********************************************************************
 *									*
 *				Refs					*
 *									*
 ************************************************************************/
pub type xmlRemoveMemo = xmlRemoveMemo_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlRemoveMemo_t {
    pub l: xmlListPtr,
    pub ap: xmlAttrPtr,
}
pub type xmlRemoveMemoPtr = *mut xmlRemoveMemo;
pub type xmlValidateMemo = xmlValidateMemo_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlValidateMemo_t {
    pub ctxt: xmlValidCtxtPtr,
    pub name: *const xmlChar,
}
pub type xmlValidateMemoPtr = *mut xmlValidateMemo;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
pub type xmlEntitiesTable = _xmlHashTable;
/* ***********************************************************************
 *									*
 *			Error handling routines				*
 *									*
 ************************************************************************/
/* *
 * xmlVErrMemory:
 * @ctxt:  an XML validation parser context
 * @extra:  extra informations
 *
 * Handle an out of memory error
 */
unsafe extern "C" fn xmlVErrMemory(mut ctxt: xmlValidCtxtPtr,
                                   mut extra: *const std::os::raw::c_char) {
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        channel = (*ctxt).error;
        data = (*ctxt).userData;
        /* Use the special values to detect if it is part of a parsing
	   context */
        if (*ctxt).finishDtd == 0xabcd1234 as std::os::raw::c_uint ||
               (*ctxt).finishDtd == 0xabcd1235 as std::os::raw::c_uint {
            let mut delta: std::os::raw::c_long =
                (ctxt as
                     *mut std::os::raw::c_char).offset_from((*ctxt).userData
                                                                 as
                                                                 *mut std::os::raw::c_char)
                    as std::os::raw::c_long;
            if delta > 0 as std::os::raw::c_int as std::os::raw::c_long &&
                   delta < 250 as std::os::raw::c_int as std::os::raw::c_long {
                pctxt = (*ctxt).userData as xmlParserCtxtPtr
            }
        }
    }
    if !extra.is_null() {
        __xmlRaiseError(None, channel, data, pctxt as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"Memory allocation failed : %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, extra);
    } else {
        __xmlRaiseError(None, channel, data, pctxt as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        b"Memory allocation failed\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
    };
}
/* *
 * xmlErrValid:
 * @ctxt:  an XML validation parser context
 * @error:  the error number
 * @extra:  extra informations
 *
 * Handle a validation error
 */
unsafe extern "C" fn xmlErrValid(mut ctxt: xmlValidCtxtPtr,
                                 mut error: xmlParserErrors,
                                 mut msg: *const std::os::raw::c_char,
                                 mut extra: *const std::os::raw::c_char) {
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        channel = (*ctxt).error;
        data = (*ctxt).userData;
        /* Use the special values to detect if it is part of a parsing
	   context */
        if (*ctxt).finishDtd == 0xabcd1234 as std::os::raw::c_uint ||
               (*ctxt).finishDtd == 0xabcd1235 as std::os::raw::c_uint {
            let mut delta: std::os::raw::c_long =
                (ctxt as
                     *mut std::os::raw::c_char).offset_from((*ctxt).userData
                                                                 as
                                                                 *mut std::os::raw::c_char)
                    as std::os::raw::c_long;
            if delta > 0 as std::os::raw::c_int as std::os::raw::c_long &&
                   delta < 250 as std::os::raw::c_int as std::os::raw::c_long {
                pctxt = (*ctxt).userData as xmlParserCtxtPtr
            }
        }
    }
    if !extra.is_null() {
        __xmlRaiseError(None, channel, data, pctxt as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                        error as std::os::raw::c_int, XML_ERR_ERROR,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, extra);
    } else {
        __xmlRaiseError(None, channel, data, pctxt as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                        error as std::os::raw::c_int, XML_ERR_ERROR,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int,
                        b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg);
    };
}
/* *
 * xmlErrValidNode:
 * @ctxt:  an XML validation parser context
 * @node:  the node raising the error
 * @error:  the error number
 * @str1:  extra informations
 * @str2:  extra informations
 * @str3:  extra informations
 *
 * Handle a validation error, provide contextual informations
 */
unsafe extern "C" fn xmlErrValidNode(mut ctxt: xmlValidCtxtPtr,
                                     mut node: xmlNodePtr,
                                     mut error: xmlParserErrors,
                                     mut msg: *const std::os::raw::c_char,
                                     mut str1: *const xmlChar,
                                     mut str2: *const xmlChar,
                                     mut str3: *const xmlChar) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        channel = (*ctxt).error;
        data = (*ctxt).userData;
        /* Use the special values to detect if it is part of a parsing
	   context */
        if (*ctxt).finishDtd == 0xabcd1234 as std::os::raw::c_uint ||
               (*ctxt).finishDtd == 0xabcd1235 as std::os::raw::c_uint {
            let mut delta: std::os::raw::c_long =
                (ctxt as
                     *mut std::os::raw::c_char).offset_from((*ctxt).userData
                                                                 as
                                                                 *mut std::os::raw::c_char)
                    as std::os::raw::c_long;
            if delta > 0 as std::os::raw::c_int as std::os::raw::c_long &&
                   delta < 250 as std::os::raw::c_int as std::os::raw::c_long {
                pctxt = (*ctxt).userData as xmlParserCtxtPtr
            }
        }
    }
    __xmlRaiseError(schannel, channel, data, pctxt as *mut std::os::raw::c_void,
                    node as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                    error as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    str3 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2, str3);
}
/* LIBXML_VALID_ENABLED or LIBXML_SCHEMAS_ENABLED */
/* *
 * xmlErrValidNodeNr:
 * @ctxt:  an XML validation parser context
 * @node:  the node raising the error
 * @error:  the error number
 * @str1:  extra informations
 * @int2:  extra informations
 * @str3:  extra informations
 *
 * Handle a validation error, provide contextual informations
 */
unsafe extern "C" fn xmlErrValidNodeNr(mut ctxt: xmlValidCtxtPtr,
                                       mut node: xmlNodePtr,
                                       mut error: xmlParserErrors,
                                       mut msg: *const std::os::raw::c_char,
                                       mut str1: *const xmlChar,
                                       mut int2: std::os::raw::c_int,
                                       mut str3: *const xmlChar) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        channel = (*ctxt).error;
        data = (*ctxt).userData;
        /* Use the special values to detect if it is part of a parsing
	   context */
        if (*ctxt).finishDtd == 0xabcd1234 as std::os::raw::c_uint ||
               (*ctxt).finishDtd == 0xabcd1235 as std::os::raw::c_uint {
            let mut delta: std::os::raw::c_long =
                (ctxt as
                     *mut std::os::raw::c_char).offset_from((*ctxt).userData
                                                                 as
                                                                 *mut std::os::raw::c_char)
                    as std::os::raw::c_long;
            if delta > 0 as std::os::raw::c_int as std::os::raw::c_long &&
                   delta < 250 as std::os::raw::c_int as std::os::raw::c_long {
                pctxt = (*ctxt).userData as xmlParserCtxtPtr
            }
        }
    }
    __xmlRaiseError(schannel, channel, data, pctxt as *mut std::os::raw::c_void,
                    node as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                    error as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str3 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, int2, 0 as std::os::raw::c_int, msg,
                    str1, int2, str3);
}
/* *
 * xmlErrValidWarning:
 * @ctxt:  an XML validation parser context
 * @node:  the node raising the error
 * @error:  the error number
 * @str1:  extra information
 * @str2:  extra information
 * @str3:  extra information
 *
 * Handle a validation error, provide contextual information
 */
unsafe extern "C" fn xmlErrValidWarning(mut ctxt: xmlValidCtxtPtr,
                                        mut node: xmlNodePtr,
                                        mut error: xmlParserErrors,
                                        mut msg: *const std::os::raw::c_char,
                                        mut str1: *const xmlChar,
                                        mut str2: *const xmlChar,
                                        mut str3: *const xmlChar) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        channel = (*ctxt).warning;
        data = (*ctxt).userData;
        /* Use the special values to detect if it is part of a parsing
	   context */
        if (*ctxt).finishDtd == 0xabcd1234 as std::os::raw::c_uint ||
               (*ctxt).finishDtd == 0xabcd1235 as std::os::raw::c_uint {
            let mut delta: std::os::raw::c_long =
                (ctxt as
                     *mut std::os::raw::c_char).offset_from((*ctxt).userData
                                                                 as
                                                                 *mut std::os::raw::c_char)
                    as std::os::raw::c_long;
            if delta > 0 as std::os::raw::c_int as std::os::raw::c_long &&
                   delta < 250 as std::os::raw::c_int as std::os::raw::c_long {
                pctxt = (*ctxt).userData as xmlParserCtxtPtr
            }
        }
    }
    __xmlRaiseError(schannel, channel, data, pctxt as *mut std::os::raw::c_void,
                    node as *mut std::os::raw::c_void, XML_FROM_VALID as std::os::raw::c_int,
                    error as std::os::raw::c_int, XML_ERR_WARNING,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    str3 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2, str3);
}
unsafe extern "C" fn vstateVPush(mut ctxt: xmlValidCtxtPtr,
                                 mut elemDecl: xmlElementPtr,
                                 mut node: xmlNodePtr) -> std::os::raw::c_int {
    if (*ctxt).vstateMax == 0 as std::os::raw::c_int || (*ctxt).vstateTab.is_null() {
        (*ctxt).vstateMax = 10 as std::os::raw::c_int;
        (*ctxt).vstateTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).vstateMax
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlValidState>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlValidState;
        if (*ctxt).vstateTab.is_null() {
            xmlVErrMemory(ctxt,
                          b"malloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if (*ctxt).vstateNr >= (*ctxt).vstateMax {
        let mut tmp: *mut xmlValidState = 0 as *mut xmlValidState;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*ctxt).vstateTab
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           ((2 as std::os::raw::c_int
                                                                 *
                                                                 (*ctxt).vstateMax)
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlValidState>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlValidState;
        if tmp.is_null() {
            xmlVErrMemory(ctxt,
                          b"realloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*ctxt).vstateMax *= 2 as std::os::raw::c_int;
        (*ctxt).vstateTab = tmp
    }
    (*ctxt).vstate =
        &mut *(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize) as
            *mut xmlValidState;
    let ref mut fresh0 =
        (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).elemDecl;
    *fresh0 = elemDecl;
    let ref mut fresh1 =
        (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).node;
    *fresh1 = node;
    if !elemDecl.is_null() &&
           (*elemDecl).etype as std::os::raw::c_uint ==
               XML_ELEMENT_TYPE_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint {
        if (*elemDecl).contModel.is_null() {
            xmlValidBuildContentModel(ctxt, elemDecl);
        }
        if !(*elemDecl).contModel.is_null() {
            let ref mut fresh2 =
                (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec;
            *fresh2 =
                xmlRegNewExecCtxt((*elemDecl).contModel, None,
                                  0 as *mut std::os::raw::c_void)
        } else {
            let ref mut fresh3 =
                (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec;
            *fresh3 = 0 as xmlRegExecCtxtPtr;
            xmlErrValidNode(ctxt, elemDecl as xmlNodePtr,
                            XML_ERR_INTERNAL_ERROR,
                            b"Failed to build content model regexp for %s\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*node).name, 0 as *const xmlChar,
                            0 as *const xmlChar);
        }
    }
    let fresh4 = (*ctxt).vstateNr;
    (*ctxt).vstateNr = (*ctxt).vstateNr + 1;
    return fresh4;
}
unsafe extern "C" fn vstateVPop(mut ctxt: xmlValidCtxtPtr) -> std::os::raw::c_int {
    let mut elemDecl: xmlElementPtr = 0 as *mut xmlElement;
    if (*ctxt).vstateNr < 1 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    (*ctxt).vstateNr -= 1;
    elemDecl =
        (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).elemDecl;
    let ref mut fresh5 =
        (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).elemDecl;
    *fresh5 = 0 as xmlElementPtr;
    let ref mut fresh6 =
        (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).node;
    *fresh6 = 0 as xmlNodePtr;
    if !elemDecl.is_null() &&
           (*elemDecl).etype as std::os::raw::c_uint ==
               XML_ELEMENT_TYPE_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint {
        xmlRegFreeExecCtxt((*(*ctxt).vstateTab.offset((*ctxt).vstateNr as
                                                          isize)).exec);
    }
    let ref mut fresh7 =
        (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec;
    *fresh7 = 0 as xmlRegExecCtxtPtr;
    if (*ctxt).vstateNr >= 1 as std::os::raw::c_int {
        (*ctxt).vstate =
            &mut *(*ctxt).vstateTab.offset(((*ctxt).vstateNr -
                                                1 as std::os::raw::c_int) as isize) as
                *mut xmlValidState
    } else { (*ctxt).vstate = 0 as *mut xmlValidState }
    return (*ctxt).vstateNr;
}
/* not LIBXML_REGEXP_ENABLED */
/* LIBXML_REGEXP_ENABLED */
unsafe extern "C" fn nodeVPush(mut ctxt: xmlValidCtxtPtr,
                               mut value: xmlNodePtr) -> std::os::raw::c_int {
    if (*ctxt).nodeMax <= 0 as std::os::raw::c_int {
        (*ctxt).nodeMax = 4 as std::os::raw::c_int;
        (*ctxt).nodeTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).nodeMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if (*ctxt).nodeTab.is_null() {
            xmlVErrMemory(ctxt,
                          b"malloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            (*ctxt).nodeMax = 0 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
    }
    if (*ctxt).nodeNr >= (*ctxt).nodeMax {
        let mut tmp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*ctxt).nodeTab as
                                                               *mut std::os::raw::c_void,
                                                           (((*ctxt).nodeMax *
                                                                 2 as
                                                                     std::os::raw::c_int)
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlNodePtr;
        if tmp.is_null() {
            xmlVErrMemory(ctxt,
                          b"realloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
        (*ctxt).nodeMax *= 2 as std::os::raw::c_int;
        (*ctxt).nodeTab = tmp
    }
    let ref mut fresh8 = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    *fresh8 = value;
    (*ctxt).node = value;
    let fresh9 = (*ctxt).nodeNr;
    (*ctxt).nodeNr = (*ctxt).nodeNr + 1;
    return fresh9;
}
unsafe extern "C" fn nodeVPop(mut ctxt: xmlValidCtxtPtr) -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    if (*ctxt).nodeNr <= 0 as std::os::raw::c_int { return 0 as xmlNodePtr }
    (*ctxt).nodeNr -= 1;
    if (*ctxt).nodeNr > 0 as std::os::raw::c_int {
        (*ctxt).node =
            *(*ctxt).nodeTab.offset(((*ctxt).nodeNr - 1 as std::os::raw::c_int) as
                                        isize)
    } else { (*ctxt).node = 0 as xmlNodePtr }
    ret = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    let ref mut fresh10 = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    *fresh10 = 0 as xmlNodePtr;
    return ret;
}
/* TODO: use hash table for accesses to elem and attribute definitions */
/* ***********************************************************************
 *									*
 *		Content model validation based on the regexps		*
 *									*
 ************************************************************************/
/* *
 * xmlValidBuildAContentModel:
 * @content:  the content model
 * @ctxt:  the schema parser context
 * @name:  the element name whose content is being built
 *
 * Generate the automata sequence needed for that type
 *
 * Returns 1 if successful or 0 in case of error.
 */
unsafe extern "C" fn xmlValidBuildAContentModel(mut content:
                                                    xmlElementContentPtr,
                                                mut ctxt: xmlValidCtxtPtr,
                                                mut name: *const xmlChar)
 -> std::os::raw::c_int {
    if content.is_null() {
        xmlErrValidNode(ctxt, 0 as xmlNodePtr, XML_ERR_INTERNAL_ERROR,
                        b"Found NULL content in content model of %s\n\x00" as
                            *const u8 as *const std::os::raw::c_char, name,
                        0 as *const xmlChar, 0 as *const xmlChar);
        return 0 as std::os::raw::c_int
    }
    match (*content).type_0 as std::os::raw::c_uint {
        1 => {
            xmlErrValidNode(ctxt, 0 as xmlNodePtr, XML_ERR_INTERNAL_ERROR,
                            b"Found PCDATA in content model of %s\n\x00" as
                                *const u8 as *const std::os::raw::c_char, name,
                            0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
        2 => {
            let mut oldstate: xmlAutomataStatePtr = (*ctxt).state;
            let mut fn_0: [xmlChar; 50] = [0; 50];
            let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
            fullname =
                xmlBuildQName((*content).name, (*content).prefix,
                              fn_0.as_mut_ptr(), 50 as std::os::raw::c_int);
            if fullname.is_null() {
                xmlVErrMemory(ctxt,
                              b"Building content model\x00" as *const u8 as
                                  *const std::os::raw::c_char);
                return 0 as std::os::raw::c_int
            }
            match (*content).ocur as std::os::raw::c_uint {
                1 => {
                    (*ctxt).state =
                        xmlAutomataNewTransition((*ctxt).am, (*ctxt).state,
                                                 0 as xmlAutomataStatePtr,
                                                 fullname,
                                                 0 as *mut std::os::raw::c_void)
                }
                2 => {
                    (*ctxt).state =
                        xmlAutomataNewTransition((*ctxt).am, (*ctxt).state,
                                                 0 as xmlAutomataStatePtr,
                                                 fullname,
                                                 0 as *mut std::os::raw::c_void);
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate,
                                          (*ctxt).state);
                }
                4 => {
                    (*ctxt).state =
                        xmlAutomataNewTransition((*ctxt).am, (*ctxt).state,
                                                 0 as xmlAutomataStatePtr,
                                                 fullname,
                                                 0 as *mut std::os::raw::c_void);
                    xmlAutomataNewTransition((*ctxt).am, (*ctxt).state,
                                             (*ctxt).state, fullname,
                                             0 as *mut std::os::raw::c_void);
                }
                3 => {
                    (*ctxt).state =
                        xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state,
                                              0 as xmlAutomataStatePtr);
                    xmlAutomataNewTransition((*ctxt).am, (*ctxt).state,
                                             (*ctxt).state, fullname,
                                             0 as *mut std::os::raw::c_void);
                }
                _ => { }
            }
            if fullname != fn_0.as_mut_ptr() &&
                   fullname != (*content).name as *mut xmlChar {
                xmlFree.expect("non-null function pointer")(fullname as
                                                                *mut std::os::raw::c_void);
            }
        }
        3 => {
            let mut oldstate_0: xmlAutomataStatePtr =
                0 as *mut xmlAutomataState;
            let mut oldend: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
            let mut ocur: xmlElementContentOccur =
                0 as xmlElementContentOccur;
            /*
	     * Simply iterate over the content
	     */
            oldstate_0 = (*ctxt).state;
            ocur = (*content).ocur;
            if ocur as std::os::raw::c_uint !=
                   XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as std::os::raw::c_uint {
                (*ctxt).state =
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_0,
                                          0 as xmlAutomataStatePtr);
                oldstate_0 = (*ctxt).state
            }
            loop  {
                xmlValidBuildAContentModel((*content).c1, ctxt, name);
                content = (*content).c2;
                if !((*content).type_0 as std::os::raw::c_uint ==
                         XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as
                             std::os::raw::c_uint &&
                         (*content).ocur as std::os::raw::c_uint ==
                             XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as
                                 std::os::raw::c_uint) {
                    break ;
                }
            }
            xmlValidBuildAContentModel(content, ctxt, name);
            oldend = (*ctxt).state;
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, oldend,
                                      0 as xmlAutomataStatePtr);
            match ocur as std::os::raw::c_uint {
                2 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_0,
                                          (*ctxt).state);
                }
                3 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_0,
                                          (*ctxt).state);
                    xmlAutomataNewEpsilon((*ctxt).am, oldend, oldstate_0);
                }
                4 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldend, oldstate_0);
                }
                1 | _ => { }
            }
        }
        4 => {
            let mut oldstate_1: xmlAutomataStatePtr =
                0 as *mut xmlAutomataState;
            let mut oldend_0: xmlAutomataStatePtr =
                0 as *mut xmlAutomataState;
            let mut ocur_0: xmlElementContentOccur =
                0 as xmlElementContentOccur;
            ocur_0 = (*content).ocur;
            if ocur_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_CONTENT_PLUS as std::os::raw::c_int as std::os::raw::c_uint ||
                   ocur_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_CONTENT_MULT as std::os::raw::c_int as std::os::raw::c_uint
               {
                (*ctxt).state =
                    xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state,
                                          0 as xmlAutomataStatePtr)
            }
            oldstate_1 = (*ctxt).state;
            oldend_0 = xmlAutomataNewState((*ctxt).am);
            loop 
                 /*
	     * iterate over the subtypes and remerge the end with an
	     * epsilon transition
	     */
                 {
                (*ctxt).state = oldstate_1;
                xmlValidBuildAContentModel((*content).c1, ctxt, name);
                xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldend_0);
                content = (*content).c2;
                if !((*content).type_0 as std::os::raw::c_uint ==
                         XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint
                         &&
                         (*content).ocur as std::os::raw::c_uint ==
                             XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as
                                 std::os::raw::c_uint) {
                    break ;
                }
            }
            (*ctxt).state = oldstate_1;
            xmlValidBuildAContentModel(content, ctxt, name);
            xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldend_0);
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, oldend_0,
                                      0 as xmlAutomataStatePtr);
            match ocur_0 as std::os::raw::c_uint {
                2 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_1,
                                          (*ctxt).state);
                }
                3 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_1,
                                          (*ctxt).state);
                    xmlAutomataNewEpsilon((*ctxt).am, oldend_0, oldstate_1);
                }
                4 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldend_0, oldstate_1);
                }
                1 | _ => { }
            }
        }
        _ => {
            xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                        b"ContentModel broken for element %s\n\x00" as
                            *const u8 as *const std::os::raw::c_char,
                        name as *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidBuildContentModel:
 * @ctxt:  a validation context
 * @elem:  an element declaration node
 *
 * (Re)Build the automata associated to the content model of this
 * element
 *
 * Returns 1 in case of success, 0 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidBuildContentModel(mut ctxt: xmlValidCtxtPtr,
                                                   mut elem: xmlElementPtr)
 -> std::os::raw::c_int {
    if ctxt.is_null() || elem.is_null() { return 0 as std::os::raw::c_int }
    if (*elem).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*elem).etype as std::os::raw::c_uint !=
           XML_ELEMENT_TYPE_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    }
    /* TODO: should we rebuild in this case ? */
    if !(*elem).contModel.is_null() {
        if xmlRegexpIsDeterminist((*elem).contModel) == 0 {
            (*ctxt).valid = 0 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
        return 1 as std::os::raw::c_int
    }
    (*ctxt).am = xmlNewAutomata();
    if (*ctxt).am.is_null() {
        xmlErrValidNode(ctxt, elem as xmlNodePtr, XML_ERR_INTERNAL_ERROR,
                        b"Cannot create automata for element %s\n\x00" as
                            *const u8 as *const std::os::raw::c_char, (*elem).name,
                        0 as *const xmlChar, 0 as *const xmlChar);
        return 0 as std::os::raw::c_int
    }
    (*ctxt).state = xmlAutomataGetInitState((*ctxt).am);
    xmlValidBuildAContentModel((*elem).content, ctxt, (*elem).name);
    xmlAutomataSetFinalState((*ctxt).am, (*ctxt).state);
    (*elem).contModel = xmlAutomataCompile((*ctxt).am);
    if xmlRegexpIsDeterminist((*elem).contModel) != 1 as std::os::raw::c_int {
        let mut expr: [std::os::raw::c_char; 5000] = [0; 5000];
        expr[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
        xmlSnprintfElementContent(expr.as_mut_ptr(), 5000 as std::os::raw::c_int,
                                  (*elem).content, 1 as std::os::raw::c_int);
        xmlErrValidNode(ctxt, elem as xmlNodePtr,
                        XML_DTD_CONTENT_NOT_DETERMINIST,
                        b"Content model of %s is not determinist: %s\n\x00" as
                            *const u8 as *const std::os::raw::c_char, (*elem).name,
                        expr.as_mut_ptr() as *mut xmlChar,
                        0 as *const xmlChar);
        (*ctxt).valid = 0 as std::os::raw::c_int;
        (*ctxt).state = 0 as xmlAutomataStatePtr;
        xmlFreeAutomata((*ctxt).am);
        (*ctxt).am = 0 as xmlAutomataPtr;
        return 0 as std::os::raw::c_int
    }
    (*ctxt).state = 0 as xmlAutomataStatePtr;
    xmlFreeAutomata((*ctxt).am);
    (*ctxt).am = 0 as xmlAutomataPtr;
    return 1 as std::os::raw::c_int;
}
/* LIBXML_REGEXP_ENABLED */
/* ***************************************************************
 *								*
 *	Util functions for data allocation/deallocation		*
 *								*
 ****************************************************************/
/* *
 * xmlNewValidCtxt:
 *
 * Allocate a validation context structure.
 *
 * Returns NULL if not, otherwise the new validation context structure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewValidCtxt() -> xmlValidCtxtPtr {
    let mut ret: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlValidCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlValidCtxtPtr;
    if ret.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlValidCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlValidCtxt>() as std::os::raw::c_ulong);
    return ret;
}
/* *
 * xmlFreeValidCtxt:
 * @cur:  the validation context to free
 *
 * Free a validation context structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeValidCtxt(mut cur: xmlValidCtxtPtr) {
    if !(*cur).vstateTab.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).vstateTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*cur).nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).nodeTab as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* LIBXML_VALID_ENABLED */
/* *
 * xmlNewDocElementContent:
 * @doc:  the document
 * @name:  the subelement name or NULL
 * @type:  the type of element content decl
 *
 * Allocate an element content structure for the document.
 *
 * Returns NULL if not, otherwise the new element content structure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocElementContent(mut doc: xmlDocPtr,
                                                 mut name: *const xmlChar,
                                                 mut type_0:
                                                     xmlElementContentType)
 -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = 0 as *mut xmlElementContent;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if !doc.is_null() { dict = (*doc).dict }
    match type_0 as std::os::raw::c_uint {
        2 => {
            if name.is_null() {
                xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                            b"xmlNewElementContent : name == NULL !\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
            }
        }
        1 | 3 | 4 => {
            if !name.is_null() {
                xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                            b"xmlNewElementContent : name != NULL !\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
            }
        }
        _ => {
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ELEMENT content corrupted invalid type\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
            return 0 as xmlElementContentPtr
        }
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlElementContent>()
                                                          as std::os::raw::c_ulong) as
            xmlElementContentPtr;
    if ret.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlElementContentPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlElementContent>() as std::os::raw::c_ulong);
    (*ret).type_0 = type_0;
    (*ret).ocur = XML_ELEMENT_CONTENT_ONCE;
    if !name.is_null() {
        let mut l: std::os::raw::c_int = 0;
        let mut tmp: *const xmlChar = 0 as *const xmlChar;
        tmp = xmlSplitQName3(name, &mut l);
        if tmp.is_null() {
            if dict.is_null() {
                (*ret).name = xmlStrdup(name)
            } else {
                (*ret).name = xmlDictLookup(dict, name, -(1 as std::os::raw::c_int))
            }
        } else if dict.is_null() {
            (*ret).prefix = xmlStrndup(name, l);
            (*ret).name = xmlStrdup(tmp)
        } else {
            (*ret).prefix = xmlDictLookup(dict, name, l);
            (*ret).name = xmlDictLookup(dict, tmp, -(1 as std::os::raw::c_int))
        }
    }
    return ret;
}
/* *
 * xmlNewElementContent:
 * @name:  the subelement name or NULL
 * @type:  the type of element content decl
 *
 * Allocate an element content structure.
 * Deprecated in favor of xmlNewDocElementContent
 *
 * Returns NULL if not, otherwise the new element content structure
 */
#[no_mangle]
pub unsafe extern "C" fn xmlNewElementContent(mut name: *const xmlChar,
                                              mut type_0:
                                                  xmlElementContentType)
 -> xmlElementContentPtr {
    return xmlNewDocElementContent(0 as xmlDocPtr, name, type_0);
}
/* *
 * xmlCopyDocElementContent:
 * @doc:  the document owning the element declaration
 * @cur:  An element content pointer.
 *
 * Build a copy of an element content description.
 *
 * Returns the new xmlElementContentPtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyDocElementContent(mut doc: xmlDocPtr,
                                                  mut cur:
                                                      xmlElementContentPtr)
 -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut prev: xmlElementContentPtr = 0 as xmlElementContentPtr;
    let mut tmp: xmlElementContentPtr = 0 as *mut xmlElementContent;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if cur.is_null() { return 0 as xmlElementContentPtr }
    if !doc.is_null() { dict = (*doc).dict }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlElementContent>()
                                                          as std::os::raw::c_ulong) as
            xmlElementContentPtr;
    if ret.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlElementContentPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlElementContent>() as std::os::raw::c_ulong);
    (*ret).type_0 = (*cur).type_0;
    (*ret).ocur = (*cur).ocur;
    if !(*cur).name.is_null() {
        if !dict.is_null() {
            (*ret).name =
                xmlDictLookup(dict, (*cur).name, -(1 as std::os::raw::c_int))
        } else { (*ret).name = xmlStrdup((*cur).name) }
    }
    if !(*cur).prefix.is_null() {
        if !dict.is_null() {
            (*ret).prefix =
                xmlDictLookup(dict, (*cur).prefix, -(1 as std::os::raw::c_int))
        } else { (*ret).prefix = xmlStrdup((*cur).prefix) }
    }
    if !(*cur).c1.is_null() {
        (*ret).c1 = xmlCopyDocElementContent(doc, (*cur).c1)
    }
    if !(*ret).c1.is_null() { (*(*ret).c1).parent = ret }
    if !(*cur).c2.is_null() {
        prev = ret;
        cur = (*cur).c2;
        while !cur.is_null() {
            tmp =
                xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlElementContent>()
                                                                  as
                                                                  std::os::raw::c_ulong)
                    as xmlElementContentPtr;
            if tmp.is_null() {
                xmlVErrMemory(0 as xmlValidCtxtPtr,
                              b"malloc failed\x00" as *const u8 as
                                  *const std::os::raw::c_char);
                return ret
            }
            memset(tmp as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                   ::std::mem::size_of::<xmlElementContent>() as
                       std::os::raw::c_ulong);
            (*tmp).type_0 = (*cur).type_0;
            (*tmp).ocur = (*cur).ocur;
            (*prev).c2 = tmp;
            if !(*cur).name.is_null() {
                if !dict.is_null() {
                    (*tmp).name =
                        xmlDictLookup(dict, (*cur).name, -(1 as std::os::raw::c_int))
                } else { (*tmp).name = xmlStrdup((*cur).name) }
            }
            if !(*cur).prefix.is_null() {
                if !dict.is_null() {
                    (*tmp).prefix =
                        xmlDictLookup(dict, (*cur).prefix,
                                      -(1 as std::os::raw::c_int))
                } else { (*tmp).prefix = xmlStrdup((*cur).prefix) }
            }
            if !(*cur).c1.is_null() {
                (*tmp).c1 = xmlCopyDocElementContent(doc, (*cur).c1)
            }
            if !(*tmp).c1.is_null() { (*(*tmp).c1).parent = ret }
            prev = tmp;
            cur = (*cur).c2
        }
    }
    return ret;
}
/* *
 * xmlCopyElementContent:
 * @cur:  An element content pointer.
 *
 * Build a copy of an element content description.
 * Deprecated, use xmlCopyDocElementContent instead
 *
 * Returns the new xmlElementContentPtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyElementContent(mut cur: xmlElementContentPtr)
 -> xmlElementContentPtr {
    return xmlCopyDocElementContent(0 as xmlDocPtr, cur);
}
/* *
 * xmlFreeDocElementContent:
 * @doc: the document owning the element declaration
 * @cur:  the element content tree to free
 *
 * Free an element content structure. The whole subtree is removed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeDocElementContent(mut doc: xmlDocPtr,
                                                  mut cur:
                                                      xmlElementContentPtr) {
    let mut next: xmlElementContentPtr = 0 as *mut xmlElementContent;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if !doc.is_null() { dict = (*doc).dict }
    while !cur.is_null() {
        next = (*cur).c2;
        match (*cur).type_0 as std::os::raw::c_uint {
            1 | 2 | 3 | 4 => { }
            _ => {
                xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                            b"Internal: ELEMENT content corrupted invalid type\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                return
            }
        }
        if !(*cur).c1.is_null() { xmlFreeDocElementContent(doc, (*cur).c1); }
        if !dict.is_null() {
            if !(*cur).name.is_null() && xmlDictOwns(dict, (*cur).name) == 0 {
                xmlFree.expect("non-null function pointer")((*cur).name as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            if !(*cur).prefix.is_null() &&
                   xmlDictOwns(dict, (*cur).prefix) == 0 {
                xmlFree.expect("non-null function pointer")((*cur).prefix as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
        } else {
            if !(*cur).name.is_null() {
                xmlFree.expect("non-null function pointer")((*cur).name as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            if !(*cur).prefix.is_null() {
                xmlFree.expect("non-null function pointer")((*cur).prefix as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
        }
        xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
        cur = next
    };
}
/* *
 * xmlFreeElementContent:
 * @cur:  the element content tree to free
 *
 * Free an element content structure. The whole subtree is removed.
 * Deprecated, use xmlFreeDocElementContent instead
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeElementContent(mut cur:
                                                   xmlElementContentPtr) {
    xmlFreeDocElementContent(0 as xmlDocPtr, cur);
}
/* *
 * xmlDumpElementContent:
 * @buf:  An XML buffer
 * @content:  An element table
 * @glob: 1 if one must print the englobing parenthesis, 0 otherwise
 *
 * This will dump the content of the element table as an XML DTD definition
 */
unsafe extern "C" fn xmlDumpElementContent(mut buf: xmlBufferPtr,
                                           mut content: xmlElementContentPtr,
                                           mut glob: std::os::raw::c_int) {
    if content.is_null() { return }
    if glob != 0 {
        xmlBufferWriteChar(buf, b"(\x00" as *const u8 as *const std::os::raw::c_char);
    }
    match (*content).type_0 as std::os::raw::c_uint {
        1 => {
            xmlBufferWriteChar(buf,
                               b"#PCDATA\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        2 => {
            if !(*content).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*content).prefix);
                xmlBufferWriteChar(buf,
                                   b":\x00" as *const u8 as
                                       *const std::os::raw::c_char);
            }
            xmlBufferWriteCHAR(buf, (*content).name);
        }
        3 => {
            if !(*content).c1.is_null() &&
                   ((*(*content).c1).type_0 as std::os::raw::c_uint ==
                        XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint
                        ||
                        (*(*content).c1).type_0 as std::os::raw::c_uint ==
                            XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as
                                std::os::raw::c_uint) {
                xmlDumpElementContent(buf, (*content).c1, 1 as std::os::raw::c_int);
            } else {
                xmlDumpElementContent(buf, (*content).c1, 0 as std::os::raw::c_int);
            }
            xmlBufferWriteChar(buf,
                               b" , \x00" as *const u8 as
                                   *const std::os::raw::c_char);
            if !(*content).c2.is_null() &&
                   ((*(*content).c2).type_0 as std::os::raw::c_uint ==
                        XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint
                        ||
                        (*(*content).c2).type_0 as std::os::raw::c_uint ==
                            XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as
                                std::os::raw::c_uint &&
                            (*(*content).c2).ocur as std::os::raw::c_uint !=
                                XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as
                                    std::os::raw::c_uint) {
                xmlDumpElementContent(buf, (*content).c2, 1 as std::os::raw::c_int);
            } else {
                xmlDumpElementContent(buf, (*content).c2, 0 as std::os::raw::c_int);
            }
        }
        4 => {
            if !(*content).c1.is_null() &&
                   ((*(*content).c1).type_0 as std::os::raw::c_uint ==
                        XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint
                        ||
                        (*(*content).c1).type_0 as std::os::raw::c_uint ==
                            XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as
                                std::os::raw::c_uint) {
                xmlDumpElementContent(buf, (*content).c1, 1 as std::os::raw::c_int);
            } else {
                xmlDumpElementContent(buf, (*content).c1, 0 as std::os::raw::c_int);
            }
            xmlBufferWriteChar(buf,
                               b" | \x00" as *const u8 as
                                   *const std::os::raw::c_char);
            if !(*content).c2.is_null() &&
                   ((*(*content).c2).type_0 as std::os::raw::c_uint ==
                        XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as std::os::raw::c_uint
                        ||
                        (*(*content).c2).type_0 as std::os::raw::c_uint ==
                            XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as
                                std::os::raw::c_uint &&
                            (*(*content).c2).ocur as std::os::raw::c_uint !=
                                XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as
                                    std::os::raw::c_uint) {
                xmlDumpElementContent(buf, (*content).c2, 1 as std::os::raw::c_int);
            } else {
                xmlDumpElementContent(buf, (*content).c2, 0 as std::os::raw::c_int);
            }
        }
        _ => {
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ELEMENT content corrupted invalid type\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
        }
    }
    if glob != 0 {
        xmlBufferWriteChar(buf, b")\x00" as *const u8 as *const std::os::raw::c_char);
    }
    match (*content).ocur as std::os::raw::c_uint {
        2 => {
            xmlBufferWriteChar(buf,
                               b"?\x00" as *const u8 as *const std::os::raw::c_char);
        }
        3 => {
            xmlBufferWriteChar(buf,
                               b"*\x00" as *const u8 as *const std::os::raw::c_char);
        }
        4 => {
            xmlBufferWriteChar(buf,
                               b"+\x00" as *const u8 as *const std::os::raw::c_char);
        }
        1 | _ => { }
    };
}
/* *
 * xmlSprintfElementContent:
 * @buf:  an output buffer
 * @content:  An element table
 * @englob: 1 if one must print the englobing parenthesis, 0 otherwise
 *
 * Deprecated, unsafe, use xmlSnprintfElementContent
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSprintfElementContent(mut buf: *mut std::os::raw::c_char,
                                                  mut content:
                                                      xmlElementContentPtr,
                                                  mut englob: std::os::raw::c_int) {
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlSnprintfElementContent:
 * @buf:  an output buffer
 * @size:  the buffer size
 * @content:  An element table
 * @englob: 1 if one must print the englobing parenthesis, 0 otherwise
 *
 * This will dump the content of the element content definition
 * Intended just for the debug routine
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSnprintfElementContent(mut buf: *mut std::os::raw::c_char,
                                                   mut size: std::os::raw::c_int,
                                                   mut content:
                                                       xmlElementContentPtr,
                                                   mut englob: std::os::raw::c_int) {
    let mut len: std::os::raw::c_int = 0;
    if content.is_null() { return }
    len = strlen(buf) as std::os::raw::c_int;
    if size - len < 50 as std::os::raw::c_int {
        if size - len > 4 as std::os::raw::c_int &&
               *buf.offset((len - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                   != '.' as i32 {
            strcat(buf, b" ...\x00" as *const u8 as *const std::os::raw::c_char);
        }
        return
    }
    if englob != 0 {
        strcat(buf, b"(\x00" as *const u8 as *const std::os::raw::c_char);
    }
    match (*content).type_0 as std::os::raw::c_uint {
        1 => {
            strcat(buf, b"#PCDATA\x00" as *const u8 as *const std::os::raw::c_char);
        }
        2 => {
            let mut qnameLen: std::os::raw::c_int = xmlStrlen((*content).name);
            if !(*content).prefix.is_null() {
                qnameLen += xmlStrlen((*content).prefix) + 1 as std::os::raw::c_int
            }
            if size - len < qnameLen + 10 as std::os::raw::c_int {
                strcat(buf, b" ...\x00" as *const u8 as *const std::os::raw::c_char);
                return
            }
            if !(*content).prefix.is_null() {
                strcat(buf, (*content).prefix as *mut std::os::raw::c_char);
                strcat(buf, b":\x00" as *const u8 as *const std::os::raw::c_char);
            }
            if !(*content).name.is_null() {
                strcat(buf, (*content).name as *mut std::os::raw::c_char);
            }
        }
        3 => {
            if (*(*content).c1).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*(*content).c1).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as std::os::raw::c_uint
               {
                xmlSnprintfElementContent(buf, size, (*content).c1,
                                          1 as std::os::raw::c_int);
            } else {
                xmlSnprintfElementContent(buf, size, (*content).c1,
                                          0 as std::os::raw::c_int);
            }
            len = strlen(buf) as std::os::raw::c_int;
            if size - len < 50 as std::os::raw::c_int {
                if size - len > 4 as std::os::raw::c_int &&
                       *buf.offset((len - 1 as std::os::raw::c_int) as isize) as
                           std::os::raw::c_int != '.' as i32 {
                    strcat(buf,
                           b" ...\x00" as *const u8 as *const std::os::raw::c_char);
                }
                return
            }
            strcat(buf, b" , \x00" as *const u8 as *const std::os::raw::c_char);
            if ((*(*content).c2).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*(*content).c2).ocur as std::os::raw::c_uint !=
                        XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as
                            std::os::raw::c_uint) &&
                   (*(*content).c2).type_0 as std::os::raw::c_uint !=
                       XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as
                           std::os::raw::c_uint {
                xmlSnprintfElementContent(buf, size, (*content).c2,
                                          1 as std::os::raw::c_int);
            } else {
                xmlSnprintfElementContent(buf, size, (*content).c2,
                                          0 as std::os::raw::c_int);
            }
        }
        4 => {
            if (*(*content).c1).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*(*content).c1).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as std::os::raw::c_uint
               {
                xmlSnprintfElementContent(buf, size, (*content).c1,
                                          1 as std::os::raw::c_int);
            } else {
                xmlSnprintfElementContent(buf, size, (*content).c1,
                                          0 as std::os::raw::c_int);
            }
            len = strlen(buf) as std::os::raw::c_int;
            if size - len < 50 as std::os::raw::c_int {
                if size - len > 4 as std::os::raw::c_int &&
                       *buf.offset((len - 1 as std::os::raw::c_int) as isize) as
                           std::os::raw::c_int != '.' as i32 {
                    strcat(buf,
                           b" ...\x00" as *const u8 as *const std::os::raw::c_char);
                }
                return
            }
            strcat(buf, b" | \x00" as *const u8 as *const std::os::raw::c_char);
            if ((*(*content).c2).type_0 as std::os::raw::c_uint ==
                    XML_ELEMENT_CONTENT_SEQ as std::os::raw::c_int as std::os::raw::c_uint ||
                    (*(*content).c2).ocur as std::os::raw::c_uint !=
                        XML_ELEMENT_CONTENT_ONCE as std::os::raw::c_int as
                            std::os::raw::c_uint) &&
                   (*(*content).c2).type_0 as std::os::raw::c_uint !=
                       XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as
                           std::os::raw::c_uint {
                xmlSnprintfElementContent(buf, size, (*content).c2,
                                          1 as std::os::raw::c_int);
            } else {
                xmlSnprintfElementContent(buf, size, (*content).c2,
                                          0 as std::os::raw::c_int);
            }
        }
        _ => { }
    }
    if (size as std::os::raw::c_ulong).wrapping_sub(strlen(buf)) <=
           2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return
    }
    if englob != 0 {
        strcat(buf, b")\x00" as *const u8 as *const std::os::raw::c_char);
    }
    match (*content).ocur as std::os::raw::c_uint {
        2 => { strcat(buf, b"?\x00" as *const u8 as *const std::os::raw::c_char); }
        3 => { strcat(buf, b"*\x00" as *const u8 as *const std::os::raw::c_char); }
        4 => { strcat(buf, b"+\x00" as *const u8 as *const std::os::raw::c_char); }
        1 | _ => { }
    };
}
/* ***************************************************************
 *								*
 *	Registration of DTD declarations			*
 *								*
 ****************************************************************/
/* *
 * xmlFreeElement:
 * @elem:  An element
 *
 * Deallocate the memory used by an element definition
 */
unsafe extern "C" fn xmlFreeElement(mut elem: xmlElementPtr) {
    if elem.is_null() { return }
    xmlUnlinkNode(elem as xmlNodePtr);
    xmlFreeDocElementContent((*elem).doc, (*elem).content);
    if !(*elem).name.is_null() {
        xmlFree.expect("non-null function pointer")((*elem).name as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*elem).prefix.is_null() {
        xmlFree.expect("non-null function pointer")((*elem).prefix as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*elem).contModel.is_null() { xmlRegFreeRegexp((*elem).contModel); }
    xmlFree.expect("non-null function pointer")(elem as *mut std::os::raw::c_void);
}
/* *
 * xmlAddElementDecl:
 * @ctxt:  the validation context
 * @dtd:  pointer to the DTD
 * @name:  the entity name
 * @type:  the element type
 * @content:  the element content tree or NULL
 *
 * Register a new element declaration
 *
 * Returns NULL if not, otherwise the entity
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddElementDecl(mut ctxt: xmlValidCtxtPtr,
                                           mut dtd: xmlDtdPtr,
                                           mut name: *const xmlChar,
                                           mut type_0: xmlElementTypeVal,
                                           mut content: xmlElementContentPtr)
 -> xmlElementPtr {
    let mut ret: xmlElementPtr = 0 as *mut xmlElement;
    let mut table: xmlElementTablePtr = 0 as *mut xmlElementTable;
    let mut oldAttributes: xmlAttributePtr = 0 as xmlAttributePtr;
    let mut ns: *mut xmlChar = 0 as *mut xmlChar;
    let mut uqname: *mut xmlChar = 0 as *mut xmlChar;
    if dtd.is_null() { return 0 as xmlElementPtr }
    if name.is_null() { return 0 as xmlElementPtr }
    match type_0 as std::os::raw::c_uint {
        1 => {
            if !content.is_null() {
                xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                            b"xmlAddElementDecl: content != NULL for EMPTY\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                return 0 as xmlElementPtr
            }
        }
        2 => {
            if !content.is_null() {
                xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                            b"xmlAddElementDecl: content != NULL for ANY\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                return 0 as xmlElementPtr
            }
        }
        3 => {
            if content.is_null() {
                xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                            b"xmlAddElementDecl: content == NULL for MIXED\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                return 0 as xmlElementPtr
            }
        }
        4 => {
            if content.is_null() {
                xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                            b"xmlAddElementDecl: content == NULL for ELEMENT\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                return 0 as xmlElementPtr
            }
        }
        _ => {
            xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ELEMENT decl corrupted invalid type\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
            return 0 as xmlElementPtr
        }
    }
    /*
     * check if name is a QName
     */
    uqname = xmlSplitQName2(name, &mut ns);
    if !uqname.is_null() { name = uqname }
    /*
     * Create the Element table if needed.
     */
    table = (*dtd).elements as xmlElementTablePtr;
    if table.is_null() {
        let mut dict: xmlDictPtr = 0 as xmlDictPtr;
        if !(*dtd).doc.is_null() { dict = (*(*dtd).doc).dict }
        table = xmlHashCreateDict(0 as std::os::raw::c_int, dict);
        (*dtd).elements = table as *mut std::os::raw::c_void
    }
    if table.is_null() {
        xmlVErrMemory(ctxt,
                      b"xmlAddElementDecl: Table creation failed!\n\x00" as
                          *const u8 as *const std::os::raw::c_char);
        if !uqname.is_null() {
            xmlFree.expect("non-null function pointer")(uqname as
                                                            *mut std::os::raw::c_void);
        }
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as
                                                            *mut std::os::raw::c_void);
        }
        return 0 as xmlElementPtr
    }
    /*
     * lookup old attributes inserted on an undefined element in the
     * internal subset.
     */
    if !(*dtd).doc.is_null() && !(*(*dtd).doc).intSubset.is_null() {
        ret =
            xmlHashLookup2((*(*(*dtd).doc).intSubset).elements as
                               xmlHashTablePtr, name, ns) as xmlElementPtr;
        if !ret.is_null() &&
               (*ret).etype as std::os::raw::c_uint ==
                   XML_ELEMENT_TYPE_UNDEFINED as std::os::raw::c_int as std::os::raw::c_uint {
            oldAttributes = (*ret).attributes;
            (*ret).attributes = 0 as xmlAttributePtr;
            xmlHashRemoveEntry2((*(*(*dtd).doc).intSubset).elements as
                                    xmlHashTablePtr, name, ns, None);
            xmlFreeElement(ret);
        }
    }
    /*
     * The element may already be present if one of its attribute
     * was registered first
     */
    ret = xmlHashLookup2(table, name, ns) as xmlElementPtr;
    if !ret.is_null() {
        if (*ret).etype as std::os::raw::c_uint !=
               XML_ELEMENT_TYPE_UNDEFINED as std::os::raw::c_int as std::os::raw::c_uint {
            /*
	     * The element is already defined in this DTD.
	     */
            xmlErrValidNode(ctxt, dtd as xmlNodePtr, XML_DTD_ELEM_REDEFINED,
                            b"Redefinition of element %s\n\x00" as *const u8
                                as *const std::os::raw::c_char, name,
                            0 as *const xmlChar, 0 as *const xmlChar);
            /* LIBXML_VALID_ENABLED */
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as
                                                                *mut std::os::raw::c_void);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as xmlElementPtr
        }
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as
                                                            *mut std::os::raw::c_void);
            ns = 0 as *mut xmlChar
        }
    } else {
        ret =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlElement>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlElementPtr;
        if ret.is_null() {
            xmlVErrMemory(ctxt,
                          b"malloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as
                                                                *mut std::os::raw::c_void);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as xmlElementPtr
        }
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlElement>() as std::os::raw::c_ulong);
        (*ret).type_0 = XML_ELEMENT_DECL;
        /*
	 * fill the structure.
	 */
        (*ret).name = xmlStrdup(name);
        if (*ret).name.is_null() {
            xmlVErrMemory(ctxt,
                          b"malloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as
                                                                *mut std::os::raw::c_void);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as
                                                                *mut std::os::raw::c_void);
            }
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlElementPtr
        }
        (*ret).prefix = ns;
        /*
	 * Validity Check:
	 * Insertion must not fail
	 */
        if xmlHashAddEntry2(table, name, ns, ret as *mut std::os::raw::c_void) != 0 {
            /*
	     * The element is already defined in this DTD.
	     */
            xmlErrValidNode(ctxt, dtd as xmlNodePtr, XML_DTD_ELEM_REDEFINED,
                            b"Redefinition of element %s\n\x00" as *const u8
                                as *const std::os::raw::c_char, name,
                            0 as *const xmlChar, 0 as *const xmlChar);
            /* LIBXML_VALID_ENABLED */
            xmlFreeElement(ret);
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as xmlElementPtr
        }
        /*
	 * For new element, may have attributes from earlier
	 * definition in internal subset
	 */
        (*ret).attributes = oldAttributes
    }
    /*
     * Finish to fill the structure.
     */
    (*ret).etype = type_0;
    /*
     * Avoid a stupid copy when called by the parser
     * and flag it by setting a special parent value
     * so the parser doesn't unallocate it.
     */
    if !ctxt.is_null() &&
           ((*ctxt).finishDtd == 0xabcd1234 as std::os::raw::c_uint ||
                (*ctxt).finishDtd == 0xabcd1235 as std::os::raw::c_uint) {
        (*ret).content = content;
        if !content.is_null() {
            (*content).parent = 1 as std::os::raw::c_int as xmlElementContentPtr
        }
    } else { (*ret).content = xmlCopyDocElementContent((*dtd).doc, content) }
    /*
     * Link it to the DTD
     */
    (*ret).parent = dtd;
    (*ret).doc = (*dtd).doc;
    if (*dtd).last.is_null() {
        (*dtd).last = ret as xmlNodePtr;
        (*dtd).children = (*dtd).last
    } else {
        (*(*dtd).last).next = ret as xmlNodePtr;
        (*ret).prev = (*dtd).last;
        (*dtd).last = ret as xmlNodePtr
    }
    if !uqname.is_null() {
        xmlFree.expect("non-null function pointer")(uqname as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
unsafe extern "C" fn xmlFreeElementTableEntry(mut elem: *mut std::os::raw::c_void,
                                              mut name: *const xmlChar) {
    xmlFreeElement(elem as xmlElementPtr);
}
/* *
 * xmlFreeElementTable:
 * @table:  An element table
 *
 * Deallocate the memory used by an element hash table.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeElementTable(mut table: xmlElementTablePtr) {
    xmlHashFree(table,
                Some(xmlFreeElementTableEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
}
/* *
 * xmlCopyElement:
 * @elem:  An element
 *
 * Build a copy of an element.
 *
 * Returns the new xmlElementPtr or NULL in case of error.
 */
unsafe extern "C" fn xmlCopyElement(mut payload: *mut std::os::raw::c_void,
                                    mut name: *const xmlChar)
 -> *mut std::os::raw::c_void {
    let mut elem: xmlElementPtr = payload as xmlElementPtr;
    let mut cur: xmlElementPtr = 0 as *mut xmlElement;
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlElement>()
                                                          as std::os::raw::c_ulong) as
            xmlElementPtr;
    if cur.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlElement>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ELEMENT_DECL;
    (*cur).etype = (*elem).etype;
    if !(*elem).name.is_null() {
        (*cur).name = xmlStrdup((*elem).name)
    } else { (*cur).name = 0 as *const xmlChar }
    if !(*elem).prefix.is_null() {
        (*cur).prefix = xmlStrdup((*elem).prefix)
    } else { (*cur).prefix = 0 as *const xmlChar }
    (*cur).content = xmlCopyElementContent((*elem).content);
    /* TODO : rebuild the attribute list on the copy */
    (*cur).attributes = 0 as xmlAttributePtr;
    return cur as *mut std::os::raw::c_void;
}
/* *
 * xmlCopyElementTable:
 * @table:  An element table
 *
 * Build a copy of an element table.
 *
 * Returns the new xmlElementTablePtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyElementTable(mut table: xmlElementTablePtr)
 -> xmlElementTablePtr {
    return xmlHashCopy(table,
                       Some(xmlCopyElement as
                                unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                     _: *const xmlChar)
                                    -> *mut std::os::raw::c_void)) as
               xmlElementTablePtr;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlDumpElementDecl:
 * @buf:  the XML buffer output
 * @elem:  An element table
 *
 * This will dump the content of the element declaration as an XML
 * DTD definition
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDumpElementDecl(mut buf: xmlBufferPtr,
                                            mut elem: xmlElementPtr) {
    if buf.is_null() || elem.is_null() { return }
    match (*elem).etype as std::os::raw::c_uint {
        1 => {
            xmlBufferWriteChar(buf,
                               b"<!ELEMENT \x00" as *const u8 as
                                   *const std::os::raw::c_char);
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf,
                                   b":\x00" as *const u8 as
                                       *const std::os::raw::c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf,
                               b" EMPTY>\n\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        2 => {
            xmlBufferWriteChar(buf,
                               b"<!ELEMENT \x00" as *const u8 as
                                   *const std::os::raw::c_char);
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf,
                                   b":\x00" as *const u8 as
                                       *const std::os::raw::c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf,
                               b" ANY>\n\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        3 => {
            xmlBufferWriteChar(buf,
                               b"<!ELEMENT \x00" as *const u8 as
                                   *const std::os::raw::c_char);
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf,
                                   b":\x00" as *const u8 as
                                       *const std::os::raw::c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf,
                               b" \x00" as *const u8 as *const std::os::raw::c_char);
            xmlDumpElementContent(buf, (*elem).content, 1 as std::os::raw::c_int);
            xmlBufferWriteChar(buf,
                               b">\n\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        4 => {
            xmlBufferWriteChar(buf,
                               b"<!ELEMENT \x00" as *const u8 as
                                   *const std::os::raw::c_char);
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf,
                                   b":\x00" as *const u8 as
                                       *const std::os::raw::c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf,
                               b" \x00" as *const u8 as *const std::os::raw::c_char);
            xmlDumpElementContent(buf, (*elem).content, 1 as std::os::raw::c_int);
            xmlBufferWriteChar(buf,
                               b">\n\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        _ => {
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ELEMENT struct corrupted invalid type\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
        }
    };
}
/* *
 * xmlDumpElementDeclScan:
 * @elem:  An element table
 * @buf:  the XML buffer output
 *
 * This routine is used by the hash scan function.  It just reverses
 * the arguments.
 */
unsafe extern "C" fn xmlDumpElementDeclScan(mut elem: *mut std::os::raw::c_void,
                                            mut buf: *mut std::os::raw::c_void,
                                            mut name: *const xmlChar) {
    xmlDumpElementDecl(buf as xmlBufferPtr, elem as xmlElementPtr);
}
/* *
 * xmlDumpElementTable:
 * @buf:  the XML buffer output
 * @table:  An element table
 *
 * This will dump the content of the element table as an XML DTD definition
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDumpElementTable(mut buf: xmlBufferPtr,
                                             mut table: xmlElementTablePtr) {
    if buf.is_null() || table.is_null() { return }
    xmlHashScan(table,
                Some(xmlDumpElementDeclScan as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()),
                buf as *mut std::os::raw::c_void);
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlCreateEnumeration:
 * @name:  the enumeration name or NULL
 *
 * create and initialize an enumeration attribute node.
 *
 * Returns the xmlEnumerationPtr just created or NULL in case
 *                of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCreateEnumeration(mut name: *const xmlChar)
 -> xmlEnumerationPtr {
    let mut ret: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlEnumeration>()
                                                          as std::os::raw::c_ulong) as
            xmlEnumerationPtr;
    if ret.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlEnumerationPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlEnumeration>() as std::os::raw::c_ulong);
    if !name.is_null() { (*ret).name = xmlStrdup(name) }
    return ret;
}
/* *
 * xmlFreeEnumeration:
 * @cur:  the tree to free.
 *
 * free an enumeration attribute node (recursive).
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeEnumeration(mut cur: xmlEnumerationPtr) {
    if cur.is_null() { return }
    if !(*cur).next.is_null() { xmlFreeEnumeration((*cur).next); }
    if !(*cur).name.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).name as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut std::os::raw::c_void);
}
/* *
 * xmlCopyEnumeration:
 * @cur:  the tree to copy.
 *
 * Copy an enumeration attribute node (recursive).
 *
 * Returns the xmlEnumerationPtr just created or NULL in case
 *                of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyEnumeration(mut cur: xmlEnumerationPtr)
 -> xmlEnumerationPtr {
    let mut ret: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    if cur.is_null() { return 0 as xmlEnumerationPtr }
    ret = xmlCreateEnumeration((*cur).name as *mut xmlChar);
    if ret.is_null() { return 0 as xmlEnumerationPtr }
    if !(*cur).next.is_null() {
        (*ret).next = xmlCopyEnumeration((*cur).next)
    } else { (*ret).next = 0 as *mut _xmlEnumeration }
    return ret;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlDumpEnumeration:
 * @buf:  the XML buffer output
 * @enum:  An enumeration
 *
 * This will dump the content of the enumeration
 */
unsafe extern "C" fn xmlDumpEnumeration(mut buf: xmlBufferPtr,
                                        mut cur: xmlEnumerationPtr) {
    if buf.is_null() || cur.is_null() { return }
    xmlBufferWriteCHAR(buf, (*cur).name);
    if (*cur).next.is_null() {
        xmlBufferWriteChar(buf, b")\x00" as *const u8 as *const std::os::raw::c_char);
    } else {
        xmlBufferWriteChar(buf,
                           b" | \x00" as *const u8 as *const std::os::raw::c_char);
        xmlDumpEnumeration(buf, (*cur).next);
    };
}
/* LIBXML_OUTPUT_ENABLED */
/* *
 * xmlScanIDAttributeDecl:
 * @ctxt:  the validation context
 * @elem:  the element name
 * @err: whether to raise errors here
 *
 * Verify that the element don't have too many ID attributes
 * declared.
 *
 * Returns the number of ID attributes found.
 */
unsafe extern "C" fn xmlScanIDAttributeDecl(mut ctxt: xmlValidCtxtPtr,
                                            mut elem: xmlElementPtr,
                                            mut err: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlAttributePtr = 0 as *mut xmlAttribute;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if elem.is_null() { return 0 as std::os::raw::c_int }
    cur = (*elem).attributes;
    while !cur.is_null() {
        if (*cur).atype as std::os::raw::c_uint ==
               XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
            ret += 1;
            if ret > 1 as std::os::raw::c_int && err != 0 {
                xmlErrValidNode(ctxt, elem as xmlNodePtr, XML_DTD_MULTIPLE_ID,
                                b"Element %s has too many ID attributes defined : %s\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                (*elem).name, (*cur).name,
                                0 as *const xmlChar);
            }
        }
        cur = (*cur).nexth
    }
    return ret;
}
/* LIBXML_VALID_ENABLED */
/* *
 * xmlFreeAttribute:
 * @elem:  An attribute
 *
 * Deallocate the memory used by an attribute definition
 */
unsafe extern "C" fn xmlFreeAttribute(mut attr: xmlAttributePtr) {
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if attr.is_null() { return }
    if !(*attr).doc.is_null() {
        dict = (*(*attr).doc).dict
    } else { dict = 0 as xmlDictPtr }
    xmlUnlinkNode(attr as xmlNodePtr);
    if !(*attr).tree.is_null() { xmlFreeEnumeration((*attr).tree); }
    if !dict.is_null() {
        if !(*attr).elem.is_null() && xmlDictOwns(dict, (*attr).elem) == 0 {
            xmlFree.expect("non-null function pointer")((*attr).elem as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        if !(*attr).name.is_null() && xmlDictOwns(dict, (*attr).name) == 0 {
            xmlFree.expect("non-null function pointer")((*attr).name as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        if !(*attr).prefix.is_null() && xmlDictOwns(dict, (*attr).prefix) == 0
           {
            xmlFree.expect("non-null function pointer")((*attr).prefix as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        if !(*attr).defaultValue.is_null() &&
               xmlDictOwns(dict, (*attr).defaultValue) == 0 {
            xmlFree.expect("non-null function pointer")((*attr).defaultValue
                                                            as *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
    } else {
        if !(*attr).elem.is_null() {
            xmlFree.expect("non-null function pointer")((*attr).elem as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        if !(*attr).name.is_null() {
            xmlFree.expect("non-null function pointer")((*attr).name as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        if !(*attr).defaultValue.is_null() {
            xmlFree.expect("non-null function pointer")((*attr).defaultValue
                                                            as *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        if !(*attr).prefix.is_null() {
            xmlFree.expect("non-null function pointer")((*attr).prefix as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
    }
    xmlFree.expect("non-null function pointer")(attr as *mut std::os::raw::c_void);
}
/* *
 * xmlAddAttributeDecl:
 * @ctxt:  the validation context
 * @dtd:  pointer to the DTD
 * @elem:  the element name
 * @name:  the attribute name
 * @ns:  the attribute namespace prefix
 * @type:  the attribute type
 * @def:  the attribute default type
 * @defaultValue:  the attribute default value
 * @tree:  if it's an enumeration, the associated list
 *
 * Register a new attribute declaration
 * Note that @tree becomes the ownership of the DTD
 *
 * Returns NULL if not new, otherwise the attribute decl
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddAttributeDecl(mut ctxt: xmlValidCtxtPtr,
                                             mut dtd: xmlDtdPtr,
                                             mut elem: *const xmlChar,
                                             mut name: *const xmlChar,
                                             mut ns: *const xmlChar,
                                             mut type_0: xmlAttributeType,
                                             mut def: xmlAttributeDefault,
                                             mut defaultValue: *const xmlChar,
                                             mut tree: xmlEnumerationPtr)
 -> xmlAttributePtr {
    let mut ret: xmlAttributePtr = 0 as *mut xmlAttribute;
    let mut table: xmlAttributeTablePtr = 0 as *mut xmlAttributeTable;
    let mut elemDef: xmlElementPtr = 0 as *mut xmlElement;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if dtd.is_null() { xmlFreeEnumeration(tree); return 0 as xmlAttributePtr }
    if name.is_null() {
        xmlFreeEnumeration(tree);
        return 0 as xmlAttributePtr
    }
    if elem.is_null() {
        xmlFreeEnumeration(tree);
        return 0 as xmlAttributePtr
    }
    if !(*dtd).doc.is_null() { dict = (*(*dtd).doc).dict }
    /*
     * Check the type and possibly the default value.
     */
    match type_0 as std::os::raw::c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => { }
        _ => {
            xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ATTRIBUTE struct corrupted invalid type\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
            xmlFreeEnumeration(tree);
            return 0 as xmlAttributePtr
        }
    }
    if !defaultValue.is_null() &&
           xmlValidateAttributeValueInternal((*dtd).doc, type_0, defaultValue)
               == 0 {
        xmlErrValidNode(ctxt, dtd as xmlNodePtr, XML_DTD_ATTRIBUTE_DEFAULT,
                        b"Attribute %s of %s: invalid default value\n\x00" as
                            *const u8 as *const std::os::raw::c_char, elem, name,
                        defaultValue);
        defaultValue = 0 as *const xmlChar;
        if !ctxt.is_null() { (*ctxt).valid = 0 as std::os::raw::c_int }
    }
    /* LIBXML_VALID_ENABLED */
    /*
     * Check first that an attribute defined in the external subset wasn't
     * already defined in the internal subset
     */
    if !(*dtd).doc.is_null() && (*(*dtd).doc).extSubset == dtd &&
           !(*(*dtd).doc).intSubset.is_null() &&
           !(*(*(*dtd).doc).intSubset).attributes.is_null() {
        ret =
            xmlHashLookup3((*(*(*dtd).doc).intSubset).attributes as
                               xmlHashTablePtr, name, ns, elem) as
                xmlAttributePtr;
        if !ret.is_null() {
            xmlFreeEnumeration(tree);
            return 0 as xmlAttributePtr
        }
    }
    /*
     * Create the Attribute table if needed.
     */
    table = (*dtd).attributes as xmlAttributeTablePtr;
    if table.is_null() {
        table = xmlHashCreateDict(0 as std::os::raw::c_int, dict);
        (*dtd).attributes = table as *mut std::os::raw::c_void
    }
    if table.is_null() {
        xmlVErrMemory(ctxt,
                      b"xmlAddAttributeDecl: Table creation failed!\n\x00" as
                          *const u8 as *const std::os::raw::c_char);
        xmlFreeEnumeration(tree);
        return 0 as xmlAttributePtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlAttribute>()
                                                          as std::os::raw::c_ulong) as
            xmlAttributePtr;
    if ret.is_null() {
        xmlVErrMemory(ctxt,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        xmlFreeEnumeration(tree);
        return 0 as xmlAttributePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlAttribute>() as std::os::raw::c_ulong);
    (*ret).type_0 = XML_ATTRIBUTE_DECL;
    /*
     * fill the structure.
     */
    (*ret).atype = type_0;
    /*
     * doc must be set before possible error causes call
     * to xmlFreeAttribute (because it's used to check on
     * dict use)
     */
    (*ret).doc = (*dtd).doc;
    if !dict.is_null() {
        (*ret).name = xmlDictLookup(dict, name, -(1 as std::os::raw::c_int));
        (*ret).prefix = xmlDictLookup(dict, ns, -(1 as std::os::raw::c_int));
        (*ret).elem = xmlDictLookup(dict, elem, -(1 as std::os::raw::c_int))
    } else {
        (*ret).name = xmlStrdup(name);
        (*ret).prefix = xmlStrdup(ns);
        (*ret).elem = xmlStrdup(elem)
    }
    (*ret).def = def;
    (*ret).tree = tree;
    if !defaultValue.is_null() {
        if !dict.is_null() {
            (*ret).defaultValue =
                xmlDictLookup(dict, defaultValue, -(1 as std::os::raw::c_int))
        } else { (*ret).defaultValue = xmlStrdup(defaultValue) }
    }
    /*
     * Validity Check:
     * Search the DTD for previous declarations of the ATTLIST
     */
    if xmlHashAddEntry3(table, (*ret).name, (*ret).prefix, (*ret).elem,
                        ret as *mut std::os::raw::c_void) < 0 as std::os::raw::c_int {
        /*
	 * The attribute is already defined in this DTD.
	 */
        xmlErrValidWarning(ctxt, dtd as xmlNodePtr,
                           XML_DTD_ATTRIBUTE_REDEFINED,
                           b"Attribute %s of element %s: already defined\n\x00"
                               as *const u8 as *const std::os::raw::c_char, name,
                           elem, 0 as *const xmlChar);
        /* LIBXML_VALID_ENABLED */
        xmlFreeAttribute(ret);
        return 0 as xmlAttributePtr
    }
    /*
     * Validity Check:
     * Multiple ID per element
     */
    elemDef = xmlGetDtdElementDesc2(dtd, elem, 1 as std::os::raw::c_int);
    if !elemDef.is_null() {
        if type_0 as std::os::raw::c_uint ==
               XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlScanIDAttributeDecl(0 as xmlValidCtxtPtr, elemDef,
                                      1 as std::os::raw::c_int) != 0 as std::os::raw::c_int {
            xmlErrValidNode(ctxt, dtd as xmlNodePtr, XML_DTD_MULTIPLE_ID,
                            b"Element %s has too may ID attributes defined : %s\n\x00"
                                as *const u8 as *const std::os::raw::c_char, elem,
                            name, 0 as *const xmlChar);
            if !ctxt.is_null() { (*ctxt).valid = 0 as std::os::raw::c_int }
        }
        /* LIBXML_VALID_ENABLED */
        /*
	 * Insert namespace default def first they need to be
	 * processed first.
	 */
        if xmlStrEqual((*ret).name,
                       b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 ||
               !(*ret).prefix.is_null() &&
                   xmlStrEqual((*ret).prefix,
                               b"xmlns\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
            (*ret).nexth = (*elemDef).attributes;
            (*elemDef).attributes = ret
        } else {
            let mut tmp: xmlAttributePtr = (*elemDef).attributes;
            while !tmp.is_null() &&
                      (xmlStrEqual((*tmp).name,
                                   b"xmlns\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) !=
                           0 ||
                           !(*ret).prefix.is_null() &&
                               xmlStrEqual((*ret).prefix,
                                           b"xmlns\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar) != 0) {
                if (*tmp).nexth.is_null() { break ; }
                tmp = (*tmp).nexth
            }
            if !tmp.is_null() {
                (*ret).nexth = (*tmp).nexth;
                (*tmp).nexth = ret
            } else {
                (*ret).nexth = (*elemDef).attributes;
                (*elemDef).attributes = ret
            }
        }
    }
    /*
     * Link it to the DTD
     */
    (*ret).parent = dtd;
    if (*dtd).last.is_null() {
        (*dtd).last = ret as xmlNodePtr;
        (*dtd).children = (*dtd).last
    } else {
        (*(*dtd).last).next = ret as xmlNodePtr;
        (*ret).prev = (*dtd).last;
        (*dtd).last = ret as xmlNodePtr
    }
    return ret;
}
unsafe extern "C" fn xmlFreeAttributeTableEntry(mut attr: *mut std::os::raw::c_void,
                                                mut name: *const xmlChar) {
    xmlFreeAttribute(attr as xmlAttributePtr);
}
/* *
 * xmlFreeAttributeTable:
 * @table:  An attribute table
 *
 * Deallocate the memory used by an entities hash table.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeAttributeTable(mut table:
                                                   xmlAttributeTablePtr) {
    xmlHashFree(table,
                Some(xmlFreeAttributeTableEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
}
/* *
 * xmlCopyAttribute:
 * @attr:  An attribute
 *
 * Build a copy of an attribute.
 *
 * Returns the new xmlAttributePtr or NULL in case of error.
 */
unsafe extern "C" fn xmlCopyAttribute(mut payload: *mut std::os::raw::c_void,
                                      mut name: *const xmlChar)
 -> *mut std::os::raw::c_void {
    let mut attr: xmlAttributePtr = payload as xmlAttributePtr;
    let mut cur: xmlAttributePtr = 0 as *mut xmlAttribute;
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlAttribute>()
                                                          as std::os::raw::c_ulong) as
            xmlAttributePtr;
    if cur.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlAttribute>() as std::os::raw::c_ulong);
    (*cur).type_0 = XML_ATTRIBUTE_DECL;
    (*cur).atype = (*attr).atype;
    (*cur).def = (*attr).def;
    (*cur).tree = xmlCopyEnumeration((*attr).tree);
    if !(*attr).elem.is_null() { (*cur).elem = xmlStrdup((*attr).elem) }
    if !(*attr).name.is_null() { (*cur).name = xmlStrdup((*attr).name) }
    if !(*attr).prefix.is_null() { (*cur).prefix = xmlStrdup((*attr).prefix) }
    if !(*attr).defaultValue.is_null() {
        (*cur).defaultValue = xmlStrdup((*attr).defaultValue)
    }
    return cur as *mut std::os::raw::c_void;
}
/* *
 * xmlCopyAttributeTable:
 * @table:  An attribute table
 *
 * Build a copy of an attribute table.
 *
 * Returns the new xmlAttributeTablePtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyAttributeTable(mut table:
                                                   xmlAttributeTablePtr)
 -> xmlAttributeTablePtr {
    return xmlHashCopy(table,
                       Some(xmlCopyAttribute as
                                unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                     _: *const xmlChar)
                                    -> *mut std::os::raw::c_void)) as
               xmlAttributeTablePtr;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlDumpAttributeDecl:
 * @buf:  the XML buffer output
 * @attr:  An attribute declaration
 *
 * This will dump the content of the attribute declaration as an XML
 * DTD definition
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDumpAttributeDecl(mut buf: xmlBufferPtr,
                                              mut attr: xmlAttributePtr) {
    if buf.is_null() || attr.is_null() { return }
    xmlBufferWriteChar(buf,
                       b"<!ATTLIST \x00" as *const u8 as *const std::os::raw::c_char);
    xmlBufferWriteCHAR(buf, (*attr).elem);
    xmlBufferWriteChar(buf, b" \x00" as *const u8 as *const std::os::raw::c_char);
    if !(*attr).prefix.is_null() {
        xmlBufferWriteCHAR(buf, (*attr).prefix);
        xmlBufferWriteChar(buf, b":\x00" as *const u8 as *const std::os::raw::c_char);
    }
    xmlBufferWriteCHAR(buf, (*attr).name);
    match (*attr).atype as std::os::raw::c_uint {
        1 => {
            xmlBufferWriteChar(buf,
                               b" CDATA\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        2 => {
            xmlBufferWriteChar(buf,
                               b" ID\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        3 => {
            xmlBufferWriteChar(buf,
                               b" IDREF\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        4 => {
            xmlBufferWriteChar(buf,
                               b" IDREFS\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        5 => {
            xmlBufferWriteChar(buf,
                               b" ENTITY\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        6 => {
            xmlBufferWriteChar(buf,
                               b" ENTITIES\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        7 => {
            xmlBufferWriteChar(buf,
                               b" NMTOKEN\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        8 => {
            xmlBufferWriteChar(buf,
                               b" NMTOKENS\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        9 => {
            xmlBufferWriteChar(buf,
                               b" (\x00" as *const u8 as *const std::os::raw::c_char);
            xmlDumpEnumeration(buf, (*attr).tree);
        }
        10 => {
            xmlBufferWriteChar(buf,
                               b" NOTATION (\x00" as *const u8 as
                                   *const std::os::raw::c_char);
            xmlDumpEnumeration(buf, (*attr).tree);
        }
        _ => {
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ATTRIBUTE struct corrupted invalid type\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
        }
    }
    match (*attr).def as std::os::raw::c_uint {
        1 => { }
        2 => {
            xmlBufferWriteChar(buf,
                               b" #REQUIRED\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        3 => {
            xmlBufferWriteChar(buf,
                               b" #IMPLIED\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        4 => {
            xmlBufferWriteChar(buf,
                               b" #FIXED\x00" as *const u8 as
                                   *const std::os::raw::c_char);
        }
        _ => {
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"Internal: ATTRIBUTE struct corrupted invalid def\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
        }
    }
    if !(*attr).defaultValue.is_null() {
        xmlBufferWriteChar(buf, b" \x00" as *const u8 as *const std::os::raw::c_char);
        xmlBufferWriteQuotedString(buf, (*attr).defaultValue);
    }
    xmlBufferWriteChar(buf, b">\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlDumpAttributeDeclScan:
 * @attr:  An attribute declaration
 * @buf:  the XML buffer output
 *
 * This is used with the hash scan function - just reverses arguments
 */
unsafe extern "C" fn xmlDumpAttributeDeclScan(mut attr: *mut std::os::raw::c_void,
                                              mut buf: *mut std::os::raw::c_void,
                                              mut name: *const xmlChar) {
    xmlDumpAttributeDecl(buf as xmlBufferPtr, attr as xmlAttributePtr);
}
/* *
 * xmlDumpAttributeTable:
 * @buf:  the XML buffer output
 * @table:  An attribute table
 *
 * This will dump the content of the attribute table as an XML DTD definition
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDumpAttributeTable(mut buf: xmlBufferPtr,
                                               mut table:
                                                   xmlAttributeTablePtr) {
    if buf.is_null() || table.is_null() { return }
    xmlHashScan(table,
                Some(xmlDumpAttributeDeclScan as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()),
                buf as *mut std::os::raw::c_void);
}
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *				NOTATIONs				*
 *									*
 ************************************************************************/
/* *
 * xmlFreeNotation:
 * @not:  A notation
 *
 * Deallocate the memory used by an notation definition
 */
unsafe extern "C" fn xmlFreeNotation(mut nota: xmlNotationPtr) {
    if nota.is_null() { return }
    if !(*nota).name.is_null() {
        xmlFree.expect("non-null function pointer")((*nota).name as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*nota).PublicID.is_null() {
        xmlFree.expect("non-null function pointer")((*nota).PublicID as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*nota).SystemID.is_null() {
        xmlFree.expect("non-null function pointer")((*nota).SystemID as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(nota as *mut std::os::raw::c_void);
}
/* *
 * xmlAddNotationDecl:
 * @dtd:  pointer to the DTD
 * @ctxt:  the validation context
 * @name:  the entity name
 * @PublicID:  the public identifier or NULL
 * @SystemID:  the system identifier or NULL
 *
 * Register a new notation declaration
 *
 * Returns NULL if not, otherwise the entity
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddNotationDecl(mut ctxt: xmlValidCtxtPtr,
                                            mut dtd: xmlDtdPtr,
                                            mut name: *const xmlChar,
                                            mut PublicID: *const xmlChar,
                                            mut SystemID: *const xmlChar)
 -> xmlNotationPtr {
    let mut ret: xmlNotationPtr = 0 as *mut xmlNotation;
    let mut table: xmlNotationTablePtr = 0 as *mut xmlNotationTable;
    if dtd.is_null() { return 0 as xmlNotationPtr }
    if name.is_null() { return 0 as xmlNotationPtr }
    if PublicID.is_null() && SystemID.is_null() { return 0 as xmlNotationPtr }
    /*
     * Create the Notation table if needed.
     */
    table = (*dtd).notations as xmlNotationTablePtr;
    if table.is_null() {
        let mut dict: xmlDictPtr = 0 as xmlDictPtr;
        if !(*dtd).doc.is_null() { dict = (*(*dtd).doc).dict }
        table = xmlHashCreateDict(0 as std::os::raw::c_int, dict);
        (*dtd).notations = table as *mut std::os::raw::c_void
    }
    if table.is_null() {
        xmlVErrMemory(ctxt,
                      b"xmlAddNotationDecl: Table creation failed!\n\x00" as
                          *const u8 as *const std::os::raw::c_char);
        return 0 as xmlNotationPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNotation>()
                                                          as std::os::raw::c_ulong) as
            xmlNotationPtr;
    if ret.is_null() {
        xmlVErrMemory(ctxt,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlNotationPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNotation>() as std::os::raw::c_ulong);
    /*
     * fill the structure.
     */
    (*ret).name = xmlStrdup(name);
    if !SystemID.is_null() { (*ret).SystemID = xmlStrdup(SystemID) }
    if !PublicID.is_null() { (*ret).PublicID = xmlStrdup(PublicID) }
    /*
     * Validity Check:
     * Check the DTD for previous declarations of the ATTLIST
     */
    if xmlHashAddEntry(table, name, ret as *mut std::os::raw::c_void) != 0 {
        xmlErrValid(0 as xmlValidCtxtPtr, XML_DTD_NOTATION_REDEFINED,
                    b"xmlAddNotationDecl: %s already defined\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    name as *const std::os::raw::c_char);
        /* LIBXML_VALID_ENABLED */
        xmlFreeNotation(ret);
        return 0 as xmlNotationPtr
    }
    return ret;
}
unsafe extern "C" fn xmlFreeNotationTableEntry(mut nota: *mut std::os::raw::c_void,
                                               mut name: *const xmlChar) {
    xmlFreeNotation(nota as xmlNotationPtr);
}
/* *
 * xmlFreeNotationTable:
 * @table:  An notation table
 *
 * Deallocate the memory used by an entities hash table.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeNotationTable(mut table:
                                                  xmlNotationTablePtr) {
    xmlHashFree(table,
                Some(xmlFreeNotationTableEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
}
/* *
 * xmlCopyNotation:
 * @nota:  A notation
 *
 * Build a copy of a notation.
 *
 * Returns the new xmlNotationPtr or NULL in case of error.
 */
unsafe extern "C" fn xmlCopyNotation(mut payload: *mut std::os::raw::c_void,
                                     mut name: *const xmlChar)
 -> *mut std::os::raw::c_void {
    let mut nota: xmlNotationPtr = payload as xmlNotationPtr;
    let mut cur: xmlNotationPtr = 0 as *mut xmlNotation;
    cur =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNotation>()
                                                          as std::os::raw::c_ulong) as
            xmlNotationPtr;
    if cur.is_null() {
        xmlVErrMemory(0 as xmlValidCtxtPtr,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    if !(*nota).name.is_null() {
        (*cur).name = xmlStrdup((*nota).name)
    } else { (*cur).name = 0 as *const xmlChar }
    if !(*nota).PublicID.is_null() {
        (*cur).PublicID = xmlStrdup((*nota).PublicID)
    } else { (*cur).PublicID = 0 as *const xmlChar }
    if !(*nota).SystemID.is_null() {
        (*cur).SystemID = xmlStrdup((*nota).SystemID)
    } else { (*cur).SystemID = 0 as *const xmlChar }
    return cur as *mut std::os::raw::c_void;
}
/* *
 * xmlCopyNotationTable:
 * @table:  A notation table
 *
 * Build a copy of a notation table.
 *
 * Returns the new xmlNotationTablePtr or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlCopyNotationTable(mut table: xmlNotationTablePtr)
 -> xmlNotationTablePtr {
    return xmlHashCopy(table,
                       Some(xmlCopyNotation as
                                unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                     _: *const xmlChar)
                                    -> *mut std::os::raw::c_void)) as
               xmlNotationTablePtr;
}
/* LIBXML_TREE_ENABLED */
/* *
 * xmlDumpNotationDecl:
 * @buf:  the XML buffer output
 * @nota:  A notation declaration
 *
 * This will dump the content the notation declaration as an XML DTD definition
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDumpNotationDecl(mut buf: xmlBufferPtr,
                                             mut nota: xmlNotationPtr) {
    if buf.is_null() || nota.is_null() { return }
    xmlBufferWriteChar(buf,
                       b"<!NOTATION \x00" as *const u8 as
                           *const std::os::raw::c_char);
    xmlBufferWriteCHAR(buf, (*nota).name);
    if !(*nota).PublicID.is_null() {
        xmlBufferWriteChar(buf,
                           b" PUBLIC \x00" as *const u8 as
                               *const std::os::raw::c_char);
        xmlBufferWriteQuotedString(buf, (*nota).PublicID);
        if !(*nota).SystemID.is_null() {
            xmlBufferWriteChar(buf,
                               b" \x00" as *const u8 as *const std::os::raw::c_char);
            xmlBufferWriteQuotedString(buf, (*nota).SystemID);
        }
    } else {
        xmlBufferWriteChar(buf,
                           b" SYSTEM \x00" as *const u8 as
                               *const std::os::raw::c_char);
        xmlBufferWriteQuotedString(buf, (*nota).SystemID);
    }
    xmlBufferWriteChar(buf, b" >\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlDumpNotationDeclScan:
 * @nota:  A notation declaration
 * @buf:  the XML buffer output
 *
 * This is called with the hash scan function, and just reverses args
 */
unsafe extern "C" fn xmlDumpNotationDeclScan(mut nota: *mut std::os::raw::c_void,
                                             mut buf: *mut std::os::raw::c_void,
                                             mut name: *const xmlChar) {
    xmlDumpNotationDecl(buf as xmlBufferPtr, nota as xmlNotationPtr);
}
/* *
 * xmlDumpNotationTable:
 * @buf:  the XML buffer output
 * @table:  A notation table
 *
 * This will dump the content of the notation table as an XML DTD definition
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDumpNotationTable(mut buf: xmlBufferPtr,
                                              mut table:
                                                  xmlNotationTablePtr) {
    if buf.is_null() || table.is_null() { return }
    xmlHashScan(table,
                Some(xmlDumpNotationDeclScan as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()),
                buf as *mut std::os::raw::c_void);
}
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *				IDs					*
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
    if !(*id).name.is_null() {
        if !(*id).name.is_null() &&
               (dict.is_null() ||
                    xmlDictOwns(dict, (*id).name) == 0 as std::os::raw::c_int) {
            xmlFree.expect("non-null function pointer")((*id).name as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
    }
    xmlFree.expect("non-null function pointer")(id as *mut std::os::raw::c_void);
}
/* *
 * xmlAddID:
 * @ctxt:  the validation context
 * @doc:  pointer to the document
 * @value:  the value name
 * @attr:  the attribute holding the ID
 *
 * Register a new id declaration
 *
 * Returns NULL if not, otherwise the new xmlIDPtr
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddID(mut ctxt: xmlValidCtxtPtr,
                                  mut doc: xmlDocPtr,
                                  mut value: *const xmlChar,
                                  mut attr: xmlAttrPtr) -> xmlIDPtr {
    let mut ret: xmlIDPtr = 0 as *mut xmlID;
    let mut table: xmlIDTablePtr = 0 as *mut xmlIDTable;
    if doc.is_null() { return 0 as xmlIDPtr }
    if value.is_null() { return 0 as xmlIDPtr }
    if attr.is_null() { return 0 as xmlIDPtr }
    /*
     * Create the ID table if needed.
     */
    table = (*doc).ids as xmlIDTablePtr;
    if table.is_null() {
        table = xmlHashCreateDict(0 as std::os::raw::c_int, (*doc).dict);
        (*doc).ids = table as *mut std::os::raw::c_void
    }
    if table.is_null() {
        xmlVErrMemory(ctxt,
                      b"xmlAddID: Table creation failed!\n\x00" as *const u8
                          as *const std::os::raw::c_char);
        return 0 as xmlIDPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlID>()
                                                          as std::os::raw::c_ulong) as
            xmlIDPtr;
    if ret.is_null() {
        xmlVErrMemory(ctxt,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlIDPtr
    }
    /*
     * fill the structure.
     */
    (*ret).value = xmlStrdup(value);
    (*ret).doc = doc;
    if !ctxt.is_null() && (*ctxt).vstateNr != 0 as std::os::raw::c_int {
        /*
	 * Operating in streaming mode, attr is gonna disapear
	 */
        if !(*doc).dict.is_null() {
            (*ret).name =
                xmlDictLookup((*doc).dict, (*attr).name, -(1 as std::os::raw::c_int))
        } else { (*ret).name = xmlStrdup((*attr).name) }
        (*ret).attr = 0 as xmlAttrPtr
    } else { (*ret).attr = attr; (*ret).name = 0 as *const xmlChar }
    (*ret).lineno = xmlGetLineNo((*attr).parent) as std::os::raw::c_int;
    if xmlHashAddEntry(table, value, ret as *mut std::os::raw::c_void) <
           0 as std::os::raw::c_int {
        /*
	 * The id is already defined in this DTD.
	 */
        if !ctxt.is_null() {
            xmlErrValidNode(ctxt, (*attr).parent, XML_DTD_ID_REDEFINED,
                            b"ID %s already defined\n\x00" as *const u8 as
                                *const std::os::raw::c_char, value,
                            0 as *const xmlChar, 0 as *const xmlChar);
        }
        /* LIBXML_VALID_ENABLED */
        xmlFreeID(ret);
        return 0 as xmlIDPtr
    }
    if !attr.is_null() { (*attr).atype = XML_ATTRIBUTE_ID }
    return ret;
}
unsafe extern "C" fn xmlFreeIDTableEntry(mut id: *mut std::os::raw::c_void,
                                         mut name: *const xmlChar) {
    xmlFreeID(id as xmlIDPtr);
}
/* *
 * xmlFreeIDTable:
 * @table:  An id table
 *
 * Deallocate the memory used by an ID hash table.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeIDTable(mut table: xmlIDTablePtr) {
    xmlHashFree(table,
                Some(xmlFreeIDTableEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
}
/* *
 * xmlIsID:
 * @doc:  the document
 * @elem:  the element carrying the attribute
 * @attr:  the attribute
 *
 * Determine whether an attribute is of type ID. In case we have DTD(s)
 * then this is done if DTD loading has been requested. In the case
 * of HTML documents parsed with the HTML parser, then ID detection is
 * done systematically.
 *
 * Returns 0 or 1 depending on the lookup result
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIsID(mut doc: xmlDocPtr, mut elem: xmlNodePtr,
                                 mut attr: xmlAttrPtr) -> std::os::raw::c_int {
    if attr.is_null() || (*attr).name.is_null() { return 0 as std::os::raw::c_int }
    if !(*attr).ns.is_null() && !(*(*attr).ns).prefix.is_null() &&
           strcmp((*attr).name as *mut std::os::raw::c_char,
                  b"id\x00" as *const u8 as *const std::os::raw::c_char) == 0 &&
           strcmp((*(*attr).ns).prefix as *mut std::os::raw::c_char,
                  b"xml\x00" as *const u8 as *const std::os::raw::c_char) == 0 {
        return 1 as std::os::raw::c_int
    }
    if doc.is_null() { return 0 as std::os::raw::c_int }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() &&
           (*doc).type_0 as std::os::raw::c_uint !=
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            if xmlStrEqual(b"id\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar, (*attr).name) != 0 ||
                   xmlStrEqual(b"name\x00" as *const u8 as *const std::os::raw::c_char
                                   as *mut xmlChar, (*attr).name) != 0 &&
                       (elem.is_null() ||
                            xmlStrEqual((*elem).name,
                                        b"a\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar) != 0) {
                return 1 as std::os::raw::c_int
            }
            return 0 as std::os::raw::c_int
        } else {
            if elem.is_null() {
                return 0 as std::os::raw::c_int
            } else {
                let mut attrDecl: xmlAttributePtr = 0 as xmlAttributePtr;
                let mut felem: [xmlChar; 50] = [0; 50];
                let mut fattr: [xmlChar; 50] = [0; 50];
                let mut fullelemname: *mut xmlChar = 0 as *mut xmlChar;
                let mut fullattrname: *mut xmlChar = 0 as *mut xmlChar;
                fullelemname =
                    if !(*elem).ns.is_null() &&
                           !(*(*elem).ns).prefix.is_null() {
                        xmlBuildQName((*elem).name, (*(*elem).ns).prefix,
                                      felem.as_mut_ptr(), 50 as std::os::raw::c_int)
                    } else { (*elem).name as *mut xmlChar };
                fullattrname =
                    if !(*attr).ns.is_null() &&
                           !(*(*attr).ns).prefix.is_null() {
                        xmlBuildQName((*attr).name, (*(*attr).ns).prefix,
                                      fattr.as_mut_ptr(), 50 as std::os::raw::c_int)
                    } else { (*attr).name as *mut xmlChar };
                if !fullelemname.is_null() && !fullattrname.is_null() {
                    attrDecl =
                        xmlGetDtdAttrDesc((*doc).intSubset, fullelemname,
                                          fullattrname);
                    if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                        attrDecl =
                            xmlGetDtdAttrDesc((*doc).extSubset, fullelemname,
                                              fullattrname)
                    }
                }
                if fullattrname != fattr.as_mut_ptr() &&
                       fullattrname != (*attr).name as *mut xmlChar {
                    xmlFree.expect("non-null function pointer")(fullattrname
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
                if fullelemname != felem.as_mut_ptr() &&
                       fullelemname != (*elem).name as *mut xmlChar {
                    xmlFree.expect("non-null function pointer")(fullelemname
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
                if !attrDecl.is_null() &&
                       (*attrDecl).atype as std::os::raw::c_uint ==
                           XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
                    return 1 as std::os::raw::c_int
                }
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRemoveID:
 * @doc:  the document
 * @attr:  the attribute
 *
 * Remove the given attribute from the ID table maintained internally.
 *
 * Returns -1 if the lookup failed and 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRemoveID(mut doc: xmlDocPtr, mut attr: xmlAttrPtr)
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
    if id.is_null() || (*id).attr != attr {
        xmlFree.expect("non-null function pointer")(ID as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    xmlHashRemoveEntry(table, ID,
                       Some(xmlFreeIDTableEntry as
                                unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                     _: *const xmlChar)
                                    -> ()));
    xmlFree.expect("non-null function pointer")(ID as *mut std::os::raw::c_void);
    (*attr).atype = 0 as xmlAttributeType;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlGetID:
 * @doc:  pointer to the document
 * @ID:  the ID value
 *
 * Search the attribute declaring the given ID
 *
 * Returns NULL if not found, otherwise the xmlAttrPtr defining the ID
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetID(mut doc: xmlDocPtr, mut ID: *const xmlChar)
 -> xmlAttrPtr {
    let mut table: xmlIDTablePtr = 0 as *mut xmlIDTable;
    let mut id: xmlIDPtr = 0 as *mut xmlID;
    if doc.is_null() { return 0 as xmlAttrPtr }
    if ID.is_null() { return 0 as xmlAttrPtr }
    table = (*doc).ids as xmlIDTablePtr;
    if table.is_null() { return 0 as xmlAttrPtr }
    id = xmlHashLookup(table, ID) as xmlIDPtr;
    if id.is_null() { return 0 as xmlAttrPtr }
    if (*id).attr.is_null() {
        /*
	 * We are operating on a stream, return a well known reference
	 * since the attribute node doesn't exist anymore
	 */
        return doc as xmlAttrPtr
    }
    return (*id).attr;
}
/* *
 * xmlFreeRef:
 * @lk:  A list link
 *
 * Deallocate the memory used by a ref definition
 */
unsafe extern "C" fn xmlFreeRef(mut lk: xmlLinkPtr) {
    let mut ref_0: xmlRefPtr = xmlLinkGetData(lk) as xmlRefPtr;
    if ref_0.is_null() { return }
    if !(*ref_0).value.is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).value as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ref_0).name.is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).name as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(ref_0 as *mut std::os::raw::c_void);
}
/* *
 * xmlFreeRefTableEntry:
 * @list_ref:  A list of references.
 *
 * Deallocate the memory used by a list of references
 */
unsafe extern "C" fn xmlFreeRefTableEntry(mut payload: *mut std::os::raw::c_void,
                                          mut name: *const xmlChar) {
    let mut list_ref: xmlListPtr = payload as xmlListPtr;
    if list_ref.is_null() { return }
    xmlListDelete(list_ref);
}
/* *
 * xmlWalkRemoveRef:
 * @data:  Contents of current link
 * @user:  Value supplied by the user
 *
 * Returns 0 to abort the walk or 1 to continue
 */
unsafe extern "C" fn xmlWalkRemoveRef(mut data: *const std::os::raw::c_void,
                                      mut user: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut attr0: xmlAttrPtr = (*(data as xmlRefPtr)).attr;
    let mut attr1: xmlAttrPtr = (*(user as xmlRemoveMemoPtr)).ap;
    let mut ref_list: xmlListPtr = (*(user as xmlRemoveMemoPtr)).l;
    if attr0 == attr1 {
        /* Matched: remove and terminate walk */
        xmlListRemoveFirst(ref_list, data as *mut std::os::raw::c_void);
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlDummyCompare
 * @data0:  Value supplied by the user
 * @data1:  Value supplied by the user
 *
 * Do nothing, return 0. Used to create unordered lists.
 */
unsafe extern "C" fn xmlDummyCompare(mut data0: *const std::os::raw::c_void,
                                     mut data1: *const std::os::raw::c_void)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlAddRef:
 * @ctxt:  the validation context
 * @doc:  pointer to the document
 * @value:  the value name
 * @attr:  the attribute holding the Ref
 *
 * Register a new ref declaration
 *
 * Returns NULL if not, otherwise the new xmlRefPtr
 */
#[no_mangle]
pub unsafe extern "C" fn xmlAddRef(mut ctxt: xmlValidCtxtPtr,
                                   mut doc: xmlDocPtr,
                                   mut value: *const xmlChar,
                                   mut attr: xmlAttrPtr) -> xmlRefPtr {
    let mut current_block: u64;
    let mut ret: xmlRefPtr = 0 as *mut xmlRef;
    let mut table: xmlRefTablePtr = 0 as *mut xmlRefTable;
    let mut ref_list: xmlListPtr = 0 as *mut xmlList;
    if doc.is_null() { return 0 as xmlRefPtr }
    if value.is_null() { return 0 as xmlRefPtr }
    if attr.is_null() { return 0 as xmlRefPtr }
    /*
     * Create the Ref table if needed.
     */
    table = (*doc).refs as xmlRefTablePtr;
    if table.is_null() {
        table = xmlHashCreateDict(0 as std::os::raw::c_int, (*doc).dict);
        (*doc).refs = table as *mut std::os::raw::c_void
    }
    if table.is_null() {
        xmlVErrMemory(ctxt,
                      b"xmlAddRef: Table creation failed!\n\x00" as *const u8
                          as *const std::os::raw::c_char);
        return 0 as xmlRefPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRef>()
                                                          as std::os::raw::c_ulong) as
            xmlRefPtr;
    if ret.is_null() {
        xmlVErrMemory(ctxt,
                      b"malloc failed\x00" as *const u8 as
                          *const std::os::raw::c_char);
        return 0 as xmlRefPtr
    }
    /*
     * fill the structure.
     */
    (*ret).value = xmlStrdup(value);
    if !ctxt.is_null() && (*ctxt).vstateNr != 0 as std::os::raw::c_int {
        /*
	 * Operating in streaming mode, attr is gonna disapear
	 */
        (*ret).name = xmlStrdup((*attr).name);
        (*ret).attr = 0 as xmlAttrPtr
    } else { (*ret).name = 0 as *const xmlChar; (*ret).attr = attr }
    (*ret).lineno = xmlGetLineNo((*attr).parent) as std::os::raw::c_int;
    /* To add a reference :-
     * References are maintained as a list of references,
     * Lookup the entry, if no entry create new nodelist
     * Add the owning node to the NodeList
     * Return the ref
     */
    ref_list = xmlHashLookup(table, value) as xmlListPtr;
    if ref_list.is_null() {
        ref_list =
            xmlListCreate(Some(xmlFreeRef as
                                   unsafe extern "C" fn(_: xmlLinkPtr) -> ()),
                          Some(xmlDummyCompare as
                                   unsafe extern "C" fn(_:
                                                            *const std::os::raw::c_void,
                                                        _:
                                                            *const std::os::raw::c_void)
                                       -> std::os::raw::c_int));
        if ref_list.is_null() {
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlAddRef: Reference list creation failed!\n\x00" as
                            *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
            current_block = 2938303588815257839;
        } else if xmlHashAddEntry(table, value, ref_list as *mut std::os::raw::c_void)
                      < 0 as std::os::raw::c_int {
            xmlListDelete(ref_list);
            xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                        b"xmlAddRef: Reference list insertion failed!\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char);
            current_block = 2938303588815257839;
        } else { current_block = 7333393191927787629; }
    } else { current_block = 7333393191927787629; }
    match current_block {
        7333393191927787629 => {
            if xmlListAppend(ref_list, ret as *mut std::os::raw::c_void) !=
                   0 as std::os::raw::c_int {
                xmlErrValid(0 as xmlValidCtxtPtr, XML_ERR_INTERNAL_ERROR,
                            b"xmlAddRef: Reference list insertion failed!\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
            } else { return ret }
        }
        _ => { }
    }
    if !ret.is_null() {
        if !(*ret).value.is_null() {
            xmlFree.expect("non-null function pointer")((*ret).value as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        if !(*ret).name.is_null() {
            xmlFree.expect("non-null function pointer")((*ret).name as
                                                            *mut std::os::raw::c_char
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
    }
    return 0 as xmlRefPtr;
}
/* *
 * xmlFreeRefTable:
 * @table:  An ref table
 *
 * Deallocate the memory used by an Ref hash table.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlFreeRefTable(mut table: xmlRefTablePtr) {
    xmlHashFree(table,
                Some(xmlFreeRefTableEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
}
/* *
 * xmlIsRef:
 * @doc:  the document
 * @elem:  the element carrying the attribute
 * @attr:  the attribute
 *
 * Determine whether an attribute is of type Ref. In case we have DTD(s)
 * then this is simple, otherwise we use an heuristic: name Ref (upper
 * or lowercase).
 *
 * Returns 0 or 1 depending on the lookup result
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIsRef(mut doc: xmlDocPtr, mut elem: xmlNodePtr,
                                  mut attr: xmlAttrPtr) -> std::os::raw::c_int {
    if attr.is_null() { return 0 as std::os::raw::c_int }
    if doc.is_null() {
        doc = (*attr).doc;
        if doc.is_null() { return 0 as std::os::raw::c_int }
    }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /* TODO @@@ */
            return 0 as std::os::raw::c_int
        } else {
            let mut attrDecl: xmlAttributePtr = 0 as *mut xmlAttribute;
            if elem.is_null() { return 0 as std::os::raw::c_int }
            attrDecl =
                xmlGetDtdAttrDesc((*doc).intSubset, (*elem).name,
                                  (*attr).name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset, (*elem).name,
                                      (*attr).name)
            }
            if !attrDecl.is_null() &&
                   ((*attrDecl).atype as std::os::raw::c_uint ==
                        XML_ATTRIBUTE_IDREF as std::os::raw::c_int as std::os::raw::c_uint ||
                        (*attrDecl).atype as std::os::raw::c_uint ==
                            XML_ATTRIBUTE_IDREFS as std::os::raw::c_int as
                                std::os::raw::c_uint) {
                return 1 as std::os::raw::c_int
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRemoveRef:
 * @doc:  the document
 * @attr:  the attribute
 *
 * Remove the given attribute from the Ref table maintained internally.
 *
 * Returns -1 if the lookup failed and 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRemoveRef(mut doc: xmlDocPtr,
                                      mut attr: xmlAttrPtr) -> std::os::raw::c_int {
    let mut ref_list: xmlListPtr = 0 as *mut xmlList;
    let mut table: xmlRefTablePtr = 0 as *mut xmlRefTable;
    let mut ID: *mut xmlChar = 0 as *mut xmlChar;
    let mut target: xmlRemoveMemo =
        xmlRemoveMemo{l: 0 as *mut xmlList, ap: 0 as *mut xmlAttr,};
    if doc.is_null() { return -(1 as std::os::raw::c_int) }
    if attr.is_null() { return -(1 as std::os::raw::c_int) }
    table = (*doc).refs as xmlRefTablePtr;
    if table.is_null() { return -(1 as std::os::raw::c_int) }
    ID = xmlNodeListGetString(doc, (*attr).children, 1 as std::os::raw::c_int);
    if ID.is_null() { return -(1 as std::os::raw::c_int) }
    ref_list = xmlHashLookup(table, ID) as xmlListPtr;
    if ref_list.is_null() {
        xmlFree.expect("non-null function pointer")(ID as *mut std::os::raw::c_void);
        return -(1 as std::os::raw::c_int)
    }
    /* At this point, ref_list refers to a list of references which
     * have the same key as the supplied attr. Our list of references
     * is ordered by reference address and we don't have that information
     * here to use when removing. We'll have to walk the list and
     * check for a matching attribute, when we find one stop the walk
     * and remove the entry.
     * The list is ordered by reference, so that means we don't have the
     * key. Passing the list and the reference to the walker means we
     * will have enough data to be able to remove the entry.
     */
    target.l = ref_list;
    target.ap = attr;
    /* Remove the supplied attr from our list */
    xmlListWalk(ref_list,
                Some(xmlWalkRemoveRef as
                         unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void)
                             -> std::os::raw::c_int),
                &mut target as *mut xmlRemoveMemo as *mut std::os::raw::c_void);
    /*If the list is empty then remove the list entry in the hash */
    if xmlListEmpty(ref_list) != 0 {
        xmlHashUpdateEntry(table, ID, 0 as *mut std::os::raw::c_void,
                           Some(xmlFreeRefTableEntry as
                                    unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                         _: *const xmlChar)
                                        -> ()));
    }
    xmlFree.expect("non-null function pointer")(ID as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlGetRefs:
 * @doc:  pointer to the document
 * @ID:  the ID value
 *
 * Find the set of references for the supplied ID.
 *
 * Returns NULL if not found, otherwise node set for the ID.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetRefs(mut doc: xmlDocPtr,
                                    mut ID: *const xmlChar) -> xmlListPtr {
    let mut table: xmlRefTablePtr = 0 as *mut xmlRefTable;
    if doc.is_null() { return 0 as xmlListPtr }
    if ID.is_null() { return 0 as xmlListPtr }
    table = (*doc).refs as xmlRefTablePtr;
    if table.is_null() { return 0 as xmlListPtr }
    return xmlHashLookup(table, ID) as xmlListPtr;
}
/* ***********************************************************************
 *									*
 *		Routines for validity checking				*
 *									*
 ************************************************************************/
/* *
 * xmlGetDtdElementDesc:
 * @dtd:  a pointer to the DtD to search
 * @name:  the element name
 *
 * Search the DTD for the description of this element
 *
 * returns the xmlElementPtr if found or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdElementDesc(mut dtd: xmlDtdPtr,
                                              mut name: *const xmlChar)
 -> xmlElementPtr {
    let mut table: xmlElementTablePtr = 0 as *mut xmlElementTable;
    let mut cur: xmlElementPtr = 0 as *mut xmlElement;
    let mut uqname: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    if dtd.is_null() || name.is_null() { return 0 as xmlElementPtr }
    if (*dtd).elements.is_null() { return 0 as xmlElementPtr }
    table = (*dtd).elements as xmlElementTablePtr;
    uqname = xmlSplitQName2(name, &mut prefix);
    if !uqname.is_null() { name = uqname }
    cur = xmlHashLookup2(table, name, prefix) as xmlElementPtr;
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as
                                                        *mut std::os::raw::c_void);
    }
    if !uqname.is_null() {
        xmlFree.expect("non-null function pointer")(uqname as
                                                        *mut std::os::raw::c_void);
    }
    return cur;
}
/*
 * valid.c : part of the code use to do the DTD handling and the validity
 *           checking
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
/* *
 * xmlGetDtdElementDesc2:
 * @dtd:  a pointer to the DtD to search
 * @name:  the element name
 * @create:  create an empty description if not found
 *
 * Search the DTD for the description of this element
 *
 * returns the xmlElementPtr if found or NULL
 */
unsafe extern "C" fn xmlGetDtdElementDesc2(mut dtd: xmlDtdPtr,
                                           mut name: *const xmlChar,
                                           mut create: std::os::raw::c_int)
 -> xmlElementPtr {
    let mut table: xmlElementTablePtr = 0 as *mut xmlElementTable;
    let mut cur: xmlElementPtr = 0 as *mut xmlElement;
    let mut uqname: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    if dtd.is_null() { return 0 as xmlElementPtr }
    if (*dtd).elements.is_null() {
        let mut dict: xmlDictPtr = 0 as xmlDictPtr;
        if !(*dtd).doc.is_null() { dict = (*(*dtd).doc).dict }
        if create == 0 { return 0 as xmlElementPtr }
        /*
	 * Create the Element table if needed.
	 */
        table = (*dtd).elements as xmlElementTablePtr;
        if table.is_null() {
            table = xmlHashCreateDict(0 as std::os::raw::c_int, dict);
            (*dtd).elements = table as *mut std::os::raw::c_void
        }
        if table.is_null() {
            xmlVErrMemory(0 as xmlValidCtxtPtr,
                          b"element table allocation failed\x00" as *const u8
                              as *const std::os::raw::c_char);
            return 0 as xmlElementPtr
        }
    }
    table = (*dtd).elements as xmlElementTablePtr;
    uqname = xmlSplitQName2(name, &mut prefix);
    if !uqname.is_null() { name = uqname }
    cur = xmlHashLookup2(table, name, prefix) as xmlElementPtr;
    if cur.is_null() && create != 0 {
        cur =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlElement>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlElementPtr;
        if cur.is_null() {
            xmlVErrMemory(0 as xmlValidCtxtPtr,
                          b"malloc failed\x00" as *const u8 as
                              *const std::os::raw::c_char);
            return 0 as xmlElementPtr
        }
        memset(cur as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlElement>() as std::os::raw::c_ulong);
        (*cur).type_0 = XML_ELEMENT_DECL;
        /*
	 * fill the structure.
	 */
        (*cur).name = xmlStrdup(name);
        (*cur).prefix = xmlStrdup(prefix);
        (*cur).etype = XML_ELEMENT_TYPE_UNDEFINED;
        xmlHashAddEntry2(table, name, prefix, cur as *mut std::os::raw::c_void);
    }
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as
                                                        *mut std::os::raw::c_void);
    }
    if !uqname.is_null() {
        xmlFree.expect("non-null function pointer")(uqname as
                                                        *mut std::os::raw::c_void);
    }
    return cur;
}
/* *
 * xmlGetDtdQElementDesc:
 * @dtd:  a pointer to the DtD to search
 * @name:  the element name
 * @prefix:  the element namespace prefix
 *
 * Search the DTD for the description of this element
 *
 * returns the xmlElementPtr if found or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdQElementDesc(mut dtd: xmlDtdPtr,
                                               mut name: *const xmlChar,
                                               mut prefix: *const xmlChar)
 -> xmlElementPtr {
    let mut table: xmlElementTablePtr = 0 as *mut xmlElementTable;
    if dtd.is_null() { return 0 as xmlElementPtr }
    if (*dtd).elements.is_null() { return 0 as xmlElementPtr }
    table = (*dtd).elements as xmlElementTablePtr;
    return xmlHashLookup2(table, name, prefix) as xmlElementPtr;
}
/* *
 * xmlGetDtdAttrDesc:
 * @dtd:  a pointer to the DtD to search
 * @elem:  the element name
 * @name:  the attribute name
 *
 * Search the DTD for the description of this attribute on
 * this element.
 *
 * returns the xmlAttributePtr if found or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdAttrDesc(mut dtd: xmlDtdPtr,
                                           mut elem: *const xmlChar,
                                           mut name: *const xmlChar)
 -> xmlAttributePtr {
    let mut table: xmlAttributeTablePtr = 0 as *mut xmlAttributeTable;
    let mut cur: xmlAttributePtr = 0 as *mut xmlAttribute;
    let mut uqname: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    if dtd.is_null() { return 0 as xmlAttributePtr }
    if (*dtd).attributes.is_null() { return 0 as xmlAttributePtr }
    table = (*dtd).attributes as xmlAttributeTablePtr;
    if table.is_null() { return 0 as xmlAttributePtr }
    uqname = xmlSplitQName2(name, &mut prefix);
    if !uqname.is_null() {
        cur = xmlHashLookup3(table, uqname, prefix, elem) as xmlAttributePtr;
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
        }
        if !uqname.is_null() {
            xmlFree.expect("non-null function pointer")(uqname as
                                                            *mut std::os::raw::c_void);
        }
    } else {
        cur =
            xmlHashLookup3(table, name, 0 as *const xmlChar, elem) as
                xmlAttributePtr
    }
    return cur;
}
/* *
 * xmlGetDtdQAttrDesc:
 * @dtd:  a pointer to the DtD to search
 * @elem:  the element name
 * @name:  the attribute name
 * @prefix:  the attribute namespace prefix
 *
 * Search the DTD for the description of this qualified attribute on
 * this element.
 *
 * returns the xmlAttributePtr if found or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdQAttrDesc(mut dtd: xmlDtdPtr,
                                            mut elem: *const xmlChar,
                                            mut name: *const xmlChar,
                                            mut prefix: *const xmlChar)
 -> xmlAttributePtr {
    let mut table: xmlAttributeTablePtr = 0 as *mut xmlAttributeTable;
    if dtd.is_null() { return 0 as xmlAttributePtr }
    if (*dtd).attributes.is_null() { return 0 as xmlAttributePtr }
    table = (*dtd).attributes as xmlAttributeTablePtr;
    return xmlHashLookup3(table, name, prefix, elem) as xmlAttributePtr;
}
/* *
 * xmlGetDtdNotationDesc:
 * @dtd:  a pointer to the DtD to search
 * @name:  the notation name
 *
 * Search the DTD for the description of this notation
 *
 * returns the xmlNotationPtr if found or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdNotationDesc(mut dtd: xmlDtdPtr,
                                               mut name: *const xmlChar)
 -> xmlNotationPtr {
    let mut table: xmlNotationTablePtr = 0 as *mut xmlNotationTable;
    if dtd.is_null() { return 0 as xmlNotationPtr }
    if (*dtd).notations.is_null() { return 0 as xmlNotationPtr }
    table = (*dtd).notations as xmlNotationTablePtr;
    return xmlHashLookup(table, name) as xmlNotationPtr;
}
/* *
 * xmlValidateNotationUse:
 * @ctxt:  the validation context
 * @doc:  the document
 * @notationName:  the notation name to check
 *
 * Validate that the given name match a notation declaration.
 * - [ VC: Notation Declared ]
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNotationUse(mut ctxt: xmlValidCtxtPtr,
                                                mut doc: xmlDocPtr,
                                                mut notationName:
                                                    *const xmlChar)
 -> std::os::raw::c_int {
    let mut notaDecl: xmlNotationPtr = 0 as *mut xmlNotation;
    if doc.is_null() || (*doc).intSubset.is_null() || notationName.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    notaDecl = xmlGetDtdNotationDesc((*doc).intSubset, notationName);
    if notaDecl.is_null() && !(*doc).extSubset.is_null() {
        notaDecl = xmlGetDtdNotationDesc((*doc).extSubset, notationName)
    }
    if notaDecl.is_null() && !ctxt.is_null() {
        xmlErrValidNode(ctxt, doc as xmlNodePtr, XML_DTD_UNKNOWN_NOTATION,
                        b"NOTATION %s is not declared\n\x00" as *const u8 as
                            *const std::os::raw::c_char, notationName,
                        0 as *const xmlChar, 0 as *const xmlChar);
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* LIBXML_VALID_ENABLED or LIBXML_SCHEMAS_ENABLED */
/* *
 * xmlIsMixedElement:
 * @doc:  the document
 * @name:  the element name
 *
 * Search in the DtDs whether an element accept Mixed content (or ANY)
 * basically if it is supposed to accept text childs
 *
 * returns 0 if no, 1 if yes, and -1 if no element description is available
 */
#[no_mangle]
pub unsafe extern "C" fn xmlIsMixedElement(mut doc: xmlDocPtr,
                                           mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut elemDecl: xmlElementPtr = 0 as *mut xmlElement;
    if doc.is_null() || (*doc).intSubset.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    elemDecl = xmlGetDtdElementDesc((*doc).intSubset, name);
    if elemDecl.is_null() && !(*doc).extSubset.is_null() {
        elemDecl = xmlGetDtdElementDesc((*doc).extSubset, name)
    }
    if elemDecl.is_null() { return -(1 as std::os::raw::c_int) }
    match (*elemDecl).etype as std::os::raw::c_uint {
        0 => { return -(1 as std::os::raw::c_int) }
        4 => { return 0 as std::os::raw::c_int }
        1 | 2 | 3 => {
            /*
	     * return 1 for EMPTY since we want VC error to pop up
	     * on <empty>     </empty> for example
	     */
            return 1 as std::os::raw::c_int
        }
        _ => { }
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn xmlIsDocNameStartChar(mut doc: xmlDocPtr,
                                           mut c: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if doc.is_null() ||
           (*doc).properties & XML_DOC_OLD10 as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        /*
	 * Use the new checks of production [4] [4a] amd [5] of the
	 * Update 5 of XML-1.0
	 */
        if c >= 'a' as i32 && c <= 'z' as i32 ||
               c >= 'A' as i32 && c <= 'Z' as i32 || c == '_' as i32 ||
               c == ':' as i32 ||
               c >= 0xc0 as std::os::raw::c_int && c <= 0xd6 as std::os::raw::c_int ||
               c >= 0xd8 as std::os::raw::c_int && c <= 0xf6 as std::os::raw::c_int ||
               c >= 0xf8 as std::os::raw::c_int && c <= 0x2ff as std::os::raw::c_int ||
               c >= 0x370 as std::os::raw::c_int && c <= 0x37d as std::os::raw::c_int ||
               c >= 0x37f as std::os::raw::c_int && c <= 0x1fff as std::os::raw::c_int ||
               c >= 0x200c as std::os::raw::c_int && c <= 0x200d as std::os::raw::c_int ||
               c >= 0x2070 as std::os::raw::c_int && c <= 0x218f as std::os::raw::c_int ||
               c >= 0x2c00 as std::os::raw::c_int && c <= 0x2fef as std::os::raw::c_int ||
               c >= 0x3001 as std::os::raw::c_int && c <= 0xd7ff as std::os::raw::c_int ||
               c >= 0xf900 as std::os::raw::c_int && c <= 0xfdcf as std::os::raw::c_int ||
               c >= 0xfdf0 as std::os::raw::c_int && c <= 0xfffd as std::os::raw::c_int ||
               c >= 0x10000 as std::os::raw::c_int && c <= 0xeffff as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
    } else if (if c < 0x100 as std::os::raw::c_int {
                   (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                        0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int
                        ||
                        0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int
                        ||
                        0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int
                        || 0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup)
               }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       0 as std::os::raw::c_int
                   } else {
                       (0x4e00 as std::os::raw::c_int <= c &&
                            c <= 0x9fa5 as std::os::raw::c_int ||
                            c == 0x3007 as std::os::raw::c_int ||
                            0x3021 as std::os::raw::c_int <= c &&
                                c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
                   }) != 0 || c == '_' as i32 || c == ':' as i32 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn xmlIsDocNameChar(mut doc: xmlDocPtr, mut c: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if doc.is_null() ||
           (*doc).properties & XML_DOC_OLD10 as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        /*
	 * Use the new checks of production [4] [4a] amd [5] of the
	 * Update 5 of XML-1.0
	 */
        if c >= 'a' as i32 && c <= 'z' as i32 ||
               c >= 'A' as i32 && c <= 'Z' as i32 ||
               c >= '0' as i32 && c <= '9' as i32 || c == '_' as i32 ||
               c == ':' as i32 || c == '-' as i32 || c == '.' as i32 ||
               c == 0xb7 as std::os::raw::c_int ||
               c >= 0xc0 as std::os::raw::c_int && c <= 0xd6 as std::os::raw::c_int ||
               c >= 0xd8 as std::os::raw::c_int && c <= 0xf6 as std::os::raw::c_int ||
               c >= 0xf8 as std::os::raw::c_int && c <= 0x2ff as std::os::raw::c_int ||
               c >= 0x300 as std::os::raw::c_int && c <= 0x36f as std::os::raw::c_int ||
               c >= 0x370 as std::os::raw::c_int && c <= 0x37d as std::os::raw::c_int ||
               c >= 0x37f as std::os::raw::c_int && c <= 0x1fff as std::os::raw::c_int ||
               c >= 0x200c as std::os::raw::c_int && c <= 0x200d as std::os::raw::c_int ||
               c >= 0x203f as std::os::raw::c_int && c <= 0x2040 as std::os::raw::c_int ||
               c >= 0x2070 as std::os::raw::c_int && c <= 0x218f as std::os::raw::c_int ||
               c >= 0x2c00 as std::os::raw::c_int && c <= 0x2fef as std::os::raw::c_int ||
               c >= 0x3001 as std::os::raw::c_int && c <= 0xd7ff as std::os::raw::c_int ||
               c >= 0xf900 as std::os::raw::c_int && c <= 0xfdcf as std::os::raw::c_int ||
               c >= 0xfdf0 as std::os::raw::c_int && c <= 0xfffd as std::os::raw::c_int ||
               c >= 0x10000 as std::os::raw::c_int && c <= 0xeffff as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
    } else if (if c < 0x100 as std::os::raw::c_int {
                   (0x41 as std::os::raw::c_int <= c && c <= 0x5a as std::os::raw::c_int ||
                        0x61 as std::os::raw::c_int <= c && c <= 0x7a as std::os::raw::c_int
                        ||
                        0xc0 as std::os::raw::c_int <= c && c <= 0xd6 as std::os::raw::c_int
                        ||
                        0xd8 as std::os::raw::c_int <= c && c <= 0xf6 as std::os::raw::c_int
                        || 0xf8 as std::os::raw::c_int <= c) as std::os::raw::c_int
               } else {
                   xmlCharInRange(c as std::os::raw::c_uint, &xmlIsBaseCharGroup)
               }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       0 as std::os::raw::c_int
                   } else {
                       (0x4e00 as std::os::raw::c_int <= c &&
                            c <= 0x9fa5 as std::os::raw::c_int ||
                            c == 0x3007 as std::os::raw::c_int ||
                            0x3021 as std::os::raw::c_int <= c &&
                                c <= 0x3029 as std::os::raw::c_int) as std::os::raw::c_int
                   }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       (0x30 as std::os::raw::c_int <= c && c <= 0x39 as std::os::raw::c_int)
                           as std::os::raw::c_int
                   } else {
                       xmlCharInRange(c as std::os::raw::c_uint, &xmlIsDigitGroup)
                   }) != 0 || c == '.' as i32 || c == '-' as i32 ||
                  c == '_' as i32 || c == ':' as i32 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       0 as std::os::raw::c_int
                   } else {
                       xmlCharInRange(c as std::os::raw::c_uint, &xmlIsCombiningGroup)
                   }) != 0 ||
                  (if c < 0x100 as std::os::raw::c_int {
                       (c == 0xb7 as std::os::raw::c_int) as std::os::raw::c_int
                   } else {
                       xmlCharInRange(c as std::os::raw::c_uint, &xmlIsExtenderGroup)
                   }) != 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlValidateNameValue:
 * @doc:  pointer to the document or NULL
 * @value:  an Name value
 *
 * Validate that the given value match Name production
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateNameValueInternal(mut doc: xmlDocPtr,
                                                  mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut val: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    if value.is_null() { return 0 as std::os::raw::c_int }
    cur = value;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    if xmlIsDocNameStartChar(doc, val) == 0 { return 0 as std::os::raw::c_int }
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize)
    }
    if val != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateNameValue:
 * @value:  an Name value
 *
 * Validate that the given value match Name production
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNameValue(mut value: *const xmlChar)
 -> std::os::raw::c_int {
    return xmlValidateNameValueInternal(0 as xmlDocPtr, value);
}
/* *
 * xmlValidateNamesValueInternal:
 * @doc:  pointer to the document or NULL
 * @value:  an Names value
 *
 * Validate that the given value match Names production
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateNamesValueInternal(mut doc: xmlDocPtr,
                                                   mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut val: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    if value.is_null() { return 0 as std::os::raw::c_int }
    cur = value;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    if xmlIsDocNameStartChar(doc, val) == 0 { return 0 as std::os::raw::c_int }
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize)
    }
    /* Should not test IS_BLANK(val) here -- see erratum E20*/
    while val == 0x20 as std::os::raw::c_int {
        while val == 0x20 as std::os::raw::c_int {
            val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
            cur = cur.offset(len as isize)
        }
        if xmlIsDocNameStartChar(doc, val) == 0 { return 0 as std::os::raw::c_int }
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize);
        while xmlIsDocNameChar(doc, val) != 0 {
            val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
            cur = cur.offset(len as isize)
        }
    }
    if val != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateNamesValue:
 * @value:  an Names value
 *
 * Validate that the given value match Names production
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNamesValue(mut value: *const xmlChar)
 -> std::os::raw::c_int {
    return xmlValidateNamesValueInternal(0 as xmlDocPtr, value);
}
/* *
 * xmlValidateNmtokenValueInternal:
 * @doc:  pointer to the document or NULL
 * @value:  an Nmtoken value
 *
 * Validate that the given value match Nmtoken production
 *
 * [ VC: Name Token ]
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateNmtokenValueInternal(mut doc: xmlDocPtr,
                                                     mut value:
                                                         *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut val: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    if value.is_null() { return 0 as std::os::raw::c_int }
    cur = value;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    if xmlIsDocNameChar(doc, val) == 0 { return 0 as std::os::raw::c_int }
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize)
    }
    if val != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateNmtokenValue:
 * @value:  an Nmtoken value
 *
 * Validate that the given value match Nmtoken production
 *
 * [ VC: Name Token ]
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNmtokenValue(mut value: *const xmlChar)
 -> std::os::raw::c_int {
    return xmlValidateNmtokenValueInternal(0 as xmlDocPtr, value);
}
/* *
 * xmlValidateNmtokensValueInternal:
 * @doc:  pointer to the document or NULL
 * @value:  an Nmtokens value
 *
 * Validate that the given value match Nmtokens production
 *
 * [ VC: Name Token ]
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateNmtokensValueInternal(mut doc: xmlDocPtr,
                                                      mut value:
                                                          *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut val: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    if value.is_null() { return 0 as std::os::raw::c_int }
    cur = value;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    cur = cur.offset(len as isize);
    while if val < 0x100 as std::os::raw::c_int {
              (val == 0x20 as std::os::raw::c_int ||
                   0x9 as std::os::raw::c_int <= val && val <= 0xa as std::os::raw::c_int ||
                   val == 0xd as std::os::raw::c_int) as std::os::raw::c_int
          } else { 0 as std::os::raw::c_int } != 0 {
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize)
    }
    if xmlIsDocNameChar(doc, val) == 0 { return 0 as std::os::raw::c_int }
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize)
    }
    /* Should not test IS_BLANK(val) here -- see erratum E20*/
    while val == 0x20 as std::os::raw::c_int {
        while val == 0x20 as std::os::raw::c_int {
            val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
            cur = cur.offset(len as isize)
        }
        if val == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
        if xmlIsDocNameChar(doc, val) == 0 { return 0 as std::os::raw::c_int }
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
        cur = cur.offset(len as isize);
        while xmlIsDocNameChar(doc, val) != 0 {
            val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
            cur = cur.offset(len as isize)
        }
    }
    if val != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateNmtokensValue:
 * @value:  an Nmtokens value
 *
 * Validate that the given value match Nmtokens production
 *
 * [ VC: Name Token ]
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNmtokensValue(mut value: *const xmlChar)
 -> std::os::raw::c_int {
    return xmlValidateNmtokensValueInternal(0 as xmlDocPtr, value);
}
/* *
 * xmlValidateNotationDecl:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @nota:  a notation definition
 *
 * Try to validate a single notation definition
 * basically it does the following checks as described by the
 * XML-1.0 recommendation:
 *  - it seems that no validity constraint exists on notation declarations
 * But this function get called anyway ...
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNotationDecl(mut ctxt: xmlValidCtxtPtr,
                                                 mut doc: xmlDocPtr,
                                                 mut nota: xmlNotationPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    return ret;
}
/* #define DEBUG_VALID_ALGO */
/* #define DEBUG_REGEXP_ALGO */
/* *
 * xmlValidateAttributeValueInternal:
 * @doc: the document
 * @type:  an attribute type
 * @value:  an attribute value
 *
 * Validate that the given attribute value match  the proper production
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateAttributeValueInternal(mut doc: xmlDocPtr,
                                                       mut type_0:
                                                           xmlAttributeType,
                                                       mut value:
                                                           *const xmlChar)
 -> std::os::raw::c_int {
    match type_0 as std::os::raw::c_uint {
        6 | 4 => { return xmlValidateNamesValueInternal(doc, value) }
        5 | 3 | 2 | 10 => { return xmlValidateNameValueInternal(doc, value) }
        8 | 9 => { return xmlValidateNmtokensValueInternal(doc, value) }
        7 => { return xmlValidateNmtokenValueInternal(doc, value) }
        1 | _ => { }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateAttributeValue:
 * @type:  an attribute type
 * @value:  an attribute value
 *
 * Validate that the given attribute value match  the proper production
 *
 * [ VC: ID ]
 * Values of type ID must match the Name production....
 *
 * [ VC: IDREF ]
 * Values of type IDREF must match the Name production, and values
 * of type IDREFS must match Names ...
 *
 * [ VC: Entity Name ]
 * Values of type ENTITY must match the Name production, values
 * of type ENTITIES must match Names ...
 *
 * [ VC: Name Token ]
 * Values of type NMTOKEN must match the Nmtoken production; values
 * of type NMTOKENS must match Nmtokens.
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateAttributeValue(mut type_0:
                                                       xmlAttributeType,
                                                   mut value: *const xmlChar)
 -> std::os::raw::c_int {
    return xmlValidateAttributeValueInternal(0 as xmlDocPtr, type_0, value);
}
/* *
 * xmlValidateAttributeValue2:
 * @ctxt:  the validation context
 * @doc:  the document
 * @name:  the attribute name (used for error reporting only)
 * @type:  the attribute type
 * @value:  the attribute value
 *
 * Validate that the given attribute value match a given type.
 * This typically cannot be done before having finished parsing
 * the subsets.
 *
 * [ VC: IDREF ]
 * Values of type IDREF must match one of the declared IDs
 * Values of type IDREFS must match a sequence of the declared IDs
 * each Name must match the value of an ID attribute on some element
 * in the XML document; i.e. IDREF values must match the value of
 * some ID attribute
 *
 * [ VC: Entity Name ]
 * Values of type ENTITY must match one declared entity
 * Values of type ENTITIES must match a sequence of declared entities
 *
 * [ VC: Notation Attributes ]
 * all notation names in the declaration must be declared.
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateAttributeValue2(mut ctxt: xmlValidCtxtPtr,
                                                mut doc: xmlDocPtr,
                                                mut name: *const xmlChar,
                                                mut type_0: xmlAttributeType,
                                                mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    match type_0 as std::os::raw::c_uint {
        5 => {
            let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
            ent = xmlGetDocEntity(doc as *const xmlDoc, value);
            /* yeah it's a bit messy... */
            if ent.is_null() && (*doc).standalone == 1 as std::os::raw::c_int {
                (*doc).standalone = 0 as std::os::raw::c_int;
                ent = xmlGetDocEntity(doc as *const xmlDoc, value)
            }
            if ent.is_null() {
                xmlErrValidNode(ctxt, doc as xmlNodePtr,
                                XML_DTD_UNKNOWN_ENTITY,
                                b"ENTITY attribute %s reference an unknown entity \"%s\"\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, name,
                                value, 0 as *const xmlChar);
                ret = 0 as std::os::raw::c_int
            } else if (*ent).etype as std::os::raw::c_uint !=
                          XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as std::os::raw::c_int
                              as std::os::raw::c_uint {
                xmlErrValidNode(ctxt, doc as xmlNodePtr, XML_DTD_ENTITY_TYPE,
                                b"ENTITY attribute %s reference an entity \"%s\" of wrong type\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, name,
                                value, 0 as *const xmlChar);
                ret = 0 as std::os::raw::c_int
            }
        }
        6 => {
            let mut dup: *mut xmlChar = 0 as *mut xmlChar;
            let mut nam: *mut xmlChar = 0 as *mut xmlChar;
            let mut cur: *mut xmlChar = 0 as *mut xmlChar;
            let mut save: xmlChar = 0;
            let mut ent_0: xmlEntityPtr = 0 as *mut xmlEntity;
            dup = xmlStrdup(value);
            if dup.is_null() { return 0 as std::os::raw::c_int }
            cur = dup;
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                nam = cur;
                while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                          !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                    *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int
                                || *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int)
                      {
                    cur = cur.offset(1)
                }
                save = *cur;
                *cur = 0 as std::os::raw::c_int as xmlChar;
                ent_0 = xmlGetDocEntity(doc as *const xmlDoc, nam);
                if ent_0.is_null() {
                    xmlErrValidNode(ctxt, doc as xmlNodePtr,
                                    XML_DTD_UNKNOWN_ENTITY,
                                    b"ENTITIES attribute %s reference an unknown entity \"%s\"\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    name, nam, 0 as *const xmlChar);
                    ret = 0 as std::os::raw::c_int
                } else if (*ent_0).etype as std::os::raw::c_uint !=
                              XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as
                                  std::os::raw::c_int as std::os::raw::c_uint {
                    xmlErrValidNode(ctxt, doc as xmlNodePtr,
                                    XML_DTD_ENTITY_TYPE,
                                    b"ENTITIES attribute %s reference an entity \"%s\" of wrong type\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    name, nam, 0 as *const xmlChar);
                    ret = 0 as std::os::raw::c_int
                }
                if save as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
                *cur = save;
                while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                              *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                          *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    cur = cur.offset(1)
                }
            }
            xmlFree.expect("non-null function pointer")(dup as
                                                            *mut std::os::raw::c_void);
        }
        10 => {
            let mut nota: xmlNotationPtr = 0 as *mut xmlNotation;
            nota = xmlGetDtdNotationDesc((*doc).intSubset, value);
            if nota.is_null() && !(*doc).extSubset.is_null() {
                nota = xmlGetDtdNotationDesc((*doc).extSubset, value)
            }
            if nota.is_null() {
                xmlErrValidNode(ctxt, doc as xmlNodePtr,
                                XML_DTD_UNKNOWN_NOTATION,
                                b"NOTATION attribute %s reference an unknown notation \"%s\"\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, name,
                                value, 0 as *const xmlChar);
                ret = 0 as std::os::raw::c_int
            }
        }
        4 | 3 | 2 | 8 | 9 | 7 | 1 | _ => { }
    }
    return ret;
}
/* *
 * xmlValidCtxtNormalizeAttributeValue:
 * @ctxt: the validation context
 * @doc:  the document
 * @elem:  the parent
 * @name:  the attribute name
 * @value:  the attribute value
 * @ctxt:  the validation context or NULL
 *
 * Does the validation related extra step of the normalization of attribute
 * values:
 *
 * If the declared value is not CDATA, then the XML processor must further
 * process the normalized attribute value by discarding any leading and
 * trailing space (#x20) characters, and by replacing sequences of space
 * (#x20) characters by single space (#x20) character.
 *
 * Also  check VC: Standalone Document Declaration in P32, and update
 *  ctxt->valid accordingly
 *
 * returns a new normalized string if normalization is needed, NULL otherwise
 *      the caller must free the returned value.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidCtxtNormalizeAttributeValue(mut ctxt:
                                                                 xmlValidCtxtPtr,
                                                             mut doc:
                                                                 xmlDocPtr,
                                                             mut elem:
                                                                 xmlNodePtr,
                                                             mut name:
                                                                 *const xmlChar,
                                                             mut value:
                                                                 *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut dst: *mut xmlChar = 0 as *mut xmlChar;
    let mut src: *const xmlChar = 0 as *const xmlChar;
    let mut attrDecl: xmlAttributePtr = 0 as xmlAttributePtr;
    let mut extsubset: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if doc.is_null() { return 0 as *mut xmlChar }
    if elem.is_null() { return 0 as *mut xmlChar }
    if name.is_null() { return 0 as *mut xmlChar }
    if value.is_null() { return 0 as *mut xmlChar }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
        fullname =
            xmlBuildQName((*elem).name, (*(*elem).ns).prefix,
                          fn_0.as_mut_ptr(), 50 as std::os::raw::c_int);
        if fullname.is_null() { return 0 as *mut xmlChar }
        attrDecl = xmlGetDtdAttrDesc((*doc).intSubset, fullname, name);
        if attrDecl.is_null() && !(*doc).extSubset.is_null() {
            attrDecl = xmlGetDtdAttrDesc((*doc).extSubset, fullname, name);
            if !attrDecl.is_null() { extsubset = 1 as std::os::raw::c_int }
        }
        if fullname != fn_0.as_mut_ptr() &&
               fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as
                                                            *mut std::os::raw::c_void);
        }
    }
    if attrDecl.is_null() && !(*doc).intSubset.is_null() {
        attrDecl = xmlGetDtdAttrDesc((*doc).intSubset, (*elem).name, name)
    }
    if attrDecl.is_null() && !(*doc).extSubset.is_null() {
        attrDecl = xmlGetDtdAttrDesc((*doc).extSubset, (*elem).name, name);
        if !attrDecl.is_null() { extsubset = 1 as std::os::raw::c_int }
    }
    if attrDecl.is_null() { return 0 as *mut xmlChar }
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_CDATA as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    ret = xmlStrdup(value);
    if ret.is_null() { return 0 as *mut xmlChar }
    src = value;
    dst = ret;
    while *src as std::os::raw::c_int == 0x20 as std::os::raw::c_int { src = src.offset(1) }
    while *src as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *src as std::os::raw::c_int == 0x20 as std::os::raw::c_int {
            while *src as std::os::raw::c_int == 0x20 as std::os::raw::c_int {
                src = src.offset(1)
            }
            if *src as std::os::raw::c_int != 0 as std::os::raw::c_int {
                let fresh11 = dst;
                dst = dst.offset(1);
                *fresh11 = 0x20 as std::os::raw::c_int as xmlChar
            }
        } else {
            let fresh12 = src;
            src = src.offset(1);
            let fresh13 = dst;
            dst = dst.offset(1);
            *fresh13 = *fresh12
        }
    }
    *dst = 0 as std::os::raw::c_int as xmlChar;
    if (*doc).standalone != 0 && extsubset == 1 as std::os::raw::c_int &&
           xmlStrEqual(value, ret) == 0 {
        xmlErrValidNode(ctxt, elem, XML_DTD_NOT_STANDALONE,
                        b"standalone: %s on %s value had to be normalized based on external subset declaration\n\x00"
                            as *const u8 as *const std::os::raw::c_char, name,
                        (*elem).name, 0 as *const xmlChar);
        (*ctxt).valid = 0 as std::os::raw::c_int
    }
    return ret;
}
/* *
 * xmlValidNormalizeAttributeValue:
 * @doc:  the document
 * @elem:  the parent
 * @name:  the attribute name
 * @value:  the attribute value
 *
 * Does the validation related extra step of the normalization of attribute
 * values:
 *
 * If the declared value is not CDATA, then the XML processor must further
 * process the normalized attribute value by discarding any leading and
 * trailing space (#x20) characters, and by replacing sequences of space
 * (#x20) characters by single space (#x20) character.
 *
 * Returns a new normalized string if normalization is needed, NULL otherwise
 *      the caller must free the returned value.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidNormalizeAttributeValue(mut doc: xmlDocPtr,
                                                         mut elem: xmlNodePtr,
                                                         mut name:
                                                             *const xmlChar,
                                                         mut value:
                                                             *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut dst: *mut xmlChar = 0 as *mut xmlChar;
    let mut src: *const xmlChar = 0 as *const xmlChar;
    let mut attrDecl: xmlAttributePtr = 0 as xmlAttributePtr;
    if doc.is_null() { return 0 as *mut xmlChar }
    if elem.is_null() { return 0 as *mut xmlChar }
    if name.is_null() { return 0 as *mut xmlChar }
    if value.is_null() { return 0 as *mut xmlChar }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
        fullname =
            xmlBuildQName((*elem).name, (*(*elem).ns).prefix,
                          fn_0.as_mut_ptr(), 50 as std::os::raw::c_int);
        if fullname.is_null() { return 0 as *mut xmlChar }
        if fullname != fn_0.as_mut_ptr() &&
               fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as
                                                            *mut std::os::raw::c_void);
        }
    }
    attrDecl = xmlGetDtdAttrDesc((*doc).intSubset, (*elem).name, name);
    if attrDecl.is_null() && !(*doc).extSubset.is_null() {
        attrDecl = xmlGetDtdAttrDesc((*doc).extSubset, (*elem).name, name)
    }
    if attrDecl.is_null() { return 0 as *mut xmlChar }
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_CDATA as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as *mut xmlChar
    }
    ret = xmlStrdup(value);
    if ret.is_null() { return 0 as *mut xmlChar }
    src = value;
    dst = ret;
    while *src as std::os::raw::c_int == 0x20 as std::os::raw::c_int { src = src.offset(1) }
    while *src as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *src as std::os::raw::c_int == 0x20 as std::os::raw::c_int {
            while *src as std::os::raw::c_int == 0x20 as std::os::raw::c_int {
                src = src.offset(1)
            }
            if *src as std::os::raw::c_int != 0 as std::os::raw::c_int {
                let fresh14 = dst;
                dst = dst.offset(1);
                *fresh14 = 0x20 as std::os::raw::c_int as xmlChar
            }
        } else {
            let fresh15 = src;
            src = src.offset(1);
            let fresh16 = dst;
            dst = dst.offset(1);
            *fresh16 = *fresh15
        }
    }
    *dst = 0 as std::os::raw::c_int as xmlChar;
    return ret;
}
unsafe extern "C" fn xmlValidateAttributeIdCallback(mut payload:
                                                        *mut std::os::raw::c_void,
                                                    mut data:
                                                        *mut std::os::raw::c_void,
                                                    mut name:
                                                        *const xmlChar) {
    let mut attr: xmlAttributePtr = payload as xmlAttributePtr;
    let mut count: *mut std::os::raw::c_int = data as *mut std::os::raw::c_int;
    if (*attr).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
        *count += 1
    };
}
/* *
 * xmlValidateAttributeDecl:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @attr:  an attribute definition
 *
 * Try to validate a single attribute definition
 * basically it does the following checks as described by the
 * XML-1.0 recommendation:
 *  - [ VC: Attribute Default Legal ]
 *  - [ VC: Enumeration ]
 *  - [ VC: ID Attribute Default ]
 *
 * The ID/IDREF uniqueness and matching are done separately
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateAttributeDecl(mut ctxt: xmlValidCtxtPtr,
                                                  mut doc: xmlDocPtr,
                                                  mut attr: xmlAttributePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut val: std::os::raw::c_int = 0;
    if doc.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
            return 0 as std::os::raw::c_int
        }
    }
    if attr.is_null() { return 1 as std::os::raw::c_int }
    /* Attribute Default Legal */
    /* Enumeration */
    if !(*attr).defaultValue.is_null() {
        val =
            xmlValidateAttributeValueInternal(doc, (*attr).atype,
                                              (*attr).defaultValue);
        if val == 0 as std::os::raw::c_int {
            xmlErrValidNode(ctxt, attr as xmlNodePtr,
                            XML_DTD_ATTRIBUTE_DEFAULT,
                            b"Syntax of default value for attribute %s of %s is not valid\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*attr).name, (*attr).elem, 0 as *const xmlChar);
        }
        ret &= val
    }
    /* ID Attribute Default */
    if (*attr).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint &&
           (*attr).def as std::os::raw::c_uint !=
               XML_ATTRIBUTE_IMPLIED as std::os::raw::c_int as std::os::raw::c_uint &&
           (*attr).def as std::os::raw::c_uint !=
               XML_ATTRIBUTE_REQUIRED as std::os::raw::c_int as std::os::raw::c_uint {
        xmlErrValidNode(ctxt, attr as xmlNodePtr, XML_DTD_ID_FIXED,
                        b"ID attribute %s of %s is not valid must be #IMPLIED or #REQUIRED\n\x00"
                            as *const u8 as *const std::os::raw::c_char, (*attr).name,
                        (*attr).elem, 0 as *const xmlChar);
        ret = 0 as std::os::raw::c_int
    }
    /* One ID per Element Type */
    if (*attr).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
        let mut nbId: std::os::raw::c_int = 0;
        /* the trick is that we parse DtD as their own internal subset */
        let mut elem: xmlElementPtr =
            xmlGetDtdElementDesc((*doc).intSubset, (*attr).elem);
        if !elem.is_null() {
            nbId =
                xmlScanIDAttributeDecl(0 as xmlValidCtxtPtr, elem,
                                       0 as std::os::raw::c_int)
        } else {
            let mut table: xmlAttributeTablePtr = 0 as *mut xmlAttributeTable;
            /*
	     * The attribute may be declared in the internal subset and the
	     * element in the external subset.
	     */
            nbId = 0 as std::os::raw::c_int;
            if !(*doc).intSubset.is_null() {
                table =
                    (*(*doc).intSubset).attributes as xmlAttributeTablePtr;
                xmlHashScan3(table, 0 as *const xmlChar, 0 as *const xmlChar,
                             (*attr).elem,
                             Some(xmlValidateAttributeIdCallback as
                                      unsafe extern "C" fn(_:
                                                               *mut std::os::raw::c_void,
                                                           _:
                                                               *mut std::os::raw::c_void,
                                                           _: *const xmlChar)
                                          -> ()),
                             &mut nbId as *mut std::os::raw::c_int as
                                 *mut std::os::raw::c_void);
            }
        }
        if nbId > 1 as std::os::raw::c_int {
            xmlErrValidNodeNr(ctxt, attr as xmlNodePtr, XML_DTD_ID_SUBSET,
                              b"Element %s has %d ID attribute defined in the internal subset : %s\n\x00"
                                  as *const u8 as *const std::os::raw::c_char,
                              (*attr).elem, nbId, (*attr).name);
        } else if !(*doc).extSubset.is_null() {
            let mut extId: std::os::raw::c_int = 0 as std::os::raw::c_int;
            elem = xmlGetDtdElementDesc((*doc).extSubset, (*attr).elem);
            if !elem.is_null() {
                extId =
                    xmlScanIDAttributeDecl(0 as xmlValidCtxtPtr, elem,
                                           0 as std::os::raw::c_int)
            }
            if extId > 1 as std::os::raw::c_int {
                xmlErrValidNodeNr(ctxt, attr as xmlNodePtr, XML_DTD_ID_SUBSET,
                                  b"Element %s has %d ID attribute defined in the external subset : %s\n\x00"
                                      as *const u8 as *const std::os::raw::c_char,
                                  (*attr).elem, extId, (*attr).name);
            } else if extId + nbId > 1 as std::os::raw::c_int {
                xmlErrValidNode(ctxt, attr as xmlNodePtr, XML_DTD_ID_SUBSET,
                                b"Element %s has ID attributes defined in the internal and external subset : %s\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                (*attr).elem, (*attr).name,
                                0 as *const xmlChar);
            }
        }
    }
    /* Validity Constraint: Enumeration */
    if !(*attr).defaultValue.is_null() && !(*attr).tree.is_null() {
        let mut tree: xmlEnumerationPtr = (*attr).tree;
        while !tree.is_null() {
            if xmlStrEqual((*tree).name, (*attr).defaultValue) != 0 {
                break ;
            }
            tree = (*tree).next
        }
        if tree.is_null() {
            xmlErrValidNode(ctxt, attr as xmlNodePtr, XML_DTD_ATTRIBUTE_VALUE,
                            b"Default value \"%s\" for attribute %s of %s is not among the enumerated set\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*attr).defaultValue, (*attr).name, (*attr).elem);
            ret = 0 as std::os::raw::c_int
        }
    }
    return ret;
}
/* *
 * xmlValidateElementDecl:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element definition
 *
 * Try to validate a single element definition
 * basically it does the following checks as described by the
 * XML-1.0 recommendation:
 *  - [ VC: One ID per Element Type ]
 *  - [ VC: No Duplicate Types ]
 *  - [ VC: Unique Element Type Declaration ]
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateElementDecl(mut ctxt: xmlValidCtxtPtr,
                                                mut doc: xmlDocPtr,
                                                mut elem: xmlElementPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut tst: xmlElementPtr = 0 as *mut xmlElement;
    if doc.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
            return 0 as std::os::raw::c_int
        }
    }
    if elem.is_null() { return 1 as std::os::raw::c_int }
    /* No Duplicate Types */
    if (*elem).etype as std::os::raw::c_uint ==
           XML_ELEMENT_TYPE_MIXED as std::os::raw::c_int as std::os::raw::c_uint {
        let mut cur: xmlElementContentPtr = 0 as *mut xmlElementContent;
        let mut next: xmlElementContentPtr = 0 as *mut xmlElementContent;
        let mut name: *const xmlChar = 0 as *const xmlChar;
        cur = (*elem).content;
        while !cur.is_null() {
            if (*cur).type_0 as std::os::raw::c_uint !=
                   XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            if (*cur).c1.is_null() { break ; }
            if (*(*cur).c1).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint
               {
                name = (*(*cur).c1).name;
                next = (*cur).c2;
                while !next.is_null() {
                    if (*next).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        if xmlStrEqual((*next).name, name) != 0 &&
                               xmlStrEqual((*next).prefix,
                                           (*(*cur).c1).prefix) != 0 {
                            if (*(*cur).c1).prefix.is_null() {
                                xmlErrValidNode(ctxt, elem as xmlNodePtr,
                                                XML_DTD_CONTENT_ERROR,
                                                b"Definition of %s has duplicate references of %s\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char,
                                                (*elem).name, name,
                                                0 as *const xmlChar);
                            } else {
                                xmlErrValidNode(ctxt, elem as xmlNodePtr,
                                                XML_DTD_CONTENT_ERROR,
                                                b"Definition of %s has duplicate references of %s:%s\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char,
                                                (*elem).name,
                                                (*(*cur).c1).prefix, name);
                            }
                            ret = 0 as std::os::raw::c_int
                        }
                        break ;
                    } else {
                        if (*next).c1.is_null() { break ; }
                        if (*(*next).c1).type_0 as std::os::raw::c_uint !=
                               XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                            break ;
                        }
                        if xmlStrEqual((*(*next).c1).name, name) != 0 &&
                               xmlStrEqual((*(*next).c1).prefix,
                                           (*(*cur).c1).prefix) != 0 {
                            if (*(*cur).c1).prefix.is_null() {
                                xmlErrValidNode(ctxt, elem as xmlNodePtr,
                                                XML_DTD_CONTENT_ERROR,
                                                b"Definition of %s has duplicate references to %s\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char,
                                                (*elem).name, name,
                                                0 as *const xmlChar);
                            } else {
                                xmlErrValidNode(ctxt, elem as xmlNodePtr,
                                                XML_DTD_CONTENT_ERROR,
                                                b"Definition of %s has duplicate references to %s:%s\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char,
                                                (*elem).name,
                                                (*(*cur).c1).prefix, name);
                            }
                            ret = 0 as std::os::raw::c_int
                        }
                        next = (*next).c2
                    }
                }
            }
            cur = (*cur).c2
        }
    }
    /* VC: Unique Element Type Declaration */
    tst = xmlGetDtdElementDesc((*doc).intSubset, (*elem).name);
    if !tst.is_null() && tst != elem &&
           ((*tst).prefix == (*elem).prefix ||
                xmlStrEqual((*tst).prefix, (*elem).prefix) != 0) &&
           (*tst).etype as std::os::raw::c_uint !=
               XML_ELEMENT_TYPE_UNDEFINED as std::os::raw::c_int as std::os::raw::c_uint {
        xmlErrValidNode(ctxt, elem as xmlNodePtr, XML_DTD_ELEM_REDEFINED,
                        b"Redefinition of element %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, (*elem).name,
                        0 as *const xmlChar, 0 as *const xmlChar);
        ret = 0 as std::os::raw::c_int
    }
    tst = xmlGetDtdElementDesc((*doc).extSubset, (*elem).name);
    if !tst.is_null() && tst != elem &&
           ((*tst).prefix == (*elem).prefix ||
                xmlStrEqual((*tst).prefix, (*elem).prefix) != 0) &&
           (*tst).etype as std::os::raw::c_uint !=
               XML_ELEMENT_TYPE_UNDEFINED as std::os::raw::c_int as std::os::raw::c_uint {
        xmlErrValidNode(ctxt, elem as xmlNodePtr, XML_DTD_ELEM_REDEFINED,
                        b"Redefinition of element %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, (*elem).name,
                        0 as *const xmlChar, 0 as *const xmlChar);
        ret = 0 as std::os::raw::c_int
    }
    /* One ID per Element Type
     * already done when registering the attribute
    if (xmlScanIDAttributeDecl(ctxt, elem) > 1) {
	ret = 0;
    } */
    return ret;
}
/* *
 * xmlValidateOneAttribute:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 * @attr:  an attribute instance
 * @value:  the attribute value (without entities processing)
 *
 * Try to validate a single attribute for an element
 * basically it does the following checks as described by the
 * XML-1.0 recommendation:
 *  - [ VC: Attribute Value Type ]
 *  - [ VC: Fixed Attribute Default ]
 *  - [ VC: Entity Name ]
 *  - [ VC: Name Token ]
 *  - [ VC: ID ]
 *  - [ VC: IDREF ]
 *  - [ VC: Entity Name ]
 *  - [ VC: Notation Attributes ]
 *
 * The ID/IDREF uniqueness and matching are done separately
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateOneAttribute(mut ctxt: xmlValidCtxtPtr,
                                                 mut doc: xmlDocPtr,
                                                 mut elem: xmlNodePtr,
                                                 mut attr: xmlAttrPtr,
                                                 mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut attrDecl: xmlAttributePtr = 0 as xmlAttributePtr;
    let mut val: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if doc.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
            return 0 as std::os::raw::c_int
        }
    }
    if elem.is_null() || (*elem).name.is_null() { return 0 as std::os::raw::c_int }
    if attr.is_null() || (*attr).name.is_null() { return 0 as std::os::raw::c_int }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
        fullname =
            xmlBuildQName((*elem).name, (*(*elem).ns).prefix,
                          fn_0.as_mut_ptr(), 50 as std::os::raw::c_int);
        if fullname.is_null() { return 0 as std::os::raw::c_int }
        if !(*attr).ns.is_null() {
            attrDecl =
                xmlGetDtdQAttrDesc((*doc).intSubset, fullname, (*attr).name,
                                   (*(*attr).ns).prefix);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdQAttrDesc((*doc).extSubset, fullname,
                                       (*attr).name, (*(*attr).ns).prefix)
            }
        } else {
            attrDecl =
                xmlGetDtdAttrDesc((*doc).intSubset, fullname, (*attr).name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset, fullname,
                                      (*attr).name)
            }
        }
        if fullname != fn_0.as_mut_ptr() &&
               fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as
                                                            *mut std::os::raw::c_void);
        }
    }
    if attrDecl.is_null() {
        if !(*attr).ns.is_null() {
            attrDecl =
                xmlGetDtdQAttrDesc((*doc).intSubset, (*elem).name,
                                   (*attr).name, (*(*attr).ns).prefix);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdQAttrDesc((*doc).extSubset, (*elem).name,
                                       (*attr).name, (*(*attr).ns).prefix)
            }
        } else {
            attrDecl =
                xmlGetDtdAttrDesc((*doc).intSubset, (*elem).name,
                                  (*attr).name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset, (*elem).name,
                                      (*attr).name)
            }
        }
    }
    /* Validity Constraint: Attribute Value Type */
    if attrDecl.is_null() {
        xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_ATTRIBUTE,
                        b"No declaration for attribute %s of element %s\n\x00"
                            as *const u8 as *const std::os::raw::c_char, (*attr).name,
                        (*elem).name, 0 as *const xmlChar);
        return 0 as std::os::raw::c_int
    }
    (*attr).atype = (*attrDecl).atype;
    val = xmlValidateAttributeValueInternal(doc, (*attrDecl).atype, value);
    if val == 0 as std::os::raw::c_int {
        xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_VALUE,
                        b"Syntax of value for attribute %s of %s is not valid\n\x00"
                            as *const u8 as *const std::os::raw::c_char, (*attr).name,
                        (*elem).name, 0 as *const xmlChar);
        ret = 0 as std::os::raw::c_int
    }
    /* Validity constraint: Fixed Attribute Default */
    if (*attrDecl).def as std::os::raw::c_uint ==
           XML_ATTRIBUTE_FIXED as std::os::raw::c_int as std::os::raw::c_uint {
        if xmlStrEqual(value, (*attrDecl).defaultValue) == 0 {
            xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_DEFAULT,
                            b"Value for attribute %s of %s is different from default \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*attr).name, (*elem).name,
                            (*attrDecl).defaultValue);
            ret = 0 as std::os::raw::c_int
        }
    }
    /* Validity Constraint: ID uniqueness */
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint {
        if xmlAddID(ctxt, doc, value, attr).is_null() {
            ret = 0 as std::os::raw::c_int
        }
    }
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_IDREF as std::os::raw::c_int as std::os::raw::c_uint ||
           (*attrDecl).atype as std::os::raw::c_uint ==
               XML_ATTRIBUTE_IDREFS as std::os::raw::c_int as std::os::raw::c_uint {
        if xmlAddRef(ctxt, doc, value, attr).is_null() {
            ret = 0 as std::os::raw::c_int
        }
    }
    /* Validity Constraint: Notation Attributes */
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NOTATION as std::os::raw::c_int as std::os::raw::c_uint {
        let mut tree: xmlEnumerationPtr = (*attrDecl).tree;
        let mut nota: xmlNotationPtr = 0 as *mut xmlNotation;
        /* First check that the given NOTATION was declared */
        nota = xmlGetDtdNotationDesc((*doc).intSubset, value);
        if nota.is_null() {
            nota = xmlGetDtdNotationDesc((*doc).extSubset, value)
        }
        if nota.is_null() {
            xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_NOTATION,
                            b"Value \"%s\" for attribute %s of %s is not a declared Notation\n\x00"
                                as *const u8 as *const std::os::raw::c_char, value,
                            (*attr).name, (*elem).name);
            ret = 0 as std::os::raw::c_int
        }
        /* Second, verify that it's among the list */
        while !tree.is_null() {
            if xmlStrEqual((*tree).name, value) != 0 { break ; }
            tree = (*tree).next
        }
        if tree.is_null() {
            xmlErrValidNode(ctxt, elem, XML_DTD_NOTATION_VALUE,
                            b"Value \"%s\" for attribute %s of %s is not among the enumerated notations\n\x00"
                                as *const u8 as *const std::os::raw::c_char, value,
                            (*attr).name, (*elem).name);
            ret = 0 as std::os::raw::c_int
        }
    }
    /* Validity Constraint: Enumeration */
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_ENUMERATION as std::os::raw::c_int as std::os::raw::c_uint {
        let mut tree_0: xmlEnumerationPtr = (*attrDecl).tree;
        while !tree_0.is_null() {
            if xmlStrEqual((*tree_0).name, value) != 0 { break ; }
            tree_0 = (*tree_0).next
        }
        if tree_0.is_null() {
            xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_VALUE,
                            b"Value \"%s\" for attribute %s of %s is not among the enumerated set\n\x00"
                                as *const u8 as *const std::os::raw::c_char, value,
                            (*attr).name, (*elem).name);
            ret = 0 as std::os::raw::c_int
        }
    }
    /* Fixed Attribute Default */
    if (*attrDecl).def as std::os::raw::c_uint ==
           XML_ATTRIBUTE_FIXED as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*attrDecl).defaultValue, value) == 0 {
        xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_VALUE,
                        b"Value for attribute %s of %s must be \"%s\"\n\x00"
                            as *const u8 as *const std::os::raw::c_char, (*attr).name,
                        (*elem).name, (*attrDecl).defaultValue);
        ret = 0 as std::os::raw::c_int
    }
    /* Extra check for the attribute value */
    ret &=
        xmlValidateAttributeValue2(ctxt, doc, (*attr).name, (*attrDecl).atype,
                                   value);
    return ret;
}
/* *
 * xmlValidateOneNamespace:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 * @prefix:  the namespace prefix
 * @ns:  an namespace declaration instance
 * @value:  the attribute value (without entities processing)
 *
 * Try to validate a single namespace declaration for an element
 * basically it does the following checks as described by the
 * XML-1.0 recommendation:
 *  - [ VC: Attribute Value Type ]
 *  - [ VC: Fixed Attribute Default ]
 *  - [ VC: Entity Name ]
 *  - [ VC: Name Token ]
 *  - [ VC: ID ]
 *  - [ VC: IDREF ]
 *  - [ VC: Entity Name ]
 *  - [ VC: Notation Attributes ]
 *
 * The ID/IDREF uniqueness and matching are done separately
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateOneNamespace(mut ctxt: xmlValidCtxtPtr,
                                                 mut doc: xmlDocPtr,
                                                 mut elem: xmlNodePtr,
                                                 mut prefix: *const xmlChar,
                                                 mut ns: xmlNsPtr,
                                                 mut value: *const xmlChar)
 -> std::os::raw::c_int {
    /* xmlElementPtr elemDecl; */
    let mut attrDecl: xmlAttributePtr = 0 as xmlAttributePtr;
    let mut val: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if doc.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
            return 0 as std::os::raw::c_int
        }
    }
    if elem.is_null() || (*elem).name.is_null() { return 0 as std::os::raw::c_int }
    if ns.is_null() || (*ns).href.is_null() { return 0 as std::os::raw::c_int }
    if !prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
        fullname =
            xmlBuildQName((*elem).name, prefix, fn_0.as_mut_ptr(),
                          50 as std::os::raw::c_int);
        if fullname.is_null() {
            xmlVErrMemory(ctxt,
                          b"Validating namespace\x00" as *const u8 as
                              *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
        if !(*ns).prefix.is_null() {
            attrDecl =
                xmlGetDtdQAttrDesc((*doc).intSubset, fullname, (*ns).prefix,
                                   b"xmlns\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdQAttrDesc((*doc).extSubset, fullname,
                                       (*ns).prefix,
                                       b"xmlns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar)
            }
        } else {
            attrDecl =
                xmlGetDtdAttrDesc((*doc).intSubset, fullname,
                                  b"xmlns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset, fullname,
                                      b"xmlns\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
            }
        }
        if fullname != fn_0.as_mut_ptr() &&
               fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as
                                                            *mut std::os::raw::c_void);
        }
    }
    if attrDecl.is_null() {
        if !(*ns).prefix.is_null() {
            attrDecl =
                xmlGetDtdQAttrDesc((*doc).intSubset, (*elem).name,
                                   (*ns).prefix,
                                   b"xmlns\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdQAttrDesc((*doc).extSubset, (*elem).name,
                                       (*ns).prefix,
                                       b"xmlns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar)
            }
        } else {
            attrDecl =
                xmlGetDtdAttrDesc((*doc).intSubset, (*elem).name,
                                  b"xmlns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset, (*elem).name,
                                      b"xmlns\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
            }
        }
    }
    /* Validity Constraint: Attribute Value Type */
    if attrDecl.is_null() {
        if !(*ns).prefix.is_null() {
            xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_ATTRIBUTE,
                            b"No declaration for attribute xmlns:%s of element %s\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*ns).prefix, (*elem).name, 0 as *const xmlChar);
        } else {
            xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_ATTRIBUTE,
                            b"No declaration for attribute xmlns of element %s\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*elem).name, 0 as *const xmlChar,
                            0 as *const xmlChar);
        }
        return 0 as std::os::raw::c_int
    }
    val = xmlValidateAttributeValueInternal(doc, (*attrDecl).atype, value);
    if val == 0 as std::os::raw::c_int {
        if !(*ns).prefix.is_null() {
            xmlErrValidNode(ctxt, elem, XML_DTD_INVALID_DEFAULT,
                            b"Syntax of value for attribute xmlns:%s of %s is not valid\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*ns).prefix, (*elem).name, 0 as *const xmlChar);
        } else {
            xmlErrValidNode(ctxt, elem, XML_DTD_INVALID_DEFAULT,
                            b"Syntax of value for attribute xmlns of %s is not valid\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*elem).name, 0 as *const xmlChar,
                            0 as *const xmlChar);
        }
        ret = 0 as std::os::raw::c_int
    }
    /* Validity constraint: Fixed Attribute Default */
    if (*attrDecl).def as std::os::raw::c_uint ==
           XML_ATTRIBUTE_FIXED as std::os::raw::c_int as std::os::raw::c_uint {
        if xmlStrEqual(value, (*attrDecl).defaultValue) == 0 {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_DEFAULT,
                                b"Value for attribute xmlns:%s of %s is different from default \"%s\"\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                (*ns).prefix, (*elem).name,
                                (*attrDecl).defaultValue);
            } else {
                xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_DEFAULT,
                                b"Value for attribute xmlns of %s is different from default \"%s\"\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                (*elem).name, (*attrDecl).defaultValue,
                                0 as *const xmlChar);
            }
            ret = 0 as std::os::raw::c_int
        }
    }
    /*
     * Casting ns to xmlAttrPtr is wrong. We'd need separate functions
     * xmlAddID and xmlAddRef for namespace declarations, but it makes
     * no practical sense to use ID types anyway.
     */
    /* Validity Constraint: Notation Attributes */
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NOTATION as std::os::raw::c_int as std::os::raw::c_uint {
        let mut tree: xmlEnumerationPtr = (*attrDecl).tree;
        let mut nota: xmlNotationPtr = 0 as *mut xmlNotation;
        /* First check that the given NOTATION was declared */
        nota = xmlGetDtdNotationDesc((*doc).intSubset, value);
        if nota.is_null() {
            nota = xmlGetDtdNotationDesc((*doc).extSubset, value)
        }
        if nota.is_null() {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_NOTATION,
                                b"Value \"%s\" for attribute xmlns:%s of %s is not a declared Notation\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, (*ns).prefix, (*elem).name);
            } else {
                xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_NOTATION,
                                b"Value \"%s\" for attribute xmlns of %s is not a declared Notation\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, (*elem).name, 0 as *const xmlChar);
            }
            ret = 0 as std::os::raw::c_int
        }
        /* Second, verify that it's among the list */
        while !tree.is_null() {
            if xmlStrEqual((*tree).name, value) != 0 { break ; }
            tree = (*tree).next
        }
        if tree.is_null() {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(ctxt, elem, XML_DTD_NOTATION_VALUE,
                                b"Value \"%s\" for attribute xmlns:%s of %s is not among the enumerated notations\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, (*ns).prefix, (*elem).name);
            } else {
                xmlErrValidNode(ctxt, elem, XML_DTD_NOTATION_VALUE,
                                b"Value \"%s\" for attribute xmlns of %s is not among the enumerated notations\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, (*elem).name, 0 as *const xmlChar);
            }
            ret = 0 as std::os::raw::c_int
        }
    }
    /* Validity Constraint: Enumeration */
    if (*attrDecl).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_ENUMERATION as std::os::raw::c_int as std::os::raw::c_uint {
        let mut tree_0: xmlEnumerationPtr = (*attrDecl).tree;
        while !tree_0.is_null() {
            if xmlStrEqual((*tree_0).name, value) != 0 { break ; }
            tree_0 = (*tree_0).next
        }
        if tree_0.is_null() {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_VALUE,
                                b"Value \"%s\" for attribute xmlns:%s of %s is not among the enumerated set\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, (*ns).prefix, (*elem).name);
            } else {
                xmlErrValidNode(ctxt, elem, XML_DTD_ATTRIBUTE_VALUE,
                                b"Value \"%s\" for attribute xmlns of %s is not among the enumerated set\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                value, (*elem).name, 0 as *const xmlChar);
            }
            ret = 0 as std::os::raw::c_int
        }
    }
    /* Fixed Attribute Default */
    if (*attrDecl).def as std::os::raw::c_uint ==
           XML_ATTRIBUTE_FIXED as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*attrDecl).defaultValue, value) == 0 {
        if !(*ns).prefix.is_null() {
            xmlErrValidNode(ctxt, elem, XML_DTD_ELEM_NAMESPACE,
                            b"Value for attribute xmlns:%s of %s must be \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*ns).prefix, (*elem).name,
                            (*attrDecl).defaultValue);
        } else {
            xmlErrValidNode(ctxt, elem, XML_DTD_ELEM_NAMESPACE,
                            b"Value for attribute xmlns of %s must be \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*elem).name, (*attrDecl).defaultValue,
                            0 as *const xmlChar);
        }
        ret = 0 as std::os::raw::c_int
    }
    /* Extra check for the attribute value */
    if !(*ns).prefix.is_null() {
        ret &=
            xmlValidateAttributeValue2(ctxt, doc, (*ns).prefix,
                                       (*attrDecl).atype, value)
    } else {
        ret &=
            xmlValidateAttributeValue2(ctxt, doc,
                                       b"xmlns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, (*attrDecl).atype,
                                       value)
    }
    return ret;
}
/* *
 * xmlSnprintfElements:
 * @buf:  an output buffer
 * @size:  the size of the buffer
 * @content:  An element
 * @glob: 1 if one must print the englobing parenthesis, 0 otherwise
 *
 * This will dump the list of elements to the buffer
 * Intended just for the debug routine
 */
unsafe extern "C" fn xmlSnprintfElements(mut buf: *mut std::os::raw::c_char,
                                         mut size: std::os::raw::c_int,
                                         mut node: xmlNodePtr,
                                         mut glob: std::os::raw::c_int) {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut len: std::os::raw::c_int = 0;
    if node.is_null() { return }
    if glob != 0 {
        strcat(buf, b"(\x00" as *const u8 as *const std::os::raw::c_char);
    }
    cur = node;
    while !cur.is_null() {
        len = strlen(buf) as std::os::raw::c_int;
        if size - len < 50 as std::os::raw::c_int {
            if size - len > 4 as std::os::raw::c_int &&
                   *buf.offset((len - 1 as std::os::raw::c_int) as isize) as
                       std::os::raw::c_int != '.' as i32 {
                strcat(buf, b" ...\x00" as *const u8 as *const std::os::raw::c_char);
            }
            return
        }
        let mut current_block_33: u64;
        match (*cur).type_0 as std::os::raw::c_uint {
            1 => {
                if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
                    if size - len <
                           xmlStrlen((*(*cur).ns).prefix) + 10 as std::os::raw::c_int
                       {
                        if size - len > 4 as std::os::raw::c_int &&
                               *buf.offset((len - 1 as std::os::raw::c_int) as isize)
                                   as std::os::raw::c_int != '.' as i32 {
                            strcat(buf,
                                   b" ...\x00" as *const u8 as
                                       *const std::os::raw::c_char);
                        }
                        return
                    }
                    strcat(buf, (*(*cur).ns).prefix as *mut std::os::raw::c_char);
                    strcat(buf, b":\x00" as *const u8 as *const std::os::raw::c_char);
                }
                if size - len < xmlStrlen((*cur).name) + 10 as std::os::raw::c_int {
                    if size - len > 4 as std::os::raw::c_int &&
                           *buf.offset((len - 1 as std::os::raw::c_int) as isize) as
                               std::os::raw::c_int != '.' as i32 {
                        strcat(buf,
                               b" ...\x00" as *const u8 as
                                   *const std::os::raw::c_char);
                    }
                    return
                }
                strcat(buf, (*cur).name as *mut std::os::raw::c_char);
                if !(*cur).next.is_null() {
                    strcat(buf, b" \x00" as *const u8 as *const std::os::raw::c_char);
                }
                current_block_33 = 18435049525520518667;
            }
            3 => {
                if xmlIsBlankNode(cur as *const xmlNode) != 0 {
                    current_block_33 = 18435049525520518667;
                } else { current_block_33 = 5665115878908524133; }
            }
            4 | 5 => { current_block_33 = 5665115878908524133; }
            2 | 9 | 21 | 13 | 10 | 11 | 12 | 18 => {
                strcat(buf, b"???\x00" as *const u8 as *const std::os::raw::c_char);
                if !(*cur).next.is_null() {
                    strcat(buf, b" \x00" as *const u8 as *const std::os::raw::c_char);
                }
                current_block_33 = 18435049525520518667;
            }
            6 | 7 | 14 | 8 | 15 | 16 | 17 | 19 | 20 | _ => {
                current_block_33 = 18435049525520518667;
            }
        }
        match current_block_33 {
            5665115878908524133 =>
            /* Falls through. */
            {
                strcat(buf, b"CDATA\x00" as *const u8 as *const std::os::raw::c_char);
                if !(*cur).next.is_null() {
                    strcat(buf, b" \x00" as *const u8 as *const std::os::raw::c_char);
                }
            }
            _ => { }
        }
        cur = (*cur).next
    }
    if glob != 0 {
        strcat(buf, b")\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* *
 * xmlValidateElementContent:
 * @ctxt:  the validation context
 * @child:  the child list
 * @elemDecl:  pointer to the element declaration
 * @warn:  emit the error message
 * @parent: the parent element (for error reporting)
 *
 * Try to validate the content model of an element
 *
 * returns 1 if valid or 0 if not and -1 in case of error
 */
unsafe extern "C" fn xmlValidateElementContent(mut ctxt: xmlValidCtxtPtr,
                                               mut child: xmlNodePtr,
                                               mut elemDecl: xmlElementPtr,
                                               mut warn: std::os::raw::c_int,
                                               mut parent: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut cont: xmlElementContentPtr = 0 as *mut xmlElementContent;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    if elemDecl.is_null() || parent.is_null() || ctxt.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    cont = (*elemDecl).content;
    name = (*elemDecl).name;
    /* Build the regexp associated to the content model */
    if (*elemDecl).contModel.is_null() {
        ret = xmlValidBuildContentModel(ctxt, elemDecl)
    }
    if (*elemDecl).contModel.is_null() {
        return -(1 as std::os::raw::c_int)
    } else {
        let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
        if xmlRegexpIsDeterminist((*elemDecl).contModel) == 0 {
            return -(1 as std::os::raw::c_int)
        }
        (*ctxt).nodeMax = 0 as std::os::raw::c_int;
        (*ctxt).nodeNr = 0 as std::os::raw::c_int;
        (*ctxt).nodeTab = 0 as *mut xmlNodePtr;
        exec =
            xmlRegNewExecCtxt((*elemDecl).contModel, None,
                              0 as *mut std::os::raw::c_void);
        if !exec.is_null() {
            cur = child;
            loop  {
                if cur.is_null() {
                    current_block = 1345366029464561491;
                    break ;
                }
                match (*cur).type_0 as std::os::raw::c_uint {
                    5 => {
                        /*
			 * Push the current node to be able to roll back
			 * and process within the entity
			 */
                        if !(*cur).children.is_null() &&
                               !(*(*cur).children).children.is_null() {
                            nodeVPush(ctxt, cur);
                            cur = (*(*cur).children).children;
                            continue ;
                        }
                    }
                    3 => {
                        if !(xmlIsBlankNode(cur as *const xmlNode) != 0) {
                            ret = 0 as std::os::raw::c_int;
                            current_block = 11577632886650045160;
                            break ;
                        }
                    }
                    4 => {
                        /* TODO */
                        ret = 0 as std::os::raw::c_int;
                        current_block = 11577632886650045160;
                        break ;
                    }
                    1 => {
                        if !(*cur).ns.is_null() &&
                               !(*(*cur).ns).prefix.is_null() {
                            let mut fn_0: [xmlChar; 50] = [0; 50];
                            let mut fullname: *mut xmlChar =
                                0 as *mut xmlChar;
                            fullname =
                                xmlBuildQName((*cur).name,
                                              (*(*cur).ns).prefix,
                                              fn_0.as_mut_ptr(),
                                              50 as std::os::raw::c_int);
                            if fullname.is_null() {
                                ret = -(1 as std::os::raw::c_int);
                                current_block = 11577632886650045160;
                                break ;
                            } else {
                                ret =
                                    xmlRegExecPushString(exec, fullname,
                                                         0 as
                                                             *mut std::os::raw::c_void);
                                if fullname != fn_0.as_mut_ptr() &&
                                       fullname != (*cur).name as *mut xmlChar
                                   {
                                    xmlFree.expect("non-null function pointer")(fullname
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                            }
                        } else {
                            ret =
                                xmlRegExecPushString(exec, (*cur).name,
                                                     0 as *mut std::os::raw::c_void)
                        }
                    }
                    _ => { }
                }
                /*
		 * Switch to next element
		 */
                cur = (*cur).next;
                while cur.is_null() {
                    cur = nodeVPop(ctxt);
                    if cur.is_null() { break ; }
                    cur = (*cur).next
                }
            }
            match current_block {
                1345366029464561491 => {
                    ret =
                        xmlRegExecPushString(exec, 0 as *const xmlChar,
                                             0 as *mut std::os::raw::c_void)
                }
                _ => { }
            }
            xmlRegFreeExecCtxt(exec);
        }
        /* LIBXML_REGEXP_ENABLED */
        /* LIBXML_REGEXP_ENABLED */
        if warn != 0 &&
               (ret != 1 as std::os::raw::c_int && ret != -(3 as std::os::raw::c_int)) {
            if !ctxt.is_null() {
                let mut expr: [std::os::raw::c_char; 5000] = [0; 5000];
                let mut list: [std::os::raw::c_char; 5000] = [0; 5000];
                expr[0 as std::os::raw::c_int as usize] =
                    0 as std::os::raw::c_int as std::os::raw::c_char;
                xmlSnprintfElementContent(&mut *expr.as_mut_ptr().offset(0 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             isize),
                                          5000 as std::os::raw::c_int, cont,
                                          1 as std::os::raw::c_int);
                list[0 as std::os::raw::c_int as usize] =
                    0 as std::os::raw::c_int as std::os::raw::c_char;
                /* LIBXML_REGEXP_ENABLED */
                xmlSnprintfElements(&mut *list.as_mut_ptr().offset(0 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       isize),
                                    5000 as std::os::raw::c_int, child,
                                    1 as std::os::raw::c_int);
                if !name.is_null() {
                    xmlErrValidNode(ctxt, parent, XML_DTD_CONTENT_MODEL,
                                    b"Element %s content does not follow the DTD, expecting %s, got %s\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    name, expr.as_mut_ptr() as *mut xmlChar,
                                    list.as_mut_ptr() as *mut xmlChar);
                } else {
                    xmlErrValidNode(ctxt, parent, XML_DTD_CONTENT_MODEL,
                                    b"Element content does not follow the DTD, expecting %s, got %s\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    expr.as_mut_ptr() as *mut xmlChar,
                                    list.as_mut_ptr() as *mut xmlChar,
                                    0 as *const xmlChar);
                }
            } else if !name.is_null() {
                xmlErrValidNode(ctxt, parent, XML_DTD_CONTENT_MODEL,
                                b"Element %s content does not follow the DTD\n\x00"
                                    as *const u8 as *const std::os::raw::c_char, name,
                                0 as *const xmlChar, 0 as *const xmlChar);
            } else {
                xmlErrValidNode(ctxt, parent, XML_DTD_CONTENT_MODEL,
                                b"Element content does not follow the DTD\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as *const xmlChar);
            }
            ret = 0 as std::os::raw::c_int
        }
        if ret == -(3 as std::os::raw::c_int) { ret = 1 as std::os::raw::c_int }
        (*ctxt).nodeMax = 0 as std::os::raw::c_int;
        (*ctxt).nodeNr = 0 as std::os::raw::c_int;
        if !(*ctxt).nodeTab.is_null() {
            xmlFree.expect("non-null function pointer")((*ctxt).nodeTab as
                                                            *mut std::os::raw::c_void);
            (*ctxt).nodeTab = 0 as *mut xmlNodePtr
        }
        return ret
    };
}
/* *
 * xmlValidateCdataElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 *
 * Check that an element follows #CDATA
 *
 * returns 1 if valid or 0 otherwise
 */
unsafe extern "C" fn xmlValidateOneCdataElement(mut ctxt: xmlValidCtxtPtr,
                                                mut doc: xmlDocPtr,
                                                mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    if ctxt.is_null() || doc.is_null() || elem.is_null() ||
           (*elem).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    child = (*elem).children;
    cur = child;
    while !cur.is_null() {
        match (*cur).type_0 as std::os::raw::c_uint {
            5 => {
                /*
		 * Push the current node to be able to roll back
		 * and process within the entity
		 */
                if !(*cur).children.is_null() &&
                       !(*(*cur).children).children.is_null() {
                    nodeVPush(ctxt, cur);
                    cur = (*(*cur).children).children;
                    continue ;
                }
            }
            8 | 7 | 3 | 4 => { }
            _ => { ret = 0 as std::os::raw::c_int; break ; }
        }
        /*
	 * Switch to next element
	 */
        cur = (*cur).next;
        while cur.is_null() {
            cur = nodeVPop(ctxt);
            if cur.is_null() { break ; }
            cur = (*cur).next
        }
    }
    (*ctxt).nodeMax = 0 as std::os::raw::c_int;
    (*ctxt).nodeNr = 0 as std::os::raw::c_int;
    if !(*ctxt).nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nodeTab as
                                                        *mut std::os::raw::c_void);
        (*ctxt).nodeTab = 0 as *mut xmlNodePtr
    }
    return ret;
}
/* *
 * xmlValidateCheckMixed:
 * @ctxt:  the validation context
 * @cont:  the mixed content model
 * @qname:  the qualified name as appearing in the serialization
 *
 * Check if the given node is part of the content model.
 *
 * Returns 1 if yes, 0 if no, -1 in case of error
 */
unsafe extern "C" fn xmlValidateCheckMixed(mut ctxt: xmlValidCtxtPtr,
                                           mut cont: xmlElementContentPtr,
                                           mut qname: *const xmlChar)
 -> std::os::raw::c_int {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut plen: std::os::raw::c_int = 0;
    name = xmlSplitQName3(qname, &mut plen);
    if name.is_null() {
        while !cont.is_null() {
            if (*cont).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint
               {
                if (*cont).prefix.is_null() &&
                       xmlStrEqual((*cont).name, qname) != 0 {
                    return 1 as std::os::raw::c_int
                }
            } else if (*cont).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as
                              std::os::raw::c_uint && !(*cont).c1.is_null() &&
                          (*(*cont).c1).type_0 as std::os::raw::c_uint ==
                              XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                if (*(*cont).c1).prefix.is_null() &&
                       xmlStrEqual((*(*cont).c1).name, qname) != 0 {
                    return 1 as std::os::raw::c_int
                }
            } else if (*cont).type_0 as std::os::raw::c_uint !=
                          XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as
                              std::os::raw::c_uint || (*cont).c1.is_null() ||
                          (*(*cont).c1).type_0 as std::os::raw::c_uint !=
                              XML_ELEMENT_CONTENT_PCDATA as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                xmlErrValid(0 as xmlValidCtxtPtr, XML_DTD_MIXED_CORRUPT,
                            b"Internal: MIXED struct corrupted\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                break ;
            }
            cont = (*cont).c2
        }
    } else {
        while !cont.is_null() {
            if (*cont).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint
               {
                if !(*cont).prefix.is_null() &&
                       xmlStrncmp((*cont).prefix, qname, plen) ==
                           0 as std::os::raw::c_int &&
                       xmlStrEqual((*cont).name, name) != 0 {
                    return 1 as std::os::raw::c_int
                }
            } else if (*cont).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as
                              std::os::raw::c_uint && !(*cont).c1.is_null() &&
                          (*(*cont).c1).type_0 as std::os::raw::c_uint ==
                              XML_ELEMENT_CONTENT_ELEMENT as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                if !(*(*cont).c1).prefix.is_null() &&
                       xmlStrncmp((*(*cont).c1).prefix, qname, plen) ==
                           0 as std::os::raw::c_int &&
                       xmlStrEqual((*(*cont).c1).name, name) != 0 {
                    return 1 as std::os::raw::c_int
                }
            } else if (*cont).type_0 as std::os::raw::c_uint !=
                          XML_ELEMENT_CONTENT_OR as std::os::raw::c_int as
                              std::os::raw::c_uint || (*cont).c1.is_null() ||
                          (*(*cont).c1).type_0 as std::os::raw::c_uint !=
                              XML_ELEMENT_CONTENT_PCDATA as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                xmlErrValid(ctxt, XML_DTD_MIXED_CORRUPT,
                            b"Internal: MIXED struct corrupted\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
                break ;
            }
            cont = (*cont).c2
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlValidGetElemDecl:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 * @extsubset:  pointer, (out) indicate if the declaration was found
 *              in the external subset.
 *
 * Finds a declaration associated to an element in the document.
 *
 * returns the pointer to the declaration or NULL if not found.
 */
unsafe extern "C" fn xmlValidGetElemDecl(mut ctxt: xmlValidCtxtPtr,
                                         mut doc: xmlDocPtr,
                                         mut elem: xmlNodePtr,
                                         mut extsubset: *mut std::os::raw::c_int)
 -> xmlElementPtr {
    let mut elemDecl: xmlElementPtr = 0 as xmlElementPtr;
    let mut prefix: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() || doc.is_null() || elem.is_null() ||
           (*elem).name.is_null() {
        return 0 as xmlElementPtr
    }
    if !extsubset.is_null() { *extsubset = 0 as std::os::raw::c_int }
    /*
     * Fetch the declaration for the qualified name
     */
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        prefix = (*(*elem).ns).prefix
    }
    if !prefix.is_null() {
        elemDecl =
            xmlGetDtdQElementDesc((*doc).intSubset, (*elem).name, prefix);
        if elemDecl.is_null() && !(*doc).extSubset.is_null() {
            elemDecl =
                xmlGetDtdQElementDesc((*doc).extSubset, (*elem).name, prefix);
            if !elemDecl.is_null() && !extsubset.is_null() {
                *extsubset = 1 as std::os::raw::c_int
            }
        }
    }
    /*
     * Fetch the declaration for the non qualified name
     * This is "non-strict" validation should be done on the
     * full QName but in that case being flexible makes sense.
     */
    if elemDecl.is_null() {
        elemDecl = xmlGetDtdElementDesc((*doc).intSubset, (*elem).name);
        if elemDecl.is_null() && !(*doc).extSubset.is_null() {
            elemDecl = xmlGetDtdElementDesc((*doc).extSubset, (*elem).name);
            if !elemDecl.is_null() && !extsubset.is_null() {
                *extsubset = 1 as std::os::raw::c_int
            }
        }
    }
    if elemDecl.is_null() {
        xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_ELEM,
                        b"No declaration for element %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, (*elem).name,
                        0 as *const xmlChar, 0 as *const xmlChar);
    }
    return elemDecl;
}
/* *
 * xmlValidatePushElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 * @qname:  the qualified name as appearing in the serialization
 *
 * Push a new element start on the validation stack.
 *
 * returns 1 if no validation problem was found or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidatePushElement(mut ctxt: xmlValidCtxtPtr,
                                                mut doc: xmlDocPtr,
                                                mut elem: xmlNodePtr,
                                                mut qname: *const xmlChar)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut eDecl: xmlElementPtr = 0 as *mut xmlElement;
    let mut extsubset: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    /* printf("PushElem %s\n", qname); */
    if (*ctxt).vstateNr > 0 as std::os::raw::c_int && !(*ctxt).vstate.is_null() {
        let mut state: xmlValidStatePtr = (*ctxt).vstate;
        let mut elemDecl: xmlElementPtr = 0 as *mut xmlElement;
        /*
	 * Check the new element against the content model of the new elem.
	 */
        if !(*state).elemDecl.is_null() {
            elemDecl = (*state).elemDecl;
            match (*elemDecl).etype as std::os::raw::c_uint {
                0 => { ret = 0 as std::os::raw::c_int }
                1 => {
                    xmlErrValidNode(ctxt, (*state).node, XML_DTD_NOT_EMPTY,
                                    b"Element %s was declared EMPTY this one has content\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    (*(*state).node).name,
                                    0 as *const xmlChar, 0 as *const xmlChar);
                    ret = 0 as std::os::raw::c_int
                }
                3 => {
                    /* simple case of declared as #PCDATA */
                    if !(*elemDecl).content.is_null() &&
                           (*(*elemDecl).content).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_CONTENT_PCDATA as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        xmlErrValidNode(ctxt, (*state).node,
                                        XML_DTD_NOT_PCDATA,
                                        b"Element %s was declared #PCDATA but contains non text nodes\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char,
                                        (*(*state).node).name,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar);
                        ret = 0 as std::os::raw::c_int
                    } else {
                        ret =
                            xmlValidateCheckMixed(ctxt, (*elemDecl).content,
                                                  qname);
                        if ret != 1 as std::os::raw::c_int {
                            xmlErrValidNode(ctxt, (*state).node,
                                            XML_DTD_INVALID_CHILD,
                                            b"Element %s is not declared in %s list of possible children\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char, qname,
                                            (*(*state).node).name,
                                            0 as *const xmlChar);
                        }
                    }
                }
                4 => {
                    /*
		     * TODO:
		     * VC: Standalone Document Declaration
		     *     - element types with element content, if white space
		     *       occurs directly within any instance of those types.
		     */
                    if !(*state).exec.is_null() {
                        ret =
                            xmlRegExecPushString((*state).exec, qname,
                                                 0 as *mut std::os::raw::c_void);
                        if ret < 0 as std::os::raw::c_int {
                            xmlErrValidNode(ctxt, (*state).node,
                                            XML_DTD_CONTENT_MODEL,
                                            b"Element %s content does not follow the DTD, Misplaced %s\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*(*state).node).name, qname,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        } else { ret = 1 as std::os::raw::c_int }
                    }
                }
                2 | _ => { }
            }
        }
    }
    eDecl = xmlValidGetElemDecl(ctxt, doc, elem, &mut extsubset);
    vstateVPush(ctxt, eDecl, elem);
    return ret;
}
/* *
 * xmlValidatePushCData:
 * @ctxt:  the validation context
 * @data:  some character data read
 * @len:  the length of the data
 *
 * check the CData parsed for validation in the current stack
 *
 * returns 1 if no validation problem was found or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidatePushCData(mut ctxt: xmlValidCtxtPtr,
                                              mut data: *const xmlChar,
                                              mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    /* printf("CDATA %s %d\n", data, len); */
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if len <= 0 as std::os::raw::c_int { return ret }
    if (*ctxt).vstateNr > 0 as std::os::raw::c_int && !(*ctxt).vstate.is_null() {
        let mut state: xmlValidStatePtr = (*ctxt).vstate;
        let mut elemDecl: xmlElementPtr = 0 as *mut xmlElement;
        /*
	 * Check the new element against the content model of the new elem.
	 */
        if !(*state).elemDecl.is_null() {
            elemDecl = (*state).elemDecl;
            match (*elemDecl).etype as std::os::raw::c_uint {
                0 => {
                    current_block = 3336000707029410388;
                    match current_block {
                        3336000707029410388 => { ret = 0 as std::os::raw::c_int }
                        13693488741914411839 => {
                            if len > 0 as std::os::raw::c_int {
                                let mut i: std::os::raw::c_int = 0;
                                i = 0 as std::os::raw::c_int;
                                while i < len {
                                    if !(*data.offset(i as isize) as
                                             std::os::raw::c_int ==
                                             0x20 as std::os::raw::c_int ||
                                             0x9 as std::os::raw::c_int <=
                                                 *data.offset(i as isize) as
                                                     std::os::raw::c_int &&
                                                 *data.offset(i as isize) as
                                                     std::os::raw::c_int <=
                                                     0xa as std::os::raw::c_int ||
                                             *data.offset(i as isize) as
                                                 std::os::raw::c_int ==
                                                 0xd as std::os::raw::c_int) {
                                        xmlErrValidNode(ctxt, (*state).node,
                                                        XML_DTD_CONTENT_MODEL,
                                                        b"Element %s content does not follow the DTD, Text not allowed\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        (*(*state).node).name,
                                                        0 as *const xmlChar,
                                                        0 as *const xmlChar);
                                        ret = 0 as std::os::raw::c_int;
                                        break ;
                                    } else { i += 1 }
                                }
                                /*
			 * TODO:
			 * VC: Standalone Document Declaration
			 *  element types with element content, if white space
			 *  occurs directly within any instance of those types.
			 */
                            }
                        }
                        _ => {
                            xmlErrValidNode(ctxt, (*state).node,
                                            XML_DTD_NOT_EMPTY,
                                            b"Element %s was declared EMPTY this one has content\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*(*state).node).name,
                                            0 as *const xmlChar,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        }
                    }
                }
                1 => {
                    current_block = 8169704822944137850;
                    match current_block {
                        3336000707029410388 => { ret = 0 as std::os::raw::c_int }
                        13693488741914411839 => {
                            if len > 0 as std::os::raw::c_int {
                                let mut i: std::os::raw::c_int = 0;
                                i = 0 as std::os::raw::c_int;
                                while i < len {
                                    if !(*data.offset(i as isize) as
                                             std::os::raw::c_int ==
                                             0x20 as std::os::raw::c_int ||
                                             0x9 as std::os::raw::c_int <=
                                                 *data.offset(i as isize) as
                                                     std::os::raw::c_int &&
                                                 *data.offset(i as isize) as
                                                     std::os::raw::c_int <=
                                                     0xa as std::os::raw::c_int ||
                                             *data.offset(i as isize) as
                                                 std::os::raw::c_int ==
                                                 0xd as std::os::raw::c_int) {
                                        xmlErrValidNode(ctxt, (*state).node,
                                                        XML_DTD_CONTENT_MODEL,
                                                        b"Element %s content does not follow the DTD, Text not allowed\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        (*(*state).node).name,
                                                        0 as *const xmlChar,
                                                        0 as *const xmlChar);
                                        ret = 0 as std::os::raw::c_int;
                                        break ;
                                    } else { i += 1 }
                                }
                            }
                        }
                        _ => {
                            xmlErrValidNode(ctxt, (*state).node,
                                            XML_DTD_NOT_EMPTY,
                                            b"Element %s was declared EMPTY this one has content\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*(*state).node).name,
                                            0 as *const xmlChar,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        }
                    }
                }
                4 => {
                    current_block = 13693488741914411839;
                    match current_block {
                        3336000707029410388 => { ret = 0 as std::os::raw::c_int }
                        13693488741914411839 => {
                            if len > 0 as std::os::raw::c_int {
                                let mut i: std::os::raw::c_int = 0;
                                i = 0 as std::os::raw::c_int;
                                while i < len {
                                    if !(*data.offset(i as isize) as
                                             std::os::raw::c_int ==
                                             0x20 as std::os::raw::c_int ||
                                             0x9 as std::os::raw::c_int <=
                                                 *data.offset(i as isize) as
                                                     std::os::raw::c_int &&
                                                 *data.offset(i as isize) as
                                                     std::os::raw::c_int <=
                                                     0xa as std::os::raw::c_int ||
                                             *data.offset(i as isize) as
                                                 std::os::raw::c_int ==
                                                 0xd as std::os::raw::c_int) {
                                        xmlErrValidNode(ctxt, (*state).node,
                                                        XML_DTD_CONTENT_MODEL,
                                                        b"Element %s content does not follow the DTD, Text not allowed\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        (*(*state).node).name,
                                                        0 as *const xmlChar,
                                                        0 as *const xmlChar);
                                        ret = 0 as std::os::raw::c_int;
                                        break ;
                                    } else { i += 1 }
                                }
                            }
                        }
                        _ => {
                            xmlErrValidNode(ctxt, (*state).node,
                                            XML_DTD_NOT_EMPTY,
                                            b"Element %s was declared EMPTY this one has content\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*(*state).node).name,
                                            0 as *const xmlChar,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        }
                    }
                }
                2 | 3 | _ => { }
            }
        }
    }
    return ret;
}
/*
 * Validation based on the regexp support
 */
/* *
 * xmlValidatePopElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 * @qname:  the qualified name as appearing in the serialization
 *
 * Pop the element end from the validation stack.
 *
 * returns 1 if no validation problem was found or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidatePopElement(mut ctxt: xmlValidCtxtPtr,
                                               mut doc: xmlDocPtr,
                                               mut elem: xmlNodePtr,
                                               mut qname: *const xmlChar)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    /* printf("PopElem %s\n", qname); */
    if (*ctxt).vstateNr > 0 as std::os::raw::c_int && !(*ctxt).vstate.is_null() {
        let mut state: xmlValidStatePtr = (*ctxt).vstate;
        let mut elemDecl: xmlElementPtr = 0 as *mut xmlElement;
        /*
	 * Check the new element against the content model of the new elem.
	 */
        if !(*state).elemDecl.is_null() {
            elemDecl = (*state).elemDecl;
            if (*elemDecl).etype as std::os::raw::c_uint ==
                   XML_ELEMENT_TYPE_ELEMENT as std::os::raw::c_int as std::os::raw::c_uint {
                if !(*state).exec.is_null() {
                    ret =
                        xmlRegExecPushString((*state).exec,
                                             0 as *const xmlChar,
                                             0 as *mut std::os::raw::c_void);
                    if ret == 0 as std::os::raw::c_int {
                        xmlErrValidNode(ctxt, (*state).node,
                                        XML_DTD_CONTENT_MODEL,
                                        b"Element %s content does not follow the DTD, Expecting more child\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char,
                                        (*(*state).node).name,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar);
                    } else {
                        /*
			 * previous validation errors should not generate
			 * a new one here
			 */
                        ret = 1 as std::os::raw::c_int
                    }
                }
            }
        }
        vstateVPop(ctxt);
    }
    return ret;
}
/* LIBXML_REGEXP_ENABLED */
/* *
 * xmlValidateOneElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 *
 * Try to validate a single element and it's attributes,
 * basically it does the following checks as described by the
 * XML-1.0 recommendation:
 *  - [ VC: Element Valid ]
 *  - [ VC: Required Attribute ]
 * Then call xmlValidateOneAttribute() for each attribute present.
 *
 * The ID/IDREF checkings are done separately
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateOneElement(mut ctxt: xmlValidCtxtPtr,
                                               mut doc: xmlDocPtr,
                                               mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut elemDecl: xmlElementPtr = 0 as xmlElementPtr;
    let mut cont: xmlElementContentPtr = 0 as *mut xmlElementContent;
    let mut attr: xmlAttributePtr = 0 as *mut xmlAttribute;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut tmp: std::os::raw::c_int = 0;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut extsubset: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if doc.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
            return 0 as std::os::raw::c_int
        }
    }
    if elem.is_null() { return 0 as std::os::raw::c_int }
    match (*elem).type_0 as std::os::raw::c_uint {
        2 => {
            xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                            b"Attribute element not expected\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const xmlChar, 0 as *const xmlChar,
                            0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
        3 => {
            if !(*elem).children.is_null() {
                xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                                b"Text element has children !\n\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as *const xmlChar);
                return 0 as std::os::raw::c_int
            }
            if !(*elem).ns.is_null() {
                xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                                b"Text element has namespace !\n\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as *const xmlChar);
                return 0 as std::os::raw::c_int
            }
            if (*elem).content.is_null() {
                xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                                b"Text element has no content !\n\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as *const xmlChar);
                return 0 as std::os::raw::c_int
            }
            return 1 as std::os::raw::c_int
        }
        19 | 20 => { return 1 as std::os::raw::c_int }
        4 | 5 | 7 | 8 => { return 1 as std::os::raw::c_int }
        6 => {
            xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                            b"Entity element not expected\n\x00" as *const u8
                                as *const std::os::raw::c_char, 0 as *const xmlChar,
                            0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
        12 => {
            xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                            b"Notation element not expected\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const xmlChar, 0 as *const xmlChar,
                            0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
        9 | 10 | 11 => {
            xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                            b"Document element not expected\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            0 as *const xmlChar, 0 as *const xmlChar,
                            0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
        13 => {
            xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                            b"HTML Document not expected\n\x00" as *const u8
                                as *const std::os::raw::c_char, 0 as *const xmlChar,
                            0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
        1 => { }
        _ => {
            xmlErrValidNode(ctxt, elem, XML_ERR_INTERNAL_ERROR,
                            b"unknown element type\n\x00" as *const u8 as
                                *const std::os::raw::c_char, 0 as *const xmlChar,
                            0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as std::os::raw::c_int
        }
    }
    /*
     * Fetch the declaration
     */
    elemDecl = xmlValidGetElemDecl(ctxt, doc, elem, &mut extsubset);
    if elemDecl.is_null() { return 0 as std::os::raw::c_int }
    /*
     * If vstateNr is not zero that means continuous validation is
     * activated, do not try to check the content model at that level.
     */
    if (*ctxt).vstateNr == 0 as std::os::raw::c_int { /* not continuous */
        /* Check that the element content matches the definition */
        match (*elemDecl).etype as std::os::raw::c_uint {
            0 => {
                xmlErrValidNode(ctxt, elem, XML_DTD_UNKNOWN_ELEM,
                                b"No declaration for element %s\n\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                (*elem).name, 0 as *const xmlChar,
                                0 as *const xmlChar);
                return 0 as std::os::raw::c_int
            }
            1 => {
                if !(*elem).children.is_null() {
                    xmlErrValidNode(ctxt, elem, XML_DTD_NOT_EMPTY,
                                    b"Element %s was declared EMPTY this one has content\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    (*elem).name, 0 as *const xmlChar,
                                    0 as *const xmlChar);
                    ret = 0 as std::os::raw::c_int
                }
            }
            3 => {
                /* simple case of declared as #PCDATA */
                if !(*elemDecl).content.is_null() &&
                       (*(*elemDecl).content).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_CONTENT_PCDATA as std::os::raw::c_int as
                               std::os::raw::c_uint {
                    ret = xmlValidateOneCdataElement(ctxt, doc, elem);
                    if ret == 0 {
                        xmlErrValidNode(ctxt, elem, XML_DTD_NOT_PCDATA,
                                        b"Element %s was declared #PCDATA but contains non text nodes\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char, (*elem).name,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar);
                    }
                } else {
                    child = (*elem).children;
                    /* Hum, this start to get messy */
                    while !child.is_null() {
                        let mut current_block_66: u64;
                        if (*child).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                           {
                            name = (*child).name;
                            if !(*child).ns.is_null() &&
                                   !(*(*child).ns).prefix.is_null() {
                                let mut fn_0: [xmlChar; 50] = [0; 50];
                                let mut fullname: *mut xmlChar =
                                    0 as *mut xmlChar;
                                fullname =
                                    xmlBuildQName((*child).name,
                                                  (*(*child).ns).prefix,
                                                  fn_0.as_mut_ptr(),
                                                  50 as std::os::raw::c_int);
                                if fullname.is_null() {
                                    return 0 as std::os::raw::c_int
                                }
                                cont = (*elemDecl).content;
                                while !cont.is_null() {
                                    if (*cont).type_0 as std::os::raw::c_uint ==
                                           XML_ELEMENT_CONTENT_ELEMENT as
                                               std::os::raw::c_int as std::os::raw::c_uint {
                                        if xmlStrEqual((*cont).name, fullname)
                                               != 0 {
                                            break ;
                                        }
                                    } else if (*cont).type_0 as std::os::raw::c_uint
                                                  ==
                                                  XML_ELEMENT_CONTENT_OR as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint &&
                                                  !(*cont).c1.is_null() &&
                                                  (*(*cont).c1).type_0 as
                                                      std::os::raw::c_uint ==
                                                      XML_ELEMENT_CONTENT_ELEMENT
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint {
                                        if xmlStrEqual((*(*cont).c1).name,
                                                       fullname) != 0 {
                                            break ;
                                        }
                                    } else if (*cont).type_0 as std::os::raw::c_uint
                                                  !=
                                                  XML_ELEMENT_CONTENT_OR as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint ||
                                                  (*cont).c1.is_null() ||
                                                  (*(*cont).c1).type_0 as
                                                      std::os::raw::c_uint !=
                                                      XML_ELEMENT_CONTENT_PCDATA
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint {
                                        xmlErrValid(0 as xmlValidCtxtPtr,
                                                    XML_DTD_MIXED_CORRUPT,
                                                    b"Internal: MIXED struct corrupted\n\x00"
                                                        as *const u8 as
                                                        *const std::os::raw::c_char,
                                                    0 as *const std::os::raw::c_char);
                                        break ;
                                    }
                                    cont = (*cont).c2
                                }
                                if fullname != fn_0.as_mut_ptr() &&
                                       fullname !=
                                           (*child).name as *mut xmlChar {
                                    xmlFree.expect("non-null function pointer")(fullname
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                if !cont.is_null() {
                                    current_block_66 = 11990767520647371493;
                                } else {
                                    current_block_66 = 17020603795727957434;
                                }
                            } else {
                                current_block_66 = 17020603795727957434;
                            }
                            match current_block_66 {
                                11990767520647371493 => { }
                                _ => {
                                    cont = (*elemDecl).content;
                                    while !cont.is_null() {
                                        if (*cont).type_0 as std::os::raw::c_uint ==
                                               XML_ELEMENT_CONTENT_ELEMENT as
                                                   std::os::raw::c_int as std::os::raw::c_uint
                                           {
                                            if xmlStrEqual((*cont).name, name)
                                                   != 0 {
                                                break ;
                                            }
                                        } else if (*cont).type_0 as
                                                      std::os::raw::c_uint ==
                                                      XML_ELEMENT_CONTENT_OR
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint &&
                                                      !(*cont).c1.is_null() &&
                                                      (*(*cont).c1).type_0 as
                                                          std::os::raw::c_uint ==
                                                          XML_ELEMENT_CONTENT_ELEMENT
                                                              as std::os::raw::c_int
                                                              as std::os::raw::c_uint
                                         {
                                            if xmlStrEqual((*(*cont).c1).name,
                                                           name) != 0 {
                                                break ;
                                            }
                                        } else if (*cont).type_0 as
                                                      std::os::raw::c_uint !=
                                                      XML_ELEMENT_CONTENT_OR
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint ||
                                                      (*cont).c1.is_null() ||
                                                      (*(*cont).c1).type_0 as
                                                          std::os::raw::c_uint !=
                                                          XML_ELEMENT_CONTENT_PCDATA
                                                              as std::os::raw::c_int
                                                              as std::os::raw::c_uint
                                         {
                                            xmlErrValid(ctxt,
                                                        XML_DTD_MIXED_CORRUPT,
                                                        b"Internal: MIXED struct corrupted\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        0 as
                                                            *const std::os::raw::c_char);
                                            break ;
                                        }
                                        cont = (*cont).c2
                                    }
                                    if cont.is_null() {
                                        xmlErrValidNode(ctxt, elem,
                                                        XML_DTD_INVALID_CHILD,
                                                        b"Element %s is not declared in %s list of possible children\n\x00"
                                                            as *const u8 as
                                                            *const std::os::raw::c_char,
                                                        name, (*elem).name,
                                                        0 as *const xmlChar);
                                        ret = 0 as std::os::raw::c_int
                                    }
                                }
                            }
                        }
                        child = (*child).next
                    }
                }
            }
            4 => {
                if (*doc).standalone == 1 as std::os::raw::c_int &&
                       extsubset == 1 as std::os::raw::c_int {
                    /*
		 * VC: Standalone Document Declaration
		 *     - element types with element content, if white space
		 *       occurs directly within any instance of those types.
		 */
                    child = (*elem).children;
                    while !child.is_null() {
                        if (*child).type_0 as std::os::raw::c_uint ==
                               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                            let mut content: *const xmlChar =
                                (*child).content;
                            while *content as std::os::raw::c_int ==
                                      0x20 as std::os::raw::c_int ||
                                      0x9 as std::os::raw::c_int <=
                                          *content as std::os::raw::c_int &&
                                          *content as std::os::raw::c_int <=
                                              0xa as std::os::raw::c_int ||
                                      *content as std::os::raw::c_int ==
                                          0xd as std::os::raw::c_int {
                                content = content.offset(1)
                            }
                            if *content as std::os::raw::c_int == 0 as std::os::raw::c_int {
                                xmlErrValidNode(ctxt, elem,
                                                XML_DTD_STANDALONE_WHITE_SPACE,
                                                b"standalone: %s declared in the external subset contains white spaces nodes\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char,
                                                (*elem).name,
                                                0 as *const xmlChar,
                                                0 as *const xmlChar);
                                ret = 0 as std::os::raw::c_int;
                                break ;
                            }
                        }
                        child = (*child).next
                    }
                }
                child = (*elem).children;
                cont = (*elemDecl).content;
                tmp =
                    xmlValidateElementContent(ctxt, child, elemDecl,
                                              1 as std::os::raw::c_int, elem);
                if tmp <= 0 as std::os::raw::c_int { ret = tmp }
            }
            2 | _ => { }
        }
    }
    /* [ VC: Required Attribute ] */
    attr = (*elemDecl).attributes;
    while !attr.is_null() {
        if (*attr).def as std::os::raw::c_uint ==
               XML_ATTRIBUTE_REQUIRED as std::os::raw::c_int as std::os::raw::c_uint {
            let mut qualified: std::os::raw::c_int = -(1 as std::os::raw::c_int);
            if (*attr).prefix.is_null() &&
                   xmlStrEqual((*attr).name,
                               b"xmlns\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
                let mut ns: xmlNsPtr = 0 as *mut xmlNs;
                ns = (*elem).nsDef;
                loop  {
                    if ns.is_null() {
                        current_block = 14184516523743666873;
                        break ;
                    }
                    if (*ns).prefix.is_null() {
                        current_block = 11691335137785802966;
                        break ;
                    }
                    ns = (*ns).next
                }
            } else if xmlStrEqual((*attr).prefix,
                                  b"xmlns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                let mut ns_0: xmlNsPtr = 0 as *mut xmlNs;
                ns_0 = (*elem).nsDef;
                loop  {
                    if ns_0.is_null() {
                        current_block = 14184516523743666873;
                        break ;
                    }
                    if xmlStrEqual((*attr).name, (*ns_0).prefix) != 0 {
                        current_block = 11691335137785802966;
                        break ;
                    }
                    ns_0 = (*ns_0).next
                }
            } else {
                let mut attrib: xmlAttrPtr = 0 as *mut xmlAttr;
                attrib = (*elem).properties;
                loop  {
                    if attrib.is_null() {
                        current_block = 14184516523743666873;
                        break ;
                    }
                    if xmlStrEqual((*attrib).name, (*attr).name) != 0 {
                        if (*attr).prefix.is_null() {
                            current_block = 11691335137785802966;
                            break ;
                        }
                        let mut nameSpace: xmlNsPtr = (*attrib).ns;
                        if nameSpace.is_null() { nameSpace = (*elem).ns }
                        /*
			     * qualified names handling is problematic, having a
			     * different prefix should be possible but DTDs don't
			     * allow to define the URI instead of the prefix :-(
			     */
                        if nameSpace.is_null() {
                            if qualified < 0 as std::os::raw::c_int {
                                qualified = 0 as std::os::raw::c_int
                            }
                        } else {
                            if !(xmlStrEqual((*nameSpace).prefix,
                                             (*attr).prefix) == 0) {
                                current_block = 11691335137785802966;
                                break ;
                            }
                            if qualified < 1 as std::os::raw::c_int {
                                qualified = 1 as std::os::raw::c_int
                            }
                        }
                    }
                    attrib = (*attrib).next
                }
            }
            match current_block {
                11691335137785802966 => { }
                _ => {
                    if qualified == -(1 as std::os::raw::c_int) {
                        if (*attr).prefix.is_null() {
                            xmlErrValidNode(ctxt, elem,
                                            XML_DTD_MISSING_ATTRIBUTE,
                                            b"Element %s does not carry attribute %s\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*elem).name, (*attr).name,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        } else {
                            xmlErrValidNode(ctxt, elem,
                                            XML_DTD_MISSING_ATTRIBUTE,
                                            b"Element %s does not carry attribute %s:%s\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*elem).name, (*attr).prefix,
                                            (*attr).name);
                            ret = 0 as std::os::raw::c_int
                        }
                    } else if qualified == 0 as std::os::raw::c_int {
                        xmlErrValidWarning(ctxt, elem, XML_DTD_NO_PREFIX,
                                           b"Element %s required attribute %s:%s has no prefix\n\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char,
                                           (*elem).name, (*attr).prefix,
                                           (*attr).name);
                    } else if qualified == 1 as std::os::raw::c_int {
                        xmlErrValidWarning(ctxt, elem,
                                           XML_DTD_DIFFERENT_PREFIX,
                                           b"Element %s required attribute %s:%s has different prefix\n\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char,
                                           (*elem).name, (*attr).prefix,
                                           (*attr).name);
                    }
                }
            }
        } else if (*attr).def as std::os::raw::c_uint ==
                      XML_ATTRIBUTE_FIXED as std::os::raw::c_int as std::os::raw::c_uint {
            /*
	     * Special tests checking #FIXED namespace declarations
	     * have the right value since this is not done as an
	     * attribute checking
	     */
            if (*attr).prefix.is_null() &&
                   xmlStrEqual((*attr).name,
                               b"xmlns\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
                let mut ns_1: xmlNsPtr = 0 as *mut xmlNs;
                ns_1 = (*elem).nsDef;
                while !ns_1.is_null() {
                    if (*ns_1).prefix.is_null() {
                        if xmlStrEqual((*attr).defaultValue, (*ns_1).href) ==
                               0 {
                            xmlErrValidNode(ctxt, elem,
                                            XML_DTD_ELEM_DEFAULT_NAMESPACE,
                                            b"Element %s namespace name for default namespace does not match the DTD\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*elem).name, 0 as *const xmlChar,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        }
                        break ;
                    } else { ns_1 = (*ns_1).next }
                }
            } else if xmlStrEqual((*attr).prefix,
                                  b"xmlns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                let mut ns_2: xmlNsPtr = 0 as *mut xmlNs;
                ns_2 = (*elem).nsDef;
                while !ns_2.is_null() {
                    if xmlStrEqual((*attr).name, (*ns_2).prefix) != 0 {
                        if xmlStrEqual((*attr).defaultValue, (*ns_2).href) ==
                               0 {
                            xmlErrValidNode(ctxt, elem,
                                            XML_DTD_ELEM_NAMESPACE,
                                            b"Element %s namespace name for %s does not match the DTD\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char,
                                            (*elem).name, (*ns_2).prefix,
                                            0 as *const xmlChar);
                            ret = 0 as std::os::raw::c_int
                        }
                        break ;
                    } else { ns_2 = (*ns_2).next }
                }
            }
        }
        /*
			     * We should allow applications to define namespaces
			     * for their application even if the DTD doesn't
			     * carry one, otherwise, basically we would always
			     * break.
			     */
        attr = (*attr).nexth
    }
    return ret;
}
/* *
 * xmlValidateRoot:
 * @ctxt:  the validation context
 * @doc:  a document instance
 *
 * Try to validate a the root element
 * basically it does the following check as described by the
 * XML-1.0 recommendation:
 *  - [ VC: Root Element Type ]
 * it doesn't try to recurse or apply other check to the element
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateRoot(mut ctxt: xmlValidCtxtPtr,
                                         mut doc: xmlDocPtr) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0;
    if doc.is_null() { return 0 as std::os::raw::c_int }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() || (*root).name.is_null() {
        xmlErrValid(ctxt, XML_DTD_NO_ROOT,
                    b"no root element\n\x00" as *const u8 as
                        *const std::os::raw::c_char, 0 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    /*
     * When doing post validation against a separate DTD, those may
     * no internal subset has been generated
     */
    if !(*doc).intSubset.is_null() && !(*(*doc).intSubset).name.is_null() {
        /*
	 * Check first the document root against the NQName
	 */
        if xmlStrEqual((*(*doc).intSubset).name, (*root).name) == 0 {
            if !(*root).ns.is_null() && !(*(*root).ns).prefix.is_null() {
                let mut fn_0: [xmlChar; 50] = [0; 50];
                let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
                fullname =
                    xmlBuildQName((*root).name, (*(*root).ns).prefix,
                                  fn_0.as_mut_ptr(), 50 as std::os::raw::c_int);
                if fullname.is_null() {
                    xmlVErrMemory(ctxt, 0 as *const std::os::raw::c_char);
                    return 0 as std::os::raw::c_int
                }
                ret = xmlStrEqual((*(*doc).intSubset).name, fullname);
                if fullname != fn_0.as_mut_ptr() &&
                       fullname != (*root).name as *mut xmlChar {
                    xmlFree.expect("non-null function pointer")(fullname as
                                                                    *mut std::os::raw::c_void);
                }
                if ret == 1 as std::os::raw::c_int {
                    current_block = 10606345926109051596;
                } else { current_block = 6057473163062296781; }
            } else { current_block = 6057473163062296781; }
            match current_block {
                10606345926109051596 => { }
                _ => {
                    if !(xmlStrEqual((*(*doc).intSubset).name,
                                     b"HTML\x00" as *const u8 as
                                         *const std::os::raw::c_char as *mut xmlChar)
                             != 0 &&
                             xmlStrEqual((*root).name,
                                         b"html\x00" as *const u8 as
                                             *const std::os::raw::c_char as
                                             *mut xmlChar) != 0) {
                        xmlErrValidNode(ctxt, root, XML_DTD_ROOT_NAME,
                                        b"root and DTD name do not match \'%s\' and \'%s\'\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char, (*root).name,
                                        (*(*doc).intSubset).name,
                                        0 as *const xmlChar);
                        return 0 as std::os::raw::c_int
                    }
                }
            }
        }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 *
 * Try to validate the subtree under an element
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateElement(mut ctxt: xmlValidCtxtPtr,
                                            mut doc: xmlDocPtr,
                                            mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if elem.is_null() { return 0 as std::os::raw::c_int }
    /*
     * XInclude elements were added after parsing in the infoset,
     * they don't really mean anything validation wise.
     */
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
           (*elem).type_0 as std::os::raw::c_uint ==
               XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint ||
           (*elem).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    }
    if doc.is_null() {
        return 0 as std::os::raw::c_int
    } else {
        if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
            return 0 as std::os::raw::c_int
        }
    }
    /*
     * Entities references have to be handled separately
     */
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_ENTITY_REF_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    }
    ret &= xmlValidateOneElement(ctxt, doc, elem);
    if (*elem).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        attr = (*elem).properties;
        while !attr.is_null() {
            value =
                xmlNodeListGetString(doc, (*attr).children, 0 as std::os::raw::c_int);
            ret &= xmlValidateOneAttribute(ctxt, doc, elem, attr, value);
            if !value.is_null() {
                xmlFree.expect("non-null function pointer")(value as
                                                                *mut std::os::raw::c_char
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            attr = (*attr).next
        }
        ns = (*elem).nsDef;
        while !ns.is_null() {
            if (*elem).ns.is_null() {
                ret &=
                    xmlValidateOneNamespace(ctxt, doc, elem,
                                            0 as *const xmlChar, ns,
                                            (*ns).href)
            } else {
                ret &=
                    xmlValidateOneNamespace(ctxt, doc, elem,
                                            (*(*elem).ns).prefix, ns,
                                            (*ns).href)
            }
            ns = (*ns).next
        }
    }
    child = (*elem).children;
    while !child.is_null() {
        ret &= xmlValidateElement(ctxt, doc, child);
        child = (*child).next
    }
    return ret;
}
/* *
 * xmlValidateRef:
 * @ref:   A reference to be validated
 * @ctxt:  Validation context
 * @name:  Name of ID we are searching for
 *
 */
unsafe extern "C" fn xmlValidateRef(mut ref_0: xmlRefPtr,
                                    mut ctxt: xmlValidCtxtPtr,
                                    mut name: *const xmlChar) {
    let mut id: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    if ref_0.is_null() { return }
    if (*ref_0).attr.is_null() && (*ref_0).name.is_null() { return }
    attr = (*ref_0).attr;
    if attr.is_null() {
        let mut dup: *mut xmlChar = 0 as *mut xmlChar;
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
        let mut cur: *mut xmlChar = 0 as *mut xmlChar;
        let mut save: xmlChar = 0;
        dup = xmlStrdup(name);
        if dup.is_null() { (*ctxt).valid = 0 as std::os::raw::c_int; return }
        cur = dup;
        while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
            str = cur;
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                cur = cur.offset(1)
            }
            save = *cur;
            *cur = 0 as std::os::raw::c_int as xmlChar;
            id = xmlGetID((*ctxt).doc, str);
            if id.is_null() {
                xmlErrValidNodeNr(ctxt, 0 as xmlNodePtr, XML_DTD_UNKNOWN_ID,
                                  b"attribute %s line %d references an unknown ID \"%s\"\n\x00"
                                      as *const u8 as *const std::os::raw::c_char,
                                  (*ref_0).name, (*ref_0).lineno, str);
                (*ctxt).valid = 0 as std::os::raw::c_int
            }
            if save as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
            *cur = save;
            while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur = cur.offset(1)
            }
        }
        xmlFree.expect("non-null function pointer")(dup as *mut std::os::raw::c_void);
    } else if (*attr).atype as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_IDREF as std::os::raw::c_int as std::os::raw::c_uint {
        id = xmlGetID((*ctxt).doc, name);
        if id.is_null() {
            xmlErrValidNode(ctxt, (*attr).parent, XML_DTD_UNKNOWN_ID,
                            b"IDREF attribute %s references an unknown ID \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*attr).name, name, 0 as *const xmlChar);
            (*ctxt).valid = 0 as std::os::raw::c_int
        }
    } else if (*attr).atype as std::os::raw::c_uint ==
                  XML_ATTRIBUTE_IDREFS as std::os::raw::c_int as std::os::raw::c_uint {
        let mut dup_0: *mut xmlChar = 0 as *mut xmlChar;
        let mut str_0: *mut xmlChar = 0 as *mut xmlChar;
        let mut cur_0: *mut xmlChar = 0 as *mut xmlChar;
        let mut save_0: xmlChar = 0;
        dup_0 = xmlStrdup(name);
        if dup_0.is_null() {
            xmlVErrMemory(ctxt,
                          b"IDREFS split\x00" as *const u8 as
                              *const std::os::raw::c_char);
            (*ctxt).valid = 0 as std::os::raw::c_int;
            return
        }
        cur_0 = dup_0;
        while *cur_0 as std::os::raw::c_int != 0 as std::os::raw::c_int {
            str_0 = cur_0;
            while *cur_0 as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      !(*cur_0 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur_0 as std::os::raw::c_int &&
                                *cur_0 as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur_0 as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                cur_0 = cur_0.offset(1)
            }
            save_0 = *cur_0;
            *cur_0 = 0 as std::os::raw::c_int as xmlChar;
            id = xmlGetID((*ctxt).doc, str_0);
            if id.is_null() {
                xmlErrValidNode(ctxt, (*attr).parent, XML_DTD_UNKNOWN_ID,
                                b"IDREFS attribute %s references an unknown ID \"%s\"\n\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                (*attr).name, str_0, 0 as *const xmlChar);
                (*ctxt).valid = 0 as std::os::raw::c_int
            }
            if save_0 as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
            *cur_0 = save_0;
            while *cur_0 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur_0 as std::os::raw::c_int &&
                          *cur_0 as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur_0 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur_0 = cur_0.offset(1)
            }
        }
        xmlFree.expect("non-null function pointer")(dup_0 as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlWalkValidateList:
 * @data:  Contents of current link
 * @user:  Value supplied by the user
 *
 * Returns 0 to abort the walk or 1 to continue
 */
unsafe extern "C" fn xmlWalkValidateList(mut data: *const std::os::raw::c_void,
                                         mut user: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut memo: xmlValidateMemoPtr = user as xmlValidateMemoPtr;
    xmlValidateRef(data as xmlRefPtr, (*memo).ctxt, (*memo).name);
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlValidateCheckRefCallback:
 * @ref_list:  List of references
 * @ctxt:  Validation context
 * @name:  Name of ID we are searching for
 *
 */
unsafe extern "C" fn xmlValidateCheckRefCallback(mut payload:
                                                     *mut std::os::raw::c_void,
                                                 mut data: *mut std::os::raw::c_void,
                                                 mut name: *const xmlChar) {
    let mut ref_list: xmlListPtr = payload as xmlListPtr;
    let mut ctxt: xmlValidCtxtPtr = data as xmlValidCtxtPtr;
    let mut memo: xmlValidateMemo =
        xmlValidateMemo{ctxt: 0 as *mut xmlValidCtxt,
                        name: 0 as *const xmlChar,};
    if ref_list.is_null() { return }
    memo.ctxt = ctxt;
    memo.name = name;
    xmlListWalk(ref_list,
                Some(xmlWalkValidateList as
                         unsafe extern "C" fn(_: *const std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void)
                             -> std::os::raw::c_int),
                &mut memo as *mut xmlValidateMemo as *mut std::os::raw::c_void);
}
/* *
 * xmlValidateDocumentFinal:
 * @ctxt:  the validation context
 * @doc:  a document instance
 *
 * Does the final step for the document validation once all the
 * incremental validation steps have been completed
 *
 * basically it does the following checks described by the XML Rec
 *
 * Check all the IDREF/IDREFS attributes definition for validity
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDocumentFinal(mut ctxt: xmlValidCtxtPtr,
                                                  mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut table: xmlRefTablePtr = 0 as *mut xmlRefTable;
    let mut save: std::os::raw::c_uint = 0;
    if ctxt.is_null() { return 0 as std::os::raw::c_int }
    if doc.is_null() {
        xmlErrValid(ctxt, XML_DTD_NO_DOC,
                    b"xmlValidateDocumentFinal: doc == NULL\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    /* trick to get correct line id report */
    save = (*ctxt).finishDtd;
    (*ctxt).finishDtd = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    /*
     * Check all the NOTATION/NOTATIONS attributes
     */
    /*
     * Check all the ENTITY/ENTITIES attributes definition for validity
     */
    /*
     * Check all the IDREF/IDREFS attributes definition for validity
     */
    table = (*doc).refs as xmlRefTablePtr;
    (*ctxt).doc = doc;
    (*ctxt).valid = 1 as std::os::raw::c_int;
    xmlHashScan(table,
                Some(xmlValidateCheckRefCallback as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()),
                ctxt as *mut std::os::raw::c_void);
    (*ctxt).finishDtd = save;
    return (*ctxt).valid;
}
/* *
 * xmlValidateDtd:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @dtd:  a dtd instance
 *
 * Try to validate the document against the dtd instance
 *
 * Basically it does check all the definitions in the DtD.
 * Note the the internal subset (if present) is de-coupled
 * (i.e. not used), which could give problems if ID or IDREF
 * is present.
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDtd(mut ctxt: xmlValidCtxtPtr,
                                        mut doc: xmlDocPtr,
                                        mut dtd: xmlDtdPtr) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut oldExt: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut oldInt: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    if dtd.is_null() { return 0 as std::os::raw::c_int }
    if doc.is_null() { return 0 as std::os::raw::c_int }
    oldExt = (*doc).extSubset;
    oldInt = (*doc).intSubset;
    (*doc).extSubset = dtd;
    (*doc).intSubset = 0 as *mut _xmlDtd;
    ret = xmlValidateRoot(ctxt, doc);
    if ret == 0 as std::os::raw::c_int {
        (*doc).extSubset = oldExt;
        (*doc).intSubset = oldInt;
        return ret
    }
    if !(*doc).ids.is_null() {
        xmlFreeIDTable((*doc).ids as xmlIDTablePtr);
        (*doc).ids = 0 as *mut std::os::raw::c_void
    }
    if !(*doc).refs.is_null() {
        xmlFreeRefTable((*doc).refs as xmlRefTablePtr);
        (*doc).refs = 0 as *mut std::os::raw::c_void
    }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    ret = xmlValidateElement(ctxt, doc, root);
    ret &= xmlValidateDocumentFinal(ctxt, doc);
    (*doc).extSubset = oldExt;
    (*doc).intSubset = oldInt;
    return ret;
}
unsafe extern "C" fn xmlValidateNotationCallback(mut payload:
                                                     *mut std::os::raw::c_void,
                                                 mut data: *mut std::os::raw::c_void,
                                                 mut name: *const xmlChar) {
    let mut cur: xmlEntityPtr = payload as xmlEntityPtr;
    let mut ctxt: xmlValidCtxtPtr = data as xmlValidCtxtPtr;
    if cur.is_null() { return }
    if (*cur).etype as std::os::raw::c_uint ==
           XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as std::os::raw::c_int as std::os::raw::c_uint
       {
        let mut notation: *mut xmlChar = (*cur).content;
        if !notation.is_null() {
            let mut ret: std::os::raw::c_int = 0;
            ret = xmlValidateNotationUse(ctxt, (*cur).doc, notation);
            if ret != 1 as std::os::raw::c_int { (*ctxt).valid = 0 as std::os::raw::c_int }
        }
    };
}
unsafe extern "C" fn xmlValidateAttributeCallback(mut payload:
                                                      *mut std::os::raw::c_void,
                                                  mut data: *mut std::os::raw::c_void,
                                                  mut name: *const xmlChar) {
    let mut cur: xmlAttributePtr = payload as xmlAttributePtr;
    let mut ctxt: xmlValidCtxtPtr = data as xmlValidCtxtPtr;
    let mut ret: std::os::raw::c_int = 0;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut elem: xmlElementPtr = 0 as xmlElementPtr;
    if cur.is_null() { return }
    match (*cur).atype as std::os::raw::c_uint {
        5 | 6 | 10 => {
            if !(*cur).defaultValue.is_null() {
                ret =
                    xmlValidateAttributeValue2(ctxt, (*ctxt).doc, (*cur).name,
                                               (*cur).atype,
                                               (*cur).defaultValue);
                if ret == 0 as std::os::raw::c_int &&
                       (*ctxt).valid == 1 as std::os::raw::c_int {
                    (*ctxt).valid = 0 as std::os::raw::c_int
                }
            }
            if !(*cur).tree.is_null() {
                let mut tree: xmlEnumerationPtr = (*cur).tree;
                while !tree.is_null() {
                    ret =
                        xmlValidateAttributeValue2(ctxt, (*ctxt).doc,
                                                   (*cur).name, (*cur).atype,
                                                   (*tree).name);
                    if ret == 0 as std::os::raw::c_int &&
                           (*ctxt).valid == 1 as std::os::raw::c_int {
                        (*ctxt).valid = 0 as std::os::raw::c_int
                    }
                    tree = (*tree).next
                }
            }
        }
        1 | 2 | 3 | 4 | 7 | 8 | 9 | _ => { }
    }
    if (*cur).atype as std::os::raw::c_uint ==
           XML_ATTRIBUTE_NOTATION as std::os::raw::c_int as std::os::raw::c_uint {
        doc = (*cur).doc;
        if (*cur).elem.is_null() {
            xmlErrValid(ctxt, XML_ERR_INTERNAL_ERROR,
                        b"xmlValidateAttributeCallback(%s): internal error\n\x00"
                            as *const u8 as *const std::os::raw::c_char,
                        (*cur).name as *const std::os::raw::c_char);
            return
        }
        if !doc.is_null() {
            elem = xmlGetDtdElementDesc((*doc).intSubset, (*cur).elem)
        }
        if elem.is_null() && !doc.is_null() {
            elem = xmlGetDtdElementDesc((*doc).extSubset, (*cur).elem)
        }
        if elem.is_null() && !(*cur).parent.is_null() &&
               (*(*cur).parent).type_0 as std::os::raw::c_uint ==
                   XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            elem =
                xmlGetDtdElementDesc((*cur).parent as xmlDtdPtr, (*cur).elem)
        }
        if elem.is_null() {
            xmlErrValidNode(ctxt, 0 as xmlNodePtr, XML_DTD_UNKNOWN_ELEM,
                            b"attribute %s: could not find decl for element %s\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*cur).name, (*cur).elem, 0 as *const xmlChar);
            return
        }
        if (*elem).etype as std::os::raw::c_uint ==
               XML_ELEMENT_TYPE_EMPTY as std::os::raw::c_int as std::os::raw::c_uint {
            xmlErrValidNode(ctxt, 0 as xmlNodePtr, XML_DTD_EMPTY_NOTATION,
                            b"NOTATION attribute %s declared for EMPTY element %s\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*cur).name, (*cur).elem, 0 as *const xmlChar);
            (*ctxt).valid = 0 as std::os::raw::c_int
        }
    };
}
/* *
 * xmlValidateDtdFinal:
 * @ctxt:  the validation context
 * @doc:  a document instance
 *
 * Does the final step for the dtds validation once all the
 * subsets have been parsed
 *
 * basically it does the following checks described by the XML Rec
 * - check that ENTITY and ENTITIES type attributes default or
 *   possible values matches one of the defined entities.
 * - check that NOTATION type attributes default or
 *   possible values matches one of the defined notations.
 *
 * returns 1 if valid or 0 if invalid and -1 if not well-formed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDtdFinal(mut ctxt: xmlValidCtxtPtr,
                                             mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut table: xmlAttributeTablePtr = 0 as *mut xmlAttributeTable;
    let mut entities: xmlEntitiesTablePtr = 0 as *mut xmlEntitiesTable;
    if doc.is_null() || ctxt.is_null() { return 0 as std::os::raw::c_int }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as std::os::raw::c_int
    }
    (*ctxt).doc = doc;
    (*ctxt).valid = 1 as std::os::raw::c_int;
    dtd = (*doc).intSubset;
    if !dtd.is_null() && !(*dtd).attributes.is_null() {
        table = (*dtd).attributes as xmlAttributeTablePtr;
        xmlHashScan(table,
                    Some(xmlValidateAttributeCallback as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    if !dtd.is_null() && !(*dtd).entities.is_null() {
        entities = (*dtd).entities as xmlEntitiesTablePtr;
        xmlHashScan(entities,
                    Some(xmlValidateNotationCallback as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    dtd = (*doc).extSubset;
    if !dtd.is_null() && !(*dtd).attributes.is_null() {
        table = (*dtd).attributes as xmlAttributeTablePtr;
        xmlHashScan(table,
                    Some(xmlValidateAttributeCallback as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    if !dtd.is_null() && !(*dtd).entities.is_null() {
        entities = (*dtd).entities as xmlEntitiesTablePtr;
        xmlHashScan(entities,
                    Some(xmlValidateNotationCallback as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    return (*ctxt).valid;
}
/* *
 * xmlValidateDocument:
 * @ctxt:  the validation context
 * @doc:  a document instance
 *
 * Try to validate the document instance
 *
 * basically it does the all the checks described by the XML Rec
 * i.e. validates the internal and external subset (if present)
 * and validate the document tree.
 *
 * returns 1 if valid or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDocument(mut ctxt: xmlValidCtxtPtr,
                                             mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() { return 0 as std::os::raw::c_int }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        xmlErrValid(ctxt, XML_DTD_NO_DTD,
                    b"no DTD found!\n\x00" as *const u8 as
                        *const std::os::raw::c_char, 0 as *const std::os::raw::c_char);
        return 0 as std::os::raw::c_int
    }
    if !(*doc).intSubset.is_null() &&
           (!(*(*doc).intSubset).SystemID.is_null() ||
                !(*(*doc).intSubset).ExternalID.is_null()) &&
           (*doc).extSubset.is_null() {
        let mut sysID: *mut xmlChar = 0 as *mut xmlChar;
        if !(*(*doc).intSubset).SystemID.is_null() {
            sysID = xmlBuildURI((*(*doc).intSubset).SystemID, (*doc).URL);
            if sysID.is_null() {
                xmlErrValid(ctxt, XML_DTD_LOAD_ERROR,
                            b"Could not build URI for external subset \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*(*doc).intSubset).SystemID as
                                *const std::os::raw::c_char);
                return 0 as std::os::raw::c_int
            }
        } else { sysID = 0 as *mut xmlChar }
        (*doc).extSubset =
            xmlParseDTD((*(*doc).intSubset).ExternalID,
                        sysID as *const xmlChar);
        if !sysID.is_null() {
            xmlFree.expect("non-null function pointer")(sysID as
                                                            *mut std::os::raw::c_void);
        }
        if (*doc).extSubset.is_null() {
            if !(*(*doc).intSubset).SystemID.is_null() {
                xmlErrValid(ctxt, XML_DTD_LOAD_ERROR,
                            b"Could not load the external subset \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*(*doc).intSubset).SystemID as
                                *const std::os::raw::c_char);
            } else {
                xmlErrValid(ctxt, XML_DTD_LOAD_ERROR,
                            b"Could not load the external subset \"%s\"\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*(*doc).intSubset).ExternalID as
                                *const std::os::raw::c_char);
            }
            return 0 as std::os::raw::c_int
        }
    }
    if !(*doc).ids.is_null() {
        xmlFreeIDTable((*doc).ids as xmlIDTablePtr);
        (*doc).ids = 0 as *mut std::os::raw::c_void
    }
    if !(*doc).refs.is_null() {
        xmlFreeRefTable((*doc).refs as xmlRefTablePtr);
        (*doc).refs = 0 as *mut std::os::raw::c_void
    }
    ret = xmlValidateDtdFinal(ctxt, doc);
    if xmlValidateRoot(ctxt, doc) == 0 { return 0 as std::os::raw::c_int }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    ret &= xmlValidateElement(ctxt, doc, root);
    ret &= xmlValidateDocumentFinal(ctxt, doc);
    return ret;
}
/* ***********************************************************************
 *									*
 *		Routines for dynamic validation editing			*
 *									*
 ************************************************************************/
/* *
 * xmlValidGetPotentialChildren:
 * @ctree:  an element content tree
 * @names:  an array to store the list of child names
 * @len:  a pointer to the number of element in the list
 * @max:  the size of the array
 *
 * Build/extend a list of  potential children allowed by the content tree
 *
 * returns the number of element in the list, or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidGetPotentialChildren(mut ctree:
                                                          *mut xmlElementContent,
                                                      mut names:
                                                          *mut *const xmlChar,
                                                      mut len:
                                                          *mut std::os::raw::c_int,
                                                      mut max: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if ctree.is_null() || names.is_null() || len.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if *len >= max { return *len }
    match (*ctree).type_0 as std::os::raw::c_uint {
        1 => {
            i = 0 as std::os::raw::c_int;
            while i < *len {
                if xmlStrEqual(b"#PCDATA\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar,
                               *names.offset(i as isize)) != 0 {
                    return *len
                }
                i += 1
            }
            let fresh17 = *len;
            *len = *len + 1;
            let ref mut fresh18 = *names.offset(fresh17 as isize);
            *fresh18 =
                b"#PCDATA\x00" as *const u8 as *const std::os::raw::c_char as
                    *mut xmlChar
        }
        2 => {
            i = 0 as std::os::raw::c_int;
            while i < *len {
                if xmlStrEqual((*ctree).name, *names.offset(i as isize)) != 0
                   {
                    return *len
                }
                i += 1
            }
            let fresh19 = *len;
            *len = *len + 1;
            let ref mut fresh20 = *names.offset(fresh19 as isize);
            *fresh20 = (*ctree).name
        }
        3 => {
            xmlValidGetPotentialChildren((*ctree).c1, names, len, max);
            xmlValidGetPotentialChildren((*ctree).c2, names, len, max);
        }
        4 => {
            xmlValidGetPotentialChildren((*ctree).c1, names, len, max);
            xmlValidGetPotentialChildren((*ctree).c2, names, len, max);
        }
        _ => { }
    }
    return *len;
}
/* Notation */
/* LIBXML_TREE_ENABLED */
/* LIBXML_OUTPUT_ENABLED */
/* Element Content */
/* the non Doc version are being deprecated */
/* the new versions with doc argument */
/* DEPRECATED */
/* LIBXML_OUTPUT_ENABLED */
/* DEPRECATED */
/* Element */
/* LIBXML_TREE_ENABLED */
/* LIBXML_OUTPUT_ENABLED */
/* Enumeration */
/* LIBXML_TREE_ENABLED */
/* Attribute */
/* LIBXML_TREE_ENABLED */
/* LIBXML_OUTPUT_ENABLED */
/* IDs */
/* IDREFs */
/* *
 * The public function calls related to validity checking.
 */
/* Allocate/Release Validation Contexts */
/* LIBXML_VALID_ENABLED */
/* LIBXML_VALID_ENABLED or LIBXML_SCHEMAS_ENABLED */
/*
 * Dummy function to suppress messages while we try out valid elements
 */
/* *
 * xmlValidGetValidElements:
 * @prev:  an element to insert after
 * @next:  an element to insert next
 * @names:  an array to store the list of child names
 * @max:  the size of the array
 *
 * This function returns the list of authorized children to insert
 * within an existing tree while respecting the validity constraints
 * forced by the Dtd. The insertion point is defined using @prev and
 * @next in the following ways:
 *  to insert before 'node': xmlValidGetValidElements(node->prev, node, ...
 *  to insert next 'node': xmlValidGetValidElements(node, node->next, ...
 *  to replace 'node': xmlValidGetValidElements(node->prev, node->next, ...
 *  to prepend a child to 'node': xmlValidGetValidElements(NULL, node->childs,
 *  to append a child to 'node': xmlValidGetValidElements(node->last, NULL, ...
 *
 * pointers to the element names are inserted at the beginning of the array
 * and do not need to be freed.
 *
 * returns the number of element in the list, or -1 in case of error. If
 *    the function returns the value @max the caller is invited to grow the
 *    receiving array and retry.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlValidGetValidElements(mut prev: *mut xmlNode,
                                                  mut next: *mut xmlNode,
                                                  mut names:
                                                      *mut *const xmlChar,
                                                  mut max: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut vctxt: xmlValidCtxt =
        xmlValidCtxt{userData: 0 as *mut std::os::raw::c_void,
                     error: None,
                     warning: None,
                     node: 0 as *mut xmlNode,
                     nodeNr: 0,
                     nodeMax: 0,
                     nodeTab: 0 as *mut xmlNodePtr,
                     finishDtd: 0,
                     doc: 0 as *mut xmlDoc,
                     valid: 0,
                     vstate: 0 as *mut xmlValidState,
                     vstateNr: 0,
                     vstateMax: 0,
                     vstateTab: 0 as *mut xmlValidState,
                     am: 0 as *mut xmlAutomata,
                     state:
                         0 as
                             *mut xmlAutomataState,}; /* this suppresses err/warn output */
    let mut nb_valid_elements: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut elements: [*const xmlChar; 256] =
        [0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar, 0 as *const xmlChar, 0 as *const xmlChar,
         0 as *const xmlChar];
    let mut nb_elements: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ref_node: *mut xmlNode = 0 as *mut xmlNode;
    let mut parent: *mut xmlNode = 0 as *mut xmlNode;
    let mut test_node: *mut xmlNode = 0 as *mut xmlNode;
    let mut prev_next: *mut xmlNode = 0 as *mut xmlNode;
    let mut next_prev: *mut xmlNode = 0 as *mut xmlNode;
    let mut parent_childs: *mut xmlNode = 0 as *mut xmlNode;
    let mut parent_last: *mut xmlNode = 0 as *mut xmlNode;
    let mut element_desc: *mut xmlElement = 0 as *mut xmlElement;
    if prev.is_null() && next.is_null() { return -(1 as std::os::raw::c_int) }
    if names.is_null() { return -(1 as std::os::raw::c_int) }
    if max <= 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    memset(&mut vctxt as *mut xmlValidCtxt as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlValidCtxt>() as std::os::raw::c_ulong);
    vctxt.error =
        Some(xmlNoValidityErr as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                      _: *const std::os::raw::c_char, _: ...) -> ());
    nb_valid_elements = 0 as std::os::raw::c_int;
    ref_node = if !prev.is_null() { prev } else { next };
    parent = (*ref_node).parent;
    /*
     * Retrieves the parent element declaration
     */
    element_desc =
        xmlGetDtdElementDesc((*(*parent).doc).intSubset, (*parent).name);
    if element_desc.is_null() && !(*(*parent).doc).extSubset.is_null() {
        element_desc =
            xmlGetDtdElementDesc((*(*parent).doc).extSubset, (*parent).name)
    }
    if element_desc.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * Do a backup of the current tree structure
     */
    prev_next =
        if !prev.is_null() { (*prev).next } else { 0 as *mut _xmlNode };
    next_prev =
        if !next.is_null() { (*next).prev } else { 0 as *mut _xmlNode };
    parent_childs = (*parent).children;
    parent_last = (*parent).last;
    /*
     * Creates a dummy node and insert it into the tree
     */
    test_node =
        xmlNewDocNode((*ref_node).doc, 0 as xmlNsPtr,
                      b"<!dummy?>\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar, 0 as *const xmlChar);
    if test_node.is_null() { return -(1 as std::os::raw::c_int) }
    (*test_node).parent = parent;
    (*test_node).prev = prev;
    (*test_node).next = next;
    name = (*test_node).name;
    if !prev.is_null() {
        (*prev).next = test_node
    } else { (*parent).children = test_node }
    if !next.is_null() {
        (*next).prev = test_node
    } else { (*parent).last = test_node }
    /*
     * Insert each potential child node and check if the parent is
     * still valid
     */
    nb_elements =
        xmlValidGetPotentialChildren((*element_desc).content,
                                     elements.as_mut_ptr(), &mut nb_elements,
                                     256 as std::os::raw::c_int);
    i = 0 as std::os::raw::c_int;
    while i < nb_elements {
        (*test_node).name = elements[i as usize];
        if xmlValidateOneElement(&mut vctxt, (*parent).doc, parent) != 0 {
            let mut j: std::os::raw::c_int = 0;
            j = 0 as std::os::raw::c_int;
            while j < nb_valid_elements {
                if xmlStrEqual(elements[i as usize],
                               *names.offset(j as isize)) != 0 {
                    break ;
                }
                j += 1
            }
            let fresh21 = nb_valid_elements;
            nb_valid_elements = nb_valid_elements + 1;
            let ref mut fresh22 = *names.offset(fresh21 as isize);
            *fresh22 = elements[i as usize];
            if nb_valid_elements >= max { break ; }
        }
        i += 1
    }
    /*
     * Restore the tree structure
     */
    if !prev.is_null() { (*prev).next = prev_next }
    if !next.is_null() { (*next).prev = next_prev }
    (*parent).children = parent_childs;
    (*parent).last = parent_last;
    /*
     * Free up the dummy node
     */
    (*test_node).name = name;
    xmlFreeNode(test_node);
    return nb_valid_elements;
}
/* __INCLUDE_ELFGCCHACK */
/* LIBXML_VALID_ENABLED */
