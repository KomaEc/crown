
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
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
    pub type _xmlHashTable;
    /* *
 * xmlAutomataStatePtr:
 *
 * A state int the automata description,
 */
    pub type _xmlAutomataState;
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
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlRegexp;
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
    /* *
 * BAD_CAST:
 *
 * Macro to cast a string to an xmlChar * when one know its safe.
 */
    /*
 * xmlChar handling
 */
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    static mut __xmlRegisterCallbacks: std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    /* *
 * xmlChildrenNode:
 *
 * Macro for compatibility naming layer with libxml1. Maps
 * to "children."
 */
    /* *
 * xmlRootNode:
 *
 * Macro for compatibility naming layer with libxml1. Maps
 * to "children".
 */
    /*
 * Variables.
 */
    /*
 * Some helper functions
 */
    #[no_mangle]
    fn xmlValidateNCName(value: *const xmlChar, space: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlBuildQName(ncname: *const xmlChar, prefix: *const xmlChar,
                     memory: *mut xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    /*
 * Creating/freeing new structures.
 */
    #[no_mangle]
    fn xmlCreateIntSubset(doc: xmlDocPtr, name: *const xmlChar,
                          ExternalID: *const xmlChar,
                          SystemID: *const xmlChar) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlNewDtd(doc: xmlDocPtr, name: *const xmlChar,
                 ExternalID: *const xmlChar, SystemID: *const xmlChar)
     -> xmlDtdPtr;
    #[no_mangle]
    fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlFreeDtd(cur: xmlDtdPtr);
    /* LIBXML_LEGACY_ENABLED */
    #[no_mangle]
    fn xmlNewNs(node: xmlNodePtr, href: *const xmlChar,
                prefix: *const xmlChar) -> xmlNsPtr;
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlNewNsProp(node: xmlNodePtr, ns: xmlNsPtr, name: *const xmlChar,
                    value: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlNewNsPropEatName(node: xmlNodePtr, ns: xmlNsPtr, name: *mut xmlChar,
                           value: *const xmlChar) -> xmlAttrPtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewDocNodeEatName(doc: xmlDocPtr, ns: xmlNsPtr, name: *mut xmlChar,
                            content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar)
     -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewDocPI(doc: xmlDocPtr, name: *const xmlChar,
                   content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewDocComment(doc: xmlDocPtr, content: *const xmlChar)
     -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewCDataBlock(doc: xmlDocPtr, content: *const xmlChar,
                        len: std::os::raw::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewCharRef(doc: xmlDocPtr, name: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewReference(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlNodePtr;
    #[no_mangle]
    fn xmlGetLastChild(parent: *const xmlNode) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED || LIBXML_HTML_ENABLED || LIBXML_SCHEMAS_ENABLED */
    #[no_mangle]
    fn xmlAddSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlUnlinkNode(cur: xmlNodePtr);
    #[no_mangle]
    fn xmlTextConcat(node: xmlNodePtr, content: *const xmlChar,
                     len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeNode(cur: xmlNodePtr);
    /*
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr,
                   nameSpace: *const xmlChar) -> xmlNsPtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_XPATH_ENABLED) */
    #[no_mangle]
    fn xmlSetNs(node: xmlNodePtr, ns: xmlNsPtr);
    #[no_mangle]
    fn xmlStringGetNodeList(doc: *const xmlDoc, value: *const xmlChar)
     -> xmlNodePtr;
    #[no_mangle]
    fn xmlStringLenGetNodeList(doc: *const xmlDoc, value: *const xmlChar,
                               len: std::os::raw::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlDictQLookup(dict: xmlDictPtr, prefix: *const xmlChar,
                      name: *const xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlDictReference(dict: xmlDictPtr) -> std::os::raw::c_int;
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
    /* Notation */
    #[no_mangle]
    fn xmlAddNotationDecl(ctxt: xmlValidCtxtPtr, dtd: xmlDtdPtr,
                          name: *const xmlChar, PublicID: *const xmlChar,
                          SystemID: *const xmlChar) -> xmlNotationPtr;
    /* LIBXML_OUTPUT_ENABLED */
    /* DEPRECATED */
    /* Element */
    #[no_mangle]
    fn xmlAddElementDecl(ctxt: xmlValidCtxtPtr, dtd: xmlDtdPtr,
                         name: *const xmlChar, type_0: xmlElementTypeVal,
                         content: xmlElementContentPtr) -> xmlElementPtr;
    #[no_mangle]
    fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
    /* LIBXML_TREE_ENABLED */
    /* Attribute */
    #[no_mangle]
    fn xmlAddAttributeDecl(ctxt: xmlValidCtxtPtr, dtd: xmlDtdPtr,
                           elem: *const xmlChar, name: *const xmlChar,
                           ns: *const xmlChar, type_0: xmlAttributeType,
                           def: xmlAttributeDefault,
                           defaultValue: *const xmlChar,
                           tree: xmlEnumerationPtr) -> xmlAttributePtr;
    /* LIBXML_OUTPUT_ENABLED */
    /* IDs */
    #[no_mangle]
    fn xmlAddID(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, value: *const xmlChar,
                attr: xmlAttrPtr) -> xmlIDPtr;
    #[no_mangle]
    fn xmlIsID(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr)
     -> std::os::raw::c_int;
    /* IDREFs */
    #[no_mangle]
    fn xmlAddRef(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, value: *const xmlChar,
                 attr: xmlAttrPtr) -> xmlRefPtr;
    #[no_mangle]
    fn xmlIsRef(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateRoot(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateElementDecl(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                              elem: xmlElementPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidNormalizeAttributeValue(doc: xmlDocPtr, elem: xmlNodePtr,
                                       name: *const xmlChar,
                                       value: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlValidCtxtNormalizeAttributeValue(ctxt: xmlValidCtxtPtr,
                                           doc: xmlDocPtr, elem: xmlNodePtr,
                                           name: *const xmlChar,
                                           value: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlValidateAttributeDecl(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                                attr: xmlAttributePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateNotationDecl(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                               nota: xmlNotationPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateDtdFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateOneElement(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                             elem: xmlNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateOneAttribute(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                               elem: xmlNodePtr, attr: xmlAttrPtr,
                               value: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateOneNamespace(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                               elem: xmlNodePtr, prefix: *const xmlChar,
                               ns: xmlNsPtr, value: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateDocumentFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlGetDtdQAttrDesc(dtd: xmlDtdPtr, elem: *const xmlChar,
                          name: *const xmlChar, prefix: *const xmlChar)
     -> xmlAttributePtr;
    #[no_mangle]
    fn xmlGetDtdQElementDesc(dtd: xmlDtdPtr, name: *const xmlChar,
                             prefix: *const xmlChar) -> xmlElementPtr;
    #[no_mangle]
    fn xmlAddDocEntity(doc: xmlDocPtr, name: *const xmlChar,
                       type_0: std::os::raw::c_int, ExternalID: *const xmlChar,
                       SystemID: *const xmlChar, content: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlAddDtdEntity(doc: xmlDocPtr, name: *const xmlChar,
                       type_0: std::os::raw::c_int, ExternalID: *const xmlChar,
                       SystemID: *const xmlChar, content: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;
    #[no_mangle]
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlGetParameterEntity(doc: xmlDocPtr, name: *const xmlChar)
     -> xmlEntityPtr;
    /*
 * Interfaces directly used by the parsers.
 */
    #[no_mangle]
    fn xmlDetectCharEncoding(in_0: *const std::os::raw::c_uchar, len: std::os::raw::c_int)
     -> xmlCharEncoding;
    /* LIBXML_SAX1_ENABLED */
    #[no_mangle]
    fn xmlParseCtxtExternalEntity(ctx: xmlParserCtxtPtr, URL: *const xmlChar,
                                  ID: *const xmlChar, lst: *mut xmlNodePtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParserAddNodeInfo(ctxt: xmlParserCtxtPtr,
                            info: xmlParserNodeInfoPtr);
    #[no_mangle]
    fn xmlLoadExternalEntity(URL: *const std::os::raw::c_char,
                             ID: *const std::os::raw::c_char, ctxt: xmlParserCtxtPtr)
     -> xmlParserInputPtr;
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
    #[no_mangle]
    fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn __docbDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    #[no_mangle]
    fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    /* *
 * IS_BASECHAR:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 * [85] BaseChar ::= ... long list see REC ...
 */
    /* *
 * IS_DIGIT:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 * [88] Digit ::= ... long list see REC ...
 */
    /* *
 * IS_DIGIT_CH:
 * @c:  an xmlChar value (usually an unsigned char)
 *
 * Behaves like IS_DIGIT but with a single byte argument
 */
    /* *
 * IS_COMBINING:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 * [87] CombiningChar ::= ... long list see REC ...
 */
    /* *
 * IS_COMBINING_CH:
 * @c:  an xmlChar (usually an unsigned char)
 *
 * Always false (all combining chars > 0xff)
 */
    /* *
 * IS_EXTENDER:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [89] Extender ::= #x00B7 | #x02D0 | #x02D1 | #x0387 | #x0640 |
 *                   #x0E46 | #x0EC6 | #x3005 | [#x3031-#x3035] |
 *                   [#x309D-#x309E] | [#x30FC-#x30FE]
 */
    /* *
 * IS_EXTENDER_CH:
 * @c:  an xmlChar value (usually an unsigned char)
 *
 * Behaves like IS_EXTENDER but with a single-byte argument
 */
    /* *
 * IS_IDEOGRAPHIC:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [86] Ideographic ::= [#x4E00-#x9FA5] | #x3007 | [#x3021-#x3029]
 */
    /* *
 * IS_LETTER:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [84] Letter ::= BaseChar | Ideographic
 */
    /* *
 * IS_LETTER_CH:
 * @c:  an xmlChar value (normally unsigned char)
 *
 * Macro behaves like IS_LETTER, but only check base chars
 *
 */
    /* *
 * IS_ASCII_LETTER:
 * @c: an xmlChar value
 *
 * Macro to check [a-zA-Z]
 *
 */
    /* *
 * IS_ASCII_DIGIT:
 * @c: an xmlChar value
 *
 * Macro to check [0-9]
 *
 */
    /* *
 * IS_PUBIDCHAR:
 * @c:  an UNICODE value (int)
 *
 * Macro to check the following production in the XML spec:
 *
 *
 * [13] PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
 */
    /* *
 * IS_PUBIDCHAR_CH:
 * @c:  an xmlChar value (normally unsigned char)
 *
 * Same as IS_PUBIDCHAR but for single-byte value
 */
    /* *
 * SKIP_EOL:
 * @p:  and UTF8 string pointer
 *
 * Skips the end of line chars.
 */
    /* *
 * MOVETO_ENDTAG:
 * @p:  and UTF8 string pointer
 *
 * Skips to the next '>' char.
 */
    /* *
 * MOVETO_STARTTAG:
 * @p:  and UTF8 string pointer
 *
 * Skips to the next '<' char.
 */
    /* *
 * Global variables used for predefined strings.
 */
    #[no_mangle]
    static xmlStringText: [xmlChar; 0];
    #[no_mangle]
    fn xmlSwitchEncoding(ctxt: xmlParserCtxtPtr, enc: xmlCharEncoding)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlPushInput(ctxt: xmlParserCtxtPtr, input: xmlParserInputPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlPopInput(ctxt: xmlParserCtxtPtr) -> xmlChar;
    #[no_mangle]
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    /* *
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSplitQName(ctxt: xmlParserCtxtPtr, name: *const xmlChar,
                     prefix: *mut *mut xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseExternalSubset(ctxt: xmlParserCtxtPtr,
                              ExternalID: *const xmlChar,
                              SystemID: *const xmlChar);
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
    #[no_mangle]
    fn xmlStringDecodeEntities(ctxt: xmlParserCtxtPtr, str: *const xmlChar,
                               what: std::os::raw::c_int, end: xmlChar, end2: xmlChar,
                               end3: xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStringLenDecodeEntities(ctxt: xmlParserCtxtPtr, str: *const xmlChar,
                                  len: std::os::raw::c_int, what: std::os::raw::c_int,
                                  end: xmlChar, end2: xmlChar, end3: xmlChar)
     -> *mut xmlChar;
    /*
 * Generated by MACROS on top of parser.c c.f. PUSH_AND_POP.
 */
    #[no_mangle]
    fn nodePush(ctxt: xmlParserCtxtPtr, value: xmlNodePtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn nodePop(ctxt: xmlParserCtxtPtr) -> xmlNodePtr;
    /* LIBXML_LEGACY_ENABLED */
    /*
 * internal only
 */
    #[no_mangle]
    fn xmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const std::os::raw::c_char);
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    #[no_mangle]
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlPathToURI(path: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn htmlNewDocNoDtD(URI: *const xmlChar, ExternalID: *const xmlChar)
     -> htmlDocPtr;
    /* LIBXML_OUTPUT_ENABLED */
    #[no_mangle]
    fn htmlIsBooleanAttr(name: *const xmlChar) -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
pub type ptrdiff_t = std::os::raw::c_long;
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
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
pub type xmlReallocFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t)
               -> *mut std::os::raw::c_void>;
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
/* Hash table for param entities if any */
/* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
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
/* *
 * xmlDoc:
 *
 * An XML document.
 */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNotation {
    pub name: *const xmlChar,
    pub PublicID: *const xmlChar,
    pub SystemID: *const xmlChar,
}
/* *
 * xmlNotation:
 *
 * A DTD Notation definition.
 */
pub type xmlNotation = _xmlNotation;
pub type xmlNotationPtr = *mut xmlNotation;
pub type xmlAttributeDefault = std::os::raw::c_uint;
pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
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
/* Enumeration name */
/* *
 * xmlAttribute:
 *
 * An Attribute declaration in a DTD.
 */
pub type xmlAttribute = _xmlAttribute;
pub type xmlAttributePtr = *mut xmlAttribute;
pub type xmlElementTypeVal = std::os::raw::c_uint;
pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
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
/* *
 * xmlElement:
 *
 * An XML Element declaration from a DTD.
 */
pub type xmlElement = _xmlElement;
pub type xmlElementPtr = *mut xmlElement;
pub type xmlNsPtr = *mut xmlNs;
/* normally an xmlDoc */
/* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
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
/* for type/PSVI informations */
/* *
 * xmlID:
 *
 * An XML ID instance.
 */
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
/* The document holding the ID */
/* *
 * xmlRef:
 *
 * An XML IDREF instance.
 */
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
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlParserNodeInfoPtr = *mut xmlParserNodeInfo;
/*
 * SAX Version 1
 */
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
pub type htmlDocPtr = xmlDocPtr;
pub type xmlURIPtr = *mut xmlURI;
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
pub type xmlRegisterNodeFunc
    =
    Option<unsafe extern "C" fn(_: xmlNodePtr) -> ()>;
/* !SIZE_T_MAX */
/* #define DEBUG_SAX2 */
/* #define DEBUG_SAX2_TREE */
/* *
 * TODO:
 *
 * macro to flag unimplemented blocks
 * XML_CATALOG_PREFER user env to select between system/public prefered
 * option. C.f. Richard Tobin <richard@cogsci.ed.ac.uk>
 *> Just FYI, I am using an environment variable XML_CATALOG_PREFER with
 *> values "system" and "public".  I have made the default be "system" to
 *> match yours.
 */
/*
 * xmlSAX2ErrMemory:
 * @ctxt:  an XML validation parser context
 * @msg:   a string to accompany the error message
 */
unsafe extern "C" fn xmlSAX2ErrMemory(mut ctxt: xmlParserCtxtPtr,
                                      mut msg: *const std::os::raw::c_char) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut str1: *const std::os::raw::c_char =
        b"out of memory\n\x00" as *const u8 as *const std::os::raw::c_char;
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_NO_MEMORY as std::os::raw::c_int;
        if !(*ctxt).sax.is_null() &&
               (*(*ctxt).sax).initialized == 0xdeedbeaf as std::os::raw::c_uint {
            schannel = (*(*ctxt).sax).serror
        }
        __xmlRaiseError(schannel, (*ctxt).vctxt.error, (*ctxt).vctxt.userData,
                        ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_PARSER as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_ERROR,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, str1,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, str1,
                        0 as *mut std::os::raw::c_void);
        (*ctxt).errNo = XML_ERR_NO_MEMORY as std::os::raw::c_int;
        (*ctxt).instate = XML_PARSER_EOF;
        (*ctxt).disableSAX = 1 as std::os::raw::c_int
    } else {
        __xmlRaiseError(schannel, None, 0 as *mut std::os::raw::c_void,
                        ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_PARSER as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_ERROR,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, str1,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, str1,
                        0 as *mut std::os::raw::c_void);
    };
}
/* *
 * xmlValidError:
 * @ctxt:  an XML validation parser context
 * @error:  the error number
 * @msg:  the error message
 * @str1:  extra data
 * @str2:  extra data
 *
 * Handle a validation error
 */
unsafe extern "C" fn xmlErrValid(mut ctxt: xmlParserCtxtPtr,
                                 mut error: xmlParserErrors,
                                 mut msg: *const std::os::raw::c_char,
                                 mut str1: *const std::os::raw::c_char,
                                 mut str2: *const std::os::raw::c_char) {
    let mut schannel: xmlStructuredErrorFunc = None;
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as std::os::raw::c_int;
        if !(*ctxt).sax.is_null() &&
               (*(*ctxt).sax).initialized == 0xdeedbeaf as std::os::raw::c_uint {
            schannel = (*(*ctxt).sax).serror
        }
        __xmlRaiseError(schannel, (*ctxt).vctxt.error, (*ctxt).vctxt.userData,
                        ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_DTD as std::os::raw::c_int, error as std::os::raw::c_int,
                        XML_ERR_ERROR, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, str1, str2,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int, msg, str1, str2);
        (*ctxt).valid = 0 as std::os::raw::c_int
    } else {
        __xmlRaiseError(schannel, None, 0 as *mut std::os::raw::c_void,
                        ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_DTD as std::os::raw::c_int, error as std::os::raw::c_int,
                        XML_ERR_ERROR, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, str1, str2,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                        0 as std::os::raw::c_int, msg, str1, str2);
    };
}
/* *
 * xmlFatalErrMsg:
 * @ctxt:  an XML parser context
 * @error:  the error number
 * @msg:  the error message
 * @str1:  an error string
 * @str2:  an error string
 *
 * Handle a fatal parser error, i.e. violating Well-Formedness constraints
 */
unsafe extern "C" fn xmlFatalErrMsg(mut ctxt: xmlParserCtxtPtr,
                                    mut error: xmlParserErrors,
                                    mut msg: *const std::os::raw::c_char,
                                    mut str1: *const xmlChar,
                                    mut str2: *const xmlChar) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() { (*ctxt).errNo = error as std::os::raw::c_int }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_PARSER as std::os::raw::c_int, error as std::os::raw::c_int,
                    XML_ERR_FATAL, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2);
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as std::os::raw::c_int;
        (*ctxt).valid = 0 as std::os::raw::c_int;
        if (*ctxt).recovery == 0 as std::os::raw::c_int {
            (*ctxt).disableSAX = 1 as std::os::raw::c_int
        }
    };
}
/* *
 * xmlWarnMsg:
 * @ctxt:  an XML parser context
 * @error:  the error number
 * @msg:  the error message
 * @str1:  an error string
 * @str2:  an error string
 *
 * Handle a parser warning
 */
unsafe extern "C" fn xmlWarnMsg(mut ctxt: xmlParserCtxtPtr,
                                mut error: xmlParserErrors,
                                mut msg: *const std::os::raw::c_char,
                                mut str1: *const xmlChar) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() { (*ctxt).errNo = error as std::os::raw::c_int }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_PARSER as std::os::raw::c_int, error as std::os::raw::c_int,
                    XML_ERR_WARNING, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, str1 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, str1);
}
/* *
 * xmlNsErrMsg:
 * @ctxt:  an XML parser context
 * @error:  the error number
 * @msg:  the error message
 * @str1:  an error string
 * @str2:  an error string
 *
 * Handle a namespace error
 */
unsafe extern "C" fn xmlNsErrMsg(mut ctxt: xmlParserCtxtPtr,
                                 mut error: xmlParserErrors,
                                 mut msg: *const std::os::raw::c_char,
                                 mut str1: *const xmlChar,
                                 mut str2: *const xmlChar) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() { (*ctxt).errNo = error as std::os::raw::c_int }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_NAMESPACE as std::os::raw::c_int, error as std::os::raw::c_int,
                    XML_ERR_ERROR, 0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2);
}
/* *
 * xmlNsWarnMsg:
 * @ctxt:  an XML parser context
 * @error:  the error number
 * @msg:  the error message
 * @str1:  an error string
 *
 * Handle a namespace warning
 */
unsafe extern "C" fn xmlNsWarnMsg(mut ctxt: xmlParserCtxtPtr,
                                  mut error: xmlParserErrors,
                                  mut msg: *const std::os::raw::c_char,
                                  mut str1: *const xmlChar,
                                  mut str2: *const xmlChar) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as std::os::raw::c_int &&
           (*ctxt).instate as std::os::raw::c_int == XML_PARSER_EOF as std::os::raw::c_int {
        return
    }
    if !ctxt.is_null() { (*ctxt).errNo = error as std::os::raw::c_int }
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    ctxt as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_NAMESPACE as std::os::raw::c_int, error as std::os::raw::c_int,
                    XML_ERR_WARNING, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, str1 as *const std::os::raw::c_char,
                    str2 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, str1, str2);
}
/* *
 * xmlSAX2GetPublicId:
 * @ctx: the user data (XML parser context)
 *
 * Provides the public ID e.g. "-//SGMLSOURCE//DTD DEMO//EN"
 *
 * Returns a xmlChar *
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetPublicId(mut ctx: *mut std::os::raw::c_void)
 -> *const xmlChar {
    /* xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx; */
    return 0 as *const xmlChar;
}
/* *
 * xmlSAX2GetSystemId:
 * @ctx: the user data (XML parser context)
 *
 * Provides the system ID, basically URL or filename e.g.
 * http://www.sgmlsource.com/dtds/memo.dtd
 *
 * Returns a xmlChar *
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetSystemId(mut ctx: *mut std::os::raw::c_void)
 -> *const xmlChar {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).input.is_null() { return 0 as *const xmlChar }
    return (*(*ctxt).input).filename as *const xmlChar;
}
/* *
 * xmlSAX2GetLineNumber:
 * @ctx: the user data (XML parser context)
 *
 * Provide the line number of the current parsing point.
 *
 * Returns an int
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetLineNumber(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).input.is_null() { return 0 as std::os::raw::c_int }
    return (*(*ctxt).input).line;
}
/* *
 * xmlSAX2GetColumnNumber:
 * @ctx: the user data (XML parser context)
 *
 * Provide the column number of the current parsing point.
 *
 * Returns an int
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetColumnNumber(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).input.is_null() { return 0 as std::os::raw::c_int }
    return (*(*ctxt).input).col;
}
/* *
 * xmlSAX2IsStandalone:
 * @ctx: the user data (XML parser context)
 *
 * Is this document tagged standalone ?
 *
 * Returns 1 if true
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2IsStandalone(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).myDoc.is_null() { return 0 as std::os::raw::c_int }
    return ((*(*ctxt).myDoc).standalone == 1 as std::os::raw::c_int) as std::os::raw::c_int;
}
/* *
 * xmlSAX2HasInternalSubset:
 * @ctx: the user data (XML parser context)
 *
 * Does this document has an internal subset
 *
 * Returns 1 if true
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2HasInternalSubset(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() { return 0 as std::os::raw::c_int }
    return ((*(*ctxt).myDoc).intSubset !=
                0 as *mut std::os::raw::c_void as *mut _xmlDtd) as std::os::raw::c_int;
}
/* *
 * xmlSAX2HasExternalSubset:
 * @ctx: the user data (XML parser context)
 *
 * Does this document has an external subset
 *
 * Returns 1 if true
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2HasExternalSubset(mut ctx: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() { return 0 as std::os::raw::c_int }
    return ((*(*ctxt).myDoc).extSubset !=
                0 as *mut std::os::raw::c_void as *mut _xmlDtd) as std::os::raw::c_int;
}
/* *
 * xmlSAX2InternalSubset:
 * @ctx:  the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on internal subset declaration.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InternalSubset(mut ctx: *mut std::os::raw::c_void,
                                               mut name: *const xmlChar,
                                               mut ExternalID: *const xmlChar,
                                               mut SystemID: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    if ctx.is_null() { return }
    if (*ctxt).myDoc.is_null() { return }
    dtd = xmlGetIntSubset((*ctxt).myDoc as *const xmlDoc);
    if !dtd.is_null() {
        if (*ctxt).html != 0 { return }
        xmlUnlinkNode(dtd as xmlNodePtr);
        xmlFreeDtd(dtd);
        (*(*ctxt).myDoc).intSubset = 0 as *mut _xmlDtd
    }
    (*(*ctxt).myDoc).intSubset =
        xmlCreateIntSubset((*ctxt).myDoc, name, ExternalID, SystemID);
    if (*(*ctxt).myDoc).intSubset.is_null() {
        xmlSAX2ErrMemory(ctxt,
                         b"xmlSAX2InternalSubset\x00" as *const u8 as
                             *const std::os::raw::c_char);
    };
}
/* *
 * xmlSAX2ExternalSubset:
 * @ctx: the user data (XML parser context)
 * @name:  the root element name
 * @ExternalID:  the external ID
 * @SystemID:  the SYSTEM ID (e.g. filename or URL)
 *
 * Callback on external subset declaration.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ExternalSubset(mut ctx: *mut std::os::raw::c_void,
                                               mut name: *const xmlChar,
                                               mut ExternalID: *const xmlChar,
                                               mut SystemID: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() { return }
    if (!ExternalID.is_null() || !SystemID.is_null()) &&
           (((*ctxt).validate != 0 || (*ctxt).loadsubset != 0 as std::os::raw::c_int)
                && ((*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null())) {
        /*
	 * Try to fetch and parse the external subset.
	 */
        let mut oldinput: xmlParserInputPtr = 0 as *mut xmlParserInput;
        let mut oldinputNr: std::os::raw::c_int = 0;
        let mut oldinputMax: std::os::raw::c_int = 0;
        let mut oldinputTab: *mut xmlParserInputPtr =
            0 as *mut xmlParserInputPtr;
        let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        let mut oldcharset: std::os::raw::c_int = 0;
        let mut oldencoding: *const xmlChar = 0 as *const xmlChar;
        /*
	 * Ask the Entity resolver to load the damn thing
	 */
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).resolveEntity.is_some() {
            input =
                (*(*ctxt).sax).resolveEntity.expect("non-null function pointer")((*ctxt).userData,
                                                                                 ExternalID,
                                                                                 SystemID)
        }
        if input.is_null() { return }
        xmlNewDtd((*ctxt).myDoc, name, ExternalID, SystemID);
        /*
	 * make sure we won't destroy the main document context
	 */
        oldinput = (*ctxt).input;
        oldinputNr = (*ctxt).inputNr;
        oldinputMax = (*ctxt).inputMax;
        oldinputTab = (*ctxt).inputTab;
        oldcharset = (*ctxt).charset;
        oldencoding = (*ctxt).encoding;
        (*ctxt).encoding = 0 as *const xmlChar;
        (*ctxt).inputTab =
            xmlMalloc.expect("non-null function pointer")((5 as std::os::raw::c_int as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlParserInputPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlParserInputPtr;
        if (*ctxt).inputTab.is_null() {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2ExternalSubset\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            (*ctxt).input = oldinput;
            (*ctxt).inputNr = oldinputNr;
            (*ctxt).inputMax = oldinputMax;
            (*ctxt).inputTab = oldinputTab;
            (*ctxt).charset = oldcharset;
            (*ctxt).encoding = oldencoding;
            return
        }
        (*ctxt).inputNr = 0 as std::os::raw::c_int;
        (*ctxt).inputMax = 5 as std::os::raw::c_int;
        (*ctxt).input = 0 as xmlParserInputPtr;
        xmlPushInput(ctxt, input);
        /*
	 * On the fly encoding conversion if needed
	 */
        if (*(*ctxt).input).length >= 4 as std::os::raw::c_int {
            enc =
                xmlDetectCharEncoding((*(*ctxt).input).cur, 4 as std::os::raw::c_int);
            xmlSwitchEncoding(ctxt, enc);
        }
        if (*input).filename.is_null() {
            (*input).filename = xmlCanonicPath(SystemID) as *mut std::os::raw::c_char
        }
        (*input).line = 1 as std::os::raw::c_int;
        (*input).col = 1 as std::os::raw::c_int;
        (*input).base = (*(*ctxt).input).cur;
        (*input).cur = (*(*ctxt).input).cur;
        (*input).free = None;
        /*
	 * let's parse that entity knowing it's an external subset.
	 */
        xmlParseExternalSubset(ctxt, ExternalID, SystemID);
        /*
	 * Free up the external entities
	 */
        while (*ctxt).inputNr > 1 as std::os::raw::c_int { xmlPopInput(ctxt); }
        xmlFreeInputStream((*ctxt).input);
        xmlFree.expect("non-null function pointer")((*ctxt).inputTab as
                                                        *mut std::os::raw::c_void);
        /*
	 * Restore the parsing context of the main entity
	 */
        (*ctxt).input = oldinput;
        (*ctxt).inputNr = oldinputNr;
        (*ctxt).inputMax = oldinputMax;
        (*ctxt).inputTab = oldinputTab;
        (*ctxt).charset = oldcharset;
        if !(*ctxt).encoding.is_null() &&
               ((*ctxt).dict.is_null() ||
                    xmlDictOwns((*ctxt).dict, (*ctxt).encoding) == 0) {
            xmlFree.expect("non-null function pointer")((*ctxt).encoding as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        (*ctxt).encoding = oldencoding
    };
}
/* *
 * xmlSAX2ResolveEntity:
 * @ctx: the user data (XML parser context)
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * The entity loader, to control the loading of external entities,
 * the application can either:
 *    - override this xmlSAX2ResolveEntity() callback in the SAX block
 *    - or better use the xmlSetExternalEntityLoader() function to
 *      set up it's own entity resolution routine
 *
 * Returns the xmlParserInputPtr if inlined or NULL for DOM behaviour.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ResolveEntity(mut ctx: *mut std::os::raw::c_void,
                                              mut publicId: *const xmlChar,
                                              mut systemId: *const xmlChar)
 -> xmlParserInputPtr {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if ctx.is_null() { return 0 as xmlParserInputPtr }
    if !(*ctxt).input.is_null() { base = (*(*ctxt).input).filename }
    if base.is_null() { base = (*ctxt).directory }
    URI = xmlBuildURI(systemId, base as *const xmlChar);
    ret =
        xmlLoadExternalEntity(URI as *const std::os::raw::c_char,
                              publicId as *const std::os::raw::c_char, ctxt);
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut std::os::raw::c_void);
    }
    return ret;
}
/* *
 * xmlSAX2GetEntity:
 * @ctx: the user data (XML parser context)
 * @name: The entity name
 *
 * Get an entity by name
 *
 * Returns the xmlEntityPtr if found.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetEntity(mut ctx: *mut std::os::raw::c_void,
                                          mut name: *const xmlChar)
 -> xmlEntityPtr {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlEntityPtr = 0 as xmlEntityPtr;
    if ctx.is_null() { return 0 as xmlEntityPtr }
    if (*ctxt).inSubset == 0 as std::os::raw::c_int {
        ret = xmlGetPredefinedEntity(name);
        if !ret.is_null() { return ret }
    }
    if !(*ctxt).myDoc.is_null() &&
           (*(*ctxt).myDoc).standalone == 1 as std::os::raw::c_int {
        if (*ctxt).inSubset == 2 as std::os::raw::c_int {
            (*(*ctxt).myDoc).standalone = 0 as std::os::raw::c_int;
            ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
            (*(*ctxt).myDoc).standalone = 1 as std::os::raw::c_int
        } else {
            ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
            if ret.is_null() {
                (*(*ctxt).myDoc).standalone = 0 as std::os::raw::c_int;
                ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
                if !ret.is_null() {
                    xmlFatalErrMsg(ctxt, XML_ERR_NOT_STANDALONE,
                                   b"Entity(%s) document marked standalone but requires external subset\n\x00"
                                       as *const u8 as *const std::os::raw::c_char,
                                   name, 0 as *const xmlChar);
                }
                (*(*ctxt).myDoc).standalone = 1 as std::os::raw::c_int
            }
        }
    } else { ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name) }
    if !ret.is_null() &&
           ((*ctxt).validate != 0 || (*ctxt).replaceEntities != 0) &&
           (*ret).children.is_null() &&
           (*ret).etype as std::os::raw::c_uint ==
               XML_EXTERNAL_GENERAL_PARSED_ENTITY as std::os::raw::c_int as
                   std::os::raw::c_uint {
        let mut val: std::os::raw::c_int = 0;
        /*
	 * for validation purposes we really need to fetch and
	 * parse the external entity
	 */
        let mut children: xmlNodePtr = 0 as *mut xmlNode;
        let mut oldnbent: std::os::raw::c_ulong = (*ctxt).nbentities;
        val =
            xmlParseCtxtExternalEntity(ctxt, (*ret).URI, (*ret).ExternalID,
                                       &mut children);
        if val == 0 as std::os::raw::c_int {
            xmlAddChildList(ret as xmlNodePtr, children);
        } else {
            xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_PROCESSING,
                           b"Failure to process entity %s\n\x00" as *const u8
                               as *const std::os::raw::c_char, name,
                           0 as *const xmlChar);
            (*ctxt).validate = 0 as std::os::raw::c_int;
            return 0 as xmlEntityPtr
        }
        (*ret).owner = 1 as std::os::raw::c_int;
        if (*ret).checked == 0 as std::os::raw::c_int {
            (*ret).checked =
                (*ctxt).nbentities.wrapping_sub(oldnbent).wrapping_add(1 as
                                                                           std::os::raw::c_int
                                                                           as
                                                                           std::os::raw::c_ulong).wrapping_mul(2
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           std::os::raw::c_ulong)
                    as std::os::raw::c_int;
            if !(*ret).content.is_null() &&
                   !xmlStrchr((*ret).content, '<' as i32 as xmlChar).is_null()
               {
                (*ret).checked |= 1 as std::os::raw::c_int
            }
        }
    }
    return ret;
}
/* *
 * xmlSAX2GetParameterEntity:
 * @ctx: the user data (XML parser context)
 * @name: The entity name
 *
 * Get a parameter entity by name
 *
 * Returns the xmlEntityPtr if found.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetParameterEntity(mut ctx: *mut std::os::raw::c_void,
                                                   mut name: *const xmlChar)
 -> xmlEntityPtr {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    if ctx.is_null() { return 0 as xmlEntityPtr }
    ret = xmlGetParameterEntity((*ctxt).myDoc, name);
    return ret;
}
/* *
 * xmlSAX2EntityDecl:
 * @ctx: the user data (XML parser context)
 * @name:  the entity name
 * @type:  the entity type
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @content: the entity value (without processing).
 *
 * An entity definition has been parsed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EntityDecl(mut ctx: *mut std::os::raw::c_void,
                                           mut name: *const xmlChar,
                                           mut type_0: std::os::raw::c_int,
                                           mut publicId: *const xmlChar,
                                           mut systemId: *const xmlChar,
                                           mut content: *mut xmlChar) {
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() { return }
    if (*ctxt).inSubset == 1 as std::os::raw::c_int {
        ent =
            xmlAddDocEntity((*ctxt).myDoc, name, type_0, publicId, systemId,
                            content);
        if ent.is_null() && (*ctxt).pedantic != 0 {
            xmlWarnMsg(ctxt, XML_WAR_ENTITY_REDEFINED,
                       b"Entity(%s) already defined in the internal subset\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name);
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI: *mut xmlChar = 0 as *mut xmlChar;
            let mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            if !(*ctxt).input.is_null() { base = (*(*ctxt).input).filename }
            if base.is_null() { base = (*ctxt).directory }
            URI = xmlBuildURI(systemId, base as *const xmlChar);
            (*ent).URI = URI
        }
    } else if (*ctxt).inSubset == 2 as std::os::raw::c_int {
        ent =
            xmlAddDtdEntity((*ctxt).myDoc, name, type_0, publicId, systemId,
                            content);
        if ent.is_null() && (*ctxt).pedantic != 0 && !(*ctxt).sax.is_null() &&
               (*(*ctxt).sax).warning.is_some() {
            (*(*ctxt).sax).warning.expect("non-null function pointer")((*ctxt).userData,
                                                                       b"Entity(%s) already defined in the external subset\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       name);
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut base_0: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            if !(*ctxt).input.is_null() { base_0 = (*(*ctxt).input).filename }
            if base_0.is_null() { base_0 = (*ctxt).directory }
            URI_0 = xmlBuildURI(systemId, base_0 as *const xmlChar);
            (*ent).URI = URI_0
        }
    } else {
        xmlFatalErrMsg(ctxt, XML_ERR_ENTITY_PROCESSING,
                       b"SAX.xmlSAX2EntityDecl(%s) called while not in subset\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
    };
}
/* *
 * xmlSAX2AttributeDecl:
 * @ctx: the user data (XML parser context)
 * @elem:  the name of the element
 * @fullname:  the attribute name
 * @type:  the attribute type
 * @def:  the type of default value
 * @defaultValue: the attribute default value
 * @tree:  the tree of enumerated value set
 *
 * An attribute definition has been parsed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2AttributeDecl(mut ctx: *mut std::os::raw::c_void,
                                              mut elem: *const xmlChar,
                                              mut fullname: *const xmlChar,
                                              mut type_0: std::os::raw::c_int,
                                              mut def: std::os::raw::c_int,
                                              mut defaultValue:
                                                  *const xmlChar,
                                              mut tree: xmlEnumerationPtr) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut attr: xmlAttributePtr = 0 as *mut xmlAttribute;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() { return }
    if xmlStrEqual(fullname,
                   b"xml:id\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 &&
           type_0 != XML_ATTRIBUTE_ID as std::os::raw::c_int {
        /*
	 * Raise the error but keep the validity flag
	 */
        let mut tmp: std::os::raw::c_int = (*ctxt).valid;
        xmlErrValid(ctxt, XML_DTD_XMLID_TYPE,
                    b"xml:id : attribute type should be ID\n\x00" as *const u8
                        as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char);
        (*ctxt).valid = tmp
    }
    /* TODO: optimize name/prefix allocation */
    name = xmlSplitQName(ctxt, fullname, &mut prefix);
    (*ctxt).vctxt.valid = 1 as std::os::raw::c_int;
    if (*ctxt).inSubset == 1 as std::os::raw::c_int {
        attr =
            xmlAddAttributeDecl(&mut (*ctxt).vctxt,
                                (*(*ctxt).myDoc).intSubset, elem, name,
                                prefix, type_0 as xmlAttributeType,
                                def as xmlAttributeDefault, defaultValue,
                                tree)
    } else if (*ctxt).inSubset == 2 as std::os::raw::c_int {
        attr =
            xmlAddAttributeDecl(&mut (*ctxt).vctxt,
                                (*(*ctxt).myDoc).extSubset, elem, name,
                                prefix, type_0 as xmlAttributeType,
                                def as xmlAttributeDefault, defaultValue,
                                tree)
    } else {
        xmlFatalErrMsg(ctxt, XML_ERR_INTERNAL_ERROR,
                       b"SAX.xmlSAX2AttributeDecl(%s) called while not in subset\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
        xmlFreeEnumeration(tree);
        return
    }
    if (*ctxt).vctxt.valid == 0 as std::os::raw::c_int {
        (*ctxt).valid = 0 as std::os::raw::c_int
    }
    if !attr.is_null() && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 &&
           !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &=
            xmlValidateAttributeDecl(&mut (*ctxt).vctxt, (*ctxt).myDoc, attr)
    }
    /* LIBXML_VALID_ENABLED */
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as
                                                        *mut std::os::raw::c_void);
    }
    if !name.is_null() {
        xmlFree.expect("non-null function pointer")(name as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlSAX2ElementDecl:
 * @ctx: the user data (XML parser context)
 * @name:  the element name
 * @type:  the element type
 * @content: the element value tree
 *
 * An element definition has been parsed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ElementDecl(mut ctx: *mut std::os::raw::c_void,
                                            mut name: *const xmlChar,
                                            mut type_0: std::os::raw::c_int,
                                            mut content:
                                                xmlElementContentPtr) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut elem: xmlElementPtr = 0 as xmlElementPtr;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() { return }
    if (*ctxt).inSubset == 1 as std::os::raw::c_int {
        elem =
            xmlAddElementDecl(&mut (*ctxt).vctxt, (*(*ctxt).myDoc).intSubset,
                              name, type_0 as xmlElementTypeVal, content)
    } else if (*ctxt).inSubset == 2 as std::os::raw::c_int {
        elem =
            xmlAddElementDecl(&mut (*ctxt).vctxt, (*(*ctxt).myDoc).extSubset,
                              name, type_0 as xmlElementTypeVal, content)
    } else {
        xmlFatalErrMsg(ctxt, XML_ERR_INTERNAL_ERROR,
                       b"SAX.xmlSAX2ElementDecl(%s) called while not in subset\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
        return
    }
    if elem.is_null() { (*ctxt).valid = 0 as std::os::raw::c_int }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 &&
           !(*ctxt).myDoc.is_null() && !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &=
            xmlValidateElementDecl(&mut (*ctxt).vctxt, (*ctxt).myDoc, elem)
    };
    /* LIBXML_VALID_ENABLED */
}
/* *
 * xmlSAX2NotationDecl:
 * @ctx: the user data (XML parser context)
 * @name: The name of the notation
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 *
 * What to do when a notation declaration has been parsed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2NotationDecl(mut ctx: *mut std::os::raw::c_void,
                                             mut name: *const xmlChar,
                                             mut publicId: *const xmlChar,
                                             mut systemId: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut nota: xmlNotationPtr = 0 as xmlNotationPtr;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() { return }
    if publicId.is_null() && systemId.is_null() {
        xmlFatalErrMsg(ctxt, XML_ERR_NOTATION_PROCESSING,
                       b"SAX.xmlSAX2NotationDecl(%s) externalID or PublicID missing\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
        return
    } else {
        if (*ctxt).inSubset == 1 as std::os::raw::c_int {
            nota =
                xmlAddNotationDecl(&mut (*ctxt).vctxt,
                                   (*(*ctxt).myDoc).intSubset, name, publicId,
                                   systemId)
        } else if (*ctxt).inSubset == 2 as std::os::raw::c_int {
            nota =
                xmlAddNotationDecl(&mut (*ctxt).vctxt,
                                   (*(*ctxt).myDoc).extSubset, name, publicId,
                                   systemId)
        } else {
            xmlFatalErrMsg(ctxt, XML_ERR_NOTATION_PROCESSING,
                           b"SAX.xmlSAX2NotationDecl(%s) called while not in subset\n\x00"
                               as *const u8 as *const std::os::raw::c_char, name,
                           0 as *const xmlChar);
            return
        }
    }
    if nota.is_null() { (*ctxt).valid = 0 as std::os::raw::c_int }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 &&
           !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &=
            xmlValidateNotationDecl(&mut (*ctxt).vctxt, (*ctxt).myDoc, nota)
    };
    /* LIBXML_VALID_ENABLED */
}
/* *
 * xmlSAX2UnparsedEntityDecl:
 * @ctx: the user data (XML parser context)
 * @name: The name of the entity
 * @publicId: The public ID of the entity
 * @systemId: The system ID of the entity
 * @notationName: the name of the notation
 *
 * What to do when an unparsed entity declaration is parsed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2UnparsedEntityDecl(mut ctx: *mut std::os::raw::c_void,
                                                   mut name: *const xmlChar,
                                                   mut publicId:
                                                       *const xmlChar,
                                                   mut systemId:
                                                       *const xmlChar,
                                                   mut notationName:
                                                       *const xmlChar) {
    let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() { return }
    if (*ctxt).inSubset == 1 as std::os::raw::c_int {
        ent =
            xmlAddDocEntity((*ctxt).myDoc, name,
                            XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as
                                std::os::raw::c_int, publicId, systemId,
                            notationName);
        if ent.is_null() && (*ctxt).pedantic != 0 && !(*ctxt).sax.is_null() &&
               (*(*ctxt).sax).warning.is_some() {
            (*(*ctxt).sax).warning.expect("non-null function pointer")((*ctxt).userData,
                                                                       b"Entity(%s) already defined in the internal subset\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       name);
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI: *mut xmlChar = 0 as *mut xmlChar;
            let mut base: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            if !(*ctxt).input.is_null() { base = (*(*ctxt).input).filename }
            if base.is_null() { base = (*ctxt).directory }
            URI = xmlBuildURI(systemId, base as *const xmlChar);
            (*ent).URI = URI
        }
    } else if (*ctxt).inSubset == 2 as std::os::raw::c_int {
        ent =
            xmlAddDtdEntity((*ctxt).myDoc, name,
                            XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as
                                std::os::raw::c_int, publicId, systemId,
                            notationName);
        if ent.is_null() && (*ctxt).pedantic != 0 && !(*ctxt).sax.is_null() &&
               (*(*ctxt).sax).warning.is_some() {
            (*(*ctxt).sax).warning.expect("non-null function pointer")((*ctxt).userData,
                                                                       b"Entity(%s) already defined in the external subset\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       name);
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut base_0: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
            if !(*ctxt).input.is_null() { base_0 = (*(*ctxt).input).filename }
            if base_0.is_null() { base_0 = (*ctxt).directory }
            URI_0 = xmlBuildURI(systemId, base_0 as *const xmlChar);
            (*ent).URI = URI_0
        }
    } else {
        xmlFatalErrMsg(ctxt, XML_ERR_INTERNAL_ERROR,
                       b"SAX.xmlSAX2UnparsedEntityDecl(%s) called while not in subset\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
    };
}
/* *
 * xmlSAX2SetDocumentLocator:
 * @ctx: the user data (XML parser context)
 * @loc: A SAX Locator
 *
 * Receive the document locator at startup, actually xmlDefaultSAXLocator
 * Everything is available on the context, so this is useless in our case.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2SetDocumentLocator(mut ctx: *mut std::os::raw::c_void,
                                                   mut loc:
                                                       xmlSAXLocatorPtr) {
    /* xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx; */
}
/* *
 * xmlSAX2StartDocument:
 * @ctx: the user data (XML parser context)
 *
 * called when the document start being processed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartDocument(mut ctx: *mut std::os::raw::c_void) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    if ctx.is_null() { return }
    if (*ctxt).html != 0 {
        if (*ctxt).myDoc.is_null() {
            (*ctxt).myDoc =
                htmlNewDocNoDtD(0 as *const xmlChar, 0 as *const xmlChar)
        }
        if (*ctxt).myDoc.is_null() {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2StartDocument\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        (*(*ctxt).myDoc).properties = XML_DOC_HTML as std::os::raw::c_int;
        (*(*ctxt).myDoc).parseFlags = (*ctxt).options
    } else {
        (*ctxt).myDoc = xmlNewDoc((*ctxt).version);
        doc = (*ctxt).myDoc;
        if !doc.is_null() {
            (*doc).properties = 0 as std::os::raw::c_int;
            if (*ctxt).options & XML_PARSE_OLD10 as std::os::raw::c_int != 0 {
                (*doc).properties |= XML_DOC_OLD10 as std::os::raw::c_int
            }
            (*doc).parseFlags = (*ctxt).options;
            if !(*ctxt).encoding.is_null() {
                (*doc).encoding = xmlStrdup((*ctxt).encoding)
            } else { (*doc).encoding = 0 as *const xmlChar }
            (*doc).standalone = (*ctxt).standalone
        } else {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2StartDocument\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        if (*ctxt).dictNames != 0 && !doc.is_null() {
            (*doc).dict = (*ctxt).dict;
            xmlDictReference((*doc).dict);
        }
    }
    if !(*ctxt).myDoc.is_null() && (*(*ctxt).myDoc).URL.is_null() &&
           !(*ctxt).input.is_null() && !(*(*ctxt).input).filename.is_null() {
        (*(*ctxt).myDoc).URL =
            xmlPathToURI((*(*ctxt).input).filename as *const xmlChar);
        if (*(*ctxt).myDoc).URL.is_null() {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2StartDocument\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    };
}
/* *
 * xmlSAX2EndDocument:
 * @ctx: the user data (XML parser context)
 *
 * called when the document end has been detected.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndDocument(mut ctx: *mut std::os::raw::c_void) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() { return }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 &&
           !(*ctxt).myDoc.is_null() && !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &=
            xmlValidateDocumentFinal(&mut (*ctxt).vctxt, (*ctxt).myDoc)
    }
    /* LIBXML_VALID_ENABLED */
    /*
     * Grab the encoding if it was added on-the-fly
     */
    if !(*ctxt).encoding.is_null() && !(*ctxt).myDoc.is_null() &&
           (*(*ctxt).myDoc).encoding.is_null() {
        (*(*ctxt).myDoc).encoding = (*ctxt).encoding;
        (*ctxt).encoding = 0 as *const xmlChar
    }
    if !(*ctxt).inputTab.is_null() && (*ctxt).inputNr > 0 as std::os::raw::c_int &&
           !(*(*ctxt).inputTab.offset(0 as std::os::raw::c_int as isize)).is_null() &&
           !(**(*ctxt).inputTab.offset(0 as std::os::raw::c_int as
                                           isize)).encoding.is_null() &&
           !(*ctxt).myDoc.is_null() && (*(*ctxt).myDoc).encoding.is_null() {
        (*(*ctxt).myDoc).encoding =
            xmlStrdup((**(*ctxt).inputTab.offset(0 as std::os::raw::c_int as
                                                     isize)).encoding)
    }
    if (*ctxt).charset != XML_CHAR_ENCODING_NONE as std::os::raw::c_int &&
           !(*ctxt).myDoc.is_null() &&
           (*(*ctxt).myDoc).charset == XML_CHAR_ENCODING_NONE as std::os::raw::c_int {
        (*(*ctxt).myDoc).charset = (*ctxt).charset
    };
}
/* *
 * xmlSAX2AttributeInternal:
 * @ctx: the user data (XML parser context)
 * @fullname:  The attribute name, including namespace prefix
 * @value:  The attribute value
 * @prefix: the prefix on the element node
 *
 * Handle an attribute that has been read by the parser.
 * The default handling is to convert the attribute into an
 * DOM subtree and past it in a new xmlAttr element added to
 * the element.
 */
unsafe extern "C" fn xmlSAX2AttributeInternal(mut ctx: *mut std::os::raw::c_void,
                                              mut fullname: *const xmlChar,
                                              mut value: *const xmlChar,
                                              mut prefix: *const xmlChar) {
    let mut current_block: u64;
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: *mut xmlChar = 0 as *mut xmlChar;
    let mut nval: *mut xmlChar = 0 as *mut xmlChar;
    let mut namespace: xmlNsPtr = 0 as *mut xmlNs;
    if (*ctxt).html != 0 {
        name = xmlStrdup(fullname);
        ns = 0 as *mut xmlChar;
        namespace = 0 as xmlNsPtr
    } else {
        /*
	 * Split the full name into a namespace prefix and the tag name
	 */
        name = xmlSplitQName(ctxt, fullname, &mut ns);
        if !name.is_null() &&
               *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            if xmlStrEqual(ns,
                           b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) != 0 {
                xmlNsErrMsg(ctxt, XML_ERR_NS_DECL_ERROR,
                            b"invalid namespace declaration \'%s\'\n\x00" as
                                *const u8 as *const std::os::raw::c_char, fullname,
                            0 as *const xmlChar);
            } else {
                xmlNsWarnMsg(ctxt, XML_WAR_NS_COLUMN,
                             b"Avoid attribute ending with \':\' like \'%s\'\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             fullname, 0 as *const xmlChar);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as
                                                                *mut std::os::raw::c_void);
            }
            ns = 0 as *mut xmlChar;
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
            name = xmlStrdup(fullname)
        }
    }
    if name.is_null() {
        xmlSAX2ErrMemory(ctxt,
                         b"xmlSAX2StartElement\x00" as *const u8 as
                             *const std::os::raw::c_char);
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as
                                                            *mut std::os::raw::c_void);
        }
        return
    }
    if (*ctxt).html != 0 && value.is_null() &&
           htmlIsBooleanAttr(fullname) != 0 {
        nval = xmlStrdup(fullname);
        value = nval as *const xmlChar
    } else {
        /*
         * Do the last stage of the attribute normalization
         * Needed for HTML too:
         *   http://www.w3.org/TR/html4/types.html#h-6.2
         */
        (*ctxt).vctxt.valid = 1 as std::os::raw::c_int;
        nval =
            xmlValidCtxtNormalizeAttributeValue(&mut (*ctxt).vctxt,
                                                (*ctxt).myDoc, (*ctxt).node,
                                                fullname, value);
        if (*ctxt).vctxt.valid != 1 as std::os::raw::c_int {
            (*ctxt).valid = 0 as std::os::raw::c_int
        }
        if !nval.is_null() { value = nval }
        /* LIBXML_VALID_ENABLED */
    }
    /*
     * Check whether it's a namespace definition
     */
    if (*ctxt).html == 0 && ns.is_null() &&
           *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'x' as i32 &&
           *name.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'm' as i32 &&
           *name.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'l' as i32 &&
           *name.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               'n' as i32 &&
           *name.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               's' as i32 &&
           *name.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        let mut nsret: xmlNsPtr = 0 as *mut xmlNs;
        let mut val: *mut xmlChar = 0 as *mut xmlChar;
        if (*ctxt).replaceEntities == 0 {
            (*ctxt).depth += 1;
            val =
                xmlStringDecodeEntities(ctxt, value, 1 as std::os::raw::c_int,
                                        0 as std::os::raw::c_int as xmlChar,
                                        0 as std::os::raw::c_int as xmlChar,
                                        0 as std::os::raw::c_int as xmlChar);
            (*ctxt).depth -= 1;
            if val.is_null() {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2StartElement\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                if !name.is_null() {
                    xmlFree.expect("non-null function pointer")(name as
                                                                    *mut std::os::raw::c_void);
                }
                if !nval.is_null() {
                    xmlFree.expect("non-null function pointer")(nval as
                                                                    *mut std::os::raw::c_void);
                }
                return
            }
        } else { val = value as *mut xmlChar }
        if *val.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               0 as std::os::raw::c_int {
            let mut uri: xmlURIPtr = 0 as *mut xmlURI;
            uri = xmlParseURI(val as *const std::os::raw::c_char);
            if uri.is_null() {
                if !(*ctxt).sax.is_null() && (*(*ctxt).sax).warning.is_some()
                   {
                    (*(*ctxt).sax).warning.expect("non-null function pointer")((*ctxt).userData,
                                                                               b"xmlns: %s not a valid URI\n\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const std::os::raw::c_char,
                                                                               val);
                }
            } else {
                if (*uri).scheme.is_null() {
                    if !(*ctxt).sax.is_null() &&
                           (*(*ctxt).sax).warning.is_some() {
                        (*(*ctxt).sax).warning.expect("non-null function pointer")((*ctxt).userData,
                                                                                   b"xmlns: URI %s is not absolute\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   val);
                    }
                }
                xmlFreeURI(uri);
            }
        }
        /* a default namespace definition */
        nsret = xmlNewNs((*ctxt).node, val, 0 as *const xmlChar);
        /*
	 * Validate also for namespace decls, they are attributes from
	 * an XML-1.0 perspective
	 */
        if !nsret.is_null() && (*ctxt).validate != 0 &&
               (*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null() &&
               !(*(*ctxt).myDoc).intSubset.is_null() {
            (*ctxt).valid &=
                xmlValidateOneNamespace(&mut (*ctxt).vctxt, (*ctxt).myDoc,
                                        (*ctxt).node, prefix, nsret, val)
        }
        /* LIBXML_VALID_ENABLED */
        if !name.is_null() {
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
        }
        if !nval.is_null() {
            xmlFree.expect("non-null function pointer")(nval as
                                                            *mut std::os::raw::c_void);
        }
        if val != value as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(val as
                                                            *mut std::os::raw::c_void);
        }
        return
    }
    if (*ctxt).html == 0 && !ns.is_null() &&
           *ns.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'x' as i32
           &&
           *ns.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'm' as i32
           &&
           *ns.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'l' as i32
           &&
           *ns.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'n' as i32
           &&
           *ns.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int == 's' as i32
           &&
           *ns.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
        let mut nsret_0: xmlNsPtr = 0 as *mut xmlNs;
        let mut val_0: *mut xmlChar = 0 as *mut xmlChar;
        if (*ctxt).replaceEntities == 0 {
            (*ctxt).depth += 1;
            val_0 =
                xmlStringDecodeEntities(ctxt, value, 1 as std::os::raw::c_int,
                                        0 as std::os::raw::c_int as xmlChar,
                                        0 as std::os::raw::c_int as xmlChar,
                                        0 as std::os::raw::c_int as xmlChar);
            (*ctxt).depth -= 1;
            if val_0.is_null() {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2StartElement\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(ns as
                                                                *mut std::os::raw::c_void);
                if !name.is_null() {
                    xmlFree.expect("non-null function pointer")(name as
                                                                    *mut std::os::raw::c_void);
                }
                if !nval.is_null() {
                    xmlFree.expect("non-null function pointer")(nval as
                                                                    *mut std::os::raw::c_void);
                }
                return
            }
        } else { val_0 = value as *mut xmlChar }
        if *val_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
            xmlNsErrMsg(ctxt, XML_NS_ERR_EMPTY,
                        b"Empty namespace name for prefix %s\n\x00" as
                            *const u8 as *const std::os::raw::c_char, name,
                        0 as *const xmlChar);
        }
        if (*ctxt).pedantic != 0 as std::os::raw::c_int &&
               *val_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   0 as std::os::raw::c_int {
            let mut uri_0: xmlURIPtr = 0 as *mut xmlURI;
            uri_0 = xmlParseURI(val_0 as *const std::os::raw::c_char);
            if uri_0.is_null() {
                xmlNsWarnMsg(ctxt, XML_WAR_NS_URI,
                             b"xmlns:%s: %s not a valid URI\n\x00" as
                                 *const u8 as *const std::os::raw::c_char, name,
                             value);
            } else {
                if (*uri_0).scheme.is_null() {
                    xmlNsWarnMsg(ctxt, XML_WAR_NS_URI_RELATIVE,
                                 b"xmlns:%s: URI %s is not absolute\n\x00" as
                                     *const u8 as *const std::os::raw::c_char, name,
                                 value);
                }
                xmlFreeURI(uri_0);
            }
        }
        /* a standard namespace definition */
        nsret_0 = xmlNewNs((*ctxt).node, val_0, name);
        xmlFree.expect("non-null function pointer")(ns as *mut std::os::raw::c_void);
        /*
	 * Validate also for namespace decls, they are attributes from
	 * an XML-1.0 perspective
	 */
        if !nsret_0.is_null() && (*ctxt).validate != 0 &&
               (*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null() &&
               !(*(*ctxt).myDoc).intSubset.is_null() {
            (*ctxt).valid &=
                xmlValidateOneNamespace(&mut (*ctxt).vctxt, (*ctxt).myDoc,
                                        (*ctxt).node, prefix, nsret_0, value)
        }
        /* LIBXML_VALID_ENABLED */
        if !name.is_null() {
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
        }
        if !nval.is_null() {
            xmlFree.expect("non-null function pointer")(nval as
                                                            *mut std::os::raw::c_void);
        }
        if val_0 != value as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(val_0 as
                                                            *mut std::os::raw::c_void);
        }
        return
    }
    if !ns.is_null() {
        namespace = xmlSearchNs((*ctxt).myDoc, (*ctxt).node, ns);
        if namespace.is_null() {
            xmlNsErrMsg(ctxt, XML_NS_ERR_UNDEFINED_NAMESPACE,
                        b"Namespace prefix %s of attribute %s is not defined\n\x00"
                            as *const u8 as *const std::os::raw::c_char, ns, name);
            current_block = 15460309861373144675;
        } else {
            let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
            prop = (*(*ctxt).node).properties;
            loop  {
                if prop.is_null() {
                    current_block = 15460309861373144675;
                    break ;
                }
                if !(*prop).ns.is_null() {
                    if xmlStrEqual(name, (*prop).name) != 0 &&
                           (namespace == (*prop).ns ||
                                xmlStrEqual((*namespace).href,
                                            (*(*prop).ns).href) != 0) {
                        xmlNsErrMsg(ctxt, XML_ERR_ATTRIBUTE_REDEFINED,
                                    b"Attribute %s in %s redefined\n\x00" as
                                        *const u8 as *const std::os::raw::c_char,
                                    name, (*namespace).href);
                        (*ctxt).wellFormed = 0 as std::os::raw::c_int;
                        if (*ctxt).recovery == 0 as std::os::raw::c_int {
                            (*ctxt).disableSAX = 1 as std::os::raw::c_int
                        }
                        if !name.is_null() {
                            xmlFree.expect("non-null function pointer")(name
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                        current_block = 17287191437276439284;
                        break ;
                    }
                }
                prop = (*prop).next
            }
        }
    } else {
        namespace = 0 as xmlNsPtr;
        current_block = 15460309861373144675;
    }
    match current_block {
        15460309861373144675 => {
            /* !!!!!! <a toto:arg="" xmlns:toto="http://toto.com"> */
            ret =
                xmlNewNsPropEatName((*ctxt).node, namespace, name,
                                    0 as *const xmlChar);
            if !ret.is_null() {
                if (*ctxt).replaceEntities == 0 as std::os::raw::c_int &&
                       (*ctxt).html == 0 {
                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                    (*ret).children =
                        xmlStringGetNodeList((*ctxt).myDoc as *const xmlDoc,
                                             value);
                    tmp = (*ret).children;
                    while !tmp.is_null() {
                        (*tmp).parent = ret as xmlNodePtr;
                        if (*tmp).next.is_null() { (*ret).last = tmp }
                        tmp = (*tmp).next
                    }
                } else if !value.is_null() {
                    (*ret).children =
                        xmlNewDocText((*ctxt).myDoc as *const xmlDoc, value);
                    (*ret).last = (*ret).children;
                    if !(*ret).children.is_null() {
                        (*(*ret).children).parent = ret as xmlNodePtr
                    }
                }
            }
            if (*ctxt).html == 0 && (*ctxt).validate != 0 &&
                   (*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null() &&
                   !(*(*ctxt).myDoc).intSubset.is_null() {
                /*
	 * If we don't substitute entities, the validation should be
	 * done on a value with replaced entities anyway.
	 */
                if (*ctxt).replaceEntities == 0 {
                    let mut val_1: *mut xmlChar = 0 as *mut xmlChar;
                    (*ctxt).depth += 1;
                    val_1 =
                        xmlStringDecodeEntities(ctxt, value, 1 as std::os::raw::c_int,
                                                0 as std::os::raw::c_int as xmlChar,
                                                0 as std::os::raw::c_int as xmlChar,
                                                0 as std::os::raw::c_int as xmlChar);
                    (*ctxt).depth -= 1;
                    if val_1.is_null() {
                        (*ctxt).valid &=
                            xmlValidateOneAttribute(&mut (*ctxt).vctxt,
                                                    (*ctxt).myDoc,
                                                    (*ctxt).node, ret, value)
                    } else {
                        let mut nvalnorm: *mut xmlChar = 0 as *mut xmlChar;
                        /*
		 * Do the last stage of the attribute normalization
		 * It need to be done twice ... it's an extra burden related
		 * to the ability to keep xmlSAX2References in attributes
		 */
                        nvalnorm =
                            xmlValidNormalizeAttributeValue((*ctxt).myDoc,
                                                            (*ctxt).node,
                                                            fullname, val_1);
                        if !nvalnorm.is_null() {
                            xmlFree.expect("non-null function pointer")(val_1
                                                                            as
                                                                            *mut std::os::raw::c_void);
                            val_1 = nvalnorm
                        }
                        (*ctxt).valid &=
                            xmlValidateOneAttribute(&mut (*ctxt).vctxt,
                                                    (*ctxt).myDoc,
                                                    (*ctxt).node, ret, val_1);
                        xmlFree.expect("non-null function pointer")(val_1 as
                                                                        *mut std::os::raw::c_void);
                    }
                } else {
                    (*ctxt).valid &=
                        xmlValidateOneAttribute(&mut (*ctxt).vctxt,
                                                (*ctxt).myDoc, (*ctxt).node,
                                                ret, value)
                }
            } else if (*ctxt).loadsubset & 8 as std::os::raw::c_int ==
                          0 as std::os::raw::c_int &&
                          ((*ctxt).replaceEntities == 0 as std::os::raw::c_int &&
                               (*ctxt).external != 2 as std::os::raw::c_int ||
                               (*ctxt).replaceEntities != 0 as std::os::raw::c_int &&
                                   (*ctxt).inSubset == 0 as std::os::raw::c_int) {
                /* LIBXML_VALID_ENABLED */
                /*
	 * when validating, the ID registration is done at the attribute
	 * validation level. Otherwise we have to do specific handling here.
	 */
                if xmlStrEqual(fullname,
                               b"xml:id\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
                    /*
	     * Add the xml:id value
	     *
	     * Open issue: normalization of the value.
	     */
                    if xmlValidateNCName(value, 1 as std::os::raw::c_int) !=
                           0 as std::os::raw::c_int {
                        xmlErrValid(ctxt, XML_DTD_XMLID_VALUE,
                                    b"xml:id : attribute value %s is not an NCName\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    value as *const std::os::raw::c_char,
                                    0 as *const std::os::raw::c_char);
                    }
                    xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, value, ret);
                } else if xmlIsID((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
                    xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, value, ret);
                } else if xmlIsRef((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
                    xmlAddRef(&mut (*ctxt).vctxt, (*ctxt).myDoc, value, ret);
                }
            }
        }
        _ => { }
    }
    if !nval.is_null() {
        xmlFree.expect("non-null function pointer")(nval as
                                                        *mut std::os::raw::c_void);
    }
    if !ns.is_null() {
        xmlFree.expect("non-null function pointer")(ns as *mut std::os::raw::c_void);
    };
}
/*
 * xmlCheckDefaultedAttributes:
 *
 * Check defaulted attributes from the DTD
 */
unsafe extern "C" fn xmlCheckDefaultedAttributes(mut ctxt: xmlParserCtxtPtr,
                                                 mut name: *const xmlChar,
                                                 mut prefix: *const xmlChar,
                                                 mut atts:
                                                     *mut *const xmlChar) {
    let mut elemDecl: xmlElementPtr = 0 as *mut xmlElement;
    let mut att: *const xmlChar = 0 as *const xmlChar;
    let mut internal: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    elemDecl =
        xmlGetDtdQElementDesc((*(*ctxt).myDoc).intSubset, name, prefix);
    if elemDecl.is_null() {
        elemDecl =
            xmlGetDtdQElementDesc((*(*ctxt).myDoc).extSubset, name, prefix);
        internal = 0 as std::os::raw::c_int
    }
    while !elemDecl.is_null() {
        let mut attr: xmlAttributePtr = (*elemDecl).attributes;
        /*
	 * Check against defaulted attributes from the external subset
	 * if the document is stamped as standalone
	 */
        if (*(*ctxt).myDoc).standalone == 1 as std::os::raw::c_int &&
               !(*(*ctxt).myDoc).extSubset.is_null() && (*ctxt).validate != 0
           {
            while !attr.is_null() {
                if !(*attr).defaultValue.is_null() &&
                       xmlGetDtdQAttrDesc((*(*ctxt).myDoc).extSubset,
                                          (*attr).elem, (*attr).name,
                                          (*attr).prefix) == attr &&
                       xmlGetDtdQAttrDesc((*(*ctxt).myDoc).intSubset,
                                          (*attr).elem, (*attr).name,
                                          (*attr).prefix).is_null() {
                    let mut fulln: *mut xmlChar = 0 as *mut xmlChar;
                    if !(*attr).prefix.is_null() {
                        fulln = xmlStrdup((*attr).prefix);
                        fulln =
                            xmlStrcat(fulln,
                                      b":\x00" as *const u8 as
                                          *const std::os::raw::c_char as
                                          *mut xmlChar);
                        fulln = xmlStrcat(fulln, (*attr).name)
                    } else { fulln = xmlStrdup((*attr).name) }
                    if fulln.is_null() {
                        xmlSAX2ErrMemory(ctxt,
                                         b"xmlSAX2StartElement\x00" as
                                             *const u8 as
                                             *const std::os::raw::c_char);
                        break ;
                    } else {
                        /*
		     * Check that the attribute is not declared in the
		     * serialization
		     */
                        att = 0 as *const xmlChar;
                        if !atts.is_null() {
                            i = 0 as std::os::raw::c_int;
                            att = *atts.offset(i as isize);
                            while !att.is_null() {
                                if xmlStrEqual(att, fulln) != 0 { break ; }
                                i += 2 as std::os::raw::c_int;
                                att = *atts.offset(i as isize)
                            }
                        }
                        if att.is_null() {
                            xmlErrValid(ctxt, XML_DTD_STANDALONE_DEFAULTED,
                                        b"standalone: attribute %s on %s defaulted from external subset\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char,
                                        fulln as *const std::os::raw::c_char,
                                        (*attr).elem as *const std::os::raw::c_char);
                        }
                        xmlFree.expect("non-null function pointer")(fulln as
                                                                        *mut std::os::raw::c_void);
                    }
                }
                attr = (*attr).nexth
            }
        }
        /*
	 * Actually insert defaulted values when needed
	 */
        attr = (*elemDecl).attributes;
        while !attr.is_null() {
            /*
	     * Make sure that attributes redefinition occuring in the
	     * internal subset are not overriden by definitions in the
	     * external subset.
	     */
            if !(*attr).defaultValue.is_null() {
                /*
		 * the element should be instantiated in the tree if:
		 *  - this is a namespace prefix
		 *  - the user required for completion in the tree
		 *    like XSLT
		 *  - there isn't already an attribute definition
		 *    in the internal subset overriding it.
		 */
                if !(*attr).prefix.is_null() &&
                       xmlStrEqual((*attr).prefix,
                                   b"xmlns\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) !=
                           0 ||
                       (*attr).prefix.is_null() &&
                           xmlStrEqual((*attr).name,
                                       b"xmlns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 ||
                       (*ctxt).loadsubset & 4 as std::os::raw::c_int != 0 {
                    let mut tst: xmlAttributePtr = 0 as *mut xmlAttribute;
                    tst =
                        xmlGetDtdQAttrDesc((*(*ctxt).myDoc).intSubset,
                                           (*attr).elem, (*attr).name,
                                           (*attr).prefix);
                    if tst == attr || tst.is_null() {
                        let mut fn_0: [xmlChar; 50] = [0; 50];
                        let mut fulln_0: *mut xmlChar = 0 as *mut xmlChar;
                        fulln_0 =
                            xmlBuildQName((*attr).name, (*attr).prefix,
                                          fn_0.as_mut_ptr(),
                                          50 as std::os::raw::c_int);
                        if fulln_0.is_null() {
                            xmlSAX2ErrMemory(ctxt,
                                             b"xmlSAX2StartElement\x00" as
                                                 *const u8 as
                                                 *const std::os::raw::c_char);
                            return
                        }
                        /*
			 * Check that the attribute is not declared in the
			 * serialization
			 */
                        att = 0 as *const xmlChar;
                        if !atts.is_null() {
                            i = 0 as std::os::raw::c_int;
                            att = *atts.offset(i as isize);
                            while !att.is_null() {
                                if xmlStrEqual(att, fulln_0) != 0 { break ; }
                                i += 2 as std::os::raw::c_int;
                                att = *atts.offset(i as isize)
                            }
                        }
                        if att.is_null() {
                            xmlSAX2AttributeInternal(ctxt as
                                                         *mut std::os::raw::c_void,
                                                     fulln_0,
                                                     (*attr).defaultValue,
                                                     prefix);
                        }
                        if fulln_0 != fn_0.as_mut_ptr() &&
                               fulln_0 != (*attr).name as *mut xmlChar {
                            xmlFree.expect("non-null function pointer")(fulln_0
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    }
                }
            }
            attr = (*attr).nexth
        }
        if !(internal == 1 as std::os::raw::c_int) { break ; }
        elemDecl =
            xmlGetDtdQElementDesc((*(*ctxt).myDoc).extSubset, name, prefix);
        internal = 0 as std::os::raw::c_int
    };
}
/* *
 * xmlSAX2StartElement:
 * @ctx: the user data (XML parser context)
 * @fullname:  The element name, including namespace prefix
 * @atts:  An array of name/value attributes pairs, NULL terminated
 *
 * called when an opening tag has been processed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartElement(mut ctx: *mut std::os::raw::c_void,
                                             mut fullname: *const xmlChar,
                                             mut atts: *mut *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut att: *const xmlChar = 0 as *const xmlChar;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    let mut i: std::os::raw::c_int = 0;
    if ctx.is_null() || fullname.is_null() || (*ctxt).myDoc.is_null() {
        return
    }
    parent = (*ctxt).node;
    /*
     * First check on validity:
     */
    if (*ctxt).validate != 0 && (*(*ctxt).myDoc).extSubset.is_null() &&
           ((*(*ctxt).myDoc).intSubset.is_null() ||
                (*(*(*ctxt).myDoc).intSubset).notations.is_null() &&
                    (*(*(*ctxt).myDoc).intSubset).elements.is_null() &&
                    (*(*(*ctxt).myDoc).intSubset).attributes.is_null() &&
                    (*(*(*ctxt).myDoc).intSubset).entities.is_null()) {
        xmlErrValid(ctxt, XML_ERR_NO_DTD,
                    b"Validation failed: no DTD found !\x00" as *const u8 as
                        *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char);
        (*ctxt).validate = 0 as std::os::raw::c_int
    }
    /*
     * Split the full name into a namespace prefix and the tag name
     */
    name = xmlSplitQName(ctxt, fullname, &mut prefix);
    /*
     * Note : the namespace resolution is deferred until the end of the
     *        attributes parsing, since local namespace can be defined as
     *        an attribute at this level.
     */
    ret =
        xmlNewDocNodeEatName((*ctxt).myDoc, 0 as xmlNsPtr, name,
                             0 as *const xmlChar);
    if ret.is_null() {
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
        }
        xmlSAX2ErrMemory(ctxt,
                         b"xmlSAX2StartElement\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return
    }
    if (*(*ctxt).myDoc).children.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
    } else if parent.is_null() { parent = (*(*ctxt).myDoc).children }
    (*ctxt).nodemem = -(1 as std::os::raw::c_int);
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if (*(*ctxt).input).line < 65535 as std::os::raw::c_int {
                (*ret).line =
                    (*(*ctxt).input).line as std::os::raw::c_short as std::os::raw::c_ushort
            } else { (*ret).line = 65535 as std::os::raw::c_int as std::os::raw::c_ushort }
        }
    }
    /*
     * We are parsing a new node.
     */
    nodePush(ctxt, ret);
    /*
     * Link the child element
     */
    if !parent.is_null() {
        if (*parent).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            xmlAddChild(parent, ret);
        } else { xmlAddSibling(parent, ret); }
    }
    /*
     * Insert all the defaulted attributes from the DTD especially namespaces
     */
    if (*ctxt).html == 0 &&
           (!(*(*ctxt).myDoc).intSubset.is_null() ||
                !(*(*ctxt).myDoc).extSubset.is_null()) {
        xmlCheckDefaultedAttributes(ctxt, name, prefix, atts);
    }
    /*
     * process all the attributes whose name start with "xmlns"
     */
    if !atts.is_null() {
        i = 0 as std::os::raw::c_int;
        let fresh0 = i;
        i = i + 1;
        att = *atts.offset(fresh0 as isize);
        let fresh1 = i;
        i = i + 1;
        value = *atts.offset(fresh1 as isize);
        if (*ctxt).html == 0 {
            while !att.is_null() && !value.is_null() {
                if *att.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'x' as i32 &&
                       *att.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 'm' as i32 &&
                       *att.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 'l' as i32 &&
                       *att.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 'n' as i32 &&
                       *att.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 's' as i32 {
                    xmlSAX2AttributeInternal(ctxt as *mut std::os::raw::c_void, att,
                                             value, prefix);
                }
                let fresh2 = i;
                i = i + 1;
                att = *atts.offset(fresh2 as isize);
                let fresh3 = i;
                i = i + 1;
                value = *atts.offset(fresh3 as isize)
            }
        }
    }
    /*
     * Search the namespace, note that since the attributes have been
     * processed, the local namespaces are available.
     */
    ns = xmlSearchNs((*ctxt).myDoc, ret, prefix);
    if ns.is_null() && !parent.is_null() {
        ns = xmlSearchNs((*ctxt).myDoc, parent, prefix)
    }
    if !prefix.is_null() && ns.is_null() {
        ns = xmlNewNs(ret, 0 as *const xmlChar, prefix);
        xmlNsWarnMsg(ctxt, XML_NS_ERR_UNDEFINED_NAMESPACE,
                     b"Namespace prefix %s is not defined\n\x00" as *const u8
                         as *const std::os::raw::c_char, prefix, 0 as *const xmlChar);
    }
    /*
     * set the namespace node, making sure that if the default namspace
     * is unbound on a parent we simply kee it NULL
     */
    if !ns.is_null() && !(*ns).href.is_null() &&
           (*(*ns).href.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                0 as std::os::raw::c_int || !(*ns).prefix.is_null()) {
        xmlSetNs(ret, ns);
    }
    /*
     * process all the other attributes
     */
    if !atts.is_null() {
        i = 0 as std::os::raw::c_int;
        let fresh4 = i;
        i = i + 1;
        att = *atts.offset(fresh4 as isize);
        let fresh5 = i;
        i = i + 1;
        value = *atts.offset(fresh5 as isize);
        if (*ctxt).html != 0 {
            while !att.is_null() {
                xmlSAX2AttributeInternal(ctxt as *mut std::os::raw::c_void, att,
                                         value, 0 as *const xmlChar);
                let fresh6 = i;
                i = i + 1;
                att = *atts.offset(fresh6 as isize);
                let fresh7 = i;
                i = i + 1;
                value = *atts.offset(fresh7 as isize)
            }
        } else {
            while !att.is_null() && !value.is_null() {
                if *att.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                       'x' as i32 ||
                       *att.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           != 'm' as i32 ||
                       *att.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           != 'l' as i32 ||
                       *att.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           != 'n' as i32 ||
                       *att.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           != 's' as i32 {
                    xmlSAX2AttributeInternal(ctxt as *mut std::os::raw::c_void, att,
                                             value, 0 as *const xmlChar);
                }
                /*
		 * Next ones
		 */
                let fresh8 = i;
                i = i + 1;
                att = *atts.offset(fresh8 as isize);
                let fresh9 = i;
                i = i + 1;
                value = *atts.offset(fresh9 as isize)
            }
        }
    }
    /*
     * If it's the Document root, finish the DTD validation and
     * check the document root element for validity
     */
    if (*ctxt).validate != 0 &&
           (*ctxt).vctxt.finishDtd == 0xabcd1234 as std::os::raw::c_uint {
        let mut chk: std::os::raw::c_int = 0;
        chk = xmlValidateDtdFinal(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        if chk <= 0 as std::os::raw::c_int { (*ctxt).valid = 0 as std::os::raw::c_int }
        if chk < 0 as std::os::raw::c_int { (*ctxt).wellFormed = 0 as std::os::raw::c_int }
        (*ctxt).valid &= xmlValidateRoot(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        (*ctxt).vctxt.finishDtd = 0xabcd1235 as std::os::raw::c_uint
    }
    /* LIBXML_VALID_ENABLED */
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlSAX2EndElement:
 * @ctx: the user data (XML parser context)
 * @name:  The element name
 *
 * called when the end of an element has been detected.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndElement(mut ctx: *mut std::os::raw::c_void,
                                           mut name: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    cur = (*ctxt).node;
    /* Capture end position and add node */
    if !cur.is_null() && (*ctxt).record_info != 0 {
        (*(*ctxt).nodeInfo).end_pos =
            (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                as std::os::raw::c_long as std::os::raw::c_ulong;
        (*(*ctxt).nodeInfo).end_line = (*(*ctxt).input).line as std::os::raw::c_ulong;
        (*(*ctxt).nodeInfo).node = cur as *const _xmlNode;
        xmlParserAddNodeInfo(ctxt, (*ctxt).nodeInfo);
    }
    (*ctxt).nodemem = -(1 as std::os::raw::c_int);
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 &&
           !(*ctxt).myDoc.is_null() && !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &=
            xmlValidateOneElement(&mut (*ctxt).vctxt, (*ctxt).myDoc, cur)
    }
    /* LIBXML_VALID_ENABLED */
    /*
     * end of parsing of this node.
     */
    nodePop(ctxt);
}
/* LIBXML_SAX1_ENABLED || LIBXML_HTML_ENABLED || LIBXML_LEGACY_ENABLED */
/*
 * xmlSAX2TextNode:
 * @ctxt:  the parser context
 * @str:  the input string
 * @len: the string length
 *
 * Callback for a text node
 *
 * Returns the newly allocated string or NULL if not needed or error
 */
unsafe extern "C" fn xmlSAX2TextNode(mut ctxt: xmlParserCtxtPtr,
                                     mut str: *const xmlChar,
                                     mut len: std::os::raw::c_int) -> xmlNodePtr {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut intern: *const xmlChar = 0 as *const xmlChar;
    /*
     * Allocate
     */
    if !(*ctxt).freeElems.is_null() {
        ret = (*ctxt).freeElems;
        (*ctxt).freeElems = (*ret).next;
        (*ctxt).freeElemsNr -= 1
    } else {
        ret =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlNode>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlNodePtr
    }
    if ret.is_null() {
        xmlErrMemory(ctxt,
                     b"xmlSAX2Characters\x00" as *const u8 as
                         *const std::os::raw::c_char);
        return 0 as xmlNodePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
    /*
     * intern the formatting blanks found between tags, or the
     * very short strings
     */
    if (*ctxt).dictNames != 0 {
        let mut cur: xmlChar = *str.offset(len as isize);
        if len <
               (2 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut std::os::raw::c_void>()
                                                    as std::os::raw::c_ulong) as
                   std::os::raw::c_int &&
               (*ctxt).options & XML_PARSE_COMPACT as std::os::raw::c_int != 0 {
            /* store the string in the node overriding properties and nsDef */
            let mut tmp: *mut xmlChar =
                &mut (*ret).properties as *mut *mut _xmlAttr as *mut xmlChar;
            memcpy(tmp as *mut std::os::raw::c_void, str as *const std::os::raw::c_void,
                   len as std::os::raw::c_ulong);
            *tmp.offset(len as isize) = 0 as std::os::raw::c_int as xmlChar;
            intern = tmp
        } else if len <= 3 as std::os::raw::c_int &&
                      (cur as std::os::raw::c_int == '\"' as i32 ||
                           cur as std::os::raw::c_int == '\'' as i32 ||
                           cur as std::os::raw::c_int == '<' as i32 &&
                               *str.offset((len + 1 as std::os::raw::c_int) as isize)
                                   as std::os::raw::c_int != '!' as i32) {
            intern = xmlDictLookup((*ctxt).dict, str, len)
        } else if (*str as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= *str as std::os::raw::c_int &&
                           *str as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                       *str as std::os::raw::c_int == 0xd as std::os::raw::c_int) &&
                      len < 60 as std::os::raw::c_int &&
                      cur as std::os::raw::c_int == '<' as i32 &&
                      *str.offset((len + 1 as std::os::raw::c_int) as isize) as
                          std::os::raw::c_int != '!' as i32 {
            let mut i: std::os::raw::c_int = 0;
            i = 1 as std::os::raw::c_int;
            loop  {
                if !(i < len) {
                    current_block = 17478428563724192186;
                    break ;
                }
                if !(*str.offset(i as isize) as std::os::raw::c_int ==
                         0x20 as std::os::raw::c_int ||
                         0x9 as std::os::raw::c_int <=
                             *str.offset(i as isize) as std::os::raw::c_int &&
                             *str.offset(i as isize) as std::os::raw::c_int <=
                                 0xa as std::os::raw::c_int ||
                         *str.offset(i as isize) as std::os::raw::c_int ==
                             0xd as std::os::raw::c_int) {
                    current_block = 1213398872049742616;
                    break ;
                }
                i += 1
            }
            match current_block {
                1213398872049742616 => { }
                _ => { intern = xmlDictLookup((*ctxt).dict, str, len) }
            }
        }
    }
    (*ret).type_0 = XML_TEXT_NODE;
    (*ret).name = xmlStringText.as_ptr();
    if intern.is_null() {
        (*ret).content = xmlStrndup(str, len);
        if (*ret).content.is_null() {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2TextNode\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlNodePtr
        }
    } else { (*ret).content = intern as *mut xmlChar }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if (*(*ctxt).input).line < 65535 as std::os::raw::c_int {
                (*ret).line =
                    (*(*ctxt).input).line as std::os::raw::c_short as std::os::raw::c_ushort
            } else {
                (*ret).line = 65535 as std::os::raw::c_int as std::os::raw::c_ushort;
                if (*ctxt).options & XML_PARSE_BIG_LINES as std::os::raw::c_int != 0 {
                    (*ret).psvi =
                        (*(*ctxt).input).line as ptrdiff_t as
                            *mut std::os::raw::c_void
                }
            }
        }
    }
    if __xmlRegisterCallbacks != 0 &&
           (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
    }
    return ret;
}
/*
 * xmlSAX2DecodeAttrEntities:
 * @ctxt:  the parser context
 * @str:  the input string
 * @len: the string length
 *
 * Remove the entities from an attribute value
 *
 * Returns the newly allocated string or NULL if not needed or error
 */
unsafe extern "C" fn xmlSAX2DecodeAttrEntities(mut ctxt: xmlParserCtxtPtr,
                                               mut str: *const xmlChar,
                                               mut end: *const xmlChar)
 -> *mut xmlChar {
    let mut current_block: u64;
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    in_0 = str;
    loop  {
        if !(in_0 < end) { current_block = 17778012151635330486; break ; }
        let fresh10 = in_0;
        in_0 = in_0.offset(1);
        if *fresh10 as std::os::raw::c_int == '&' as i32 {
            current_block = 687054393598216150;
            break ;
        }
    }
    match current_block {
        17778012151635330486 => { return 0 as *mut xmlChar }
        _ => {
            (*ctxt).depth += 1;
            ret =
                xmlStringLenDecodeEntities(ctxt, str,
                                           end.offset_from(str) as
                                               std::os::raw::c_long as std::os::raw::c_int,
                                           1 as std::os::raw::c_int,
                                           0 as std::os::raw::c_int as xmlChar,
                                           0 as std::os::raw::c_int as xmlChar,
                                           0 as std::os::raw::c_int as xmlChar);
            (*ctxt).depth -= 1;
            return ret
        }
    };
}
/* LIBXML_VALID_ENABLED */
/* *
 * xmlSAX2AttributeNs:
 * @ctx: the user data (XML parser context)
 * @localname:  the local name of the attribute
 * @prefix:  the attribute namespace prefix if available
 * @URI:  the attribute namespace name if available
 * @value:  Start of the attribute value
 * @valueend: end of the attribute value
 *
 * Handle an attribute that has been read by the parser.
 * The default handling is to convert the attribute into an
 * DOM subtree and past it in a new xmlAttr element added to
 * the element.
 */
unsafe extern "C" fn xmlSAX2AttributeNs(mut ctxt: xmlParserCtxtPtr,
                                        mut localname: *const xmlChar,
                                        mut prefix: *const xmlChar,
                                        mut value: *const xmlChar,
                                        mut valueend: *const xmlChar) {
    let mut ret: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut namespace: xmlNsPtr = 0 as xmlNsPtr;
    let mut dup: *mut xmlChar = 0 as *mut xmlChar;
    /*
     * Note: if prefix == NULL, the attribute is not in the default namespace
     */
    if !prefix.is_null() {
        namespace = xmlSearchNs((*ctxt).myDoc, (*ctxt).node, prefix)
    }
    /*
     * allocate the node
     */
    if !(*ctxt).freeAttrs.is_null() {
        ret = (*ctxt).freeAttrs;
        (*ctxt).freeAttrs = (*ret).next;
        (*ctxt).freeAttrsNr -= 1;
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlAttr>() as std::os::raw::c_ulong);
        (*ret).type_0 = XML_ATTRIBUTE_NODE;
        (*ret).parent = (*ctxt).node;
        (*ret).doc = (*ctxt).myDoc;
        (*ret).ns = namespace;
        if (*ctxt).dictNames != 0 {
            (*ret).name = localname
        } else { (*ret).name = xmlStrdup(localname) }
        /* link at the end to preserv order, TODO speed up with a last */
        if (*(*ctxt).node).properties.is_null() {
            (*(*ctxt).node).properties = ret
        } else {
            let mut prev: xmlAttrPtr = (*(*ctxt).node).properties;
            while !(*prev).next.is_null() { prev = (*prev).next }
            (*prev).next = ret;
            (*ret).prev = prev
        }
        if __xmlRegisterCallbacks != 0 &&
               (*__xmlRegisterNodeDefaultValue()).is_some() {
            (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret
                                                                                       as
                                                                                       xmlNodePtr);
        }
    } else {
        if (*ctxt).dictNames != 0 {
            ret =
                xmlNewNsPropEatName((*ctxt).node, namespace,
                                    localname as *mut xmlChar,
                                    0 as *const xmlChar)
        } else {
            ret =
                xmlNewNsProp((*ctxt).node, namespace, localname,
                             0 as *const xmlChar)
        }
        if ret.is_null() {
            xmlErrMemory(ctxt,
                         b"xmlSAX2AttributeNs\x00" as *const u8 as
                             *const std::os::raw::c_char);
            return
        }
    }
    if (*ctxt).replaceEntities == 0 as std::os::raw::c_int && (*ctxt).html == 0 {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        /*
	 * We know that if there is an entity reference, then
	 * the string has been dup'ed and terminates with 0
	 * otherwise with ' or "
	 */
        if *valueend as std::os::raw::c_int != 0 as std::os::raw::c_int {
            tmp =
                xmlSAX2TextNode(ctxt, value,
                                valueend.offset_from(value) as
                                    std::os::raw::c_long as std::os::raw::c_int);
            (*ret).children = tmp;
            (*ret).last = tmp;
            if !tmp.is_null() {
                (*tmp).doc = (*ret).doc;
                (*tmp).parent = ret as xmlNodePtr
            }
        } else {
            (*ret).children =
                xmlStringLenGetNodeList((*ctxt).myDoc as *const xmlDoc, value,
                                        valueend.offset_from(value)
                                            as std::os::raw::c_long as std::os::raw::c_int);
            tmp = (*ret).children;
            while !tmp.is_null() {
                (*tmp).doc = (*ret).doc;
                (*tmp).parent = ret as xmlNodePtr;
                if (*tmp).next.is_null() { (*ret).last = tmp }
                tmp = (*tmp).next
            }
        }
    } else if !value.is_null() {
        let mut tmp_0: xmlNodePtr = 0 as *mut xmlNode;
        tmp_0 =
            xmlSAX2TextNode(ctxt, value,
                            valueend.offset_from(value) as
                                std::os::raw::c_long as std::os::raw::c_int);
        (*ret).children = tmp_0;
        (*ret).last = tmp_0;
        if !tmp_0.is_null() {
            (*tmp_0).doc = (*ret).doc;
            (*tmp_0).parent = ret as xmlNodePtr
        }
    }
    if (*ctxt).html == 0 && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
           && !(*ctxt).myDoc.is_null() &&
           !(*(*ctxt).myDoc).intSubset.is_null() {
        /*
	 * If we don't substitute entities, the validation should be
	 * done on a value with replaced entities anyway.
	 */
        if (*ctxt).replaceEntities == 0 {
            dup = xmlSAX2DecodeAttrEntities(ctxt, value, valueend);
            if dup.is_null() {
                if *valueend as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    (*ctxt).valid &=
                        xmlValidateOneAttribute(&mut (*ctxt).vctxt,
                                                (*ctxt).myDoc, (*ctxt).node,
                                                ret, value)
                } else {
                    /*
		     * That should already be normalized.
		     * cheaper to finally allocate here than duplicate
		     * entry points in the full validation code
		     */
                    dup =
                        xmlStrndup(value,
                                   valueend.offset_from(value) as
                                       std::os::raw::c_long as std::os::raw::c_int);
                    (*ctxt).valid &=
                        xmlValidateOneAttribute(&mut (*ctxt).vctxt,
                                                (*ctxt).myDoc, (*ctxt).node,
                                                ret, dup)
                }
            } else {
                /*
		 * dup now contains a string of the flattened attribute
		 * content with entities substitued. Check if we need to
		 * apply an extra layer of normalization.
		 * It need to be done twice ... it's an extra burden related
		 * to the ability to keep references in attributes
		 */
                if !(*ctxt).attsSpecial.is_null() {
                    let mut nvalnorm: *mut xmlChar = 0 as *mut xmlChar;
                    let mut fn_0: [xmlChar; 50] = [0; 50];
                    let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
                    fullname =
                        xmlBuildQName(localname, prefix, fn_0.as_mut_ptr(),
                                      50 as std::os::raw::c_int);
                    if !fullname.is_null() {
                        (*ctxt).vctxt.valid = 1 as std::os::raw::c_int;
                        nvalnorm =
                            xmlValidCtxtNormalizeAttributeValue(&mut (*ctxt).vctxt,
                                                                (*ctxt).myDoc,
                                                                (*ctxt).node,
                                                                fullname,
                                                                dup);
                        if (*ctxt).vctxt.valid != 1 as std::os::raw::c_int {
                            (*ctxt).valid = 0 as std::os::raw::c_int
                        }
                        if fullname != fn_0.as_mut_ptr() &&
                               fullname != localname as *mut xmlChar {
                            xmlFree.expect("non-null function pointer")(fullname
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                        if !nvalnorm.is_null() {
                            xmlFree.expect("non-null function pointer")(dup as
                                                                            *mut std::os::raw::c_void);
                            dup = nvalnorm
                        }
                    }
                }
                (*ctxt).valid &=
                    xmlValidateOneAttribute(&mut (*ctxt).vctxt, (*ctxt).myDoc,
                                            (*ctxt).node, ret, dup)
            }
        } else {
            /*
	     * if entities already have been substitued, then
	     * the attribute as passed is already normalized
	     */
            dup =
                xmlStrndup(value,
                           valueend.offset_from(value) as
                               std::os::raw::c_long as std::os::raw::c_int);
            (*ctxt).valid &=
                xmlValidateOneAttribute(&mut (*ctxt).vctxt, (*ctxt).myDoc,
                                        (*ctxt).node, ret, dup)
        }
    } else if (*ctxt).loadsubset & 8 as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                  ((*ctxt).replaceEntities == 0 as std::os::raw::c_int &&
                       (*ctxt).external != 2 as std::os::raw::c_int ||
                       (*ctxt).replaceEntities != 0 as std::os::raw::c_int &&
                           (*ctxt).inSubset == 0 as std::os::raw::c_int) {
        /* LIBXML_VALID_ENABLED */
        /*
	 * when validating, the ID registration is done at the attribute
	 * validation level. Otherwise we have to do specific handling here.
	 */
        if prefix == (*ctxt).str_xml &&
               *localname.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   'i' as i32 &&
               *localname.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   'd' as i32 &&
               *localname.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
            /*
	     * Add the xml:id value
	     *
	     * Open issue: normalization of the value.
	     */
            if dup.is_null() {
                dup =
                    xmlStrndup(value,
                               valueend.offset_from(value) as
                                   std::os::raw::c_long as std::os::raw::c_int)
            }
            if xmlValidateNCName(dup, 1 as std::os::raw::c_int) != 0 as std::os::raw::c_int {
                xmlErrValid(ctxt, XML_DTD_XMLID_VALUE,
                            b"xml:id : attribute value %s is not an NCName\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            dup as *const std::os::raw::c_char,
                            0 as *const std::os::raw::c_char);
            }
            xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, dup, ret);
        } else if xmlIsID((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
            /* might be worth duplicate entry points and not copy */
            if dup.is_null() {
                dup =
                    xmlStrndup(value,
                               valueend.offset_from(value) as
                                   std::os::raw::c_long as std::os::raw::c_int)
            }
            xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, dup, ret);
        } else if xmlIsRef((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
            if dup.is_null() {
                dup =
                    xmlStrndup(value,
                               valueend.offset_from(value) as
                                   std::os::raw::c_long as std::os::raw::c_int)
            }
            xmlAddRef(&mut (*ctxt).vctxt, (*ctxt).myDoc, dup, ret);
        }
    }
    if !dup.is_null() {
        xmlFree.expect("non-null function pointer")(dup as *mut std::os::raw::c_void);
    };
}
/* *
 * xmlSAX2StartElementNs:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 * @nb_namespaces:  number of namespace definitions on that node
 * @namespaces:  pointer to the array of prefix/URI pairs namespace definitions
 * @nb_attributes:  the number of attributes on that node
 * @nb_defaulted:  the number of defaulted attributes.
 * @attributes:  pointer to the array of (localname/prefix/URI/value/end)
 *               attribute values.
 *
 * SAX2 callback when an element start has been detected by the parser.
 * It provides the namespace informations for the element, as well as
 * the new namespace declarations on the element.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartElementNs(mut ctx: *mut std::os::raw::c_void,
                                               mut localname: *const xmlChar,
                                               mut prefix: *const xmlChar,
                                               mut URI: *const xmlChar,
                                               mut nb_namespaces: std::os::raw::c_int,
                                               mut namespaces:
                                                   *mut *const xmlChar,
                                               mut nb_attributes: std::os::raw::c_int,
                                               mut nb_defaulted: std::os::raw::c_int,
                                               mut attributes:
                                                   *mut *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    let mut last: xmlNsPtr = 0 as xmlNsPtr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut uri: *const xmlChar = 0 as *const xmlChar;
    let mut pref: *const xmlChar = 0 as *const xmlChar;
    let mut lname: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    if ctx.is_null() { return }
    parent = (*ctxt).node;
    /*
     * First check on validity:
     */
    if (*ctxt).validate != 0 && (*(*ctxt).myDoc).extSubset.is_null() &&
           ((*(*ctxt).myDoc).intSubset.is_null() ||
                (*(*(*ctxt).myDoc).intSubset).notations.is_null() &&
                    (*(*(*ctxt).myDoc).intSubset).elements.is_null() &&
                    (*(*(*ctxt).myDoc).intSubset).attributes.is_null() &&
                    (*(*(*ctxt).myDoc).intSubset).entities.is_null()) {
        xmlErrValid(ctxt, XML_DTD_NO_DTD,
                    b"Validation failed: no DTD found !\x00" as *const u8 as
                        *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char);
        (*ctxt).validate = 0 as std::os::raw::c_int
    }
    /*
     * Take care of the rare case of an undefined namespace prefix
     */
    if !prefix.is_null() && URI.is_null() {
        if (*ctxt).dictNames != 0 {
            let mut fullname: *const xmlChar = 0 as *const xmlChar;
            fullname = xmlDictQLookup((*ctxt).dict, prefix, localname);
            if !fullname.is_null() { localname = fullname }
        } else {
            lname =
                xmlBuildQName(localname, prefix, 0 as *mut xmlChar,
                              0 as std::os::raw::c_int)
        }
    }
    /*
     * allocate the node
     */
    if !(*ctxt).freeElems.is_null() {
        ret = (*ctxt).freeElems;
        (*ctxt).freeElems = (*ret).next;
        (*ctxt).freeElemsNr -= 1;
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlNode>() as std::os::raw::c_ulong);
        (*ret).type_0 = XML_ELEMENT_NODE;
        if (*ctxt).dictNames != 0 {
            (*ret).name = localname
        } else {
            if lname.is_null() {
                (*ret).name = xmlStrdup(localname)
            } else { (*ret).name = lname }
            if (*ret).name.is_null() {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2StartElementNs\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return
            }
        }
        if __xmlRegisterCallbacks != 0 &&
               (*__xmlRegisterNodeDefaultValue()).is_some() {
            (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
        }
    } else {
        if (*ctxt).dictNames != 0 {
            ret =
                xmlNewDocNodeEatName((*ctxt).myDoc, 0 as xmlNsPtr,
                                     localname as *mut xmlChar,
                                     0 as *const xmlChar)
        } else if lname.is_null() {
            ret =
                xmlNewDocNode((*ctxt).myDoc, 0 as xmlNsPtr, localname,
                              0 as *const xmlChar)
        } else {
            ret =
                xmlNewDocNodeEatName((*ctxt).myDoc, 0 as xmlNsPtr, lname,
                                     0 as *const xmlChar)
        }
        if ret.is_null() {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2StartElementNs\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
    }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if (*(*ctxt).input).line < 65535 as std::os::raw::c_int {
                (*ret).line =
                    (*(*ctxt).input).line as std::os::raw::c_short as std::os::raw::c_ushort
            } else { (*ret).line = 65535 as std::os::raw::c_int as std::os::raw::c_ushort }
        }
    }
    if parent.is_null() { xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret); }
    /*
     * Build the namespace list
     */
    i = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    while j < nb_namespaces {
        let fresh11 = i;
        i = i + 1;
        pref = *namespaces.offset(fresh11 as isize);
        let fresh12 = i;
        i = i + 1;
        uri = *namespaces.offset(fresh12 as isize);
        ns = xmlNewNs(0 as xmlNodePtr, uri, pref);
        if !ns.is_null() {
            if last.is_null() {
                last = ns;
                (*ret).nsDef = last
            } else { (*last).next = ns; last = ns }
            if !URI.is_null() && prefix == pref { (*ret).ns = ns }
            if (*ctxt).html == 0 && (*ctxt).validate != 0 &&
                   (*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null() &&
                   !(*(*ctxt).myDoc).intSubset.is_null() {
                (*ctxt).valid &=
                    xmlValidateOneNamespace(&mut (*ctxt).vctxt, (*ctxt).myDoc,
                                            ret, prefix, ns, uri)
            }
        }
        /* LIBXML_VALID_ENABLED */
        j += 1
    }
    (*ctxt).nodemem = -(1 as std::os::raw::c_int);
    /*
             * any out of memory error would already have been raised
             * but we can't be guaranteed it's the actual error due to the
             * API, best is to skip in this case
             */
    /*
     * We are parsing a new node.
     */
    nodePush(ctxt, ret);
    /*
     * Link the child element
     */
    if !parent.is_null() {
        if (*parent).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            xmlAddChild(parent, ret);
        } else { xmlAddSibling(parent, ret); }
    }
    /*
     * Insert the defaulted attributes from the DTD only if requested:
     */
    if nb_defaulted != 0 as std::os::raw::c_int &&
           (*ctxt).loadsubset & 4 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        nb_attributes -= nb_defaulted
    }
    /*
     * Search the namespace if it wasn't already found
     * Note that, if prefix is NULL, this searches for the default Ns
     */
    if !URI.is_null() && (*ret).ns.is_null() {
        (*ret).ns = xmlSearchNs((*ctxt).myDoc, parent, prefix);
        if (*ret).ns.is_null() &&
               xmlStrEqual(prefix,
                           b"xml\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) != 0 {
            (*ret).ns = xmlSearchNs((*ctxt).myDoc, ret, prefix)
        }
        if (*ret).ns.is_null() {
            ns = xmlNewNs(ret, 0 as *const xmlChar, prefix);
            if ns.is_null() {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2StartElementNs\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return
            }
            if !prefix.is_null() {
                xmlNsWarnMsg(ctxt, XML_NS_ERR_UNDEFINED_NAMESPACE,
                             b"Namespace prefix %s was not found\n\x00" as
                                 *const u8 as *const std::os::raw::c_char, prefix,
                             0 as *const xmlChar);
            } else {
                xmlNsWarnMsg(ctxt, XML_NS_ERR_UNDEFINED_NAMESPACE,
                             b"Namespace default prefix was not found\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             0 as *const xmlChar, 0 as *const xmlChar);
            }
        }
    }
    /*
     * process all the other attributes
     */
    if nb_attributes > 0 as std::os::raw::c_int {
        let mut current_block_105: u64;
        j = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < nb_attributes {
            /*
	     * Handle the rare case of an undefined atribute prefix
	     */
            if !(*attributes.offset((j + 1 as std::os::raw::c_int) as
                                        isize)).is_null() &&
                   (*attributes.offset((j + 2 as std::os::raw::c_int) as
                                           isize)).is_null() {
                if (*ctxt).dictNames != 0 {
                    let mut fullname_0: *const xmlChar = 0 as *const xmlChar;
                    fullname_0 =
                        xmlDictQLookup((*ctxt).dict,
                                       *attributes.offset((j +
                                                               1 as
                                                                   std::os::raw::c_int)
                                                              as isize),
                                       *attributes.offset(j as isize));
                    if !fullname_0.is_null() {
                        xmlSAX2AttributeNs(ctxt, fullname_0,
                                           0 as *const xmlChar,
                                           *attributes.offset((j +
                                                                   3 as
                                                                       std::os::raw::c_int)
                                                                  as isize),
                                           *attributes.offset((j +
                                                                   4 as
                                                                       std::os::raw::c_int)
                                                                  as isize));
                        current_block_105 = 8937240710477387595;
                    } else { current_block_105 = 10863493864285401582; }
                } else {
                    lname =
                        xmlBuildQName(*attributes.offset(j as isize),
                                      *attributes.offset((j +
                                                              1 as
                                                                  std::os::raw::c_int)
                                                             as isize),
                                      0 as *mut xmlChar, 0 as std::os::raw::c_int);
                    if !lname.is_null() {
                        xmlSAX2AttributeNs(ctxt, lname, 0 as *const xmlChar,
                                           *attributes.offset((j +
                                                                   3 as
                                                                       std::os::raw::c_int)
                                                                  as isize),
                                           *attributes.offset((j +
                                                                   4 as
                                                                       std::os::raw::c_int)
                                                                  as isize));
                        xmlFree.expect("non-null function pointer")(lname as
                                                                        *mut std::os::raw::c_void);
                        current_block_105 = 8937240710477387595;
                    } else { current_block_105 = 10863493864285401582; }
                }
            } else { current_block_105 = 10863493864285401582; }
            match current_block_105 {
                10863493864285401582 => {
                    xmlSAX2AttributeNs(ctxt, *attributes.offset(j as isize),
                                       *attributes.offset((j +
                                                               1 as
                                                                   std::os::raw::c_int)
                                                              as isize),
                                       *attributes.offset((j +
                                                               3 as
                                                                   std::os::raw::c_int)
                                                              as isize),
                                       *attributes.offset((j +
                                                               4 as
                                                                   std::os::raw::c_int)
                                                              as isize));
                }
                _ => { }
            }
            i += 1;
            j += 5 as std::os::raw::c_int
        }
    }
    /*
     * If it's the Document root, finish the DTD validation and
     * check the document root element for validity
     */
    if (*ctxt).validate != 0 &&
           (*ctxt).vctxt.finishDtd == 0xabcd1234 as std::os::raw::c_uint {
        let mut chk: std::os::raw::c_int = 0;
        chk = xmlValidateDtdFinal(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        if chk <= 0 as std::os::raw::c_int { (*ctxt).valid = 0 as std::os::raw::c_int }
        if chk < 0 as std::os::raw::c_int { (*ctxt).wellFormed = 0 as std::os::raw::c_int }
        (*ctxt).valid &= xmlValidateRoot(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        (*ctxt).vctxt.finishDtd = 0xabcd1235 as std::os::raw::c_uint
    };
    /* LIBXML_VALID_ENABLED */
}
/* *
 * xmlSAX2EndElementNs:
 * @ctx:  the user data (XML parser context)
 * @localname:  the local name of the element
 * @prefix:  the element namespace prefix if available
 * @URI:  the element namespace name if available
 *
 * SAX2 callback when an element end has been detected by the parser.
 * It provides the namespace informations for the element.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndElementNs(mut ctx: *mut std::os::raw::c_void,
                                             mut localname: *const xmlChar,
                                             mut prefix: *const xmlChar,
                                             mut URI: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut node_info: xmlParserNodeInfo =
        xmlParserNodeInfo{node: 0 as *const _xmlNode,
                          begin_pos: 0,
                          begin_line: 0,
                          end_pos: 0,
                          end_line: 0,};
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    cur = (*ctxt).node;
    /* Capture end position and add node */
    if (*ctxt).record_info != 0 && !cur.is_null() {
        node_info.end_pos =
            (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                as std::os::raw::c_long as std::os::raw::c_ulong;
        node_info.end_line = (*(*ctxt).input).line as std::os::raw::c_ulong;
        node_info.node = cur as *const _xmlNode;
        xmlParserAddNodeInfo(ctxt, &mut node_info);
    }
    (*ctxt).nodemem = -(1 as std::os::raw::c_int);
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 &&
           !(*ctxt).myDoc.is_null() && !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &=
            xmlValidateOneElement(&mut (*ctxt).vctxt, (*ctxt).myDoc, cur)
    }
    /* LIBXML_VALID_ENABLED */
    /*
     * end of parsing of this node.
     */
    nodePop(ctxt);
}
/* *
 * xmlSAX2Reference:
 * @ctx: the user data (XML parser context)
 * @name:  The entity name
 *
 * called when an entity xmlSAX2Reference is detected.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Reference(mut ctx: *mut std::os::raw::c_void,
                                          mut name: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    if *name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '#' as i32 {
        ret = xmlNewCharRef((*ctxt).myDoc, name)
    } else { ret = xmlNewReference((*ctxt).myDoc as *const xmlDoc, name) }
    if xmlAddChild((*ctxt).node, ret).is_null() { xmlFreeNode(ret); };
}
/* *
 * xmlSAX2Characters:
 * @ctx: the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * receiving some chars from the parser.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Characters(mut ctx: *mut std::os::raw::c_void,
                                           mut ch: *const xmlChar,
                                           mut len: std::os::raw::c_int) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut lastChild: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    /*
     * Handle the data if any. If there is no child
     * add it as content, otherwise if the last child is text,
     * concatenate it, else create a new node of type text.
     */
    if (*ctxt).node.is_null() { return }
    lastChild = (*(*ctxt).node).last;
    /*
     * Here we needed an accelerator mechanism in case of very large
     * elements. Use an attribute in the structure !!!
     */
    if lastChild.is_null() {
        lastChild = xmlSAX2TextNode(ctxt, ch, len);
        if !lastChild.is_null() {
            (*(*ctxt).node).children = lastChild;
            (*(*ctxt).node).last = lastChild;
            (*lastChild).parent = (*ctxt).node;
            (*lastChild).doc = (*(*ctxt).node).doc;
            (*ctxt).nodelen = len;
            (*ctxt).nodemem = len + 1 as std::os::raw::c_int
        } else {
            xmlSAX2ErrMemory(ctxt,
                             b"xmlSAX2Characters\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
    } else {
        let mut coalesceText: std::os::raw::c_int =
            (!lastChild.is_null() &&
                 (*lastChild).type_0 as std::os::raw::c_uint ==
                     XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                 (*lastChild).name == xmlStringText.as_ptr()) as std::os::raw::c_int;
        if coalesceText != 0 && (*ctxt).nodemem != 0 as std::os::raw::c_int {
            /*
	     * The whole point of maintaining nodelen and nodemem,
	     * xmlTextConcat is too costly, i.e. compute length,
	     * reallocate a new buffer, move data, append ch. Here
	     * We try to minimaze realloc() uses and avoid copying
	     * and recomputing length over and over.
	     */
            if (*lastChild).content ==
                   &mut (*lastChild).properties as *mut *mut _xmlAttr as
                       *mut xmlChar {
                (*lastChild).content = xmlStrdup((*lastChild).content);
                (*lastChild).properties = 0 as *mut _xmlAttr
            } else if (*ctxt).nodemem == (*ctxt).nodelen + 1 as std::os::raw::c_int &&
                          xmlDictOwns((*ctxt).dict, (*lastChild).content) != 0
             {
                (*lastChild).content = xmlStrdup((*lastChild).content)
            }
            if (*lastChild).content.is_null() {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2Characters: xmlStrdup returned NULL\x00"
                                     as *const u8 as *const std::os::raw::c_char);
                return
            }
            if ((*ctxt).nodelen as size_t).wrapping_add(len as size_t) >
                   10000000 as std::os::raw::c_int as std::os::raw::c_ulong &&
                   (*ctxt).options & XML_PARSE_HUGE as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2Characters: huge text node\x00" as
                                     *const u8 as *const std::os::raw::c_char);
                return
            }
            if (*ctxt).nodelen as size_t >
                   (-(1 as std::os::raw::c_int) as size_t).wrapping_sub(len as size_t)
                   ||
                   ((*ctxt).nodemem as size_t).wrapping_add(len as size_t) >
                       (-(1 as std::os::raw::c_int) as
                            size_t).wrapping_div(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2Characters overflow prevented\x00"
                                     as *const u8 as *const std::os::raw::c_char);
                return
            }
            if (*ctxt).nodelen + len >= (*ctxt).nodemem {
                let mut newbuf: *mut xmlChar = 0 as *mut xmlChar;
                let mut size: size_t = 0;
                size = ((*ctxt).nodemem + len) as size_t;
                size =
                    (size as
                         std::os::raw::c_ulong).wrapping_mul(2 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        size_t as size_t;
                newbuf =
                    xmlRealloc.expect("non-null function pointer")((*lastChild).content
                                                                       as
                                                                       *mut std::os::raw::c_void,
                                                                   size) as
                        *mut xmlChar;
                if newbuf.is_null() {
                    xmlSAX2ErrMemory(ctxt,
                                     b"xmlSAX2Characters\x00" as *const u8 as
                                         *const std::os::raw::c_char);
                    return
                }
                (*ctxt).nodemem = size as std::os::raw::c_int;
                (*lastChild).content = newbuf
            }
            memcpy(&mut *(*lastChild).content.offset((*ctxt).nodelen as isize)
                       as *mut xmlChar as *mut std::os::raw::c_void,
                   ch as *const std::os::raw::c_void, len as std::os::raw::c_ulong);
            (*ctxt).nodelen += len;
            *(*lastChild).content.offset((*ctxt).nodelen as isize) =
                0 as std::os::raw::c_int as xmlChar
        } else if coalesceText != 0 {
            if xmlTextConcat(lastChild, ch, len) != 0 {
                xmlSAX2ErrMemory(ctxt,
                                 b"xmlSAX2Characters\x00" as *const u8 as
                                     *const std::os::raw::c_char);
            }
            if !(*(*ctxt).node).children.is_null() {
                (*ctxt).nodelen = xmlStrlen((*lastChild).content);
                (*ctxt).nodemem = (*ctxt).nodelen + 1 as std::os::raw::c_int
            }
        } else {
            /* Mixed content, first time */
            lastChild = xmlSAX2TextNode(ctxt, ch, len);
            if !lastChild.is_null() {
                xmlAddChild((*ctxt).node, lastChild);
                if !(*(*ctxt).node).children.is_null() {
                    (*ctxt).nodelen = len;
                    (*ctxt).nodemem = len + 1 as std::os::raw::c_int
                }
            }
        }
    };
}
/* *
 * xmlSAX2IgnorableWhitespace:
 * @ctx: the user data (XML parser context)
 * @ch:  a xmlChar string
 * @len: the number of xmlChar
 *
 * receiving some ignorable whitespaces from the parser.
 * UNUSED: by default the DOM building will use xmlSAX2Characters
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2IgnorableWhitespace(mut ctx:
                                                        *mut std::os::raw::c_void,
                                                    mut ch: *const xmlChar,
                                                    mut len: std::os::raw::c_int) {
    /* xmlParserCtxtPtr ctxt = (xmlParserCtxtPtr) ctx; */
}
/* *
 * xmlSAX2ProcessingInstruction:
 * @ctx: the user data (XML parser context)
 * @target:  the target name
 * @data: the PI data's
 *
 * A processing instruction has been parsed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ProcessingInstruction(mut ctx:
                                                          *mut std::os::raw::c_void,
                                                      mut target:
                                                          *const xmlChar,
                                                      mut data:
                                                          *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    parent = (*ctxt).node;
    ret = xmlNewDocPI((*ctxt).myDoc, target, data);
    if ret.is_null() { return }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if (*(*ctxt).input).line < 65535 as std::os::raw::c_int {
                (*ret).line =
                    (*(*ctxt).input).line as std::os::raw::c_short as std::os::raw::c_ushort
            } else { (*ret).line = 65535 as std::os::raw::c_int as std::os::raw::c_ushort }
        }
    }
    if (*ctxt).inSubset == 1 as std::os::raw::c_int {
        xmlAddChild((*(*ctxt).myDoc).intSubset as xmlNodePtr, ret);
        return
    } else {
        if (*ctxt).inSubset == 2 as std::os::raw::c_int {
            xmlAddChild((*(*ctxt).myDoc).extSubset as xmlNodePtr, ret);
            return
        }
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
        return
    }
    if (*parent).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlAddChild(parent, ret);
    } else { xmlAddSibling(parent, ret); };
}
/* *
 * xmlSAX2Comment:
 * @ctx: the user data (XML parser context)
 * @value:  the xmlSAX2Comment content
 *
 * A xmlSAX2Comment has been parsed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Comment(mut ctx: *mut std::os::raw::c_void,
                                        mut value: *const xmlChar) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    parent = (*ctxt).node;
    ret = xmlNewDocComment((*ctxt).myDoc, value);
    if ret.is_null() { return }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if (*(*ctxt).input).line < 65535 as std::os::raw::c_int {
                (*ret).line =
                    (*(*ctxt).input).line as std::os::raw::c_short as std::os::raw::c_ushort
            } else { (*ret).line = 65535 as std::os::raw::c_int as std::os::raw::c_ushort }
        }
    }
    if (*ctxt).inSubset == 1 as std::os::raw::c_int {
        xmlAddChild((*(*ctxt).myDoc).intSubset as xmlNodePtr, ret);
        return
    } else {
        if (*ctxt).inSubset == 2 as std::os::raw::c_int {
            xmlAddChild((*(*ctxt).myDoc).extSubset as xmlNodePtr, ret);
            return
        }
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
        return
    }
    if (*parent).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlAddChild(parent, ret);
    } else { xmlAddSibling(parent, ret); };
}
/* *
 * xmlSAX2CDataBlock:
 * @ctx: the user data (XML parser context)
 * @value:  The pcdata content
 * @len:  the block length
 *
 * called when a pcdata block has been parsed
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2CDataBlock(mut ctx: *mut std::os::raw::c_void,
                                           mut value: *const xmlChar,
                                           mut len: std::os::raw::c_int) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    let mut lastChild: xmlNodePtr = 0 as *mut xmlNode;
    if ctx.is_null() { return }
    lastChild = xmlGetLastChild((*ctxt).node as *const xmlNode);
    if !lastChild.is_null() &&
           (*lastChild).type_0 as std::os::raw::c_uint ==
               XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlTextConcat(lastChild, value, len);
    } else {
        ret = xmlNewCDataBlock((*ctxt).myDoc, value, len);
        if xmlAddChild((*ctxt).node, ret).is_null() { xmlFreeNode(ret); }
    };
}
static mut xmlSAX2DefaultVersionValue: std::os::raw::c_int = 2 as std::os::raw::c_int;
/* *
 * xmlSAXDefaultVersion:
 * @version:  the version, 1 or 2
 *
 * Set the default version of SAX used globally by the library.
 * By default, during initialization the default is set to 2.
 * Note that it is generally a better coding style to use
 * xmlSAXVersion() to set up the version explicitly for a given
 * parsing context.
 *
 * Returns the previous value in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAXDefaultVersion(mut version: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = xmlSAX2DefaultVersionValue;
    if version != 1 as std::os::raw::c_int && version != 2 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    xmlSAX2DefaultVersionValue = version;
    return ret;
}
/* LIBXML_SAX1_ENABLED */
/* *
 * xmlSAXVersion:
 * @hdlr:  the SAX handler
 * @version:  the version, 1 or 2
 *
 * Initialize the default XML SAX handler according to the version
 *
 * Returns 0 in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAXVersion(mut hdlr: *mut xmlSAXHandler,
                                       mut version: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if hdlr.is_null() { return -(1 as std::os::raw::c_int) }
    if version == 2 as std::os::raw::c_int {
        (*hdlr).startElement = None;
        (*hdlr).endElement = None;
        (*hdlr).startElementNs =
            Some(xmlSAX2StartElementNs as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *const xmlChar,
                                          _: *const xmlChar, _: std::os::raw::c_int,
                                          _: *mut *const xmlChar,
                                          _: std::os::raw::c_int, _: std::os::raw::c_int,
                                          _: *mut *const xmlChar) -> ());
        (*hdlr).endElementNs =
            Some(xmlSAX2EndElementNs as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *const xmlChar,
                                          _: *const xmlChar) -> ());
        (*hdlr).serror = None;
        (*hdlr).initialized = 0xdeedbeaf as std::os::raw::c_uint
    } else if version == 1 as std::os::raw::c_int {
        (*hdlr).startElement =
            Some(xmlSAX2StartElement as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar,
                                          _: *mut *const xmlChar) -> ());
        (*hdlr).endElement =
            Some(xmlSAX2EndElement as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const xmlChar) -> ());
        (*hdlr).initialized = 1 as std::os::raw::c_int as std::os::raw::c_uint
        /* LIBXML_SAX1_ENABLED */
    } else { return -(1 as std::os::raw::c_int) }
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
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSAX2InitDefaultSAXHandler:
 * @hdlr:  the SAX handler
 * @warning:  flag if non-zero sets the handler warning procedure
 *
 * Initialize the default XML SAX2 handler
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitDefaultSAXHandler(mut hdlr:
                                                          *mut xmlSAXHandler,
                                                      mut warning:
                                                          std::os::raw::c_int) {
    if hdlr.is_null() ||
           (*hdlr).initialized != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    xmlSAXVersion(hdlr, xmlSAX2DefaultVersionValue);
    if warning == 0 as std::os::raw::c_int {
        (*hdlr).warning = None
    } else {
        (*hdlr).warning =
            Some(xmlParserWarning as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *const std::os::raw::c_char, _: ...)
                         -> ())
    };
}
/* *
 * xmlDefaultSAXHandlerInit:
 *
 * Initialize the default SAX2 handler
 */
#[no_mangle]
pub unsafe extern "C" fn xmlDefaultSAXHandlerInit() {
    xmlSAXVersion(__xmlDefaultSAXHandler() as xmlSAXHandlerPtr,
                  1 as std::os::raw::c_int);
    /* LIBXML_SAX1_ENABLED */
}
/* *
 * xmlSAX2InitHtmlDefaultSAXHandler:
 * @hdlr:  the SAX handler
 *
 * Initialize the default HTML SAX2 handler
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitHtmlDefaultSAXHandler(mut hdlr:
                                                              *mut xmlSAXHandler) {
    if hdlr.is_null() ||
           (*hdlr).initialized != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
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
/* *
 * htmlDefaultSAXHandlerInit:
 *
 * Initialize the default SAX handler
 */
#[no_mangle]
pub unsafe extern "C" fn htmlDefaultSAXHandlerInit() {
    xmlSAX2InitHtmlDefaultSAXHandler(__htmlDefaultSAXHandler() as
                                         xmlSAXHandlerPtr);
}
/* LIBXML_HTML_ENABLED */
/* *
 * xmlSAX2InitDocbDefaultSAXHandler:
 * @hdlr:  the SAX handler
 *
 * Initialize the default DocBook SAX2 handler
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitDocbDefaultSAXHandler(mut hdlr:
                                                              *mut xmlSAXHandler) {
    if hdlr.is_null() ||
           (*hdlr).initialized != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
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
/* LIBXML_SAX1_ENABLED */
/* *
 * docbDefaultSAXHandlerInit:
 *
 * Initialize the default SAX handler
 */
#[no_mangle]
pub unsafe extern "C" fn docbDefaultSAXHandlerInit() {
    xmlSAX2InitDocbDefaultSAXHandler(__docbDefaultSAXHandler() as
                                         xmlSAXHandlerPtr);
}
/* __INCLUDE_ELFGCCHACK */
/* LIBXML_DOCB_ENABLED */
