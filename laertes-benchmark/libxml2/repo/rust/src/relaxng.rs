
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
 * Summary: regular expressions handling
 * Description: basic API for libxml regular expressions handling used
 *              for XML Schemas and validation.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /* *
 * xmlRegexpPtr:
 *
 * A libxml regular expression, they can actually be far more complex
 * thank the POSIX regex expressions.
 */
    pub type _xmlRegexp;
    /* *
 * xmlRegExecCtxtPtr:
 *
 * A libxml progressive regular expression evaluation context
 */
    pub type _xmlRegExecCtxt;
    pub type _xmlSchemaVal;
    pub type _xmlSchemaParserCtxt;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCharStrdup(cur: *const std::os::raw::c_char) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlEscapeFormatString(msg: *mut *mut xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
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
    fn xmlRegExecPushString2(exec: xmlRegExecCtxtPtr, value: *const xmlChar,
                             value2: *const xmlChar, data: *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
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
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlCopyDoc(doc: xmlDocPtr, recursive: std::os::raw::c_int) -> xmlDocPtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewChild(parent: xmlNodePtr, ns: xmlNsPtr, name: *const xmlChar,
                   content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlIsBlankNode(node: *const xmlNode) -> std::os::raw::c_int;
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
    /*
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr,
                   nameSpace: *const xmlChar) -> xmlNsPtr;
    /*
 * Changing the content.
 */
    #[no_mangle]
    fn xmlSetProp(node: xmlNodePtr, name: *const xmlChar,
                  value: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlHasProp(node: *const xmlNode, name: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlNodeListGetString(doc: xmlDocPtr, list: *const xmlNode,
                            inLine: std::os::raw::c_int) -> *mut xmlChar;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeSetContent(cur: xmlNodePtr, content: *const xmlChar);
    #[no_mangle]
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlUnsetProp(node: xmlNodePtr, name: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
    /*
 * Constructor and destructor.
 */
    #[no_mangle]
    fn xmlHashCreate(size: std::os::raw::c_int) -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    /*
 * Add a new entry to the hash table.
 */
    #[no_mangle]
    fn xmlHashAddEntry(table: xmlHashTablePtr, name: *const xmlChar,
                       userdata: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashAddEntry2(table: xmlHashTablePtr, name: *const xmlChar,
                        name2: *const xmlChar, userdata: *mut std::os::raw::c_void)
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
    fn xmlAutomataNewTransition(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                                to: xmlAutomataStatePtr,
                                token: *const xmlChar,
                                data: *mut std::os::raw::c_void)
     -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewTransition2(am: xmlAutomataPtr,
                                 from: xmlAutomataStatePtr,
                                 to: xmlAutomataStatePtr,
                                 token: *const xmlChar,
                                 token2: *const xmlChar,
                                 data: *mut std::os::raw::c_void)
     -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataNewEpsilon(am: xmlAutomataPtr, from: xmlAutomataStatePtr,
                             to: xmlAutomataStatePtr) -> xmlAutomataStatePtr;
    #[no_mangle]
    fn xmlAutomataCompile(am: xmlAutomataPtr) -> xmlRegexpPtr;
    #[no_mangle]
    fn xmlAutomataIsDeterminist(am: xmlAutomataPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateDocumentFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                     URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                     options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlURIEscapeStr(str: *const xmlChar, list: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    #[no_mangle]
    fn xmlSchemaFreeValue(val: xmlSchemaValPtr);
    #[no_mangle]
    fn xmlSchemaFreeFacet(facet: xmlSchemaFacetPtr);
    #[no_mangle]
    fn xmlSchemaValidateFacet(base: xmlSchemaTypePtr,
                              facet: xmlSchemaFacetPtr, value: *const xmlChar,
                              val: xmlSchemaValPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaCheckFacet(facet: xmlSchemaFacetPtr,
                           typeDecl: xmlSchemaTypePtr,
                           ctxt: xmlSchemaParserCtxtPtr, name: *const xmlChar)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaNewFacet() -> xmlSchemaFacetPtr;
    #[no_mangle]
    fn xmlSchemaGetPredefinedType(name: *const xmlChar, ns: *const xmlChar)
     -> xmlSchemaTypePtr;
    #[no_mangle]
    fn xmlSchemaCompareValues(x: xmlSchemaValPtr, y: xmlSchemaValPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaValPredefTypeNode(type_0: xmlSchemaTypePtr,
                                  value: *const xmlChar,
                                  val: *mut xmlSchemaValPtr, node: xmlNodePtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaCleanupTypes();
    /* ***********************************************************************
 *									*
 *		Compiling element content into regexp			*
 *									*
 * Sometime the element content can be compiled into a pure regexp,	*
 * This allows a faster execution and streamability at that level	*
 *									*
 ************************************************************************/
    /* from automata.c but not exported */
    #[no_mangle]
    fn xmlAutomataSetFlags(am: xmlAutomataPtr, flags: std::os::raw::c_int);
}
pub type xmlChar = std::os::raw::c_uchar;
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
pub type ptrdiff_t = std::os::raw::c_long;
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
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
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
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
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
/* a compiled content model if available */
/* *
 * _xmlRelaxNG:
 *
 * A RelaxNGs definition
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNG {
    pub _private: *mut std::os::raw::c_void,
    pub topgrammar: xmlRelaxNGGrammarPtr,
    pub doc: xmlDocPtr,
    pub idref: std::os::raw::c_int,
    pub defs: xmlHashTablePtr,
    pub refs: xmlHashTablePtr,
    pub documents: xmlRelaxNGDocumentPtr,
    pub includes: xmlRelaxNGIncludePtr,
    pub defNr: std::os::raw::c_int,
    pub defTab: *mut xmlRelaxNGDefinePtr,
}
pub type xmlRelaxNGDefinePtr = *mut xmlRelaxNGDefine;
pub type xmlRelaxNGDefine = _xmlRelaxNGDefine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGDefine {
    pub type_0: xmlRelaxNGType,
    pub node: xmlNodePtr,
    pub name: *mut xmlChar,
    pub ns: *mut xmlChar,
    pub value: *mut xmlChar,
    pub data: *mut std::os::raw::c_void,
    pub content: xmlRelaxNGDefinePtr,
    pub parent: xmlRelaxNGDefinePtr,
    pub next: xmlRelaxNGDefinePtr,
    pub attrs: xmlRelaxNGDefinePtr,
    pub nameClass: xmlRelaxNGDefinePtr,
    pub nextHash: xmlRelaxNGDefinePtr,
    pub depth: std::os::raw::c_short,
    pub dflags: std::os::raw::c_short,
    pub contModel: xmlRegexpPtr,
}
pub type xmlRelaxNGType = std::os::raw::c_int;
pub const XML_RELAXNG_START: xmlRelaxNGType = 20;
pub const XML_RELAXNG_INTERLEAVE: xmlRelaxNGType = 19;
pub const XML_RELAXNG_GROUP: xmlRelaxNGType = 18;
pub const XML_RELAXNG_CHOICE: xmlRelaxNGType = 17;
pub const XML_RELAXNG_ONEORMORE: xmlRelaxNGType = 16;
pub const XML_RELAXNG_ZEROORMORE: xmlRelaxNGType = 15;
pub const XML_RELAXNG_OPTIONAL: xmlRelaxNGType = 14;
pub const XML_RELAXNG_PARENTREF: xmlRelaxNGType = 13;
pub const XML_RELAXNG_EXTERNALREF: xmlRelaxNGType = 12;
pub const XML_RELAXNG_REF: xmlRelaxNGType = 11;
pub const XML_RELAXNG_DEF: xmlRelaxNGType = 10;
pub const XML_RELAXNG_ATTRIBUTE: xmlRelaxNGType = 9;
pub const XML_RELAXNG_LIST: xmlRelaxNGType = 8;
pub const XML_RELAXNG_VALUE: xmlRelaxNGType = 7;
pub const XML_RELAXNG_PARAM: xmlRelaxNGType = 6;
pub const XML_RELAXNG_DATATYPE: xmlRelaxNGType = 5;
pub const XML_RELAXNG_ELEMENT: xmlRelaxNGType = 4;
pub const XML_RELAXNG_TEXT: xmlRelaxNGType = 3;
pub const XML_RELAXNG_EXCEPT: xmlRelaxNGType = 2;
pub const XML_RELAXNG_NOT_ALLOWED: xmlRelaxNGType = 1;
pub const XML_RELAXNG_EMPTY: xmlRelaxNGType = 0;
pub const XML_RELAXNG_NOOP: xmlRelaxNGType = -1;
pub type xmlRelaxNGIncludePtr = *mut xmlRelaxNGInclude;
pub type xmlRelaxNGInclude = _xmlRelaxNGInclude;
/* signal error in content model
                                 * outside the regexp */
/* *
 * xmlRelaxNGInclude:
 *
 * Structure associated to a RelaxNGs document element
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGInclude {
    pub next: xmlRelaxNGIncludePtr,
    pub href: *mut xmlChar,
    pub doc: xmlDocPtr,
    pub content: xmlRelaxNGDefinePtr,
    pub schema: xmlRelaxNGPtr,
}
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
/*
 * Summary: implementation of the Relax-NG validation
 * Description: implementation of the Relax-NG validation
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGDocumentPtr = *mut xmlRelaxNGDocument;
pub type xmlRelaxNGDocument = _xmlRelaxNGDocument;
/* the schema */
/* *
 * xmlRelaxNGDocument:
 *
 * Structure associated to a RelaxNGs document element
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGDocument {
    pub next: xmlRelaxNGDocumentPtr,
    pub href: *mut xmlChar,
    pub doc: xmlDocPtr,
    pub content: xmlRelaxNGDefinePtr,
    pub schema: xmlRelaxNGPtr,
    pub externalRef: std::os::raw::c_int,
}
pub type xmlRelaxNGGrammarPtr = *mut xmlRelaxNGGrammar;
pub type xmlRelaxNGGrammar = _xmlRelaxNGGrammar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGGrammar {
    pub parent: xmlRelaxNGGrammarPtr,
    pub children: xmlRelaxNGGrammarPtr,
    pub next: xmlRelaxNGGrammarPtr,
    pub start: xmlRelaxNGDefinePtr,
    pub combine: xmlRelaxNGCombine,
    pub startList: xmlRelaxNGDefinePtr,
    pub defs: xmlHashTablePtr,
    pub refs: xmlHashTablePtr,
}
pub type xmlRelaxNGCombine = std::os::raw::c_uint;
pub const XML_RELAXNG_COMBINE_INTERLEAVE: xmlRelaxNGCombine = 2;
pub const XML_RELAXNG_COMBINE_CHOICE: xmlRelaxNGCombine = 1;
pub const XML_RELAXNG_COMBINE_UNDEFINED: xmlRelaxNGCombine = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGParserCtxt {
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlRelaxNGValidityErrorFunc,
    pub warning: xmlRelaxNGValidityWarningFunc,
    pub serror: xmlStructuredErrorFunc,
    pub err: xmlRelaxNGValidErr,
    pub schema: xmlRelaxNGPtr,
    pub grammar: xmlRelaxNGGrammarPtr,
    pub parentgrammar: xmlRelaxNGGrammarPtr,
    pub flags: std::os::raw::c_int,
    pub nbErrors: std::os::raw::c_int,
    pub nbWarnings: std::os::raw::c_int,
    pub define: *const xmlChar,
    pub def: xmlRelaxNGDefinePtr,
    pub nbInterleaves: std::os::raw::c_int,
    pub interleaves: xmlHashTablePtr,
    pub documents: xmlRelaxNGDocumentPtr,
    pub includes: xmlRelaxNGIncludePtr,
    pub URL: *mut xmlChar,
    pub document: xmlDocPtr,
    pub defNr: std::os::raw::c_int,
    pub defMax: std::os::raw::c_int,
    pub defTab: *mut xmlRelaxNGDefinePtr,
    pub buffer: *const std::os::raw::c_char,
    pub size: std::os::raw::c_int,
    pub doc: xmlRelaxNGDocumentPtr,
    pub docNr: std::os::raw::c_int,
    pub docMax: std::os::raw::c_int,
    pub docTab: *mut xmlRelaxNGDocumentPtr,
    pub inc: xmlRelaxNGIncludePtr,
    pub incNr: std::os::raw::c_int,
    pub incMax: std::os::raw::c_int,
    pub incTab: *mut xmlRelaxNGIncludePtr,
    pub idref: std::os::raw::c_int,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
    pub crng: std::os::raw::c_int,
    pub freedoc: std::os::raw::c_int,
}
pub type xmlRelaxNGValidErr = std::os::raw::c_uint;
pub const XML_RELAXNG_ERR_TEXTWRONG: xmlRelaxNGValidErr = 39;
pub const XML_RELAXNG_ERR_ELEMWRONG: xmlRelaxNGValidErr = 38;
pub const XML_RELAXNG_ERR_INTERNAL: xmlRelaxNGValidErr = 37;
pub const XML_RELAXNG_ERR_LACKDATA: xmlRelaxNGValidErr = 36;
pub const XML_RELAXNG_ERR_EXTRADATA: xmlRelaxNGValidErr = 35;
pub const XML_RELAXNG_ERR_NOGRAMMAR: xmlRelaxNGValidErr = 34;
pub const XML_RELAXNG_ERR_LIST: xmlRelaxNGValidErr = 33;
pub const XML_RELAXNG_ERR_VALUE: xmlRelaxNGValidErr = 32;
pub const XML_RELAXNG_ERR_DATATYPE: xmlRelaxNGValidErr = 31;
pub const XML_RELAXNG_ERR_LISTELEM: xmlRelaxNGValidErr = 30;
pub const XML_RELAXNG_ERR_VALELEM: xmlRelaxNGValidErr = 29;
pub const XML_RELAXNG_ERR_DATAELEM: xmlRelaxNGValidErr = 28;
pub const XML_RELAXNG_ERR_INVALIDATTR: xmlRelaxNGValidErr = 27;
pub const XML_RELAXNG_ERR_EXTRACONTENT: xmlRelaxNGValidErr = 26;
pub const XML_RELAXNG_ERR_CONTENTVALID: xmlRelaxNGValidErr = 25;
pub const XML_RELAXNG_ERR_ATTRVALID: xmlRelaxNGValidErr = 24;
pub const XML_RELAXNG_ERR_NOTELEM: xmlRelaxNGValidErr = 23;
pub const XML_RELAXNG_ERR_NOELEM: xmlRelaxNGValidErr = 22;
pub const XML_RELAXNG_ERR_ELEMNOTEMPTY: xmlRelaxNGValidErr = 21;
pub const XML_RELAXNG_ERR_ATTREXTRANS: xmlRelaxNGValidErr = 20;
pub const XML_RELAXNG_ERR_ELEMEXTRANS: xmlRelaxNGValidErr = 19;
pub const XML_RELAXNG_ERR_ATTRWRONGNS: xmlRelaxNGValidErr = 18;
pub const XML_RELAXNG_ERR_ELEMWRONGNS: xmlRelaxNGValidErr = 17;
pub const XML_RELAXNG_ERR_ATTRNONS: xmlRelaxNGValidErr = 16;
pub const XML_RELAXNG_ERR_ELEMNONS: xmlRelaxNGValidErr = 15;
pub const XML_RELAXNG_ERR_ATTRNAME: xmlRelaxNGValidErr = 14;
pub const XML_RELAXNG_ERR_ELEMNAME: xmlRelaxNGValidErr = 13;
pub const XML_RELAXNG_ERR_INTEREXTRA: xmlRelaxNGValidErr = 12;
pub const XML_RELAXNG_ERR_INTERSEQ: xmlRelaxNGValidErr = 11;
pub const XML_RELAXNG_ERR_INTERNODATA: xmlRelaxNGValidErr = 10;
pub const XML_RELAXNG_ERR_LISTEMPTY: xmlRelaxNGValidErr = 9;
pub const XML_RELAXNG_ERR_LISTEXTRA: xmlRelaxNGValidErr = 8;
pub const XML_RELAXNG_ERR_NODEFINE: xmlRelaxNGValidErr = 7;
pub const XML_RELAXNG_ERR_NOSTATE: xmlRelaxNGValidErr = 6;
pub const XML_RELAXNG_ERR_TYPECMP: xmlRelaxNGValidErr = 5;
pub const XML_RELAXNG_ERR_DUPID: xmlRelaxNGValidErr = 4;
pub const XML_RELAXNG_ERR_TYPEVAL: xmlRelaxNGValidErr = 3;
pub const XML_RELAXNG_ERR_TYPE: xmlRelaxNGValidErr = 2;
pub const XML_RELAXNG_ERR_MEMORY: xmlRelaxNGValidErr = 1;
pub const XML_RELAXNG_OK: xmlRelaxNGValidErr = 0;
/* *
 * A schemas validation context
 */
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
/* second arg */
/* *
 * xmlRelaxNGValidCtxt:
 *
 * A RelaxNGs validation context
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGValidCtxt {
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlRelaxNGValidityErrorFunc,
    pub warning: xmlRelaxNGValidityWarningFunc,
    pub serror: xmlStructuredErrorFunc,
    pub nbErrors: std::os::raw::c_int,
    pub schema: xmlRelaxNGPtr,
    pub doc: xmlDocPtr,
    pub flags: std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub idref: std::os::raw::c_int,
    pub errNo: std::os::raw::c_int,
    pub err: xmlRelaxNGValidErrorPtr,
    pub errNr: std::os::raw::c_int,
    pub errMax: std::os::raw::c_int,
    pub errTab: xmlRelaxNGValidErrorPtr,
    pub state: xmlRelaxNGValidStatePtr,
    pub states: xmlRelaxNGStatesPtr,
    pub freeState: xmlRelaxNGStatesPtr,
    pub freeStatesNr: std::os::raw::c_int,
    pub freeStatesMax: std::os::raw::c_int,
    pub freeStates: *mut xmlRelaxNGStatesPtr,
    pub elem: xmlRegExecCtxtPtr,
    pub elemNr: std::os::raw::c_int,
    pub elemMax: std::os::raw::c_int,
    pub elemTab: *mut xmlRegExecCtxtPtr,
    pub pstate: std::os::raw::c_int,
    pub pnode: xmlNodePtr,
    pub pdef: xmlRelaxNGDefinePtr,
    pub perr: std::os::raw::c_int,
}
pub type xmlRelaxNGStatesPtr = *mut xmlRelaxNGStates;
/* the array of attributes */
/* *
 * xmlRelaxNGStates:
 *
 * A RelaxNGs container for validation state
 */
pub type xmlRelaxNGStates = _xmlRelaxNGStates;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGStates {
    pub nbState: std::os::raw::c_int,
    pub maxState: std::os::raw::c_int,
    pub tabState: *mut xmlRelaxNGValidStatePtr,
}
pub type xmlRelaxNGValidStatePtr = *mut xmlRelaxNGValidState;
pub type xmlRelaxNGValidState = _xmlRelaxNGValidState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGValidState {
    pub node: xmlNodePtr,
    pub seq: xmlNodePtr,
    pub nbAttrs: std::os::raw::c_int,
    pub maxAttrs: std::os::raw::c_int,
    pub nbAttrLeft: std::os::raw::c_int,
    pub value: *mut xmlChar,
    pub endvalue: *mut xmlChar,
    pub attrs: *mut xmlAttrPtr,
}
pub type xmlRelaxNGValidErrorPtr = *mut xmlRelaxNGValidError;
/* *
 * xmlRelaxNGValidError:
 *
 * A RelaxNGs validation error
 */
pub type xmlRelaxNGValidError = _xmlRelaxNGValidError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGValidError {
    pub err: xmlRelaxNGValidErr,
    pub flags: std::os::raw::c_int,
    pub node: xmlNodePtr,
    pub seq: xmlNodePtr,
    pub arg1: *const xmlChar,
    pub arg2: *const xmlChar,
}
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
pub const XML_RELAXNGP_CRNG: C2RustUnnamed_1 = 2;
pub const XML_RELAXNGP_FREE_DOC: C2RustUnnamed_1 = 1;
pub const XML_RELAXNGP_NONE: C2RustUnnamed_1 = 0;
/* *
 * xmlRelaxNGTypeFree:
 * @data:  data needed for the library
 * @result:  the value to free
 *
 * Function provided by a type library to free a returned result
 */
pub type xmlRelaxNGTypeFree
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_void)
               -> ()>;
/* *
 * xmlRelaxNGFacetCheck:
 * @data:  data needed for the library
 * @type:  the type name
 * @facet:  the facet name
 * @val:  the facet value
 * @strval:  the string value
 * @value:  the value to check
 *
 * Function provided by a type library to check a value facet
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
pub type xmlRelaxNGFacetCheck
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *const xmlChar,
                                _: *const xmlChar, _: *mut std::os::raw::c_void)
               -> std::os::raw::c_int>;
/* *
 * xmlRelaxNGTypeCompare:
 * @data:  data needed for the library
 * @type:  the type name
 * @value1:  the first value
 * @value2:  the second value
 *
 * Function provided by a type library to compare two values accordingly
 * to a type.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
pub type xmlRelaxNGTypeCompare
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: xmlNodePtr,
                                _: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: xmlNodePtr) -> std::os::raw::c_int>;
/* *
 * xmlRelaxNGTypeCheck:
 * @data:  data needed for the library
 * @type:  the type name
 * @value:  the value to check
 * @result:  place to store the result if needed
 *
 * Function provided by a type library to check if a value match a type
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
pub type xmlRelaxNGTypeCheck
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar, _: *mut *mut std::os::raw::c_void,
                                _: xmlNodePtr) -> std::os::raw::c_int>;
/* ***********************************************************************
 *									*
 *		Preliminary type checking interfaces			*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGTypeHave:
 * @data:  data needed for the library
 * @type:  the type name
 * @value:  the value to check
 *
 * Function provided by a type library to check if a type is exported
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
pub type xmlRelaxNGTypeHave
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar)
               -> std::os::raw::c_int>;
pub type xmlRelaxNGTypeLibraryPtr = *mut xmlRelaxNGTypeLibrary;
pub type xmlRelaxNGTypeLibrary = _xmlRelaxNGTypeLibrary;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGTypeLibrary {
    pub namespace: *const xmlChar,
    pub data: *mut std::os::raw::c_void,
    pub have: xmlRelaxNGTypeHave,
    pub check: xmlRelaxNGTypeCheck,
    pub comp: xmlRelaxNGTypeCompare,
    pub facet: xmlRelaxNGFacetCheck,
    pub freef: xmlRelaxNGTypeFree,
}
pub type xmlSchemaValPtr = *mut xmlSchemaVal;
pub type xmlSchemaVal = _xmlSchemaVal;
pub type xmlSchemaFacetPtr = *mut xmlSchemaFacet;
pub type xmlSchemaFacet = _xmlSchemaFacet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacet {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaFacet,
    pub value: *const xmlChar,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub fixed: std::os::raw::c_int,
    pub whitespace: std::os::raw::c_int,
    pub val: xmlSchemaValPtr,
    pub regexp: xmlRegexpPtr,
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
pub type xmlSchemaTypeType = std::os::raw::c_uint;
pub const XML_SCHEMA_EXTRA_ATTR_USE_PROHIB: xmlSchemaTypeType = 2001;
pub const XML_SCHEMA_EXTRA_QNAMEREF: xmlSchemaTypeType = 2000;
pub const XML_SCHEMA_FACET_MINLENGTH: xmlSchemaTypeType = 1011;
pub const XML_SCHEMA_FACET_MAXLENGTH: xmlSchemaTypeType = 1010;
pub const XML_SCHEMA_FACET_LENGTH: xmlSchemaTypeType = 1009;
pub const XML_SCHEMA_FACET_WHITESPACE: xmlSchemaTypeType = 1008;
pub const XML_SCHEMA_FACET_ENUMERATION: xmlSchemaTypeType = 1007;
pub const XML_SCHEMA_FACET_PATTERN: xmlSchemaTypeType = 1006;
pub const XML_SCHEMA_FACET_FRACTIONDIGITS: xmlSchemaTypeType = 1005;
pub const XML_SCHEMA_FACET_TOTALDIGITS: xmlSchemaTypeType = 1004;
pub const XML_SCHEMA_FACET_MAXEXCLUSIVE: xmlSchemaTypeType = 1003;
pub const XML_SCHEMA_FACET_MAXINCLUSIVE: xmlSchemaTypeType = 1002;
pub const XML_SCHEMA_FACET_MINEXCLUSIVE: xmlSchemaTypeType = 1001;
pub const XML_SCHEMA_FACET_MININCLUSIVE: xmlSchemaTypeType = 1000;
pub const XML_SCHEMA_TYPE_ATTRIBUTE_USE: xmlSchemaTypeType = 26;
pub const XML_SCHEMA_TYPE_PARTICLE: xmlSchemaTypeType = 25;
pub const XML_SCHEMA_TYPE_IDC_KEYREF: xmlSchemaTypeType = 24;
pub const XML_SCHEMA_TYPE_IDC_KEY: xmlSchemaTypeType = 23;
pub const XML_SCHEMA_TYPE_IDC_UNIQUE: xmlSchemaTypeType = 22;
pub const XML_SCHEMA_TYPE_ANY_ATTRIBUTE: xmlSchemaTypeType = 21;
pub const XML_SCHEMA_TYPE_UNION: xmlSchemaTypeType = 20;
pub const XML_SCHEMA_TYPE_LIST: xmlSchemaTypeType = 19;
pub const XML_SCHEMA_TYPE_NOTATION: xmlSchemaTypeType = 18;
pub const XML_SCHEMA_TYPE_GROUP: xmlSchemaTypeType = 17;
pub const XML_SCHEMA_TYPE_ATTRIBUTEGROUP: xmlSchemaTypeType = 16;
pub const XML_SCHEMA_TYPE_ATTRIBUTE: xmlSchemaTypeType = 15;
pub const XML_SCHEMA_TYPE_ELEMENT: xmlSchemaTypeType = 14;
pub const XML_SCHEMA_TYPE_EXTENSION: xmlSchemaTypeType = 13;
pub const XML_SCHEMA_TYPE_RESTRICTION: xmlSchemaTypeType = 12;
pub const XML_SCHEMA_TYPE_UR: xmlSchemaTypeType = 11;
pub const XML_SCHEMA_TYPE_COMPLEX_CONTENT: xmlSchemaTypeType = 10;
pub const XML_SCHEMA_TYPE_SIMPLE_CONTENT: xmlSchemaTypeType = 9;
pub const XML_SCHEMA_TYPE_ALL: xmlSchemaTypeType = 8;
pub const XML_SCHEMA_TYPE_CHOICE: xmlSchemaTypeType = 7;
pub const XML_SCHEMA_TYPE_SEQUENCE: xmlSchemaTypeType = 6;
pub const XML_SCHEMA_TYPE_COMPLEX: xmlSchemaTypeType = 5;
pub const XML_SCHEMA_TYPE_SIMPLE: xmlSchemaTypeType = 4;
pub const XML_SCHEMA_TYPE_FACET: xmlSchemaTypeType = 3;
pub const XML_SCHEMA_TYPE_ANY: xmlSchemaTypeType = 2;
pub const XML_SCHEMA_TYPE_BASIC: xmlSchemaTypeType = 1;
pub type xmlSchemaTypePtr = *mut xmlSchemaType;
pub type xmlSchemaType = _xmlSchemaType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaType {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaType,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_0: *const xmlChar,
    pub refNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub subtypes: xmlSchemaTypePtr,
    pub attributes: xmlSchemaAttributePtr,
    pub node: xmlNodePtr,
    pub minOccurs: std::os::raw::c_int,
    pub maxOccurs: std::os::raw::c_int,
    pub flags: std::os::raw::c_int,
    pub contentType: xmlSchemaContentType,
    pub base: *const xmlChar,
    pub baseNs: *const xmlChar,
    pub baseType: xmlSchemaTypePtr,
    pub facets: xmlSchemaFacetPtr,
    pub redef: *mut _xmlSchemaType,
    pub recurse: std::os::raw::c_int,
    pub attributeUses: *mut xmlSchemaAttributeLinkPtr,
    pub attributeWildcard: xmlSchemaWildcardPtr,
    pub builtInType: std::os::raw::c_int,
    pub memberTypes: xmlSchemaTypeLinkPtr,
    pub facetSet: xmlSchemaFacetLinkPtr,
    pub refPrefix: *const xmlChar,
    pub contentTypeDef: xmlSchemaTypePtr,
    pub contModel: xmlRegexpPtr,
    pub targetNamespace: *const xmlChar,
    pub attrUses: *mut std::os::raw::c_void,
}
pub type xmlSchemaFacetLinkPtr = *mut xmlSchemaFacetLink;
pub type xmlSchemaFacetLink = _xmlSchemaFacetLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacetLink {
    pub next: *mut _xmlSchemaFacetLink,
    pub facet: xmlSchemaFacetPtr,
}
pub type xmlSchemaTypeLinkPtr = *mut xmlSchemaTypeLink;
pub type xmlSchemaTypeLink = _xmlSchemaTypeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaTypeLink {
    pub next: *mut _xmlSchemaTypeLink,
    pub type_0: xmlSchemaTypePtr,
}
pub type xmlSchemaWildcardPtr = *mut xmlSchemaWildcard;
pub type xmlSchemaWildcard = _xmlSchemaWildcard;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaWildcard {
    pub type_0: xmlSchemaTypeType,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub minOccurs: std::os::raw::c_int,
    pub maxOccurs: std::os::raw::c_int,
    pub processContents: std::os::raw::c_int,
    pub any: std::os::raw::c_int,
    pub nsSet: xmlSchemaWildcardNsPtr,
    pub negNsSet: xmlSchemaWildcardNsPtr,
    pub flags: std::os::raw::c_int,
}
pub type xmlSchemaWildcardNsPtr = *mut xmlSchemaWildcardNs;
pub type xmlSchemaWildcardNs = _xmlSchemaWildcardNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaWildcardNs {
    pub next: *mut _xmlSchemaWildcardNs,
    pub value: *const xmlChar,
}
pub type xmlSchemaAttributeLinkPtr = *mut xmlSchemaAttributeLink;
/* Deprecated; not used */
/* *
 * xmlSchemaAttributeLink:
 * Used to build a list of attribute uses on complexType definitions.
 * WARNING: Deprecated; not used.
 */
pub type xmlSchemaAttributeLink = _xmlSchemaAttributeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAttributeLink {
    pub next: *mut _xmlSchemaAttributeLink,
    pub attr: *mut _xmlSchemaAttribute,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAttribute {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaAttribute,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_0: *const xmlChar,
    pub refNs: *const xmlChar,
    pub typeName: *const xmlChar,
    pub typeNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub base: xmlSchemaTypePtr,
    pub occurs: std::os::raw::c_int,
    pub defValue: *const xmlChar,
    pub subtypes: xmlSchemaTypePtr,
    pub node: xmlNodePtr,
    pub targetNamespace: *const xmlChar,
    pub flags: std::os::raw::c_int,
    pub refPrefix: *const xmlChar,
    pub defVal: xmlSchemaValPtr,
    pub refDecl: xmlSchemaAttributePtr,
}
pub type xmlSchemaAttributePtr = *mut xmlSchemaAttribute;
/* the annotation */
/* *
 * XML_SCHEMAS_ANYATTR_SKIP:
 *
 * Skip unknown attribute from validation
 * Obsolete, not used anymore.
 */
/* *
 * XML_SCHEMAS_ANYATTR_LAX:
 *
 * Ignore validation non definition on attributes
 * Obsolete, not used anymore.
 */
/* *
 * XML_SCHEMAS_ANYATTR_STRICT:
 *
 * Apply strict validation rules on attributes
 * Obsolete, not used anymore.
 */
/* *
 * XML_SCHEMAS_ANY_SKIP:
 *
 * Skip unknown attribute from validation
 */
/* *
 * XML_SCHEMAS_ANY_LAX:
 *
 * Used by wildcards.
 * Validate if type found, don't worry if not found
 */
/* *
 * XML_SCHEMAS_ANY_STRICT:
 *
 * Used by wildcards.
 * Apply strict validation rules
 */
/* *
 * XML_SCHEMAS_ATTR_USE_PROHIBITED:
 *
 * Used by wildcards.
 * The attribute is prohibited.
 */
/* *
 * XML_SCHEMAS_ATTR_USE_REQUIRED:
 *
 * The attribute is required.
 */
/* *
 * XML_SCHEMAS_ATTR_USE_OPTIONAL:
 *
 * The attribute is optional.
 */
/* *
 * XML_SCHEMAS_ATTR_GLOBAL:
 *
 * allow elements in no namespace
 */
/* *
 * XML_SCHEMAS_ATTR_NSDEFAULT:
 *
 * allow elements in no namespace
 */
/* *
 * XML_SCHEMAS_ATTR_INTERNAL_RESOLVED:
 *
 * this is set when the "type" and "ref" references
 * have been resolved.
 */
/* *
 * XML_SCHEMAS_ATTR_FIXED:
 *
 * the attribute has a fixed value
 */
/* *
 * xmlSchemaAttribute:
 * An attribute definition.
 */
pub type xmlSchemaAttribute = _xmlSchemaAttribute;
pub type xmlSchemaContentType = std::os::raw::c_uint;
pub const XML_SCHEMA_CONTENT_ANY: xmlSchemaContentType = 7;
pub const XML_SCHEMA_CONTENT_BASIC: xmlSchemaContentType = 6;
pub const XML_SCHEMA_CONTENT_MIXED_OR_ELEMENTS: xmlSchemaContentType = 5;
pub const XML_SCHEMA_CONTENT_SIMPLE: xmlSchemaContentType = 4;
pub const XML_SCHEMA_CONTENT_MIXED: xmlSchemaContentType = 3;
pub const XML_SCHEMA_CONTENT_ELEMENTS: xmlSchemaContentType = 2;
pub const XML_SCHEMA_CONTENT_EMPTY: xmlSchemaContentType = 1;
pub const XML_SCHEMA_CONTENT_UNKNOWN: xmlSchemaContentType = 0;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlRelaxNGPartitionPtr = *mut xmlRelaxNGPartition;
/* *
 * xmlRelaxNGPartitions:
 *
 * A RelaxNGs partition associated to an interleave group
 */
pub type xmlRelaxNGPartition = _xmlRelaxNGPartition;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGPartition {
    pub nbgroups: std::os::raw::c_int,
    pub triage: xmlHashTablePtr,
    pub flags: std::os::raw::c_int,
    pub groups: *mut xmlRelaxNGInterleaveGroupPtr,
}
pub type xmlRelaxNGInterleaveGroupPtr = *mut xmlRelaxNGInterleaveGroup;
/* *
 * xmlRelaxNGInterleaveGroup:
 *
 * A RelaxNGs partition set associated to lists of definitions
 */
pub type xmlRelaxNGInterleaveGroup = _xmlRelaxNGInterleaveGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGInterleaveGroup {
    pub rule: xmlRelaxNGDefinePtr,
    pub defs: *mut xmlRelaxNGDefinePtr,
    pub attrs: *mut xmlRelaxNGDefinePtr,
}
pub type xmlRelaxNGContentType = std::os::raw::c_int;
pub const XML_RELAXNG_CONTENT_COMPLEX: xmlRelaxNGContentType = 2;
pub const XML_RELAXNG_CONTENT_SIMPLE: xmlRelaxNGContentType = 1;
pub const XML_RELAXNG_CONTENT_EMPTY: xmlRelaxNGContentType = 0;
pub const XML_RELAXNG_CONTENT_ERROR: xmlRelaxNGContentType = -1;
/*
 * relaxng.c : implementation of the Relax-NG handling and validity checking
 *
 * See Copyright for the status of this software.
 *
 * Daniel Veillard <veillard@redhat.com>
 */
/* *
 * TODO:
 * - add support for DTD compatibility spec
 *   http://www.oasis-open.org/committees/relax-ng/compatibility-20011203.html
 * - report better mem allocations pbms at runtime and abort immediately.
 */
/*
 * The Relax-NG namespace
 */
static mut xmlRelaxNGNs: *const xmlChar =
    b"http://relaxng.org/ns/structure/1.0\x00" as *const u8 as
        *const std::os::raw::c_char as *const xmlChar;
/* 1 if an external ref */
/* ***********************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/
/* *
 * xmlRngPErrMemory:
 * @ctxt:  an Relax-NG parser context
 * @extra:  extra informations
 *
 * Handle a redefinition of attribute error
 */
unsafe extern "C" fn xmlRngPErrMemory(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                      mut extra: *const std::os::raw::c_char) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        if (*ctxt).serror.is_some() {
            schannel = (*ctxt).serror
        } else { channel = (*ctxt).error }
        data = (*ctxt).userData;
        (*ctxt).nbErrors += 1
    }
    if !extra.is_null() {
        __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        XML_FROM_RELAXNGP as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"Memory allocation failed : %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, extra);
    } else {
        __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        XML_FROM_RELAXNGP as std::os::raw::c_int,
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
 * xmlRngVErrMemory:
 * @ctxt:  a Relax-NG validation context
 * @extra:  extra informations
 *
 * Handle a redefinition of attribute error
 */
unsafe extern "C" fn xmlRngVErrMemory(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                      mut extra: *const std::os::raw::c_char) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        if (*ctxt).serror.is_some() {
            schannel = (*ctxt).serror
        } else { channel = (*ctxt).error }
        data = (*ctxt).userData;
        (*ctxt).nbErrors += 1
    }
    if !extra.is_null() {
        __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        XML_FROM_RELAXNGV as std::os::raw::c_int,
                        XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_FATAL,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        b"Memory allocation failed : %s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, extra);
    } else {
        __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        XML_FROM_RELAXNGV as std::os::raw::c_int,
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
 * xmlRngPErr:
 * @ctxt:  a Relax-NG parser context
 * @node:  the node raising the error
 * @error:  the error code
 * @msg:  message
 * @str1:  extra info
 * @str2:  extra info
 *
 * Handle a Relax NG Parsing error
 */
unsafe extern "C" fn xmlRngPErr(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                mut node: xmlNodePtr, mut error: std::os::raw::c_int,
                                mut msg: *const std::os::raw::c_char,
                                mut str1: *const xmlChar,
                                mut str2: *const xmlChar) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        if (*ctxt).serror.is_some() {
            schannel = (*ctxt).serror
        } else { channel = (*ctxt).error }
        data = (*ctxt).userData;
        (*ctxt).nbErrors += 1
    }
    __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                    node as *mut std::os::raw::c_void,
                    XML_FROM_RELAXNGP as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2);
}
/* *
 * xmlRngVErr:
 * @ctxt:  a Relax-NG validation context
 * @node:  the node raising the error
 * @error:  the error code
 * @msg:  message
 * @str1:  extra info
 * @str2:  extra info
 *
 * Handle a Relax NG Validation error
 */
unsafe extern "C" fn xmlRngVErr(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                mut node: xmlNodePtr, mut error: std::os::raw::c_int,
                                mut msg: *const std::os::raw::c_char,
                                mut str1: *const xmlChar,
                                mut str2: *const xmlChar) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        if (*ctxt).serror.is_some() {
            schannel = (*ctxt).serror
        } else { channel = (*ctxt).error }
        data = (*ctxt).userData;
        (*ctxt).nbErrors += 1
    }
    __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                    node as *mut std::os::raw::c_void,
                    XML_FROM_RELAXNGV as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2);
}
/* *
 * xmlRelaxNGFreeDocument:
 * @docu:  a document structure
 *
 * Deallocate a RelaxNG document structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeDocument(mut docu: xmlRelaxNGDocumentPtr) {
    if docu.is_null() { return }
    if !(*docu).href.is_null() {
        xmlFree.expect("non-null function pointer")((*docu).href as
                                                        *mut std::os::raw::c_void);
    }
    if !(*docu).doc.is_null() { xmlFreeDoc((*docu).doc); }
    if !(*docu).schema.is_null() {
        xmlRelaxNGFreeInnerSchema((*docu).schema);
    }
    xmlFree.expect("non-null function pointer")(docu as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGFreeDocumentList:
 * @docu:  a list of  document structure
 *
 * Deallocate a RelaxNG document structures.
 */
unsafe extern "C" fn xmlRelaxNGFreeDocumentList(mut docu:
                                                    xmlRelaxNGDocumentPtr) {
    let mut next: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
    while !docu.is_null() {
        next = (*docu).next;
        xmlRelaxNGFreeDocument(docu);
        docu = next
    };
}
/* *
 * xmlRelaxNGFreeInclude:
 * @incl:  a include structure
 *
 * Deallocate a RelaxNG include structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeInclude(mut incl: xmlRelaxNGIncludePtr) {
    if incl.is_null() { return }
    if !(*incl).href.is_null() {
        xmlFree.expect("non-null function pointer")((*incl).href as
                                                        *mut std::os::raw::c_void);
    }
    if !(*incl).doc.is_null() { xmlFreeDoc((*incl).doc); }
    if !(*incl).schema.is_null() { xmlRelaxNGFree((*incl).schema); }
    xmlFree.expect("non-null function pointer")(incl as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGFreeIncludeList:
 * @incl:  a include structure list
 *
 * Deallocate a RelaxNG include structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeIncludeList(mut incl:
                                                   xmlRelaxNGIncludePtr) {
    let mut next: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
    while !incl.is_null() {
        next = (*incl).next;
        xmlRelaxNGFreeInclude(incl);
        incl = next
    };
}
/* *
 * xmlRelaxNGNewRelaxNG:
 * @ctxt:  a Relax-NG validation context (optional)
 *
 * Allocate a new RelaxNG structure.
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlRelaxNGNewRelaxNG(mut ctxt: xmlRelaxNGParserCtxtPtr)
 -> xmlRelaxNGPtr {
    let mut ret: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNG>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGPtr;
    if ret.is_null() {
        xmlRngPErrMemory(ctxt, 0 as *const std::os::raw::c_char);
        return 0 as xmlRelaxNGPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNG>() as std::os::raw::c_ulong);
    return ret;
}
/* *
 * xmlRelaxNGFreeInnerSchema:
 * @schema:  a schema structure
 *
 * Deallocate a RelaxNG schema structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeInnerSchema(mut schema: xmlRelaxNGPtr) {
    if schema.is_null() { return }
    if !(*schema).doc.is_null() { xmlFreeDoc((*schema).doc); }
    if !(*schema).defTab.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < (*schema).defNr {
            xmlRelaxNGFreeDefine(*(*schema).defTab.offset(i as isize));
            i += 1
        }
        xmlFree.expect("non-null function pointer")((*schema).defTab as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(schema as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGFree:
 * @schema:  a schema structure
 *
 * Deallocate a RelaxNG structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFree(mut schema: xmlRelaxNGPtr) {
    if schema.is_null() { return }
    if !(*schema).topgrammar.is_null() {
        xmlRelaxNGFreeGrammar((*schema).topgrammar);
    }
    if !(*schema).doc.is_null() { xmlFreeDoc((*schema).doc); }
    if !(*schema).documents.is_null() {
        xmlRelaxNGFreeDocumentList((*schema).documents);
    }
    if !(*schema).includes.is_null() {
        xmlRelaxNGFreeIncludeList((*schema).includes);
    }
    if !(*schema).defTab.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < (*schema).defNr {
            xmlRelaxNGFreeDefine(*(*schema).defTab.offset(i as isize));
            i += 1
        }
        xmlFree.expect("non-null function pointer")((*schema).defTab as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(schema as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGNewGrammar:
 * @ctxt:  a Relax-NG validation context (optional)
 *
 * Allocate a new RelaxNG grammar.
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlRelaxNGNewGrammar(mut ctxt: xmlRelaxNGParserCtxtPtr)
 -> xmlRelaxNGGrammarPtr {
    let mut ret: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGGrammar>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGGrammarPtr;
    if ret.is_null() {
        xmlRngPErrMemory(ctxt, 0 as *const std::os::raw::c_char);
        return 0 as xmlRelaxNGGrammarPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGGrammar>() as std::os::raw::c_ulong);
    return ret;
}
/* the freeing function */
/* ***********************************************************************
 *									*
 *			Allocation functions				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGFreeGrammar:
 * @grammar:  a grammar structure
 *
 * Deallocate a RelaxNG grammar structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeGrammar(mut grammar:
                                               xmlRelaxNGGrammarPtr) {
    if grammar.is_null() { return }
    if !(*grammar).children.is_null() {
        xmlRelaxNGFreeGrammar((*grammar).children);
    }
    if !(*grammar).next.is_null() { xmlRelaxNGFreeGrammar((*grammar).next); }
    if !(*grammar).refs.is_null() { xmlHashFree((*grammar).refs, None); }
    if !(*grammar).defs.is_null() { xmlHashFree((*grammar).defs, None); }
    xmlFree.expect("non-null function pointer")(grammar as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGNewDefine:
 * @ctxt:  a Relax-NG validation context
 * @node:  the node in the input document.
 *
 * Allocate a new RelaxNG define.
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlRelaxNGNewDefine(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                         mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if (*ctxt).defMax == 0 as std::os::raw::c_int {
        (*ctxt).defMax = 16 as std::os::raw::c_int;
        (*ctxt).defNr = 0 as std::os::raw::c_int;
        (*ctxt).defTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).defMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGDefinePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlRelaxNGDefinePtr;
        if (*ctxt).defTab.is_null() {
            xmlRngPErrMemory(ctxt,
                             b"allocating define\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as xmlRelaxNGDefinePtr
        }
    } else if (*ctxt).defMax <= (*ctxt).defNr {
        let mut tmp: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
        (*ctxt).defMax *= 2 as std::os::raw::c_int;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*ctxt).defTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).defMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGDefinePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRelaxNGDefinePtr;
        if tmp.is_null() {
            xmlRngPErrMemory(ctxt,
                             b"allocating define\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as xmlRelaxNGDefinePtr
        }
        (*ctxt).defTab = tmp
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGDefine>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGDefinePtr;
    if ret.is_null() {
        xmlRngPErrMemory(ctxt,
                         b"allocating define\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlRelaxNGDefinePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGDefine>() as std::os::raw::c_ulong);
    let fresh0 = (*ctxt).defNr;
    (*ctxt).defNr = (*ctxt).defNr + 1;
    let ref mut fresh1 = *(*ctxt).defTab.offset(fresh0 as isize);
    *fresh1 = ret;
    (*ret).node = node;
    (*ret).depth = -(1 as std::os::raw::c_int) as std::os::raw::c_short;
    return ret;
}
/* *
 * xmlRelaxNGFreePartition:
 * @partitions:  a partition set structure
 *
 * Deallocate RelaxNG partition set structures.
 */
unsafe extern "C" fn xmlRelaxNGFreePartition(mut partitions:
                                                 xmlRelaxNGPartitionPtr) {
    let mut group: xmlRelaxNGInterleaveGroupPtr =
        0 as *mut xmlRelaxNGInterleaveGroup;
    let mut j: std::os::raw::c_int = 0;
    if !partitions.is_null() {
        if !(*partitions).groups.is_null() {
            j = 0 as std::os::raw::c_int;
            while j < (*partitions).nbgroups {
                group = *(*partitions).groups.offset(j as isize);
                if !group.is_null() {
                    if !(*group).defs.is_null() {
                        xmlFree.expect("non-null function pointer")((*group).defs
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    }
                    if !(*group).attrs.is_null() {
                        xmlFree.expect("non-null function pointer")((*group).attrs
                                                                        as
                                                                        *mut std::os::raw::c_void);
                    }
                    xmlFree.expect("non-null function pointer")(group as
                                                                    *mut std::os::raw::c_void);
                }
                j += 1
            }
            xmlFree.expect("non-null function pointer")((*partitions).groups
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        if !(*partitions).triage.is_null() {
            xmlHashFree((*partitions).triage, None);
        }
        xmlFree.expect("non-null function pointer")(partitions as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlRelaxNGFreeDefine:
 * @define:  a define structure
 *
 * Deallocate a RelaxNG define structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeDefine(mut define: xmlRelaxNGDefinePtr) {
    if define.is_null() { return }
    if (*define).type_0 as std::os::raw::c_int == XML_RELAXNG_VALUE as std::os::raw::c_int &&
           !(*define).attrs.is_null() {
        let mut lib: xmlRelaxNGTypeLibraryPtr =
            0 as *mut xmlRelaxNGTypeLibrary;
        lib = (*define).data as xmlRelaxNGTypeLibraryPtr;
        if !lib.is_null() && (*lib).freef.is_some() {
            (*lib).freef.expect("non-null function pointer")((*lib).data,
                                                             (*define).attrs
                                                                 as
                                                                 *mut std::os::raw::c_void);
        }
    }
    if !(*define).data.is_null() &&
           (*define).type_0 as std::os::raw::c_int ==
               XML_RELAXNG_INTERLEAVE as std::os::raw::c_int {
        xmlRelaxNGFreePartition((*define).data as xmlRelaxNGPartitionPtr);
    }
    if !(*define).data.is_null() &&
           (*define).type_0 as std::os::raw::c_int ==
               XML_RELAXNG_CHOICE as std::os::raw::c_int {
        xmlHashFree((*define).data as xmlHashTablePtr, None);
    }
    if !(*define).name.is_null() {
        xmlFree.expect("non-null function pointer")((*define).name as
                                                        *mut std::os::raw::c_void);
    }
    if !(*define).ns.is_null() {
        xmlFree.expect("non-null function pointer")((*define).ns as
                                                        *mut std::os::raw::c_void);
    }
    if !(*define).value.is_null() {
        xmlFree.expect("non-null function pointer")((*define).value as
                                                        *mut std::os::raw::c_void);
    }
    if !(*define).contModel.is_null() {
        xmlRegFreeRegexp((*define).contModel);
    }
    xmlFree.expect("non-null function pointer")(define as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGNewStates:
 * @ctxt:  a Relax-NG validation context
 * @size:  the default size for the container
 *
 * Allocate a new RelaxNG validation state container
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlRelaxNGNewStates(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                         mut size: std::os::raw::c_int)
 -> xmlRelaxNGStatesPtr {
    let mut ret: xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStates;
    if !ctxt.is_null() && !(*ctxt).freeStates.is_null() &&
           (*ctxt).freeStatesNr > 0 as std::os::raw::c_int {
        (*ctxt).freeStatesNr -= 1;
        ret = *(*ctxt).freeStates.offset((*ctxt).freeStatesNr as isize);
        (*ret).nbState = 0 as std::os::raw::c_int;
        return ret
    }
    if size < 16 as std::os::raw::c_int { size = 16 as std::os::raw::c_int }
    ret =
        xmlMalloc.expect("non-null function pointer")((::std::mem::size_of::<xmlRelaxNGStates>()
                                                           as
                                                           std::os::raw::c_ulong).wrapping_add(((size
                                                                                             -
                                                                                             1
                                                                                                 as
                                                                                                 std::os::raw::c_int)
                                                                                            as
                                                                                            std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGValidStatePtr>()
                                                                                                                            as
                                                                                                                            std::os::raw::c_ulong)))
            as xmlRelaxNGStatesPtr;
    if ret.is_null() {
        xmlRngVErrMemory(ctxt,
                         b"allocating states\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlRelaxNGStatesPtr
    }
    (*ret).nbState = 0 as std::os::raw::c_int;
    (*ret).maxState = size;
    (*ret).tabState =
        xmlMalloc.expect("non-null function pointer")((size as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGValidStatePtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut xmlRelaxNGValidStatePtr;
    if (*ret).tabState.is_null() {
        xmlRngVErrMemory(ctxt,
                         b"allocating states\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlRelaxNGStatesPtr
    }
    return ret;
}
/* *
 * xmlRelaxNGAddStateUniq:
 * @ctxt:  a Relax-NG validation context
 * @states:  the states container
 * @state:  the validation state
 *
 * Add a RelaxNG validation state to the container without checking
 * for unicity.
 *
 * Return 1 in case of success and 0 if this is a duplicate and -1 on error
 */
unsafe extern "C" fn xmlRelaxNGAddStatesUniq(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                             mut states: xmlRelaxNGStatesPtr,
                                             mut state:
                                                 xmlRelaxNGValidStatePtr)
 -> std::os::raw::c_int {
    if state.is_null() { return -(1 as std::os::raw::c_int) }
    if (*states).nbState >= (*states).maxState {
        let mut tmp: *mut xmlRelaxNGValidStatePtr =
            0 as *mut xmlRelaxNGValidStatePtr;
        let mut size: std::os::raw::c_int = 0;
        size = (*states).maxState * 2 as std::os::raw::c_int;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*states).tabState
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           (size as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGValidStatePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRelaxNGValidStatePtr;
        if tmp.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"adding states\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*states).tabState = tmp;
        (*states).maxState = size
    }
    let fresh2 = (*states).nbState;
    (*states).nbState = (*states).nbState + 1;
    let ref mut fresh3 = *(*states).tabState.offset(fresh2 as isize);
    *fresh3 = state;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGAddState:
 * @ctxt:  a Relax-NG validation context
 * @states:  the states container
 * @state:  the validation state
 *
 * Add a RelaxNG validation state to the container
 *
 * Return 1 in case of success and 0 if this is a duplicate and -1 on error
 */
unsafe extern "C" fn xmlRelaxNGAddStates(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                         mut states: xmlRelaxNGStatesPtr,
                                         mut state: xmlRelaxNGValidStatePtr)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if state.is_null() || states.is_null() { return -(1 as std::os::raw::c_int) }
    if (*states).nbState >= (*states).maxState {
        let mut tmp: *mut xmlRelaxNGValidStatePtr =
            0 as *mut xmlRelaxNGValidStatePtr;
        let mut size: std::os::raw::c_int = 0;
        size = (*states).maxState * 2 as std::os::raw::c_int;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*states).tabState
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           (size as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGValidStatePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRelaxNGValidStatePtr;
        if tmp.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"adding states\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
        (*states).tabState = tmp;
        (*states).maxState = size
    }
    i = 0 as std::os::raw::c_int;
    while i < (*states).nbState {
        if xmlRelaxNGEqualValidState(ctxt, state,
                                     *(*states).tabState.offset(i as isize))
               != 0 {
            xmlRelaxNGFreeValidState(ctxt, state);
            return 0 as std::os::raw::c_int
        }
        i += 1
    }
    let fresh4 = (*states).nbState;
    (*states).nbState = (*states).nbState + 1;
    let ref mut fresh5 = *(*states).tabState.offset(fresh4 as isize);
    *fresh5 = state;
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGFreeStates:
 * @ctxt:  a Relax-NG validation context
 * @states:  teh container
 *
 * Free a RelaxNG validation state container
 */
unsafe extern "C" fn xmlRelaxNGFreeStates(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                          mut states: xmlRelaxNGStatesPtr) {
    if states.is_null() { return }
    if !ctxt.is_null() && (*ctxt).freeStates.is_null() {
        (*ctxt).freeStatesMax = 40 as std::os::raw::c_int;
        (*ctxt).freeStatesNr = 0 as std::os::raw::c_int;
        (*ctxt).freeStates =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).freeStatesMax
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGStatesPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlRelaxNGStatesPtr;
        if (*ctxt).freeStates.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"storing states\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
        }
    } else if !ctxt.is_null() && (*ctxt).freeStatesNr >= (*ctxt).freeStatesMax
     {
        let mut tmp: *mut xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStatesPtr;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*ctxt).freeStates
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           ((2 as std::os::raw::c_int
                                                                 *
                                                                 (*ctxt).freeStatesMax)
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGStatesPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRelaxNGStatesPtr;
        if tmp.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"storing states\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")((*states).tabState as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(states as
                                                            *mut std::os::raw::c_void);
            return
        }
        (*ctxt).freeStates = tmp;
        (*ctxt).freeStatesMax *= 2 as std::os::raw::c_int
    }
    if ctxt.is_null() || (*ctxt).freeStates.is_null() {
        xmlFree.expect("non-null function pointer")((*states).tabState as
                                                        *mut std::os::raw::c_void);
        xmlFree.expect("non-null function pointer")(states as
                                                        *mut std::os::raw::c_void);
    } else {
        let fresh6 = (*ctxt).freeStatesNr;
        (*ctxt).freeStatesNr = (*ctxt).freeStatesNr + 1;
        let ref mut fresh7 = *(*ctxt).freeStates.offset(fresh6 as isize);
        *fresh7 = states
    };
}
/* *
 * xmlRelaxNGNewValidState:
 * @ctxt:  a Relax-NG validation context
 * @node:  the current node or NULL for the document
 *
 * Allocate a new RelaxNG validation state
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlRelaxNGNewValidState(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                             mut node: xmlNodePtr)
 -> xmlRelaxNGValidStatePtr {
    let mut ret: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut attrs: [xmlAttrPtr; 20] = [0 as *mut xmlAttr; 20];
    let mut nbAttrs: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut root: xmlNodePtr = 0 as xmlNodePtr;
    if node.is_null() {
        root = xmlDocGetRootElement((*ctxt).doc as *const xmlDoc);
        if root.is_null() { return 0 as xmlRelaxNGValidStatePtr }
    } else {
        attr = (*node).properties;
        while !attr.is_null() {
            if nbAttrs < 20 as std::os::raw::c_int {
                let fresh8 = nbAttrs;
                nbAttrs = nbAttrs + 1;
                attrs[fresh8 as usize] = attr
            } else { nbAttrs += 1 }
            attr = (*attr).next
        }
    }
    if !(*ctxt).freeState.is_null() &&
           (*(*ctxt).freeState).nbState > 0 as std::os::raw::c_int {
        (*(*ctxt).freeState).nbState -= 1;
        ret =
            *(*(*ctxt).freeState).tabState.offset((*(*ctxt).freeState).nbState
                                                      as isize)
    } else {
        ret =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGValidState>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlRelaxNGValidStatePtr;
        if ret.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"allocating states\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as xmlRelaxNGValidStatePtr
        }
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlRelaxNGValidState>() as
                   std::os::raw::c_ulong);
    }
    (*ret).value = 0 as *mut xmlChar;
    (*ret).endvalue = 0 as *mut xmlChar;
    if node.is_null() {
        (*ret).node = (*ctxt).doc as xmlNodePtr;
        (*ret).seq = root
    } else { (*ret).node = node; (*ret).seq = (*node).children }
    (*ret).nbAttrs = 0 as std::os::raw::c_int;
    if nbAttrs > 0 as std::os::raw::c_int {
        if (*ret).attrs.is_null() {
            if nbAttrs < 4 as std::os::raw::c_int {
                (*ret).maxAttrs = 4 as std::os::raw::c_int
            } else { (*ret).maxAttrs = nbAttrs }
            (*ret).attrs =
                xmlMalloc.expect("non-null function pointer")(((*ret).maxAttrs
                                                                   as
                                                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlAttrPtr>()
                                                                                                   as
                                                                                                   std::os::raw::c_ulong))
                    as *mut xmlAttrPtr;
            if (*ret).attrs.is_null() {
                xmlRngVErrMemory(ctxt,
                                 b"allocating states\n\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return ret
            }
        } else if (*ret).maxAttrs < nbAttrs {
            let mut tmp: *mut xmlAttrPtr = 0 as *mut xmlAttrPtr;
            tmp =
                xmlRealloc.expect("non-null function pointer")((*ret).attrs as
                                                                   *mut std::os::raw::c_void,
                                                               (nbAttrs as
                                                                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlAttrPtr>()
                                                                                                    as
                                                                                                    std::os::raw::c_ulong))
                    as *mut xmlAttrPtr;
            if tmp.is_null() {
                xmlRngVErrMemory(ctxt,
                                 b"allocating states\n\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                return ret
            }
            (*ret).attrs = tmp;
            (*ret).maxAttrs = nbAttrs
        }
        (*ret).nbAttrs = nbAttrs;
        if nbAttrs < 20 as std::os::raw::c_int {
            memcpy((*ret).attrs as *mut std::os::raw::c_void,
                   attrs.as_mut_ptr() as *const std::os::raw::c_void,
                   (::std::mem::size_of::<xmlAttrPtr>() as
                        std::os::raw::c_ulong).wrapping_mul(nbAttrs as
                                                        std::os::raw::c_ulong));
        } else {
            attr = (*node).properties;
            nbAttrs = 0 as std::os::raw::c_int;
            while !attr.is_null() {
                let fresh9 = nbAttrs;
                nbAttrs = nbAttrs + 1;
                let ref mut fresh10 = *(*ret).attrs.offset(fresh9 as isize);
                *fresh10 = attr;
                attr = (*attr).next
            }
        }
    }
    (*ret).nbAttrLeft = (*ret).nbAttrs;
    return ret;
}
/* *
 * xmlRelaxNGCopyValidState:
 * @ctxt:  a Relax-NG validation context
 * @state:  a validation state
 *
 * Copy the validation state
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlRelaxNGCopyValidState(mut ctxt:
                                                  xmlRelaxNGValidCtxtPtr,
                                              mut state:
                                                  xmlRelaxNGValidStatePtr)
 -> xmlRelaxNGValidStatePtr {
    let mut ret: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut maxAttrs: std::os::raw::c_uint = 0;
    let mut attrs: *mut xmlAttrPtr = 0 as *mut xmlAttrPtr;
    if state.is_null() { return 0 as xmlRelaxNGValidStatePtr }
    if !(*ctxt).freeState.is_null() &&
           (*(*ctxt).freeState).nbState > 0 as std::os::raw::c_int {
        (*(*ctxt).freeState).nbState -= 1;
        ret =
            *(*(*ctxt).freeState).tabState.offset((*(*ctxt).freeState).nbState
                                                      as isize)
    } else {
        ret =
            xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGValidState>()
                                                              as
                                                              std::os::raw::c_ulong)
                as xmlRelaxNGValidStatePtr;
        if ret.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"allocating states\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as xmlRelaxNGValidStatePtr
        }
        memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlRelaxNGValidState>() as
                   std::os::raw::c_ulong);
    }
    attrs = (*ret).attrs;
    maxAttrs = (*ret).maxAttrs as std::os::raw::c_uint;
    memcpy(ret as *mut std::os::raw::c_void, state as *const std::os::raw::c_void,
           ::std::mem::size_of::<xmlRelaxNGValidState>() as std::os::raw::c_ulong);
    (*ret).attrs = attrs;
    (*ret).maxAttrs = maxAttrs as std::os::raw::c_int;
    if (*state).nbAttrs > 0 as std::os::raw::c_int {
        if (*ret).attrs.is_null() {
            (*ret).maxAttrs = (*state).maxAttrs;
            (*ret).attrs =
                xmlMalloc.expect("non-null function pointer")(((*ret).maxAttrs
                                                                   as
                                                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlAttrPtr>()
                                                                                                   as
                                                                                                   std::os::raw::c_ulong))
                    as *mut xmlAttrPtr;
            if (*ret).attrs.is_null() {
                xmlRngVErrMemory(ctxt,
                                 b"allocating states\n\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                (*ret).nbAttrs = 0 as std::os::raw::c_int;
                return ret
            }
        } else if (*ret).maxAttrs < (*state).nbAttrs {
            let mut tmp: *mut xmlAttrPtr = 0 as *mut xmlAttrPtr;
            tmp =
                xmlRealloc.expect("non-null function pointer")((*ret).attrs as
                                                                   *mut std::os::raw::c_void,
                                                               ((*state).maxAttrs
                                                                    as
                                                                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlAttrPtr>()
                                                                                                    as
                                                                                                    std::os::raw::c_ulong))
                    as *mut xmlAttrPtr;
            if tmp.is_null() {
                xmlRngVErrMemory(ctxt,
                                 b"allocating states\n\x00" as *const u8 as
                                     *const std::os::raw::c_char);
                (*ret).nbAttrs = 0 as std::os::raw::c_int;
                return ret
            }
            (*ret).maxAttrs = (*state).maxAttrs;
            (*ret).attrs = tmp
        }
        memcpy((*ret).attrs as *mut std::os::raw::c_void,
               (*state).attrs as *const std::os::raw::c_void,
               ((*state).nbAttrs as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlAttrPtr>()
                                                    as std::os::raw::c_ulong));
    }
    return ret;
}
/* *
 * xmlRelaxNGEqualValidState:
 * @ctxt:  a Relax-NG validation context
 * @state1:  a validation state
 * @state2:  a validation state
 *
 * Compare the validation states for equality
 *
 * Returns 1 if equald, 0 otherwise
 */
unsafe extern "C" fn xmlRelaxNGEqualValidState(mut ctxt:
                                                   xmlRelaxNGValidCtxtPtr,
                                               mut state1:
                                                   xmlRelaxNGValidStatePtr,
                                               mut state2:
                                                   xmlRelaxNGValidStatePtr)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if state1.is_null() || state2.is_null() { return 0 as std::os::raw::c_int }
    if state1 == state2 { return 1 as std::os::raw::c_int }
    if (*state1).node != (*state2).node { return 0 as std::os::raw::c_int }
    if (*state1).seq != (*state2).seq { return 0 as std::os::raw::c_int }
    if (*state1).nbAttrLeft != (*state2).nbAttrLeft {
        return 0 as std::os::raw::c_int
    }
    if (*state1).nbAttrs != (*state2).nbAttrs { return 0 as std::os::raw::c_int }
    if (*state1).endvalue != (*state2).endvalue { return 0 as std::os::raw::c_int }
    if (*state1).value != (*state2).value &&
           xmlStrEqual((*state1).value, (*state2).value) == 0 {
        return 0 as std::os::raw::c_int
    }
    i = 0 as std::os::raw::c_int;
    while i < (*state1).nbAttrs {
        if *(*state1).attrs.offset(i as isize) !=
               *(*state2).attrs.offset(i as isize) {
            return 0 as std::os::raw::c_int
        }
        i += 1
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGFreeValidState:
 * @state:  a validation state structure
 *
 * Deallocate a RelaxNG validation state structure.
 */
unsafe extern "C" fn xmlRelaxNGFreeValidState(mut ctxt:
                                                  xmlRelaxNGValidCtxtPtr,
                                              mut state:
                                                  xmlRelaxNGValidStatePtr) {
    if state.is_null() { return }
    if !ctxt.is_null() && (*ctxt).freeState.is_null() {
        (*ctxt).freeState = xmlRelaxNGNewStates(ctxt, 40 as std::os::raw::c_int)
    }
    if ctxt.is_null() || (*ctxt).freeState.is_null() {
        if !(*state).attrs.is_null() {
            xmlFree.expect("non-null function pointer")((*state).attrs as
                                                            *mut std::os::raw::c_void);
        }
        xmlFree.expect("non-null function pointer")(state as
                                                        *mut std::os::raw::c_void);
    } else { xmlRelaxNGAddStatesUniq(ctxt, (*ctxt).freeState, state); };
}
/* ***********************************************************************
 *									*
 *			Semi internal functions				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxParserSetFlag:
 * @ctxt: a RelaxNG parser context
 * @flags: a set of flags values
 *
 * Semi private function used to pass informations to a parser context
 * which are a combination of xmlRelaxNGParserFlag .
 *
 * Returns 0 if success and -1 in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxParserSetFlag(mut ctxt:
                                                   xmlRelaxNGParserCtxtPtr,
                                               mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if flags & XML_RELAXNGP_FREE_DOC as std::os::raw::c_int != 0 {
        (*ctxt).crng |= XML_RELAXNGP_FREE_DOC as std::os::raw::c_int;
        flags -= XML_RELAXNGP_FREE_DOC as std::os::raw::c_int
    }
    if flags & XML_RELAXNGP_CRNG as std::os::raw::c_int != 0 {
        (*ctxt).crng |= XML_RELAXNGP_CRNG as std::os::raw::c_int;
        flags -= XML_RELAXNGP_CRNG as std::os::raw::c_int
    }
    if flags != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGIncludePush:
 * @ctxt:  the parser context
 * @value:  the element doc
 *
 * Pushes a new include on top of the include stack
 *
 * Returns 0 in case of error, the index in the stack otherwise
 */
unsafe extern "C" fn xmlRelaxNGIncludePush(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                           mut value: xmlRelaxNGIncludePtr)
 -> std::os::raw::c_int {
    if (*ctxt).incTab.is_null() {
        (*ctxt).incMax = 4 as std::os::raw::c_int;
        (*ctxt).incNr = 0 as std::os::raw::c_int;
        (*ctxt).incTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).incMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGIncludePtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlRelaxNGIncludePtr;
        if (*ctxt).incTab.is_null() {
            xmlRngPErrMemory(ctxt,
                             b"allocating include\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    if (*ctxt).incNr >= (*ctxt).incMax {
        (*ctxt).incMax *= 2 as std::os::raw::c_int;
        (*ctxt).incTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).incTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).incMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGIncludePtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRelaxNGIncludePtr;
        if (*ctxt).incTab.is_null() {
            xmlRngPErrMemory(ctxt,
                             b"allocating include\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    let ref mut fresh11 = *(*ctxt).incTab.offset((*ctxt).incNr as isize);
    *fresh11 = value;
    (*ctxt).inc = value;
    let fresh12 = (*ctxt).incNr;
    (*ctxt).incNr = (*ctxt).incNr + 1;
    return fresh12;
}
/* *
 * xmlRelaxNGIncludePop:
 * @ctxt: the parser context
 *
 * Pops the top include from the include stack
 *
 * Returns the include just removed
 */
unsafe extern "C" fn xmlRelaxNGIncludePop(mut ctxt: xmlRelaxNGParserCtxtPtr)
 -> xmlRelaxNGIncludePtr {
    let mut ret: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
    if (*ctxt).incNr <= 0 as std::os::raw::c_int { return 0 as xmlRelaxNGIncludePtr }
    (*ctxt).incNr -= 1;
    if (*ctxt).incNr > 0 as std::os::raw::c_int {
        (*ctxt).inc =
            *(*ctxt).incTab.offset(((*ctxt).incNr - 1 as std::os::raw::c_int) as
                                       isize)
    } else { (*ctxt).inc = 0 as xmlRelaxNGIncludePtr }
    ret = *(*ctxt).incTab.offset((*ctxt).incNr as isize);
    let ref mut fresh13 = *(*ctxt).incTab.offset((*ctxt).incNr as isize);
    *fresh13 = 0 as xmlRelaxNGIncludePtr;
    return ret;
}
/* *
 * xmlRelaxNGRemoveRedefine:
 * @ctxt: the parser context
 * @URL:  the normalized URL
 * @target:  the included target
 * @name:  the define name to eliminate
 *
 * Applies the elimination algorithm of 4.7
 *
 * Returns 0 in case of error, 1 in case of success.
 */
unsafe extern "C" fn xmlRelaxNGRemoveRedefine(mut ctxt:
                                                  xmlRelaxNGParserCtxtPtr,
                                              mut URL: *const xmlChar,
                                              mut target: xmlNodePtr,
                                              mut name: *const xmlChar)
 -> std::os::raw::c_int {
    let mut found: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp2: xmlNodePtr = 0 as *mut xmlNode;
    let mut name2: *mut xmlChar = 0 as *mut xmlChar;
    tmp = target;
    while !tmp.is_null() {
        tmp2 = (*tmp).next;
        if name.is_null() &&
               (!tmp.is_null() && !(*tmp).ns.is_null() &&
                    (*tmp).type_0 as std::os::raw::c_uint ==
                        XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                    xmlStrEqual((*tmp).name,
                                b"start\x00" as *const u8 as
                                    *const std::os::raw::c_char as *const xmlChar) !=
                        0 &&
                    xmlStrEqual((*(*tmp).ns).href, xmlRelaxNGNs) != 0) {
            found = 1 as std::os::raw::c_int;
            xmlUnlinkNode(tmp);
            xmlFreeNode(tmp);
        } else if !name.is_null() &&
                      (!tmp.is_null() && !(*tmp).ns.is_null() &&
                           (*tmp).type_0 as std::os::raw::c_uint ==
                               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                           &&
                           xmlStrEqual((*tmp).name,
                                       b"define\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *const xmlChar) != 0 &&
                           xmlStrEqual((*(*tmp).ns).href, xmlRelaxNGNs) != 0)
         {
            name2 =
                xmlGetProp(tmp as *const xmlNode,
                           b"name\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar);
            xmlRelaxNGNormExtSpace(name2);
            if !name2.is_null() {
                if xmlStrEqual(name, name2) != 0 {
                    found = 1 as std::os::raw::c_int;
                    xmlUnlinkNode(tmp);
                    xmlFreeNode(tmp);
                }
                xmlFree.expect("non-null function pointer")(name2 as
                                                                *mut std::os::raw::c_void);
            }
        } else if !tmp.is_null() && !(*tmp).ns.is_null() &&
                      (*tmp).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      xmlStrEqual((*tmp).name,
                                  b"include\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      xmlStrEqual((*(*tmp).ns).href, xmlRelaxNGNs) != 0 {
            let mut href: *mut xmlChar = 0 as *mut xmlChar;
            let mut inc: xmlRelaxNGDocumentPtr =
                (*tmp).psvi as xmlRelaxNGDocumentPtr;
            if !inc.is_null() && !(*inc).doc.is_null() &&
                   !(*(*inc).doc).children.is_null() {
                if xmlStrEqual((*(*(*inc).doc).children).name,
                               b"grammar\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
                    if xmlRelaxNGRemoveRedefine(ctxt, href,
                                                (*xmlDocGetRootElement((*inc).doc
                                                                           as
                                                                           *const xmlDoc)).children,
                                                name) == 1 as std::os::raw::c_int {
                        found = 1 as std::os::raw::c_int
                    }
                }
            }
        }
        tmp = tmp2
    }
    return found;
}
/* *
 * xmlRelaxNGLoadInclude:
 * @ctxt: the parser context
 * @URL:  the normalized URL
 * @node: the include node.
 * @ns:  the namespace passed from the context.
 *
 * First lookup if the document is already loaded into the parser context,
 * check against recursion. If not found the resource is loaded and
 * the content is preprocessed before being returned back to the caller.
 *
 * Returns the xmlRelaxNGIncludePtr or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGLoadInclude(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                           mut URL: *const xmlChar,
                                           mut node: xmlNodePtr,
                                           mut ns: *const xmlChar)
 -> xmlRelaxNGIncludePtr {
    let mut ret: xmlRelaxNGIncludePtr = 0 as xmlRelaxNGIncludePtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut i: std::os::raw::c_int = 0;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * check against recursion in the stack
     */
    i = 0 as std::os::raw::c_int;
    while i < (*ctxt).incNr {
        if xmlStrEqual((**(*ctxt).incTab.offset(i as isize)).href, URL) != 0 {
            xmlRngPErr(ctxt, 0 as xmlNodePtr,
                       XML_RNGP_INCLUDE_RECURSE as std::os::raw::c_int,
                       b"Detected an Include recursion for %s\n\x00" as
                           *const u8 as *const std::os::raw::c_char, URL,
                       0 as *const xmlChar);
            return 0 as xmlRelaxNGIncludePtr
        }
        i += 1
    }
    /*
     * load the document
     */
    doc =
        xmlReadFile(URL as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int);
    if doc.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_PARSE_ERROR as std::os::raw::c_int,
                   b"xmlRelaxNG: could not load %s\n\x00" as *const u8 as
                       *const std::os::raw::c_char, URL, 0 as *const xmlChar);
        return 0 as xmlRelaxNGIncludePtr
    }
    /*
     * Allocate the document structures and register it first.
     */
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGInclude>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGIncludePtr;
    if ret.is_null() {
        xmlRngPErrMemory(ctxt,
                         b"allocating include\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        xmlFreeDoc(doc);
        return 0 as xmlRelaxNGIncludePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGInclude>() as std::os::raw::c_ulong);
    (*ret).doc = doc;
    (*ret).href = xmlStrdup(URL);
    (*ret).next = (*ctxt).includes;
    (*ctxt).includes = ret;
    /*
     * transmit the ns if needed
     */
    if !ns.is_null() {
        root = xmlDocGetRootElement(doc as *const xmlDoc);
        if !root.is_null() {
            if xmlHasProp(root as *const xmlNode,
                          b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar).is_null() {
                xmlSetProp(root,
                           b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar, ns);
            }
        }
    }
    /*
     * push it on the stack
     */
    xmlRelaxNGIncludePush(ctxt, ret);
    /*
     * Some preprocessing of the document content, this include recursing
     * in the include stack.
     */
    doc = xmlRelaxNGCleanupDoc(ctxt, doc);
    if doc.is_null() {
        (*ctxt).inc = 0 as xmlRelaxNGIncludePtr;
        return 0 as xmlRelaxNGIncludePtr
    }
    /*
     * Pop up the include from the stack
     */
    xmlRelaxNGIncludePop(ctxt);
    /*
     * Check that the top element is a grammar
     */
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_EMPTY as std::os::raw::c_int,
                   b"xmlRelaxNG: included document is empty %s\n\x00" as
                       *const u8 as *const std::os::raw::c_char, URL,
                   0 as *const xmlChar);
        return 0 as xmlRelaxNGIncludePtr
    }
    if !(!root.is_null() && !(*root).ns.is_null() &&
             (*root).type_0 as std::os::raw::c_uint ==
                 XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
             xmlStrEqual((*root).name,
                         b"grammar\x00" as *const u8 as *const std::os::raw::c_char as
                             *const xmlChar) != 0 &&
             xmlStrEqual((*(*root).ns).href, xmlRelaxNGNs) != 0) {
        xmlRngPErr(ctxt, node, XML_RNGP_GRAMMAR_MISSING as std::os::raw::c_int,
                   b"xmlRelaxNG: included document %s root is not a grammar\n\x00"
                       as *const u8 as *const std::os::raw::c_char, URL,
                   0 as *const xmlChar);
        return 0 as xmlRelaxNGIncludePtr
    }
    /*
     * Elimination of redefined rules in the include.
     */
    cur = (*node).children;
    while !cur.is_null() {
        if !cur.is_null() && !(*cur).ns.is_null() &&
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*cur).name,
                           b"start\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar) != 0 &&
               xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0 {
            let mut found: std::os::raw::c_int = 0 as std::os::raw::c_int;
            found =
                xmlRelaxNGRemoveRedefine(ctxt, URL, (*root).children,
                                         0 as *const xmlChar);
            if found == 0 {
                xmlRngPErr(ctxt, node, XML_RNGP_START_MISSING as std::os::raw::c_int,
                           b"xmlRelaxNG: include %s has a start but not the included grammar\n\x00"
                               as *const u8 as *const std::os::raw::c_char, URL,
                           0 as *const xmlChar);
            }
        } else if !cur.is_null() && !(*cur).ns.is_null() &&
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      xmlStrEqual((*cur).name,
                                  b"define\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0 {
            let mut name: *mut xmlChar = 0 as *mut xmlChar;
            name =
                xmlGetProp(cur as *const xmlNode,
                           b"name\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar);
            if name.is_null() {
                xmlRngPErr(ctxt, node, XML_RNGP_NAME_MISSING as std::os::raw::c_int,
                           b"xmlRelaxNG: include %s has define without name\n\x00"
                               as *const u8 as *const std::os::raw::c_char, URL,
                           0 as *const xmlChar);
            } else {
                let mut found_0: std::os::raw::c_int = 0;
                xmlRelaxNGNormExtSpace(name);
                found_0 =
                    xmlRelaxNGRemoveRedefine(ctxt, URL, (*root).children,
                                             name);
                if found_0 == 0 {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_DEFINE_MISSING as std::os::raw::c_int,
                               b"xmlRelaxNG: include %s has a define %s but not the included grammar\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, URL,
                               name);
                }
                xmlFree.expect("non-null function pointer")(name as
                                                                *mut std::os::raw::c_void);
            }
        }
        cur = (*cur).next
    }
    return ret;
}
/* *
 * xmlRelaxNGValidErrorPush:
 * @ctxt:  the validation context
 * @err:  the error code
 * @arg1:  the first string argument
 * @arg2:  the second string argument
 * @dup:  arg need to be duplicated
 *
 * Pushes a new error on top of the error stack
 *
 * Returns 0 in case of error, the index in the stack otherwise
 */
unsafe extern "C" fn xmlRelaxNGValidErrorPush(mut ctxt:
                                                  xmlRelaxNGValidCtxtPtr,
                                              mut err: xmlRelaxNGValidErr,
                                              mut arg1: *const xmlChar,
                                              mut arg2: *const xmlChar,
                                              mut dup: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    if (*ctxt).errTab.is_null() {
        (*ctxt).errMax = 8 as std::os::raw::c_int;
        (*ctxt).errNr = 0 as std::os::raw::c_int;
        (*ctxt).errTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).errMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGValidError>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as xmlRelaxNGValidErrorPtr;
        if (*ctxt).errTab.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"pushing error\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
        (*ctxt).err = 0 as xmlRelaxNGValidErrorPtr
    }
    if (*ctxt).errNr >= (*ctxt).errMax {
        (*ctxt).errMax *= 2 as std::os::raw::c_int;
        (*ctxt).errTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).errTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).errMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGValidError>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as xmlRelaxNGValidErrorPtr;
        if (*ctxt).errTab.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"pushing error\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
        (*ctxt).err =
            &mut *(*ctxt).errTab.offset(((*ctxt).errNr - 1 as std::os::raw::c_int) as
                                            isize) as
                *mut xmlRelaxNGValidError
    }
    if !(*ctxt).err.is_null() && !(*ctxt).state.is_null() &&
           (*(*ctxt).err).node == (*(*ctxt).state).node &&
           (*(*ctxt).err).err as std::os::raw::c_uint == err as std::os::raw::c_uint {
        return (*ctxt).errNr
    }
    cur =
        &mut *(*ctxt).errTab.offset((*ctxt).errNr as isize) as
            *mut xmlRelaxNGValidError;
    (*cur).err = err;
    if dup != 0 {
        (*cur).arg1 = xmlStrdup(arg1);
        (*cur).arg2 = xmlStrdup(arg2);
        (*cur).flags = 1 as std::os::raw::c_int
    } else {
        (*cur).arg1 = arg1;
        (*cur).arg2 = arg2;
        (*cur).flags = 0 as std::os::raw::c_int
    }
    if !(*ctxt).state.is_null() {
        (*cur).node = (*(*ctxt).state).node;
        (*cur).seq = (*(*ctxt).state).seq
    } else { (*cur).node = 0 as xmlNodePtr; (*cur).seq = 0 as xmlNodePtr }
    (*ctxt).err = cur;
    let fresh14 = (*ctxt).errNr;
    (*ctxt).errNr = (*ctxt).errNr + 1;
    return fresh14;
}
/* *
 * xmlRelaxNGValidErrorPop:
 * @ctxt: the validation context
 *
 * Pops the top error from the error stack
 */
unsafe extern "C" fn xmlRelaxNGValidErrorPop(mut ctxt:
                                                 xmlRelaxNGValidCtxtPtr) {
    let mut cur: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    if (*ctxt).errNr <= 0 as std::os::raw::c_int {
        (*ctxt).err = 0 as xmlRelaxNGValidErrorPtr;
        return
    }
    (*ctxt).errNr -= 1;
    if (*ctxt).errNr > 0 as std::os::raw::c_int {
        (*ctxt).err =
            &mut *(*ctxt).errTab.offset(((*ctxt).errNr - 1 as std::os::raw::c_int) as
                                            isize) as
                *mut xmlRelaxNGValidError
    } else { (*ctxt).err = 0 as xmlRelaxNGValidErrorPtr }
    cur =
        &mut *(*ctxt).errTab.offset((*ctxt).errNr as isize) as
            *mut xmlRelaxNGValidError;
    if (*cur).flags & 1 as std::os::raw::c_int != 0 {
        if !(*cur).arg1.is_null() {
            xmlFree.expect("non-null function pointer")((*cur).arg1 as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        (*cur).arg1 = 0 as *const xmlChar;
        if !(*cur).arg2.is_null() {
            xmlFree.expect("non-null function pointer")((*cur).arg2 as
                                                            *mut xmlChar as
                                                            *mut std::os::raw::c_void);
        }
        (*cur).arg2 = 0 as *const xmlChar;
        (*cur).flags = 0 as std::os::raw::c_int
    };
}
/* *
 * xmlRelaxNGDocumentPush:
 * @ctxt:  the parser context
 * @value:  the element doc
 *
 * Pushes a new doc on top of the doc stack
 *
 * Returns 0 in case of error, the index in the stack otherwise
 */
unsafe extern "C" fn xmlRelaxNGDocumentPush(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                            mut value: xmlRelaxNGDocumentPtr)
 -> std::os::raw::c_int {
    if (*ctxt).docTab.is_null() {
        (*ctxt).docMax = 4 as std::os::raw::c_int;
        (*ctxt).docNr = 0 as std::os::raw::c_int;
        (*ctxt).docTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).docMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGDocumentPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlRelaxNGDocumentPtr;
        if (*ctxt).docTab.is_null() {
            xmlRngPErrMemory(ctxt,
                             b"adding document\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    if (*ctxt).docNr >= (*ctxt).docMax {
        (*ctxt).docMax *= 2 as std::os::raw::c_int;
        (*ctxt).docTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).docTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).docMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGDocumentPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRelaxNGDocumentPtr;
        if (*ctxt).docTab.is_null() {
            xmlRngPErrMemory(ctxt,
                             b"adding document\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int
        }
    }
    let ref mut fresh15 = *(*ctxt).docTab.offset((*ctxt).docNr as isize);
    *fresh15 = value;
    (*ctxt).doc = value;
    let fresh16 = (*ctxt).docNr;
    (*ctxt).docNr = (*ctxt).docNr + 1;
    return fresh16;
}
/* *
 * xmlRelaxNGDocumentPop:
 * @ctxt: the parser context
 *
 * Pops the top doc from the doc stack
 *
 * Returns the doc just removed
 */
unsafe extern "C" fn xmlRelaxNGDocumentPop(mut ctxt: xmlRelaxNGParserCtxtPtr)
 -> xmlRelaxNGDocumentPtr {
    let mut ret: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
    if (*ctxt).docNr <= 0 as std::os::raw::c_int { return 0 as xmlRelaxNGDocumentPtr }
    (*ctxt).docNr -= 1;
    if (*ctxt).docNr > 0 as std::os::raw::c_int {
        (*ctxt).doc =
            *(*ctxt).docTab.offset(((*ctxt).docNr - 1 as std::os::raw::c_int) as
                                       isize)
    } else { (*ctxt).doc = 0 as xmlRelaxNGDocumentPtr }
    ret = *(*ctxt).docTab.offset((*ctxt).docNr as isize);
    let ref mut fresh17 = *(*ctxt).docTab.offset((*ctxt).docNr as isize);
    *fresh17 = 0 as xmlRelaxNGDocumentPtr;
    return ret;
}
/* *
 * xmlRelaxNGLoadExternalRef:
 * @ctxt: the parser context
 * @URL:  the normalized URL
 * @ns:  the inherited ns if any
 *
 * First lookup if the document is already loaded into the parser context,
 * check against recursion. If not found the resource is loaded and
 * the content is preprocessed before being returned back to the caller.
 *
 * Returns the xmlRelaxNGDocumentPtr or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGLoadExternalRef(mut ctxt:
                                                   xmlRelaxNGParserCtxtPtr,
                                               mut URL: *const xmlChar,
                                               mut ns: *const xmlChar)
 -> xmlRelaxNGDocumentPtr {
    let mut ret: xmlRelaxNGDocumentPtr = 0 as xmlRelaxNGDocumentPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut i: std::os::raw::c_int = 0;
    /*
     * check against recursion in the stack
     */
    i = 0 as std::os::raw::c_int;
    while i < (*ctxt).docNr {
        if xmlStrEqual((**(*ctxt).docTab.offset(i as isize)).href, URL) != 0 {
            xmlRngPErr(ctxt, 0 as xmlNodePtr,
                       XML_RNGP_EXTERNALREF_RECURSE as std::os::raw::c_int,
                       b"Detected an externalRef recursion for %s\n\x00" as
                           *const u8 as *const std::os::raw::c_char, URL,
                       0 as *const xmlChar);
            return 0 as xmlRelaxNGDocumentPtr
        }
        i += 1
    }
    /*
     * load the document
     */
    doc =
        xmlReadFile(URL as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int);
    if doc.is_null() {
        xmlRngPErr(ctxt, 0 as xmlNodePtr, XML_RNGP_PARSE_ERROR as std::os::raw::c_int,
                   b"xmlRelaxNG: could not load %s\n\x00" as *const u8 as
                       *const std::os::raw::c_char, URL, 0 as *const xmlChar);
        return 0 as xmlRelaxNGDocumentPtr
    }
    /*
     * Allocate the document structures and register it first.
     */
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGDocument>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGDocumentPtr;
    if ret.is_null() {
        xmlRngPErr(ctxt, doc as xmlNodePtr, XML_ERR_NO_MEMORY as std::os::raw::c_int,
                   b"xmlRelaxNG: allocate memory for doc %s\n\x00" as
                       *const u8 as *const std::os::raw::c_char, URL,
                   0 as *const xmlChar);
        xmlFreeDoc(doc);
        return 0 as xmlRelaxNGDocumentPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGDocument>() as std::os::raw::c_ulong);
    (*ret).doc = doc;
    (*ret).href = xmlStrdup(URL);
    (*ret).next = (*ctxt).documents;
    (*ret).externalRef = 1 as std::os::raw::c_int;
    (*ctxt).documents = ret;
    /*
     * transmit the ns if needed
     */
    if !ns.is_null() {
        root = xmlDocGetRootElement(doc as *const xmlDoc);
        if !root.is_null() {
            if xmlHasProp(root as *const xmlNode,
                          b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar).is_null() {
                xmlSetProp(root,
                           b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar, ns);
            }
        }
    }
    /*
     * push it on the stack and register it in the hash table
     */
    xmlRelaxNGDocumentPush(ctxt, ret);
    /*
     * Some preprocessing of the document content
     */
    doc = xmlRelaxNGCleanupDoc(ctxt, doc);
    if doc.is_null() {
        (*ctxt).doc = 0 as xmlRelaxNGDocumentPtr;
        return 0 as xmlRelaxNGDocumentPtr
    }
    xmlRelaxNGDocumentPop(ctxt);
    return ret;
}
/* ***********************************************************************
 *									*
 *			Error functions					*
 *									*
 ************************************************************************/
unsafe extern "C" fn xmlRelaxNGDefName(mut def: xmlRelaxNGDefinePtr)
 -> *const std::os::raw::c_char {
    if def.is_null() {
        return b"none\x00" as *const u8 as *const std::os::raw::c_char
    }
    match (*def).type_0 as std::os::raw::c_int {
        0 => { return b"empty\x00" as *const u8 as *const std::os::raw::c_char }
        1 => { return b"notAllowed\x00" as *const u8 as *const std::os::raw::c_char }
        2 => { return b"except\x00" as *const u8 as *const std::os::raw::c_char }
        3 => { return b"text\x00" as *const u8 as *const std::os::raw::c_char }
        4 => { return b"element\x00" as *const u8 as *const std::os::raw::c_char }
        5 => { return b"datatype\x00" as *const u8 as *const std::os::raw::c_char }
        7 => { return b"value\x00" as *const u8 as *const std::os::raw::c_char }
        8 => { return b"list\x00" as *const u8 as *const std::os::raw::c_char }
        9 => { return b"attribute\x00" as *const u8 as *const std::os::raw::c_char }
        10 => { return b"def\x00" as *const u8 as *const std::os::raw::c_char }
        11 => { return b"ref\x00" as *const u8 as *const std::os::raw::c_char }
        12 => {
            return b"externalRef\x00" as *const u8 as *const std::os::raw::c_char
        }
        13 => { return b"parentRef\x00" as *const u8 as *const std::os::raw::c_char }
        14 => { return b"optional\x00" as *const u8 as *const std::os::raw::c_char }
        15 => { return b"zeroOrMore\x00" as *const u8 as *const std::os::raw::c_char }
        16 => { return b"oneOrMore\x00" as *const u8 as *const std::os::raw::c_char }
        17 => { return b"choice\x00" as *const u8 as *const std::os::raw::c_char }
        18 => { return b"group\x00" as *const u8 as *const std::os::raw::c_char }
        19 => { return b"interleave\x00" as *const u8 as *const std::os::raw::c_char }
        20 => { return b"start\x00" as *const u8 as *const std::os::raw::c_char }
        -1 => { return b"noop\x00" as *const u8 as *const std::os::raw::c_char }
        6 => { return b"param\x00" as *const u8 as *const std::os::raw::c_char }
        _ => { }
    }
    return b"unknown\x00" as *const u8 as *const std::os::raw::c_char;
}
/* *
 * xmlRelaxNGGetErrorString:
 * @err:  the error code
 * @arg1:  the first string argument
 * @arg2:  the second string argument
 *
 * computes a formatted error string for the given error code and args
 *
 * Returns the error string, it must be deallocated by the caller
 */
unsafe extern "C" fn xmlRelaxNGGetErrorString(mut err: xmlRelaxNGValidErr,
                                              mut arg1: *const xmlChar,
                                              mut arg2: *const xmlChar)
 -> *mut xmlChar {
    let mut msg: [std::os::raw::c_char; 1000] = [0; 1000];
    let mut result: *mut xmlChar = 0 as *mut xmlChar;
    if arg1.is_null() {
        arg1 = b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
    }
    if arg2.is_null() {
        arg2 = b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
    }
    msg[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    match err as std::os::raw::c_uint {
        0 => { return 0 as *mut xmlChar }
        1 => {
            return xmlCharStrdup(b"out of memory\n\x00" as *const u8 as
                                     *const std::os::raw::c_char)
        }
        2 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"failed to validate type %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        3 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Type %s doesn\'t allow value \'%s\'\n\x00" as *const u8
                         as *const std::os::raw::c_char, arg1, arg2);
        }
        4 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"ID %s redefined\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        5 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"failed to compare type %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        6 => {
            return xmlCharStrdup(b"Internal error: no state\n\x00" as
                                     *const u8 as *const std::os::raw::c_char)
        }
        7 => {
            return xmlCharStrdup(b"Internal error: no define\n\x00" as
                                     *const u8 as *const std::os::raw::c_char)
        }
        37 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Internal error: %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        8 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Extra data in list: %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        10 => {
            return xmlCharStrdup(b"Internal: interleave block has no data\n\x00"
                                     as *const u8 as *const std::os::raw::c_char)
        }
        11 => {
            return xmlCharStrdup(b"Invalid sequence in interleave\n\x00" as
                                     *const u8 as *const std::os::raw::c_char)
        }
        12 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Extra element %s in interleave\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        13 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Expecting element %s, got %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1, arg2);
        }
        15 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Expecting a namespace for element %s\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        17 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Element %s has wrong namespace: expecting %s\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1, arg2);
        }
        38 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Did not expect element %s there\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        39 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Did not expect text in element %s content\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        19 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Expecting no namespace for element %s\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        21 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Expecting element %s to be empty\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        22 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Expecting an element %s, got nothing\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        23 => {
            return xmlCharStrdup(b"Expecting an element got text\n\x00" as
                                     *const u8 as *const std::os::raw::c_char)
        }
        24 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Element %s failed to validate attributes\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        25 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Element %s failed to validate content\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        26 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Element %s has extra content: %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1, arg2);
        }
        27 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Invalid attribute %s for element %s\n\x00" as *const u8
                         as *const std::os::raw::c_char, arg1, arg2);
        }
        36 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Datatype element %s contains no data\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        28 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Datatype element %s has child elements\n\x00" as
                         *const u8 as *const std::os::raw::c_char, arg1);
        }
        29 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Value element %s has child elements\n\x00" as *const u8
                         as *const std::os::raw::c_char, arg1);
        }
        30 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"List element %s has child elements\n\x00" as *const u8
                         as *const std::os::raw::c_char, arg1);
        }
        31 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Error validating datatype %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        32 => {
            snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"Error validating value %s\n\x00" as *const u8 as
                         *const std::os::raw::c_char, arg1);
        }
        33 => {
            return xmlCharStrdup(b"Error validating list\n\x00" as *const u8
                                     as *const std::os::raw::c_char)
        }
        34 => {
            return xmlCharStrdup(b"No top grammar defined\n\x00" as *const u8
                                     as *const std::os::raw::c_char)
        }
        35 => {
            return xmlCharStrdup(b"Extra data in the document\n\x00" as
                                     *const u8 as *const std::os::raw::c_char)
        }
        _ => {
            return xmlCharStrdup(b"Unknown error !\n\x00" as *const u8 as
                                     *const std::os::raw::c_char)
        }
    }
    if msg[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int {
        snprintf(msg.as_mut_ptr(), 1000 as std::os::raw::c_int as std::os::raw::c_ulong,
                 b"Unknown error code %d\n\x00" as *const u8 as
                     *const std::os::raw::c_char, err as std::os::raw::c_uint);
    }
    msg[(1000 as std::os::raw::c_int - 1 as std::os::raw::c_int) as usize] =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    result = xmlCharStrdup(msg.as_mut_ptr());
    return xmlEscapeFormatString(&mut result);
}
/* *
 * xmlRelaxNGShowValidError:
 * @ctxt:  the validation context
 * @err:  the error number
 * @node:  the node
 * @child:  the node child generating the problem.
 * @arg1:  the first argument
 * @arg2:  the second argument
 *
 * Show a validation error.
 */
unsafe extern "C" fn xmlRelaxNGShowValidError(mut ctxt:
                                                  xmlRelaxNGValidCtxtPtr,
                                              mut err: xmlRelaxNGValidErr,
                                              mut node: xmlNodePtr,
                                              mut child: xmlNodePtr,
                                              mut arg1: *const xmlChar,
                                              mut arg2: *const xmlChar) {
    let mut msg: *mut xmlChar = 0 as *mut xmlChar;
    if (*ctxt).flags & 8 as std::os::raw::c_int != 0 { return }
    msg = xmlRelaxNGGetErrorString(err, arg1, arg2);
    if msg.is_null() { return }
    if (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
        (*ctxt).errNo = err as std::os::raw::c_int
    }
    xmlRngVErr(ctxt, if child.is_null() { node } else { child },
               err as std::os::raw::c_int, msg as *const std::os::raw::c_char, arg1, arg2);
    xmlFree.expect("non-null function pointer")(msg as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGPopErrors:
 * @ctxt:  the validation context
 * @level:  the error level in the stack
 *
 * pop and discard all errors until the given level is reached
 */
unsafe extern "C" fn xmlRelaxNGPopErrors(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                         mut level: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut err: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    i = level;
    while i < (*ctxt).errNr {
        err =
            &mut *(*ctxt).errTab.offset(i as isize) as
                *mut xmlRelaxNGValidError;
        if (*err).flags & 1 as std::os::raw::c_int != 0 {
            if !(*err).arg1.is_null() {
                xmlFree.expect("non-null function pointer")((*err).arg1 as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            (*err).arg1 = 0 as *const xmlChar;
            if !(*err).arg2.is_null() {
                xmlFree.expect("non-null function pointer")((*err).arg2 as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            (*err).arg2 = 0 as *const xmlChar;
            (*err).flags = 0 as std::os::raw::c_int
        }
        i += 1
    }
    (*ctxt).errNr = level;
    if (*ctxt).errNr <= 0 as std::os::raw::c_int {
        (*ctxt).err = 0 as xmlRelaxNGValidErrorPtr
    };
}
/* *
 * xmlRelaxNGDumpValidError:
 * @ctxt:  the validation context
 *
 * Show all validation error over a given index.
 */
unsafe extern "C" fn xmlRelaxNGDumpValidError(mut ctxt:
                                                  xmlRelaxNGValidCtxtPtr) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut err: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    let mut dup: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    i = 0 as std::os::raw::c_int;
    k = 0 as std::os::raw::c_int;
    while i < (*ctxt).errNr {
        let mut current_block_14: u64;
        err =
            &mut *(*ctxt).errTab.offset(i as isize) as
                *mut xmlRelaxNGValidError;
        if k < 5 as std::os::raw::c_int {
            j = 0 as std::os::raw::c_int;
            loop  {
                if !(j < i) {
                    current_block_14 = 11812396948646013369;
                    break ;
                }
                dup =
                    &mut *(*ctxt).errTab.offset(j as isize) as
                        *mut xmlRelaxNGValidError;
                if (*err).err as std::os::raw::c_uint == (*dup).err as std::os::raw::c_uint &&
                       (*err).node == (*dup).node &&
                       xmlStrEqual((*err).arg1, (*dup).arg1) != 0 &&
                       xmlStrEqual((*err).arg2, (*dup).arg2) != 0 {
                    current_block_14 = 2025441756593285306;
                    break ;
                }
                j += 1
            }
            match current_block_14 {
                2025441756593285306 => { }
                _ => {
                    xmlRelaxNGShowValidError(ctxt, (*err).err, (*err).node,
                                             (*err).seq, (*err).arg1,
                                             (*err).arg2);
                    k += 1
                }
            }
        }
        if (*err).flags & 1 as std::os::raw::c_int != 0 {
            if !(*err).arg1.is_null() {
                xmlFree.expect("non-null function pointer")((*err).arg1 as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            (*err).arg1 = 0 as *const xmlChar;
            if !(*err).arg2.is_null() {
                xmlFree.expect("non-null function pointer")((*err).arg2 as
                                                                *mut xmlChar
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            (*err).arg2 = 0 as *const xmlChar;
            (*err).flags = 0 as std::os::raw::c_int
        }
        i += 1
    }
    (*ctxt).errNr = 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGAddValidError:
 * @ctxt:  the validation context
 * @err:  the error number
 * @arg1:  the first argument
 * @arg2:  the second argument
 * @dup:  need to dup the args
 *
 * Register a validation error, either generating it if it's sure
 * or stacking it for later handling if unsure.
 */
unsafe extern "C" fn xmlRelaxNGAddValidError(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                             mut err: xmlRelaxNGValidErr,
                                             mut arg1: *const xmlChar,
                                             mut arg2: *const xmlChar,
                                             mut dup: std::os::raw::c_int) {
    if ctxt.is_null() { return }
    if (*ctxt).flags & 8 as std::os::raw::c_int != 0 { return }
    /*
     * generate the error directly
     */
    if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int ||
           (*ctxt).flags & 2 as std::os::raw::c_int != 0 {
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        let mut seq: xmlNodePtr = 0 as *mut xmlNode;
        /*
         * Flush first any stacked error which might be the
         * real cause of the problem.
         */
        if (*ctxt).errNr != 0 as std::os::raw::c_int {
            xmlRelaxNGDumpValidError(ctxt);
        }
        if !(*ctxt).state.is_null() {
            node = (*(*ctxt).state).node;
            seq = (*(*ctxt).state).seq
        } else { seq = 0 as xmlNodePtr; node = seq }
        if node.is_null() && seq.is_null() { node = (*ctxt).pnode }
        xmlRelaxNGShowValidError(ctxt, err, node, seq, arg1, arg2);
    } else {
        /*
     * Stack the error for later processing if needed
     */
        xmlRelaxNGValidErrorPush(ctxt, err, arg1, arg2, dup);
    };
}
/* *
 * xmlRelaxNGSchemaTypeHave:
 * @data:  data needed for the library
 * @type:  the type name
 *
 * Check if the given type is provided by
 * the W3C XMLSchema Datatype library.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGSchemaTypeHave(mut data: *mut std::os::raw::c_void,
                                              mut type_0: *const xmlChar)
 -> std::os::raw::c_int {
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    if type_0.is_null() { return -(1 as std::os::raw::c_int) }
    typ =
        xmlSchemaGetPredefinedType(type_0,
                                   b"http://www.w3.org/2001/XMLSchema\x00" as
                                       *const u8 as *const std::os::raw::c_char as
                                       *mut xmlChar);
    if typ.is_null() { return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGSchemaTypeCheck:
 * @data:  data needed for the library
 * @type:  the type name
 * @value:  the value to check
 * @node:  the node
 *
 * Check if the given type and value are validated by
 * the W3C XMLSchema Datatype library.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGSchemaTypeCheck(mut data: *mut std::os::raw::c_void,
                                               mut type_0: *const xmlChar,
                                               mut value: *const xmlChar,
                                               mut result:
                                                   *mut *mut std::os::raw::c_void,
                                               mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    let mut ret: std::os::raw::c_int = 0;
    if type_0.is_null() || value.is_null() { return -(1 as std::os::raw::c_int) }
    typ =
        xmlSchemaGetPredefinedType(type_0,
                                   b"http://www.w3.org/2001/XMLSchema\x00" as
                                       *const u8 as *const std::os::raw::c_char as
                                       *mut xmlChar);
    if typ.is_null() { return -(1 as std::os::raw::c_int) }
    ret =
        xmlSchemaValPredefTypeNode(typ, value, result as *mut xmlSchemaValPtr,
                                   node);
    if ret == 2 as std::os::raw::c_int {
        /* special ID error code */
        return 2 as std::os::raw::c_int
    }
    if ret == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if ret > 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlRelaxNGSchemaFacetCheck:
 * @data:  data needed for the library
 * @type:  the type name
 * @facet:  the facet name
 * @val:  the facet value
 * @strval:  the string value
 * @value:  the value to check
 *
 * Function provided by a type library to check a value facet
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGSchemaFacetCheck(mut data: *mut std::os::raw::c_void,
                                                mut type_0: *const xmlChar,
                                                mut facetname: *const xmlChar,
                                                mut val: *const xmlChar,
                                                mut strval: *const xmlChar,
                                                mut value: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut facet: xmlSchemaFacetPtr = 0 as *mut xmlSchemaFacet;
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    let mut ret: std::os::raw::c_int = 0;
    if type_0.is_null() || strval.is_null() { return -(1 as std::os::raw::c_int) }
    typ =
        xmlSchemaGetPredefinedType(type_0,
                                   b"http://www.w3.org/2001/XMLSchema\x00" as
                                       *const u8 as *const std::os::raw::c_char as
                                       *mut xmlChar);
    if typ.is_null() { return -(1 as std::os::raw::c_int) }
    facet = xmlSchemaNewFacet();
    if facet.is_null() { return -(1 as std::os::raw::c_int) }
    if xmlStrEqual(facetname,
                   b"minInclusive\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_MININCLUSIVE
    } else if xmlStrEqual(facetname,
                          b"minExclusive\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_MINEXCLUSIVE
    } else if xmlStrEqual(facetname,
                          b"maxInclusive\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_MAXINCLUSIVE
    } else if xmlStrEqual(facetname,
                          b"maxExclusive\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_MAXEXCLUSIVE
    } else if xmlStrEqual(facetname,
                          b"totalDigits\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_TOTALDIGITS
    } else if xmlStrEqual(facetname,
                          b"fractionDigits\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_FRACTIONDIGITS
    } else if xmlStrEqual(facetname,
                          b"pattern\x00" as *const u8 as *const std::os::raw::c_char
                              as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_PATTERN
    } else if xmlStrEqual(facetname,
                          b"enumeration\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_ENUMERATION
    } else if xmlStrEqual(facetname,
                          b"whiteSpace\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_WHITESPACE
    } else if xmlStrEqual(facetname,
                          b"length\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_LENGTH
    } else if xmlStrEqual(facetname,
                          b"maxLength\x00" as *const u8 as *const std::os::raw::c_char
                              as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_MAXLENGTH
    } else if xmlStrEqual(facetname,
                          b"minLength\x00" as *const u8 as *const std::os::raw::c_char
                              as *mut xmlChar) != 0 {
        (*facet).type_0 = XML_SCHEMA_FACET_MINLENGTH
    } else { xmlSchemaFreeFacet(facet); return -(1 as std::os::raw::c_int) }
    (*facet).value = val;
    ret =
        xmlSchemaCheckFacet(facet, typ, 0 as xmlSchemaParserCtxtPtr, type_0);
    if ret != 0 as std::os::raw::c_int {
        xmlSchemaFreeFacet(facet);
        return -(1 as std::os::raw::c_int)
    }
    ret =
        xmlSchemaValidateFacet(typ, facet, strval, value as xmlSchemaValPtr);
    xmlSchemaFreeFacet(facet);
    if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGSchemaFreeValue:
 * @data:  data needed for the library
 * @value:  the value to free
 *
 * Function provided by a type library to free a Schemas value
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGSchemaFreeValue(mut data: *mut std::os::raw::c_void,
                                               mut value: *mut std::os::raw::c_void) {
    xmlSchemaFreeValue(value as xmlSchemaValPtr);
}
/* *
 * xmlRelaxNGSchemaTypeCompare:
 * @data:  data needed for the library
 * @type:  the type name
 * @value1:  the first value
 * @value2:  the second value
 *
 * Compare two values for equality accordingly a type from the W3C XMLSchema
 * Datatype library.
 *
 * Returns 1 if equal, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGSchemaTypeCompare(mut data: *mut std::os::raw::c_void,
                                                 mut type_0: *const xmlChar,
                                                 mut value1: *const xmlChar,
                                                 mut ctxt1: xmlNodePtr,
                                                 mut comp1: *mut std::os::raw::c_void,
                                                 mut value2: *const xmlChar,
                                                 mut ctxt2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    let mut res1: xmlSchemaValPtr = 0 as xmlSchemaValPtr;
    let mut res2: xmlSchemaValPtr = 0 as xmlSchemaValPtr;
    if type_0.is_null() || value1.is_null() || value2.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    typ =
        xmlSchemaGetPredefinedType(type_0,
                                   b"http://www.w3.org/2001/XMLSchema\x00" as
                                       *const u8 as *const std::os::raw::c_char as
                                       *mut xmlChar);
    if typ.is_null() { return -(1 as std::os::raw::c_int) }
    if comp1.is_null() {
        ret = xmlSchemaValPredefTypeNode(typ, value1, &mut res1, ctxt1);
        if ret != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        if res1.is_null() { return -(1 as std::os::raw::c_int) }
    } else { res1 = comp1 as xmlSchemaValPtr }
    ret = xmlSchemaValPredefTypeNode(typ, value2, &mut res2, ctxt2);
    if ret != 0 as std::os::raw::c_int {
        if res1 != comp1 as xmlSchemaValPtr { xmlSchemaFreeValue(res1); }
        return -(1 as std::os::raw::c_int)
    }
    ret = xmlSchemaCompareValues(res1, res2);
    if res1 != comp1 as xmlSchemaValPtr { xmlSchemaFreeValue(res1); }
    xmlSchemaFreeValue(res2);
    if ret == -(2 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
    if ret == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGDefaultTypeHave:
 * @data:  data needed for the library
 * @type:  the type name
 *
 * Check if the given type is provided by
 * the default datatype library.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGDefaultTypeHave(mut data: *mut std::os::raw::c_void,
                                               mut type_0: *const xmlChar)
 -> std::os::raw::c_int {
    if type_0.is_null() { return -(1 as std::os::raw::c_int) }
    if xmlStrEqual(type_0,
                   b"string\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        return 1 as std::os::raw::c_int
    }
    if xmlStrEqual(type_0,
                   b"token\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGDefaultTypeCheck:
 * @data:  data needed for the library
 * @type:  the type name
 * @value:  the value to check
 * @node:  the node
 *
 * Check if the given type and value are validated by
 * the default datatype library.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGDefaultTypeCheck(mut data: *mut std::os::raw::c_void,
                                                mut type_0: *const xmlChar,
                                                mut value: *const xmlChar,
                                                mut result:
                                                    *mut *mut std::os::raw::c_void,
                                                mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    if xmlStrEqual(type_0,
                   b"string\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        return 1 as std::os::raw::c_int
    }
    if xmlStrEqual(type_0,
                   b"token\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGDefaultTypeCompare:
 * @data:  data needed for the library
 * @type:  the type name
 * @value1:  the first value
 * @value2:  the second value
 *
 * Compare two values accordingly a type from the default
 * datatype library.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGDefaultTypeCompare(mut data: *mut std::os::raw::c_void,
                                                  mut type_0: *const xmlChar,
                                                  mut value1: *const xmlChar,
                                                  mut ctxt1: xmlNodePtr,
                                                  mut comp1:
                                                      *mut std::os::raw::c_void,
                                                  mut value2: *const xmlChar,
                                                  mut ctxt2: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if xmlStrEqual(type_0,
                   b"string\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        ret = xmlStrEqual(value1, value2)
    } else if xmlStrEqual(type_0,
                          b"token\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar) != 0 {
        if xmlStrEqual(value1, value2) == 0 {
            let mut nval: *mut xmlChar = 0 as *mut xmlChar;
            let mut nvalue: *mut xmlChar = 0 as *mut xmlChar;
            /*
             * TODO: trivial optimizations are possible by
             * computing at compile-time
             */
            nval = xmlRelaxNGNormalize(0 as xmlRelaxNGValidCtxtPtr, value1);
            nvalue = xmlRelaxNGNormalize(0 as xmlRelaxNGValidCtxtPtr, value2);
            if nval.is_null() || nvalue.is_null() {
                ret = -(1 as std::os::raw::c_int)
            } else if xmlStrEqual(nval, nvalue) != 0 {
                ret = 1 as std::os::raw::c_int
            } else { ret = 0 as std::os::raw::c_int }
            if !nval.is_null() {
                xmlFree.expect("non-null function pointer")(nval as
                                                                *mut std::os::raw::c_void);
            }
            if !nvalue.is_null() {
                xmlFree.expect("non-null function pointer")(nvalue as
                                                                *mut std::os::raw::c_void);
            }
        } else { ret = 1 as std::os::raw::c_int }
    }
    return ret;
}
static mut xmlRelaxNGTypeInitialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlRelaxNGRegisteredTypes: xmlHashTablePtr =
    0 as *const xmlHashTable as xmlHashTablePtr;
/* *
 * xmlRelaxNGFreeTypeLibrary:
 * @lib:  the type library structure
 * @namespace:  the URI bound to the library
 *
 * Free the structure associated to the type library
 */
unsafe extern "C" fn xmlRelaxNGFreeTypeLibrary(mut payload: *mut std::os::raw::c_void,
                                               mut namespace:
                                                   *const xmlChar) {
    let mut lib: xmlRelaxNGTypeLibraryPtr =
        payload as xmlRelaxNGTypeLibraryPtr;
    if lib.is_null() { return }
    if !(*lib).namespace.is_null() {
        xmlFree.expect("non-null function pointer")((*lib).namespace as
                                                        *mut xmlChar as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(lib as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGRegisterTypeLibrary:
 * @namespace:  the URI bound to the library
 * @data:  data associated to the library
 * @have:  the provide function
 * @check:  the checking function
 * @comp:  the comparison function
 *
 * Register a new type library
 *
 * Returns 0 in case of success and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGRegisterTypeLibrary(mut namespace:
                                                       *const xmlChar,
                                                   mut data:
                                                       *mut std::os::raw::c_void,
                                                   mut have:
                                                       xmlRelaxNGTypeHave,
                                                   mut check:
                                                       xmlRelaxNGTypeCheck,
                                                   mut comp:
                                                       xmlRelaxNGTypeCompare,
                                                   mut facet:
                                                       xmlRelaxNGFacetCheck,
                                                   mut freef:
                                                       xmlRelaxNGTypeFree)
 -> std::os::raw::c_int {
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
    let mut ret: std::os::raw::c_int = 0;
    if xmlRelaxNGRegisteredTypes.is_null() || namespace.is_null() ||
           check.is_none() || comp.is_none() {
        return -(1 as std::os::raw::c_int)
    }
    if !xmlHashLookup(xmlRelaxNGRegisteredTypes, namespace).is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Relax-NG types library \'%s\' already registered\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   namespace);
        return -(1 as std::os::raw::c_int)
    }
    lib =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGTypeLibrary>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGTypeLibraryPtr;
    if lib.is_null() {
        xmlRngVErrMemory(0 as xmlRelaxNGValidCtxtPtr,
                         b"adding types library\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    memset(lib as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGTypeLibrary>() as std::os::raw::c_ulong);
    (*lib).namespace = xmlStrdup(namespace);
    (*lib).data = data;
    (*lib).have = have;
    (*lib).comp = comp;
    (*lib).check = check;
    (*lib).facet = facet;
    (*lib).freef = freef;
    ret =
        xmlHashAddEntry(xmlRelaxNGRegisteredTypes, namespace,
                        lib as *mut std::os::raw::c_void);
    if ret < 0 as std::os::raw::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Relax-NG types library failed to register \'%s\'\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   namespace);
        xmlRelaxNGFreeTypeLibrary(lib as *mut std::os::raw::c_void, namespace);
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGInitTypes:
 *
 * Initilize the default type libraries.
 *
 * Returns 0 in case of success and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGInitTypes() -> std::os::raw::c_int {
    if xmlRelaxNGTypeInitialized != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    xmlRelaxNGRegisteredTypes = xmlHashCreate(10 as std::os::raw::c_int);
    if xmlRelaxNGRegisteredTypes.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Failed to allocate sh table for Relax-NG types\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    xmlRelaxNGRegisterTypeLibrary(b"http://www.w3.org/2001/XMLSchema-datatypes\x00"
                                      as *const u8 as *const std::os::raw::c_char as
                                      *mut xmlChar, 0 as *mut std::os::raw::c_void,
                                  Some(xmlRelaxNGSchemaTypeHave as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar)
                                               -> std::os::raw::c_int),
                                  Some(xmlRelaxNGSchemaTypeCheck as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *mut *mut std::os::raw::c_void,
                                                                _: xmlNodePtr)
                                               -> std::os::raw::c_int),
                                  Some(xmlRelaxNGSchemaTypeCompare as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _: xmlNodePtr,
                                                                _:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _: xmlNodePtr)
                                               -> std::os::raw::c_int),
                                  Some(xmlRelaxNGSchemaFacetCheck as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *mut std::os::raw::c_void)
                                               -> std::os::raw::c_int),
                                  Some(xmlRelaxNGSchemaFreeValue as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *mut std::os::raw::c_void)
                                               -> ()));
    xmlRelaxNGRegisterTypeLibrary(xmlRelaxNGNs, 0 as *mut std::os::raw::c_void,
                                  Some(xmlRelaxNGDefaultTypeHave as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar)
                                               -> std::os::raw::c_int),
                                  Some(xmlRelaxNGDefaultTypeCheck as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *mut *mut std::os::raw::c_void,
                                                                _: xmlNodePtr)
                                               -> std::os::raw::c_int),
                                  Some(xmlRelaxNGDefaultTypeCompare as
                                           unsafe extern "C" fn(_:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _:
                                                                    *const xmlChar,
                                                                _: xmlNodePtr,
                                                                _:
                                                                    *mut std::os::raw::c_void,
                                                                _:
                                                                    *const xmlChar,
                                                                _: xmlNodePtr)
                                               -> std::os::raw::c_int), None, None);
    xmlRelaxNGTypeInitialized = 1 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGCleanupTypes:
 *
 * Cleanup the default Schemas type library associated to RelaxNG
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGCleanupTypes() {
    xmlSchemaCleanupTypes();
    if xmlRelaxNGTypeInitialized == 0 as std::os::raw::c_int { return }
    xmlHashFree(xmlRelaxNGRegisteredTypes,
                Some(xmlRelaxNGFreeTypeLibrary as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
    xmlRelaxNGTypeInitialized = 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGIsCompileable:
 * @define:  the definition to check
 *
 * Check if a definition is nullable.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGIsCompileable(mut def: xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if def.is_null() { return -(1 as std::os::raw::c_int) }
    if (*def).type_0 as std::os::raw::c_int != XML_RELAXNG_ELEMENT as std::os::raw::c_int &&
           (*def).dflags as std::os::raw::c_int &
               (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int != 0 {
        return 1 as std::os::raw::c_int
    }
    if (*def).type_0 as std::os::raw::c_int != XML_RELAXNG_ELEMENT as std::os::raw::c_int &&
           (*def).dflags as std::os::raw::c_int &
               (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int
    }
    match (*def).type_0 as std::os::raw::c_int {
        -1 => { ret = xmlRelaxNGIsCompileable((*def).content) }
        3 | 0 => { ret = 1 as std::os::raw::c_int }
        4 => {
            /*
             * Check if the element content is compileable
             */
            if (*def).dflags as std::os::raw::c_int &
                   (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int == 0 as std::os::raw::c_int
                   &&
                   (*def).dflags as std::os::raw::c_int &
                       (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                let mut list: xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefine;
                list = (*def).content;
                while !list.is_null() {
                    ret = xmlRelaxNGIsCompileable(list);
                    if ret != 1 as std::os::raw::c_int { break ; }
                    list = (*list).next
                }
                /*
		 * Because the routine is recursive, we must guard against
		 * discovering both COMPILABLE and NOT_COMPILABLE
		 */
                if ret == 0 as std::os::raw::c_int {
                    (*def).dflags =
                        ((*def).dflags as std::os::raw::c_int &
                             !((1 as std::os::raw::c_int) << 6 as std::os::raw::c_int)) as
                            std::os::raw::c_short;
                    (*def).dflags =
                        ((*def).dflags as std::os::raw::c_int |
                             (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int) as
                            std::os::raw::c_short
                }
                if ret == 1 as std::os::raw::c_int &&
                       {
                           (*def).dflags =
                               ((*def).dflags as std::os::raw::c_int &
                                    (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int) as
                                   std::os::raw::c_short;
                           ((*def).dflags) == 0
                       } {
                    (*def).dflags =
                        ((*def).dflags as std::os::raw::c_int |
                             (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int) as
                            std::os::raw::c_short
                }
            }
            /*
             * All elements return a compileable status unless they
             * are generic like anyName
             */
            if !(*def).nameClass.is_null() || (*def).name.is_null() {
                ret = 0 as std::os::raw::c_int
            } else { ret = 1 as std::os::raw::c_int }
            return ret
        }
        11 | 12 | 13 => {
            if (*def).depth as std::os::raw::c_int == -(20 as std::os::raw::c_int) {
                return 1 as std::os::raw::c_int
            } else {
                let mut list_0: xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefine;
                (*def).depth = -(20 as std::os::raw::c_int) as std::os::raw::c_short;
                list_0 = (*def).content;
                while !list_0.is_null() {
                    ret = xmlRelaxNGIsCompileable(list_0);
                    if ret != 1 as std::os::raw::c_int { break ; }
                    list_0 = (*list_0).next
                }
            }
        }
        20 | 14 | 15 | 16 | 17 | 18 | 10 => {
            let mut list_1: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            list_1 = (*def).content;
            while !list_1.is_null() {
                ret = xmlRelaxNGIsCompileable(list_1);
                if ret != 1 as std::os::raw::c_int { break ; }
                list_1 = (*list_1).next
            }
        }
        2 | 9 | 19 | 5 | 8 | 6 | 7 | 1 => { ret = 0 as std::os::raw::c_int }
        _ => { }
    }
    if ret == 0 as std::os::raw::c_int {
        (*def).dflags =
            ((*def).dflags as std::os::raw::c_int |
                 (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int) as std::os::raw::c_short
    }
    if ret == 1 as std::os::raw::c_int {
        (*def).dflags =
            ((*def).dflags as std::os::raw::c_int |
                 (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int) as std::os::raw::c_short
    }
    return ret;
}
/* *
 * xmlRelaxNGCompile:
 * ctxt:  the RelaxNG parser context
 * @define:  the definition tree to compile
 *
 * Compile the set of definitions, it works recursively, till the
 * element boundaries, where it tries to compile the content if possible
 *
 * Returns 0 if success and -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGCompile(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                       mut def: xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if ctxt.is_null() || def.is_null() { return -(1 as std::os::raw::c_int) }
    match (*def).type_0 as std::os::raw::c_int {
        20 => {
            if xmlRelaxNGIsCompileable(def) == 1 as std::os::raw::c_int &&
                   (*def).depth as std::os::raw::c_int != -(25 as std::os::raw::c_int) {
                let mut oldam: xmlAutomataPtr = (*ctxt).am;
                let mut oldstate: xmlAutomataStatePtr = (*ctxt).state;
                (*def).depth = -(25 as std::os::raw::c_int) as std::os::raw::c_short;
                list = (*def).content;
                (*ctxt).am = xmlNewAutomata();
                if (*ctxt).am.is_null() { return -(1 as std::os::raw::c_int) }
                /*
                 * assume identical strings but not same pointer are different
                 * atoms, needed for non-determinism detection
                 * That way if 2 elements with the same name are in a choice
                 * branch the automata is found non-deterministic and
                 * we fallback to the normal validation which does the right
                 * thing of exploring both choices.
                 */
                xmlAutomataSetFlags((*ctxt).am, 1 as std::os::raw::c_int);
                (*ctxt).state = xmlAutomataGetInitState((*ctxt).am);
                while !list.is_null() {
                    xmlRelaxNGCompile(ctxt, list);
                    list = (*list).next
                }
                xmlAutomataSetFinalState((*ctxt).am, (*ctxt).state);
                if xmlAutomataIsDeterminist((*ctxt).am) != 0 {
                    (*def).contModel = xmlAutomataCompile((*ctxt).am)
                }
                xmlFreeAutomata((*ctxt).am);
                (*ctxt).state = oldstate;
                (*ctxt).am = oldam
            }
        }
        4 => {
            if !(*ctxt).am.is_null() && !(*def).name.is_null() {
                (*ctxt).state =
                    xmlAutomataNewTransition2((*ctxt).am, (*ctxt).state,
                                              0 as xmlAutomataStatePtr,
                                              (*def).name, (*def).ns,
                                              def as *mut std::os::raw::c_void)
            }
            if (*def).dflags as std::os::raw::c_int &
                   (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int != 0 &&
                   (*def).depth as std::os::raw::c_int != -(25 as std::os::raw::c_int) {
                let mut oldam_0: xmlAutomataPtr = (*ctxt).am;
                let mut oldstate_0: xmlAutomataStatePtr = (*ctxt).state;
                (*def).depth = -(25 as std::os::raw::c_int) as std::os::raw::c_short;
                list = (*def).content;
                (*ctxt).am = xmlNewAutomata();
                if (*ctxt).am.is_null() { return -(1 as std::os::raw::c_int) }
                xmlAutomataSetFlags((*ctxt).am, 1 as std::os::raw::c_int);
                (*ctxt).state = xmlAutomataGetInitState((*ctxt).am);
                while !list.is_null() {
                    xmlRelaxNGCompile(ctxt, list);
                    list = (*list).next
                }
                xmlAutomataSetFinalState((*ctxt).am, (*ctxt).state);
                (*def).contModel = xmlAutomataCompile((*ctxt).am);
                if xmlRegexpIsDeterminist((*def).contModel) == 0 {
                    /*
                     * we can only use the automata if it is determinist
                     */
                    xmlRegFreeRegexp((*def).contModel);
                    (*def).contModel = 0 as xmlRegexpPtr
                }
                xmlFreeAutomata((*ctxt).am);
                (*ctxt).state = oldstate_0;
                (*ctxt).am = oldam_0
            } else {
                let mut oldam_1: xmlAutomataPtr = (*ctxt).am;
                /*
                 * we can't build the content model for this element content
                 * but it still might be possible to build it for some of its
                 * children, recurse.
                 */
                ret = xmlRelaxNGTryCompile(ctxt, def);
                (*ctxt).am = oldam_1
            }
        }
        -1 => { ret = xmlRelaxNGCompile(ctxt, (*def).content) }
        14 => {
            let mut oldstate_1: xmlAutomataStatePtr = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next
            }
            xmlAutomataNewEpsilon((*ctxt).am, oldstate_1, (*ctxt).state);
        }
        15 => {
            let mut oldstate_2: xmlAutomataStatePtr =
                0 as *mut xmlAutomataState;
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state,
                                      0 as xmlAutomataStatePtr);
            oldstate_2 = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next
            }
            xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldstate_2);
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, oldstate_2,
                                      0 as xmlAutomataStatePtr)
        }
        16 => {
            let mut oldstate_3: xmlAutomataStatePtr =
                0 as *mut xmlAutomataState;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next
            }
            oldstate_3 = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next
            }
            xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldstate_3);
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, oldstate_3,
                                      0 as xmlAutomataStatePtr)
        }
        17 => {
            let mut target: xmlAutomataStatePtr = 0 as xmlAutomataStatePtr;
            let mut oldstate_4: xmlAutomataStatePtr = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                (*ctxt).state = oldstate_4;
                ret = xmlRelaxNGCompile(ctxt, list);
                if ret != 0 as std::os::raw::c_int { break ; }
                if target.is_null() {
                    target = (*ctxt).state
                } else {
                    xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, target);
                }
                list = (*list).next
            }
            (*ctxt).state = target
        }
        11 | 12 | 13 | 18 | 10 => {
            list = (*def).content;
            while !list.is_null() {
                ret = xmlRelaxNGCompile(ctxt, list);
                if ret != 0 as std::os::raw::c_int { break ; }
                list = (*list).next
            }
        }
        3 => {
            let mut oldstate_5: xmlAutomataStatePtr =
                0 as *mut xmlAutomataState;
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state,
                                      0 as xmlAutomataStatePtr);
            oldstate_5 = (*ctxt).state;
            xmlRelaxNGCompile(ctxt, (*def).content);
            xmlAutomataNewTransition((*ctxt).am, (*ctxt).state, (*ctxt).state,
                                     b"#text\x00" as *const u8 as
                                         *const std::os::raw::c_char as *mut xmlChar,
                                     0 as *mut std::os::raw::c_void);
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, oldstate_5,
                                      0 as xmlAutomataStatePtr)
        }
        0 => {
            (*ctxt).state =
                xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state,
                                      0 as xmlAutomataStatePtr)
        }
        2 | 9 | 19 | 1 | 5 | 8 | 6 | 7 => {
            /* This should not happen and generate an internal error */
            fprintf(stderr,
                    b"RNG internal error trying to compile %s\n\x00" as
                        *const u8 as *const std::os::raw::c_char,
                    xmlRelaxNGDefName(def));
        }
        _ => { }
    }
    return ret;
}
/* *
 * xmlRelaxNGTryCompile:
 * ctxt:  the RelaxNG parser context
 * @define:  the definition tree to compile
 *
 * Try to compile the set of definitions, it works recursively,
 * possibly ignoring parts which cannot be compiled.
 *
 * Returns 0 if success and -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGTryCompile(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                          mut def: xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if ctxt.is_null() || def.is_null() { return -(1 as std::os::raw::c_int) }
    if (*def).type_0 as std::os::raw::c_int == XML_RELAXNG_START as std::os::raw::c_int ||
           (*def).type_0 as std::os::raw::c_int == XML_RELAXNG_ELEMENT as std::os::raw::c_int
       {
        ret = xmlRelaxNGIsCompileable(def);
        if (*def).dflags as std::os::raw::c_int &
               (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int != 0 &&
               (*def).depth as std::os::raw::c_int != -(25 as std::os::raw::c_int) {
            (*ctxt).am = 0 as xmlAutomataPtr;
            ret = xmlRelaxNGCompile(ctxt, def);
            return ret
        }
    }
    match (*def).type_0 as std::os::raw::c_int {
        -1 => { ret = xmlRelaxNGTryCompile(ctxt, (*def).content) }
        3 | 5 | 8 | 6 | 7 | 0 | 4 => { ret = 0 as std::os::raw::c_int }
        14 | 15 | 16 | 17 | 18 | 10 | 20 | 11 | 12 | 13 => {
            list = (*def).content;
            while !list.is_null() {
                ret = xmlRelaxNGTryCompile(ctxt, list);
                if ret != 0 as std::os::raw::c_int { break ; }
                list = (*list).next
            }
        }
        2 | 9 | 19 | 1 => { ret = 0 as std::os::raw::c_int }
        _ => { }
    }
    return ret;
}
/* *
 * xmlRelaxNGIsNullable:
 * @define:  the definition to verify
 *
 * Check if a definition is nullable.
 *
 * Returns 1 if yes, 0 if no and -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGIsNullable(mut define: xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ret: std::os::raw::c_int = 0;
    if define.is_null() { return -(1 as std::os::raw::c_int) }
    if (*define).dflags as std::os::raw::c_int &
           (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
        return 1 as std::os::raw::c_int
    }
    if (*define).dflags as std::os::raw::c_int &
           (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int
    }
    match (*define).type_0 as std::os::raw::c_int {
        0 | 3 => { ret = 1 as std::os::raw::c_int }
        -1 | 10 | 11 | 12 | 13 | 16 => {
            ret = xmlRelaxNGIsNullable((*define).content)
        }
        2 | 1 | 4 | 5 | 6 | 7 | 8 | 9 => { ret = 0 as std::os::raw::c_int }
        17 => {
            let mut list: xmlRelaxNGDefinePtr = (*define).content;
            loop  {
                if list.is_null() {
                    current_block = 15089075282327824602;
                    break ;
                }
                ret = xmlRelaxNGIsNullable(list);
                if ret != 0 as std::os::raw::c_int {
                    current_block = 13711301349868859165;
                    break ;
                }
                list = (*list).next
            }
            match current_block {
                13711301349868859165 => { }
                _ => { ret = 0 as std::os::raw::c_int }
            }
        }
        20 | 19 | 18 => {
            let mut list_0: xmlRelaxNGDefinePtr = (*define).content;
            loop  {
                if list_0.is_null() {
                    current_block = 14359455889292382949;
                    break ;
                }
                ret = xmlRelaxNGIsNullable(list_0);
                if ret != 1 as std::os::raw::c_int {
                    current_block = 13711301349868859165;
                    break ;
                }
                list_0 = (*list_0).next
            }
            match current_block {
                13711301349868859165 => { }
                _ => { return 1 as std::os::raw::c_int }
            }
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    if ret == 0 as std::os::raw::c_int {
        (*define).dflags =
            ((*define).dflags as std::os::raw::c_int |
                 (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) as std::os::raw::c_short
    }
    if ret == 1 as std::os::raw::c_int {
        (*define).dflags =
            ((*define).dflags as std::os::raw::c_int |
                 (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int) as std::os::raw::c_short
    }
    return ret;
}
/* *
 * xmlRelaxNGIsBlank:
 * @str:  a string
 *
 * Check if a string is ignorable c.f. 4.2. Whitespace
 *
 * Returns 1 if the string is NULL or made of blanks chars, 0 otherwise
 */
unsafe extern "C" fn xmlRelaxNGIsBlank(mut str: *mut xmlChar) -> std::os::raw::c_int {
    if str.is_null() { return 1 as std::os::raw::c_int }
    while *str as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if !(*str as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                 0x9 as std::os::raw::c_int <= *str as std::os::raw::c_int &&
                     *str as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                 *str as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            return 0 as std::os::raw::c_int
        }
        str = str.offset(1)
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGGetDataTypeLibrary:
 * @ctxt:  a Relax-NG parser context
 * @node:  the current data or value element
 *
 * Applies algorithm from 4.3. datatypeLibrary attribute
 *
 * Returns the datatypeLibary value or NULL if not found
 */
unsafe extern "C" fn xmlRelaxNGGetDataTypeLibrary(mut ctxt:
                                                      xmlRelaxNGParserCtxtPtr,
                                                  mut node: xmlNodePtr)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut escape: *mut xmlChar = 0 as *mut xmlChar;
    if node.is_null() { return 0 as *mut xmlChar }
    if !node.is_null() && !(*node).ns.is_null() &&
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*node).name,
                       b"data\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 &&
           xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 ||
           !node.is_null() && !(*node).ns.is_null() &&
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*node).name,
                           b"value\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar) != 0 &&
               xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        ret =
            xmlGetProp(node as *const xmlNode,
                       b"datatypeLibrary\x00" as *const u8 as
                           *const std::os::raw::c_char as *mut xmlChar);
        if !ret.is_null() {
            if *ret.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                xmlFree.expect("non-null function pointer")(ret as
                                                                *mut std::os::raw::c_void);
                return 0 as *mut xmlChar
            }
            escape =
                xmlURIEscapeStr(ret,
                                b":/#?\x00" as *const u8 as
                                    *const std::os::raw::c_char as *mut xmlChar);
            if escape.is_null() { return ret }
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return escape
        }
    }
    node = (*node).parent;
    while !node.is_null() &&
              (*node).type_0 as std::os::raw::c_uint ==
                  XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        ret =
            xmlGetProp(node as *const xmlNode,
                       b"datatypeLibrary\x00" as *const u8 as
                           *const std::os::raw::c_char as *mut xmlChar);
        if !ret.is_null() {
            if *ret.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                xmlFree.expect("non-null function pointer")(ret as
                                                                *mut std::os::raw::c_void);
                return 0 as *mut xmlChar
            }
            escape =
                xmlURIEscapeStr(ret,
                                b":/#?\x00" as *const u8 as
                                    *const std::os::raw::c_char as *mut xmlChar);
            if escape.is_null() { return ret }
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return escape
        }
        node = (*node).parent
    }
    return 0 as *mut xmlChar;
}
/* *
 * xmlRelaxNGParseValue:
 * @ctxt:  a Relax-NG parser context
 * @node:  the data node.
 *
 * parse the content of a RelaxNG value node.
 *
 * Returns the definition pointer or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseValue(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                          mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as xmlRelaxNGTypeLibraryPtr;
    let mut type_0: *mut xmlChar = 0 as *mut xmlChar;
    let mut library: *mut xmlChar = 0 as *mut xmlChar;
    let mut success: std::os::raw::c_int = 0 as std::os::raw::c_int;
    def = xmlRelaxNGNewDefine(ctxt, node);
    if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
    (*def).type_0 = XML_RELAXNG_VALUE;
    type_0 =
        xmlGetProp(node as *const xmlNode,
                   b"type\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    if !type_0.is_null() {
        xmlRelaxNGNormExtSpace(type_0);
        if xmlValidateNCName(type_0, 0 as std::os::raw::c_int) != 0 {
            xmlRngPErr(ctxt, node, XML_RNGP_TYPE_VALUE as std::os::raw::c_int,
                       b"value type \'%s\' is not an NCName\n\x00" as
                           *const u8 as *const std::os::raw::c_char, type_0,
                       0 as *const xmlChar);
        }
        library = xmlRelaxNGGetDataTypeLibrary(ctxt, node);
        if library.is_null() {
            library =
                xmlStrdup(b"http://relaxng.org/ns/structure/1.0\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar)
        }
        (*def).name = type_0;
        (*def).ns = library;
        lib =
            xmlHashLookup(xmlRelaxNGRegisteredTypes, library) as
                xmlRelaxNGTypeLibraryPtr;
        if lib.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_UNKNOWN_TYPE_LIB as std::os::raw::c_int,
                       b"Use of unregistered type library \'%s\'\n\x00" as
                           *const u8 as *const std::os::raw::c_char, library,
                       0 as *const xmlChar);
            (*def).data = 0 as *mut std::os::raw::c_void
        } else {
            (*def).data = lib as *mut std::os::raw::c_void;
            if (*lib).have.is_none() {
                xmlRngPErr(ctxt, node, XML_RNGP_ERROR_TYPE_LIB as std::os::raw::c_int,
                           b"Internal error with type library \'%s\': no \'have\'\n\x00"
                               as *const u8 as *const std::os::raw::c_char, library,
                           0 as *const xmlChar);
            } else {
                success =
                    (*lib).have.expect("non-null function pointer")((*lib).data,
                                                                    (*def).name);
                if success != 1 as std::os::raw::c_int {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_TYPE_NOT_FOUND as std::os::raw::c_int,
                               b"Error type \'%s\' is not exported by type library \'%s\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               (*def).name, library);
                }
            }
        }
    }
    if (*node).children.is_null() {
        (*def).value =
            xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar)
    } else if (*(*node).children).type_0 as std::os::raw::c_uint !=
                  XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  (*(*node).children).type_0 as std::os::raw::c_uint !=
                      XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                  !(*(*node).children).next.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_TEXT_EXPECTED as std::os::raw::c_int,
                   b"Expecting a single text value for <value>content\n\x00"
                       as *const u8 as *const std::os::raw::c_char,
                   0 as *const xmlChar, 0 as *const xmlChar);
    } else if !def.is_null() {
        (*def).value = xmlNodeGetContent(node as *const xmlNode);
        if (*def).value.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_VALUE_NO_CONTENT as std::os::raw::c_int,
                       b"Element <value> has no content\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        } else if !lib.is_null() && (*lib).check.is_some() &&
                      success == 1 as std::os::raw::c_int {
            let mut val: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
            success =
                (*lib).check.expect("non-null function pointer")((*lib).data,
                                                                 (*def).name,
                                                                 (*def).value,
                                                                 &mut val,
                                                                 node);
            if success != 1 as std::os::raw::c_int {
                xmlRngPErr(ctxt, node, XML_RNGP_INVALID_VALUE as std::os::raw::c_int,
                           b"Value \'%s\' is not acceptable for type \'%s\'\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           (*def).value, (*def).name);
            } else if !val.is_null() {
                (*def).attrs = val as xmlRelaxNGDefinePtr
            }
        }
    }
    return def;
}
/* *
 * xmlRelaxNGParseData:
 * @ctxt:  a Relax-NG parser context
 * @node:  the data node.
 *
 * parse the content of a RelaxNG data node.
 *
 * Returns the definition pointer or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseData(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                         mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut except: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut param: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut lastparam: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
    let mut type_0: *mut xmlChar = 0 as *mut xmlChar;
    let mut library: *mut xmlChar = 0 as *mut xmlChar;
    let mut content: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: std::os::raw::c_int = 0;
    type_0 =
        xmlGetProp(node as *const xmlNode,
                   b"type\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    if type_0.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_TYPE_MISSING as std::os::raw::c_int,
                   b"data has no type\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return 0 as xmlRelaxNGDefinePtr
    }
    xmlRelaxNGNormExtSpace(type_0);
    if xmlValidateNCName(type_0, 0 as std::os::raw::c_int) != 0 {
        xmlRngPErr(ctxt, node, XML_RNGP_TYPE_VALUE as std::os::raw::c_int,
                   b"data type \'%s\' is not an NCName\n\x00" as *const u8 as
                       *const std::os::raw::c_char, type_0, 0 as *const xmlChar);
    }
    library = xmlRelaxNGGetDataTypeLibrary(ctxt, node);
    if library.is_null() {
        library =
            xmlStrdup(b"http://relaxng.org/ns/structure/1.0\x00" as *const u8
                          as *const std::os::raw::c_char as *mut xmlChar)
    }
    def = xmlRelaxNGNewDefine(ctxt, node);
    if def.is_null() {
        xmlFree.expect("non-null function pointer")(type_0 as
                                                        *mut std::os::raw::c_void);
        return 0 as xmlRelaxNGDefinePtr
    }
    (*def).type_0 = XML_RELAXNG_DATATYPE;
    (*def).name = type_0;
    (*def).ns = library;
    lib =
        xmlHashLookup(xmlRelaxNGRegisteredTypes, library) as
            xmlRelaxNGTypeLibraryPtr;
    if lib.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_UNKNOWN_TYPE_LIB as std::os::raw::c_int,
                   b"Use of unregistered type library \'%s\'\n\x00" as
                       *const u8 as *const std::os::raw::c_char, library,
                   0 as *const xmlChar);
        (*def).data = 0 as *mut std::os::raw::c_void
    } else {
        (*def).data = lib as *mut std::os::raw::c_void;
        if (*lib).have.is_none() {
            xmlRngPErr(ctxt, node, XML_RNGP_ERROR_TYPE_LIB as std::os::raw::c_int,
                       b"Internal error with type library \'%s\': no \'have\'\n\x00"
                           as *const u8 as *const std::os::raw::c_char, library,
                       0 as *const xmlChar);
        } else {
            tmp =
                (*lib).have.expect("non-null function pointer")((*lib).data,
                                                                (*def).name);
            if tmp != 1 as std::os::raw::c_int {
                xmlRngPErr(ctxt, node, XML_RNGP_TYPE_NOT_FOUND as std::os::raw::c_int,
                           b"Error type \'%s\' is not exported by type library \'%s\'\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           (*def).name, library);
            } else if xmlStrEqual(library,
                                  b"http://www.w3.org/2001/XMLSchema-datatypes\x00"
                                      as *const u8 as *const std::os::raw::c_char as
                                      *mut xmlChar) != 0 &&
                          (xmlStrEqual((*def).name,
                                       b"IDREF\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 ||
                               xmlStrEqual((*def).name,
                                           b"IDREFS\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar) != 0) {
                (*ctxt).idref = 1 as std::os::raw::c_int
            }
        }
    }
    content = (*node).children;
    /*
     * Handle optional params
     */
    while !content.is_null() {
        if xmlStrEqual((*content).name,
                       b"param\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) == 0 {
            break ;
        }
        if xmlStrEqual(library,
                       b"http://relaxng.org/ns/structure/1.0\x00" as *const u8
                           as *const std::os::raw::c_char as *mut xmlChar) != 0 {
            xmlRngPErr(ctxt, node, XML_RNGP_PARAM_FORBIDDEN as std::os::raw::c_int,
                       b"Type library \'%s\' does not allow type parameters\n\x00"
                           as *const u8 as *const std::os::raw::c_char, library,
                       0 as *const xmlChar);
            content = (*content).next;
            while !content.is_null() &&
                      xmlStrEqual((*content).name,
                                  b"param\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                content = (*content).next
            }
        } else {
            param = xmlRelaxNGNewDefine(ctxt, node);
            if !param.is_null() {
                (*param).type_0 = XML_RELAXNG_PARAM;
                (*param).name =
                    xmlGetProp(content as *const xmlNode,
                               b"name\x00" as *const u8 as *const std::os::raw::c_char
                                   as *mut xmlChar);
                if (*param).name.is_null() {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_PARAM_NAME_MISSING as std::os::raw::c_int,
                               b"param has no name\n\x00" as *const u8 as
                                   *const std::os::raw::c_char, 0 as *const xmlChar,
                               0 as *const xmlChar);
                }
                (*param).value = xmlNodeGetContent(content as *const xmlNode);
                if lastparam.is_null() {
                    lastparam = param;
                    (*def).attrs = lastparam
                } else { (*lastparam).next = param; lastparam = param }
                !lib.is_null();
            }
            content = (*content).next
        }
    }
    /*
     * Handle optional except
     */
    if !content.is_null() &&
           xmlStrEqual((*content).name,
                       b"except\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
        let mut child: xmlNodePtr = 0 as *mut xmlNode;
        let mut tmp2: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
        except = xmlRelaxNGNewDefine(ctxt, node);
        if except.is_null() { return def }
        (*except).type_0 = XML_RELAXNG_EXCEPT;
        child = (*content).children;
        (*def).content = except;
        if child.is_null() {
            xmlRngPErr(ctxt, content,
                       XML_RNGP_EXCEPT_NO_CONTENT as std::os::raw::c_int,
                       b"except has no content\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
        while !child.is_null() {
            tmp2 = xmlRelaxNGParsePattern(ctxt, child);
            if !tmp2.is_null() {
                if last.is_null() {
                    last = tmp2;
                    (*except).content = last
                } else { (*last).next = tmp2; last = tmp2 }
            }
            child = (*child).next
        }
        content = (*content).next
    }
    /*
     * Check there is no unhandled data
     */
    if !content.is_null() {
        xmlRngPErr(ctxt, content, XML_RNGP_DATA_CONTENT as std::os::raw::c_int,
                   b"Element data has unexpected content %s\n\x00" as
                       *const u8 as *const std::os::raw::c_char, (*content).name,
                   0 as *const xmlChar);
    }
    return def;
}
static mut invalidName: *const xmlChar =
    b"\x01\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar;
/* *
 * xmlRelaxNGCompareNameClasses:
 * @defs1:  the first element/attribute defs
 * @defs2:  the second element/attribute defs
 * @name:  the restriction on the name
 * @ns:  the restriction on the namespace
 *
 * Compare the 2 lists of element definitions. The comparison is
 * that if both lists do not accept the same QNames, it returns 1
 * If the 2 lists can accept the same QName the comparison returns 0
 *
 * Returns 1 disttinct, 0 if equal
 */
unsafe extern "C" fn xmlRelaxNGCompareNameClasses(mut def1:
                                                      xmlRelaxNGDefinePtr,
                                                  mut def2:
                                                      xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut node: xmlNode =
        xmlNode{_private: 0 as *mut std::os::raw::c_void,
                type_0: 0 as xmlElementType,
                name: 0 as *const xmlChar,
                children: 0 as *mut _xmlNode,
                last: 0 as *mut _xmlNode,
                parent: 0 as *mut _xmlNode,
                next: 0 as *mut _xmlNode,
                prev: 0 as *mut _xmlNode,
                doc: 0 as *mut _xmlDoc,
                ns: 0 as *mut xmlNs,
                content: 0 as *mut xmlChar,
                properties: 0 as *mut _xmlAttr,
                nsDef: 0 as *mut xmlNs,
                psvi: 0 as *mut std::os::raw::c_void,
                line: 0,
                extra: 0,};
    let mut ns: xmlNs =
        xmlNs{next: 0 as *mut _xmlNs,
              type_0: 0 as xmlElementType,
              href: 0 as *const xmlChar,
              prefix: 0 as *const xmlChar,
              _private: 0 as *mut std::os::raw::c_void,
              context: 0 as *mut _xmlDoc,};
    let mut ctxt: xmlRelaxNGValidCtxt =
        xmlRelaxNGValidCtxt{userData: 0 as *mut std::os::raw::c_void,
                            error: None,
                            warning: None,
                            serror: None,
                            nbErrors: 0,
                            schema: 0 as *mut xmlRelaxNG,
                            doc: 0 as *mut xmlDoc,
                            flags: 0,
                            depth: 0,
                            idref: 0,
                            errNo: 0,
                            err: 0 as *mut xmlRelaxNGValidError,
                            errNr: 0,
                            errMax: 0,
                            errTab: 0 as *mut xmlRelaxNGValidError,
                            state: 0 as *mut xmlRelaxNGValidState,
                            states: 0 as *mut xmlRelaxNGStates,
                            freeState: 0 as *mut xmlRelaxNGStates,
                            freeStatesNr: 0,
                            freeStatesMax: 0,
                            freeStates: 0 as *mut xmlRelaxNGStatesPtr,
                            elem: 0 as *mut xmlRegExecCtxt,
                            elemNr: 0,
                            elemMax: 0,
                            elemTab: 0 as *mut xmlRegExecCtxtPtr,
                            pstate: 0,
                            pnode: 0 as *mut xmlNode,
                            pdef: 0 as *mut xmlRelaxNGDefine,
                            perr: 0,};
    memset(&mut ctxt as *mut xmlRelaxNGValidCtxt as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGValidCtxt>() as std::os::raw::c_ulong);
    ctxt.flags = 1 as std::os::raw::c_int | 8 as std::os::raw::c_int;
    if (*def1).type_0 as std::os::raw::c_int == XML_RELAXNG_ELEMENT as std::os::raw::c_int ||
           (*def1).type_0 as std::os::raw::c_int ==
               XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
        if (*def2).type_0 as std::os::raw::c_int == XML_RELAXNG_TEXT as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
        if !(*def1).name.is_null() {
            node.name = (*def1).name
        } else { node.name = invalidName }
        if !(*def1).ns.is_null() {
            if *(*def1).ns.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                node.ns = 0 as *mut xmlNs
            } else { node.ns = &mut ns; ns.href = (*def1).ns }
        } else { node.ns = 0 as *mut xmlNs }
        if xmlRelaxNGElementMatch(&mut ctxt, def2, &mut node) != 0 {
            if !(*def1).nameClass.is_null() {
                ret = xmlRelaxNGCompareNameClasses((*def1).nameClass, def2)
            } else { ret = 0 as std::os::raw::c_int }
        } else { ret = 1 as std::os::raw::c_int }
    } else if (*def1).type_0 as std::os::raw::c_int == XML_RELAXNG_TEXT as std::os::raw::c_int
     {
        if (*def2).type_0 as std::os::raw::c_int == XML_RELAXNG_TEXT as std::os::raw::c_int {
            return 0 as std::os::raw::c_int
        }
        return 1 as std::os::raw::c_int
    } else {
        if (*def1).type_0 as std::os::raw::c_int == XML_RELAXNG_EXCEPT as std::os::raw::c_int
           {
            ret = xmlRelaxNGCompareNameClasses((*def1).content, def2);
            if ret == 0 as std::os::raw::c_int {
                ret = 1 as std::os::raw::c_int
            } else if ret == 1 as std::os::raw::c_int { ret = 0 as std::os::raw::c_int }
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       3831 as
                                                                           std::os::raw::c_int);
            ret = 0 as std::os::raw::c_int
        }
    }
    if ret == 0 as std::os::raw::c_int { return ret }
    if (*def2).type_0 as std::os::raw::c_int == XML_RELAXNG_ELEMENT as std::os::raw::c_int ||
           (*def2).type_0 as std::os::raw::c_int ==
               XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
        if !(*def2).name.is_null() {
            node.name = (*def2).name
        } else { node.name = invalidName }
        node.ns = &mut ns;
        if !(*def2).ns.is_null() {
            if *(*def2).ns.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int {
                node.ns = 0 as *mut xmlNs
            } else { ns.href = (*def2).ns }
        } else { ns.href = invalidName }
        if xmlRelaxNGElementMatch(&mut ctxt, def1, &mut node) != 0 {
            if !(*def2).nameClass.is_null() {
                ret = xmlRelaxNGCompareNameClasses((*def2).nameClass, def1)
            } else { ret = 0 as std::os::raw::c_int }
        } else { ret = 1 as std::os::raw::c_int }
    } else {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"relaxng.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   3862 as
                                                                       std::os::raw::c_int);
        ret = 0 as std::os::raw::c_int
    }
    return ret;
}
/* *
 * xmlRelaxNGCompareElemDefLists:
 * @ctxt:  a Relax-NG parser context
 * @defs1:  the first list of element/attribute defs
 * @defs2:  the second list of element/attribute defs
 *
 * Compare the 2 lists of element or attribute definitions. The comparison
 * is that if both lists do not accept the same QNames, it returns 1
 * If the 2 lists can accept the same QName the comparison returns 0
 *
 * Returns 1 disttinct, 0 if equal
 */
unsafe extern "C" fn xmlRelaxNGCompareElemDefLists(mut ctxt:
                                                       xmlRelaxNGParserCtxtPtr,
                                                   mut def1:
                                                       *mut xmlRelaxNGDefinePtr,
                                                   mut def2:
                                                       *mut xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut basedef2: *mut xmlRelaxNGDefinePtr = def2;
    if def1.is_null() || def2.is_null() { return 1 as std::os::raw::c_int }
    if (*def1).is_null() || (*def2).is_null() { return 1 as std::os::raw::c_int }
    while !(*def1).is_null() {
        while !(*def2).is_null() {
            if xmlRelaxNGCompareNameClasses(*def1, *def2) == 0 as std::os::raw::c_int
               {
                return 0 as std::os::raw::c_int
            }
            def2 = def2.offset(1)
        }
        def2 = basedef2;
        def1 = def1.offset(1)
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGGenerateAttributes:
 * @ctxt:  a Relax-NG parser context
 * @def:  the definition definition
 *
 * Check if the definition can only generate attributes
 *
 * Returns 1 if yes, 0 if no and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGGenerateAttributes(mut ctxt:
                                                      xmlRelaxNGParserCtxtPtr,
                                                  mut def:
                                                      xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut parent: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    /*
     * Don't run that check in case of error. Infinite recursion
     * becomes possible.
     */
    if (*ctxt).nbErrors != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    parent = 0 as xmlRelaxNGDefinePtr;
    cur = def;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_ELEMENT as std::os::raw::c_int
               ||
               (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_TEXT as std::os::raw::c_int
               ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_DATATYPE as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_PARAM as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_LIST as std::os::raw::c_int
               ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_VALUE as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_EMPTY as std::os::raw::c_int {
            return 0 as std::os::raw::c_int
        }
        if (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_CHOICE as std::os::raw::c_int
               ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_INTERLEAVE as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_GROUP as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_ZEROORMORE as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_OPTIONAL as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_PARENTREF as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_EXTERNALREF as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_REF as std::os::raw::c_int
               ||
               (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_DEF as std::os::raw::c_int
           {
            if !(*cur).content.is_null() {
                parent = cur;
                cur = (*cur).content;
                tmp = cur;
                while !tmp.is_null() {
                    (*tmp).parent = parent;
                    tmp = (*tmp).next
                }
                continue ;
            }
        }
        if cur == def { break ; }
        if !(*cur).next.is_null() {
            cur = (*cur).next
        } else {
            loop  {
                cur = (*cur).parent;
                if cur.is_null() { break ; }
                if cur == def { return 1 as std::os::raw::c_int }
                if !(*cur).next.is_null() {
                    cur = (*cur).next;
                    break ;
                } else if cur.is_null() { break ; }
            }
        }
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGGetElements:
 * @ctxt:  a Relax-NG parser context
 * @def:  the definition definition
 * @eora:  gather elements (0) or attributes (1)
 *
 * Compute the list of top elements a definition can generate
 *
 * Returns a list of elements or NULL if none was found.
 */
unsafe extern "C" fn xmlRelaxNGGetElements(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                           mut def: xmlRelaxNGDefinePtr,
                                           mut eora: std::os::raw::c_int)
 -> *mut xmlRelaxNGDefinePtr {
    let mut ret: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
    let mut parent: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut max: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
     * Don't run that check in case of error. Infinite recursion
     * becomes possible.
     */
    if (*ctxt).nbErrors != 0 as std::os::raw::c_int {
        return 0 as *mut xmlRelaxNGDefinePtr
    }
    parent = 0 as xmlRelaxNGDefinePtr;
    cur = def;
    while !cur.is_null() {
        if eora == 0 as std::os::raw::c_int &&
               ((*cur).type_0 as std::os::raw::c_int ==
                    XML_RELAXNG_ELEMENT as std::os::raw::c_int ||
                    (*cur).type_0 as std::os::raw::c_int ==
                        XML_RELAXNG_TEXT as std::os::raw::c_int) ||
               eora == 1 as std::os::raw::c_int &&
                   (*cur).type_0 as std::os::raw::c_int ==
                       XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
            if ret.is_null() {
                max = 10 as std::os::raw::c_int;
                ret =
                    xmlMalloc.expect("non-null function pointer")(((max +
                                                                        1 as
                                                                            std::os::raw::c_int)
                                                                       as
                                                                       std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGDefinePtr>()
                                                                                                       as
                                                                                                       std::os::raw::c_ulong))
                        as *mut xmlRelaxNGDefinePtr;
                if ret.is_null() {
                    xmlRngPErrMemory(ctxt,
                                     b"getting element list\n\x00" as
                                         *const u8 as *const std::os::raw::c_char);
                    return 0 as *mut xmlRelaxNGDefinePtr
                }
            } else if max <= len {
                let mut temp: *mut xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefinePtr;
                max *= 2 as std::os::raw::c_int;
                temp =
                    xmlRealloc.expect("non-null function pointer")(ret as
                                                                       *mut std::os::raw::c_void,
                                                                   ((max +
                                                                         1 as
                                                                             std::os::raw::c_int)
                                                                        as
                                                                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGDefinePtr>()
                                                                                                        as
                                                                                                        std::os::raw::c_ulong))
                        as *mut xmlRelaxNGDefinePtr;
                if temp.is_null() {
                    xmlRngPErrMemory(ctxt,
                                     b"getting element list\n\x00" as
                                         *const u8 as *const std::os::raw::c_char);
                    xmlFree.expect("non-null function pointer")(ret as
                                                                    *mut std::os::raw::c_void);
                    return 0 as *mut xmlRelaxNGDefinePtr
                }
                ret = temp
            }
            let fresh18 = len;
            len = len + 1;
            let ref mut fresh19 = *ret.offset(fresh18 as isize);
            *fresh19 = cur;
            let ref mut fresh20 = *ret.offset(len as isize);
            *fresh20 = 0 as xmlRelaxNGDefinePtr
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_CHOICE as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_INTERLEAVE as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_GROUP as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_ZEROORMORE as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_OPTIONAL as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_PARENTREF as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_REF as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_DEF as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_EXTERNALREF as std::os::raw::c_int {
            /*
             * Don't go within elements or attributes or string values.
             * Just gather the element top list
             */
            if !(*cur).content.is_null() {
                parent = cur;
                cur = (*cur).content;
                tmp = cur;
                while !tmp.is_null() {
                    (*tmp).parent = parent;
                    tmp = (*tmp).next
                }
                continue ;
            }
        }
        if cur == def { break ; }
        if !(*cur).next.is_null() {
            cur = (*cur).next
        } else {
            loop  {
                cur = (*cur).parent;
                if cur.is_null() { break ; }
                if cur == def { return ret }
                if !(*cur).next.is_null() {
                    cur = (*cur).next;
                    break ;
                } else if cur.is_null() { break ; }
            }
        }
    }
    return ret;
}
/* *
 * xmlRelaxNGCheckChoiceDeterminism:
 * @ctxt:  a Relax-NG parser context
 * @def:  the choice definition
 *
 * Also used to find indeterministic pattern in choice
 */
unsafe extern "C" fn xmlRelaxNGCheckChoiceDeterminism(mut ctxt:
                                                          xmlRelaxNGParserCtxtPtr,
                                                      mut def:
                                                          xmlRelaxNGDefinePtr) {
    let mut list: *mut *mut xmlRelaxNGDefinePtr =
        0 as *mut *mut xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut nbchild: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut is_nullable: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut is_indeterminist: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut triage: xmlHashTablePtr = 0 as xmlHashTablePtr;
    let mut is_triable: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if def.is_null() ||
           (*def).type_0 as std::os::raw::c_int != XML_RELAXNG_CHOICE as std::os::raw::c_int {
        return
    }
    if (*def).dflags as std::os::raw::c_int & (1 as std::os::raw::c_int) << 5 as std::os::raw::c_int
           != 0 {
        return
    }
    /*
     * Don't run that check in case of error. Infinite recursion
     * becomes possible.
     */
    if (*ctxt).nbErrors != 0 as std::os::raw::c_int { return }
    is_nullable = xmlRelaxNGIsNullable(def);
    cur = (*def).content;
    while !cur.is_null() { nbchild += 1; cur = (*cur).next }
    list =
        xmlMalloc.expect("non-null function pointer")((nbchild as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlRelaxNGDefinePtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut *mut xmlRelaxNGDefinePtr;
    if list.is_null() {
        xmlRngPErrMemory(ctxt,
                         b"building choice\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return
    }
    i = 0 as std::os::raw::c_int;
    /*
     * a bit strong but safe
     */
    if is_nullable == 0 as std::os::raw::c_int {
        triage = xmlHashCreate(10 as std::os::raw::c_int)
    } else { is_triable = 0 as std::os::raw::c_int }
    cur = (*def).content;
    while !cur.is_null() {
        let ref mut fresh21 = *list.offset(i as isize);
        *fresh21 = xmlRelaxNGGetElements(ctxt, cur, 0 as std::os::raw::c_int);
        if (*list.offset(i as isize)).is_null() ||
               (*(*list.offset(i as
                                   isize)).offset(0 as std::os::raw::c_int as
                                                      isize)).is_null() {
            is_triable = 0 as std::os::raw::c_int
        } else if is_triable == 1 as std::os::raw::c_int {
            let mut tmp: *mut xmlRelaxNGDefinePtr =
                0 as *mut xmlRelaxNGDefinePtr;
            let mut res: std::os::raw::c_int = 0;
            tmp = *list.offset(i as isize);
            while !(*tmp).is_null() && is_triable == 1 as std::os::raw::c_int {
                if (**tmp).type_0 as std::os::raw::c_int ==
                       XML_RELAXNG_TEXT as std::os::raw::c_int {
                    res =
                        xmlHashAddEntry2(triage,
                                         b"#text\x00" as *const u8 as
                                             *const std::os::raw::c_char as
                                             *mut xmlChar,
                                         0 as *const xmlChar,
                                         cur as *mut std::os::raw::c_void);
                    if res != 0 as std::os::raw::c_int {
                        is_triable = -(1 as std::os::raw::c_int)
                    }
                } else if (**tmp).type_0 as std::os::raw::c_int ==
                              XML_RELAXNG_ELEMENT as std::os::raw::c_int &&
                              !(**tmp).name.is_null() {
                    if (**tmp).ns.is_null() ||
                           *(**tmp).ns.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == 0 as std::os::raw::c_int {
                        res =
                            xmlHashAddEntry2(triage, (**tmp).name,
                                             0 as *const xmlChar,
                                             cur as *mut std::os::raw::c_void)
                    } else {
                        res =
                            xmlHashAddEntry2(triage, (**tmp).name, (**tmp).ns,
                                             cur as *mut std::os::raw::c_void)
                    }
                    if res != 0 as std::os::raw::c_int {
                        is_triable = -(1 as std::os::raw::c_int)
                    }
                } else if (**tmp).type_0 as std::os::raw::c_int ==
                              XML_RELAXNG_ELEMENT as std::os::raw::c_int {
                    if (**tmp).ns.is_null() ||
                           *(**tmp).ns.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == 0 as std::os::raw::c_int {
                        res =
                            xmlHashAddEntry2(triage,
                                             b"#any\x00" as *const u8 as
                                                 *const std::os::raw::c_char as
                                                 *mut xmlChar,
                                             0 as *const xmlChar,
                                             cur as *mut std::os::raw::c_void)
                    } else {
                        res =
                            xmlHashAddEntry2(triage,
                                             b"#any\x00" as *const u8 as
                                                 *const std::os::raw::c_char as
                                                 *mut xmlChar, (**tmp).ns,
                                             cur as *mut std::os::raw::c_void)
                    }
                    if res != 0 as std::os::raw::c_int {
                        is_triable = -(1 as std::os::raw::c_int)
                    }
                } else { is_triable = -(1 as std::os::raw::c_int) }
                tmp = tmp.offset(1)
            }
        }
        i += 1;
        cur = (*cur).next
    }
    i = 0 as std::os::raw::c_int;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            j = 0 as std::os::raw::c_int;
            while j < i {
                if !(*list.offset(j as isize)).is_null() {
                    ret =
                        xmlRelaxNGCompareElemDefLists(ctxt,
                                                      *list.offset(i as
                                                                       isize),
                                                      *list.offset(j as
                                                                       isize));
                    if ret == 0 as std::os::raw::c_int {
                        is_indeterminist = 1 as std::os::raw::c_int
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            xmlFree.expect("non-null function pointer")(*list.offset(i as
                                                                         isize)
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    }
    xmlFree.expect("non-null function pointer")(list as *mut std::os::raw::c_void);
    if is_indeterminist != 0 {
        (*def).dflags =
            ((*def).dflags as std::os::raw::c_int |
                 (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as std::os::raw::c_short
    }
    if is_triable == 1 as std::os::raw::c_int {
        (*def).dflags =
            ((*def).dflags as std::os::raw::c_int |
                 (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int) as std::os::raw::c_short;
        (*def).data = triage as *mut std::os::raw::c_void
    } else if !triage.is_null() { xmlHashFree(triage, None); }
    (*def).dflags =
        ((*def).dflags as std::os::raw::c_int |
             (1 as std::os::raw::c_int) << 5 as std::os::raw::c_int) as std::os::raw::c_short;
}
/* *
 * xmlRelaxNGCheckGroupAttrs:
 * @ctxt:  a Relax-NG parser context
 * @def:  the group definition
 *
 * Detects violations of rule 7.3
 */
unsafe extern "C" fn xmlRelaxNGCheckGroupAttrs(mut ctxt:
                                                   xmlRelaxNGParserCtxtPtr,
                                               mut def: xmlRelaxNGDefinePtr) {
    let mut list: *mut *mut xmlRelaxNGDefinePtr =
        0 as *mut *mut xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut nbchild: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    if def.is_null() ||
           (*def).type_0 as std::os::raw::c_int != XML_RELAXNG_GROUP as std::os::raw::c_int &&
               (*def).type_0 as std::os::raw::c_int !=
                   XML_RELAXNG_ELEMENT as std::os::raw::c_int {
        return
    }
    if (*def).dflags as std::os::raw::c_int & (1 as std::os::raw::c_int) << 5 as std::os::raw::c_int
           != 0 {
        return
    }
    /*
     * Don't run that check in case of error. Infinite recursion
     * becomes possible.
     */
    if (*ctxt).nbErrors != 0 as std::os::raw::c_int { return }
    cur = (*def).attrs;
    while !cur.is_null() { nbchild += 1; cur = (*cur).next }
    cur = (*def).content;
    while !cur.is_null() { nbchild += 1; cur = (*cur).next }
    list =
        xmlMalloc.expect("non-null function pointer")((nbchild as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlRelaxNGDefinePtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut *mut xmlRelaxNGDefinePtr;
    if list.is_null() {
        xmlRngPErrMemory(ctxt,
                         b"building group\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return
    }
    i = 0 as std::os::raw::c_int;
    cur = (*def).attrs;
    while !cur.is_null() {
        let ref mut fresh22 = *list.offset(i as isize);
        *fresh22 = xmlRelaxNGGetElements(ctxt, cur, 1 as std::os::raw::c_int);
        i += 1;
        cur = (*cur).next
    }
    cur = (*def).content;
    while !cur.is_null() {
        let ref mut fresh23 = *list.offset(i as isize);
        *fresh23 = xmlRelaxNGGetElements(ctxt, cur, 1 as std::os::raw::c_int);
        i += 1;
        cur = (*cur).next
    }
    i = 0 as std::os::raw::c_int;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            j = 0 as std::os::raw::c_int;
            while j < i {
                if !(*list.offset(j as isize)).is_null() {
                    ret =
                        xmlRelaxNGCompareElemDefLists(ctxt,
                                                      *list.offset(i as
                                                                       isize),
                                                      *list.offset(j as
                                                                       isize));
                    if ret == 0 as std::os::raw::c_int {
                        xmlRngPErr(ctxt, (*def).node,
                                   XML_RNGP_GROUP_ATTR_CONFLICT as
                                       std::os::raw::c_int,
                                   b"Attributes conflicts in group\n\x00" as
                                       *const u8 as *const std::os::raw::c_char,
                                   0 as *const xmlChar, 0 as *const xmlChar);
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            xmlFree.expect("non-null function pointer")(*list.offset(i as
                                                                         isize)
                                                            as
                                                            *mut std::os::raw::c_void);
        }
        i += 1
    }
    xmlFree.expect("non-null function pointer")(list as *mut std::os::raw::c_void);
    (*def).dflags =
        ((*def).dflags as std::os::raw::c_int |
             (1 as std::os::raw::c_int) << 5 as std::os::raw::c_int) as std::os::raw::c_short;
}
/* *
 * xmlRelaxNGComputeInterleaves:
 * @def:  the interleave definition
 * @ctxt:  a Relax-NG parser context
 * @name:  the definition name
 *
 * A lot of work for preprocessing interleave definitions
 * is potentially needed to get a decent execution speed at runtime
 *   - trying to get a total order on the element nodes generated
 *     by the interleaves, order the list of interleave definitions
 *     following that order.
 *   - if <text/> is used to handle mixed content, it is better to
 *     flag this in the define and simplify the runtime checking
 *     algorithm
 */
unsafe extern "C" fn xmlRelaxNGComputeInterleaves(mut payload:
                                                      *mut std::os::raw::c_void,
                                                  mut data: *mut std::os::raw::c_void,
                                                  mut name: *const xmlChar) {
    let mut current_block: u64;
    let mut def: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
    let mut partitions: xmlRelaxNGPartitionPtr = 0 as xmlRelaxNGPartitionPtr;
    let mut groups: *mut xmlRelaxNGInterleaveGroupPtr =
        0 as *mut xmlRelaxNGInterleaveGroupPtr;
    let mut group: xmlRelaxNGInterleaveGroupPtr =
        0 as *mut xmlRelaxNGInterleaveGroup;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut res: std::os::raw::c_int = 0;
    let mut nbgroups: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut nbchild: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut is_mixed: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut is_determinist: std::os::raw::c_int = 1 as std::os::raw::c_int;
    /*
     * Don't run that check in case of error. Infinite recursion
     * becomes possible.
     */
    if (*ctxt).nbErrors != 0 as std::os::raw::c_int { return }
    cur = (*def).content;
    while !cur.is_null() { nbchild += 1; cur = (*cur).next }
    groups =
        xmlMalloc.expect("non-null function pointer")((nbchild as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRelaxNGInterleaveGroupPtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut xmlRelaxNGInterleaveGroupPtr;
    if !groups.is_null() {
        cur = (*def).content;
        loop  {
            if cur.is_null() { current_block = 15768484401365413375; break ; }
            let ref mut fresh24 = *groups.offset(nbgroups as isize);
            *fresh24 =
                xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGInterleaveGroup>()
                                                                  as
                                                                  std::os::raw::c_ulong)
                    as xmlRelaxNGInterleaveGroupPtr;
            if (*groups.offset(nbgroups as isize)).is_null() {
                current_block = 3729150195017652645;
                break ;
            }
            if (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_TEXT as std::os::raw::c_int
               {
                is_mixed += 1
            }
            let ref mut fresh25 = (**groups.offset(nbgroups as isize)).rule;
            *fresh25 = cur;
            let ref mut fresh26 = (**groups.offset(nbgroups as isize)).defs;
            *fresh26 = xmlRelaxNGGetElements(ctxt, cur, 0 as std::os::raw::c_int);
            let ref mut fresh27 = (**groups.offset(nbgroups as isize)).attrs;
            *fresh27 = xmlRelaxNGGetElements(ctxt, cur, 1 as std::os::raw::c_int);
            nbgroups += 1;
            cur = (*cur).next
        }
        match current_block {
            3729150195017652645 => { }
            _ => {
                /*
     * Let's check that all rules makes a partitions according to 7.4
     */
                partitions =
                    xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGPartition>()
                                                                      as
                                                                      std::os::raw::c_ulong)
                        as xmlRelaxNGPartitionPtr;
                if !partitions.is_null() {
                    memset(partitions as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                           ::std::mem::size_of::<xmlRelaxNGPartition>() as
                               std::os::raw::c_ulong);
                    (*partitions).nbgroups = nbgroups;
                    (*partitions).triage = xmlHashCreate(nbgroups);
                    i = 0 as std::os::raw::c_int;
                    while i < nbgroups {
                        group = *groups.offset(i as isize);
                        j = i + 1 as std::os::raw::c_int;
                        while j < nbgroups {
                            if !(*groups.offset(j as isize)).is_null() {
                                ret =
                                    xmlRelaxNGCompareElemDefLists(ctxt,
                                                                  (*group).defs,
                                                                  (**groups.offset(j
                                                                                       as
                                                                                       isize)).defs);
                                if ret == 0 as std::os::raw::c_int {
                                    xmlRngPErr(ctxt, (*def).node,
                                               XML_RNGP_ELEM_TEXT_CONFLICT as
                                                   std::os::raw::c_int,
                                               b"Element or text conflicts in interleave\n\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char,
                                               0 as *const xmlChar,
                                               0 as *const xmlChar);
                                }
                                ret =
                                    xmlRelaxNGCompareElemDefLists(ctxt,
                                                                  (*group).attrs,
                                                                  (**groups.offset(j
                                                                                       as
                                                                                       isize)).attrs);
                                if ret == 0 as std::os::raw::c_int {
                                    xmlRngPErr(ctxt, (*def).node,
                                               XML_RNGP_ATTR_CONFLICT as
                                                   std::os::raw::c_int,
                                               b"Attributes conflicts in interleave\n\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char,
                                               0 as *const xmlChar,
                                               0 as *const xmlChar);
                                }
                            }
                            j += 1
                        }
                        tmp = (*group).defs;
                        if !tmp.is_null() && !(*tmp).is_null() {
                            while !(*tmp).is_null() {
                                if (**tmp).type_0 as std::os::raw::c_int ==
                                       XML_RELAXNG_TEXT as std::os::raw::c_int {
                                    res =
                                        xmlHashAddEntry2((*partitions).triage,
                                                         b"#text\x00" as
                                                             *const u8 as
                                                             *const std::os::raw::c_char
                                                             as *mut xmlChar,
                                                         0 as *const xmlChar,
                                                         (i +
                                                              1 as
                                                                  std::os::raw::c_int)
                                                             as ptrdiff_t as
                                                             *mut std::os::raw::c_void);
                                    if res != 0 as std::os::raw::c_int {
                                        is_determinist = -(1 as std::os::raw::c_int)
                                    }
                                } else if (**tmp).type_0 as std::os::raw::c_int ==
                                              XML_RELAXNG_ELEMENT as
                                                  std::os::raw::c_int &&
                                              !(**tmp).name.is_null() {
                                    if (**tmp).ns.is_null() ||
                                           *(**tmp).ns.offset(0 as std::os::raw::c_int
                                                                  as isize) as
                                               std::os::raw::c_int == 0 as std::os::raw::c_int
                                       {
                                        res =
                                            xmlHashAddEntry2((*partitions).triage,
                                                             (**tmp).name,
                                                             0 as
                                                                 *const xmlChar,
                                                             (i +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as ptrdiff_t
                                                                 as
                                                                 *mut std::os::raw::c_void)
                                    } else {
                                        res =
                                            xmlHashAddEntry2((*partitions).triage,
                                                             (**tmp).name,
                                                             (**tmp).ns,
                                                             (i +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as ptrdiff_t
                                                                 as
                                                                 *mut std::os::raw::c_void)
                                    }
                                    if res != 0 as std::os::raw::c_int {
                                        is_determinist = -(1 as std::os::raw::c_int)
                                    }
                                } else if (**tmp).type_0 as std::os::raw::c_int ==
                                              XML_RELAXNG_ELEMENT as
                                                  std::os::raw::c_int {
                                    if (**tmp).ns.is_null() ||
                                           *(**tmp).ns.offset(0 as std::os::raw::c_int
                                                                  as isize) as
                                               std::os::raw::c_int == 0 as std::os::raw::c_int
                                       {
                                        res =
                                            xmlHashAddEntry2((*partitions).triage,
                                                             b"#any\x00" as
                                                                 *const u8 as
                                                                 *const std::os::raw::c_char
                                                                 as
                                                                 *mut xmlChar,
                                                             0 as
                                                                 *const xmlChar,
                                                             (i +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as ptrdiff_t
                                                                 as
                                                                 *mut std::os::raw::c_void)
                                    } else {
                                        res =
                                            xmlHashAddEntry2((*partitions).triage,
                                                             b"#any\x00" as
                                                                 *const u8 as
                                                                 *const std::os::raw::c_char
                                                                 as
                                                                 *mut xmlChar,
                                                             (**tmp).ns,
                                                             (i +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as ptrdiff_t
                                                                 as
                                                                 *mut std::os::raw::c_void)
                                    }
                                    if !(**tmp).nameClass.is_null() {
                                        is_determinist = 2 as std::os::raw::c_int
                                    }
                                    if res != 0 as std::os::raw::c_int {
                                        is_determinist = -(1 as std::os::raw::c_int)
                                    }
                                } else {
                                    is_determinist = -(1 as std::os::raw::c_int)
                                }
                                tmp = tmp.offset(1)
                            }
                        } else { is_determinist = 0 as std::os::raw::c_int }
                        i += 1
                    }
                    (*partitions).groups = groups;
                    /*
     * and save the partition list back in the def
     */
                    (*def).data = partitions as *mut std::os::raw::c_void;
                    if is_mixed != 0 as std::os::raw::c_int {
                        (*def).dflags =
                            ((*def).dflags as std::os::raw::c_int |
                                 (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int) as
                                std::os::raw::c_short
                    }
                    if is_determinist == 1 as std::os::raw::c_int {
                        (*partitions).flags = 1 as std::os::raw::c_int
                    }
                    if is_determinist == 2 as std::os::raw::c_int {
                        (*partitions).flags =
                            1 as std::os::raw::c_int | 2 as std::os::raw::c_int
                    }
                    return
                }
            }
        }
    }
    xmlRngPErrMemory(ctxt,
                     b"in interleave computation\n\x00" as *const u8 as
                         *const std::os::raw::c_char);
    if !groups.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < nbgroups {
            if !(*groups.offset(i as isize)).is_null() {
                if !(**groups.offset(i as isize)).defs.is_null() {
                    xmlFree.expect("non-null function pointer")((**groups.offset(i
                                                                                     as
                                                                                     isize)).defs
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
                xmlFree.expect("non-null function pointer")(*groups.offset(i
                                                                               as
                                                                               isize)
                                                                as
                                                                *mut std::os::raw::c_void);
            }
            i += 1
        }
        xmlFree.expect("non-null function pointer")(groups as
                                                        *mut std::os::raw::c_void);
    }
    xmlRelaxNGFreePartition(partitions);
}
/* *
 * xmlRelaxNGParseInterleave:
 * @ctxt:  a Relax-NG parser context
 * @node:  the data node.
 *
 * parse the content of a RelaxNG interleave node.
 *
 * Returns the definition pointer or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseInterleave(mut ctxt:
                                                   xmlRelaxNGParserCtxtPtr,
                                               mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    def = xmlRelaxNGNewDefine(ctxt, node);
    if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
    (*def).type_0 = XML_RELAXNG_INTERLEAVE;
    if (*ctxt).interleaves.is_null() {
        (*ctxt).interleaves = xmlHashCreate(10 as std::os::raw::c_int)
    }
    if (*ctxt).interleaves.is_null() {
        xmlRngPErrMemory(ctxt,
                         b"create interleaves\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
    } else {
        let mut name: [std::os::raw::c_char; 32] = [0; 32];
        let fresh28 = (*ctxt).nbInterleaves;
        (*ctxt).nbInterleaves = (*ctxt).nbInterleaves + 1;
        snprintf(name.as_mut_ptr(), 32 as std::os::raw::c_int as std::os::raw::c_ulong,
                 b"interleave%d\x00" as *const u8 as *const std::os::raw::c_char,
                 fresh28);
        if xmlHashAddEntry((*ctxt).interleaves,
                           name.as_mut_ptr() as *mut xmlChar,
                           def as *mut std::os::raw::c_void) < 0 as std::os::raw::c_int {
            xmlRngPErr(ctxt, node, XML_RNGP_INTERLEAVE_ADD as std::os::raw::c_int,
                       b"Failed to add %s to hash table\n\x00" as *const u8 as
                           *const std::os::raw::c_char,
                       name.as_mut_ptr() as *const xmlChar,
                       0 as *const xmlChar);
        }
    }
    child = (*node).children;
    if child.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_INTERLEAVE_NO_CONTENT as std::os::raw::c_int,
                   b"Element interleave is empty\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
    }
    while !child.is_null() {
        if !child.is_null() && !(*child).ns.is_null() &&
               (*child).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*child).name,
                           b"element\x00" as *const u8 as *const std::os::raw::c_char
                               as *const xmlChar) != 0 &&
               xmlStrEqual((*(*child).ns).href, xmlRelaxNGNs) != 0 {
            cur = xmlRelaxNGParseElement(ctxt, child)
        } else { cur = xmlRelaxNGParsePattern(ctxt, child) }
        if !cur.is_null() {
            (*cur).parent = def;
            if last.is_null() {
                last = cur;
                (*def).content = last
            } else { (*last).next = cur; last = cur }
        }
        child = (*child).next
    }
    return def;
}
/* *
 * xmlRelaxNGParseInclude:
 * @ctxt:  a Relax-NG parser context
 * @node:  the include node
 *
 * Integrate the content of an include node in the current grammar
 *
 * Returns 0 in case of success or -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseInclude(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                            mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut incl: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tmp: std::os::raw::c_int = 0;
    incl = (*node).psvi as xmlRelaxNGIncludePtr;
    if incl.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_INCLUDE_EMPTY as std::os::raw::c_int,
                   b"Include node has no data\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    root = xmlDocGetRootElement((*incl).doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_EMPTY as std::os::raw::c_int,
                   b"Include document is empty\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    if xmlStrEqual((*root).name,
                   b"grammar\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) == 0 {
        xmlRngPErr(ctxt, node, XML_RNGP_GRAMMAR_MISSING as std::os::raw::c_int,
                   b"Include document root is not a grammar\n\x00" as
                       *const u8 as *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Merge the definition from both the include and the internal list
     */
    if !(*root).children.is_null() {
        tmp = xmlRelaxNGParseGrammarContent(ctxt, (*root).children);
        if tmp != 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
    }
    if !(*node).children.is_null() {
        tmp = xmlRelaxNGParseGrammarContent(ctxt, (*node).children);
        if tmp != 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
    }
    return ret;
}
/* *
 * xmlRelaxNGParseDefine:
 * @ctxt:  a Relax-NG parser context
 * @node:  the define node
 *
 * parse the content of a RelaxNG define element node.
 *
 * Returns 0 in case of success or -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseDefine(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                           mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tmp: std::os::raw::c_int = 0;
    let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut olddefine: *const xmlChar = 0 as *const xmlChar;
    name =
        xmlGetProp(node as *const xmlNode,
                   b"name\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    if name.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_DEFINE_NAME_MISSING as std::os::raw::c_int,
                   b"define has no name\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
    } else {
        xmlRelaxNGNormExtSpace(name);
        if xmlValidateNCName(name, 0 as std::os::raw::c_int) != 0 {
            xmlRngPErr(ctxt, node,
                       XML_RNGP_INVALID_DEFINE_NAME as std::os::raw::c_int,
                       b"define name \'%s\' is not an NCName\n\x00" as
                           *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
        }
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
            return -(1 as std::os::raw::c_int)
        }
        (*def).type_0 = XML_RELAXNG_DEF;
        (*def).name = name;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_DEFINE_EMPTY as std::os::raw::c_int,
                       b"define has no children\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        } else {
            olddefine = (*ctxt).define;
            (*ctxt).define = name;
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        0 as std::os::raw::c_int);
            (*ctxt).define = olddefine
        }
        if (*(*ctxt).grammar).defs.is_null() {
            (*(*ctxt).grammar).defs = xmlHashCreate(10 as std::os::raw::c_int)
        }
        if (*(*ctxt).grammar).defs.is_null() {
            xmlRngPErr(ctxt, node,
                       XML_RNGP_DEFINE_CREATE_FAILED as std::os::raw::c_int,
                       b"Could not create definition hash\n\x00" as *const u8
                           as *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
            ret = -(1 as std::os::raw::c_int)
        } else {
            tmp =
                xmlHashAddEntry((*(*ctxt).grammar).defs, name,
                                def as *mut std::os::raw::c_void);
            if tmp < 0 as std::os::raw::c_int {
                let mut prev: xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefine;
                prev =
                    xmlHashLookup((*(*ctxt).grammar).defs, name) as
                        xmlRelaxNGDefinePtr;
                if prev.is_null() {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_DEFINE_CREATE_FAILED as std::os::raw::c_int,
                               b"Internal error on define aggregation of %s\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, name,
                               0 as *const xmlChar);
                    ret = -(1 as std::os::raw::c_int)
                } else {
                    while !(*prev).nextHash.is_null() {
                        prev = (*prev).nextHash
                    }
                    (*prev).nextHash = def
                }
            }
        }
    }
    return ret;
}
/* *
 * xmlRelaxNGParseImportRef:
 * @payload: the parser context
 * @data: the current grammar
 * @name: the reference name
 *
 * Import import one references into the current grammar
 */
unsafe extern "C" fn xmlRelaxNGParseImportRef(mut payload: *mut std::os::raw::c_void,
                                              mut data: *mut std::os::raw::c_void,
                                              mut name: *const xmlChar) {
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut def: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut tmp: std::os::raw::c_int = 0;
    (*def).dflags =
        ((*def).dflags as std::os::raw::c_int |
             (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) as std::os::raw::c_short;
    tmp =
        xmlHashAddEntry((*(*ctxt).grammar).refs, name,
                        def as *mut std::os::raw::c_void);
    if tmp < 0 as std::os::raw::c_int {
        let mut prev: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        prev =
            xmlHashLookup((*(*ctxt).grammar).refs, (*def).name) as
                xmlRelaxNGDefinePtr;
        if prev.is_null() {
            if !(*def).name.is_null() {
                xmlRngPErr(ctxt, 0 as xmlNodePtr,
                           XML_RNGP_REF_CREATE_FAILED as std::os::raw::c_int,
                           b"Error refs definitions \'%s\'\n\x00" as *const u8
                               as *const std::os::raw::c_char, (*def).name,
                           0 as *const xmlChar);
            } else {
                xmlRngPErr(ctxt, 0 as xmlNodePtr,
                           XML_RNGP_REF_CREATE_FAILED as std::os::raw::c_int,
                           b"Error refs definitions\n\x00" as *const u8 as
                               *const std::os::raw::c_char, 0 as *const xmlChar,
                           0 as *const xmlChar);
            }
        } else { (*def).nextHash = (*prev).nextHash; (*prev).nextHash = def }
    };
}
/* *
 * xmlRelaxNGParseImportRefs:
 * @ctxt: the parser context
 * @grammar: the sub grammar
 *
 * Import references from the subgrammar into the current grammar
 *
 * Returns 0 in case of success, -1 in case of failure
 */
unsafe extern "C" fn xmlRelaxNGParseImportRefs(mut ctxt:
                                                   xmlRelaxNGParserCtxtPtr,
                                               mut grammar:
                                                   xmlRelaxNGGrammarPtr)
 -> std::os::raw::c_int {
    if ctxt.is_null() || grammar.is_null() || (*ctxt).grammar.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if (*grammar).refs.is_null() { return 0 as std::os::raw::c_int }
    if (*(*ctxt).grammar).refs.is_null() {
        (*(*ctxt).grammar).refs = xmlHashCreate(10 as std::os::raw::c_int)
    }
    if (*(*ctxt).grammar).refs.is_null() {
        xmlRngPErr(ctxt, 0 as xmlNodePtr,
                   XML_RNGP_REF_CREATE_FAILED as std::os::raw::c_int,
                   b"Could not create references hash\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    xmlHashScan((*grammar).refs,
                Some(xmlRelaxNGParseImportRef as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()),
                ctxt as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGProcessExternalRef:
 * @ctxt: the parser context
 * @node:  the externlRef node
 *
 * Process and compile an externlRef node
 *
 * Returns the xmlRelaxNGDefinePtr or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGProcessExternalRef(mut ctxt:
                                                      xmlRelaxNGParserCtxtPtr,
                                                  mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut docu: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: *mut xmlChar = 0 as *mut xmlChar;
    let mut newNs: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut oldflags: std::os::raw::c_int = 0;
    let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    docu = (*node).psvi as xmlRelaxNGDocumentPtr;
    if !docu.is_null() {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_EXTERNALREF;
        if (*docu).content.is_null() {
            /*
             * Then do the parsing for good
             */
            root = xmlDocGetRootElement((*docu).doc as *const xmlDoc);
            if root.is_null() {
                xmlRngPErr(ctxt, node,
                           XML_RNGP_EXTERNALREF_EMTPY as std::os::raw::c_int,
                           b"xmlRelaxNGParse: %s is empty\n\x00" as *const u8
                               as *const std::os::raw::c_char, (*ctxt).URL,
                           0 as *const xmlChar);
                return 0 as xmlRelaxNGDefinePtr
            }
            /*
             * ns transmission rules
             */
            ns =
                xmlGetProp(root as *const xmlNode,
                           b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar);
            if ns.is_null() {
                tmp = node;
                while !tmp.is_null() &&
                          (*tmp).type_0 as std::os::raw::c_uint ==
                              XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                      {
                    ns =
                        xmlGetProp(tmp as *const xmlNode,
                                   b"ns\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                    if !ns.is_null() { break ; }
                    tmp = (*tmp).parent
                }
                if !ns.is_null() {
                    xmlSetProp(root,
                               b"ns\x00" as *const u8 as *const std::os::raw::c_char
                                   as *mut xmlChar, ns);
                    newNs = 1 as std::os::raw::c_int;
                    xmlFree.expect("non-null function pointer")(ns as
                                                                    *mut std::os::raw::c_void);
                }
            } else {
                xmlFree.expect("non-null function pointer")(ns as
                                                                *mut std::os::raw::c_void);
            }
            /*
             * Parsing to get a precompiled schemas.
             */
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int;
            (*docu).schema = xmlRelaxNGParseDocument(ctxt, root);
            (*ctxt).flags = oldflags;
            if !(*docu).schema.is_null() &&
                   !(*(*docu).schema).topgrammar.is_null() {
                (*docu).content = (*(*(*docu).schema).topgrammar).start;
                if !(*(*(*docu).schema).topgrammar).refs.is_null() {
                    xmlRelaxNGParseImportRefs(ctxt,
                                              (*(*docu).schema).topgrammar);
                }
            }
            /*
             * the externalRef may be reused in a different ns context
             */
            if newNs == 1 as std::os::raw::c_int {
                xmlUnsetProp(root,
                             b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                                 *mut xmlChar);
            }
        }
        (*def).content = (*docu).content
    } else { def = 0 as xmlRelaxNGDefinePtr }
    return def;
}
/* *
 * xmlRelaxNGParsePattern:
 * @ctxt:  a Relax-NG parser context
 * @node:  the pattern node.
 *
 * parse the content of a RelaxNG pattern node.
 *
 * Returns the definition pointer or NULL in case of error or if no
 *     pattern is generated.
 */
unsafe extern "C" fn xmlRelaxNGParsePattern(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                            mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    if node.is_null() { return 0 as xmlRelaxNGDefinePtr }
    if !node.is_null() && !(*node).ns.is_null() &&
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*node).name,
                       b"element\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 &&
           xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGParseElement(ctxt, node)
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"attribute\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGParseAttribute(ctxt, node)
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"empty\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_EMPTY;
        if !(*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_NOT_EMPTY as std::os::raw::c_int,
                       b"empty: had a child node\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"text\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_TEXT;
        if !(*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_TEXT_HAS_CHILD as std::os::raw::c_int,
                       b"text: had a child node\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"zeroOrMore\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_ZEROORMORE;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Element %s is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, (*node).name,
                       0 as *const xmlChar);
        } else {
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        1 as std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"oneOrMore\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_ONEORMORE;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Element %s is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, (*node).name,
                       0 as *const xmlChar);
        } else {
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        1 as std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"optional\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_OPTIONAL;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Element %s is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, (*node).name,
                       0 as *const xmlChar);
        } else {
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        1 as std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"choice\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_CHOICE;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Element %s is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, (*node).name,
                       0 as *const xmlChar);
        } else {
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        0 as std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"group\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_GROUP;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Element %s is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, (*node).name,
                       0 as *const xmlChar);
        } else {
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        0 as std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"ref\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_REF;
        (*def).name =
            xmlGetProp(node as *const xmlNode,
                       b"name\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        if (*def).name.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_REF_NO_NAME as std::os::raw::c_int,
                       b"ref has no name\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        } else {
            xmlRelaxNGNormExtSpace((*def).name);
            if xmlValidateNCName((*def).name, 0 as std::os::raw::c_int) != 0 {
                xmlRngPErr(ctxt, node,
                           XML_RNGP_REF_NAME_INVALID as std::os::raw::c_int,
                           b"ref name \'%s\' is not an NCName\n\x00" as
                               *const u8 as *const std::os::raw::c_char, (*def).name,
                           0 as *const xmlChar);
            }
        }
        if !(*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_REF_NOT_EMPTY as std::os::raw::c_int,
                       b"ref is not empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
        if (*(*ctxt).grammar).refs.is_null() {
            (*(*ctxt).grammar).refs = xmlHashCreate(10 as std::os::raw::c_int)
        }
        if (*(*ctxt).grammar).refs.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_REF_CREATE_FAILED as std::os::raw::c_int,
                       b"Could not create references hash\n\x00" as *const u8
                           as *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
            def = 0 as xmlRelaxNGDefinePtr
        } else {
            let mut tmp: std::os::raw::c_int = 0;
            tmp =
                xmlHashAddEntry((*(*ctxt).grammar).refs, (*def).name,
                                def as *mut std::os::raw::c_void);
            if tmp < 0 as std::os::raw::c_int {
                let mut prev: xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefine;
                prev =
                    xmlHashLookup((*(*ctxt).grammar).refs, (*def).name) as
                        xmlRelaxNGDefinePtr;
                if prev.is_null() {
                    if !(*def).name.is_null() {
                        xmlRngPErr(ctxt, node,
                                   XML_RNGP_REF_CREATE_FAILED as std::os::raw::c_int,
                                   b"Error refs definitions \'%s\'\n\x00" as
                                       *const u8 as *const std::os::raw::c_char,
                                   (*def).name, 0 as *const xmlChar);
                    } else {
                        xmlRngPErr(ctxt, node,
                                   XML_RNGP_REF_CREATE_FAILED as std::os::raw::c_int,
                                   b"Error refs definitions\n\x00" as
                                       *const u8 as *const std::os::raw::c_char,
                                   0 as *const xmlChar, 0 as *const xmlChar);
                    }
                    def = 0 as xmlRelaxNGDefinePtr
                } else {
                    (*def).nextHash = (*prev).nextHash;
                    (*prev).nextHash = def
                }
            }
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"data\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGParseData(ctxt, node)
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"value\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGParseValue(ctxt, node)
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"list\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_LIST;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Element %s is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, (*node).name,
                       0 as *const xmlChar);
        } else {
            (*def).content =
                xmlRelaxNGParsePatterns(ctxt, (*node).children,
                                        0 as std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"interleave\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGParseInterleave(ctxt, node)
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"externalRef\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGProcessExternalRef(ctxt, node)
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"notAllowed\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_NOT_ALLOWED;
        if !(*node).children.is_null() {
            xmlRngPErr(ctxt, node,
                       XML_RNGP_NOTALLOWED_NOT_EMPTY as std::os::raw::c_int,
                       b"xmlRelaxNGParse: notAllowed element is not empty\n\x00"
                           as *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"grammar\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut old: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut oldparent: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        oldparent = (*ctxt).parentgrammar;
        old = (*ctxt).grammar;
        (*ctxt).parentgrammar = old;
        grammar = xmlRelaxNGParseGrammar(ctxt, (*node).children);
        if !old.is_null() {
            (*ctxt).grammar = old;
            (*ctxt).parentgrammar = oldparent
        }
        if !grammar.is_null() {
            def = (*grammar).start
        } else { def = 0 as xmlRelaxNGDefinePtr }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"parentRef\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        if (*ctxt).parentgrammar.is_null() {
            xmlRngPErr(ctxt, node,
                       XML_RNGP_PARENTREF_NO_PARENT as std::os::raw::c_int,
                       b"Use of parentRef without a parent grammar\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as xmlRelaxNGDefinePtr
        }
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*def).type_0 = XML_RELAXNG_PARENTREF;
        (*def).name =
            xmlGetProp(node as *const xmlNode,
                       b"name\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        if (*def).name.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_PARENTREF_NO_NAME as std::os::raw::c_int,
                       b"parentRef has no name\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        } else {
            xmlRelaxNGNormExtSpace((*def).name);
            if xmlValidateNCName((*def).name, 0 as std::os::raw::c_int) != 0 {
                xmlRngPErr(ctxt, node,
                           XML_RNGP_PARENTREF_NAME_INVALID as std::os::raw::c_int,
                           b"parentRef name \'%s\' is not an NCName\n\x00" as
                               *const u8 as *const std::os::raw::c_char, (*def).name,
                           0 as *const xmlChar);
            }
        }
        if !(*node).children.is_null() {
            xmlRngPErr(ctxt, node,
                       XML_RNGP_PARENTREF_NOT_EMPTY as std::os::raw::c_int,
                       b"parentRef is not empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
        if (*(*ctxt).parentgrammar).refs.is_null() {
            (*(*ctxt).parentgrammar).refs = xmlHashCreate(10 as std::os::raw::c_int)
        }
        if (*(*ctxt).parentgrammar).refs.is_null() {
            xmlRngPErr(ctxt, node,
                       XML_RNGP_PARENTREF_CREATE_FAILED as std::os::raw::c_int,
                       b"Could not create references hash\n\x00" as *const u8
                           as *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
            def = 0 as xmlRelaxNGDefinePtr
        } else if !(*def).name.is_null() {
            let mut tmp_0: std::os::raw::c_int = 0;
            tmp_0 =
                xmlHashAddEntry((*(*ctxt).parentgrammar).refs, (*def).name,
                                def as *mut std::os::raw::c_void);
            if tmp_0 < 0 as std::os::raw::c_int {
                let mut prev_0: xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefine;
                prev_0 =
                    xmlHashLookup((*(*ctxt).parentgrammar).refs, (*def).name)
                        as xmlRelaxNGDefinePtr;
                if prev_0.is_null() {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_PARENTREF_CREATE_FAILED as
                                   std::os::raw::c_int,
                               b"Internal error parentRef definitions \'%s\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               (*def).name, 0 as *const xmlChar);
                    def = 0 as xmlRelaxNGDefinePtr
                } else {
                    (*def).nextHash = (*prev_0).nextHash;
                    (*prev_0).nextHash = def
                }
            }
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"mixed\x00" as *const u8 as *const std::os::raw::c_char
                                  as *const xmlChar) != 0 &&
                  xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_EMPTY_CONSTRUCT as std::os::raw::c_int,
                       b"Mixed is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
            def = 0 as xmlRelaxNGDefinePtr
        } else {
            def = xmlRelaxNGParseInterleave(ctxt, node);
            if !def.is_null() {
                let mut tmp_1: xmlRelaxNGDefinePtr =
                    0 as *mut xmlRelaxNGDefine;
                if !(*def).content.is_null() &&
                       !(*(*def).content).next.is_null() {
                    tmp_1 = xmlRelaxNGNewDefine(ctxt, node);
                    if !tmp_1.is_null() {
                        (*tmp_1).type_0 = XML_RELAXNG_GROUP;
                        (*tmp_1).content = (*def).content;
                        (*def).content = tmp_1
                    }
                }
                tmp_1 = xmlRelaxNGNewDefine(ctxt, node);
                if tmp_1.is_null() { return def }
                (*tmp_1).type_0 = XML_RELAXNG_TEXT;
                (*tmp_1).next = (*def).content;
                (*def).content = tmp_1
            }
        }
    } else {
        xmlRngPErr(ctxt, node, XML_RNGP_UNKNOWN_CONSTRUCT as std::os::raw::c_int,
                   b"Unexpected node %s is not a pattern\n\x00" as *const u8
                       as *const std::os::raw::c_char, (*node).name,
                   0 as *const xmlChar);
        def = 0 as xmlRelaxNGDefinePtr
    }
    return def;
}
/* ***********************************************************************
 *									*
 *			Parsing functions				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGParseAttribute:
 * @ctxt:  a Relax-NG parser context
 * @node:  the element node
 *
 * parse the content of a RelaxNG attribute node.
 *
 * Returns the definition pointer or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGParseAttribute(mut ctxt:
                                                  xmlRelaxNGParserCtxtPtr,
                                              mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut old_flags: std::os::raw::c_int = 0;
    ret = xmlRelaxNGNewDefine(ctxt, node);
    if ret.is_null() { return 0 as xmlRelaxNGDefinePtr }
    (*ret).type_0 = XML_RELAXNG_ATTRIBUTE;
    (*ret).parent = (*ctxt).def;
    child = (*node).children;
    if child.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_ATTRIBUTE_EMPTY as std::os::raw::c_int,
                   b"xmlRelaxNGParseattribute: attribute has no children\n\x00"
                       as *const u8 as *const std::os::raw::c_char,
                   0 as *const xmlChar, 0 as *const xmlChar);
        return ret
    }
    old_flags = (*ctxt).flags;
    (*ctxt).flags |= (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int;
    cur = xmlRelaxNGParseNameClass(ctxt, child, ret);
    if !cur.is_null() { child = (*child).next }
    if !child.is_null() {
        cur = xmlRelaxNGParsePattern(ctxt, child);
        if !cur.is_null() {
            match (*cur).type_0 as std::os::raw::c_int {
                0 | 1 | 3 | 4 | 5 | 7 | 8 | 11 | 13 | 12 | 10 | 16 | 15 | 14 |
                17 | 18 | 19 | 9 => {
                    (*ret).content = cur;
                    (*cur).parent = ret
                }
                20 | 6 | 2 => {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_ATTRIBUTE_CONTENT as std::os::raw::c_int,
                               b"attribute has invalid content\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
                -1 => {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_ATTRIBUTE_NOOP as std::os::raw::c_int,
                               b"RNG Internal error, noop found in attribute\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
                _ => { }
            }
        }
        child = (*child).next
    }
    if !child.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_ATTRIBUTE_CHILDREN as std::os::raw::c_int,
                   b"attribute has multiple children\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
    }
    (*ctxt).flags = old_flags;
    return ret;
}
/* *
 * xmlRelaxNGParseExceptNameClass:
 * @ctxt:  a Relax-NG parser context
 * @node:  the except node
 * @attr:  1 if within an attribute, 0 if within an element
 *
 * parse the content of a RelaxNG nameClass node.
 *
 * Returns the definition pointer or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGParseExceptNameClass(mut ctxt:
                                                        xmlRelaxNGParserCtxtPtr,
                                                    mut node: xmlNodePtr,
                                                    mut attr: std::os::raw::c_int)
 -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    if !(!node.is_null() && !(*node).ns.is_null() &&
             (*node).type_0 as std::os::raw::c_uint ==
                 XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
             xmlStrEqual((*node).name,
                         b"except\x00" as *const u8 as *const std::os::raw::c_char as
                             *const xmlChar) != 0 &&
             xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0) {
        xmlRngPErr(ctxt, node, XML_RNGP_EXCEPT_MISSING as std::os::raw::c_int,
                   b"Expecting an except node\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return 0 as xmlRelaxNGDefinePtr
    }
    if !(*node).next.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_EXCEPT_MULTIPLE as std::os::raw::c_int,
                   b"exceptNameClass allows only a single except node\n\x00"
                       as *const u8 as *const std::os::raw::c_char,
                   0 as *const xmlChar, 0 as *const xmlChar);
    }
    if (*node).children.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_EXCEPT_EMPTY as std::os::raw::c_int,
                   b"except has no content\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return 0 as xmlRelaxNGDefinePtr
    }
    ret = xmlRelaxNGNewDefine(ctxt, node);
    if ret.is_null() { return 0 as xmlRelaxNGDefinePtr }
    (*ret).type_0 = XML_RELAXNG_EXCEPT;
    child = (*node).children;
    while !child.is_null() {
        cur = xmlRelaxNGNewDefine(ctxt, child);
        if cur.is_null() { break ; }
        if attr != 0 {
            (*cur).type_0 = XML_RELAXNG_ATTRIBUTE
        } else { (*cur).type_0 = XML_RELAXNG_ELEMENT }
        if !xmlRelaxNGParseNameClass(ctxt, child, cur).is_null() {
            if last.is_null() {
                (*ret).content = cur
            } else { (*last).next = cur }
            last = cur
        }
        child = (*child).next
    }
    return ret;
}
/* *
 * xmlRelaxNGParseNameClass:
 * @ctxt:  a Relax-NG parser context
 * @node:  the nameClass node
 * @def:  the current definition
 *
 * parse the content of a RelaxNG nameClass node.
 *
 * Returns the definition pointer or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGParseNameClass(mut ctxt:
                                                  xmlRelaxNGParserCtxtPtr,
                                              mut node: xmlNodePtr,
                                              mut def: xmlRelaxNGDefinePtr)
 -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    ret = def;
    if !node.is_null() && !(*node).ns.is_null() &&
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*node).name,
                       b"name\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 &&
           xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 ||
           !node.is_null() && !(*node).ns.is_null() &&
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*node).name,
                           b"anyName\x00" as *const u8 as *const std::os::raw::c_char
                               as *const xmlChar) != 0 &&
               xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 ||
           !node.is_null() && !(*node).ns.is_null() &&
               (*node).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*node).name,
                           b"nsName\x00" as *const u8 as *const std::os::raw::c_char
                               as *const xmlChar) != 0 &&
               xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        if (*def).type_0 as std::os::raw::c_int != XML_RELAXNG_ELEMENT as std::os::raw::c_int
               &&
               (*def).type_0 as std::os::raw::c_int !=
                   XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
            ret = xmlRelaxNGNewDefine(ctxt, node);
            if ret.is_null() { return 0 as xmlRelaxNGDefinePtr }
            (*ret).parent = def;
            if (*ctxt).flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
                (*ret).type_0 = XML_RELAXNG_ATTRIBUTE
            } else { (*ret).type_0 = XML_RELAXNG_ELEMENT }
        }
    }
    if !node.is_null() && !(*node).ns.is_null() &&
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*node).name,
                       b"name\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 &&
           xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        val = xmlNodeGetContent(node as *const xmlNode);
        xmlRelaxNGNormExtSpace(val);
        if xmlValidateNCName(val, 0 as std::os::raw::c_int) != 0 {
            if !(*node).parent.is_null() {
                xmlRngPErr(ctxt, node, XML_RNGP_ELEMENT_NAME as std::os::raw::c_int,
                           b"Element %s name \'%s\' is not an NCName\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           (*(*node).parent).name, val);
            } else {
                xmlRngPErr(ctxt, node, XML_RNGP_ELEMENT_NAME as std::os::raw::c_int,
                           b"name \'%s\' is not an NCName\n\x00" as *const u8
                               as *const std::os::raw::c_char, val,
                           0 as *const xmlChar);
            }
        }
        (*ret).name = val;
        val =
            xmlGetProp(node as *const xmlNode,
                       b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        (*ret).ns = val;
        if (*ctxt).flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 &&
               !val.is_null() &&
               xmlStrEqual(val,
                           b"http://www.w3.org/2000/xmlns\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut xmlChar) != 0 {
            xmlRngPErr(ctxt, node, XML_RNGP_XML_NS as std::os::raw::c_int,
                       b"Attribute with namespace \'%s\' is not allowed\n\x00"
                           as *const u8 as *const std::os::raw::c_char, val,
                       0 as *const xmlChar);
        }
        if (*ctxt).flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 &&
               !val.is_null() &&
               *val.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
               xmlStrEqual((*ret).name,
                           b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) != 0 {
            xmlRngPErr(ctxt, node, XML_RNGP_XMLNS_NAME as std::os::raw::c_int,
                       b"Attribute with QName \'xmlns\' is not allowed\n\x00"
                           as *const u8 as *const std::os::raw::c_char, val,
                       0 as *const xmlChar);
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"anyName\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        (*ret).name = 0 as *mut xmlChar;
        (*ret).ns = 0 as *mut xmlChar;
        if !(*node).children.is_null() {
            (*ret).nameClass =
                xmlRelaxNGParseExceptNameClass(ctxt, (*node).children,
                                               ((*def).type_0 as std::os::raw::c_int
                                                    ==
                                                    XML_RELAXNG_ATTRIBUTE as
                                                        std::os::raw::c_int) as
                                                   std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"nsName\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        (*ret).name = 0 as *mut xmlChar;
        (*ret).ns =
            xmlGetProp(node as *const xmlNode,
                       b"ns\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        if (*ret).ns.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_NSNAME_NO_NS as std::os::raw::c_int,
                       b"nsName has no ns attribute\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
        if (*ctxt).flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 &&
               !(*ret).ns.is_null() &&
               xmlStrEqual((*ret).ns,
                           b"http://www.w3.org/2000/xmlns\x00" as *const u8 as
                               *const std::os::raw::c_char as *mut xmlChar) != 0 {
            xmlRngPErr(ctxt, node, XML_RNGP_XML_NS as std::os::raw::c_int,
                       b"Attribute with namespace \'%s\' is not allowed\n\x00"
                           as *const u8 as *const std::os::raw::c_char, (*ret).ns,
                       0 as *const xmlChar);
        }
        if !(*node).children.is_null() {
            (*ret).nameClass =
                xmlRelaxNGParseExceptNameClass(ctxt, (*node).children,
                                               ((*def).type_0 as std::os::raw::c_int
                                                    ==
                                                    XML_RELAXNG_ATTRIBUTE as
                                                        std::os::raw::c_int) as
                                                   std::os::raw::c_int)
        }
    } else if !node.is_null() && !(*node).ns.is_null() &&
                  (*node).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*node).name,
                              b"choice\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        let mut child: xmlNodePtr = 0 as *mut xmlNode;
        let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
        ret = xmlRelaxNGNewDefine(ctxt, node);
        if ret.is_null() { return 0 as xmlRelaxNGDefinePtr }
        (*ret).parent = def;
        (*ret).type_0 = XML_RELAXNG_CHOICE;
        if (*node).children.is_null() {
            xmlRngPErr(ctxt, node, XML_RNGP_CHOICE_EMPTY as std::os::raw::c_int,
                       b"Element choice is empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        } else {
            child = (*node).children;
            while !child.is_null() {
                tmp = xmlRelaxNGParseNameClass(ctxt, child, ret);
                if !tmp.is_null() {
                    if last.is_null() {
                        (*ret).nameClass = tmp;
                        last = (*ret).nameClass
                    } else { (*last).next = tmp; last = tmp }
                }
                child = (*child).next
            }
        }
    } else {
        xmlRngPErr(ctxt, node, XML_RNGP_CHOICE_CONTENT as std::os::raw::c_int,
                   b"expecting name, anyName, nsName or choice : got %s\n\x00"
                       as *const u8 as *const std::os::raw::c_char,
                   if node.is_null() {
                       b"nothing\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar
                   } else { (*node).name }, 0 as *const xmlChar);
        return 0 as xmlRelaxNGDefinePtr
    }
    if ret != def {
        if (*def).nameClass.is_null() {
            (*def).nameClass = ret
        } else {
            tmp = (*def).nameClass;
            while !(*tmp).next.is_null() { tmp = (*tmp).next }
            (*tmp).next = ret
        }
    }
    return ret;
}
/* *
 * xmlRelaxNGParseElement:
 * @ctxt:  a Relax-NG parser context
 * @node:  the element node
 *
 * parse the content of a RelaxNG element node.
 *
 * Returns the definition pointer or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGParseElement(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                            mut node: xmlNodePtr)
 -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut last: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut olddefine: *const xmlChar = 0 as *const xmlChar;
    ret = xmlRelaxNGNewDefine(ctxt, node);
    if ret.is_null() { return 0 as xmlRelaxNGDefinePtr }
    (*ret).type_0 = XML_RELAXNG_ELEMENT;
    (*ret).parent = (*ctxt).def;
    child = (*node).children;
    if child.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_ELEMENT_EMPTY as std::os::raw::c_int,
                   b"xmlRelaxNGParseElement: element has no children\n\x00" as
                       *const u8 as *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return ret
    }
    cur = xmlRelaxNGParseNameClass(ctxt, child, ret);
    if !cur.is_null() { child = (*child).next }
    if child.is_null() {
        xmlRngPErr(ctxt, node, XML_RNGP_ELEMENT_NO_CONTENT as std::os::raw::c_int,
                   b"xmlRelaxNGParseElement: element has no content\n\x00" as
                       *const u8 as *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return ret
    }
    olddefine = (*ctxt).define;
    (*ctxt).define = 0 as *const xmlChar;
    last = 0 as xmlRelaxNGDefinePtr;
    while !child.is_null() {
        cur = xmlRelaxNGParsePattern(ctxt, child);
        if !cur.is_null() {
            (*cur).parent = ret;
            match (*cur).type_0 as std::os::raw::c_int {
                0 | 1 | 3 | 4 | 5 | 7 | 8 | 11 | 13 | 12 | 10 | 15 | 16 | 14 |
                17 | 18 | 19 => {
                    if last.is_null() {
                        last = cur;
                        (*ret).content = last
                    } else {
                        if (*last).type_0 as std::os::raw::c_int ==
                               XML_RELAXNG_ELEMENT as std::os::raw::c_int &&
                               (*ret).content == last {
                            (*ret).content = xmlRelaxNGNewDefine(ctxt, node);
                            if !(*ret).content.is_null() {
                                (*(*ret).content).type_0 = XML_RELAXNG_GROUP;
                                (*(*ret).content).content = last
                            } else { (*ret).content = last }
                        }
                        (*last).next = cur;
                        last = cur
                    }
                }
                9 => { (*cur).next = (*ret).attrs; (*ret).attrs = cur }
                20 => {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_ELEMENT_CONTENT as std::os::raw::c_int,
                               b"RNG Internal error, start found in element\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
                6 => {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_ELEMENT_CONTENT as std::os::raw::c_int,
                               b"RNG Internal error, param found in element\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
                2 => {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_ELEMENT_CONTENT as std::os::raw::c_int,
                               b"RNG Internal error, except found in element\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
                -1 => {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_ELEMENT_CONTENT as std::os::raw::c_int,
                               b"RNG Internal error, noop found in element\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
                _ => { }
            }
        }
        child = (*child).next
    }
    (*ctxt).define = olddefine;
    return ret;
}
/* *
 * xmlRelaxNGParsePatterns:
 * @ctxt:  a Relax-NG parser context
 * @nodes:  list of nodes
 * @group:  use an implicit <group> for elements
 *
 * parse the content of a RelaxNG start node.
 *
 * Returns the definition pointer or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGParsePatterns(mut ctxt:
                                                 xmlRelaxNGParserCtxtPtr,
                                             mut nodes: xmlNodePtr,
                                             mut group: std::os::raw::c_int)
 -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut parent: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    parent = (*ctxt).def;
    while !nodes.is_null() {
        if !nodes.is_null() && !(*nodes).ns.is_null() &&
               (*nodes).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*nodes).name,
                           b"element\x00" as *const u8 as *const std::os::raw::c_char
                               as *const xmlChar) != 0 &&
               xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0 {
            cur = xmlRelaxNGParseElement(ctxt, nodes);
            if def.is_null() {
                last = cur;
                def = last
            } else {
                if group == 1 as std::os::raw::c_int &&
                       (*def).type_0 as std::os::raw::c_int ==
                           XML_RELAXNG_ELEMENT as std::os::raw::c_int && def == last {
                    def = xmlRelaxNGNewDefine(ctxt, nodes);
                    (*def).type_0 = XML_RELAXNG_GROUP;
                    (*def).content = last
                }
                (*last).next = cur;
                last = cur
            }
            (*cur).parent = parent
        } else {
            cur = xmlRelaxNGParsePattern(ctxt, nodes);
            if !cur.is_null() {
                if def.is_null() {
                    last = cur;
                    def = last
                } else { (*last).next = cur; last = cur }
            }
        }
        nodes = (*nodes).next
    }
    return def;
}
/* *
 * xmlRelaxNGParseStart:
 * @ctxt:  a Relax-NG parser context
 * @nodes:  start children nodes
 *
 * parse the content of a RelaxNG start node.
 *
 * Returns 0 in case of success, -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseStart(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                          mut nodes: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut last: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if nodes.is_null() {
        xmlRngPErr(ctxt, nodes, XML_RNGP_START_EMPTY as std::os::raw::c_int,
                   b"start has no children\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    if !nodes.is_null() && !(*nodes).ns.is_null() &&
           (*nodes).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*nodes).name,
                       b"empty\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 &&
           xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, nodes);
        if def.is_null() { return -(1 as std::os::raw::c_int) }
        (*def).type_0 = XML_RELAXNG_EMPTY;
        if !(*nodes).children.is_null() {
            xmlRngPErr(ctxt, nodes, XML_RNGP_EMPTY_CONTENT as std::os::raw::c_int,
                       b"element empty is not empty\n\x00" as *const u8 as
                           *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
    } else if !nodes.is_null() && !(*nodes).ns.is_null() &&
                  (*nodes).type_0 as std::os::raw::c_uint ==
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                  xmlStrEqual((*nodes).name,
                              b"notAllowed\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar) != 0
                  && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0 {
        def = xmlRelaxNGNewDefine(ctxt, nodes);
        if def.is_null() { return -(1 as std::os::raw::c_int) }
        (*def).type_0 = XML_RELAXNG_NOT_ALLOWED;
        if !(*nodes).children.is_null() {
            xmlRngPErr(ctxt, nodes,
                       XML_RNGP_NOTALLOWED_NOT_EMPTY as std::os::raw::c_int,
                       b"element notAllowed is not empty\n\x00" as *const u8
                           as *const std::os::raw::c_char, 0 as *const xmlChar,
                       0 as *const xmlChar);
        }
    } else { def = xmlRelaxNGParsePatterns(ctxt, nodes, 1 as std::os::raw::c_int) }
    if !(*(*ctxt).grammar).start.is_null() {
        last = (*(*ctxt).grammar).start;
        while !(*last).next.is_null() { last = (*last).next }
        (*last).next = def
    } else { (*(*ctxt).grammar).start = def }
    nodes = (*nodes).next;
    if !nodes.is_null() {
        xmlRngPErr(ctxt, nodes, XML_RNGP_START_CONTENT as std::os::raw::c_int,
                   b"start more than one children\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlRelaxNGParseGrammarContent:
 * @ctxt:  a Relax-NG parser context
 * @nodes:  grammar children nodes
 *
 * parse the content of a RelaxNG grammar node.
 *
 * Returns 0 in case of success, -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseGrammarContent(mut ctxt:
                                                       xmlRelaxNGParserCtxtPtr,
                                                   mut nodes: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tmp: std::os::raw::c_int = 0;
    if nodes.is_null() {
        xmlRngPErr(ctxt, nodes, XML_RNGP_GRAMMAR_EMPTY as std::os::raw::c_int,
                   b"grammar has no children\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return -(1 as std::os::raw::c_int)
    }
    while !nodes.is_null() {
        if !nodes.is_null() && !(*nodes).ns.is_null() &&
               (*nodes).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               xmlStrEqual((*nodes).name,
                           b"start\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar) != 0 &&
               xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0 {
            if (*nodes).children.is_null() {
                xmlRngPErr(ctxt, nodes, XML_RNGP_START_EMPTY as std::os::raw::c_int,
                           b"start has no children\n\x00" as *const u8 as
                               *const std::os::raw::c_char, 0 as *const xmlChar,
                           0 as *const xmlChar);
            } else {
                tmp = xmlRelaxNGParseStart(ctxt, (*nodes).children);
                if tmp != 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
            }
        } else if !nodes.is_null() && !(*nodes).ns.is_null() &&
                      (*nodes).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      xmlStrEqual((*nodes).name,
                                  b"define\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0 {
            tmp = xmlRelaxNGParseDefine(ctxt, nodes);
            if tmp != 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
        } else if !nodes.is_null() && !(*nodes).ns.is_null() &&
                      (*nodes).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      xmlStrEqual((*nodes).name,
                                  b"include\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0 {
            tmp = xmlRelaxNGParseInclude(ctxt, nodes);
            if tmp != 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
        } else {
            xmlRngPErr(ctxt, nodes, XML_RNGP_GRAMMAR_CONTENT as std::os::raw::c_int,
                       b"grammar has unexpected child %s\n\x00" as *const u8
                           as *const std::os::raw::c_char, (*nodes).name,
                       0 as *const xmlChar);
            ret = -(1 as std::os::raw::c_int)
        }
        nodes = (*nodes).next
    }
    return ret;
}
/* *
 * xmlRelaxNGCheckReference:
 * @ref:  the ref
 * @ctxt:  a Relax-NG parser context
 * @name:  the name associated to the defines
 *
 * Applies the 4.17. combine attribute rule for all the define
 * element of a given grammar using the same name.
 */
unsafe extern "C" fn xmlRelaxNGCheckReference(mut payload: *mut std::os::raw::c_void,
                                              mut data: *mut std::os::raw::c_void,
                                              mut name: *const xmlChar) {
    let mut ref_0: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    /*
     * Those rules don't apply to imported ref from xmlRelaxNGParseImportRef
     */
    if (*ref_0).dflags as std::os::raw::c_int & (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int
           != 0 {
        return
    }
    grammar = (*ctxt).grammar;
    if grammar.is_null() {
        xmlRngPErr(ctxt, (*ref_0).node, XML_ERR_INTERNAL_ERROR as std::os::raw::c_int,
                   b"Internal error: no grammar in CheckReference %s\n\x00" as
                       *const u8 as *const std::os::raw::c_char, name,
                   0 as *const xmlChar);
        return
    }
    if !(*ref_0).content.is_null() {
        xmlRngPErr(ctxt, (*ref_0).node, XML_ERR_INTERNAL_ERROR as std::os::raw::c_int,
                   b"Internal error: reference has content in CheckReference %s\n\x00"
                       as *const u8 as *const std::os::raw::c_char, name,
                   0 as *const xmlChar);
        return
    }
    if !(*grammar).defs.is_null() {
        def = xmlHashLookup((*grammar).defs, name) as xmlRelaxNGDefinePtr;
        if !def.is_null() {
            cur = ref_0;
            while !cur.is_null() {
                (*cur).content = def;
                cur = (*cur).nextHash
            }
        } else {
            xmlRngPErr(ctxt, (*ref_0).node,
                       XML_RNGP_REF_NO_DEF as std::os::raw::c_int,
                       b"Reference %s has no matching definition\n\x00" as
                           *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
        }
    } else {
        xmlRngPErr(ctxt, (*ref_0).node, XML_RNGP_REF_NO_DEF as std::os::raw::c_int,
                   b"Reference %s has no matching definition\n\x00" as
                       *const u8 as *const std::os::raw::c_char, name,
                   0 as *const xmlChar);
    };
}
/* *
 * xmlRelaxNGCheckCombine:
 * @define:  the define(s) list
 * @ctxt:  a Relax-NG parser context
 * @name:  the name associated to the defines
 *
 * Applies the 4.17. combine attribute rule for all the define
 * element of a given grammar using the same name.
 */
unsafe extern "C" fn xmlRelaxNGCheckCombine(mut payload: *mut std::os::raw::c_void,
                                            mut data: *mut std::os::raw::c_void,
                                            mut name: *const xmlChar) {
    let mut define: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut combine: *mut xmlChar = 0 as *mut xmlChar;
    let mut choiceOrInterleave: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut missing: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut last: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp2: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if (*define).nextHash.is_null() { return }
    cur = define;
    while !cur.is_null() {
        combine =
            xmlGetProp((*cur).node as *const xmlNode,
                       b"combine\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
        if !combine.is_null() {
            if xmlStrEqual(combine,
                           b"choice\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 {
                if choiceOrInterleave == -(1 as std::os::raw::c_int) {
                    choiceOrInterleave = 1 as std::os::raw::c_int
                } else if choiceOrInterleave == 0 as std::os::raw::c_int {
                    xmlRngPErr(ctxt, (*define).node,
                               XML_RNGP_DEF_CHOICE_AND_INTERLEAVE as
                                   std::os::raw::c_int,
                               b"Defines for %s use both \'choice\' and \'interleave\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, name,
                               0 as *const xmlChar);
                }
            } else if xmlStrEqual(combine,
                                  b"interleave\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                if choiceOrInterleave == -(1 as std::os::raw::c_int) {
                    choiceOrInterleave = 0 as std::os::raw::c_int
                } else if choiceOrInterleave == 1 as std::os::raw::c_int {
                    xmlRngPErr(ctxt, (*define).node,
                               XML_RNGP_DEF_CHOICE_AND_INTERLEAVE as
                                   std::os::raw::c_int,
                               b"Defines for %s use both \'choice\' and \'interleave\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, name,
                               0 as *const xmlChar);
                }
            } else {
                xmlRngPErr(ctxt, (*define).node,
                           XML_RNGP_UNKNOWN_COMBINE as std::os::raw::c_int,
                           b"Defines for %s use unknown combine value \'%s\'\'\n\x00"
                               as *const u8 as *const std::os::raw::c_char, name,
                           combine);
            }
            xmlFree.expect("non-null function pointer")(combine as
                                                            *mut std::os::raw::c_void);
        } else if missing == 0 as std::os::raw::c_int {
            missing = 1 as std::os::raw::c_int
        } else {
            xmlRngPErr(ctxt, (*define).node,
                       XML_RNGP_NEED_COMBINE as std::os::raw::c_int,
                       b"Some defines for %s needs the combine attribute\n\x00"
                           as *const u8 as *const std::os::raw::c_char, name,
                       0 as *const xmlChar);
        }
        cur = (*cur).nextHash
    }
    if choiceOrInterleave == -(1 as std::os::raw::c_int) {
        choiceOrInterleave = 0 as std::os::raw::c_int
    }
    cur = xmlRelaxNGNewDefine(ctxt, (*define).node);
    if cur.is_null() { return }
    if choiceOrInterleave == 0 as std::os::raw::c_int {
        (*cur).type_0 = XML_RELAXNG_INTERLEAVE
    } else { (*cur).type_0 = XML_RELAXNG_CHOICE }
    tmp = define;
    last = 0 as xmlRelaxNGDefinePtr;
    while !tmp.is_null() {
        if !(*tmp).content.is_null() {
            if !(*(*tmp).content).next.is_null() {
                /*
                 * we need first to create a wrapper.
                 */
                tmp2 = xmlRelaxNGNewDefine(ctxt, (*(*tmp).content).node);
                if tmp2.is_null() { break ; }
                (*tmp2).type_0 = XML_RELAXNG_GROUP;
                (*tmp2).content = (*tmp).content
            } else { tmp2 = (*tmp).content }
            if last.is_null() {
                (*cur).content = tmp2
            } else { (*last).next = tmp2 }
            last = tmp2
        }
        (*tmp).content = cur;
        tmp = (*tmp).nextHash
    }
    (*define).content = cur;
    if choiceOrInterleave == 0 as std::os::raw::c_int {
        if (*ctxt).interleaves.is_null() {
            (*ctxt).interleaves = xmlHashCreate(10 as std::os::raw::c_int)
        }
        if (*ctxt).interleaves.is_null() {
            xmlRngPErr(ctxt, (*define).node,
                       XML_RNGP_INTERLEAVE_CREATE_FAILED as std::os::raw::c_int,
                       b"Failed to create interleaves hash table\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
        } else {
            let mut tmpname: [std::os::raw::c_char; 32] = [0; 32];
            let fresh29 = (*ctxt).nbInterleaves;
            (*ctxt).nbInterleaves = (*ctxt).nbInterleaves + 1;
            snprintf(tmpname.as_mut_ptr(), 32 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"interleave%d\x00" as *const u8 as *const std::os::raw::c_char,
                     fresh29);
            if xmlHashAddEntry((*ctxt).interleaves,
                               tmpname.as_mut_ptr() as *mut xmlChar,
                               cur as *mut std::os::raw::c_void) < 0 as std::os::raw::c_int {
                xmlRngPErr(ctxt, (*define).node,
                           XML_RNGP_INTERLEAVE_CREATE_FAILED as std::os::raw::c_int,
                           b"Failed to add %s to hash table\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           tmpname.as_mut_ptr() as *const xmlChar,
                           0 as *const xmlChar);
            }
        }
    };
}
/* *
 * xmlRelaxNGCombineStart:
 * @ctxt:  a Relax-NG parser context
 * @grammar:  the grammar
 *
 * Applies the 4.17. combine rule for all the start
 * element of a given grammar.
 */
unsafe extern "C" fn xmlRelaxNGCombineStart(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                            mut grammar:
                                                xmlRelaxNGGrammarPtr) {
    let mut starts: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut combine: *mut xmlChar = 0 as *mut xmlChar;
    let mut choiceOrInterleave: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut missing: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    starts = (*grammar).start;
    if starts.is_null() || (*starts).next.is_null() { return }
    cur = starts;
    while !cur.is_null() {
        if (*cur).node.is_null() || (*(*cur).node).parent.is_null() ||
               xmlStrEqual((*(*(*cur).node).parent).name,
                           b"start\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) == 0 {
            combine = 0 as *mut xmlChar;
            xmlRngPErr(ctxt, (*cur).node,
                       XML_RNGP_START_MISSING as std::os::raw::c_int,
                       b"Internal error: start element not found\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
        } else {
            combine =
                xmlGetProp((*(*cur).node).parent,
                           b"combine\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar)
        }
        if !combine.is_null() {
            if xmlStrEqual(combine,
                           b"choice\x00" as *const u8 as *const std::os::raw::c_char
                               as *mut xmlChar) != 0 {
                if choiceOrInterleave == -(1 as std::os::raw::c_int) {
                    choiceOrInterleave = 1 as std::os::raw::c_int
                } else if choiceOrInterleave == 0 as std::os::raw::c_int {
                    xmlRngPErr(ctxt, (*cur).node,
                               XML_RNGP_START_CHOICE_AND_INTERLEAVE as
                                   std::os::raw::c_int,
                               b"<start> use both \'choice\' and \'interleave\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
            } else if xmlStrEqual(combine,
                                  b"interleave\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                if choiceOrInterleave == -(1 as std::os::raw::c_int) {
                    choiceOrInterleave = 0 as std::os::raw::c_int
                } else if choiceOrInterleave == 1 as std::os::raw::c_int {
                    xmlRngPErr(ctxt, (*cur).node,
                               XML_RNGP_START_CHOICE_AND_INTERLEAVE as
                                   std::os::raw::c_int,
                               b"<start> use both \'choice\' and \'interleave\'\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
            } else {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_UNKNOWN_COMBINE as std::os::raw::c_int,
                           b"<start> uses unknown combine value \'%s\'\'\n\x00"
                               as *const u8 as *const std::os::raw::c_char, combine,
                           0 as *const xmlChar);
            }
            xmlFree.expect("non-null function pointer")(combine as
                                                            *mut std::os::raw::c_void);
        } else if missing == 0 as std::os::raw::c_int {
            missing = 1 as std::os::raw::c_int
        } else {
            xmlRngPErr(ctxt, (*cur).node,
                       XML_RNGP_NEED_COMBINE as std::os::raw::c_int,
                       b"Some <start> element miss the combine attribute\n\x00"
                           as *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
        }
        cur = (*cur).next
    }
    if choiceOrInterleave == -(1 as std::os::raw::c_int) {
        choiceOrInterleave = 0 as std::os::raw::c_int
    }
    cur = xmlRelaxNGNewDefine(ctxt, (*starts).node);
    if cur.is_null() { return }
    if choiceOrInterleave == 0 as std::os::raw::c_int {
        (*cur).type_0 = XML_RELAXNG_INTERLEAVE
    } else { (*cur).type_0 = XML_RELAXNG_CHOICE }
    (*cur).content = (*grammar).start;
    (*grammar).start = cur;
    if choiceOrInterleave == 0 as std::os::raw::c_int {
        if (*ctxt).interleaves.is_null() {
            (*ctxt).interleaves = xmlHashCreate(10 as std::os::raw::c_int)
        }
        if (*ctxt).interleaves.is_null() {
            xmlRngPErr(ctxt, (*cur).node,
                       XML_RNGP_INTERLEAVE_CREATE_FAILED as std::os::raw::c_int,
                       b"Failed to create interleaves hash table\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
        } else {
            let mut tmpname: [std::os::raw::c_char; 32] = [0; 32];
            let fresh30 = (*ctxt).nbInterleaves;
            (*ctxt).nbInterleaves = (*ctxt).nbInterleaves + 1;
            snprintf(tmpname.as_mut_ptr(), 32 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"interleave%d\x00" as *const u8 as *const std::os::raw::c_char,
                     fresh30);
            if xmlHashAddEntry((*ctxt).interleaves,
                               tmpname.as_mut_ptr() as *mut xmlChar,
                               cur as *mut std::os::raw::c_void) < 0 as std::os::raw::c_int {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_INTERLEAVE_CREATE_FAILED as std::os::raw::c_int,
                           b"Failed to add %s to hash table\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           tmpname.as_mut_ptr() as *const xmlChar,
                           0 as *const xmlChar);
            }
        }
    };
}
/* *
 * xmlRelaxNGCheckCycles:
 * @ctxt:  a Relax-NG parser context
 * @nodes:  grammar children nodes
 * @depth:  the counter
 *
 * Check for cycles.
 *
 * Returns 0 if check passed, and -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGCheckCycles(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                           mut cur: xmlRelaxNGDefinePtr,
                                           mut depth: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while ret == 0 as std::os::raw::c_int && !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_REF as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_PARENTREF as std::os::raw::c_int {
            if (*cur).depth as std::os::raw::c_int == -(1 as std::os::raw::c_int) {
                (*cur).depth = depth as std::os::raw::c_short;
                ret = xmlRelaxNGCheckCycles(ctxt, (*cur).content, depth);
                (*cur).depth = -(2 as std::os::raw::c_int) as std::os::raw::c_short
            } else if depth == (*cur).depth as std::os::raw::c_int {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_REF_CYCLE as std::os::raw::c_int,
                           b"Detected a cycle in %s references\n\x00" as
                               *const u8 as *const std::os::raw::c_char, (*cur).name,
                           0 as *const xmlChar);
                return -(1 as std::os::raw::c_int)
            }
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_ELEMENT as std::os::raw::c_int {
            ret =
                xmlRelaxNGCheckCycles(ctxt, (*cur).content,
                                      depth + 1 as std::os::raw::c_int)
        } else { ret = xmlRelaxNGCheckCycles(ctxt, (*cur).content, depth) }
        cur = (*cur).next
    }
    return ret;
}
/* *
 * xmlRelaxNGTryUnlink:
 * @ctxt:  a Relax-NG parser context
 * @cur:  the definition to unlink
 * @parent:  the parent definition
 * @prev:  the previous sibling definition
 *
 * Try to unlink a definition. If not possble make it a NOOP
 *
 * Returns the new prev definition
 */
unsafe extern "C" fn xmlRelaxNGTryUnlink(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                         mut cur: xmlRelaxNGDefinePtr,
                                         mut parent: xmlRelaxNGDefinePtr,
                                         mut prev: xmlRelaxNGDefinePtr)
 -> xmlRelaxNGDefinePtr {
    if !prev.is_null() {
        (*prev).next = (*cur).next
    } else if !parent.is_null() {
        if (*parent).content == cur {
            (*parent).content = (*cur).next
        } else if (*parent).attrs == cur {
            (*parent).attrs = (*cur).next
        } else if (*parent).nameClass == cur {
            (*parent).nameClass = (*cur).next
        }
    } else { (*cur).type_0 = XML_RELAXNG_NOOP; prev = cur }
    return prev;
}
/* *
 * xmlRelaxNGSimplify:
 * @ctxt:  a Relax-NG parser context
 * @nodes:  grammar children nodes
 *
 * Check for simplification of empty and notAllowed
 */
unsafe extern "C" fn xmlRelaxNGSimplify(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                        mut cur: xmlRelaxNGDefinePtr,
                                        mut parent: xmlRelaxNGDefinePtr) {
    let mut prev: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_REF as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_PARENTREF as std::os::raw::c_int {
            if (*cur).depth as std::os::raw::c_int != -(3 as std::os::raw::c_int) {
                (*cur).depth = -(3 as std::os::raw::c_int) as std::os::raw::c_short;
                xmlRelaxNGSimplify(ctxt, (*cur).content, cur);
            }
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_NOT_ALLOWED as std::os::raw::c_int {
            (*cur).parent = parent;
            if !parent.is_null() &&
                   ((*parent).type_0 as std::os::raw::c_int ==
                        XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int ||
                        (*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_LIST as std::os::raw::c_int ||
                        (*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_GROUP as std::os::raw::c_int ||
                        (*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_INTERLEAVE as std::os::raw::c_int ||
                        (*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
                        (*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_ZEROORMORE as std::os::raw::c_int) {
                (*parent).type_0 = XML_RELAXNG_NOT_ALLOWED;
                break ;
            } else if !parent.is_null() &&
                          (*parent).type_0 as std::os::raw::c_int ==
                              XML_RELAXNG_CHOICE as std::os::raw::c_int {
                prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev)
            } else { prev = cur }
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_EMPTY as std::os::raw::c_int {
            (*cur).parent = parent;
            if !parent.is_null() &&
                   ((*parent).type_0 as std::os::raw::c_int ==
                        XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
                        (*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_ZEROORMORE as std::os::raw::c_int) {
                (*parent).type_0 = XML_RELAXNG_EMPTY;
                break ;
            } else if !parent.is_null() &&
                          ((*parent).type_0 as std::os::raw::c_int ==
                               XML_RELAXNG_GROUP as std::os::raw::c_int ||
                               (*parent).type_0 as std::os::raw::c_int ==
                                   XML_RELAXNG_INTERLEAVE as std::os::raw::c_int) {
                prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev)
            } else { prev = cur }
        } else {
            (*cur).parent = parent;
            if !(*cur).content.is_null() {
                xmlRelaxNGSimplify(ctxt, (*cur).content, cur);
            }
            if (*cur).type_0 as std::os::raw::c_int !=
                   XML_RELAXNG_VALUE as std::os::raw::c_int && !(*cur).attrs.is_null()
               {
                xmlRelaxNGSimplify(ctxt, (*cur).attrs, cur);
            }
            if !(*cur).nameClass.is_null() {
                xmlRelaxNGSimplify(ctxt, (*cur).nameClass, cur);
            }
            /*
             * On Elements, try to move attribute only generating rules on
             * the attrs rules.
             */
            if (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_ELEMENT as std::os::raw::c_int {
                let mut attronly: std::os::raw::c_int = 0;
                let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                let mut pre: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                while !(*cur).content.is_null() {
                    attronly =
                        xmlRelaxNGGenerateAttributes(ctxt, (*cur).content);
                    if !(attronly == 1 as std::os::raw::c_int) { break ; }
                    /*
                         * migrate cur->content to attrs
                         */
                    tmp = (*cur).content;
                    (*cur).content = (*tmp).next;
                    (*tmp).next = (*cur).attrs;
                    (*cur).attrs = tmp
                }
                pre = (*cur).content;
                while !pre.is_null() && !(*pre).next.is_null() {
                    tmp = (*pre).next;
                    attronly = xmlRelaxNGGenerateAttributes(ctxt, tmp);
                    if attronly == 1 as std::os::raw::c_int {
                        /*
                         * migrate tmp to attrs
                         */
                        (*pre).next = (*tmp).next;
                        (*tmp).next = (*cur).attrs;
                        (*cur).attrs = tmp
                    } else { pre = tmp }
                }
            }
            /*
             * This may result in a simplification
             */
            if (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_GROUP as std::os::raw::c_int ||
                   (*cur).type_0 as std::os::raw::c_int ==
                       XML_RELAXNG_INTERLEAVE as std::os::raw::c_int {
                if (*cur).content.is_null() {
                    (*cur).type_0 = XML_RELAXNG_EMPTY
                } else if (*(*cur).content).next.is_null() {
                    if parent.is_null() && prev.is_null() {
                        (*cur).type_0 = XML_RELAXNG_NOOP
                    } else if prev.is_null() {
                        (*parent).content = (*cur).content;
                        (*(*cur).content).next = (*cur).next;
                        cur = (*cur).content
                    } else {
                        (*(*cur).content).next = (*cur).next;
                        (*prev).next = (*cur).content;
                        cur = (*cur).content
                    }
                }
            }
            /*
             * the current node may have been transformed back
             */
            if (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_EXCEPT as std::os::raw::c_int &&
                   !(*cur).content.is_null() &&
                   (*(*cur).content).type_0 as std::os::raw::c_int ==
                       XML_RELAXNG_NOT_ALLOWED as std::os::raw::c_int {
                prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev)
            } else if (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_NOT_ALLOWED as std::os::raw::c_int {
                if !parent.is_null() &&
                       ((*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int ||
                            (*parent).type_0 as std::os::raw::c_int ==
                                XML_RELAXNG_LIST as std::os::raw::c_int ||
                            (*parent).type_0 as std::os::raw::c_int ==
                                XML_RELAXNG_GROUP as std::os::raw::c_int ||
                            (*parent).type_0 as std::os::raw::c_int ==
                                XML_RELAXNG_INTERLEAVE as std::os::raw::c_int ||
                            (*parent).type_0 as std::os::raw::c_int ==
                                XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
                            (*parent).type_0 as std::os::raw::c_int ==
                                XML_RELAXNG_ZEROORMORE as std::os::raw::c_int) {
                    (*parent).type_0 = XML_RELAXNG_NOT_ALLOWED;
                    break ;
                } else if !parent.is_null() &&
                              (*parent).type_0 as std::os::raw::c_int ==
                                  XML_RELAXNG_CHOICE as std::os::raw::c_int {
                    prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev)
                } else { prev = cur }
            } else if (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_EMPTY as std::os::raw::c_int {
                if !parent.is_null() &&
                       ((*parent).type_0 as std::os::raw::c_int ==
                            XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
                            (*parent).type_0 as std::os::raw::c_int ==
                                XML_RELAXNG_ZEROORMORE as std::os::raw::c_int) {
                    (*parent).type_0 = XML_RELAXNG_EMPTY;
                    break ;
                } else if !parent.is_null() &&
                              ((*parent).type_0 as std::os::raw::c_int ==
                                   XML_RELAXNG_GROUP as std::os::raw::c_int ||
                                   (*parent).type_0 as std::os::raw::c_int ==
                                       XML_RELAXNG_INTERLEAVE as std::os::raw::c_int
                                   ||
                                   (*parent).type_0 as std::os::raw::c_int ==
                                       XML_RELAXNG_CHOICE as std::os::raw::c_int) {
                    prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev)
                } else { prev = cur }
            } else { prev = cur }
        }
        cur = (*cur).next
    };
}
/* *
 * xmlRelaxNGGroupContentType:
 * @ct1:  the first content type
 * @ct2:  the second content type
 *
 * Try to group 2 content types
 *
 * Returns the content type
 */
unsafe extern "C" fn xmlRelaxNGGroupContentType(mut ct1:
                                                    xmlRelaxNGContentType,
                                                mut ct2:
                                                    xmlRelaxNGContentType)
 -> xmlRelaxNGContentType {
    if ct1 as std::os::raw::c_int == XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int ||
           ct2 as std::os::raw::c_int == XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int {
        return XML_RELAXNG_CONTENT_ERROR
    }
    if ct1 as std::os::raw::c_int == XML_RELAXNG_CONTENT_EMPTY as std::os::raw::c_int {
        return ct2
    }
    if ct2 as std::os::raw::c_int == XML_RELAXNG_CONTENT_EMPTY as std::os::raw::c_int {
        return ct1
    }
    if ct1 as std::os::raw::c_int == XML_RELAXNG_CONTENT_COMPLEX as std::os::raw::c_int &&
           ct2 as std::os::raw::c_int == XML_RELAXNG_CONTENT_COMPLEX as std::os::raw::c_int {
        return XML_RELAXNG_CONTENT_COMPLEX
    }
    return XML_RELAXNG_CONTENT_ERROR;
}
/* *
 * xmlRelaxNGMaxContentType:
 * @ct1:  the first content type
 * @ct2:  the second content type
 *
 * Compute the max content-type
 *
 * Returns the content type
 */
unsafe extern "C" fn xmlRelaxNGMaxContentType(mut ct1: xmlRelaxNGContentType,
                                              mut ct2: xmlRelaxNGContentType)
 -> xmlRelaxNGContentType {
    if ct1 as std::os::raw::c_int == XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int ||
           ct2 as std::os::raw::c_int == XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int {
        return XML_RELAXNG_CONTENT_ERROR
    }
    if ct1 as std::os::raw::c_int == XML_RELAXNG_CONTENT_SIMPLE as std::os::raw::c_int ||
           ct2 as std::os::raw::c_int == XML_RELAXNG_CONTENT_SIMPLE as std::os::raw::c_int {
        return XML_RELAXNG_CONTENT_SIMPLE
    }
    if ct1 as std::os::raw::c_int == XML_RELAXNG_CONTENT_COMPLEX as std::os::raw::c_int ||
           ct2 as std::os::raw::c_int == XML_RELAXNG_CONTENT_COMPLEX as std::os::raw::c_int {
        return XML_RELAXNG_CONTENT_COMPLEX
    }
    return XML_RELAXNG_CONTENT_EMPTY;
}
/* *
 * xmlRelaxNGCheckRules:
 * @ctxt:  a Relax-NG parser context
 * @cur:  the current definition
 * @flags:  some accumulated flags
 * @ptype:  the parent type
 *
 * Check for rules in section 7.1 and 7.2
 *
 * Returns the content type of @cur
 */
unsafe extern "C" fn xmlRelaxNGCheckRules(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                          mut cur: xmlRelaxNGDefinePtr,
                                          mut flags: std::os::raw::c_int,
                                          mut ptype: xmlRelaxNGType)
 -> xmlRelaxNGContentType {
    let mut nflags: std::os::raw::c_int = 0;
    let mut ret: xmlRelaxNGContentType = XML_RELAXNG_CONTENT_EMPTY;
    let mut tmp: xmlRelaxNGContentType = XML_RELAXNG_CONTENT_EMPTY;
    let mut val: xmlRelaxNGContentType = XML_RELAXNG_CONTENT_EMPTY;
    while !cur.is_null() {
        ret = XML_RELAXNG_CONTENT_EMPTY;
        if (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_REF as std::os::raw::c_int ||
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_PARENTREF as std::os::raw::c_int {
            /*
            * This should actually be caught by list//element(ref) at the
            * element boundaries, c.f. Bug #159968 local refs are dropped
            * in step 4.19.
            */
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_REF as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//ref\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if (*cur).content.is_null() {
                if (*cur).type_0 as std::os::raw::c_int ==
                       XML_RELAXNG_PARENTREF as std::os::raw::c_int {
                    xmlRngPErr(ctxt, (*cur).node,
                               XML_RNGP_REF_NO_DEF as std::os::raw::c_int,
                               b"Internal found no define for parent refs\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                } else {
                    xmlRngPErr(ctxt, (*cur).node,
                               XML_RNGP_REF_NO_DEF as std::os::raw::c_int,
                               b"Internal found no define for ref %s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               if !(*cur).name.is_null() {
                                   (*cur).name
                               } else {
                                   b"null\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar
                               }, 0 as *const xmlChar);
                }
            }
            if (*cur).depth as std::os::raw::c_int > -(4 as std::os::raw::c_int) {
                (*cur).depth = -(4 as std::os::raw::c_int) as std::os::raw::c_short;
                ret =
                    xmlRelaxNGCheckRules(ctxt, (*cur).content, flags,
                                         (*cur).type_0);
                (*cur).depth =
                    (ret as std::os::raw::c_int - 15 as std::os::raw::c_int) as std::os::raw::c_short
            } else if (*cur).depth as std::os::raw::c_int == -(4 as std::os::raw::c_int) {
                ret = XML_RELAXNG_CONTENT_COMPLEX
            } else {
                ret =
                    ((*cur).depth as std::os::raw::c_int + 15 as std::os::raw::c_int) as
                        xmlRelaxNGContentType
            }
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_ELEMENT as std::os::raw::c_int {
            /*
             * The 7.3 Attribute derivation rule for groups is plugged there
             */
            xmlRelaxNGCheckGroupAttrs(ctxt, cur);
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_ELEM as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//element(ref)\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_LIST_ELEM as std::os::raw::c_int,
                           b"Found forbidden pattern list//element(ref)\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_ATTR_ELEM as std::os::raw::c_int,
                           b"Found forbidden pattern attribute//element(ref)\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_ATTR_ELEM as std::os::raw::c_int,
                           b"Found forbidden pattern attribute//element(ref)\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            /*
             * reset since in the simple form elements are only child
             * of grammar/define
             */
            nflags = 0 as std::os::raw::c_int;
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).attrs, nflags,
                                     (*cur).type_0);
            if ret as std::os::raw::c_int != XML_RELAXNG_CONTENT_EMPTY as std::os::raw::c_int
               {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_ELEM_CONTENT_EMPTY as std::os::raw::c_int,
                           b"Element %s attributes have a content type error\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           (*cur).name, 0 as *const xmlChar);
            }
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags,
                                     (*cur).type_0);
            if ret as std::os::raw::c_int == XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int
               {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_ELEM_CONTENT_ERROR as std::os::raw::c_int,
                           b"Element %s has a content type error\n\x00" as
                               *const u8 as *const std::os::raw::c_char, (*cur).name,
                           0 as *const xmlChar);
            } else { ret = XML_RELAXNG_CONTENT_COMPLEX }
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_ATTR_ATTR as std::os::raw::c_int,
                           b"Found forbidden pattern attribute//attribute\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_LIST_ATTR as std::os::raw::c_int,
                           b"Found forbidden pattern list//attribute\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 5 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_ONEMORE_GROUP_ATTR as std::os::raw::c_int,
                           b"Found forbidden pattern oneOrMore//group//attribute\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR as
                               std::os::raw::c_int,
                           b"Found forbidden pattern oneOrMore//interleave//attribute\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_ATTR as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//attribute\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_ATTR as std::os::raw::c_int,
                           b"Found forbidden pattern start//attribute\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int == 0 &&
                   (*cur).name.is_null() {
                if (*cur).ns.is_null() {
                    xmlRngPErr(ctxt, (*cur).node,
                               XML_RNGP_ANYNAME_ATTR_ANCESTOR as std::os::raw::c_int,
                               b"Found anyName attribute without oneOrMore ancestor\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                } else {
                    xmlRngPErr(ctxt, (*cur).node,
                               XML_RNGP_NSNAME_ATTR_ANCESTOR as std::os::raw::c_int,
                               b"Found nsName attribute without oneOrMore ancestor\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               0 as *const xmlChar, 0 as *const xmlChar);
                }
            }
            nflags = flags | (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int;
            xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
            ret = XML_RELAXNG_CONTENT_EMPTY
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_ONEORMORE as std::os::raw::c_int ||
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_ZEROORMORE as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_ONEMORE as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//oneOrMore\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_ONEMORE as std::os::raw::c_int,
                           b"Found forbidden pattern start//oneOrMore\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            nflags = flags | (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int;
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags,
                                     (*cur).type_0);
            ret = xmlRelaxNGGroupContentType(ret, ret)
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_LIST as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_LIST_LIST as std::os::raw::c_int,
                           b"Found forbidden pattern list//list\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_LIST as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//list\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_LIST as std::os::raw::c_int,
                           b"Found forbidden pattern start//list\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            nflags = flags | (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int;
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags,
                                     (*cur).type_0)
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_GROUP as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_GROUP as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//group\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_GROUP as std::os::raw::c_int,
                           b"Found forbidden pattern start//group\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
                nflags = flags | (1 as std::os::raw::c_int) << 5 as std::os::raw::c_int
            } else { nflags = flags }
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags,
                                     (*cur).type_0);
            /*
             * The 7.3 Attribute derivation rule for groups is plugged there
             */
            xmlRelaxNGCheckGroupAttrs(ctxt, cur);
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_INTERLEAVE as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_LIST_INTERLEAVE as std::os::raw::c_int,
                           b"Found forbidden pattern list//interleave\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//interleave\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE as std::os::raw::c_int,
                           b"Found forbidden pattern start//interleave\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
                nflags = flags | (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int
            } else { nflags = flags }
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags,
                                     (*cur).type_0)
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_EXCEPT as std::os::raw::c_int {
            if !(*cur).parent.is_null() &&
                   (*(*cur).parent).type_0 as std::os::raw::c_int ==
                       XML_RELAXNG_DATATYPE as std::os::raw::c_int {
                nflags = flags | (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int
            } else { nflags = flags }
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags,
                                     (*cur).type_0)
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_DATATYPE as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_DATA as std::os::raw::c_int,
                           b"Found forbidden pattern start//data\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
            ret = XML_RELAXNG_CONTENT_SIMPLE
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_VALUE as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_VALUE as std::os::raw::c_int,
                           b"Found forbidden pattern start//value\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
            ret = XML_RELAXNG_CONTENT_SIMPLE
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_TEXT as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_LIST_TEXT as std::os::raw::c_int,
                           b"Found forbidden pattern list//text\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_TEXT as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//text\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_TEXT as std::os::raw::c_int,
                           b"Found forbidden pattern start//text\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            ret = XML_RELAXNG_CONTENT_COMPLEX
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_EMPTY as std::os::raw::c_int {
            if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_DATA_EXCEPT_EMPTY as std::os::raw::c_int,
                           b"Found forbidden pattern data/except//empty\n\x00"
                               as *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 {
                xmlRngPErr(ctxt, (*cur).node,
                           XML_RNGP_PAT_START_EMPTY as std::os::raw::c_int,
                           b"Found forbidden pattern start//empty\n\x00" as
                               *const u8 as *const std::os::raw::c_char,
                           0 as *const xmlChar, 0 as *const xmlChar);
            }
            ret = XML_RELAXNG_CONTENT_EMPTY
        } else if (*cur).type_0 as std::os::raw::c_int ==
                      XML_RELAXNG_CHOICE as std::os::raw::c_int {
            xmlRelaxNGCheckChoiceDeterminism(ctxt, cur);
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, flags,
                                     (*cur).type_0)
        } else {
            ret =
                xmlRelaxNGCheckRules(ctxt, (*cur).content, flags,
                                     (*cur).type_0)
        }
        cur = (*cur).next;
        if ptype as std::os::raw::c_int == XML_RELAXNG_GROUP as std::os::raw::c_int {
            val = xmlRelaxNGGroupContentType(val, ret)
        } else if ptype as std::os::raw::c_int ==
                      XML_RELAXNG_INTERLEAVE as std::os::raw::c_int {
            /*
             * TODO: scan complain that tmp is never used, seems on purpose
             *       need double-checking
             */
            tmp = xmlRelaxNGGroupContentType(val, ret);
            if tmp as std::os::raw::c_int != XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int
               {
                tmp = xmlRelaxNGMaxContentType(val, ret)
            }
        } else if ptype as std::os::raw::c_int == XML_RELAXNG_CHOICE as std::os::raw::c_int {
            val = xmlRelaxNGMaxContentType(val, ret)
        } else if ptype as std::os::raw::c_int == XML_RELAXNG_LIST as std::os::raw::c_int {
            val = XML_RELAXNG_CONTENT_SIMPLE
        } else if ptype as std::os::raw::c_int == XML_RELAXNG_EXCEPT as std::os::raw::c_int {
            if ret as std::os::raw::c_int == XML_RELAXNG_CONTENT_ERROR as std::os::raw::c_int
               {
                val = XML_RELAXNG_CONTENT_ERROR
            } else { val = XML_RELAXNG_CONTENT_SIMPLE }
        } else { val = xmlRelaxNGGroupContentType(val, ret) }
    }
    return val;
}
/* *
 * xmlRelaxNGParseGrammar:
 * @ctxt:  a Relax-NG parser context
 * @nodes:  grammar children nodes
 *
 * parse a Relax-NG <grammar> node
 *
 * Returns the internal xmlRelaxNGGrammarPtr built or
 *         NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseGrammar(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                            mut nodes: xmlNodePtr)
 -> xmlRelaxNGGrammarPtr {
    let mut ret: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut tmp: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut old: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    ret = xmlRelaxNGNewGrammar(ctxt);
    if ret.is_null() { return 0 as xmlRelaxNGGrammarPtr }
    /*
     * Link the new grammar in the tree
     */
    (*ret).parent = (*ctxt).grammar;
    if !(*ctxt).grammar.is_null() {
        tmp = (*(*ctxt).grammar).children;
        if tmp.is_null() {
            (*(*ctxt).grammar).children = ret
        } else {
            while !(*tmp).next.is_null() { tmp = (*tmp).next }
            (*tmp).next = ret
        }
    }
    old = (*ctxt).grammar;
    (*ctxt).grammar = ret;
    xmlRelaxNGParseGrammarContent(ctxt, nodes);
    (*ctxt).grammar = ret;
    if (*ctxt).grammar.is_null() {
        xmlRngPErr(ctxt, nodes, XML_RNGP_GRAMMAR_CONTENT as std::os::raw::c_int,
                   b"Failed to parse <grammar> content\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
    } else if (*(*ctxt).grammar).start.is_null() {
        xmlRngPErr(ctxt, nodes, XML_RNGP_GRAMMAR_NO_START as std::os::raw::c_int,
                   b"Element <grammar> has no <start>\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
    }
    /*
     * Apply 4.17 merging rules to defines and starts
     */
    xmlRelaxNGCombineStart(ctxt, ret);
    if !(*ret).defs.is_null() {
        xmlHashScan((*ret).defs,
                    Some(xmlRelaxNGCheckCombine as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    /*
     * link together defines and refs in this grammar
     */
    if !(*ret).refs.is_null() {
        xmlHashScan((*ret).refs,
                    Some(xmlRelaxNGCheckReference as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    /* @@@@ */
    (*ctxt).grammar = old;
    return ret;
}
/* *
 * xmlRelaxNGParseDocument:
 * @ctxt:  a Relax-NG parser context
 * @node:  the root node of the RelaxNG schema
 *
 * parse a Relax-NG definition resource and build an internal
 * xmlRelaxNG struture which can be used to validate instances.
 *
 * Returns the internal XML RelaxNG structure built or
 *         NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGParseDocument(mut ctxt:
                                                 xmlRelaxNGParserCtxtPtr,
                                             mut node: xmlNodePtr)
 -> xmlRelaxNGPtr {
    let mut schema: xmlRelaxNGPtr = 0 as xmlRelaxNGPtr;
    let mut olddefine: *const xmlChar = 0 as *const xmlChar;
    let mut old: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    if ctxt.is_null() || node.is_null() { return 0 as xmlRelaxNGPtr }
    schema = xmlRelaxNGNewRelaxNG(ctxt);
    if schema.is_null() { return 0 as xmlRelaxNGPtr }
    olddefine = (*ctxt).define;
    (*ctxt).define = 0 as *const xmlChar;
    if !node.is_null() && !(*node).ns.is_null() &&
           (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
           xmlStrEqual((*node).name,
                       b"grammar\x00" as *const u8 as *const std::os::raw::c_char as
                           *const xmlChar) != 0 &&
           xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0 {
        (*schema).topgrammar = xmlRelaxNGParseGrammar(ctxt, (*node).children);
        if (*schema).topgrammar.is_null() {
            xmlRelaxNGFree(schema);
            return 0 as xmlRelaxNGPtr
        }
    } else {
        let mut tmp: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut ret: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        ret = xmlRelaxNGNewGrammar(ctxt);
        (*schema).topgrammar = ret;
        if (*schema).topgrammar.is_null() {
            xmlRelaxNGFree(schema);
            return 0 as xmlRelaxNGPtr
        }
        /*
         * Link the new grammar in the tree
         */
        (*ret).parent = (*ctxt).grammar;
        if !(*ctxt).grammar.is_null() {
            tmp = (*(*ctxt).grammar).children;
            if tmp.is_null() {
                (*(*ctxt).grammar).children = ret
            } else {
                while !(*tmp).next.is_null() { tmp = (*tmp).next }
                (*tmp).next = ret
            }
        }
        old = (*ctxt).grammar;
        (*ctxt).grammar = ret;
        xmlRelaxNGParseStart(ctxt, node);
        if !old.is_null() { (*ctxt).grammar = old }
    }
    (*ctxt).define = olddefine;
    if !(*(*schema).topgrammar).start.is_null() {
        xmlRelaxNGCheckCycles(ctxt, (*(*schema).topgrammar).start,
                              0 as std::os::raw::c_int);
        if (*ctxt).flags & (1 as std::os::raw::c_int) << 7 as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
            xmlRelaxNGSimplify(ctxt, (*(*schema).topgrammar).start,
                               0 as xmlRelaxNGDefinePtr);
            while !(*(*schema).topgrammar).start.is_null() &&
                      (*(*(*schema).topgrammar).start).type_0 as std::os::raw::c_int
                          == XML_RELAXNG_NOOP as std::os::raw::c_int &&
                      !(*(*(*schema).topgrammar).start).next.is_null() {
                (*(*schema).topgrammar).start =
                    (*(*(*schema).topgrammar).start).content
            }
            xmlRelaxNGCheckRules(ctxt, (*(*schema).topgrammar).start,
                                 (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int,
                                 XML_RELAXNG_NOOP);
        }
    }
    return schema;
}
/*
 * Interfaces for parsing.
 */
/* ***********************************************************************
 *									*
 *			Reading RelaxNGs				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGNewParserCtxt:
 * @URL:  the location of the schema
 *
 * Create an XML RelaxNGs parse context for that file/resource expected
 * to contain an XML RelaxNGs file.
 *
 * Returns the parser context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewParserCtxt(mut URL: *const std::os::raw::c_char)
 -> xmlRelaxNGParserCtxtPtr {
    let mut ret: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    if URL.is_null() { return 0 as xmlRelaxNGParserCtxtPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGParserCtxtPtr;
    if ret.is_null() {
        xmlRngPErrMemory(0 as xmlRelaxNGParserCtxtPtr,
                         b"building parser\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlRelaxNGParserCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGParserCtxt>() as std::os::raw::c_ulong);
    (*ret).URL = xmlStrdup(URL as *const xmlChar);
    (*ret).error = *__xmlGenericError();
    (*ret).userData = *__xmlGenericErrorContext();
    return ret;
}
/* *
 * xmlRelaxNGNewMemParserCtxt:
 * @buffer:  a pointer to a char array containing the schemas
 * @size:  the size of the array
 *
 * Create an XML RelaxNGs parse context for that memory buffer expected
 * to contain an XML RelaxNGs file.
 *
 * Returns the parser context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewMemParserCtxt(mut buffer:
                                                        *const std::os::raw::c_char,
                                                    mut size: std::os::raw::c_int)
 -> xmlRelaxNGParserCtxtPtr {
    let mut ret: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    if buffer.is_null() || size <= 0 as std::os::raw::c_int {
        return 0 as xmlRelaxNGParserCtxtPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGParserCtxtPtr;
    if ret.is_null() {
        xmlRngPErrMemory(0 as xmlRelaxNGParserCtxtPtr,
                         b"building parser\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlRelaxNGParserCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGParserCtxt>() as std::os::raw::c_ulong);
    (*ret).buffer = buffer;
    (*ret).size = size;
    (*ret).error = *__xmlGenericError();
    (*ret).userData = *__xmlGenericErrorContext();
    return ret;
}
/* *
 * xmlRelaxNGNewDocParserCtxt:
 * @doc:  a preparsed document tree
 *
 * Create an XML RelaxNGs parser context for that document.
 * Note: since the process of compiling a RelaxNG schemas modifies the
 *       document, the @doc parameter is duplicated internally.
 *
 * Returns the parser context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewDocParserCtxt(mut doc: xmlDocPtr)
 -> xmlRelaxNGParserCtxtPtr {
    let mut ret: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    let mut copy: xmlDocPtr = 0 as *mut xmlDoc;
    if doc.is_null() { return 0 as xmlRelaxNGParserCtxtPtr }
    copy = xmlCopyDoc(doc, 1 as std::os::raw::c_int);
    if copy.is_null() { return 0 as xmlRelaxNGParserCtxtPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGParserCtxtPtr;
    if ret.is_null() {
        xmlRngPErrMemory(0 as xmlRelaxNGParserCtxtPtr,
                         b"building parser\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlRelaxNGParserCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGParserCtxt>() as std::os::raw::c_ulong);
    (*ret).document = copy;
    (*ret).freedoc = 1 as std::os::raw::c_int;
    (*ret).userData = *__xmlGenericErrorContext();
    return ret;
}
/* *
 * xmlRelaxNGFreeParserCtxt:
 * @ctxt:  the schema parser context
 *
 * Free the resources associated to the schema parser context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFreeParserCtxt(mut ctxt:
                                                      xmlRelaxNGParserCtxtPtr) {
    if ctxt.is_null() { return }
    if !(*ctxt).URL.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).URL as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).doc.is_null() { xmlRelaxNGFreeDocument((*ctxt).doc); }
    if !(*ctxt).interleaves.is_null() {
        xmlHashFree((*ctxt).interleaves, None);
    }
    if !(*ctxt).documents.is_null() {
        xmlRelaxNGFreeDocumentList((*ctxt).documents);
    }
    if !(*ctxt).includes.is_null() {
        xmlRelaxNGFreeIncludeList((*ctxt).includes);
    }
    if !(*ctxt).docTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).docTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).incTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).incTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).defTab.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < (*ctxt).defNr {
            xmlRelaxNGFreeDefine(*(*ctxt).defTab.offset(i as isize));
            i += 1
        }
        xmlFree.expect("non-null function pointer")((*ctxt).defTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).document.is_null() && (*ctxt).freedoc != 0 {
        xmlFreeDoc((*ctxt).document);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/* *
 * xmlRelaxNGNormExtSpace:
 * @value:  a value
 *
 * Removes the leading and ending spaces of the value
 * The string is modified "in situ"
 */
unsafe extern "C" fn xmlRelaxNGNormExtSpace(mut value: *mut xmlChar) {
    let mut start: *mut xmlChar = value;
    let mut cur: *mut xmlChar = value;
    if value.is_null() { return }
    while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                  *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        cur = cur.offset(1)
    }
    if cur == start {
        loop  {
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                cur = cur.offset(1)
            }
            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { return }
            start = cur;
            while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur = cur.offset(1)
            }
            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                *start = 0 as std::os::raw::c_int as xmlChar;
                return
            }
        }
    } else {
        loop  {
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                let fresh31 = cur;
                cur = cur.offset(1);
                let fresh32 = start;
                start = start.offset(1);
                *fresh32 = *fresh31
            }
            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                *start = 0 as std::os::raw::c_int as xmlChar;
                return
            }
            /* don't try to normalize the inner spaces */
            while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                          *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                cur = cur.offset(1)
            }
            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                *start = 0 as std::os::raw::c_int as xmlChar;
                return
            }
            let fresh33 = cur;
            cur = cur.offset(1);
            let fresh34 = start;
            start = start.offset(1);
            *fresh34 = *fresh33
        }
    };
}
/* *
 * xmlRelaxNGCleanupAttributes:
 * @ctxt:  a Relax-NG parser context
 * @node:  a Relax-NG node
 *
 * Check all the attributes on the given node
 */
unsafe extern "C" fn xmlRelaxNGCleanupAttributes(mut ctxt:
                                                     xmlRelaxNGParserCtxtPtr,
                                                 mut node: xmlNodePtr) {
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut next: xmlAttrPtr = 0 as *mut xmlAttr;
    cur = (*node).properties;
    while !cur.is_null() {
        next = (*cur).next;
        if (*cur).ns.is_null() ||
               xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0 {
            if xmlStrEqual((*cur).name,
                           b"name\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar) != 0 {
                if xmlStrEqual((*node).name,
                               b"element\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) == 0
                       &&
                       xmlStrEqual((*node).name,
                                   b"attribute\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 &&
                       xmlStrEqual((*node).name,
                                   b"ref\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 &&
                       xmlStrEqual((*node).name,
                                   b"parentRef\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 &&
                       xmlStrEqual((*node).name,
                                   b"param\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 &&
                       xmlStrEqual((*node).name,
                                   b"define\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_FORBIDDEN_ATTRIBUTE as std::os::raw::c_int,
                               b"Attribute %s is not allowed on %s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               (*cur).name, (*node).name);
                }
            } else if xmlStrEqual((*cur).name,
                                  b"type\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                if xmlStrEqual((*node).name,
                               b"value\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) == 0
                       &&
                       xmlStrEqual((*node).name,
                                   b"data\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_FORBIDDEN_ATTRIBUTE as std::os::raw::c_int,
                               b"Attribute %s is not allowed on %s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               (*cur).name, (*node).name);
                }
            } else if xmlStrEqual((*cur).name,
                                  b"href\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                if xmlStrEqual((*node).name,
                               b"externalRef\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) == 0
                       &&
                       xmlStrEqual((*node).name,
                                   b"include\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_FORBIDDEN_ATTRIBUTE as std::os::raw::c_int,
                               b"Attribute %s is not allowed on %s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               (*cur).name, (*node).name);
                }
            } else if xmlStrEqual((*cur).name,
                                  b"combine\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                if xmlStrEqual((*node).name,
                               b"start\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) == 0
                       &&
                       xmlStrEqual((*node).name,
                                   b"define\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 {
                    xmlRngPErr(ctxt, node,
                               XML_RNGP_FORBIDDEN_ATTRIBUTE as std::os::raw::c_int,
                               b"Attribute %s is not allowed on %s\n\x00" as
                                   *const u8 as *const std::os::raw::c_char,
                               (*cur).name, (*node).name);
                }
            } else if xmlStrEqual((*cur).name,
                                  b"datatypeLibrary\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) !=
                          0 {
                let mut val: *mut xmlChar = 0 as *mut xmlChar;
                let mut uri: xmlURIPtr = 0 as *mut xmlURI;
                val =
                    xmlNodeListGetString((*node).doc, (*cur).children,
                                         1 as std::os::raw::c_int);
                if !val.is_null() {
                    if *val.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           != 0 as std::os::raw::c_int {
                        uri = xmlParseURI(val as *const std::os::raw::c_char);
                        if uri.is_null() {
                            xmlRngPErr(ctxt, node,
                                       XML_RNGP_INVALID_URI as std::os::raw::c_int,
                                       b"Attribute %s contains invalid URI %s\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char, (*cur).name,
                                       val);
                        } else {
                            if (*uri).scheme.is_null() {
                                xmlRngPErr(ctxt, node,
                                           XML_RNGP_URI_NOT_ABSOLUTE as
                                               std::os::raw::c_int,
                                           b"Attribute %s URI %s is not absolute\n\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char,
                                           (*cur).name, val);
                            }
                            if !(*uri).fragment.is_null() {
                                xmlRngPErr(ctxt, node,
                                           XML_RNGP_URI_FRAGMENT as
                                               std::os::raw::c_int,
                                           b"Attribute %s URI %s has a fragment ID\n\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char,
                                           (*cur).name, val);
                            }
                            xmlFreeURI(uri);
                        }
                    }
                    xmlFree.expect("non-null function pointer")(val as
                                                                    *mut std::os::raw::c_void);
                }
            } else if xmlStrEqual((*cur).name,
                                  b"ns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar) ==
                          0 {
                xmlRngPErr(ctxt, node,
                           XML_RNGP_UNKNOWN_ATTRIBUTE as std::os::raw::c_int,
                           b"Unknown attribute %s on %s\n\x00" as *const u8 as
                               *const std::os::raw::c_char, (*cur).name,
                           (*node).name);
            }
        }
        cur = next
    };
}
/* *
 * xmlRelaxNGCleanupTree:
 * @ctxt:  a Relax-NG parser context
 * @root:  an xmlNodePtr subtree
 *
 * Cleanup the subtree from unwanted nodes for parsing, resolve
 * Include and externalRef lookups.
 */
unsafe extern "C" fn xmlRelaxNGCleanupTree(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                           mut root: xmlNodePtr) {
    let mut current_block: u64;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut delete: xmlNodePtr = 0 as *mut xmlNode;
    delete = 0 as xmlNodePtr;
    cur = root;
    while !cur.is_null() {
        if !delete.is_null() {
            xmlUnlinkNode(delete);
            xmlFreeNode(delete);
            delete = 0 as xmlNodePtr
        }
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            /*
             * Simplification 4.1. Annotations
             */
            if (*cur).ns.is_null() ||
                   xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) == 0 {
                if !(*cur).parent.is_null() &&
                       (*(*cur).parent).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                       (xmlStrEqual((*(*cur).parent).name,
                                    b"name\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar)
                            != 0 ||
                            xmlStrEqual((*(*cur).parent).name,
                                        b"value\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar) != 0 ||
                            xmlStrEqual((*(*cur).parent).name,
                                        b"param\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar) != 0) {
                    xmlRngPErr(ctxt, cur,
                               XML_RNGP_FOREIGN_ELEMENT as std::os::raw::c_int,
                               b"element %s doesn\'t allow foreign elements\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               (*(*cur).parent).name, 0 as *const xmlChar);
                }
                delete = cur;
                current_block = 2961777392053818233;
            } else {
                xmlRelaxNGCleanupAttributes(ctxt, cur);
                if xmlStrEqual((*cur).name,
                               b"externalRef\x00" as *const u8 as
                                   *const std::os::raw::c_char as *mut xmlChar) != 0 {
                    let mut href: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns: *mut xmlChar = 0 as *mut xmlChar;
                    let mut base: *mut xmlChar = 0 as *mut xmlChar;
                    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
                    let mut docu: xmlRelaxNGDocumentPtr =
                        0 as *mut xmlRelaxNGDocument;
                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
                    ns =
                        xmlGetProp(cur as *const xmlNode,
                                   b"ns\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                    if ns.is_null() {
                        tmp = (*cur).parent;
                        while !tmp.is_null() &&
                                  (*tmp).type_0 as std::os::raw::c_uint ==
                                      XML_ELEMENT_NODE as std::os::raw::c_int as
                                          std::os::raw::c_uint {
                            ns =
                                xmlGetProp(tmp as *const xmlNode,
                                           b"ns\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar);
                            if !ns.is_null() { break ; }
                            tmp = (*tmp).parent
                        }
                    }
                    href =
                        xmlGetProp(cur as *const xmlNode,
                                   b"href\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                    if href.is_null() {
                        xmlRngPErr(ctxt, cur,
                                   XML_RNGP_MISSING_HREF as std::os::raw::c_int,
                                   b"xmlRelaxNGParse: externalRef has no href attribute\n\x00"
                                       as *const u8 as *const std::os::raw::c_char,
                                   0 as *const xmlChar, 0 as *const xmlChar);
                        if !ns.is_null() {
                            xmlFree.expect("non-null function pointer")(ns as
                                                                            *mut std::os::raw::c_void);
                        }
                        delete = cur;
                        current_block = 2961777392053818233;
                    } else {
                        uri = xmlParseURI(href as *const std::os::raw::c_char);
                        if uri.is_null() {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_HREF_ERROR as std::os::raw::c_int,
                                       b"Incorrect URI for externalRef %s\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char, href,
                                       0 as *const xmlChar);
                            if !ns.is_null() {
                                xmlFree.expect("non-null function pointer")(ns
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            if !href.is_null() {
                                xmlFree.expect("non-null function pointer")(href
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            delete = cur;
                            current_block = 2961777392053818233;
                        } else if !(*uri).fragment.is_null() {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_HREF_ERROR as std::os::raw::c_int,
                                       b"Fragment forbidden in URI for externalRef %s\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char, href,
                                       0 as *const xmlChar);
                            if !ns.is_null() {
                                xmlFree.expect("non-null function pointer")(ns
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            xmlFreeURI(uri);
                            if !href.is_null() {
                                xmlFree.expect("non-null function pointer")(href
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            delete = cur;
                            current_block = 2961777392053818233;
                        } else {
                            xmlFreeURI(uri);
                            base =
                                xmlNodeGetBase((*cur).doc,
                                               cur as *const xmlNode);
                            URL = xmlBuildURI(href, base);
                            if URL.is_null() {
                                xmlRngPErr(ctxt, cur,
                                           XML_RNGP_HREF_ERROR as std::os::raw::c_int,
                                           b"Failed to compute URL for externalRef %s\n\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char, href,
                                           0 as *const xmlChar);
                                if !ns.is_null() {
                                    xmlFree.expect("non-null function pointer")(ns
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                if !href.is_null() {
                                    xmlFree.expect("non-null function pointer")(href
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                if !base.is_null() {
                                    xmlFree.expect("non-null function pointer")(base
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                delete = cur;
                                current_block = 2961777392053818233;
                            } else {
                                if !href.is_null() {
                                    xmlFree.expect("non-null function pointer")(href
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                if !base.is_null() {
                                    xmlFree.expect("non-null function pointer")(base
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                docu =
                                    xmlRelaxNGLoadExternalRef(ctxt, URL, ns);
                                if docu.is_null() {
                                    xmlRngPErr(ctxt, cur,
                                               XML_RNGP_EXTERNAL_REF_FAILURE
                                                   as std::os::raw::c_int,
                                               b"Failed to load externalRef %s\n\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char, URL,
                                               0 as *const xmlChar);
                                    if !ns.is_null() {
                                        xmlFree.expect("non-null function pointer")(ns
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                    }
                                    xmlFree.expect("non-null function pointer")(URL
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                    delete = cur;
                                    current_block = 2961777392053818233;
                                } else {
                                    if !ns.is_null() {
                                        xmlFree.expect("non-null function pointer")(ns
                                                                                        as
                                                                                        *mut std::os::raw::c_void);
                                    }
                                    xmlFree.expect("non-null function pointer")(URL
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                    (*cur).psvi = docu as *mut std::os::raw::c_void;
                                    current_block = 1771738965274008886;
                                }
                            }
                        }
                    }
                } else if xmlStrEqual((*cur).name,
                                      b"include\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
                              != 0 {
                    let mut href_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut base_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut URL_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut incl: xmlRelaxNGIncludePtr =
                        0 as *mut xmlRelaxNGInclude;
                    let mut tmp_0: xmlNodePtr = 0 as *mut xmlNode;
                    href_0 =
                        xmlGetProp(cur as *const xmlNode,
                                   b"href\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                    if href_0.is_null() {
                        xmlRngPErr(ctxt, cur,
                                   XML_RNGP_MISSING_HREF as std::os::raw::c_int,
                                   b"xmlRelaxNGParse: include has no href attribute\n\x00"
                                       as *const u8 as *const std::os::raw::c_char,
                                   0 as *const xmlChar, 0 as *const xmlChar);
                        delete = cur;
                        current_block = 2961777392053818233;
                    } else {
                        base_0 =
                            xmlNodeGetBase((*cur).doc, cur as *const xmlNode);
                        URL_0 = xmlBuildURI(href_0, base_0);
                        if URL_0.is_null() {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_HREF_ERROR as std::os::raw::c_int,
                                       b"Failed to compute URL for include %s\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char, href_0,
                                       0 as *const xmlChar);
                            if !href_0.is_null() {
                                xmlFree.expect("non-null function pointer")(href_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            if !base_0.is_null() {
                                xmlFree.expect("non-null function pointer")(base_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            delete = cur;
                            current_block = 2961777392053818233;
                        } else {
                            if !href_0.is_null() {
                                xmlFree.expect("non-null function pointer")(href_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            if !base_0.is_null() {
                                xmlFree.expect("non-null function pointer")(base_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            ns_0 =
                                xmlGetProp(cur as *const xmlNode,
                                           b"ns\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar);
                            if ns_0.is_null() {
                                tmp_0 = (*cur).parent;
                                while !tmp_0.is_null() &&
                                          (*tmp_0).type_0 as std::os::raw::c_uint ==
                                              XML_ELEMENT_NODE as std::os::raw::c_int
                                                  as std::os::raw::c_uint {
                                    ns_0 =
                                        xmlGetProp(tmp_0 as *const xmlNode,
                                                   b"ns\x00" as *const u8 as
                                                       *const std::os::raw::c_char as
                                                       *mut xmlChar);
                                    if !ns_0.is_null() { break ; }
                                    tmp_0 = (*tmp_0).parent
                                }
                            }
                            incl =
                                xmlRelaxNGLoadInclude(ctxt, URL_0, cur, ns_0);
                            if !ns_0.is_null() {
                                xmlFree.expect("non-null function pointer")(ns_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            if incl.is_null() {
                                xmlRngPErr(ctxt, cur,
                                           XML_RNGP_INCLUDE_FAILURE as
                                               std::os::raw::c_int,
                                           b"Failed to load include %s\n\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char, URL_0,
                                           0 as *const xmlChar);
                                xmlFree.expect("non-null function pointer")(URL_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                                delete = cur;
                                current_block = 2961777392053818233;
                            } else {
                                xmlFree.expect("non-null function pointer")(URL_0
                                                                                as
                                                                                *mut std::os::raw::c_void);
                                (*cur).psvi = incl as *mut std::os::raw::c_void;
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                } else if xmlStrEqual((*cur).name,
                                      b"element\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
                              != 0 ||
                              xmlStrEqual((*cur).name,
                                          b"attribute\x00" as *const u8 as
                                              *const std::os::raw::c_char as
                                              *mut xmlChar) != 0 {
                    let mut name: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns_1: *mut xmlChar = 0 as *mut xmlChar;
                    let mut text: xmlNodePtr = 0 as xmlNodePtr;
                    /*
                     * Simplification 4.8. name attribute of element
                     * and attribute elements
                     */
                    name =
                        xmlGetProp(cur as *const xmlNode,
                                   b"name\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                    if !name.is_null() {
                        if (*cur).children.is_null() {
                            text =
                                xmlNewChild(cur, (*cur).ns,
                                            b"name\x00" as *const u8 as
                                                *const std::os::raw::c_char as
                                                *mut xmlChar, name)
                        } else {
                            let mut node: xmlNodePtr = 0 as *mut xmlNode;
                            node =
                                xmlNewDocNode((*cur).doc, (*cur).ns,
                                              b"name\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar,
                                              0 as *const xmlChar);
                            if !node.is_null() {
                                xmlAddPrevSibling((*cur).children, node);
                                text = xmlNewText(name);
                                xmlAddChild(node, text);
                                text = node
                            }
                        }
                        if text.is_null() {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_CREATE_FAILURE as std::os::raw::c_int,
                                       b"Failed to create a name %s element\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char, name,
                                       0 as *const xmlChar);
                        }
                        xmlUnsetProp(cur,
                                     b"name\x00" as *const u8 as
                                         *const std::os::raw::c_char as *mut xmlChar);
                        xmlFree.expect("non-null function pointer")(name as
                                                                        *mut std::os::raw::c_void);
                        ns_1 =
                            xmlGetProp(cur as *const xmlNode,
                                       b"ns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar);
                        if !ns_1.is_null() {
                            if !text.is_null() {
                                xmlSetProp(text,
                                           b"ns\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar, ns_1);
                                /* xmlUnsetProp(cur, BAD_CAST "ns"); */
                            }
                            xmlFree.expect("non-null function pointer")(ns_1
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        } else if xmlStrEqual((*cur).name,
                                              b"attribute\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar) != 0 {
                            xmlSetProp(text,
                                       b"ns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar,
                                       b"\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar);
                        }
                    }
                    current_block = 1771738965274008886;
                } else if xmlStrEqual((*cur).name,
                                      b"name\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
                              != 0 ||
                              xmlStrEqual((*cur).name,
                                          b"nsName\x00" as *const u8 as
                                              *const std::os::raw::c_char as
                                              *mut xmlChar) != 0 ||
                              xmlStrEqual((*cur).name,
                                          b"value\x00" as *const u8 as
                                              *const std::os::raw::c_char as
                                              *mut xmlChar) != 0 {
                    /*
                     * Simplification 4.8. name attribute of element
                     * and attribute elements
                     */
                    if xmlHasProp(cur as *const xmlNode,
                                  b"ns\x00" as *const u8 as
                                      *const std::os::raw::c_char as
                                      *mut xmlChar).is_null() {
                        let mut node_0: xmlNodePtr = 0 as *mut xmlNode;
                        let mut ns_2: *mut xmlChar = 0 as *mut xmlChar;
                        node_0 = (*cur).parent;
                        while !node_0.is_null() &&
                                  (*node_0).type_0 as std::os::raw::c_uint ==
                                      XML_ELEMENT_NODE as std::os::raw::c_int as
                                          std::os::raw::c_uint {
                            ns_2 =
                                xmlGetProp(node_0 as *const xmlNode,
                                           b"ns\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar);
                            if !ns_2.is_null() { break ; }
                            node_0 = (*node_0).parent
                        }
                        if ns_2.is_null() {
                            xmlSetProp(cur,
                                       b"ns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar,
                                       b"\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar);
                        } else {
                            xmlSetProp(cur,
                                       b"ns\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, ns_2);
                            xmlFree.expect("non-null function pointer")(ns_2
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    }
                    if xmlStrEqual((*cur).name,
                                   b"name\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) !=
                           0 {
                        let mut name_0: *mut xmlChar = 0 as *mut xmlChar;
                        let mut local: *mut xmlChar = 0 as *mut xmlChar;
                        let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
                        /*
                         * Simplification: 4.10. QNames
                         */
                        name_0 = xmlNodeGetContent(cur as *const xmlNode);
                        if !name_0.is_null() {
                            local = xmlSplitQName2(name_0, &mut prefix);
                            if !local.is_null() {
                                let mut ns_3: xmlNsPtr = 0 as *mut xmlNs;
                                ns_3 = xmlSearchNs((*cur).doc, cur, prefix);
                                if ns_3.is_null() {
                                    xmlRngPErr(ctxt, cur,
                                               XML_RNGP_PREFIX_UNDEFINED as
                                                   std::os::raw::c_int,
                                               b"xmlRelaxNGParse: no namespace for prefix %s\n\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char,
                                               prefix, 0 as *const xmlChar);
                                } else {
                                    xmlSetProp(cur,
                                               b"ns\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               (*ns_3).href);
                                    xmlNodeSetContent(cur, local);
                                }
                                xmlFree.expect("non-null function pointer")(local
                                                                                as
                                                                                *mut std::os::raw::c_void);
                                xmlFree.expect("non-null function pointer")(prefix
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            xmlFree.expect("non-null function pointer")(name_0
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    }
                    /*
                     * 4.16
                     */
                    if xmlStrEqual((*cur).name,
                                   b"nsName\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) !=
                           0 {
                        if (*ctxt).flags &
                               (1 as std::os::raw::c_int) << 9 as std::os::raw::c_int != 0 {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME as
                                           std::os::raw::c_int,
                                       b"Found nsName/except//nsName forbidden construct\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char,
                                       0 as *const xmlChar,
                                       0 as *const xmlChar);
                        }
                    }
                    current_block = 1771738965274008886;
                } else if xmlStrEqual((*cur).name,
                                      b"except\x00" as *const u8 as
                                          *const std::os::raw::c_char as *mut xmlChar)
                              != 0 && cur != root {
                    let mut oldflags: std::os::raw::c_int = (*ctxt).flags;
                    /*
                     * 4.16
                     */
                    if !(*cur).parent.is_null() &&
                           xmlStrEqual((*(*cur).parent).name,
                                       b"anyName\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 {
                        (*ctxt).flags |=
                            (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int;
                        xmlRelaxNGCleanupTree(ctxt, cur);
                        (*ctxt).flags = oldflags;
                        current_block = 2961777392053818233;
                    } else if !(*cur).parent.is_null() &&
                                  xmlStrEqual((*(*cur).parent).name,
                                              b"nsName\x00" as *const u8 as
                                                  *const std::os::raw::c_char as
                                                  *mut xmlChar) != 0 {
                        (*ctxt).flags |=
                            (1 as std::os::raw::c_int) << 9 as std::os::raw::c_int;
                        xmlRelaxNGCleanupTree(ctxt, cur);
                        (*ctxt).flags = oldflags;
                        current_block = 2961777392053818233;
                    } else { current_block = 1771738965274008886; }
                } else {
                    if xmlStrEqual((*cur).name,
                                   b"anyName\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) !=
                           0 {
                        /*
                     * 4.16
                     */
                        if (*ctxt).flags &
                               (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int != 0 {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME as
                                           std::os::raw::c_int,
                                       b"Found anyName/except//anyName forbidden construct\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char,
                                       0 as *const xmlChar,
                                       0 as *const xmlChar);
                        } else if (*ctxt).flags &
                                      (1 as std::os::raw::c_int) << 9 as std::os::raw::c_int
                                      != 0 {
                            xmlRngPErr(ctxt, cur,
                                       XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME as
                                           std::os::raw::c_int,
                                       b"Found nsName/except//anyName forbidden construct\n\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char,
                                       0 as *const xmlChar,
                                       0 as *const xmlChar);
                        }
                    }
                    current_block = 1771738965274008886;
                }
                match current_block {
                    2961777392053818233 => { }
                    _ =>
                    /*
                 * This is not an else since "include" is transformed
                 * into a div
                 */
                    {
                        if xmlStrEqual((*cur).name,
                                       b"div\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) != 0 {
                            let mut ns_4: *mut xmlChar = 0 as *mut xmlChar;
                            let mut child: xmlNodePtr = 0 as *mut xmlNode;
                            let mut ins: xmlNodePtr = 0 as *mut xmlNode;
                            let mut tmp_1: xmlNodePtr = 0 as *mut xmlNode;
                            /*
                     * implements rule 4.11
                     */
                            ns_4 =
                                xmlGetProp(cur as *const xmlNode,
                                           b"ns\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar);
                            child = (*cur).children;
                            ins = cur;
                            while !child.is_null() {
                                if !ns_4.is_null() {
                                    if xmlHasProp(child as *const xmlNode,
                                                  b"ns\x00" as *const u8 as
                                                      *const std::os::raw::c_char as
                                                      *mut xmlChar).is_null()
                                       {
                                        xmlSetProp(child,
                                                   b"ns\x00" as *const u8 as
                                                       *const std::os::raw::c_char as
                                                       *mut xmlChar, ns_4);
                                    }
                                }
                                tmp_1 = (*child).next;
                                xmlUnlinkNode(child);
                                ins = xmlAddNextSibling(ins, child);
                                child = tmp_1
                            }
                            if !ns_4.is_null() {
                                xmlFree.expect("non-null function pointer")(ns_4
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            /*
		     * Since we are about to delete cur, if its nsDef is non-NULL we
		     * need to preserve it (it contains the ns definitions for the
		     * children we just moved).  We'll just stick it on to the end
		     * of cur->parent's list, since it's never going to be re-serialized
		     * (bug 143738).
		     */
                            if !(*cur).nsDef.is_null() &&
                                   !(*cur).parent.is_null() {
                                let mut parDef: xmlNsPtr =
                                    &mut (*(*cur).parent).nsDef as
                                        *mut *mut xmlNs as xmlNsPtr;
                                while !(*parDef).next.is_null() {
                                    parDef = (*parDef).next
                                }
                                (*parDef).next = (*cur).nsDef;
                                (*cur).nsDef = 0 as *mut xmlNs
                            }
                            delete = cur;
                            current_block = 2961777392053818233;
                        } else { current_block = 3788568606521286043; }
                    }
                }
            }
        } else if (*cur).type_0 as std::os::raw::c_uint ==
                      XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                              std::os::raw::c_uint {
            if xmlRelaxNGIsBlank((*cur).content) != 0 {
                if !(*cur).parent.is_null() &&
                       (*(*cur).parent).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    if xmlStrEqual((*(*cur).parent).name,
                                   b"value\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar) ==
                           0 &&
                           xmlStrEqual((*(*cur).parent).name,
                                       b"param\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar) == 0 {
                        delete = cur
                    }
                    current_block = 3788568606521286043;
                } else { delete = cur; current_block = 2961777392053818233; }
            } else { current_block = 3788568606521286043; }
        } else { delete = cur; current_block = 2961777392053818233; }
        match current_block {
            3788568606521286043 =>
            /*
         * Simplification 4.2 whitespaces
         */
            /*
         * Skip to next node
         */
            {
                if !(*cur).children.is_null() {
                    if (*(*cur).children).type_0 as std::os::raw::c_uint !=
                           XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
                           (*(*cur).children).type_0 as std::os::raw::c_uint !=
                               XML_ENTITY_REF_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint &&
                           (*(*cur).children).type_0 as std::os::raw::c_uint !=
                               XML_ENTITY_NODE as std::os::raw::c_int as std::os::raw::c_uint
                       {
                        cur = (*cur).children;
                        continue ;
                    }
                }
            }
            _ => { }
        }
        if !(*cur).next.is_null() {
            cur = (*cur).next
        } else {
            loop  {
                cur = (*cur).parent;
                if cur.is_null() { break ; }
                if cur == root {
                    cur = 0 as xmlNodePtr;
                    break ;
                } else if !(*cur).next.is_null() {
                    cur = (*cur).next;
                    break ;
                } else if cur.is_null() { break ; }
            }
        }
    }
    if !delete.is_null() {
        xmlUnlinkNode(delete);
        xmlFreeNode(delete);
        delete = 0 as xmlNodePtr
    };
}
/* ***********************************************************************
 *									*
 *			Document functions				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGCleanupDoc:
 * @ctxt:  a Relax-NG parser context
 * @doc:  an xmldocPtr document pointer
 *
 * Cleanup the document from unwanted nodes for parsing, resolve
 * Include and externalRef lookups.
 *
 * Returns the cleaned up document or NULL in case of error
 */
unsafe extern "C" fn xmlRelaxNGCleanupDoc(mut ctxt: xmlRelaxNGParserCtxtPtr,
                                          mut doc: xmlDocPtr) -> xmlDocPtr {
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * Extract the root
     */
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(ctxt, doc as xmlNodePtr, XML_RNGP_EMPTY as std::os::raw::c_int,
                   b"xmlRelaxNGParse: %s is empty\n\x00" as *const u8 as
                       *const std::os::raw::c_char, (*ctxt).URL, 0 as *const xmlChar);
        return 0 as xmlDocPtr
    }
    xmlRelaxNGCleanupTree(ctxt, root);
    return doc;
}
/* *
 * xmlRelaxNGParse:
 * @ctxt:  a Relax-NG parser context
 *
 * parse a schema definition resource and build an internal
 * XML Shema struture which can be used to validate instances.
 *
 * Returns the internal XML RelaxNG structure built from the resource or
 *         NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGParse(mut ctxt: xmlRelaxNGParserCtxtPtr)
 -> xmlRelaxNGPtr {
    let mut ret: xmlRelaxNGPtr = 0 as xmlRelaxNGPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    xmlRelaxNGInitTypes();
    if ctxt.is_null() { return 0 as xmlRelaxNGPtr }
    /*
     * First step is to parse the input document into an DOM/Infoset
     */
    if !(*ctxt).URL.is_null() {
        doc =
            xmlReadFile((*ctxt).URL as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
        if doc.is_null() {
            xmlRngPErr(ctxt, 0 as xmlNodePtr,
                       XML_RNGP_PARSE_ERROR as std::os::raw::c_int,
                       b"xmlRelaxNGParse: could not load %s\n\x00" as
                           *const u8 as *const std::os::raw::c_char, (*ctxt).URL,
                       0 as *const xmlChar);
            return 0 as xmlRelaxNGPtr
        }
    } else if !(*ctxt).buffer.is_null() {
        doc =
            xmlReadMemory((*ctxt).buffer, (*ctxt).size,
                          0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                          0 as std::os::raw::c_int);
        if doc.is_null() {
            xmlRngPErr(ctxt, 0 as xmlNodePtr,
                       XML_RNGP_PARSE_ERROR as std::os::raw::c_int,
                       b"xmlRelaxNGParse: could not parse schemas\n\x00" as
                           *const u8 as *const std::os::raw::c_char,
                       0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as xmlRelaxNGPtr
        }
        (*doc).URL =
            xmlStrdup(b"in_memory_buffer\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar);
        (*ctxt).URL =
            xmlStrdup(b"in_memory_buffer\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar)
    } else if !(*ctxt).document.is_null() {
        doc = (*ctxt).document
    } else {
        xmlRngPErr(ctxt, 0 as xmlNodePtr, XML_RNGP_EMPTY as std::os::raw::c_int,
                   b"xmlRelaxNGParse: nothing to parse\n\x00" as *const u8 as
                       *const std::os::raw::c_char, 0 as *const xmlChar,
                   0 as *const xmlChar);
        return 0 as xmlRelaxNGPtr
    }
    (*ctxt).document = doc;
    /*
     * Some preprocessing of the document content
     */
    doc = xmlRelaxNGCleanupDoc(ctxt, doc);
    if doc.is_null() {
        xmlFreeDoc((*ctxt).document);
        (*ctxt).document = 0 as xmlDocPtr;
        return 0 as xmlRelaxNGPtr
    }
    /*
     * Then do the parsing for good
     */
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(ctxt, doc as xmlNodePtr, XML_RNGP_EMPTY as std::os::raw::c_int,
                   b"xmlRelaxNGParse: %s is empty\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   if !(*ctxt).URL.is_null() {
                       (*ctxt).URL
                   } else {
                       b"schemas\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar
                   }, 0 as *const xmlChar);
        xmlFreeDoc((*ctxt).document);
        (*ctxt).document = 0 as xmlDocPtr;
        return 0 as xmlRelaxNGPtr
    }
    ret = xmlRelaxNGParseDocument(ctxt, root);
    if ret.is_null() {
        xmlFreeDoc((*ctxt).document);
        (*ctxt).document = 0 as xmlDocPtr;
        return 0 as xmlRelaxNGPtr
    }
    /*
     * Check the ref/defines links
     */
    /*
     * try to preprocess interleaves
     */
    if !(*ctxt).interleaves.is_null() {
        xmlHashScan((*ctxt).interleaves,
                    Some(xmlRelaxNGComputeInterleaves as
                             unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: *mut std::os::raw::c_void,
                                                  _: *const xmlChar) -> ()),
                    ctxt as *mut std::os::raw::c_void);
    }
    /*
     * if there was a parsing error return NULL
     */
    if (*ctxt).nbErrors > 0 as std::os::raw::c_int {
        xmlRelaxNGFree(ret);
        (*ctxt).document = 0 as xmlDocPtr;
        xmlFreeDoc(doc);
        return 0 as xmlRelaxNGPtr
    }
    /*
     * try to compile (parts of) the schemas
     */
    if !(*ret).topgrammar.is_null() && !(*(*ret).topgrammar).start.is_null() {
        if (*(*(*ret).topgrammar).start).type_0 as std::os::raw::c_int !=
               XML_RELAXNG_START as std::os::raw::c_int {
            let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            def = xmlRelaxNGNewDefine(ctxt, 0 as xmlNodePtr);
            if !def.is_null() {
                (*def).type_0 = XML_RELAXNG_START;
                (*def).content = (*(*ret).topgrammar).start;
                (*(*ret).topgrammar).start = def
            }
        }
        xmlRelaxNGTryCompile(ctxt, (*(*ret).topgrammar).start);
    }
    /*
     * Transfer the pointer for cleanup at the schema level.
     */
    (*ret).doc = doc;
    (*ctxt).document = 0 as xmlDocPtr;
    (*ret).documents = (*ctxt).documents;
    (*ctxt).documents = 0 as xmlRelaxNGDocumentPtr;
    (*ret).includes = (*ctxt).includes;
    (*ctxt).includes = 0 as xmlRelaxNGIncludePtr;
    (*ret).defNr = (*ctxt).defNr;
    (*ret).defTab = (*ctxt).defTab;
    (*ctxt).defTab = 0 as *mut xmlRelaxNGDefinePtr;
    if (*ctxt).idref == 1 as std::os::raw::c_int { (*ret).idref = 1 as std::os::raw::c_int }
    return ret;
}
/* *
 * xmlRelaxNGSetParserErrors:
 * @ctxt:  a Relax-NG validation context
 * @err:  the error callback
 * @warn:  the warning callback
 * @ctx:  contextual data for the callbacks
 *
 * Set the callback functions used to handle errors for a validation context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetParserErrors(mut ctxt:
                                                       xmlRelaxNGParserCtxtPtr,
                                                   mut err:
                                                       xmlRelaxNGValidityErrorFunc,
                                                   mut warn:
                                                       xmlRelaxNGValidityWarningFunc,
                                                   mut ctx:
                                                       *mut std::os::raw::c_void) {
    if ctxt.is_null() { return }
    (*ctxt).error = err;
    (*ctxt).warning = warn;
    (*ctxt).serror = None;
    (*ctxt).userData = ctx;
}
/* *
 * xmlRelaxNGGetParserErrors:
 * @ctxt:  a Relax-NG validation context
 * @err:  the error callback result
 * @warn:  the warning callback result
 * @ctx:  contextual data for the callbacks result
 *
 * Get the callback information used to handle errors for a validation context
 *
 * Returns -1 in case of failure, 0 otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGGetParserErrors(mut ctxt:
                                                       xmlRelaxNGParserCtxtPtr,
                                                   mut err:
                                                       *mut xmlRelaxNGValidityErrorFunc,
                                                   mut warn:
                                                       *mut xmlRelaxNGValidityWarningFunc,
                                                   mut ctx:
                                                       *mut *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if !err.is_null() { *err = (*ctxt).error }
    if !warn.is_null() { *warn = (*ctxt).warning }
    if !ctx.is_null() { *ctx = (*ctxt).userData }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGSetParserStructuredErrors:
 * @ctxt:  a Relax-NG parser context
 * @serror:  the error callback
 * @ctx:  contextual data for the callbacks
 *
 * Set the callback functions used to handle errors for a parsing context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetParserStructuredErrors(mut ctxt:
                                                                 xmlRelaxNGParserCtxtPtr,
                                                             mut serror:
                                                                 xmlStructuredErrorFunc,
                                                             mut ctx:
                                                                 *mut std::os::raw::c_void) {
    if ctxt.is_null() { return }
    (*ctxt).serror = serror;
    (*ctxt).error = None;
    (*ctxt).warning = None;
    (*ctxt).userData = ctx;
}
/* *
 * xmlRelaxNGDumpDefines:
 * @output:  the file output
 * @defines:  a list of define structures
 *
 * Dump a RelaxNG structure back
 */
unsafe extern "C" fn xmlRelaxNGDumpDefines(mut output: *mut FILE,
                                           mut defines: xmlRelaxNGDefinePtr) {
    while !defines.is_null() {
        xmlRelaxNGDumpDefine(output, defines);
        defines = (*defines).next
    };
}
/* ***********************************************************************
 *									*
 *			Dump back a compiled form			*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGDumpDefine:
 * @output:  the file output
 * @define:  a define structure
 *
 * Dump a RelaxNG structure back
 */
unsafe extern "C" fn xmlRelaxNGDumpDefine(mut output: *mut FILE,
                                          mut define: xmlRelaxNGDefinePtr) {
    if define.is_null() { return }
    match (*define).type_0 as std::os::raw::c_int {
        0 => {
            fprintf(output,
                    b"<empty/>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        1 => {
            fprintf(output,
                    b"<notAllowed/>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        3 => {
            fprintf(output,
                    b"<text/>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        4 => {
            fprintf(output,
                    b"<element>\n\x00" as *const u8 as *const std::os::raw::c_char);
            if !(*define).name.is_null() {
                fprintf(output,
                        b"<name\x00" as *const u8 as *const std::os::raw::c_char);
                if !(*define).ns.is_null() {
                    fprintf(output,
                            b" ns=\"%s\"\x00" as *const u8 as
                                *const std::os::raw::c_char, (*define).ns);
                }
                fprintf(output,
                        b">%s</name>\n\x00" as *const u8 as
                            *const std::os::raw::c_char, (*define).name);
            }
            xmlRelaxNGDumpDefines(output, (*define).attrs);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</element>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        8 => {
            fprintf(output,
                    b"<list>\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</list>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        16 => {
            fprintf(output,
                    b"<oneOrMore>\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</oneOrMore>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        15 => {
            fprintf(output,
                    b"<zeroOrMore>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</zeroOrMore>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        17 => {
            fprintf(output,
                    b"<choice>\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</choice>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        18 => {
            fprintf(output,
                    b"<group>\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</group>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        19 => {
            fprintf(output,
                    b"<interleave>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</interleave>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        14 => {
            fprintf(output,
                    b"<optional>\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</optional>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        9 => {
            fprintf(output,
                    b"<attribute>\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</attribute>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        10 => {
            fprintf(output,
                    b"<define\x00" as *const u8 as *const std::os::raw::c_char);
            if !(*define).name.is_null() {
                fprintf(output,
                        b" name=\"%s\"\x00" as *const u8 as
                            *const std::os::raw::c_char, (*define).name);
            }
            fprintf(output, b">\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</define>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        11 => {
            fprintf(output, b"<ref\x00" as *const u8 as *const std::os::raw::c_char);
            if !(*define).name.is_null() {
                fprintf(output,
                        b" name=\"%s\"\x00" as *const u8 as
                            *const std::os::raw::c_char, (*define).name);
            }
            fprintf(output, b">\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</ref>\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        13 => {
            fprintf(output,
                    b"<parentRef\x00" as *const u8 as *const std::os::raw::c_char);
            if !(*define).name.is_null() {
                fprintf(output,
                        b" name=\"%s\"\x00" as *const u8 as
                            *const std::os::raw::c_char, (*define).name);
            }
            fprintf(output, b">\n\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</parentRef>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        12 => {
            fprintf(output,
                    b"<externalRef>\x00" as *const u8 as *const std::os::raw::c_char);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output,
                    b"</externalRef>\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        5 | 7 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       7804 as
                                                                           std::os::raw::c_int);
        }
        20 | 2 | 6 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       7808 as
                                                                           std::os::raw::c_int);
        }
        -1 => { xmlRelaxNGDumpDefines(output, (*define).content); }
        _ => { }
    };
}
/* *
 * xmlRelaxNGDumpGrammar:
 * @output:  the file output
 * @grammar:  a grammar structure
 * @top:  is this a top grammar
 *
 * Dump a RelaxNG structure back
 */
unsafe extern "C" fn xmlRelaxNGDumpGrammar(mut output: *mut FILE,
                                           mut grammar: xmlRelaxNGGrammarPtr,
                                           mut top: std::os::raw::c_int) {
    if grammar.is_null() { return }
    fprintf(output, b"<grammar\x00" as *const u8 as *const std::os::raw::c_char);
    if top != 0 {
        fprintf(output,
                b" xmlns=\"http://relaxng.org/ns/structure/1.0\"\x00" as
                    *const u8 as *const std::os::raw::c_char);
    }
    match (*grammar).combine as std::os::raw::c_uint {
        0 => { }
        1 => {
            fprintf(output,
                    b" combine=\"choice\"\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        2 => {
            fprintf(output,
                    b" combine=\"interleave\"\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
        _ => {
            fprintf(output,
                    b" <!-- invalid combine value -->\x00" as *const u8 as
                        *const std::os::raw::c_char);
        }
    }
    fprintf(output, b">\n\x00" as *const u8 as *const std::os::raw::c_char);
    if (*grammar).start.is_null() {
        fprintf(output,
                b" <!-- grammar had no start -->\x00" as *const u8 as
                    *const std::os::raw::c_char);
    } else {
        fprintf(output, b"<start>\n\x00" as *const u8 as *const std::os::raw::c_char);
        xmlRelaxNGDumpDefine(output, (*grammar).start);
        fprintf(output,
                b"</start>\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* TODO ? Dump the defines ? */
    fprintf(output, b"</grammar>\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/* *
 * xmlRelaxNGDump:
 * @output:  the file output
 * @schema:  a schema structure
 *
 * Dump a RelaxNG structure back
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGDump(mut output: *mut FILE,
                                        mut schema: xmlRelaxNGPtr) {
    if output.is_null() { return }
    if schema.is_null() {
        fprintf(output,
                b"RelaxNG empty or failed to compile\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return
    }
    fprintf(output, b"RelaxNG: \x00" as *const u8 as *const std::os::raw::c_char);
    if (*schema).doc.is_null() {
        fprintf(output,
                b"no document\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else if !(*(*schema).doc).URL.is_null() {
        fprintf(output, b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                (*(*schema).doc).URL);
    } else { fprintf(output, b"\n\x00" as *const u8 as *const std::os::raw::c_char); }
    if (*schema).topgrammar.is_null() {
        fprintf(output,
                b"RelaxNG has no top grammar\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return
    }
    xmlRelaxNGDumpGrammar(output, (*schema).topgrammar, 1 as std::os::raw::c_int);
}
/* *
 * xmlRelaxNGDumpTree:
 * @output:  the file output
 * @schema:  a schema structure
 *
 * Dump the transformed RelaxNG tree.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGDumpTree(mut output: *mut FILE,
                                            mut schema: xmlRelaxNGPtr) {
    if output.is_null() { return }
    if schema.is_null() {
        fprintf(output,
                b"RelaxNG empty or failed to compile\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return
    }
    if (*schema).doc.is_null() {
        fprintf(output,
                b"no document\n\x00" as *const u8 as *const std::os::raw::c_char);
    } else { xmlDocDump(output, (*schema).doc); };
}
/* *
 * xmlRelaxNGValidateCompiledCallback:
 * @exec:  the regular expression instance
 * @token:  the token which matched
 * @transdata:  callback data, the define for the subelement if available
 @ @inputdata:  callback data, the Relax NG validation context
 *
 * Handle the callback and if needed validate the element children.
 */
unsafe extern "C" fn xmlRelaxNGValidateCompiledCallback(mut exec:
                                                            xmlRegExecCtxtPtr,
                                                        mut token:
                                                            *const xmlChar,
                                                        mut transdata:
                                                            *mut std::os::raw::c_void,
                                                        mut inputdata:
                                                            *mut std::os::raw::c_void) {
    let mut ctxt: xmlRelaxNGValidCtxtPtr =
        inputdata as xmlRelaxNGValidCtxtPtr;
    let mut define: xmlRelaxNGDefinePtr = transdata as xmlRelaxNGDefinePtr;
    let mut ret: std::os::raw::c_int = 0;
    if ctxt.is_null() {
        fprintf(stderr,
                b"callback on %s missing context\n\x00" as *const u8 as
                    *const std::os::raw::c_char, token);
        return
    }
    if define.is_null() {
        if *token.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '#' as i32 {
            return
        }
        fprintf(stderr,
                b"callback on %s missing define\n\x00" as *const u8 as
                    *const std::os::raw::c_char, token);
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as std::os::raw::c_int
        }
        return
    }
    if ctxt.is_null() || define.is_null() {
        fprintf(stderr,
                b"callback on %s missing info\n\x00" as *const u8 as
                    *const std::os::raw::c_char, token);
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as std::os::raw::c_int
        }
        return
    } else {
        if (*define).type_0 as std::os::raw::c_int !=
               XML_RELAXNG_ELEMENT as std::os::raw::c_int {
            fprintf(stderr,
                    b"callback on %s define is not element\n\x00" as *const u8
                        as *const std::os::raw::c_char, token);
            if (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
                (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as std::os::raw::c_int
            }
            return
        }
    }
    ret = xmlRelaxNGValidateDefinition(ctxt, define);
    if ret != 0 as std::os::raw::c_int { (*ctxt).perr = ret };
}
/* *
 * xmlRelaxNGValidateCompiledContent:
 * @ctxt:  the RelaxNG validation context
 * @regexp:  the regular expression as compiled
 * @content:  list of children to test against the regexp
 *
 * Validate the content model of an element or start using the regexp
 *
 * Returns 0 in case of success, -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGValidateCompiledContent(mut ctxt:
                                                           xmlRelaxNGValidCtxtPtr,
                                                       mut regexp:
                                                           xmlRegexpPtr,
                                                       mut content:
                                                           xmlNodePtr)
 -> std::os::raw::c_int {
    let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut oldperr: std::os::raw::c_int = 0;
    if ctxt.is_null() || regexp.is_null() { return -(1 as std::os::raw::c_int) }
    oldperr = (*ctxt).perr;
    exec =
        xmlRegNewExecCtxt(regexp,
                          Some(xmlRelaxNGValidateCompiledCallback as
                                   unsafe extern "C" fn(_: xmlRegExecCtxtPtr,
                                                        _: *const xmlChar,
                                                        _: *mut std::os::raw::c_void,
                                                        _: *mut std::os::raw::c_void)
                                       -> ()), ctxt as *mut std::os::raw::c_void);
    (*ctxt).perr = 0 as std::os::raw::c_int;
    cur = content;
    while !cur.is_null() {
        (*(*ctxt).state).seq = cur;
        match (*cur).type_0 as std::os::raw::c_uint {
            3 | 4 => {
                if !(xmlIsBlankNode(cur as *const xmlNode) != 0) {
                    ret =
                        xmlRegExecPushString(exec,
                                             b"#text\x00" as *const u8 as
                                                 *const std::os::raw::c_char as
                                                 *mut xmlChar,
                                             ctxt as *mut std::os::raw::c_void);
                    if ret < 0 as std::os::raw::c_int {
                        xmlRelaxNGAddValidError(ctxt,
                                                XML_RELAXNG_ERR_TEXTWRONG,
                                                (*(*cur).parent).name,
                                                0 as *const xmlChar,
                                                0 as std::os::raw::c_int);
                    }
                }
            }
            1 => {
                if !(*cur).ns.is_null() {
                    ret =
                        xmlRegExecPushString2(exec, (*cur).name,
                                              (*(*cur).ns).href,
                                              ctxt as *mut std::os::raw::c_void)
                } else {
                    ret =
                        xmlRegExecPushString(exec, (*cur).name,
                                             ctxt as *mut std::os::raw::c_void)
                }
                if ret < 0 as std::os::raw::c_int {
                    xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMWRONG,
                                            (*cur).name, 0 as *const xmlChar,
                                            0 as std::os::raw::c_int);
                }
            }
            _ => { }
        }
        if ret < 0 as std::os::raw::c_int { break ; }
        /*
         * Switch to next element
         */
        cur = (*cur).next
    }
    ret =
        xmlRegExecPushString(exec, 0 as *const xmlChar,
                             0 as *mut std::os::raw::c_void);
    if ret == 1 as std::os::raw::c_int {
        ret = 0 as std::os::raw::c_int;
        (*(*ctxt).state).seq = 0 as xmlNodePtr
    } else if ret == 0 as std::os::raw::c_int {
        /*
         * TODO: get some of the names needed to exit the current state of exec
         */
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOELEM,
                                b"\x00" as *const u8 as *const std::os::raw::c_char as
                                    *mut xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        ret = -(1 as std::os::raw::c_int);
        if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            xmlRelaxNGDumpValidError(ctxt);
        }
    } else { ret = -(1 as std::os::raw::c_int) }
    xmlRegFreeExecCtxt(exec);
    /*
     * There might be content model errors outside of the pure
     * regexp validation, e.g. for attribute values.
     */
    if ret == 0 as std::os::raw::c_int && (*ctxt).perr != 0 as std::os::raw::c_int {
        ret = (*ctxt).perr
    }
    (*ctxt).perr = oldperr;
    return ret;
}
/* *
 * xmlRelaxNGElemPush:
 * @ctxt:  the validation context
 * @exec:  the regexp runtime for the new content model
 *
 * Push a new regexp for the current node content model on the stack
 *
 * Returns 0 in case of success and -1 in case of error.
 */
unsafe extern "C" fn xmlRelaxNGElemPush(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                        mut exec: xmlRegExecCtxtPtr)
 -> std::os::raw::c_int {
    if (*ctxt).elemTab.is_null() {
        (*ctxt).elemMax = 10 as std::os::raw::c_int;
        (*ctxt).elemTab =
            xmlMalloc.expect("non-null function pointer")(((*ctxt).elemMax as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRegExecCtxtPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlRegExecCtxtPtr;
        if (*ctxt).elemTab.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"validating\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    if (*ctxt).elemNr >= (*ctxt).elemMax {
        (*ctxt).elemMax *= 2 as std::os::raw::c_int;
        (*ctxt).elemTab =
            xmlRealloc.expect("non-null function pointer")((*ctxt).elemTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*ctxt).elemMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlRegExecCtxtPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlRegExecCtxtPtr;
        if (*ctxt).elemTab.is_null() {
            xmlRngVErrMemory(ctxt,
                             b"validating\n\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return -(1 as std::os::raw::c_int)
        }
    }
    let fresh35 = (*ctxt).elemNr;
    (*ctxt).elemNr = (*ctxt).elemNr + 1;
    let ref mut fresh36 = *(*ctxt).elemTab.offset(fresh35 as isize);
    *fresh36 = exec;
    (*ctxt).elem = exec;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGElemPop:
 * @ctxt:  the validation context
 *
 * Pop the regexp of the current node content model from the stack
 *
 * Returns the exec or NULL if empty
 */
unsafe extern "C" fn xmlRelaxNGElemPop(mut ctxt: xmlRelaxNGValidCtxtPtr)
 -> xmlRegExecCtxtPtr {
    let mut ret: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    if (*ctxt).elemNr <= 0 as std::os::raw::c_int { return 0 as xmlRegExecCtxtPtr }
    (*ctxt).elemNr -= 1;
    ret = *(*ctxt).elemTab.offset((*ctxt).elemNr as isize);
    let ref mut fresh37 = *(*ctxt).elemTab.offset((*ctxt).elemNr as isize);
    *fresh37 = 0 as xmlRegExecCtxtPtr;
    if (*ctxt).elemNr > 0 as std::os::raw::c_int {
        (*ctxt).elem =
            *(*ctxt).elemTab.offset(((*ctxt).elemNr - 1 as std::os::raw::c_int) as
                                        isize)
    } else { (*ctxt).elem = 0 as xmlRegExecCtxtPtr }
    return ret;
}
/* *
 * xmlRelaxNGValidateProgressiveCallback:
 * @exec:  the regular expression instance
 * @token:  the token which matched
 * @transdata:  callback data, the define for the subelement if available
 @ @inputdata:  callback data, the Relax NG validation context
 *
 * Handle the callback and if needed validate the element children.
 * some of the in/out informations are passed via the context in @inputdata.
 */
unsafe extern "C" fn xmlRelaxNGValidateProgressiveCallback(mut exec:
                                                               xmlRegExecCtxtPtr,
                                                           mut token:
                                                               *const xmlChar,
                                                           mut transdata:
                                                               *mut std::os::raw::c_void,
                                                           mut inputdata:
                                                               *mut std::os::raw::c_void) {
    let mut ctxt: xmlRelaxNGValidCtxtPtr =
        inputdata as xmlRelaxNGValidCtxtPtr;
    let mut define: xmlRelaxNGDefinePtr = transdata as xmlRelaxNGDefinePtr;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut oldstate: xmlRelaxNGValidStatePtr =
        0 as *mut xmlRelaxNGValidState;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut oldflags: std::os::raw::c_int = 0;
    if ctxt.is_null() {
        fprintf(stderr,
                b"callback on %s missing context\n\x00" as *const u8 as
                    *const std::os::raw::c_char, token);
        return
    }
    node = (*ctxt).pnode;
    (*ctxt).pstate = 1 as std::os::raw::c_int;
    if define.is_null() {
        if *token.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '#' as i32 {
            return
        }
        fprintf(stderr,
                b"callback on %s missing define\n\x00" as *const u8 as
                    *const std::os::raw::c_char, token);
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as std::os::raw::c_int
        }
        (*ctxt).pstate = -(1 as std::os::raw::c_int);
        return
    }
    if ctxt.is_null() || define.is_null() {
        fprintf(stderr,
                b"callback on %s missing info\n\x00" as *const u8 as
                    *const std::os::raw::c_char, token);
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as std::os::raw::c_int
        }
        (*ctxt).pstate = -(1 as std::os::raw::c_int);
        return
    } else {
        if (*define).type_0 as std::os::raw::c_int !=
               XML_RELAXNG_ELEMENT as std::os::raw::c_int {
            fprintf(stderr,
                    b"callback on %s define is not element\n\x00" as *const u8
                        as *const std::os::raw::c_char, token);
            if (*ctxt).errNo == XML_RELAXNG_OK as std::os::raw::c_int {
                (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as std::os::raw::c_int
            }
            (*ctxt).pstate = -(1 as std::os::raw::c_int);
            return
        }
    }
    if (*node).type_0 as std::os::raw::c_uint !=
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOTELEM,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            xmlRelaxNGDumpValidError(ctxt);
        }
        (*ctxt).pstate = -(1 as std::os::raw::c_int);
        return
    }
    if (*define).contModel.is_null() {
        /*
         * this node cannot be validated in a streamable fashion
         */
        (*ctxt).pstate = 0 as std::os::raw::c_int;
        (*ctxt).pdef = define;
        return
    }
    exec =
        xmlRegNewExecCtxt((*define).contModel,
                          Some(xmlRelaxNGValidateProgressiveCallback as
                                   unsafe extern "C" fn(_: xmlRegExecCtxtPtr,
                                                        _: *const xmlChar,
                                                        _: *mut std::os::raw::c_void,
                                                        _: *mut std::os::raw::c_void)
                                       -> ()), ctxt as *mut std::os::raw::c_void);
    if exec.is_null() { (*ctxt).pstate = -(1 as std::os::raw::c_int); return }
    xmlRelaxNGElemPush(ctxt, exec);
    /*
     * Validate the attributes part of the content.
     */
    state = xmlRelaxNGNewValidState(ctxt, node);
    if state.is_null() { (*ctxt).pstate = -(1 as std::os::raw::c_int); return }
    oldstate = (*ctxt).state;
    (*ctxt).state = state;
    if !(*define).attrs.is_null() {
        ret = xmlRelaxNGValidateAttributeList(ctxt, (*define).attrs);
        if ret != 0 as std::os::raw::c_int {
            (*ctxt).pstate = -(1 as std::os::raw::c_int);
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ATTRVALID,
                                    (*node).name, 0 as *const xmlChar,
                                    0 as std::os::raw::c_int);
        }
    }
    if !(*ctxt).state.is_null() {
        (*(*ctxt).state).seq = 0 as xmlNodePtr;
        ret = xmlRelaxNGValidateElementEnd(ctxt, 1 as std::os::raw::c_int);
        if ret != 0 as std::os::raw::c_int { (*ctxt).pstate = -(1 as std::os::raw::c_int) }
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
    } else if !(*ctxt).states.is_null() {
        let mut tmp: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut i: std::os::raw::c_int = 0;
        oldflags = (*ctxt).flags;
        i = 0 as std::os::raw::c_int;
        while i < (*(*ctxt).states).nbState {
            state = *(*(*ctxt).states).tabState.offset(i as isize);
            (*ctxt).state = state;
            (*(*ctxt).state).seq = 0 as xmlNodePtr;
            if xmlRelaxNGValidateElementEnd(ctxt, 0 as std::os::raw::c_int) ==
                   0 as std::os::raw::c_int {
                tmp = 0 as std::os::raw::c_int;
                break ;
            } else { i += 1 }
        }
        if tmp != 0 as std::os::raw::c_int {
            /*
             * validation error, log the message for the "best" one
             */
            (*ctxt).flags |= 1 as std::os::raw::c_int;
            xmlRelaxNGLogBestError(ctxt);
        }
        i = 0 as std::os::raw::c_int;
        while i < (*(*ctxt).states).nbState {
            xmlRelaxNGFreeValidState(ctxt,
                                     *(*(*ctxt).states).tabState.offset(i as
                                                                            isize));
            i += 1
        }
        xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
        (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
        if ret == 0 as std::os::raw::c_int && tmp == -(1 as std::os::raw::c_int) {
            (*ctxt).pstate = -(1 as std::os::raw::c_int)
        }
        (*ctxt).flags = oldflags
    }
    if (*ctxt).pstate == -(1 as std::os::raw::c_int) {
        if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            xmlRelaxNGDumpValidError(ctxt);
        }
    }
    (*ctxt).state = oldstate;
}
/*
 * Interfaces for progressive validation when possible
 */
/* *
 * xmlRelaxNGValidatePushElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 *
 * Push a new element start on the RelaxNG validation stack.
 *
 * returns 1 if no validation problem was found or 0 if validating the
 *         element requires a full node, and -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidatePushElement(mut ctxt:
                                                           xmlRelaxNGValidCtxtPtr,
                                                       mut doc: xmlDocPtr,
                                                       mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if ctxt.is_null() || elem.is_null() { return -(1 as std::os::raw::c_int) }
    if (*ctxt).elem.is_null() {
        let mut schema: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
        let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
        let mut define: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        schema = (*ctxt).schema;
        if schema.is_null() {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOGRAMMAR,
                                    0 as *const xmlChar, 0 as *const xmlChar,
                                    0 as std::os::raw::c_int);
            return -(1 as std::os::raw::c_int)
        }
        grammar = (*schema).topgrammar;
        if grammar.is_null() || (*grammar).start.is_null() {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOGRAMMAR,
                                    0 as *const xmlChar, 0 as *const xmlChar,
                                    0 as std::os::raw::c_int);
            return -(1 as std::os::raw::c_int)
        }
        define = (*grammar).start;
        if (*define).contModel.is_null() {
            (*ctxt).pdef = define;
            return 0 as std::os::raw::c_int
        }
        exec =
            xmlRegNewExecCtxt((*define).contModel,
                              Some(xmlRelaxNGValidateProgressiveCallback as
                                       unsafe extern "C" fn(_:
                                                                xmlRegExecCtxtPtr,
                                                            _: *const xmlChar,
                                                            _:
                                                                *mut std::os::raw::c_void,
                                                            _:
                                                                *mut std::os::raw::c_void)
                                           -> ()), ctxt as *mut std::os::raw::c_void);
        if exec.is_null() { return -(1 as std::os::raw::c_int) }
        xmlRelaxNGElemPush(ctxt, exec);
    }
    (*ctxt).pnode = elem;
    (*ctxt).pstate = 0 as std::os::raw::c_int;
    if !(*elem).ns.is_null() {
        ret =
            xmlRegExecPushString2((*ctxt).elem, (*elem).name,
                                  (*(*elem).ns).href,
                                  ctxt as *mut std::os::raw::c_void)
    } else {
        ret =
            xmlRegExecPushString((*ctxt).elem, (*elem).name,
                                 ctxt as *mut std::os::raw::c_void)
    }
    if ret < 0 as std::os::raw::c_int {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMWRONG, (*elem).name,
                                0 as *const xmlChar, 0 as std::os::raw::c_int);
    } else if (*ctxt).pstate == 0 as std::os::raw::c_int {
        ret = 0 as std::os::raw::c_int
    } else if (*ctxt).pstate < 0 as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    } else { ret = 1 as std::os::raw::c_int }
    return ret;
}
/* *
 * xmlRelaxNGValidatePushCData:
 * @ctxt:  the RelaxNG validation context
 * @data:  some character data read
 * @len:  the length of the data
 *
 * check the CData parsed for validation in the current stack
 *
 * returns 1 if no validation problem was found or -1 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidatePushCData(mut ctxt:
                                                         xmlRelaxNGValidCtxtPtr,
                                                     mut data: *const xmlChar,
                                                     mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if ctxt.is_null() || (*ctxt).elem.is_null() || data.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    while *data as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if !(*data as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                 0x9 as std::os::raw::c_int <= *data as std::os::raw::c_int &&
                     *data as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                 *data as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            break ;
        }
        data = data.offset(1)
    }
    if *data as std::os::raw::c_int == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    ret =
        xmlRegExecPushString((*ctxt).elem,
                             b"#text\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar, ctxt as *mut std::os::raw::c_void);
    if ret < 0 as std::os::raw::c_int {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_TEXTWRONG,
                                b" TODO \x00" as *const u8 as
                                    *const std::os::raw::c_char as *mut xmlChar,
                                0 as *const xmlChar, 0 as std::os::raw::c_int);
        return -(1 as std::os::raw::c_int)
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGValidatePopElement:
 * @ctxt:  the RelaxNG validation context
 * @doc:  a document instance
 * @elem:  an element instance
 *
 * Pop the element end from the RelaxNG validation stack.
 *
 * returns 1 if no validation problem was found or 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidatePopElement(mut ctxt:
                                                          xmlRelaxNGValidCtxtPtr,
                                                      mut doc: xmlDocPtr,
                                                      mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    if ctxt.is_null() || (*ctxt).elem.is_null() || elem.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    /*
     * verify that we reached a terminal state of the content model.
     */
    exec = xmlRelaxNGElemPop(ctxt);
    ret =
        xmlRegExecPushString(exec, 0 as *const xmlChar,
                             0 as *mut std::os::raw::c_void);
    if ret == 0 as std::os::raw::c_int {
        /*
         * TODO: get some of the names needed to exit the current state of exec
         */
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOELEM,
                                b"\x00" as *const u8 as *const std::os::raw::c_char as
                                    *mut xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        ret = -(1 as std::os::raw::c_int)
    } else if ret < 0 as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    } else { ret = 1 as std::os::raw::c_int }
    xmlRegFreeExecCtxt(exec);
    return ret;
}
/* *
 * xmlRelaxNGValidateFullElement:
 * @ctxt:  the validation context
 * @doc:  a document instance
 * @elem:  an element instance
 *
 * Validate a full subtree when xmlRelaxNGValidatePushElement() returned
 * 0 and the content of the node has been expanded.
 *
 * returns 1 if no validation problem was found or -1 in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidateFullElement(mut ctxt:
                                                           xmlRelaxNGValidCtxtPtr,
                                                       mut doc: xmlDocPtr,
                                                       mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    if ctxt.is_null() || (*ctxt).pdef.is_null() || elem.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    state = xmlRelaxNGNewValidState(ctxt, (*elem).parent);
    if state.is_null() { return -(1 as std::os::raw::c_int) }
    (*state).seq = elem;
    (*ctxt).state = state;
    (*ctxt).errNo = XML_RELAXNG_OK as std::os::raw::c_int;
    ret = xmlRelaxNGValidateDefinition(ctxt, (*ctxt).pdef);
    if ret != 0 as std::os::raw::c_int ||
           (*ctxt).errNo != XML_RELAXNG_OK as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    } else { ret = 1 as std::os::raw::c_int }
    xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
    (*ctxt).state = 0 as xmlRelaxNGValidStatePtr;
    return ret;
}
/* *
 * xmlRelaxNGSkipIgnored:
 * @ctxt:  a schema validation context
 * @node:  the top node.
 *
 * Skip ignorable nodes in that context
 *
 * Returns the new sibling or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGSkipIgnored(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                           mut node: xmlNodePtr)
 -> xmlNodePtr {
    /*
     * TODO complete and handle entities
     */
    while !node.is_null() &&
              ((*node).type_0 as std::os::raw::c_uint ==
                   XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_XINCLUDE_START as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_XINCLUDE_END as std::os::raw::c_int as std::os::raw::c_uint ||
                   ((*node).type_0 as std::os::raw::c_uint ==
                        XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                        (*node).type_0 as std::os::raw::c_uint ==
                            XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                std::os::raw::c_uint) &&
                       ((*ctxt).flags & 4 as std::os::raw::c_int != 0 ||
                            xmlRelaxNGIsBlank((*node).content) != 0)) {
        node = (*node).next
    }
    return node;
}
/* ***********************************************************************
 *									*
 *			Type library hooks				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGNormalize:
 * @ctxt:  a schema validation context
 * @str:  the string to normalize
 *
 * Implements the  normalizeWhiteSpace( s ) function from
 * section 6.2.9 of the spec
 *
 * Returns the new string or NULL in case of error.
 */
unsafe extern "C" fn xmlRelaxNGNormalize(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                         mut str: *const xmlChar)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut p: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    let mut len: std::os::raw::c_int = 0;
    if str.is_null() { return 0 as *mut xmlChar }
    tmp = str;
    while *tmp as std::os::raw::c_int != 0 as std::os::raw::c_int { tmp = tmp.offset(1) }
    len = tmp.offset_from(str) as std::os::raw::c_long as std::os::raw::c_int;
    ret =
        xmlMallocAtomic.expect("non-null function pointer")(((len +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong))
            as *mut xmlChar;
    if ret.is_null() {
        xmlRngVErrMemory(ctxt,
                         b"validating\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as *mut xmlChar
    }
    p = ret;
    while *str as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *str as std::os::raw::c_int &&
                  *str as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *str as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        str = str.offset(1)
    }
    while *str as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *str as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *str as std::os::raw::c_int &&
                   *str as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *str as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            while *str as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *str as std::os::raw::c_int &&
                          *str as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *str as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                str = str.offset(1)
            }
            if *str as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
            let fresh38 = p;
            p = p.offset(1);
            *fresh38 = ' ' as i32 as xmlChar
        } else {
            let fresh39 = str;
            str = str.offset(1);
            let fresh40 = p;
            p = p.offset(1);
            *fresh40 = *fresh39
        }
    }
    *p = 0 as std::os::raw::c_int as xmlChar;
    return ret;
}
/* *
 * xmlRelaxNGValidateDatatype:
 * @ctxt:  a Relax-NG validation context
 * @value:  the string value
 * @type:  the datatype definition
 * @node:  the node
 *
 * Validate the given value against the dataype
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateDatatype(mut ctxt:
                                                    xmlRelaxNGValidCtxtPtr,
                                                mut value: *const xmlChar,
                                                mut define:
                                                    xmlRelaxNGDefinePtr,
                                                mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut tmp: std::os::raw::c_int = 0;
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
    let mut result: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if define.is_null() || (*define).data.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    lib = (*define).data as xmlRelaxNGTypeLibraryPtr;
    if (*lib).check.is_some() {
        if !(*define).attrs.is_null() &&
               (*(*define).attrs).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_PARAM as std::os::raw::c_int {
            ret =
                (*lib).check.expect("non-null function pointer")((*lib).data,
                                                                 (*define).name,
                                                                 value,
                                                                 &mut result,
                                                                 node)
        } else {
            ret =
                (*lib).check.expect("non-null function pointer")((*lib).data,
                                                                 (*define).name,
                                                                 value,
                                                                 0 as
                                                                     *mut *mut std::os::raw::c_void,
                                                                 node)
        }
    } else { ret = -(1 as std::os::raw::c_int) }
    if ret < 0 as std::os::raw::c_int {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_TYPE, (*define).name,
                                0 as *const xmlChar, 0 as std::os::raw::c_int);
        if !result.is_null() && !lib.is_null() && (*lib).freef.is_some() {
            (*lib).freef.expect("non-null function pointer")((*lib).data,
                                                             result);
        }
        return -(1 as std::os::raw::c_int)
    } else {
        if ret == 1 as std::os::raw::c_int {
            ret = 0 as std::os::raw::c_int
        } else if ret == 2 as std::os::raw::c_int {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_DUPID, value,
                                    0 as *const xmlChar, 1 as std::os::raw::c_int);
        } else {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_TYPEVAL,
                                    (*define).name, value, 1 as std::os::raw::c_int);
            ret = -(1 as std::os::raw::c_int)
        }
    }
    cur = (*define).attrs;
    while ret == 0 as std::os::raw::c_int && !cur.is_null() &&
              (*cur).type_0 as std::os::raw::c_int == XML_RELAXNG_PARAM as std::os::raw::c_int
          {
        if (*lib).facet.is_some() {
            tmp =
                (*lib).facet.expect("non-null function pointer")((*lib).data,
                                                                 (*define).name,
                                                                 (*cur).name,
                                                                 (*cur).value,
                                                                 value,
                                                                 result);
            if tmp != 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
        }
        cur = (*cur).next
    }
    if ret == 0 as std::os::raw::c_int && !(*define).content.is_null() {
        let mut oldvalue: *const xmlChar = 0 as *const xmlChar;
        let mut oldendvalue: *const xmlChar = 0 as *const xmlChar;
        oldvalue = (*(*ctxt).state).value;
        oldendvalue = (*(*ctxt).state).endvalue;
        (*(*ctxt).state).value = value as *mut xmlChar;
        (*(*ctxt).state).endvalue = 0 as *mut xmlChar;
        ret = xmlRelaxNGValidateValue(ctxt, (*define).content);
        (*(*ctxt).state).value = oldvalue as *mut xmlChar;
        (*(*ctxt).state).endvalue = oldendvalue as *mut xmlChar
    }
    if !result.is_null() && !lib.is_null() && (*lib).freef.is_some() {
        (*lib).freef.expect("non-null function pointer")((*lib).data, result);
    }
    return ret;
}
/* *
 * xmlRelaxNGNextValue:
 * @ctxt:  a Relax-NG validation context
 *
 * Skip to the next value when validating within a list
 *
 * Returns 0 if the operation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGNextValue(mut ctxt: xmlRelaxNGValidCtxtPtr)
 -> std::os::raw::c_int {
    let mut cur: *mut xmlChar = 0 as *mut xmlChar;
    cur = (*(*ctxt).state).value;
    if cur.is_null() || (*(*ctxt).state).endvalue.is_null() {
        (*(*ctxt).state).value = 0 as *mut xmlChar;
        (*(*ctxt).state).endvalue = 0 as *mut xmlChar;
        return 0 as std::os::raw::c_int
    }
    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int { cur = cur.offset(1) }
    while cur != (*(*ctxt).state).endvalue &&
              *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
        cur = cur.offset(1)
    }
    if cur == (*(*ctxt).state).endvalue {
        (*(*ctxt).state).value = 0 as *mut xmlChar
    } else { (*(*ctxt).state).value = cur }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGValidateValueList:
 * @ctxt:  a Relax-NG validation context
 * @defines:  the list of definitions to verify
 *
 * Validate the given set of definitions for the current value
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateValueList(mut ctxt:
                                                     xmlRelaxNGValidCtxtPtr,
                                                 mut defines:
                                                     xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while !defines.is_null() {
        ret = xmlRelaxNGValidateValue(ctxt, defines);
        if ret != 0 as std::os::raw::c_int { break ; }
        defines = (*defines).next
    }
    return ret;
}
/* ***********************************************************************
 *									*
 *		Generic interpreted validation implementation		*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGValidateValue:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to verify
 *
 * Validate the given definition for the current value
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateValue(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                             mut define: xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut oldflags: std::os::raw::c_int = 0;
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    value = (*(*ctxt).state).value;
    let mut current_block_141: u64;
    match (*define).type_0 as std::os::raw::c_int {
        0 => {
            if !value.is_null() &&
                   *value.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                       0 as std::os::raw::c_int {
                let mut idx: std::os::raw::c_int = 0 as std::os::raw::c_int;
                while *value.offset(idx as isize) as std::os::raw::c_int ==
                          0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <=
                              *value.offset(idx as isize) as std::os::raw::c_int &&
                              *value.offset(idx as isize) as std::os::raw::c_int <=
                                  0xa as std::os::raw::c_int ||
                          *value.offset(idx as isize) as std::os::raw::c_int ==
                              0xd as std::os::raw::c_int {
                    idx += 1
                }
                if *value.offset(idx as isize) as std::os::raw::c_int !=
                       0 as std::os::raw::c_int {
                    ret = -(1 as std::os::raw::c_int)
                }
            }
            current_block_141 = 6328367678128271922;
        }
        3 => { current_block_141 = 6328367678128271922; }
        7 => {
            if xmlStrEqual(value, (*define).value) == 0 {
                if !(*define).name.is_null() {
                    let mut lib: xmlRelaxNGTypeLibraryPtr =
                        0 as *mut xmlRelaxNGTypeLibrary;
                    lib = (*define).data as xmlRelaxNGTypeLibraryPtr;
                    if !lib.is_null() && (*lib).comp.is_some() {
                        ret =
                            (*lib).comp.expect("non-null function pointer")((*lib).data,
                                                                            (*define).name,
                                                                            (*define).value,
                                                                            (*define).node,
                                                                            (*define).attrs
                                                                                as
                                                                                *mut std::os::raw::c_void,
                                                                            value,
                                                                            (*(*ctxt).state).node)
                    } else { ret = -(1 as std::os::raw::c_int) }
                    if ret < 0 as std::os::raw::c_int {
                        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_TYPECMP,
                                                (*define).name,
                                                0 as *const xmlChar,
                                                0 as std::os::raw::c_int);
                        return -(1 as std::os::raw::c_int)
                    } else {
                        if ret == 1 as std::os::raw::c_int {
                            ret = 0 as std::os::raw::c_int
                        } else { ret = -(1 as std::os::raw::c_int) }
                    }
                } else {
                    let mut nval: *mut xmlChar = 0 as *mut xmlChar;
                    let mut nvalue: *mut xmlChar = 0 as *mut xmlChar;
                    /*
                         * TODO: trivial optimizations are possible by
                         * computing at compile-time
                         */
                    nval = xmlRelaxNGNormalize(ctxt, (*define).value);
                    nvalue = xmlRelaxNGNormalize(ctxt, value);
                    if nval.is_null() || nvalue.is_null() ||
                           xmlStrEqual(nval, nvalue) == 0 {
                        ret = -(1 as std::os::raw::c_int)
                    }
                    if !nval.is_null() {
                        xmlFree.expect("non-null function pointer")(nval as
                                                                        *mut std::os::raw::c_void);
                    }
                    if !nvalue.is_null() {
                        xmlFree.expect("non-null function pointer")(nvalue as
                                                                        *mut std::os::raw::c_void);
                    }
                }
            }
            if ret == 0 as std::os::raw::c_int { xmlRelaxNGNextValue(ctxt); }
            current_block_141 = 6328367678128271922;
        }
        5 => {
            ret =
                xmlRelaxNGValidateDatatype(ctxt, value, define,
                                           (*(*ctxt).state).seq);
            if ret == 0 as std::os::raw::c_int { xmlRelaxNGNextValue(ctxt); }
            current_block_141 = 6328367678128271922;
        }
        17 => {
            let mut list: xmlRelaxNGDefinePtr = (*define).content;
            let mut oldvalue: *mut xmlChar = 0 as *mut xmlChar;
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as std::os::raw::c_int;
            oldvalue = (*(*ctxt).state).value;
            while !list.is_null() {
                ret = xmlRelaxNGValidateValue(ctxt, list);
                if ret == 0 as std::os::raw::c_int { break ; }
                (*(*ctxt).state).value = oldvalue;
                list = (*list).next
            }
            (*ctxt).flags = oldflags;
            if ret != 0 as std::os::raw::c_int {
                if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*ctxt).errNr > 0 as std::os::raw::c_int {
                xmlRelaxNGPopErrors(ctxt, 0 as std::os::raw::c_int);
            }
            current_block_141 = 6328367678128271922;
        }
        8 => {
            let mut list_0: xmlRelaxNGDefinePtr = (*define).content;
            let mut oldvalue_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut oldend: *mut xmlChar = 0 as *mut xmlChar;
            let mut val: *mut xmlChar = 0 as *mut xmlChar;
            let mut cur: *mut xmlChar = 0 as *mut xmlChar;
            oldvalue_0 = (*(*ctxt).state).value;
            oldend = (*(*ctxt).state).endvalue;
            val = xmlStrdup(oldvalue_0);
            if val.is_null() {
                val =
                    xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar)
            }
            if val.is_null() {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOSTATE,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                return -(1 as std::os::raw::c_int)
            }
            cur = val;
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                if *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                           *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                       *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    *cur = 0 as std::os::raw::c_int as xmlChar;
                    cur = cur.offset(1);
                    while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                  *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                              *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        let fresh41 = cur;
                        cur = cur.offset(1);
                        *fresh41 = 0 as std::os::raw::c_int as xmlChar
                    }
                } else { cur = cur.offset(1) }
            }
            (*(*ctxt).state).endvalue = cur;
            cur = val;
            while *cur as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                      cur != (*(*ctxt).state).endvalue {
                cur = cur.offset(1)
            }
            (*(*ctxt).state).value = cur;
            while !list_0.is_null() {
                if (*(*ctxt).state).value == (*(*ctxt).state).endvalue {
                    (*(*ctxt).state).value = 0 as *mut xmlChar
                }
                ret = xmlRelaxNGValidateValue(ctxt, list_0);
                if ret != 0 as std::os::raw::c_int { break ; }
                list_0 = (*list_0).next
            }
            if ret == 0 as std::os::raw::c_int && !(*(*ctxt).state).value.is_null() &&
                   (*(*ctxt).state).value != (*(*ctxt).state).endvalue {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_LISTEXTRA,
                                        (*(*ctxt).state).value,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                ret = -(1 as std::os::raw::c_int)
            }
            xmlFree.expect("non-null function pointer")(val as
                                                            *mut std::os::raw::c_void);
            (*(*ctxt).state).value = oldvalue_0;
            (*(*ctxt).state).endvalue = oldend;
            current_block_141 = 6328367678128271922;
        }
        16 => {
            ret = xmlRelaxNGValidateValueList(ctxt, (*define).content);
            if ret != 0 as std::os::raw::c_int {
                current_block_141 = 6328367678128271922;
            } else { current_block_141 = 9521147444787763968; }
        }
        15 => { current_block_141 = 9521147444787763968; }
        14 => {
            let mut temp_0: *mut xmlChar = 0 as *mut xmlChar;
            if (*(*ctxt).state).value.is_null() ||
                   *(*(*ctxt).state).value as std::os::raw::c_int == 0 as std::os::raw::c_int
               {
                ret = 0 as std::os::raw::c_int
            } else {
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as std::os::raw::c_int;
                temp_0 = (*(*ctxt).state).value;
                ret = xmlRelaxNGValidateValue(ctxt, (*define).content);
                (*ctxt).flags = oldflags;
                if ret != 0 as std::os::raw::c_int {
                    (*(*ctxt).state).value = temp_0;
                    if (*ctxt).errNr > 0 as std::os::raw::c_int {
                        xmlRelaxNGPopErrors(ctxt, 0 as std::os::raw::c_int);
                    }
                    ret = 0 as std::os::raw::c_int
                } else if (*ctxt).errNr > 0 as std::os::raw::c_int {
                    xmlRelaxNGPopErrors(ctxt, 0 as std::os::raw::c_int);
                }
            }
            current_block_141 = 6328367678128271922;
        }
        2 => {
            let mut list_1: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            list_1 = (*define).content;
            while !list_1.is_null() {
                ret = xmlRelaxNGValidateValue(ctxt, list_1);
                if ret == 0 as std::os::raw::c_int {
                    ret = -(1 as std::os::raw::c_int);
                    break ;
                } else { ret = 0 as std::os::raw::c_int; list_1 = (*list_1).next }
            }
            current_block_141 = 6328367678128271922;
        }
        10 | 18 => {
            let mut list_2: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            list_2 = (*define).content;
            while !list_2.is_null() {
                ret = xmlRelaxNGValidateValue(ctxt, list_2);
                if ret != 0 as std::os::raw::c_int {
                    ret = -(1 as std::os::raw::c_int);
                    break ;
                } else { ret = 0 as std::os::raw::c_int; list_2 = (*list_2).next }
            }
            current_block_141 = 6328367678128271922;
        }
        11 | 13 => {
            if (*define).content.is_null() {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NODEFINE,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                ret = -(1 as std::os::raw::c_int)
            } else { ret = xmlRelaxNGValidateValue(ctxt, (*define).content) }
            current_block_141 = 6328367678128271922;
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       8988 as
                                                                           std::os::raw::c_int);
            ret = -(1 as std::os::raw::c_int);
            current_block_141 = 6328367678128271922;
        }
    }
    match current_block_141 {
        9521147444787763968 =>
        /* Falls through. */
        {
            let mut cur_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut temp: *mut xmlChar = 0 as *mut xmlChar;
            if (*(*ctxt).state).value.is_null() ||
                   *(*(*ctxt).state).value as std::os::raw::c_int == 0 as std::os::raw::c_int
               {
                ret = 0 as std::os::raw::c_int
            } else {
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as std::os::raw::c_int;
                cur_0 = (*(*ctxt).state).value;
                temp = 0 as *mut xmlChar;
                while !cur_0.is_null() && cur_0 != (*(*ctxt).state).endvalue
                          && temp != cur_0 {
                    temp = cur_0;
                    ret =
                        xmlRelaxNGValidateValueList(ctxt, (*define).content);
                    if ret != 0 as std::os::raw::c_int {
                        (*(*ctxt).state).value = temp;
                        ret = 0 as std::os::raw::c_int;
                        break ;
                    } else { cur_0 = (*(*ctxt).state).value }
                }
                (*ctxt).flags = oldflags;
                if (*ctxt).errNr > 0 as std::os::raw::c_int {
                    xmlRelaxNGPopErrors(ctxt, 0 as std::os::raw::c_int);
                }
            }
        }
        _ => { }
    }
    return ret;
}
/* *
 * xmlRelaxNGValidateValueContent:
 * @ctxt:  a Relax-NG validation context
 * @defines:  the list of definitions to verify
 *
 * Validate the given definitions for the current value
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateValueContent(mut ctxt:
                                                        xmlRelaxNGValidCtxtPtr,
                                                    mut defines:
                                                        xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while !defines.is_null() {
        ret = xmlRelaxNGValidateValue(ctxt, defines);
        if ret != 0 as std::os::raw::c_int { break ; }
        defines = (*defines).next
    }
    return ret;
}
/* *
 * xmlRelaxNGAttributeMatch:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to check
 * @prop:  the attribute
 *
 * Check if the attribute matches the definition nameClass
 *
 * Returns 1 if the attribute matches, 0 if no, or -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGAttributeMatch(mut ctxt:
                                                  xmlRelaxNGValidCtxtPtr,
                                              mut define: xmlRelaxNGDefinePtr,
                                              mut prop: xmlAttrPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if !(*define).name.is_null() {
        if xmlStrEqual((*define).name, (*prop).name) == 0 {
            return 0 as std::os::raw::c_int
        }
    }
    if !(*define).ns.is_null() {
        if *(*define).ns.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
            if !(*prop).ns.is_null() { return 0 as std::os::raw::c_int }
        } else if (*prop).ns.is_null() ||
                      xmlStrEqual((*define).ns, (*(*prop).ns).href) == 0 {
            return 0 as std::os::raw::c_int
        }
    }
    if (*define).nameClass.is_null() { return 1 as std::os::raw::c_int }
    define = (*define).nameClass;
    if (*define).type_0 as std::os::raw::c_int == XML_RELAXNG_EXCEPT as std::os::raw::c_int {
        let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        list = (*define).content;
        while !list.is_null() {
            ret = xmlRelaxNGAttributeMatch(ctxt, list, prop);
            if ret == 1 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
            if ret < 0 as std::os::raw::c_int { return ret }
            list = (*list).next
        }
    } else if (*define).type_0 as std::os::raw::c_int ==
                  XML_RELAXNG_CHOICE as std::os::raw::c_int {
        let mut list_0: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        list_0 = (*define).nameClass;
        while !list_0.is_null() {
            ret = xmlRelaxNGAttributeMatch(ctxt, list_0, prop);
            if ret == 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
            if ret < 0 as std::os::raw::c_int { return ret }
            list_0 = (*list_0).next
        }
        return 0 as std::os::raw::c_int
    } else {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"relaxng.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   9076 as
                                                                       std::os::raw::c_int);
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGValidateAttribute:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to verify
 *
 * Validate the given attribute definition for that node
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateAttribute(mut ctxt:
                                                     xmlRelaxNGValidCtxtPtr,
                                                 mut define:
                                                     xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    let mut oldvalue: *mut xmlChar = 0 as *mut xmlChar;
    let mut prop: xmlAttrPtr = 0 as xmlAttrPtr;
    let mut tmp: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut oldseq: xmlNodePtr = 0 as *mut xmlNode;
    if (*(*ctxt).state).nbAttrLeft <= 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if !(*define).name.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < (*(*ctxt).state).nbAttrs {
            tmp = *(*(*ctxt).state).attrs.offset(i as isize);
            if !tmp.is_null() && xmlStrEqual((*define).name, (*tmp).name) != 0
               {
                if ((*define).ns.is_null() ||
                        *(*define).ns.offset(0 as std::os::raw::c_int as isize) as
                            std::os::raw::c_int == 0 as std::os::raw::c_int) &&
                       (*tmp).ns.is_null() ||
                       !(*tmp).ns.is_null() &&
                           xmlStrEqual((*define).ns, (*(*tmp).ns).href) != 0 {
                    prop = tmp;
                    break ;
                }
            }
            i += 1
        }
        if !prop.is_null() {
            value =
                xmlNodeListGetString((*prop).doc, (*prop).children,
                                     1 as std::os::raw::c_int);
            oldvalue = (*(*ctxt).state).value;
            oldseq = (*(*ctxt).state).seq;
            (*(*ctxt).state).seq = prop as xmlNodePtr;
            (*(*ctxt).state).value = value;
            (*(*ctxt).state).endvalue = 0 as *mut xmlChar;
            ret = xmlRelaxNGValidateValueContent(ctxt, (*define).content);
            if !(*(*ctxt).state).value.is_null() {
                value = (*(*ctxt).state).value
            }
            if !value.is_null() {
                xmlFree.expect("non-null function pointer")(value as
                                                                *mut std::os::raw::c_void);
            }
            (*(*ctxt).state).value = oldvalue;
            (*(*ctxt).state).seq = oldseq;
            if ret == 0 as std::os::raw::c_int {
                /*
                 * flag the attribute as processed
                 */
                let ref mut fresh42 =
                    *(*(*ctxt).state).attrs.offset(i as isize);
                *fresh42 = 0 as xmlAttrPtr;
                (*(*ctxt).state).nbAttrLeft -= 1
            }
        } else { ret = -(1 as std::os::raw::c_int) }
    } else {
        i = 0 as std::os::raw::c_int;
        while i < (*(*ctxt).state).nbAttrs {
            tmp = *(*(*ctxt).state).attrs.offset(i as isize);
            if !tmp.is_null() &&
                   xmlRelaxNGAttributeMatch(ctxt, define, tmp) ==
                       1 as std::os::raw::c_int {
                prop = tmp;
                break ;
            } else { i += 1 }
        }
        if !prop.is_null() {
            value =
                xmlNodeListGetString((*prop).doc, (*prop).children,
                                     1 as std::os::raw::c_int);
            oldvalue = (*(*ctxt).state).value;
            oldseq = (*(*ctxt).state).seq;
            (*(*ctxt).state).seq = prop as xmlNodePtr;
            (*(*ctxt).state).value = value;
            ret = xmlRelaxNGValidateValueContent(ctxt, (*define).content);
            if !(*(*ctxt).state).value.is_null() {
                value = (*(*ctxt).state).value
            }
            if !value.is_null() {
                xmlFree.expect("non-null function pointer")(value as
                                                                *mut std::os::raw::c_void);
            }
            (*(*ctxt).state).value = oldvalue;
            (*(*ctxt).state).seq = oldseq;
            if ret == 0 as std::os::raw::c_int {
                /*
                 * flag the attribute as processed
                 */
                let ref mut fresh43 =
                    *(*(*ctxt).state).attrs.offset(i as isize);
                *fresh43 = 0 as xmlAttrPtr;
                (*(*ctxt).state).nbAttrLeft -= 1
            }
        } else { ret = -(1 as std::os::raw::c_int) }
    }
    return ret;
}
/* ***********************************************************************
 *									*
 *		Progressive validation of when possible			*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGValidateAttributeList:
 * @ctxt:  a Relax-NG validation context
 * @define:  the list of definition to verify
 *
 * Validate the given node against the list of attribute definitions
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateAttributeList(mut ctxt:
                                                         xmlRelaxNGValidCtxtPtr,
                                                     mut defines:
                                                         xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut res: std::os::raw::c_int = 0;
    let mut needmore: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    cur = defines;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_int ==
               XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
            if xmlRelaxNGValidateAttribute(ctxt, cur) != 0 as std::os::raw::c_int {
                ret = -(1 as std::os::raw::c_int)
            }
        } else { needmore = 1 as std::os::raw::c_int }
        cur = (*cur).next
    }
    if needmore == 0 { return ret }
    cur = defines;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_int !=
               XML_RELAXNG_ATTRIBUTE as std::os::raw::c_int {
            if !(*ctxt).state.is_null() || !(*ctxt).states.is_null() {
                res = xmlRelaxNGValidateDefinition(ctxt, cur);
                if res < 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
            } else {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOSTATE,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                return -(1 as std::os::raw::c_int)
            }
            if res == -(1 as std::os::raw::c_int) { break ; }
        }
        cur = (*cur).next
    }
    return ret;
}
/* *
 * xmlRelaxNGNodeMatchesList:
 * @node:  the node
 * @list:  a NULL terminated array of definitions
 *
 * Check if a node can be matched by one of the definitions
 *
 * Returns 1 if matches 0 otherwise
 */
unsafe extern "C" fn xmlRelaxNGNodeMatchesList(mut node: xmlNodePtr,
                                               mut list:
                                                   *mut xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tmp: std::os::raw::c_int = 0;
    if node.is_null() || list.is_null() { return 0 as std::os::raw::c_int }
    let fresh44 = i;
    i = i + 1;
    cur = *list.offset(fresh44 as isize);
    while !cur.is_null() {
        if (*node).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*cur).type_0 as std::os::raw::c_int ==
                   XML_RELAXNG_ELEMENT as std::os::raw::c_int {
            tmp =
                xmlRelaxNGElementMatch(0 as xmlRelaxNGValidCtxtPtr, cur,
                                       node);
            if tmp == 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
        } else if ((*node).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*node).type_0 as std::os::raw::c_uint ==
                           XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint) &&
                      (*cur).type_0 as std::os::raw::c_int ==
                          XML_RELAXNG_TEXT as std::os::raw::c_int {
            return 1 as std::os::raw::c_int
        }
        let fresh45 = i;
        i = i + 1;
        cur = *list.offset(fresh45 as isize)
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGValidateInterleave:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to verify
 *
 * Validate an interleave definition for a node.
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateInterleave(mut ctxt:
                                                      xmlRelaxNGValidCtxtPtr,
                                                  mut define:
                                                      xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut nbgroups: std::os::raw::c_int = 0;
    let mut errNr: std::os::raw::c_int = (*ctxt).errNr;
    let mut oldflags: std::os::raw::c_int = 0;
    let mut oldstate: xmlRelaxNGValidStatePtr =
        0 as *mut xmlRelaxNGValidState;
    let mut partitions: xmlRelaxNGPartitionPtr =
        0 as *mut xmlRelaxNGPartition;
    let mut group: xmlRelaxNGInterleaveGroupPtr =
        0 as xmlRelaxNGInterleaveGroupPtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut start: xmlNodePtr = 0 as *mut xmlNode;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut lastchg: xmlNodePtr = 0 as xmlNodePtr;
    let mut lastelem: xmlNodePtr = 0 as *mut xmlNode;
    let mut list: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
    let mut lasts: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
    if !(*define).data.is_null() {
        partitions = (*define).data as xmlRelaxNGPartitionPtr;
        nbgroups = (*partitions).nbgroups
    } else {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_INTERNODATA,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Optimizations for MIXED
     */
    oldflags = (*ctxt).flags;
    if (*define).dflags as std::os::raw::c_int &
           (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
        (*ctxt).flags |= 4 as std::os::raw::c_int;
        if nbgroups == 2 as std::os::raw::c_int {
            /*
             * this is a pure <mixed> case
             */
            if !(*ctxt).state.is_null() {
                (*(*ctxt).state).seq =
                    xmlRelaxNGSkipIgnored(ctxt, (*(*ctxt).state).seq)
            }
            if (*(**(*partitions).groups.offset(0 as std::os::raw::c_int as
                                                    isize)).rule).type_0 as
                   std::os::raw::c_int == XML_RELAXNG_TEXT as std::os::raw::c_int {
                ret =
                    xmlRelaxNGValidateDefinition(ctxt,
                                                 (**(*partitions).groups.offset(1
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    isize)).rule)
            } else {
                ret =
                    xmlRelaxNGValidateDefinition(ctxt,
                                                 (**(*partitions).groups.offset(0
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    isize)).rule)
            }
            if ret == 0 as std::os::raw::c_int {
                if !(*ctxt).state.is_null() {
                    (*(*ctxt).state).seq =
                        xmlRelaxNGSkipIgnored(ctxt, (*(*ctxt).state).seq)
                }
            }
            (*ctxt).flags = oldflags;
            return ret
        }
    }
    /*
     * Build arrays to store the first and last node of the chain
     * pertaining to each group
     */
    list =
        xmlMalloc.expect("non-null function pointer")((nbgroups as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut xmlNodePtr;
    if list.is_null() {
        xmlRngVErrMemory(ctxt,
                         b"validating\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    memset(list as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           (nbgroups as
                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                as std::os::raw::c_ulong));
    lasts =
        xmlMalloc.expect("non-null function pointer")((nbgroups as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut xmlNodePtr;
    if lasts.is_null() {
        xmlRngVErrMemory(ctxt,
                         b"validating\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    memset(lasts as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           (nbgroups as
                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlNodePtr>()
                                                as std::os::raw::c_ulong));
    /*
     * Walk the sequence of children finding the right group and
     * sorting them in sequences.
     */
    cur = (*(*ctxt).state).seq;
    cur = xmlRelaxNGSkipIgnored(ctxt, cur);
    start = cur;
    while !cur.is_null() {
        (*(*ctxt).state).seq = cur;
        if !(*partitions).triage.is_null() &&
               (*partitions).flags & 1 as std::os::raw::c_int != 0 {
            let mut tmp: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
            if (*cur).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                   (*cur).type_0 as std::os::raw::c_uint ==
                       XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                tmp =
                    xmlHashLookup2((*partitions).triage,
                                   b"#text\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar,
                                   0 as *const xmlChar)
            } else if (*cur).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                if !(*cur).ns.is_null() {
                    tmp =
                        xmlHashLookup2((*partitions).triage, (*cur).name,
                                       (*(*cur).ns).href);
                    if tmp.is_null() {
                        tmp =
                            xmlHashLookup2((*partitions).triage,
                                           b"#any\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar,
                                           (*(*cur).ns).href)
                    }
                } else {
                    tmp =
                        xmlHashLookup2((*partitions).triage, (*cur).name,
                                       0 as *const xmlChar)
                }
                if tmp.is_null() {
                    tmp =
                        xmlHashLookup2((*partitions).triage,
                                       b"#any\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, 0 as *const xmlChar)
                }
            }
            if tmp.is_null() {
                i = nbgroups
            } else {
                i =
                    (tmp as ptrdiff_t - 1 as std::os::raw::c_int as std::os::raw::c_long) as
                        std::os::raw::c_int;
                if (*partitions).flags & 2 as std::os::raw::c_int != 0 {
                    group = *(*partitions).groups.offset(i as isize);
                    if xmlRelaxNGNodeMatchesList(cur, (*group).defs) == 0 {
                        i = nbgroups
                    }
                }
            }
        } else {
            i = 0 as std::os::raw::c_int;
            while i < nbgroups {
                group = *(*partitions).groups.offset(i as isize);
                if !group.is_null() {
                    if xmlRelaxNGNodeMatchesList(cur, (*group).defs) != 0 {
                        break ;
                    }
                }
                i += 1
            }
        }
        /*
         * We break as soon as an element not matched is found
         */
        if i >= nbgroups { break ; }
        if !(*lasts.offset(i as isize)).is_null() {
            let ref mut fresh46 = (**lasts.offset(i as isize)).next;
            *fresh46 = cur;
            let ref mut fresh47 = *lasts.offset(i as isize);
            *fresh47 = cur
        } else {
            let ref mut fresh48 = *list.offset(i as isize);
            *fresh48 = cur;
            let ref mut fresh49 = *lasts.offset(i as isize);
            *fresh49 = cur
        }
        if !(*cur).next.is_null() {
            lastchg = (*cur).next
        } else { lastchg = cur }
        cur = xmlRelaxNGSkipIgnored(ctxt, (*cur).next)
    }
    if ret != 0 as std::os::raw::c_int {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_INTERSEQ,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        ret = -(1 as std::os::raw::c_int)
    } else {
        lastelem = cur;
        oldstate = (*ctxt).state;
        i = 0 as std::os::raw::c_int;
        loop  {
            if !(i < nbgroups) {
                current_block = 5482373152242628851;
                break ;
            }
            (*ctxt).state = xmlRelaxNGCopyValidState(ctxt, oldstate);
            if (*ctxt).state.is_null() {
                ret = -(1 as std::os::raw::c_int);
                current_block = 5482373152242628851;
                break ;
            } else {
                group = *(*partitions).groups.offset(i as isize);
                if !(*lasts.offset(i as isize)).is_null() {
                    last = (**lasts.offset(i as isize)).next;
                    let ref mut fresh50 = (**lasts.offset(i as isize)).next;
                    *fresh50 = 0 as *mut _xmlNode
                }
                (*(*ctxt).state).seq = *list.offset(i as isize);
                ret = xmlRelaxNGValidateDefinition(ctxt, (*group).rule);
                if ret != 0 as std::os::raw::c_int {
                    current_block = 5482373152242628851;
                    break ;
                }
                if !(*ctxt).state.is_null() {
                    cur = (*(*ctxt).state).seq;
                    cur = xmlRelaxNGSkipIgnored(ctxt, cur);
                    xmlRelaxNGFreeValidState(ctxt, oldstate);
                    oldstate = (*ctxt).state;
                    (*ctxt).state = 0 as xmlRelaxNGValidStatePtr;
                    if !cur.is_null() {
                        xmlRelaxNGAddValidError(ctxt,
                                                XML_RELAXNG_ERR_INTEREXTRA,
                                                (*cur).name,
                                                0 as *const xmlChar,
                                                0 as std::os::raw::c_int);
                        ret = -(1 as std::os::raw::c_int);
                        (*ctxt).state = oldstate;
                        current_block = 7672226494481485911;
                        break ;
                    }
                } else if !(*ctxt).states.is_null() {
                    let mut j: std::os::raw::c_int = 0;
                    let mut found: std::os::raw::c_int = 0 as std::os::raw::c_int;
                    let mut best: std::os::raw::c_int = -(1 as std::os::raw::c_int);
                    let mut lowattr: std::os::raw::c_int = -(1 as std::os::raw::c_int);
                    /*
	     * PBM: what happen if there is attributes checks in the interleaves
	     */
                    j = 0 as std::os::raw::c_int;
                    while j < (*(*ctxt).states).nbState {
                        cur =
                            (**(*(*ctxt).states).tabState.offset(j as
                                                                     isize)).seq;
                        cur = xmlRelaxNGSkipIgnored(ctxt, cur);
                        if cur.is_null() {
                            if found == 0 as std::os::raw::c_int {
                                lowattr =
                                    (**(*(*ctxt).states).tabState.offset(j as
                                                                             isize)).nbAttrLeft;
                                best = j
                            }
                            found = 1 as std::os::raw::c_int;
                            if (**(*(*ctxt).states).tabState.offset(j as
                                                                        isize)).nbAttrLeft
                                   <= lowattr {
                                /* try  to keep the latest one to mach old heuristic */
                                lowattr =
                                    (**(*(*ctxt).states).tabState.offset(j as
                                                                             isize)).nbAttrLeft;
                                best = j
                            }
                            if lowattr == 0 as std::os::raw::c_int { break ; }
                        } else if found == 0 as std::os::raw::c_int {
                            if lowattr == -(1 as std::os::raw::c_int) {
                                lowattr =
                                    (**(*(*ctxt).states).tabState.offset(j as
                                                                             isize)).nbAttrLeft;
                                best = j
                            } else if (**(*(*ctxt).states).tabState.offset(j
                                                                               as
                                                                               isize)).nbAttrLeft
                                          <= lowattr {
                                /* try  to keep the latest one to mach old heuristic */
                                lowattr =
                                    (**(*(*ctxt).states).tabState.offset(j as
                                                                             isize)).nbAttrLeft;
                                best = j
                            }
                        }
                        j += 1
                    }
                    /*
	     * BIG PBM: here we pick only one restarting point :-(
	     */
                    if (*(*ctxt).states).nbState > 0 as std::os::raw::c_int {
                        xmlRelaxNGFreeValidState(ctxt, oldstate);
                        if best != -(1 as std::os::raw::c_int) {
                            oldstate =
                                *(*(*ctxt).states).tabState.offset(best as
                                                                       isize);
                            let ref mut fresh51 =
                                *(*(*ctxt).states).tabState.offset(best as
                                                                       isize);
                            *fresh51 = 0 as xmlRelaxNGValidStatePtr
                        } else {
                            oldstate =
                                *(*(*ctxt).states).tabState.offset(((*(*ctxt).states).nbState
                                                                        -
                                                                        1 as
                                                                            std::os::raw::c_int)
                                                                       as
                                                                       isize);
                            let ref mut fresh52 =
                                *(*(*ctxt).states).tabState.offset(((*(*ctxt).states).nbState
                                                                        -
                                                                        1 as
                                                                            std::os::raw::c_int)
                                                                       as
                                                                       isize);
                            *fresh52 = 0 as xmlRelaxNGValidStatePtr;
                            (*(*ctxt).states).nbState -= 1
                        }
                    }
                    j = 0 as std::os::raw::c_int;
                    while j < (*(*ctxt).states).nbState {
                        xmlRelaxNGFreeValidState(ctxt,
                                                 *(*(*ctxt).states).tabState.offset(j
                                                                                        as
                                                                                        isize));
                        j += 1
                    }
                    xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                    (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
                    if found == 0 as std::os::raw::c_int {
                        if cur.is_null() {
                            xmlRelaxNGAddValidError(ctxt,
                                                    XML_RELAXNG_ERR_INTEREXTRA,
                                                    b"noname\x00" as *const u8
                                                        as *const std::os::raw::c_char
                                                        as *const xmlChar,
                                                    0 as *const xmlChar,
                                                    0 as std::os::raw::c_int);
                        } else {
                            xmlRelaxNGAddValidError(ctxt,
                                                    XML_RELAXNG_ERR_INTEREXTRA,
                                                    (*cur).name,
                                                    0 as *const xmlChar,
                                                    0 as std::os::raw::c_int);
                        }
                        ret = -(1 as std::os::raw::c_int);
                        (*ctxt).state = oldstate;
                        current_block = 7672226494481485911;
                        break ;
                    }
                } else {
                    ret = -(1 as std::os::raw::c_int);
                    current_block = 5482373152242628851;
                    break ;
                }
                if !(*lasts.offset(i as isize)).is_null() {
                    let ref mut fresh53 = (**lasts.offset(i as isize)).next;
                    *fresh53 = last
                }
                i += 1
            }
        }
        match current_block {
            7672226494481485911 => { }
            _ => {
                if !(*ctxt).state.is_null() {
                    xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                }
                (*ctxt).state = oldstate;
                (*(*ctxt).state).seq = lastelem;
                if ret != 0 as std::os::raw::c_int {
                    xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_INTERSEQ,
                                            0 as *const xmlChar,
                                            0 as *const xmlChar,
                                            0 as std::os::raw::c_int);
                    ret = -(1 as std::os::raw::c_int)
                }
            }
        }
    }
    (*ctxt).flags = oldflags;
    /*
     * builds the next links chain from the prev one
     */
    cur = lastchg;
    while !cur.is_null() {
        if cur == start || (*cur).prev.is_null() { break ; }
        (*(*cur).prev).next = cur;
        cur = (*cur).prev
    }
    if ret == 0 as std::os::raw::c_int {
        if (*ctxt).errNr > errNr { xmlRelaxNGPopErrors(ctxt, errNr); }
    }
    xmlFree.expect("non-null function pointer")(list as *mut std::os::raw::c_void);
    xmlFree.expect("non-null function pointer")(lasts as *mut std::os::raw::c_void);
    return ret;
}
/* *
 * xmlRelaxNGValidateDefinitionList:
 * @ctxt:  a Relax-NG validation context
 * @define:  the list of definition to verify
 *
 * Validate the given node content against the (list) of definitions
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateDefinitionList(mut ctxt:
                                                          xmlRelaxNGValidCtxtPtr,
                                                      mut defines:
                                                          xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut res: std::os::raw::c_int = 0;
    if defines.is_null() {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_INTERNAL,
                                b"NULL definition list\x00" as *const u8 as
                                    *const std::os::raw::c_char as *mut xmlChar,
                                0 as *const xmlChar, 0 as std::os::raw::c_int);
        return -(1 as std::os::raw::c_int)
    }
    while !defines.is_null() {
        if !(*ctxt).state.is_null() || !(*ctxt).states.is_null() {
            res = xmlRelaxNGValidateDefinition(ctxt, defines);
            if res < 0 as std::os::raw::c_int { ret = -(1 as std::os::raw::c_int) }
        } else {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOSTATE,
                                    0 as *const xmlChar, 0 as *const xmlChar,
                                    0 as std::os::raw::c_int);
            return -(1 as std::os::raw::c_int)
        }
        if res == -(1 as std::os::raw::c_int) { break ; }
        defines = (*defines).next
    }
    return ret;
}
/* *
 * xmlRelaxNGElementMatch:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to check
 * @elem:  the element
 *
 * Check if the element matches the definition nameClass
 *
 * Returns 1 if the element matches, 0 if no, or -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGElementMatch(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                            mut define: xmlRelaxNGDefinePtr,
                                            mut elem: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut oldflags: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !(*define).name.is_null() {
        if xmlStrEqual((*elem).name, (*define).name) == 0 {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMNAME,
                                    (*define).name, (*elem).name,
                                    0 as std::os::raw::c_int);
            return 0 as std::os::raw::c_int
        }
    }
    if !(*define).ns.is_null() &&
           *(*define).ns.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               0 as std::os::raw::c_int {
        if (*elem).ns.is_null() {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMNONS,
                                    (*elem).name, 0 as *const xmlChar,
                                    0 as std::os::raw::c_int);
            return 0 as std::os::raw::c_int
        } else {
            if xmlStrEqual((*(*elem).ns).href, (*define).ns) == 0 {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMWRONGNS,
                                        (*elem).name, (*define).ns,
                                        0 as std::os::raw::c_int);
                return 0 as std::os::raw::c_int
            }
        }
    } else if !(*elem).ns.is_null() && !(*define).ns.is_null() &&
                  (*define).name.is_null() {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMEXTRANS,
                                (*elem).name, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        return 0 as std::os::raw::c_int
    } else {
        if !(*elem).ns.is_null() && !(*define).name.is_null() {
            xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMEXTRANS,
                                    (*define).name, 0 as *const xmlChar,
                                    0 as std::os::raw::c_int);
            return 0 as std::os::raw::c_int
        }
    }
    if (*define).nameClass.is_null() { return 1 as std::os::raw::c_int }
    define = (*define).nameClass;
    if (*define).type_0 as std::os::raw::c_int == XML_RELAXNG_EXCEPT as std::os::raw::c_int {
        let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        if !ctxt.is_null() {
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as std::os::raw::c_int
        }
        list = (*define).content;
        while !list.is_null() {
            ret = xmlRelaxNGElementMatch(ctxt, list, elem);
            if ret == 1 as std::os::raw::c_int {
                if !ctxt.is_null() { (*ctxt).flags = oldflags }
                return 0 as std::os::raw::c_int
            }
            if ret < 0 as std::os::raw::c_int {
                if !ctxt.is_null() { (*ctxt).flags = oldflags }
                return ret
            }
            list = (*list).next
        }
        ret = 1 as std::os::raw::c_int;
        if !ctxt.is_null() { (*ctxt).flags = oldflags }
    } else if (*define).type_0 as std::os::raw::c_int ==
                  XML_RELAXNG_CHOICE as std::os::raw::c_int {
        let mut list_0: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        if !ctxt.is_null() {
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as std::os::raw::c_int
        }
        list_0 = (*define).nameClass;
        while !list_0.is_null() {
            ret = xmlRelaxNGElementMatch(ctxt, list_0, elem);
            if ret == 1 as std::os::raw::c_int {
                if !ctxt.is_null() { (*ctxt).flags = oldflags }
                return 1 as std::os::raw::c_int
            }
            if ret < 0 as std::os::raw::c_int {
                if !ctxt.is_null() { (*ctxt).flags = oldflags }
                return ret
            }
            list_0 = (*list_0).next
        }
        if !ctxt.is_null() {
            if ret != 0 as std::os::raw::c_int {
                if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*ctxt).errNr > 0 as std::os::raw::c_int {
                xmlRelaxNGPopErrors(ctxt, 0 as std::os::raw::c_int);
            }
        }
        ret = 0 as std::os::raw::c_int;
        if !ctxt.is_null() { (*ctxt).flags = oldflags }
    } else {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"relaxng.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   9714 as
                                                                       std::os::raw::c_int);
        ret = -(1 as std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlRelaxNGBestState:
 * @ctxt:  a Relax-NG validation context
 *
 * Find the "best" state in the ctxt->states list of states to report
 * errors about. I.e. a state with no element left in the child list
 * or the one with the less attributes left.
 * This is called only if a falidation error was detected
 *
 * Returns the index of the "best" state or -1 in case of error
 */
unsafe extern "C" fn xmlRelaxNGBestState(mut ctxt: xmlRelaxNGValidCtxtPtr)
 -> std::os::raw::c_int {
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut i: std::os::raw::c_int = 0;
    let mut tmp: std::os::raw::c_int = 0;
    let mut best: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut value: std::os::raw::c_int = 1000000 as std::os::raw::c_int;
    if ctxt.is_null() || (*ctxt).states.is_null() ||
           (*(*ctxt).states).nbState <= 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    i = 0 as std::os::raw::c_int;
    while i < (*(*ctxt).states).nbState {
        state = *(*(*ctxt).states).tabState.offset(i as isize);
        if !state.is_null() {
            if !(*state).seq.is_null() {
                if best == -(1 as std::os::raw::c_int) ||
                       value > 100000 as std::os::raw::c_int {
                    value = 100000 as std::os::raw::c_int;
                    best = i
                }
            } else {
                tmp = (*state).nbAttrLeft;
                if best == -(1 as std::os::raw::c_int) || value > tmp {
                    value = tmp;
                    best = i
                }
            }
        }
        i += 1
    }
    return best;
}
/* *
 * xmlRelaxNGLogBestError:
 * @ctxt:  a Relax-NG validation context
 *
 * Find the "best" state in the ctxt->states list of states to report
 * errors about and log it.
 */
unsafe extern "C" fn xmlRelaxNGLogBestError(mut ctxt:
                                                xmlRelaxNGValidCtxtPtr) {
    let mut best: std::os::raw::c_int = 0;
    if ctxt.is_null() || (*ctxt).states.is_null() ||
           (*(*ctxt).states).nbState <= 0 as std::os::raw::c_int {
        return
    }
    best = xmlRelaxNGBestState(ctxt);
    if best >= 0 as std::os::raw::c_int && best < (*(*ctxt).states).nbState {
        (*ctxt).state = *(*(*ctxt).states).tabState.offset(best as isize);
        xmlRelaxNGValidateElementEnd(ctxt, 1 as std::os::raw::c_int);
    };
}
/* *
 * xmlRelaxNGValidateElementEnd:
 * @ctxt:  a Relax-NG validation context
 * @dolog:  indicate that error logging should be done
 *
 * Validate the end of the element, implements check that
 * there is nothing left not consumed in the element content
 * or in the attribute list.
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateElementEnd(mut ctxt:
                                                      xmlRelaxNGValidCtxtPtr,
                                                  mut dolog: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    state = (*ctxt).state;
    if !(*state).seq.is_null() {
        (*state).seq = xmlRelaxNGSkipIgnored(ctxt, (*state).seq);
        if !(*state).seq.is_null() {
            if dolog != 0 {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_EXTRACONTENT,
                                        (*(*state).node).name,
                                        (*(*state).seq).name,
                                        0 as std::os::raw::c_int);
            }
            return -(1 as std::os::raw::c_int)
        }
    }
    i = 0 as std::os::raw::c_int;
    while i < (*state).nbAttrs {
        if !(*(*state).attrs.offset(i as isize)).is_null() {
            if dolog != 0 {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_INVALIDATTR,
                                        (**(*state).attrs.offset(i as
                                                                     isize)).name,
                                        (*(*state).node).name,
                                        0 as std::os::raw::c_int);
            }
            return -(1 as std::os::raw::c_int) - i
        }
        i += 1
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGValidateState:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to verify
 *
 * Validate the current state against the definition
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateState(mut ctxt: xmlRelaxNGValidCtxtPtr,
                                             mut define: xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut tmp: std::os::raw::c_int = 0;
    let mut oldflags: std::os::raw::c_int = 0;
    let mut errNr: std::os::raw::c_int = 0;
    let mut oldstate: xmlRelaxNGValidStatePtr = 0 as xmlRelaxNGValidStatePtr;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    if define.is_null() {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NODEFINE,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        return -(1 as std::os::raw::c_int)
    }
    if !(*ctxt).state.is_null() {
        node = (*(*ctxt).state).seq
    } else { node = 0 as xmlNodePtr }
    (*ctxt).depth += 1;
    let mut current_block_442: u64;
    match (*define).type_0 as std::os::raw::c_int {
        0 => {
            xmlRelaxNGSkipIgnored(ctxt, node);
            ret = 0 as std::os::raw::c_int;
            current_block_442 = 11850832584604245957;
        }
        1 => {
            ret = -(1 as std::os::raw::c_int);
            current_block_442 = 11850832584604245957;
        }
        3 => {
            while !node.is_null() &&
                      ((*node).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*node).type_0 as std::os::raw::c_uint ==
                               XML_COMMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                           ||
                           (*node).type_0 as std::os::raw::c_uint ==
                               XML_PI_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*node).type_0 as std::os::raw::c_uint ==
                               XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint) {
                node = (*node).next
            }
            (*(*ctxt).state).seq = node;
            current_block_442 = 11850832584604245957;
        }
        4 => {
            errNr = (*ctxt).errNr;
            node = xmlRelaxNGSkipIgnored(ctxt, node);
            if node.is_null() {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOELEM,
                                        (*define).name, 0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                ret = -(1 as std::os::raw::c_int);
                if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*node).type_0 as std::os::raw::c_uint !=
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOTELEM,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                ret = -(1 as std::os::raw::c_int);
                if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*node).psvi == define as *mut std::os::raw::c_void {
                (*(*ctxt).state).seq =
                    xmlRelaxNGSkipIgnored(ctxt, (*node).next);
                if (*ctxt).errNr > errNr { xmlRelaxNGPopErrors(ctxt, errNr); }
                if (*ctxt).errNr != 0 as std::os::raw::c_int {
                    while !(*ctxt).err.is_null() &&
                              ((*(*ctxt).err).err as std::os::raw::c_uint ==
                                   XML_RELAXNG_ERR_ELEMNAME as std::os::raw::c_int as
                                       std::os::raw::c_uint &&
                                   xmlStrEqual((*(*ctxt).err).arg2,
                                               (*node).name) != 0 ||
                                   (*(*ctxt).err).err as std::os::raw::c_uint ==
                                       XML_RELAXNG_ERR_ELEMEXTRANS as
                                           std::os::raw::c_int as std::os::raw::c_uint &&
                                       xmlStrEqual((*(*ctxt).err).arg1,
                                                   (*node).name) != 0 ||
                                   (*(*ctxt).err).err as std::os::raw::c_uint ==
                                       XML_RELAXNG_ERR_NOELEM as std::os::raw::c_int
                                           as std::os::raw::c_uint ||
                                   (*(*ctxt).err).err as std::os::raw::c_uint ==
                                       XML_RELAXNG_ERR_NOTELEM as std::os::raw::c_int
                                           as std::os::raw::c_uint) {
                        xmlRelaxNGValidErrorPop(ctxt);
                    }
                }
            } else {
                ret = xmlRelaxNGElementMatch(ctxt, define, node);
                if ret <= 0 as std::os::raw::c_int {
                    ret = -(1 as std::os::raw::c_int);
                    if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        xmlRelaxNGDumpValidError(ctxt);
                    }
                } else {
                    ret = 0 as std::os::raw::c_int;
                    if (*ctxt).errNr != 0 as std::os::raw::c_int {
                        if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                        while !(*ctxt).err.is_null() &&
                                  ((*(*ctxt).err).err as std::os::raw::c_uint ==
                                       XML_RELAXNG_ERR_ELEMNAME as std::os::raw::c_int
                                           as std::os::raw::c_uint &&
                                       xmlStrEqual((*(*ctxt).err).arg2,
                                                   (*node).name) != 0 ||
                                       (*(*ctxt).err).err as std::os::raw::c_uint ==
                                           XML_RELAXNG_ERR_ELEMEXTRANS as
                                               std::os::raw::c_int as std::os::raw::c_uint &&
                                           xmlStrEqual((*(*ctxt).err).arg1,
                                                       (*node).name) != 0 ||
                                       (*(*ctxt).err).err as std::os::raw::c_uint ==
                                           XML_RELAXNG_ERR_NOELEM as
                                               std::os::raw::c_int as std::os::raw::c_uint ||
                                       (*(*ctxt).err).err as std::os::raw::c_uint ==
                                           XML_RELAXNG_ERR_NOTELEM as
                                               std::os::raw::c_int as std::os::raw::c_uint) {
                            xmlRelaxNGValidErrorPop(ctxt);
                        }
                    }
                    errNr = (*ctxt).errNr;
                    oldflags = (*ctxt).flags;
                    if (*ctxt).flags & 4 as std::os::raw::c_int != 0 {
                        (*ctxt).flags -= 4 as std::os::raw::c_int
                    }
                    state = xmlRelaxNGNewValidState(ctxt, node);
                    if state.is_null() {
                        ret = -(1 as std::os::raw::c_int);
                        if (*ctxt).flags & 1 as std::os::raw::c_int ==
                               0 as std::os::raw::c_int {
                            xmlRelaxNGDumpValidError(ctxt);
                        }
                    } else {
                        oldstate = (*ctxt).state;
                        (*ctxt).state = state;
                        if !(*define).attrs.is_null() {
                            tmp =
                                xmlRelaxNGValidateAttributeList(ctxt,
                                                                (*define).attrs);
                            if tmp != 0 as std::os::raw::c_int {
                                ret = -(1 as std::os::raw::c_int);
                                xmlRelaxNGAddValidError(ctxt,
                                                        XML_RELAXNG_ERR_ATTRVALID,
                                                        (*node).name,
                                                        0 as *const xmlChar,
                                                        0 as std::os::raw::c_int);
                            }
                        }
                        if !(*define).contModel.is_null() {
                            let mut nstate: xmlRelaxNGValidStatePtr =
                                0 as *mut xmlRelaxNGValidState;
                            let mut tmpstate: xmlRelaxNGValidStatePtr =
                                (*ctxt).state;
                            let mut tmpstates: xmlRelaxNGStatesPtr =
                                (*ctxt).states;
                            let mut nseq: xmlNodePtr = 0 as *mut xmlNode;
                            nstate = xmlRelaxNGNewValidState(ctxt, node);
                            (*ctxt).state = nstate;
                            (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
                            tmp =
                                xmlRelaxNGValidateCompiledContent(ctxt,
                                                                  (*define).contModel,
                                                                  (*(*ctxt).state).seq);
                            nseq = (*(*ctxt).state).seq;
                            (*ctxt).state = tmpstate;
                            (*ctxt).states = tmpstates;
                            xmlRelaxNGFreeValidState(ctxt, nstate);
                            if tmp != 0 as std::os::raw::c_int {
                                ret = -(1 as std::os::raw::c_int)
                            }
                            if !(*ctxt).states.is_null() {
                                tmp = -(1 as std::os::raw::c_int);
                                i = 0 as std::os::raw::c_int;
                                while i < (*(*ctxt).states).nbState {
                                    state =
                                        *(*(*ctxt).states).tabState.offset(i
                                                                               as
                                                                               isize);
                                    (*ctxt).state = state;
                                    (*(*ctxt).state).seq = nseq;
                                    if xmlRelaxNGValidateElementEnd(ctxt,
                                                                    0 as
                                                                        std::os::raw::c_int)
                                           == 0 as std::os::raw::c_int {
                                        tmp = 0 as std::os::raw::c_int;
                                        break ;
                                    } else { i += 1 }
                                }
                                if tmp != 0 as std::os::raw::c_int {
                                    /*
             * This node was already validated successfully against
             * this definition.
             */
                                    /*
                         * validation error, log the message for the "best" one
                         */
                                    (*ctxt).flags |= 1 as std::os::raw::c_int;
                                    xmlRelaxNGLogBestError(ctxt);
                                }
                                i = 0 as std::os::raw::c_int;
                                while i < (*(*ctxt).states).nbState {
                                    xmlRelaxNGFreeValidState(ctxt,
                                                             *(*(*ctxt).states).tabState.offset(i
                                                                                                    as
                                                                                                    isize));
                                    i += 1
                                }
                                xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                                (*ctxt).flags = oldflags;
                                (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
                                if ret == 0 as std::os::raw::c_int &&
                                       tmp == -(1 as std::os::raw::c_int) {
                                    ret = -(1 as std::os::raw::c_int)
                                }
                            } else {
                                state = (*ctxt).state;
                                if !(*ctxt).state.is_null() {
                                    (*(*ctxt).state).seq = nseq
                                }
                                if ret == 0 as std::os::raw::c_int {
                                    ret =
                                        xmlRelaxNGValidateElementEnd(ctxt,
                                                                     1 as
                                                                         std::os::raw::c_int)
                                }
                                xmlRelaxNGFreeValidState(ctxt, state);
                            }
                        } else {
                            if !(*define).content.is_null() {
                                tmp =
                                    xmlRelaxNGValidateDefinitionList(ctxt,
                                                                     (*define).content);
                                if tmp != 0 as std::os::raw::c_int {
                                    ret = -(1 as std::os::raw::c_int);
                                    if (*ctxt).state.is_null() {
                                        (*ctxt).state = oldstate;
                                        xmlRelaxNGAddValidError(ctxt,
                                                                XML_RELAXNG_ERR_CONTENTVALID,
                                                                (*node).name,
                                                                0 as
                                                                    *const xmlChar,
                                                                0 as
                                                                    std::os::raw::c_int);
                                        (*ctxt).state =
                                            0 as xmlRelaxNGValidStatePtr
                                    } else {
                                        xmlRelaxNGAddValidError(ctxt,
                                                                XML_RELAXNG_ERR_CONTENTVALID,
                                                                (*node).name,
                                                                0 as
                                                                    *const xmlChar,
                                                                0 as
                                                                    std::os::raw::c_int);
                                    }
                                }
                            }
                            if !(*ctxt).states.is_null() {
                                tmp = -(1 as std::os::raw::c_int);
                                i = 0 as std::os::raw::c_int;
                                while i < (*(*ctxt).states).nbState {
                                    state =
                                        *(*(*ctxt).states).tabState.offset(i
                                                                               as
                                                                               isize);
                                    (*ctxt).state = state;
                                    if xmlRelaxNGValidateElementEnd(ctxt,
                                                                    0 as
                                                                        std::os::raw::c_int)
                                           == 0 as std::os::raw::c_int {
                                        tmp = 0 as std::os::raw::c_int;
                                        break ;
                                    } else { i += 1 }
                                }
                                if tmp != 0 as std::os::raw::c_int {
                                    /*
                         * validation error, log the message for the "best" one
                         */
                                    (*ctxt).flags |= 1 as std::os::raw::c_int;
                                    xmlRelaxNGLogBestError(ctxt);
                                }
                                i = 0 as std::os::raw::c_int;
                                while i < (*(*ctxt).states).nbState {
                                    xmlRelaxNGFreeValidState(ctxt,
                                                             *(*(*ctxt).states).tabState.offset(i
                                                                                                    as
                                                                                                    isize));
                                    let ref mut fresh54 =
                                        *(*(*ctxt).states).tabState.offset(i
                                                                               as
                                                                               isize);
                                    *fresh54 = 0 as xmlRelaxNGValidStatePtr;
                                    i += 1
                                }
                                xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                                (*ctxt).flags = oldflags;
                                (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
                                if ret == 0 as std::os::raw::c_int &&
                                       tmp == -(1 as std::os::raw::c_int) {
                                    ret = -(1 as std::os::raw::c_int)
                                }
                            } else {
                                state = (*ctxt).state;
                                if ret == 0 as std::os::raw::c_int {
                                    ret =
                                        xmlRelaxNGValidateElementEnd(ctxt,
                                                                     1 as
                                                                         std::os::raw::c_int)
                                }
                                xmlRelaxNGFreeValidState(ctxt, state);
                            }
                        }
                        if ret == 0 as std::os::raw::c_int {
                            (*node).psvi = define as *mut std::os::raw::c_void
                        }
                        (*ctxt).flags = oldflags;
                        (*ctxt).state = oldstate;
                        if !oldstate.is_null() {
                            (*oldstate).seq =
                                xmlRelaxNGSkipIgnored(ctxt, (*node).next)
                        }
                        if ret != 0 as std::os::raw::c_int {
                            if (*ctxt).flags & 1 as std::os::raw::c_int ==
                                   0 as std::os::raw::c_int {
                                xmlRelaxNGDumpValidError(ctxt);
                                ret = 0 as std::os::raw::c_int
                            }
                        } else if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                    }
                }
            }
            current_block_442 = 11850832584604245957;
        }
        14 => {
            errNr = (*ctxt).errNr;
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as std::os::raw::c_int;
            oldstate = xmlRelaxNGCopyValidState(ctxt, (*ctxt).state);
            ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
            if ret != 0 as std::os::raw::c_int {
                if !(*ctxt).state.is_null() {
                    xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                }
                (*ctxt).state = oldstate;
                (*ctxt).flags = oldflags;
                ret = 0 as std::os::raw::c_int;
                if (*ctxt).errNr > errNr { xmlRelaxNGPopErrors(ctxt, errNr); }
            } else {
                if !(*ctxt).states.is_null() {
                    xmlRelaxNGAddStates(ctxt, (*ctxt).states, oldstate);
                    current_block_442 = 6958737826328148217;
                } else {
                    (*ctxt).states =
                        xmlRelaxNGNewStates(ctxt, 1 as std::os::raw::c_int);
                    if (*ctxt).states.is_null() {
                        xmlRelaxNGFreeValidState(ctxt, oldstate);
                        (*ctxt).flags = oldflags;
                        ret = -(1 as std::os::raw::c_int);
                        if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                        current_block_442 = 11850832584604245957;
                    } else {
                        xmlRelaxNGAddStates(ctxt, (*ctxt).states, oldstate);
                        xmlRelaxNGAddStates(ctxt, (*ctxt).states,
                                            (*ctxt).state);
                        (*ctxt).state = 0 as xmlRelaxNGValidStatePtr;
                        current_block_442 = 6958737826328148217;
                    }
                }
                match current_block_442 {
                    11850832584604245957 => { }
                    _ => {
                        (*ctxt).flags = oldflags;
                        ret = 0 as std::os::raw::c_int;
                        if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                    }
                }
            }
            current_block_442 = 11850832584604245957;
        }
        16 => {
            errNr = (*ctxt).errNr;
            ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
            if ret != 0 as std::os::raw::c_int {
                current_block_442 = 11850832584604245957;
            } else {
                if (*ctxt).errNr > errNr { xmlRelaxNGPopErrors(ctxt, errNr); }
                current_block_442 = 8487579351791723214;
            }
        }
        15 => { current_block_442 = 8487579351791723214; }
        17 => {
            let mut list: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
            let mut states_0: xmlRelaxNGStatesPtr = 0 as xmlRelaxNGStatesPtr;
            node = xmlRelaxNGSkipIgnored(ctxt, node);
            errNr = (*ctxt).errNr;
            if (*define).dflags as std::os::raw::c_int &
                   (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 &&
                   !(*define).data.is_null() && !node.is_null() {
                /*
		     * node == NULL can't be optimized since IS_TRIABLE
		     * doesn't account for choice which may lead to
		     * only attributes.
		     */
                let mut triage: xmlHashTablePtr =
                    (*define).data as xmlHashTablePtr;
                /*
                     * Something we can optimize cleanly there is only one
                     * possble branch out !
                     */
                if (*node).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*node).type_0 as std::os::raw::c_uint ==
                           XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                    list =
                        xmlHashLookup2(triage,
                                       b"#text\x00" as *const u8 as
                                           *const std::os::raw::c_char as
                                           *mut xmlChar, 0 as *const xmlChar)
                            as xmlRelaxNGDefinePtr
                } else if (*node).type_0 as std::os::raw::c_uint ==
                              XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint
                 {
                    if !(*node).ns.is_null() {
                        list =
                            xmlHashLookup2(triage, (*node).name,
                                           (*(*node).ns).href) as
                                xmlRelaxNGDefinePtr;
                        if list.is_null() {
                            list =
                                xmlHashLookup2(triage,
                                               b"#any\x00" as *const u8 as
                                                   *const std::os::raw::c_char as
                                                   *mut xmlChar,
                                               (*(*node).ns).href) as
                                    xmlRelaxNGDefinePtr
                        }
                    } else {
                        list =
                            xmlHashLookup2(triage, (*node).name,
                                           0 as *const xmlChar) as
                                xmlRelaxNGDefinePtr
                    }
                    if list.is_null() {
                        list =
                            xmlHashLookup2(triage,
                                           b"#any\x00" as *const u8 as
                                               *const std::os::raw::c_char as
                                               *mut xmlChar,
                                           0 as *const xmlChar) as
                                xmlRelaxNGDefinePtr
                    }
                }
                if list.is_null() {
                    ret = -(1 as std::os::raw::c_int);
                    xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_ELEMWRONG,
                                            (*node).name, 0 as *const xmlChar,
                                            0 as std::os::raw::c_int);
                } else {
                    ret = xmlRelaxNGValidateDefinition(ctxt, list);
                    (ret) == 0 as std::os::raw::c_int;
                }
            } else {
                list = (*define).content;
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as std::os::raw::c_int;
                while !list.is_null() {
                    oldstate = xmlRelaxNGCopyValidState(ctxt, (*ctxt).state);
                    ret = xmlRelaxNGValidateDefinition(ctxt, list);
                    if ret == 0 as std::os::raw::c_int {
                        if states_0.is_null() {
                            states_0 =
                                xmlRelaxNGNewStates(ctxt, 1 as std::os::raw::c_int)
                        }
                        if !(*ctxt).state.is_null() {
                            xmlRelaxNGAddStates(ctxt, states_0,
                                                (*ctxt).state);
                        } else if !(*ctxt).states.is_null() {
                            i = 0 as std::os::raw::c_int;
                            while i < (*(*ctxt).states).nbState {
                                xmlRelaxNGAddStates(ctxt, states_0,
                                                    *(*(*ctxt).states).tabState.offset(i
                                                                                           as
                                                                                           isize));
                                i += 1
                            }
                            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                            (*ctxt).states = 0 as xmlRelaxNGStatesPtr
                        }
                    } else { xmlRelaxNGFreeValidState(ctxt, (*ctxt).state); }
                    (*ctxt).state = oldstate;
                    list = (*list).next
                }
                if !states_0.is_null() {
                    xmlRelaxNGFreeValidState(ctxt, oldstate);
                    (*ctxt).states = states_0;
                    (*ctxt).state = 0 as xmlRelaxNGValidStatePtr;
                    ret = 0 as std::os::raw::c_int
                } else { (*ctxt).states = 0 as xmlRelaxNGStatesPtr }
                (*ctxt).flags = oldflags;
                if ret != 0 as std::os::raw::c_int {
                    if (*ctxt).flags & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        xmlRelaxNGDumpValidError(ctxt);
                    }
                } else if (*ctxt).errNr > errNr {
                    xmlRelaxNGPopErrors(ctxt, errNr);
                }
            }
            current_block_442 = 11850832584604245957;
        }
        10 | 18 => {
            ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
            current_block_442 = 11850832584604245957;
        }
        19 => {
            ret = xmlRelaxNGValidateInterleave(ctxt, define);
            current_block_442 = 11850832584604245957;
        }
        9 => {
            ret = xmlRelaxNGValidateAttribute(ctxt, define);
            current_block_442 = 11850832584604245957;
        }
        20 | -1 | 11 | 12 | 13 => {
            ret = xmlRelaxNGValidateDefinition(ctxt, (*define).content);
            current_block_442 = 11850832584604245957;
        }
        5 => {
            let mut child: xmlNodePtr = 0 as *mut xmlNode;
            let mut content: *mut xmlChar = 0 as *mut xmlChar;
            child = node;
            while !child.is_null() {
                if (*child).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_DATAELEM,
                                            (*(*node).parent).name,
                                            0 as *const xmlChar,
                                            0 as std::os::raw::c_int);
                    ret = -(1 as std::os::raw::c_int);
                    break ;
                } else {
                    if (*child).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*child).type_0 as std::os::raw::c_uint ==
                               XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        content = xmlStrcat(content, (*child).content)
                    }
                    /* TODO: handle entities ... */
                    child = (*child).next
                }
            }
            if ret == -(1 as std::os::raw::c_int) {
                if !content.is_null() {
                    xmlFree.expect("non-null function pointer")(content as
                                                                    *mut std::os::raw::c_void);
                }
            } else {
                if content.is_null() {
                    content =
                        xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char
                                      as *mut xmlChar);
                    if content.is_null() {
                        xmlRngVErrMemory(ctxt,
                                         b"validating\n\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                        ret = -(1 as std::os::raw::c_int);
                        current_block_442 = 11850832584604245957;
                    } else { current_block_442 = 4267368308198710847; }
                } else { current_block_442 = 4267368308198710847; }
                match current_block_442 {
                    11850832584604245957 => { }
                    _ => {
                        ret =
                            xmlRelaxNGValidateDatatype(ctxt, content, define,
                                                       (*(*ctxt).state).seq);
                        if ret == -(1 as std::os::raw::c_int) {
                            xmlRelaxNGAddValidError(ctxt,
                                                    XML_RELAXNG_ERR_DATATYPE,
                                                    (*define).name,
                                                    0 as *const xmlChar,
                                                    0 as std::os::raw::c_int);
                        } else if ret == 0 as std::os::raw::c_int {
                            (*(*ctxt).state).seq = 0 as xmlNodePtr
                        }
                        if !content.is_null() {
                            xmlFree.expect("non-null function pointer")(content
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    }
                }
            }
            current_block_442 = 11850832584604245957;
        }
        7 => {
            let mut content_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut oldvalue: *mut xmlChar = 0 as *mut xmlChar;
            let mut child_0: xmlNodePtr = 0 as *mut xmlNode;
            child_0 = node;
            while !child_0.is_null() {
                if (*child_0).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_VALELEM,
                                            (*(*node).parent).name,
                                            0 as *const xmlChar,
                                            0 as std::os::raw::c_int);
                    ret = -(1 as std::os::raw::c_int);
                    break ;
                } else {
                    if (*child_0).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*child_0).type_0 as std::os::raw::c_uint ==
                               XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        content_0 = xmlStrcat(content_0, (*child_0).content)
                    }
                    /* TODO: handle entities ... */
                    child_0 = (*child_0).next
                }
            }
            if ret == -(1 as std::os::raw::c_int) {
                if !content_0.is_null() {
                    xmlFree.expect("non-null function pointer")(content_0 as
                                                                    *mut std::os::raw::c_void);
                }
            } else {
                if content_0.is_null() {
                    content_0 =
                        xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char
                                      as *mut xmlChar);
                    if content_0.is_null() {
                        xmlRngVErrMemory(ctxt,
                                         b"validating\n\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                        ret = -(1 as std::os::raw::c_int);
                        current_block_442 = 11850832584604245957;
                    } else { current_block_442 = 13351260019855268589; }
                } else { current_block_442 = 13351260019855268589; }
                match current_block_442 {
                    11850832584604245957 => { }
                    _ => {
                        oldvalue = (*(*ctxt).state).value;
                        (*(*ctxt).state).value = content_0;
                        ret = xmlRelaxNGValidateValue(ctxt, define);
                        (*(*ctxt).state).value = oldvalue;
                        if ret == -(1 as std::os::raw::c_int) {
                            xmlRelaxNGAddValidError(ctxt,
                                                    XML_RELAXNG_ERR_VALUE,
                                                    (*define).name,
                                                    0 as *const xmlChar,
                                                    0 as std::os::raw::c_int);
                        } else if ret == 0 as std::os::raw::c_int {
                            (*(*ctxt).state).seq = 0 as xmlNodePtr
                        }
                        if !content_0.is_null() {
                            xmlFree.expect("non-null function pointer")(content_0
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    }
                }
            }
            current_block_442 = 11850832584604245957;
        }
        8 => {
            let mut content_1: *mut xmlChar = 0 as *mut xmlChar;
            let mut child_1: xmlNodePtr = 0 as *mut xmlNode;
            let mut oldvalue_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut oldendvalue: *mut xmlChar = 0 as *mut xmlChar;
            let mut len: std::os::raw::c_int = 0;
            /*
                 * Make sure it's only text nodes
                 */
            content_1 = 0 as *mut xmlChar;
            child_1 = node;
            while !child_1.is_null() {
                if (*child_1).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_LISTELEM,
                                            (*(*node).parent).name,
                                            0 as *const xmlChar,
                                            0 as std::os::raw::c_int);
                    ret = -(1 as std::os::raw::c_int);
                    break ;
                } else {
                    if (*child_1).type_0 as std::os::raw::c_uint ==
                           XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*child_1).type_0 as std::os::raw::c_uint ==
                               XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        content_1 = xmlStrcat(content_1, (*child_1).content)
                    }
                    /* TODO: handle entities ... */
                    child_1 = (*child_1).next
                }
            }
            if ret == -(1 as std::os::raw::c_int) {
                if !content_1.is_null() {
                    xmlFree.expect("non-null function pointer")(content_1 as
                                                                    *mut std::os::raw::c_void);
                }
            } else {
                if content_1.is_null() {
                    content_1 =
                        xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char
                                      as *mut xmlChar);
                    if content_1.is_null() {
                        xmlRngVErrMemory(ctxt,
                                         b"validating\n\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                        ret = -(1 as std::os::raw::c_int);
                        current_block_442 = 11850832584604245957;
                    } else { current_block_442 = 2004714061185701110; }
                } else { current_block_442 = 2004714061185701110; }
                match current_block_442 {
                    11850832584604245957 => { }
                    _ => {
                        len = xmlStrlen(content_1);
                        oldvalue_0 = (*(*ctxt).state).value;
                        oldendvalue = (*(*ctxt).state).endvalue;
                        (*(*ctxt).state).value = content_1;
                        (*(*ctxt).state).endvalue =
                            content_1.offset(len as isize);
                        ret = xmlRelaxNGValidateValue(ctxt, define);
                        (*(*ctxt).state).value = oldvalue_0;
                        (*(*ctxt).state).endvalue = oldendvalue;
                        if ret == -(1 as std::os::raw::c_int) {
                            xmlRelaxNGAddValidError(ctxt,
                                                    XML_RELAXNG_ERR_LIST,
                                                    0 as *const xmlChar,
                                                    0 as *const xmlChar,
                                                    0 as std::os::raw::c_int);
                        } else if ret == 0 as std::os::raw::c_int && !node.is_null() {
                            (*(*ctxt).state).seq = (*node).next
                        }
                        if !content_1.is_null() {
                            xmlFree.expect("non-null function pointer")(content_1
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                    }
                }
            }
            current_block_442 = 11850832584604245957;
        }
        2 | 6 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       10575
                                                                           as
                                                                           std::os::raw::c_int);
            ret = -(1 as std::os::raw::c_int);
            current_block_442 = 11850832584604245957;
        }
        _ => { current_block_442 = 11850832584604245957; }
    }
    match current_block_442 {
        8487579351791723214 =>
        /* Falls through. */
        {
            let mut progress: std::os::raw::c_int = 0;
            let mut states: xmlRelaxNGStatesPtr = 0 as xmlRelaxNGStatesPtr;
            let mut res: xmlRelaxNGStatesPtr = 0 as xmlRelaxNGStatesPtr;
            let mut base: std::os::raw::c_int = 0;
            let mut j: std::os::raw::c_int = 0;
            errNr = (*ctxt).errNr;
            res = xmlRelaxNGNewStates(ctxt, 1 as std::os::raw::c_int);
            if res.is_null() {
                ret = -(1 as std::os::raw::c_int)
            } else {
                /*
                 * All the input states are also exit states
                 */
                if !(*ctxt).state.is_null() {
                    xmlRelaxNGAddStates(ctxt, res,
                                        xmlRelaxNGCopyValidState(ctxt,
                                                                 (*ctxt).state));
                } else {
                    j = 0 as std::os::raw::c_int;
                    while j < (*(*ctxt).states).nbState {
                        xmlRelaxNGAddStates(ctxt, res,
                                            xmlRelaxNGCopyValidState(ctxt,
                                                                     *(*(*ctxt).states).tabState.offset(j
                                                                                                            as
                                                                                                            isize)));
                        j += 1
                    }
                }
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as std::os::raw::c_int;
                loop  {
                    progress = 0 as std::os::raw::c_int;
                    base = (*res).nbState;
                    if !(*ctxt).states.is_null() {
                        states = (*ctxt).states;
                        i = 0 as std::os::raw::c_int;
                        while i < (*states).nbState {
                            (*ctxt).state =
                                *(*states).tabState.offset(i as isize);
                            (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
                            ret =
                                xmlRelaxNGValidateDefinitionList(ctxt,
                                                                 (*define).content);
                            if ret == 0 as std::os::raw::c_int {
                                if !(*ctxt).state.is_null() {
                                    tmp =
                                        xmlRelaxNGAddStates(ctxt, res,
                                                            (*ctxt).state);
                                    (*ctxt).state =
                                        0 as xmlRelaxNGValidStatePtr;
                                    if tmp == 1 as std::os::raw::c_int {
                                        progress = 1 as std::os::raw::c_int
                                    }
                                } else if !(*ctxt).states.is_null() {
                                    j = 0 as std::os::raw::c_int;
                                    while j < (*(*ctxt).states).nbState {
                                        tmp =
                                            xmlRelaxNGAddStates(ctxt, res,
                                                                *(*(*ctxt).states).tabState.offset(j
                                                                                                       as
                                                                                                       isize));
                                        if tmp == 1 as std::os::raw::c_int {
                                            progress = 1 as std::os::raw::c_int
                                        }
                                        j += 1
                                    }
                                    xmlRelaxNGFreeStates(ctxt,
                                                         (*ctxt).states);
                                    (*ctxt).states = 0 as xmlRelaxNGStatesPtr
                                }
                            } else if !(*ctxt).state.is_null() {
                                xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                                (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
                            }
                            i += 1
                        }
                    } else {
                        ret =
                            xmlRelaxNGValidateDefinitionList(ctxt,
                                                             (*define).content);
                        if ret != 0 as std::os::raw::c_int {
                            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                            (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
                        } else {
                            base = (*res).nbState;
                            if !(*ctxt).state.is_null() {
                                tmp =
                                    xmlRelaxNGAddStates(ctxt, res,
                                                        (*ctxt).state);
                                (*ctxt).state = 0 as xmlRelaxNGValidStatePtr;
                                if tmp == 1 as std::os::raw::c_int {
                                    progress = 1 as std::os::raw::c_int
                                }
                            } else if !(*ctxt).states.is_null() {
                                j = 0 as std::os::raw::c_int;
                                while j < (*(*ctxt).states).nbState {
                                    tmp =
                                        xmlRelaxNGAddStates(ctxt, res,
                                                            *(*(*ctxt).states).tabState.offset(j
                                                                                                   as
                                                                                                   isize));
                                    if tmp == 1 as std::os::raw::c_int {
                                        progress = 1 as std::os::raw::c_int
                                    }
                                    j += 1
                                }
                                if states.is_null() {
                                    states = (*ctxt).states
                                } else {
                                    xmlRelaxNGFreeStates(ctxt,
                                                         (*ctxt).states);
                                }
                                (*ctxt).states = 0 as xmlRelaxNGStatesPtr
                            }
                        }
                    }
                    if progress != 0 {
                        /*
                         * Collect all the new nodes added at that step
                         * and make them the new node set
                         */
                        if (*res).nbState - base == 1 as std::os::raw::c_int {
                            (*ctxt).state =
                                xmlRelaxNGCopyValidState(ctxt,
                                                         *(*res).tabState.offset(base
                                                                                     as
                                                                                     isize))
                        } else {
                            if states.is_null() {
                                xmlRelaxNGNewStates(ctxt,
                                                    (*res).nbState - base);
                                states = (*ctxt).states;
                                if states.is_null() {
                                    progress = 0 as std::os::raw::c_int;
                                    break ;
                                }
                            }
                            (*states).nbState = 0 as std::os::raw::c_int;
                            i = base;
                            while i < (*res).nbState {
                                xmlRelaxNGAddStates(ctxt, states,
                                                    xmlRelaxNGCopyValidState(ctxt,
                                                                             *(*res).tabState.offset(i
                                                                                                         as
                                                                                                         isize)));
                                i += 1
                            }
                            (*ctxt).states = states
                        }
                    }
                    if !(progress == 1 as std::os::raw::c_int) { break ; }
                }
                if !states.is_null() { xmlRelaxNGFreeStates(ctxt, states); }
                (*ctxt).states = res;
                (*ctxt).flags = oldflags;
                ret = 0 as std::os::raw::c_int
            }
        }
        _ => { }
    }
    (*ctxt).depth -= 1;
    return ret;
}
/* LIBXML_OUTPUT_ENABLED */
/* ***********************************************************************
 *									*
 *		Validation of compiled content				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGValidateDefinition:
 * @ctxt:  a Relax-NG validation context
 * @define:  the definition to verify
 *
 * Validate the current node lists against the definition
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateDefinition(mut ctxt:
                                                      xmlRelaxNGValidCtxtPtr,
                                                  mut define:
                                                      xmlRelaxNGDefinePtr)
 -> std::os::raw::c_int {
    let mut states: xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStates;
    let mut res: xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStates;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut oldflags: std::os::raw::c_int = 0;
    /*
     * We should NOT have both ctxt->state and ctxt->states
     */
    if !(*ctxt).state.is_null() && !(*ctxt).states.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"relaxng.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   10614 as
                                                                       std::os::raw::c_int);
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
        (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
    }
    if (*ctxt).states.is_null() ||
           (*(*ctxt).states).nbState == 1 as std::os::raw::c_int {
        if !(*ctxt).states.is_null() {
            (*ctxt).state =
                *(*(*ctxt).states).tabState.offset(0 as std::os::raw::c_int as isize);
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            (*ctxt).states = 0 as xmlRelaxNGStatesPtr
        }
        ret = xmlRelaxNGValidateState(ctxt, define);
        if !(*ctxt).state.is_null() && !(*ctxt).states.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       10626
                                                                           as
                                                                           std::os::raw::c_int);
            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
            (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
        }
        if !(*ctxt).states.is_null() &&
               (*(*ctxt).states).nbState == 1 as std::os::raw::c_int {
            (*ctxt).state =
                *(*(*ctxt).states).tabState.offset(0 as std::os::raw::c_int as isize);
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            (*ctxt).states = 0 as xmlRelaxNGStatesPtr
        }
        return ret
    }
    states = (*ctxt).states;
    (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
    res = 0 as xmlRelaxNGStatesPtr;
    j = 0 as std::os::raw::c_int;
    oldflags = (*ctxt).flags;
    (*ctxt).flags |= 1 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < (*states).nbState {
        (*ctxt).state = *(*states).tabState.offset(i as isize);
        (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
        ret = xmlRelaxNGValidateState(ctxt, define);
        /*
         * We should NOT have both ctxt->state and ctxt->states
         */
        if !(*ctxt).state.is_null() && !(*ctxt).states.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"relaxng.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       10651
                                                                           as
                                                                           std::os::raw::c_int);
            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
            (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
        }
        if ret == 0 as std::os::raw::c_int {
            if (*ctxt).states.is_null() {
                if !res.is_null() {
                    /* add the state to the container */
                    xmlRelaxNGAddStates(ctxt, res, (*ctxt).state);
                    (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
                } else {
                    /* add the state directly in states */
                    let fresh55 = j;
                    j = j + 1;
                    let ref mut fresh56 =
                        *(*states).tabState.offset(fresh55 as isize);
                    *fresh56 = (*ctxt).state;
                    (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
                }
            } else if res.is_null() {
                /* make it the new container and copy other results */
                res = (*ctxt).states;
                (*ctxt).states = 0 as xmlRelaxNGStatesPtr;
                k = 0 as std::os::raw::c_int;
                while k < j {
                    xmlRelaxNGAddStates(ctxt, res,
                                        *(*states).tabState.offset(k as
                                                                       isize));
                    k += 1
                }
            } else {
                /* add all the new results to res and reff the container */
                k = 0 as std::os::raw::c_int;
                while k < (*(*ctxt).states).nbState {
                    xmlRelaxNGAddStates(ctxt, res,
                                        *(*(*ctxt).states).tabState.offset(k
                                                                               as
                                                                               isize));
                    k += 1
                }
                xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                (*ctxt).states = 0 as xmlRelaxNGStatesPtr
            }
        } else if !(*ctxt).state.is_null() {
            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
            (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
        } else if !(*ctxt).states.is_null() {
            k = 0 as std::os::raw::c_int;
            while k < (*(*ctxt).states).nbState {
                xmlRelaxNGFreeValidState(ctxt,
                                         *(*(*ctxt).states).tabState.offset(k
                                                                                as
                                                                                isize));
                k += 1
            }
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            (*ctxt).states = 0 as xmlRelaxNGStatesPtr
        }
        i += 1
    }
    (*ctxt).flags = oldflags;
    if !res.is_null() {
        xmlRelaxNGFreeStates(ctxt, states);
        (*ctxt).states = res;
        ret = 0 as std::os::raw::c_int
    } else if j > 1 as std::os::raw::c_int {
        (*states).nbState = j;
        (*ctxt).states = states;
        ret = 0 as std::os::raw::c_int
    } else if j == 1 as std::os::raw::c_int {
        (*ctxt).state = *(*states).tabState.offset(0 as std::os::raw::c_int as isize);
        xmlRelaxNGFreeStates(ctxt, states);
        ret = 0 as std::os::raw::c_int
    } else {
        ret = -(1 as std::os::raw::c_int);
        xmlRelaxNGFreeStates(ctxt, states);
        if !(*ctxt).states.is_null() {
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            (*ctxt).states = 0 as xmlRelaxNGStatesPtr
        }
    }
    if !(*ctxt).state.is_null() && !(*ctxt).states.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"relaxng.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   10717 as
                                                                       std::os::raw::c_int);
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
        (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
    }
    return ret;
}
/* *
 * xmlRelaxNGValidateDocument:
 * @ctxt:  a Relax-NG validation context
 * @doc:  the document
 *
 * Validate the given document
 *
 * Returns 0 if the validation succeeded or an error code.
 */
unsafe extern "C" fn xmlRelaxNGValidateDocument(mut ctxt:
                                                    xmlRelaxNGValidCtxtPtr,
                                                mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut schema: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
    let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if ctxt.is_null() || (*ctxt).schema.is_null() || doc.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    (*ctxt).errNo = XML_RELAXNG_OK as std::os::raw::c_int;
    schema = (*ctxt).schema;
    grammar = (*schema).topgrammar;
    if grammar.is_null() {
        xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_NOGRAMMAR,
                                0 as *const xmlChar, 0 as *const xmlChar,
                                0 as std::os::raw::c_int);
        return -(1 as std::os::raw::c_int)
    }
    state = xmlRelaxNGNewValidState(ctxt, 0 as xmlNodePtr);
    (*ctxt).state = state;
    ret = xmlRelaxNGValidateDefinition(ctxt, (*grammar).start);
    if !(*ctxt).state.is_null() && !(*state).seq.is_null() {
        state = (*ctxt).state;
        node = (*state).seq;
        node = xmlRelaxNGSkipIgnored(ctxt, node);
        if !node.is_null() {
            if ret != -(1 as std::os::raw::c_int) {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_EXTRADATA,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                ret = -(1 as std::os::raw::c_int)
            }
        }
    } else if !(*ctxt).states.is_null() {
        let mut i: std::os::raw::c_int = 0;
        let mut tmp: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        i = 0 as std::os::raw::c_int;
        while i < (*(*ctxt).states).nbState {
            state = *(*(*ctxt).states).tabState.offset(i as isize);
            node = (*state).seq;
            node = xmlRelaxNGSkipIgnored(ctxt, node);
            if node.is_null() { tmp = 0 as std::os::raw::c_int }
            xmlRelaxNGFreeValidState(ctxt, state);
            i += 1
        }
        if tmp == -(1 as std::os::raw::c_int) {
            if ret != -(1 as std::os::raw::c_int) {
                xmlRelaxNGAddValidError(ctxt, XML_RELAXNG_ERR_EXTRADATA,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                        0 as std::os::raw::c_int);
                ret = -(1 as std::os::raw::c_int)
            }
        }
    }
    if !(*ctxt).state.is_null() {
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
        (*ctxt).state = 0 as xmlRelaxNGValidStatePtr
    }
    if ret != 0 as std::os::raw::c_int { xmlRelaxNGDumpValidError(ctxt); }
    if (*ctxt).idref == 1 as std::os::raw::c_int {
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
                         state: 0 as *mut xmlAutomataState,};
        memset(&mut vctxt as *mut xmlValidCtxt as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<xmlValidCtxt>() as std::os::raw::c_ulong);
        vctxt.valid = 1 as std::os::raw::c_int;
        vctxt.error = (*ctxt).error;
        vctxt.warning = (*ctxt).warning;
        vctxt.userData = (*ctxt).userData;
        if xmlValidateDocumentFinal(&mut vctxt, doc) != 1 as std::os::raw::c_int {
            ret = -(1 as std::os::raw::c_int)
        }
    }
    /* LIBXML_VALID_ENABLED */
    if ret == 0 as std::os::raw::c_int &&
           (*ctxt).errNo != XML_RELAXNG_OK as std::os::raw::c_int {
        ret = -(1 as std::os::raw::c_int)
    }
    return ret;
}
/* *
 * xmlRelaxNGCleanPSVI:
 * @node:  an input element or document
 *
 * Call this routine to speed up XPath computation on static documents.
 * This stamps all the element nodes with the document order
 * Like for line information, the order is kept in the element->content
 * field, the value stored is actually - the node number (starting at -1)
 * to be able to differentiate from line numbers.
 *
 * Returns the number of elements found in the document or -1 in case
 *    of error.
 */
unsafe extern "C" fn xmlRelaxNGCleanPSVI(mut node: xmlNodePtr) {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if node.is_null() ||
           (*node).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*node).type_0 as std::os::raw::c_uint !=
                   XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               (*node).type_0 as std::os::raw::c_uint !=
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if (*node).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        (*node).psvi = 0 as *mut std::os::raw::c_void
    }
    cur = (*node).children;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            (*cur).psvi = 0 as *mut std::os::raw::c_void;
            if !(*cur).children.is_null() {
                cur = (*cur).children;
                continue ;
            }
        }
        if !(*cur).next.is_null() {
            cur = (*cur).next
        } else {
            loop  {
                cur = (*cur).parent;
                if cur.is_null() { break ; }
                if cur == node {
                    cur = 0 as xmlNodePtr;
                    break ;
                } else if !(*cur).next.is_null() {
                    cur = (*cur).next;
                    break ;
                } else if cur.is_null() { break ; }
            }
        }
    };
}
/* ***********************************************************************
 *									*
 *			Validation interfaces				*
 *									*
 ************************************************************************/
/* *
 * xmlRelaxNGNewValidCtxt:
 * @schema:  a precompiled XML RelaxNGs
 *
 * Create an XML RelaxNGs validation context based on the given schema
 *
 * Returns the validation context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewValidCtxt(mut schema: xmlRelaxNGPtr)
 -> xmlRelaxNGValidCtxtPtr {
    let mut ret: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlRelaxNGValidCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlRelaxNGValidCtxtPtr;
    if ret.is_null() {
        xmlRngVErrMemory(0 as xmlRelaxNGValidCtxtPtr,
                         b"building context\n\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlRelaxNGValidCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlRelaxNGValidCtxt>() as std::os::raw::c_ulong);
    (*ret).schema = schema;
    (*ret).error = *__xmlGenericError();
    (*ret).userData = *__xmlGenericErrorContext();
    (*ret).errNr = 0 as std::os::raw::c_int;
    (*ret).errMax = 0 as std::os::raw::c_int;
    (*ret).err = 0 as xmlRelaxNGValidErrorPtr;
    (*ret).errTab = 0 as xmlRelaxNGValidErrorPtr;
    if !schema.is_null() { (*ret).idref = (*schema).idref }
    (*ret).states = 0 as xmlRelaxNGStatesPtr;
    (*ret).freeState = 0 as xmlRelaxNGStatesPtr;
    (*ret).freeStates = 0 as *mut xmlRelaxNGStatesPtr;
    (*ret).errNo = XML_RELAXNG_OK as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlRelaxNGFreeValidCtxt:
 * @ctxt:  the schema validation context
 *
 * Free the resources associated to the schema validation context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFreeValidCtxt(mut ctxt:
                                                     xmlRelaxNGValidCtxtPtr) {
    let mut k: std::os::raw::c_int = 0;
    if ctxt.is_null() { return }
    if !(*ctxt).states.is_null() {
        xmlRelaxNGFreeStates(0 as xmlRelaxNGValidCtxtPtr, (*ctxt).states);
    }
    if !(*ctxt).freeState.is_null() {
        k = 0 as std::os::raw::c_int;
        while k < (*(*ctxt).freeState).nbState {
            xmlRelaxNGFreeValidState(0 as xmlRelaxNGValidCtxtPtr,
                                     *(*(*ctxt).freeState).tabState.offset(k
                                                                               as
                                                                               isize));
            k += 1
        }
        xmlRelaxNGFreeStates(0 as xmlRelaxNGValidCtxtPtr, (*ctxt).freeState);
    }
    if !(*ctxt).freeStates.is_null() {
        k = 0 as std::os::raw::c_int;
        while k < (*ctxt).freeStatesNr {
            xmlRelaxNGFreeStates(0 as xmlRelaxNGValidCtxtPtr,
                                 *(*ctxt).freeStates.offset(k as isize));
            k += 1
        }
        xmlFree.expect("non-null function pointer")((*ctxt).freeStates as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).errTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).errTab as
                                                        *mut std::os::raw::c_void);
    }
    if !(*ctxt).elemTab.is_null() {
        let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
        exec = xmlRelaxNGElemPop(ctxt);
        while !exec.is_null() {
            xmlRegFreeExecCtxt(exec);
            exec = xmlRelaxNGElemPop(ctxt)
        }
        xmlFree.expect("non-null function pointer")((*ctxt).elemTab as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/* LIBXML_OUTPUT_ENABLED */
/*
 * Interfaces for validating
 */
/* *
 * xmlRelaxNGSetValidErrors:
 * @ctxt:  a Relax-NG validation context
 * @err:  the error function
 * @warn: the warning function
 * @ctx: the functions context
 *
 * Set the error and warning callback informations
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetValidErrors(mut ctxt:
                                                      xmlRelaxNGValidCtxtPtr,
                                                  mut err:
                                                      xmlRelaxNGValidityErrorFunc,
                                                  mut warn:
                                                      xmlRelaxNGValidityWarningFunc,
                                                  mut ctx:
                                                      *mut std::os::raw::c_void) {
    if ctxt.is_null() { return }
    (*ctxt).error = err;
    (*ctxt).warning = warn;
    (*ctxt).userData = ctx;
    (*ctxt).serror = None;
}
/* *
 * xmlRelaxNGSetValidStructuredErrors:
 * @ctxt:  a Relax-NG validation context
 * @serror:  the structured error function
 * @ctx: the functions context
 *
 * Set the structured error callback
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetValidStructuredErrors(mut ctxt:
                                                                xmlRelaxNGValidCtxtPtr,
                                                            mut serror:
                                                                xmlStructuredErrorFunc,
                                                            mut ctx:
                                                                *mut std::os::raw::c_void) {
    if ctxt.is_null() { return }
    (*ctxt).serror = serror;
    (*ctxt).error = None;
    (*ctxt).warning = None;
    (*ctxt).userData = ctx;
}
/* *
 * xmlRelaxNGGetValidErrors:
 * @ctxt:  a Relax-NG validation context
 * @err:  the error function result
 * @warn: the warning function result
 * @ctx: the functions context result
 *
 * Get the error and warning callback informations
 *
 * Returns -1 in case of error and 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGGetValidErrors(mut ctxt:
                                                      xmlRelaxNGValidCtxtPtr,
                                                  mut err:
                                                      *mut xmlRelaxNGValidityErrorFunc,
                                                  mut warn:
                                                      *mut xmlRelaxNGValidityWarningFunc,
                                                  mut ctx:
                                                      *mut *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    if ctxt.is_null() { return -(1 as std::os::raw::c_int) }
    if !err.is_null() { *err = (*ctxt).error }
    if !warn.is_null() { *warn = (*ctxt).warning }
    if !ctx.is_null() { *ctx = (*ctxt).userData }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlRelaxNGValidateDoc:
 * @ctxt:  a Relax-NG validation context
 * @doc:  a parsed document tree
 *
 * Validate a document tree in memory.
 *
 * Returns 0 if the document is valid, a positive error code
 *     number otherwise and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidateDoc(mut ctxt:
                                                   xmlRelaxNGValidCtxtPtr,
                                               mut doc: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    if ctxt.is_null() || doc.is_null() { return -(1 as std::os::raw::c_int) }
    (*ctxt).doc = doc;
    ret = xmlRelaxNGValidateDocument(ctxt, doc);
    /*
     * Remove all left PSVI
     */
    xmlRelaxNGCleanPSVI(doc as xmlNodePtr);
    /*
     * TODO: build error codes
     */
    if ret == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int }
    return ret;
}
/* LIBXML_SCHEMAS_ENABLED */
/* __INCLUDE_ELFGCCHACK */
