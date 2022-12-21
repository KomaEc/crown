
extern "C" {
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
    pub type _xmlXPathCompExpr;
    #[no_mangle]
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrncmp(str1: *const xmlChar, str2: *const xmlChar,
                  len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewTextLen(content: *const xmlChar, len: std::os::raw::c_int) -> xmlNodePtr;
    #[no_mangle]
    fn xmlCopyNode(node: xmlNodePtr, recursive: std::os::raw::c_int) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlAddNextSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlResetError(err: xmlErrorPtr);
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
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
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
    fn xmlXPathObjectCopy(val: xmlXPathObjectPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathCmpNodes(node1: xmlNodePtr, node2: xmlNodePtr) -> std::os::raw::c_int;
    /* *
 * Context handling.
 */
    #[no_mangle]
    fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
    /* LIBXML_XPATH_ENABLED */
    #[no_mangle]
    fn xmlXPathInit();
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    #[no_mangle]
    fn xmlXPathErr(ctxt: xmlXPathParserContextPtr, error: std::os::raw::c_int);
    /* *
 * Extending a context.
 */
    #[no_mangle]
    fn xmlXPathRegisterNs(ctxt: xmlXPathContextPtr, prefix: *const xmlChar,
                          ns_uri: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXPathRegisterFunc(ctxt: xmlXPathContextPtr, name: *const xmlChar,
                            f: xmlXPathFunction) -> std::os::raw::c_int;
    /* *
 * Utilities to extend XPath.
 */
    #[no_mangle]
    fn xmlXPathNewParserContext(str: *const xmlChar, ctxt: xmlXPathContextPtr)
     -> xmlXPathParserContextPtr;
    #[no_mangle]
    fn xmlXPathFreeParserContext(ctxt: xmlXPathParserContextPtr);
    /* TODO: remap to xmlXPathValuePop and Push. */
    #[no_mangle]
    fn valuePop(ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn valuePush(ctxt: xmlXPathParserContextPtr, value: xmlXPathObjectPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXPathNewString(val: *const xmlChar) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathNewNodeSet(val: xmlNodePtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathRoot(ctxt: xmlXPathParserContextPtr);
    #[no_mangle]
    fn xmlXPathEvalExpr(ctxt: xmlXPathParserContextPtr);
    #[no_mangle]
    fn xmlXPathParseName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlXPathParseNCName(ctxt: xmlXPathParserContextPtr) -> *mut xmlChar;
    #[no_mangle]
    fn xmlXPathEvaluatePredicateResult(ctxt: xmlXPathParserContextPtr,
                                       res: xmlXPathObjectPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlXPathIdFunction(ctxt: xmlXPathParserContextPtr, nargs: std::os::raw::c_int);
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
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
pub type xmlNsPtr = *mut xmlNs;
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
/* *
 * The set of XPath error codes.
 */
/*
 * A node-set (an unordered collection of nodes without duplicates).
 */
/* number of nodes in the set */
/* size of the array as allocated */
/* array of nodes in no particular order */
/* @@ with_ns to check wether namespace nodes should be looked at @@ */
/*
 * An expression is evaluated to yield an object, which
 * has one of the following four basic types:
 *   - node-set
 *   - boolean
 *   - number
 *   - string
 *
 * @@ XPointer will add more types !
 */
/* An XSLT value tree, non modifiable */
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
/*
 * Extra type: a name and a conversion function.
 */
/* the type name */
/* the conversion function */
/*
 * Extra variable: a name and a value.
 */
/* the variable name */
/* the value */
/* *
 * xmlXPathEvalFunc:
 * @ctxt: an XPath parser context
 * @nargs: the number of arguments passed to the function
 *
 * An XPath evaluation function, the parameters are on the XPath context stack.
 */
/*
 * Extra function: a name and a evaluation function.
 */
/* the function name */
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
/*
 * Extra axis: a name and an axis function.
 */
/* the axis name */
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
pub type xmlXPathFuncLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathFunction>;
pub type xmlXPathFunction
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr, _: std::os::raw::c_int)
               -> ()>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
/*
 * The structure of a compiled expression form is not public.
 */
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
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const xmlChar,
                                _: *const xmlChar) -> xmlXPathObjectPtr>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
pub type xmlXPathAxisFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathParserContextPtr,
                                _: xmlXPathObjectPtr) -> xmlXPathObjectPtr>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
pub type xmlXPathConvertFunc
    =
    Option<unsafe extern "C" fn(_: xmlXPathObjectPtr, _: std::os::raw::c_int)
               -> std::os::raw::c_int>;
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
pub const XPATH_FORBID_VARIABLE_ERROR: C2RustUnnamed_1 = 24;
pub const XPATH_STACK_ERROR: C2RustUnnamed_1 = 23;
pub const XPATH_INVALID_CTXT: C2RustUnnamed_1 = 22;
pub const XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_1 = 21;
pub const XPATH_ENCODING_ERROR: C2RustUnnamed_1 = 20;
pub const XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_1 = 19;
pub const XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_1 = 18;
pub const XPTR_RESOURCE_ERROR: C2RustUnnamed_1 = 17;
pub const XPTR_SYNTAX_ERROR: C2RustUnnamed_1 = 16;
pub const XPATH_MEMORY_ERROR: C2RustUnnamed_1 = 15;
pub const XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_1 = 14;
pub const XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_1 = 13;
pub const XPATH_INVALID_ARITY: C2RustUnnamed_1 = 12;
pub const XPATH_INVALID_TYPE: C2RustUnnamed_1 = 11;
pub const XPATH_INVALID_OPERAND: C2RustUnnamed_1 = 10;
pub const XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_1 = 9;
pub const XPATH_UNCLOSED_ERROR: C2RustUnnamed_1 = 8;
pub const XPATH_EXPR_ERROR: C2RustUnnamed_1 = 7;
pub const XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_1 = 6;
pub const XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_1 = 5;
pub const XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_1 = 4;
pub const XPATH_START_LITERAL_ERROR: C2RustUnnamed_1 = 3;
pub const XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_1 = 2;
pub const XPATH_NUMBER_ERROR: C2RustUnnamed_1 = 1;
pub const XPATH_EXPRESSION_OK: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLocationSet {
    pub locNr: std::os::raw::c_int,
    pub locMax: std::os::raw::c_int,
    pub locTab: *mut xmlXPathObjectPtr,
}
pub type xmlLocationSet = _xmlLocationSet;
pub type xmlLocationSetPtr = *mut xmlLocationSet;
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
/*
 * xpointer.c : Code to handle XML Pointer
 *
 * Base implementation was made accordingly to
 * W3C Candidate Recommendation 7 June 2000
 * http://www.w3.org/TR/2000/CR-xptr-20000607
 *
 * Added support for the element() scheme described in:
 * W3C Proposed Recommendation 13 November 2002
 * http://www.w3.org/TR/2002/PR-xptr-element-20021113/
 *
 * See Copyright for the status of this software.
 *
 * daniel@veillard.com
 */
/* To avoid EBCDIC trouble when parsing on zOS */
/*
 * TODO: better handling of error cases, the full expression should
 *       be parsed beforehand instead of a progressive evaluation
 * TODO: Access into entities references are not supported now ...
 *       need a start to be able to pop out of entities refs since
 *       parent is the endity declaration, not the ref.
 */
/* Add support of the xmlns() xpointer scheme to initialize the namespaces */
/* #define DEBUG_RANGES */
/* ***********************************************************************
 *									*
 *		Some factorized error routines				*
 *									*
 ************************************************************************/
/* *
 * xmlXPtrErrMemory:
 * @extra:  extra informations
 *
 * Handle a redefinition of attribute error
 */
unsafe extern "C" fn xmlXPtrErrMemory(mut extra: *const std::os::raw::c_char) {
    __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                    0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                    XML_FROM_XPOINTER as std::os::raw::c_int,
                    XML_ERR_NO_MEMORY as std::os::raw::c_int, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int, extra,
                    0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                    0 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                    b"Memory allocation failed : %s\n\x00" as *const u8 as
                        *const std::os::raw::c_char, extra);
}
/* *
 * xmlXPtrErr:
 * @ctxt:  an XPTR evaluation context
 * @extra:  extra informations
 *
 * Handle a redefinition of attribute error
 */
unsafe extern "C" fn xmlXPtrErr(mut ctxt: xmlXPathParserContextPtr,
                                mut error: std::os::raw::c_int,
                                mut msg: *const std::os::raw::c_char,
                                mut extra: *const xmlChar) {
    if !ctxt.is_null() { (*ctxt).error = error }
    if ctxt.is_null() || (*ctxt).context.is_null() {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void, 0 as *mut std::os::raw::c_void,
                        XML_FROM_XPOINTER as std::os::raw::c_int, error,
                        XML_ERR_ERROR, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, extra as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, 0 as std::os::raw::c_int, msg, extra);
        return
    }
    /* cleanup current last error */
    xmlResetError(&mut (*(*ctxt).context).lastError);
    (*(*ctxt).context).lastError.domain = XML_FROM_XPOINTER as std::os::raw::c_int;
    (*(*ctxt).context).lastError.code = error;
    (*(*ctxt).context).lastError.level = XML_ERR_ERROR;
    (*(*ctxt).context).lastError.str1 =
        xmlStrdup((*ctxt).base) as *mut std::os::raw::c_char;
    (*(*ctxt).context).lastError.int1 =
        (*ctxt).cur.offset_from((*ctxt).base) as std::os::raw::c_long as
            std::os::raw::c_int;
    (*(*ctxt).context).lastError.node =
        (*(*ctxt).context).debugNode as *mut std::os::raw::c_void;
    if (*(*ctxt).context).error.is_some() {
        (*(*ctxt).context).error.expect("non-null function pointer")((*(*ctxt).context).userData,
                                                                     &mut (*(*ctxt).context).lastError);
    } else {
        __xmlRaiseError(None, None, 0 as *mut std::os::raw::c_void,
                        0 as *mut std::os::raw::c_void,
                        (*(*ctxt).context).debugNode as *mut std::os::raw::c_void,
                        XML_FROM_XPOINTER as std::os::raw::c_int, error,
                        XML_ERR_ERROR, 0 as *const std::os::raw::c_char,
                        0 as std::os::raw::c_int, extra as *const std::os::raw::c_char,
                        (*ctxt).base as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char,
                        (*ctxt).cur.offset_from((*ctxt).base) as
                            std::os::raw::c_long as std::os::raw::c_int, 0 as std::os::raw::c_int,
                        msg, extra);
    };
}
/* *
 * xmlXPtrGetArity:
 * @cur:  the node
 *
 * Returns the number of child for an element, -1 in case of error
 */
unsafe extern "C" fn xmlXPtrGetArity(mut cur: xmlNodePtr) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    cur = (*cur).children;
    i = 0 as std::os::raw::c_int;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            i += 1
        }
        cur = (*cur).next
    }
    return i;
}
/* *
 * xmlXPtrGetIndex:
 * @cur:  the node
 *
 * Returns the index of the node in its parent children list, -1
 *         in case of error
 */
unsafe extern "C" fn xmlXPtrGetIndex(mut cur: xmlNodePtr) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    i = 1 as std::os::raw::c_int;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            i += 1
        }
        cur = (*cur).prev
    }
    return i;
}
/* *
 * xmlXPtrGetNthChild:
 * @cur:  the node
 * @no:  the child number
 *
 * Returns the @no'th element child of @cur or NULL
 */
unsafe extern "C" fn xmlXPtrGetNthChild(mut cur: xmlNodePtr,
                                        mut no: std::os::raw::c_int) -> xmlNodePtr {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return cur
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
/* ***********************************************************************
 *									*
 *		Handling of XPointer specific types			*
 *									*
 ************************************************************************/
/* *
 * xmlXPtrCmpPoints:
 * @node1:  the first node
 * @index1:  the first index
 * @node2:  the second node
 * @index2:  the second index
 *
 * Compare two points w.r.t document order
 *
 * Returns -2 in case of error 1 if first point < second point, 0 if
 *         that's the same point, -1 otherwise
 */
unsafe extern "C" fn xmlXPtrCmpPoints(mut node1: xmlNodePtr,
                                      mut index1: std::os::raw::c_int,
                                      mut node2: xmlNodePtr,
                                      mut index2: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if node1.is_null() || node2.is_null() { return -(2 as std::os::raw::c_int) }
    /*
     * a couple of optimizations which will avoid computations in most cases
     */
    if node1 == node2 {
        if index1 < index2 { return 1 as std::os::raw::c_int }
        if index1 > index2 { return -(1 as std::os::raw::c_int) }
        return 0 as std::os::raw::c_int
    }
    return xmlXPathCmpNodes(node1, node2);
}
/* *
 * xmlXPtrNewPoint:
 * @node:  the xmlNodePtr
 * @indx:  the indx within the node
 *
 * Create a new xmlXPathObjectPtr of type point
 *
 * Returns the newly created object.
 */
unsafe extern "C" fn xmlXPtrNewPoint(mut node: xmlNodePtr,
                                     mut indx: std::os::raw::c_int)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if node.is_null() { return 0 as xmlXPathObjectPtr }
    if indx < 0 as std::os::raw::c_int { return 0 as xmlXPathObjectPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>()
                                                          as std::os::raw::c_ulong) as
            xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPtrErrMemory(b"allocating point\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlXPathObjectPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXPathObject>() as std::os::raw::c_ulong);
    (*ret).type_0 = XPATH_POINT;
    (*ret).user = node as *mut std::os::raw::c_void;
    (*ret).index = indx;
    return ret;
}
/* *
 * xmlXPtrRangeCheckOrder:
 * @range:  an object range
 *
 * Make sure the points in the range are in the right order
 */
unsafe extern "C" fn xmlXPtrRangeCheckOrder(mut range: xmlXPathObjectPtr) {
    let mut tmp: std::os::raw::c_int = 0;
    let mut tmp2: xmlNodePtr = 0 as *mut xmlNode;
    if range.is_null() { return }
    if (*range).type_0 as std::os::raw::c_uint !=
           XPATH_RANGE as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if (*range).user2.is_null() { return }
    tmp =
        xmlXPtrCmpPoints((*range).user as xmlNodePtr, (*range).index,
                         (*range).user2 as xmlNodePtr, (*range).index2);
    if tmp == -(1 as std::os::raw::c_int) {
        tmp2 = (*range).user as xmlNodePtr;
        (*range).user = (*range).user2;
        (*range).user2 = tmp2 as *mut std::os::raw::c_void;
        tmp = (*range).index;
        (*range).index = (*range).index2;
        (*range).index2 = tmp
    };
}
/* *
 * xmlXPtrRangesEqual:
 * @range1:  the first range
 * @range2:  the second range
 *
 * Compare two ranges
 *
 * Returns 1 if equal, 0 otherwise
 */
unsafe extern "C" fn xmlXPtrRangesEqual(mut range1: xmlXPathObjectPtr,
                                        mut range2: xmlXPathObjectPtr)
 -> std::os::raw::c_int {
    if range1 == range2 { return 1 as std::os::raw::c_int }
    if range1.is_null() || range2.is_null() { return 0 as std::os::raw::c_int }
    if (*range1).type_0 as std::os::raw::c_uint != (*range2).type_0 as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*range1).type_0 as std::os::raw::c_uint !=
           XPATH_RANGE as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if (*range1).user != (*range2).user { return 0 as std::os::raw::c_int }
    if (*range1).index != (*range2).index { return 0 as std::os::raw::c_int }
    if (*range1).user2 != (*range2).user2 { return 0 as std::os::raw::c_int }
    if (*range1).index2 != (*range2).index2 { return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlXPtrNewRangeInternal:
 * @start:  the starting node
 * @startindex:  the start index
 * @end:  the ending point
 * @endindex:  the ending index
 *
 * Internal function to create a new xmlXPathObjectPtr of type range
 *
 * Returns the newly created object.
 */
unsafe extern "C" fn xmlXPtrNewRangeInternal(mut start: xmlNodePtr,
                                             mut startindex: std::os::raw::c_int,
                                             mut end: xmlNodePtr,
                                             mut endindex: std::os::raw::c_int)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    /*
     * Namespace nodes must be copied (see xmlXPathNodeSetDupNs).
     * Disallow them for now.
     */
    if !start.is_null() &&
           (*start).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlXPathObjectPtr
    }
    if !end.is_null() &&
           (*end).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlXPathObjectPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>()
                                                          as std::os::raw::c_ulong) as
            xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPtrErrMemory(b"allocating range\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlXPathObjectPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXPathObject>() as std::os::raw::c_ulong);
    (*ret).type_0 = XPATH_RANGE;
    (*ret).user = start as *mut std::os::raw::c_void;
    (*ret).index = startindex;
    (*ret).user2 = end as *mut std::os::raw::c_void;
    (*ret).index2 = endindex;
    return ret;
}
/* *
 * xmlXPtrNewRange:
 * @start:  the starting node
 * @startindex:  the start index
 * @end:  the ending point
 * @endindex:  the ending index
 *
 * Create a new xmlXPathObjectPtr of type range
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewRange(mut start: xmlNodePtr,
                                         mut startindex: std::os::raw::c_int,
                                         mut end: xmlNodePtr,
                                         mut endindex: std::os::raw::c_int)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    if end.is_null() { return 0 as xmlXPathObjectPtr }
    if startindex < 0 as std::os::raw::c_int { return 0 as xmlXPathObjectPtr }
    if endindex < 0 as std::os::raw::c_int { return 0 as xmlXPathObjectPtr }
    ret = xmlXPtrNewRangeInternal(start, startindex, end, endindex);
    xmlXPtrRangeCheckOrder(ret);
    return ret;
}
/* *
 * xmlXPtrNewRangePoints:
 * @start:  the starting point
 * @end:  the ending point
 *
 * Create a new xmlXPathObjectPtr of type range using 2 Points
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewRangePoints(mut start: xmlXPathObjectPtr,
                                               mut end: xmlXPathObjectPtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    if end.is_null() { return 0 as xmlXPathObjectPtr }
    if (*start).type_0 as std::os::raw::c_uint !=
           XPATH_POINT as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlXPathObjectPtr
    }
    if (*end).type_0 as std::os::raw::c_uint !=
           XPATH_POINT as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlXPathObjectPtr
    }
    ret =
        xmlXPtrNewRangeInternal((*start).user as xmlNodePtr, (*start).index,
                                (*end).user as xmlNodePtr, (*end).index);
    xmlXPtrRangeCheckOrder(ret);
    return ret;
}
/* *
 * xmlXPtrNewRangePointNode:
 * @start:  the starting point
 * @end:  the ending node
 *
 * Create a new xmlXPathObjectPtr of type range from a point to a node
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewRangePointNode(mut start:
                                                      xmlXPathObjectPtr,
                                                  mut end: xmlNodePtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    if end.is_null() { return 0 as xmlXPathObjectPtr }
    if (*start).type_0 as std::os::raw::c_uint !=
           XPATH_POINT as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlXPathObjectPtr
    }
    ret =
        xmlXPtrNewRangeInternal((*start).user as xmlNodePtr, (*start).index,
                                end, -(1 as std::os::raw::c_int));
    xmlXPtrRangeCheckOrder(ret);
    return ret;
}
/* *
 * xmlXPtrNewRangeNodePoint:
 * @start:  the starting node
 * @end:  the ending point
 *
 * Create a new xmlXPathObjectPtr of type range from a node to a point
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewRangeNodePoint(mut start: xmlNodePtr,
                                                  mut end: xmlXPathObjectPtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    if end.is_null() { return 0 as xmlXPathObjectPtr }
    if (*end).type_0 as std::os::raw::c_uint !=
           XPATH_POINT as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlXPathObjectPtr
    }
    ret =
        xmlXPtrNewRangeInternal(start, -(1 as std::os::raw::c_int),
                                (*end).user as xmlNodePtr, (*end).index);
    xmlXPtrRangeCheckOrder(ret);
    return ret;
}
/* *
 * xmlXPtrNewRangeNodes:
 * @start:  the starting node
 * @end:  the ending node
 *
 * Create a new xmlXPathObjectPtr of type range using 2 nodes
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewRangeNodes(mut start: xmlNodePtr,
                                              mut end: xmlNodePtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    if end.is_null() { return 0 as xmlXPathObjectPtr }
    ret =
        xmlXPtrNewRangeInternal(start, -(1 as std::os::raw::c_int), end,
                                -(1 as std::os::raw::c_int));
    xmlXPtrRangeCheckOrder(ret);
    return ret;
}
/* *
 * xmlXPtrNewCollapsedRange:
 * @start:  the starting and ending node
 *
 * Create a new xmlXPathObjectPtr of type range using a single nodes
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewCollapsedRange(mut start: xmlNodePtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    ret =
        xmlXPtrNewRangeInternal(start, -(1 as std::os::raw::c_int), 0 as xmlNodePtr,
                                -(1 as std::os::raw::c_int));
    return ret;
}
/* *
 * xmlXPtrNewRangeNodeObject:
 * @start:  the starting node
 * @end:  the ending object
 *
 * Create a new xmlXPathObjectPtr of type range from a not to an object
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewRangeNodeObject(mut start: xmlNodePtr,
                                                   mut end: xmlXPathObjectPtr)
 -> xmlXPathObjectPtr {
    let mut endNode: xmlNodePtr = 0 as *mut xmlNode;
    let mut endIndex: std::os::raw::c_int = 0;
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if start.is_null() { return 0 as xmlXPathObjectPtr }
    if end.is_null() { return 0 as xmlXPathObjectPtr }
    match (*end).type_0 as std::os::raw::c_uint {
        5 => { endNode = (*end).user as xmlNodePtr; endIndex = (*end).index }
        6 => {
            endNode = (*end).user2 as xmlNodePtr;
            endIndex = (*end).index2
        }
        1 => {
            /*
	     * Empty set ...
	     */
            if (*end).nodesetval.is_null() ||
                   (*(*end).nodesetval).nodeNr <= 0 as std::os::raw::c_int {
                return 0 as xmlXPathObjectPtr
            }
            endNode =
                *(*(*end).nodesetval).nodeTab.offset(((*(*end).nodesetval).nodeNr
                                                          - 1 as std::os::raw::c_int)
                                                         as isize);
            endIndex = -(1 as std::os::raw::c_int)
        }
        _ => {
            /* TODO */
            return 0 as xmlXPathObjectPtr
        }
    }
    ret =
        xmlXPtrNewRangeInternal(start, -(1 as std::os::raw::c_int), endNode,
                                endIndex);
    xmlXPtrRangeCheckOrder(ret);
    return ret;
}
/* *
 * xmlXPtrLocationSetCreate:
 * @val:  an initial xmlXPathObjectPtr, or NULL
 *
 * Create a new xmlLocationSetPtr of type double and of value @val
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrLocationSetCreate(mut val: xmlXPathObjectPtr)
 -> xmlLocationSetPtr {
    let mut ret: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlLocationSet>()
                                                          as std::os::raw::c_ulong) as
            xmlLocationSetPtr;
    if ret.is_null() {
        xmlXPtrErrMemory(b"allocating locationset\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlLocationSetPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlLocationSet>() as std::os::raw::c_ulong);
    if !val.is_null() {
        (*ret).locTab =
            xmlMalloc.expect("non-null function pointer")((10 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlXPathObjectPtr;
        if (*ret).locTab.is_null() {
            xmlXPtrErrMemory(b"allocating locationset\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            xmlFree.expect("non-null function pointer")(ret as
                                                            *mut std::os::raw::c_void);
            return 0 as xmlLocationSetPtr
        }
        memset((*ret).locTab as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               (10 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>()
                                                    as std::os::raw::c_ulong));
        (*ret).locMax = 10 as std::os::raw::c_int;
        let fresh0 = (*ret).locNr;
        (*ret).locNr = (*ret).locNr + 1;
        let ref mut fresh1 = *(*ret).locTab.offset(fresh0 as isize);
        *fresh1 = val
    }
    return ret;
}
/* *
 * xmlXPtrLocationSetAdd:
 * @cur:  the initial range set
 * @val:  a new xmlXPathObjectPtr
 *
 * add a new xmlXPathObjectPtr to an existing LocationSet
 * If the location already exist in the set @val is freed.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrLocationSetAdd(mut cur: xmlLocationSetPtr,
                                               mut val: xmlXPathObjectPtr) {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() || val.is_null() { return }
    /*
     * check against doublons
     */
    i = 0 as std::os::raw::c_int;
    while i < (*cur).locNr {
        if xmlXPtrRangesEqual(*(*cur).locTab.offset(i as isize), val) != 0 {
            xmlXPathFreeObject(val);
            return
        }
        i += 1
    }
    /*
     * grow the locTab if needed
     */
    if (*cur).locMax == 0 as std::os::raw::c_int {
        (*cur).locTab =
            xmlMalloc.expect("non-null function pointer")((10 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlXPathObjectPtr;
        if (*cur).locTab.is_null() {
            xmlXPtrErrMemory(b"adding location to set\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        memset((*cur).locTab as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               (10 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>()
                                                    as std::os::raw::c_ulong));
        (*cur).locMax = 10 as std::os::raw::c_int
    } else if (*cur).locNr == (*cur).locMax {
        let mut temp: *mut xmlXPathObjectPtr = 0 as *mut xmlXPathObjectPtr;
        (*cur).locMax *= 2 as std::os::raw::c_int;
        temp =
            xmlRealloc.expect("non-null function pointer")((*cur).locTab as
                                                               *mut std::os::raw::c_void,
                                                           ((*cur).locMax as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut xmlXPathObjectPtr;
        if temp.is_null() {
            xmlXPtrErrMemory(b"adding location to set\x00" as *const u8 as
                                 *const std::os::raw::c_char);
            return
        }
        (*cur).locTab = temp
    }
    let fresh2 = (*cur).locNr;
    (*cur).locNr = (*cur).locNr + 1;
    let ref mut fresh3 = *(*cur).locTab.offset(fresh2 as isize);
    *fresh3 = val;
}
/* *
 * xmlXPtrLocationSetMerge:
 * @val1:  the first LocationSet
 * @val2:  the second LocationSet
 *
 * Merges two rangesets, all ranges from @val2 are added to @val1
 *
 * Returns val1 once extended or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrLocationSetMerge(mut val1: xmlLocationSetPtr,
                                                 mut val2: xmlLocationSetPtr)
 -> xmlLocationSetPtr {
    let mut i: std::os::raw::c_int = 0;
    if val1.is_null() { return 0 as xmlLocationSetPtr }
    if val2.is_null() { return val1 }
    /*
     * !!!!! this can be optimized a lot, knowing that both
     *       val1 and val2 already have unicity of their values.
     */
    i = 0 as std::os::raw::c_int;
    while i < (*val2).locNr {
        xmlXPtrLocationSetAdd(val1, *(*val2).locTab.offset(i as isize));
        i += 1
    }
    return val1;
}
/* *
 * xmlXPtrLocationSetDel:
 * @cur:  the initial range set
 * @val:  an xmlXPathObjectPtr
 *
 * Removes an xmlXPathObjectPtr from an existing LocationSet
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrLocationSetDel(mut cur: xmlLocationSetPtr,
                                               mut val: xmlXPathObjectPtr) {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() { return }
    if val.is_null() { return }
    /*
     * check against doublons
     */
    i = 0 as std::os::raw::c_int;
    while i < (*cur).locNr {
        if *(*cur).locTab.offset(i as isize) == val { break ; }
        i += 1
    }
    if i >= (*cur).locNr { return }
    (*cur).locNr -= 1;
    while i < (*cur).locNr {
        let ref mut fresh4 = *(*cur).locTab.offset(i as isize);
        *fresh4 = *(*cur).locTab.offset((i + 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    let ref mut fresh5 = *(*cur).locTab.offset((*cur).locNr as isize);
    *fresh5 = 0 as xmlXPathObjectPtr;
}
/* *
 * xmlXPtrLocationSetRemove:
 * @cur:  the initial range set
 * @val:  the index to remove
 *
 * Removes an entry from an existing LocationSet list.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrLocationSetRemove(mut cur: xmlLocationSetPtr,
                                                  mut val: std::os::raw::c_int) {
    if cur.is_null() { return }
    if val >= (*cur).locNr { return }
    (*cur).locNr -= 1;
    while val < (*cur).locNr {
        let ref mut fresh6 = *(*cur).locTab.offset(val as isize);
        *fresh6 = *(*cur).locTab.offset((val + 1 as std::os::raw::c_int) as isize);
        val += 1
    }
    let ref mut fresh7 = *(*cur).locTab.offset((*cur).locNr as isize);
    *fresh7 = 0 as xmlXPathObjectPtr;
}
/* *
 * xmlXPtrFreeLocationSet:
 * @obj:  the xmlLocationSetPtr to free
 *
 * Free the LocationSet compound (not the actual ranges !).
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrFreeLocationSet(mut obj: xmlLocationSetPtr) {
    let mut i: std::os::raw::c_int = 0;
    if obj.is_null() { return }
    if !(*obj).locTab.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < (*obj).locNr {
            xmlXPathFreeObject(*(*obj).locTab.offset(i as isize));
            i += 1
        }
        xmlFree.expect("non-null function pointer")((*obj).locTab as
                                                        *mut std::os::raw::c_void);
    }
    xmlFree.expect("non-null function pointer")(obj as *mut std::os::raw::c_void);
}
/* *
 * xmlXPtrNewLocationSetNodes:
 * @start:  the start NodePtr value
 * @end:  the end NodePtr value or NULL
 *
 * Create a new xmlXPathObjectPtr of type LocationSet and initialize
 * it with the single range made of the two nodes @start and @end
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewLocationSetNodes(mut start: xmlNodePtr,
                                                    mut end: xmlNodePtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>()
                                                          as std::os::raw::c_ulong) as
            xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPtrErrMemory(b"allocating locationset\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlXPathObjectPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXPathObject>() as std::os::raw::c_ulong);
    (*ret).type_0 = XPATH_LOCATIONSET;
    if end.is_null() {
        (*ret).user =
            xmlXPtrLocationSetCreate(xmlXPtrNewCollapsedRange(start)) as
                *mut std::os::raw::c_void
    } else {
        (*ret).user =
            xmlXPtrLocationSetCreate(xmlXPtrNewRangeNodes(start, end)) as
                *mut std::os::raw::c_void
    }
    return ret;
}
/* *
 * xmlXPtrNewLocationSetNodeSet:
 * @set:  a node set
 *
 * Create a new xmlXPathObjectPtr of type LocationSet and initialize
 * it with all the nodes from @set
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewLocationSetNodeSet(mut set: xmlNodeSetPtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>()
                                                          as std::os::raw::c_ulong) as
            xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPtrErrMemory(b"allocating locationset\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlXPathObjectPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXPathObject>() as std::os::raw::c_ulong);
    (*ret).type_0 = XPATH_LOCATIONSET;
    if !set.is_null() {
        let mut i: std::os::raw::c_int = 0;
        let mut newset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
        newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
        if newset.is_null() { return ret }
        i = 0 as std::os::raw::c_int;
        while i < (*set).nodeNr {
            xmlXPtrLocationSetAdd(newset,
                                  xmlXPtrNewCollapsedRange(*(*set).nodeTab.offset(i
                                                                                      as
                                                                                      isize)));
            i += 1
        }
        (*ret).user = newset as *mut std::os::raw::c_void
    }
    return ret;
}
/* *
 * xmlXPtrWrapLocationSet:
 * @val:  the LocationSet value
 *
 * Wrap the LocationSet @val in a new xmlXPathObjectPtr
 *
 * Returns the newly created object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrWrapLocationSet(mut val: xmlLocationSetPtr)
 -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlXPathObject>()
                                                          as std::os::raw::c_ulong) as
            xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPtrErrMemory(b"allocating locationset\x00" as *const u8 as
                             *const std::os::raw::c_char);
        return 0 as xmlXPathObjectPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlXPathObject>() as std::os::raw::c_ulong);
    (*ret).type_0 = XPATH_LOCATIONSET;
    (*ret).user = val as *mut std::os::raw::c_void;
    return ret;
}
/*
 * xmlXPtrGetChildNo:
 * @ctxt:  the XPointer Parser context
 * @index:  the child number
 *
 * Move the current node of the nodeset on the stack to the
 * given child if found
 */
unsafe extern "C" fn xmlXPtrGetChildNo(mut ctxt: xmlXPathParserContextPtr,
                                       mut indx: std::os::raw::c_int) {
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut oldset: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    obj = valuePop(ctxt);
    oldset = (*obj).nodesetval;
    if indx <= 0 as std::os::raw::c_int || oldset.is_null() ||
           (*oldset).nodeNr != 1 as std::os::raw::c_int {
        xmlXPathFreeObject(obj);
        valuePush(ctxt, xmlXPathNewNodeSet(0 as xmlNodePtr));
        return
    }
    cur =
        xmlXPtrGetNthChild(*(*oldset).nodeTab.offset(0 as std::os::raw::c_int as
                                                         isize), indx);
    if cur.is_null() {
        xmlXPathFreeObject(obj);
        valuePush(ctxt, xmlXPathNewNodeSet(0 as xmlNodePtr));
        return
    }
    let ref mut fresh8 = *(*oldset).nodeTab.offset(0 as std::os::raw::c_int as isize);
    *fresh8 = cur;
    valuePush(ctxt, obj);
}
/* *
 * xmlXPtrEvalXPtrPart:
 * @ctxt:  the XPointer Parser context
 * @name:  the preparsed Scheme for the XPtrPart
 *
 * XPtrPart ::= 'xpointer' '(' XPtrExpr ')'
 *            | Scheme '(' SchemeSpecificExpr ')'
 *
 * Scheme   ::=  NCName - 'xpointer' [VC: Non-XPointer schemes]
 *
 * SchemeSpecificExpr ::= StringWithBalancedParens
 *
 * StringWithBalancedParens ::=
 *              [^()]* ('(' StringWithBalancedParens ')' [^()]*)*
 *              [VC: Parenthesis escaping]
 *
 * XPtrExpr ::= Expr [VC: Parenthesis escaping]
 *
 * VC: Parenthesis escaping:
 *   The end of an XPointer part is signaled by the right parenthesis ")"
 *   character that is balanced with the left parenthesis "(" character
 *   that began the part. Any unbalanced parenthesis character inside the
 *   expression, even within literals, must be escaped with a circumflex (^)
 *   character preceding it. If the expression contains any literal
 *   occurrences of the circumflex, each must be escaped with an additional
 *   circumflex (that is, ^^). If the unescaped parentheses in the expression
 *   are not balanced, a syntax error results.
 *
 * Parse and evaluate an XPtrPart. Basically it generates the unescaped
 * string and if the scheme is 'xpointer' it will call the XPath interpreter.
 *
 * TODO: there is no new scheme registration mechanism
 */
unsafe extern "C" fn xmlXPtrEvalXPtrPart(mut ctxt: xmlXPathParserContextPtr,
                                         mut name: *mut xmlChar) {
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: *mut xmlChar = 0 as *mut xmlChar;
    let mut len: std::os::raw::c_int = 0;
    let mut level: std::os::raw::c_int = 0;
    if name.is_null() { name = xmlXPathParseName(ctxt) }
    if name.is_null() {
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
        return
    }
    if *(*ctxt).cur as std::os::raw::c_int != '(' as i32 {
        xmlFree.expect("non-null function pointer")(name as
                                                        *mut std::os::raw::c_void);
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
        return
    }
    if *(*ctxt).cur as std::os::raw::c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1)
    } else { };
    level = 1 as std::os::raw::c_int;
    len = xmlStrlen((*ctxt).cur);
    len += 1;
    buffer =
        xmlMallocAtomic.expect("non-null function pointer")((len as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong))
            as *mut xmlChar;
    if buffer.is_null() {
        xmlXPtrErrMemory(b"allocating buffer\x00" as *const u8 as
                             *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(name as
                                                        *mut std::os::raw::c_void);
        return
    }
    cur = buffer;
    while *(*ctxt).cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *(*ctxt).cur as std::os::raw::c_int == ')' as i32 {
            level -= 1;
            if level == 0 as std::os::raw::c_int {
                if *(*ctxt).cur as std::os::raw::c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1)
                } else { };
                break ;
            }
        } else if *(*ctxt).cur as std::os::raw::c_int == '(' as i32 {
            level += 1
        } else if *(*ctxt).cur as std::os::raw::c_int == '^' as i32 {
            if *(*ctxt).cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                   == ')' as i32 ||
                   *(*ctxt).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == '(' as i32 ||
                   *(*ctxt).cur.offset(1 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int == '^' as i32 {
                if *(*ctxt).cur as std::os::raw::c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1)
                } else { };
            }
        }
        let fresh9 = cur;
        cur = cur.offset(1);
        *fresh9 = *(*ctxt).cur;
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
    }
    *cur = 0 as std::os::raw::c_int as xmlChar;
    if level != 0 as std::os::raw::c_int &&
           *(*ctxt).cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
        xmlFree.expect("non-null function pointer")(name as
                                                        *mut std::os::raw::c_void);
        xmlFree.expect("non-null function pointer")(buffer as
                                                        *mut std::os::raw::c_void);
        xmlXPathErr(ctxt, XPTR_SYNTAX_ERROR as std::os::raw::c_int);
        return
    }
    if xmlStrEqual(name,
                   b"xpointer\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar) != 0 {
        let mut left: *const xmlChar = (*ctxt).cur;
        (*ctxt).cur = buffer;
        /*
	 * To evaluate an xpointer scheme element (4.3) we need:
	 *   context initialized to the root
	 *   context position initalized to 1
	 *   context size initialized to 1
	 */
        (*(*ctxt).context).node = (*(*ctxt).context).doc as xmlNodePtr;
        (*(*ctxt).context).proximityPosition = 1 as std::os::raw::c_int;
        (*(*ctxt).context).contextSize = 1 as std::os::raw::c_int;
        xmlXPathEvalExpr(ctxt);
        (*ctxt).cur = left
    } else if xmlStrEqual(name,
                          b"element\x00" as *const u8 as *const std::os::raw::c_char
                              as *mut xmlChar) != 0 {
        let mut left_0: *const xmlChar = (*ctxt).cur;
        let mut name2: *mut xmlChar = 0 as *mut xmlChar;
        (*ctxt).cur = buffer;
        if *buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '/' as i32 {
            xmlXPathRoot(ctxt);
            xmlXPtrEvalChildSeq(ctxt, 0 as *mut xmlChar);
        } else {
            name2 = xmlXPathParseName(ctxt);
            if name2.is_null() {
                (*ctxt).cur = left_0;
                xmlFree.expect("non-null function pointer")(buffer as
                                                                *mut std::os::raw::c_void);
                xmlFree.expect("non-null function pointer")(name as
                                                                *mut std::os::raw::c_void);
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
                return
            }
            xmlXPtrEvalChildSeq(ctxt, name2);
        }
        (*ctxt).cur = left_0
    } else if xmlStrEqual(name,
                          b"xmlns\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar) != 0 {
        let mut left_1: *const xmlChar = (*ctxt).cur;
        let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
        let mut URI: *mut xmlChar = 0 as *mut xmlChar;
        let mut value: xmlURIPtr = 0 as *mut xmlURI;
        (*ctxt).cur = buffer;
        prefix = xmlXPathParseNCName(ctxt);
        if prefix.is_null() {
            xmlFree.expect("non-null function pointer")(buffer as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
            xmlXPathErr(ctxt, XPTR_SYNTAX_ERROR as std::os::raw::c_int);
            return
        }
        while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                      *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if *(*ctxt).cur as std::os::raw::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else { };
        }
        if *(*ctxt).cur as std::os::raw::c_int != '=' as i32 {
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(buffer as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
            xmlXPathErr(ctxt, XPTR_SYNTAX_ERROR as std::os::raw::c_int);
            return
        }
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
        while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                      *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if *(*ctxt).cur as std::os::raw::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else { };
        }
        /* XPTR_XMLNS_SCHEME */
        value = xmlParseURI((*ctxt).cur as *const std::os::raw::c_char);
        if value.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(buffer as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
            xmlXPathErr(ctxt, XPTR_SYNTAX_ERROR as std::os::raw::c_int);
            return
        }
        URI = xmlSaveUri(value);
        xmlFreeURI(value);
        if URI.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(buffer as
                                                            *mut std::os::raw::c_void);
            xmlFree.expect("non-null function pointer")(name as
                                                            *mut std::os::raw::c_void);
            xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
            return
        }
        xmlXPathRegisterNs((*ctxt).context, prefix, URI);
        (*ctxt).cur = left_1;
        xmlFree.expect("non-null function pointer")(URI as *mut std::os::raw::c_void);
        xmlFree.expect("non-null function pointer")(prefix as
                                                        *mut std::os::raw::c_void);
    } else {
        xmlXPtrErr(ctxt, XML_XPTR_UNKNOWN_SCHEME as std::os::raw::c_int,
                   b"unsupported scheme \'%s\'\n\x00" as *const u8 as
                       *const std::os::raw::c_char, name);
    }
    xmlFree.expect("non-null function pointer")(buffer as *mut std::os::raw::c_void);
    xmlFree.expect("non-null function pointer")(name as *mut std::os::raw::c_void);
}
/* @@ check escaping in the XPointer WD */
/* *
 * xmlXPtrEvalFullXPtr:
 * @ctxt:  the XPointer Parser context
 * @name:  the preparsed Scheme for the first XPtrPart
 *
 * FullXPtr ::= XPtrPart (S? XPtrPart)*
 *
 * As the specs says:
 * -----------
 * When multiple XPtrParts are provided, they must be evaluated in
 * left-to-right order. If evaluation of one part fails, the nexti
 * is evaluated. The following conditions cause XPointer part failure:
 *
 * - An unknown scheme
 * - A scheme that does not locate any sub-resource present in the resource
 * - A scheme that is not applicable to the media type of the resource
 *
 * The XPointer application must consume a failed XPointer part and
 * attempt to evaluate the next one, if any. The result of the first
 * XPointer part whose evaluation succeeds is taken to be the fragment
 * located by the XPointer as a whole. If all the parts fail, the result
 * for the XPointer as a whole is a sub-resource error.
 * -----------
 *
 * Parse and evaluate a Full XPtr i.e. possibly a cascade of XPath based
 * expressions or other schemes.
 */
unsafe extern "C" fn xmlXPtrEvalFullXPtr(mut ctxt: xmlXPathParserContextPtr,
                                         mut name: *mut xmlChar) {
    if name.is_null() { name = xmlXPathParseName(ctxt) }
    if name.is_null() {
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
        return
    }
    while !name.is_null() {
        (*ctxt).error = XPATH_EXPRESSION_OK as std::os::raw::c_int;
        xmlXPtrEvalXPtrPart(ctxt, name);
        /* in case of syntax error, break here */
        if (*ctxt).error != XPATH_EXPRESSION_OK as std::os::raw::c_int &&
               (*ctxt).error != XML_XPTR_UNKNOWN_SCHEME as std::os::raw::c_int {
            return
        }
        /*
	 * If the returned value is a non-empty nodeset
	 * or location set, return here.
	 */
        if !(*ctxt).value.is_null() {
            let mut obj: xmlXPathObjectPtr = (*ctxt).value;
            match (*obj).type_0 as std::os::raw::c_uint {
                7 => {
                    let mut loc: xmlLocationSetPtr =
                        (*(*ctxt).value).user as xmlLocationSetPtr;
                    if !loc.is_null() && (*loc).locNr > 0 as std::os::raw::c_int {
                        return
                    }
                }
                1 => {
                    let mut loc_0: xmlNodeSetPtr =
                        (*(*ctxt).value).nodesetval;
                    if !loc_0.is_null() && (*loc_0).nodeNr > 0 as std::os::raw::c_int
                       {
                        return
                    }
                }
                _ => { }
            }
            loop 
                 /*
	     * Evaluating to improper values is equivalent to
	     * a sub-resource error, clean-up the stack
	     */
                 {
                obj = valuePop(ctxt);
                if !obj.is_null() { xmlXPathFreeObject(obj); }
                if obj.is_null() { break ; }
            }
        }
        /*
	 * Is there another XPointer part.
	 */
        while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                      *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if *(*ctxt).cur as std::os::raw::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else { };
        }
        name = xmlXPathParseName(ctxt)
    };
}
/* ***********************************************************************
 *									*
 *			The parser					*
 *									*
 ************************************************************************/
/* *
 * xmlXPtrEvalChildSeq:
 * @ctxt:  the XPointer Parser context
 * @name:  a possible ID name of the child sequence
 *
 *  ChildSeq ::= '/1' ('/' [0-9]*)*
 *             | Name ('/' [0-9]*)+
 *
 * Parse and evaluate a Child Sequence. This routine also handle the
 * case of a Bare Name used to get a document ID.
 */
unsafe extern "C" fn xmlXPtrEvalChildSeq(mut ctxt: xmlXPathParserContextPtr,
                                         mut name: *mut xmlChar) {
    /*
     * XPointer don't allow by syntax to address in mutirooted trees
     * this might prove useful in some cases, warn about it.
     */
    if name.is_null() && *(*ctxt).cur as std::os::raw::c_int == '/' as i32 &&
           *(*ctxt).cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               '1' as i32 {
        xmlXPtrErr(ctxt, XML_XPTR_CHILDSEQ_START as std::os::raw::c_int,
                   b"warning: ChildSeq not starting by /1\n\x00" as *const u8
                       as *const std::os::raw::c_char, 0 as *const xmlChar);
    }
    if !name.is_null() {
        valuePush(ctxt, xmlXPathNewString(name));
        xmlFree.expect("non-null function pointer")(name as
                                                        *mut std::os::raw::c_void);
        xmlXPathIdFunction(ctxt, 1 as std::os::raw::c_int);
        if (*ctxt).error != XPATH_EXPRESSION_OK as std::os::raw::c_int { return }
    }
    while *(*ctxt).cur as std::os::raw::c_int == '/' as i32 {
        let mut child: std::os::raw::c_int = 0 as std::os::raw::c_int;
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
        while *(*ctxt).cur as std::os::raw::c_int >= '0' as i32 &&
                  *(*ctxt).cur as std::os::raw::c_int <= '9' as i32 {
            child =
                child * 10 as std::os::raw::c_int +
                    (*(*ctxt).cur as std::os::raw::c_int - '0' as i32);
            if *(*ctxt).cur as std::os::raw::c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1)
            } else { };
        }
        xmlXPtrGetChildNo(ctxt, child);
    };
}
/* *
 * xmlXPtrEvalXPointer:
 * @ctxt:  the XPointer Parser context
 *
 *  XPointer ::= Name
 *             | ChildSeq
 *             | FullXPtr
 *
 * Parse and evaluate an XPointer
 */
unsafe extern "C" fn xmlXPtrEvalXPointer(mut ctxt: xmlXPathParserContextPtr) {
    if (*ctxt).valueTab.is_null() {
        /* Allocate the value stack */
        (*ctxt).valueTab =
            xmlMalloc.expect("non-null function pointer")((10 as std::os::raw::c_int
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlXPathObjectPtr>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut xmlXPathObjectPtr;
        if (*ctxt).valueTab.is_null() {
            xmlXPtrErrMemory(b"allocating evaluation context\x00" as *const u8
                                 as *const std::os::raw::c_char);
            return
        }
        (*ctxt).valueNr = 0 as std::os::raw::c_int;
        (*ctxt).valueMax = 10 as std::os::raw::c_int;
        (*ctxt).value = 0 as xmlXPathObjectPtr;
        (*ctxt).valueFrame = 0 as std::os::raw::c_int
    }
    while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                  *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
    }
    if *(*ctxt).cur as std::os::raw::c_int == '/' as i32 {
        xmlXPathRoot(ctxt);
        xmlXPtrEvalChildSeq(ctxt, 0 as *mut xmlChar);
    } else {
        let mut name: *mut xmlChar = 0 as *mut xmlChar;
        name = xmlXPathParseName(ctxt);
        if name.is_null() {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
            return
        }
        if *(*ctxt).cur as std::os::raw::c_int == '(' as i32 {
            xmlXPtrEvalFullXPtr(ctxt, name);
            /* Short evaluation */
            return
        } else {
            /* this handle both Bare Names and Child Sequences */
            xmlXPtrEvalChildSeq(ctxt, name);
        }
    }
    while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                  *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
    }
    if *(*ctxt).cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
        return
    };
}
/* *
 * xmlXPtrNewContext:
 * @doc:  the XML document
 * @here:  the node that directly contains the XPointer being evaluated or NULL
 * @origin:  the element from which a user or program initiated traversal of
 *           the link, or NULL.
 *
 * Create a new XPointer context
 *
 * Returns the xmlXPathContext just allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrNewContext(mut doc: xmlDocPtr,
                                           mut here: xmlNodePtr,
                                           mut origin: xmlNodePtr)
 -> xmlXPathContextPtr {
    let mut ret: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    ret = xmlXPathNewContext(doc);
    if ret.is_null() { return ret }
    (*ret).xptr = 1 as std::os::raw::c_int;
    (*ret).here = here;
    (*ret).origin = origin;
    xmlXPathRegisterFunc(ret,
                         b"range\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         Some(xmlXPtrRangeFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    xmlXPathRegisterFunc(ret,
                         b"range-inside\x00" as *const u8 as
                             *const std::os::raw::c_char as *mut xmlChar,
                         Some(xmlXPtrRangeInsideFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    xmlXPathRegisterFunc(ret,
                         b"string-range\x00" as *const u8 as
                             *const std::os::raw::c_char as *mut xmlChar,
                         Some(xmlXPtrStringRangeFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    xmlXPathRegisterFunc(ret,
                         b"start-point\x00" as *const u8 as
                             *const std::os::raw::c_char as *mut xmlChar,
                         Some(xmlXPtrStartPointFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    xmlXPathRegisterFunc(ret,
                         b"end-point\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut xmlChar,
                         Some(xmlXPtrEndPointFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    xmlXPathRegisterFunc(ret,
                         b"here\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         Some(xmlXPtrHereFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    xmlXPathRegisterFunc(ret,
                         b" origin\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut xmlChar,
                         Some(xmlXPtrOriginFunction as
                                  unsafe extern "C" fn(_:
                                                           xmlXPathParserContextPtr,
                                                       _: std::os::raw::c_int)
                                      -> ()));
    return ret;
}
/* *
 * xmlXPtrEval:
 * @str:  the XPointer expression
 * @ctx:  the XPointer context
 *
 * Evaluate the XPath Location Path in the given context.
 *
 * Returns the xmlXPathObjectPtr resulting from the evaluation or NULL.
 *         the caller has to free the object.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrEval(mut str: *const xmlChar,
                                     mut ctx: xmlXPathContextPtr)
 -> xmlXPathObjectPtr {
    let mut ctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut res: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut init: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut stack: std::os::raw::c_int = 0 as std::os::raw::c_int;
    xmlXPathInit();
    if ctx.is_null() || str.is_null() { return 0 as xmlXPathObjectPtr }
    ctxt = xmlXPathNewParserContext(str, ctx);
    if ctxt.is_null() { return 0 as xmlXPathObjectPtr }
    (*ctxt).xptr = 1 as std::os::raw::c_int;
    xmlXPtrEvalXPointer(ctxt);
    if !(*ctxt).value.is_null() &&
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint &&
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPtrErr(ctxt, XML_XPTR_EVAL_FAILED as std::os::raw::c_int,
                   b"xmlXPtrEval: evaluation failed to return a node set\n\x00"
                       as *const u8 as *const std::os::raw::c_char,
                   0 as *const xmlChar);
    } else { res = valuePop(ctxt) }
    loop  {
        tmp = valuePop(ctxt);
        if !tmp.is_null() {
            if tmp != init {
                if (*tmp).type_0 as std::os::raw::c_uint ==
                       XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
                    /*
		     * Evaluation may push a root nodeset which is unused
		     */
                    let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
                    set = (*tmp).nodesetval;
                    if set.is_null() || (*set).nodeNr != 1 as std::os::raw::c_int ||
                           *(*set).nodeTab.offset(0 as std::os::raw::c_int as isize)
                               != (*ctx).doc as xmlNodePtr {
                        stack += 1
                    }
                } else { stack += 1 }
            }
            xmlXPathFreeObject(tmp);
        }
        if tmp.is_null() { break ; }
    }
    if stack != 0 as std::os::raw::c_int {
        xmlXPtrErr(ctxt, XML_XPTR_EXTRA_OBJECTS as std::os::raw::c_int,
                   b"xmlXPtrEval: object(s) left on the eval stack\n\x00" as
                       *const u8 as *const std::os::raw::c_char, 0 as *const xmlChar);
    }
    if (*ctxt).error != XPATH_EXPRESSION_OK as std::os::raw::c_int {
        xmlXPathFreeObject(res);
        res = 0 as xmlXPathObjectPtr
    }
    xmlXPathFreeParserContext(ctxt);
    return res;
}
/* *
 * xmlXPtrBuildRangeNodeList:
 * @range:  a range object
 *
 * Build a node list tree copy of the range
 *
 * Returns an xmlNodePtr list or NULL.
 *         the caller has to free the node tree.
 */
unsafe extern "C" fn xmlXPtrBuildRangeNodeList(mut range: xmlXPathObjectPtr)
 -> xmlNodePtr {
    /* pointers to generated nodes */
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut parent: xmlNodePtr = 0 as xmlNodePtr;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    /* pointers to traversal nodes */
    let mut start: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut end: xmlNodePtr = 0 as *mut xmlNode;
    let mut index1: std::os::raw::c_int = 0;
    let mut index2: std::os::raw::c_int = 0;
    if range.is_null() { return 0 as xmlNodePtr }
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
    if end.is_null() { return xmlCopyNode(start, 1 as std::os::raw::c_int) }
    if (*end).type_0 as std::os::raw::c_uint ==
           XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlNodePtr
    }
    cur = start;
    index1 = (*range).index;
    index2 = (*range).index2;
    /*
		     * Now gather the remaining nodes from cur to end
		     */
    while !cur.is_null() {
        if cur == end {
            if (*cur).type_0 as std::os::raw::c_uint ==
                   XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                let mut content: *const xmlChar = (*cur).content;
                let mut len: std::os::raw::c_int = 0;
                if content.is_null() {
                    tmp = xmlNewTextLen(0 as *const xmlChar, 0 as std::os::raw::c_int)
                } else {
                    len = index2;
                    if cur == start && index1 > 1 as std::os::raw::c_int {
                        content =
                            content.offset((index1 - 1 as std::os::raw::c_int) as
                                               isize);
                        len -= index1 - 1 as std::os::raw::c_int;
                        index1 = 0 as std::os::raw::c_int
                    } else { len = index2 }
                    tmp = xmlNewTextLen(content, len)
                }
                /* single sub text node selection */
                if list.is_null() { return tmp }
                /* prune and return full set */
                if !last.is_null() {
                    xmlAddNextSibling(last, tmp);
                } else { xmlAddChild(parent, tmp); }
                return list
            } else {
                tmp = xmlCopyNode(cur, 0 as std::os::raw::c_int);
                if list.is_null() {
                    list = tmp
                } else if !last.is_null() {
                    xmlAddNextSibling(last, tmp);
                } else { xmlAddChild(parent, tmp); }
                last = 0 as xmlNodePtr;
                parent = tmp;
                if index2 > 1 as std::os::raw::c_int {
                    end = xmlXPtrGetNthChild(cur, index2 - 1 as std::os::raw::c_int);
                    index2 = 0 as std::os::raw::c_int
                }
                if cur == start && index1 > 1 as std::os::raw::c_int {
                    cur = xmlXPtrGetNthChild(cur, index1 - 1 as std::os::raw::c_int);
                    index1 = 0 as std::os::raw::c_int
                } else { cur = (*cur).children }
                /* while */
            }
        } else {
            if cur == start && list.is_null() {
                /* looks superfluous but ... */
                if (*cur).type_0 as std::os::raw::c_uint ==
                       XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                    let mut content_0: *const xmlChar = (*cur).content;
                    if content_0.is_null() {
                        tmp =
                            xmlNewTextLen(0 as *const xmlChar,
                                          0 as std::os::raw::c_int)
                    } else {
                        if index1 > 1 as std::os::raw::c_int {
                            content_0 =
                                content_0.offset((index1 - 1 as std::os::raw::c_int)
                                                     as isize)
                        }
                        tmp = xmlNewText(content_0)
                    }
                    list = tmp;
                    last = list
                } else if cur == start && index1 > 1 as std::os::raw::c_int {
                    tmp = xmlCopyNode(cur, 0 as std::os::raw::c_int);
                    list = tmp;
                    parent = tmp;
                    last = 0 as xmlNodePtr;
                    cur = xmlXPtrGetNthChild(cur, index1 - 1 as std::os::raw::c_int);
                    index1 = 0 as std::os::raw::c_int;
                    continue ;
                    /* while */
                } else {
                    tmp =
                        xmlCopyNode(cur,
                                    1 as
                                        std::os::raw::c_int); /* handle crossing entities -> stack needed */
                    list = tmp;
                    parent = 0 as xmlNodePtr;
                    last = tmp
                }
            } else {
                tmp = 0 as xmlNodePtr;
                match (*cur).type_0 as std::os::raw::c_uint {
                    14 | 15 | 16 | 6 => { }
                    17 => {
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   b"xpointer.c\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   1532
                                                                                       as
                                                                                       std::os::raw::c_int);
                    }
                    19 | 20 => { }
                    2 => {
                        /* Humm, should not happen ! */
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Internal error at %s:%d\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   b"xpointer.c\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   1540
                                                                                       as
                                                                                       std::os::raw::c_int);
                    }
                    _ => { tmp = xmlCopyNode(cur, 1 as std::os::raw::c_int) }
                }
                if !tmp.is_null() {
                    if list.is_null() || last.is_null() && parent.is_null() {
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Internal error at %s:%d\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   b"xpointer.c\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   1548
                                                                                       as
                                                                                       std::os::raw::c_int);
                        return 0 as xmlNodePtr
                    }
                    if !last.is_null() {
                        xmlAddNextSibling(last, tmp);
                    } else { xmlAddChild(parent, tmp); last = tmp }
                }
            }
            /*
	 * Skip to next node in document order
	 */
            if list.is_null() || last.is_null() && parent.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Internal error at %s:%d\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           b"xpointer.c\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           1563
                                                                               as
                                                                               std::os::raw::c_int);
                return 0 as xmlNodePtr
            }
            cur = xmlXPtrAdvanceNode(cur, 0 as *mut std::os::raw::c_int)
        }
    }
    return list;
}
/* *
 * xmlXPtrBuildNodeList:
 * @obj:  the XPointer result from the evaluation.
 *
 * Build a node list tree copy of the XPointer result.
 * This will drop Attributes and Namespace declarations.
 *
 * Returns an xmlNodePtr list or NULL.
 *         the caller has to free the node tree.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrBuildNodeList(mut obj: xmlXPathObjectPtr)
 -> xmlNodePtr {
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut i: std::os::raw::c_int = 0;
    if obj.is_null() { return 0 as xmlNodePtr }
    match (*obj).type_0 as std::os::raw::c_uint {
        1 => {
            let mut set: xmlNodeSetPtr = (*obj).nodesetval;
            if set.is_null() { return 0 as xmlNodePtr }
            i = 0 as std::os::raw::c_int;
            while i < (*set).nodeNr {
                if !(*(*set).nodeTab.offset(i as isize)).is_null() {
                    match (**(*set).nodeTab.offset(i as isize)).type_0 as
                              std::os::raw::c_uint {
                        2 | 18 | 10 | 11 | 12 | 14 | 15 | 16 | 17 => { }
                        3 | 4 | 1 | 5 | 6 | 7 | 8 | 9 | 13 | 21 | 19 | 20 | _
                        => {
                            if last.is_null() {
                                last =
                                    xmlCopyNode(*(*set).nodeTab.offset(i as
                                                                           isize),
                                                1 as std::os::raw::c_int);
                                list = last
                            } else {
                                xmlAddNextSibling(last,
                                                  xmlCopyNode(*(*set).nodeTab.offset(i
                                                                                         as
                                                                                         isize),
                                                              1 as
                                                                  std::os::raw::c_int));
                                if !(*last).next.is_null() {
                                    last = (*last).next
                                }
                            }
                            /* for */
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
                        xmlXPtrBuildNodeList(*(*set_0).locTab.offset(i as
                                                                         isize));
                    list = last
                } else {
                    xmlAddNextSibling(last,
                                      xmlXPtrBuildNodeList(*(*set_0).locTab.offset(i
                                                                                       as
                                                                                       isize)));
                }
                if !last.is_null() {
                    while !(*last).next.is_null() { last = (*last).next }
                }
                i += 1
            }
        }
        6 => { return xmlXPtrBuildRangeNodeList(obj) }
        5 => {
            return xmlCopyNode((*obj).user as xmlNodePtr, 0 as std::os::raw::c_int)
        }
        _ => { }
    }
    return list;
}
/* ***********************************************************************
 *									*
 *			XPointer functions				*
 *									*
 ************************************************************************/
/* *
 * xmlXPtrNbLocChildren:
 * @node:  an xmlNodePtr
 *
 * Count the number of location children of @node or the length of the
 * string value in case of text/PI/Comments nodes
 *
 * Returns the number of location children
 */
unsafe extern "C" fn xmlXPtrNbLocChildren(mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if node.is_null() { return -(1 as std::os::raw::c_int) }
    match (*node).type_0 as std::os::raw::c_uint {
        13 | 9 | 1 => {
            node = (*node).children;
            while !node.is_null() {
                if (*node).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    ret += 1
                }
                node = (*node).next
            }
        }
        2 => { return -(1 as std::os::raw::c_int) }
        7 | 8 | 3 | 4 | 5 => { ret = xmlStrlen((*node).content) }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    return ret;
}
/* *
 * xmlXPtrHereFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing here() operation
 * as described in 5.4.3
 */
unsafe extern "C" fn xmlXPtrHereFunction(mut ctxt: xmlXPathParserContextPtr,
                                         mut nargs: std::os::raw::c_int) {
    if ctxt.is_null() { return }
    if nargs != 0 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 0 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as std::os::raw::c_int);
        return
    }
    if (*(*ctxt).context).here.is_null() {
        xmlXPathErr(ctxt, XPTR_SYNTAX_ERROR as std::os::raw::c_int);
        return
    }
    valuePush(ctxt,
              xmlXPtrNewLocationSetNodes((*(*ctxt).context).here,
                                         0 as xmlNodePtr));
}
/* *
 * xmlXPtrOriginFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing origin() operation
 * as described in 5.4.3
 */
unsafe extern "C" fn xmlXPtrOriginFunction(mut ctxt: xmlXPathParserContextPtr,
                                           mut nargs: std::os::raw::c_int) {
    if ctxt.is_null() { return }
    if nargs != 0 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 0 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as std::os::raw::c_int);
        return
    }
    if (*(*ctxt).context).origin.is_null() {
        xmlXPathErr(ctxt, XPTR_SYNTAX_ERROR as std::os::raw::c_int);
        return
    }
    valuePush(ctxt,
              xmlXPtrNewLocationSetNodes((*(*ctxt).context).origin,
                                         0 as xmlNodePtr));
}
/* *
 * xmlXPtrStartPointFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing start-point() operation
 * as described in 5.4.3
 * ----------------
 * location-set start-point(location-set)
 *
 * For each location x in the argument location-set, start-point adds a
 * location of type point to the result location-set. That point represents
 * the start point of location x and is determined by the following rules:
 *
 * - If x is of type point, the start point is x.
 * - If x is of type range, the start point is the start point of x.
 * - If x is of type root, element, text, comment, or processing instruction,
 * - the container node of the start point is x and the index is 0.
 * - If x is of type attribute or namespace, the function must signal a
 *   syntax error.
 * ----------------
 *
 */
unsafe extern "C" fn xmlXPtrStartPointFunction(mut ctxt:
                                                   xmlXPathParserContextPtr,
                                               mut nargs: std::os::raw::c_int) {
    let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut point: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut newset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
    let mut oldset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
    if ctxt.is_null() { return }
    if nargs != 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as std::os::raw::c_int);
        return
    }
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    obj = valuePop(ctxt);
    if (*obj).type_0 as std::os::raw::c_uint ==
           XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        /*
	 * First convert to a location set
	 */
        tmp = xmlXPtrNewLocationSetNodeSet((*obj).nodesetval);
        xmlXPathFreeObject(obj);
        if tmp.is_null() {
            xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
            return
        }
        obj = tmp
    }
    newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
    if newset.is_null() {
        xmlXPathFreeObject(obj);
        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
        return
    }
    oldset = (*obj).user as xmlLocationSetPtr;
    if !oldset.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < (*oldset).locNr {
            tmp = *(*oldset).locTab.offset(i as isize);
            if !tmp.is_null() {
                point = 0 as xmlXPathObjectPtr;
                match (*tmp).type_0 as std::os::raw::c_uint {
                    5 => {
                        point =
                            xmlXPtrNewPoint((*tmp).user as xmlNodePtr,
                                            (*tmp).index)
                    }
                    6 => {
                        let mut node: xmlNodePtr = (*tmp).user as xmlNodePtr;
                        if !node.is_null() {
                            if (*node).type_0 as std::os::raw::c_uint ==
                                   XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint ||
                                   (*node).type_0 as std::os::raw::c_uint ==
                                       XML_NAMESPACE_DECL as std::os::raw::c_int as
                                           std::os::raw::c_uint {
                                xmlXPathFreeObject(obj);
                                xmlXPtrFreeLocationSet(newset);
                                xmlXPathErr(ctxt,
                                            XPTR_SYNTAX_ERROR as std::os::raw::c_int);
                                return
                            }
                            point = xmlXPtrNewPoint(node, (*tmp).index)
                        }
                    }
                    _ => { }
                }
                if !point.is_null() { xmlXPtrLocationSetAdd(newset, point); }
            }
            i += 1
        }
    }
    xmlXPathFreeObject(obj);
    valuePush(ctxt, xmlXPtrWrapLocationSet(newset));
}
/* *
 * xmlXPtrEndPointFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing end-point() operation
 * as described in 5.4.3
 * ----------------------------
 * location-set end-point(location-set)
 *
 * For each location x in the argument location-set, end-point adds a
 * location of type point to the result location-set. That point represents
 * the end point of location x and is determined by the following rules:
 *
 * - If x is of type point, the resulting point is x.
 * - If x is of type range, the resulting point is the end point of x.
 * - If x is of type root or element, the container node of the resulting
 *   point is x and the index is the number of location children of x.
 * - If x is of type text, comment, or processing instruction, the container
 *   node of the resulting point is x and the index is the length of the
 *   string-value of x.
 * - If x is of type attribute or namespace, the function must signal a
 *   syntax error.
 * ----------------------------
 */
unsafe extern "C" fn xmlXPtrEndPointFunction(mut ctxt:
                                                 xmlXPathParserContextPtr,
                                             mut nargs: std::os::raw::c_int) {
    let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut point: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut newset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
    let mut oldset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
    if ctxt.is_null() { return }
    if nargs != 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as std::os::raw::c_int);
        return
    }
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    obj = valuePop(ctxt);
    if (*obj).type_0 as std::os::raw::c_uint ==
           XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        /*
	 * First convert to a location set
	 */
        tmp = xmlXPtrNewLocationSetNodeSet((*obj).nodesetval);
        xmlXPathFreeObject(obj);
        if tmp.is_null() {
            xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
            return
        }
        obj = tmp
    }
    newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
    if newset.is_null() {
        xmlXPathFreeObject(obj);
        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
        return
    }
    oldset = (*obj).user as xmlLocationSetPtr;
    if !oldset.is_null() {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < (*oldset).locNr {
            tmp = *(*oldset).locTab.offset(i as isize);
            if !tmp.is_null() {
                point = 0 as xmlXPathObjectPtr;
                match (*tmp).type_0 as std::os::raw::c_uint {
                    5 => {
                        point =
                            xmlXPtrNewPoint((*tmp).user as xmlNodePtr,
                                            (*tmp).index)
                    }
                    6 => {
                        let mut node: xmlNodePtr = (*tmp).user2 as xmlNodePtr;
                        if !node.is_null() {
                            if (*node).type_0 as std::os::raw::c_uint ==
                                   XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                       std::os::raw::c_uint ||
                                   (*node).type_0 as std::os::raw::c_uint ==
                                       XML_NAMESPACE_DECL as std::os::raw::c_int as
                                           std::os::raw::c_uint {
                                xmlXPathFreeObject(obj);
                                xmlXPtrFreeLocationSet(newset);
                                xmlXPathErr(ctxt,
                                            XPTR_SYNTAX_ERROR as std::os::raw::c_int);
                                return
                            }
                            point = xmlXPtrNewPoint(node, (*tmp).index2)
                        } else if (*tmp).user.is_null() {
                            point =
                                xmlXPtrNewPoint(node,
                                                xmlXPtrNbLocChildren(node))
                        }
                    }
                    _ => { }
                }
                if !point.is_null() { xmlXPtrLocationSetAdd(newset, point); }
            }
            i += 1
        }
    }
    xmlXPathFreeObject(obj);
    valuePush(ctxt, xmlXPtrWrapLocationSet(newset));
}
/* *
 * xmlXPtrCoveringRange:
 * @ctxt:  the XPointer Parser context
 * @loc:  the location for which the covering range must be computed
 *
 * A covering range is a range that wholly encompasses a location
 * Section 5.3.3. Covering Ranges for All Location Types
 *        http://www.w3.org/TR/xptr#N2267
 *
 * Returns a new location or NULL in case of error
 */
unsafe extern "C" fn xmlXPtrCoveringRange(mut ctxt: xmlXPathParserContextPtr,
                                          mut loc: xmlXPathObjectPtr)
 -> xmlXPathObjectPtr {
    if loc.is_null() { return 0 as xmlXPathObjectPtr }
    if ctxt.is_null() || (*ctxt).context.is_null() ||
           (*(*ctxt).context).doc.is_null() {
        return 0 as xmlXPathObjectPtr
    }
    match (*loc).type_0 as std::os::raw::c_uint {
        5 => {
            return xmlXPtrNewRange((*loc).user as xmlNodePtr, (*loc).index,
                                   (*loc).user as xmlNodePtr, (*loc).index)
            /* missed one case ??? */
        }
        6 => {
            if !(*loc).user2.is_null() {
                return xmlXPtrNewRange((*loc).user as xmlNodePtr,
                                       (*loc).index,
                                       (*loc).user2 as xmlNodePtr,
                                       (*loc).index2)
            } else {
                let mut node: xmlNodePtr = (*loc).user as xmlNodePtr;
                if node == (*(*ctxt).context).doc as xmlNodePtr {
                    return xmlXPtrNewRange(node, 0 as std::os::raw::c_int, node,
                                           xmlXPtrGetArity(node))
                } else {
                    match (*node).type_0 as std::os::raw::c_uint {
                        2 => {
                            /* !!! our model is slightly different than XPath */
                            return xmlXPtrNewRange(node, 0 as std::os::raw::c_int,
                                                   node,
                                                   xmlXPtrGetArity(node))
                        }
                        1 | 3 | 4 | 5 | 7 | 8 | 9 | 12 | 13 => {
                            let mut indx: std::os::raw::c_int = xmlXPtrGetIndex(node);
                            node = (*node).parent;
                            return xmlXPtrNewRange(node,
                                                   indx - 1 as std::os::raw::c_int,
                                                   node,
                                                   indx + 1 as std::os::raw::c_int)
                        }
                        _ => { return 0 as xmlXPathObjectPtr }
                    }
                }
            }
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"xpointer.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       1995 as
                                                                           std::os::raw::c_int);
        }
    }
    return 0 as xmlXPathObjectPtr;
}
/* *
 * xmlXPtrRangeFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing the range() function 5.4.3
 *  location-set range(location-set )
 *
 *  The range function returns ranges covering the locations in
 *  the argument location-set. For each location x in the argument
 *  location-set, a range location representing the covering range of
 *  x is added to the result location-set.
 */
unsafe extern "C" fn xmlXPtrRangeFunction(mut ctxt: xmlXPathParserContextPtr,
                                          mut nargs: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut set: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut oldset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    let mut newset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    if ctxt.is_null() { return }
    if nargs != 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as std::os::raw::c_int);
        return
    }
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    set = valuePop(ctxt);
    if (*set).type_0 as std::os::raw::c_uint ==
           XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        /*
	 * First convert to a location set
	 */
        tmp = xmlXPtrNewLocationSetNodeSet((*set).nodesetval);
        xmlXPathFreeObject(set);
        if tmp.is_null() {
            xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
            return
        }
        set = tmp
    }
    oldset = (*set).user as xmlLocationSetPtr;
    /*
     * The loop is to compute the covering range for each item and add it
     */
    newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
    if newset.is_null() {
        xmlXPathFreeObject(set);
        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
        return
    }
    if !oldset.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < (*oldset).locNr {
            xmlXPtrLocationSetAdd(newset,
                                  xmlXPtrCoveringRange(ctxt,
                                                       *(*oldset).locTab.offset(i
                                                                                    as
                                                                                    isize)));
            i += 1
        }
    }
    /*
     * Save the new value and cleanup
     */
    valuePush(ctxt, xmlXPtrWrapLocationSet(newset));
    xmlXPathFreeObject(set);
}
/* *
 * xmlXPtrInsideRange:
 * @ctxt:  the XPointer Parser context
 * @loc:  the location for which the inside range must be computed
 *
 * A inside range is a range described in the range-inside() description
 *
 * Returns a new location or NULL in case of error
 */
unsafe extern "C" fn xmlXPtrInsideRange(mut ctxt: xmlXPathParserContextPtr,
                                        mut loc: xmlXPathObjectPtr)
 -> xmlXPathObjectPtr {
    if loc.is_null() { return 0 as xmlXPathObjectPtr }
    if ctxt.is_null() || (*ctxt).context.is_null() ||
           (*(*ctxt).context).doc.is_null() {
        return 0 as xmlXPathObjectPtr
    }
    match (*loc).type_0 as std::os::raw::c_uint {
        5 => {
            let mut node: xmlNodePtr = (*loc).user as xmlNodePtr;
            match (*node).type_0 as std::os::raw::c_uint {
                7 | 8 | 3 | 4 => {
                    if (*node).content.is_null() {
                        return xmlXPtrNewRange(node, 0 as std::os::raw::c_int, node,
                                               0 as std::os::raw::c_int)
                    } else {
                        return xmlXPtrNewRange(node, 0 as std::os::raw::c_int, node,
                                               xmlStrlen((*node).content))
                    }
                }
                2 | 1 | 5 | 9 | 12 | 13 => {
                    return xmlXPtrNewRange(node, 0 as std::os::raw::c_int, node,
                                           xmlXPtrGetArity(node))
                }
                _ => { }
            }
            return 0 as xmlXPathObjectPtr
            /* missed one case ??? */
        }
        6 => {
            let mut node_0: xmlNodePtr = (*loc).user as xmlNodePtr;
            if !(*loc).user2.is_null() {
                return xmlXPtrNewRange(node_0, (*loc).index,
                                       (*loc).user2 as xmlNodePtr,
                                       (*loc).index2)
            } else {
                match (*node_0).type_0 as std::os::raw::c_uint {
                    7 | 8 | 3 | 4 => {
                        if (*node_0).content.is_null() {
                            return xmlXPtrNewRange(node_0, 0 as std::os::raw::c_int,
                                                   node_0, 0 as std::os::raw::c_int)
                        } else {
                            return xmlXPtrNewRange(node_0, 0 as std::os::raw::c_int,
                                                   node_0,
                                                   xmlStrlen((*node_0).content))
                        }
                    }
                    2 | 1 | 5 | 9 | 12 | 13 => {
                        return xmlXPtrNewRange(node_0, 0 as std::os::raw::c_int,
                                               node_0,
                                               xmlXPtrGetArity(node_0))
                    }
                    _ => { }
                }
                return 0 as xmlXPathObjectPtr
            }
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"xpointer.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       2142 as
                                                                           std::os::raw::c_int);
        }
    }
    return 0 as xmlXPathObjectPtr;
}
/* *
 * xmlXPtrRangeInsideFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing the range-inside() function 5.4.3
 *  location-set range-inside(location-set )
 *
 *  The range-inside function returns ranges covering the contents of
 *  the locations in the argument location-set. For each location x in
 *  the argument location-set, a range location is added to the result
 *  location-set. If x is a range location, then x is added to the
 *  result location-set. If x is not a range location, then x is used
 *  as the container location of the start and end points of the range
 *  location to be added; the index of the start point of the range is
 *  zero; if the end point is a character point then its index is the
 *  length of the string-value of x, and otherwise is the number of
 *  location children of x.
 *
 */
unsafe extern "C" fn xmlXPtrRangeInsideFunction(mut ctxt:
                                                    xmlXPathParserContextPtr,
                                                mut nargs: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut set: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut oldset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    let mut newset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    if ctxt.is_null() { return }
    if nargs != 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as std::os::raw::c_int);
        return
    }
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    set = valuePop(ctxt);
    if (*set).type_0 as std::os::raw::c_uint ==
           XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        /*
	 * First convert to a location set
	 */
        tmp = xmlXPtrNewLocationSetNodeSet((*set).nodesetval);
        xmlXPathFreeObject(set);
        if tmp.is_null() {
            xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
            return
        }
        set = tmp
    }
    oldset = (*set).user as xmlLocationSetPtr;
    /*
     * The loop is to compute the covering range for each item and add it
     */
    newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
    if newset.is_null() {
        xmlXPathFreeObject(set);
        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
        return
    }
    i = 0 as std::os::raw::c_int;
    while i < (*oldset).locNr {
        xmlXPtrLocationSetAdd(newset,
                              xmlXPtrInsideRange(ctxt,
                                                 *(*oldset).locTab.offset(i as
                                                                              isize)));
        i += 1
    }
    /*
     * Save the new value and cleanup
     */
    valuePush(ctxt, xmlXPtrWrapLocationSet(newset));
    xmlXPathFreeObject(set);
}
/* *
 * xmlXPtrRangeToFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Implement the range-to() XPointer function
 *
 * Obsolete. range-to is not a real function but a special type of location
 * step which is handled in xpath.c.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrRangeToFunction(mut ctxt:
                                                    xmlXPathParserContextPtr,
                                                mut nargs: std::os::raw::c_int) {
    xmlXPathErr(ctxt, XPATH_EXPR_ERROR as std::os::raw::c_int);
}
/* ***********************************************************************
 *									*
 *		A few helper functions for child sequences		*
 *									*
 ************************************************************************/
/* xmlXPtrAdvanceNode is a private function, but used by xinclude.c */
/* *
 * xmlXPtrAdvanceNode:
 * @cur:  the node
 * @level: incremented/decremented to show level in tree
 *
 * Advance to the next element or text node in document order
 * TODO: add a stack for entering/exiting entities
 *
 * Returns -1 in case of failure, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrAdvanceNode(mut cur: xmlNodePtr,
                                            mut level: *mut std::os::raw::c_int)
 -> xmlNodePtr {
    let mut current_block: u64;
    'c_11151:
        loop  {
            if cur.is_null() ||
                   (*cur).type_0 as std::os::raw::c_uint ==
                       XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
                return 0 as xmlNodePtr
            }
            if !(*cur).children.is_null() {
                cur = (*cur).children;
                if !level.is_null() { *level += 1 }
                current_block = 8677763631002396070;
            } else { current_block = 4183526789030448324; }
            loop  {
                match current_block {
                    4183526789030448324 =>
                    /* This label should only be needed if something is wrong! */
                    {
                        if !(*cur).next.is_null() {
                            cur = (*cur).next;
                            current_block = 8677763631002396070;
                        } else {
                            loop  {
                                cur = (*cur).parent;
                                if !level.is_null() { *level -= 1 }
                                if cur.is_null() { return 0 as xmlNodePtr }
                                if !(*cur).next.is_null() {
                                    cur = (*cur).next;
                                    current_block = 8677763631002396070;
                                    break ;
                                } else if cur.is_null() {
                                    current_block = 8677763631002396070;
                                    break ;
                                }
                            }
                        }
                    }
                    _ => {
                        if !((*cur).type_0 as std::os::raw::c_uint !=
                                 XML_ELEMENT_NODE as std::os::raw::c_int as
                                     std::os::raw::c_uint &&
                                 (*cur).type_0 as std::os::raw::c_uint !=
                                     XML_TEXT_NODE as std::os::raw::c_int as
                                         std::os::raw::c_uint &&
                                 (*cur).type_0 as std::os::raw::c_uint !=
                                     XML_DOCUMENT_NODE as std::os::raw::c_int as
                                         std::os::raw::c_uint &&
                                 (*cur).type_0 as std::os::raw::c_uint !=
                                     XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as
                                         std::os::raw::c_uint &&
                                 (*cur).type_0 as std::os::raw::c_uint !=
                                     XML_CDATA_SECTION_NODE as std::os::raw::c_int as
                                         std::os::raw::c_uint) {
                            break 'c_11151 ;
                        }
                        if !((*cur).type_0 as std::os::raw::c_uint ==
                                 XML_ENTITY_REF_NODE as std::os::raw::c_int as
                                     std::os::raw::c_uint) {
                            break ;
                        }
                        /* Shouldn't happen */
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   b"xpointer.c\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   2275
                                                                                       as
                                                                                       std::os::raw::c_int);
                        current_block = 4183526789030448324;
                    }
                }
            }
        }
    return cur;
}
/* *
 * xmlXPtrAdvanceChar:
 * @node:  the node
 * @indx:  the indx
 * @bytes:  the number of bytes
 *
 * Advance a point of the associated number of bytes (not UTF8 chars)
 *
 * Returns -1 in case of failure, 0 otherwise
 */
unsafe extern "C" fn xmlXPtrAdvanceChar(mut node: *mut xmlNodePtr,
                                        mut indx: *mut std::os::raw::c_int,
                                        mut bytes: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut pos: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    if node.is_null() || indx.is_null() { return -(1 as std::os::raw::c_int) }
    cur = *node;
    if cur.is_null() ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    pos = *indx;
    while bytes >= 0 as std::os::raw::c_int {
        /*
	 * First position to the beginning of the first text node
	 * corresponding to this point
	 */
        while !cur.is_null() &&
                  ((*cur).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as
                               std::os::raw::c_uint) {
            if pos > 0 as std::os::raw::c_int {
                cur = xmlXPtrGetNthChild(cur, pos);
                pos = 0 as std::os::raw::c_int
            } else {
                cur = xmlXPtrAdvanceNode(cur, 0 as *mut std::os::raw::c_int);
                pos = 0 as std::os::raw::c_int
            }
        }
        if cur.is_null() {
            *node = 0 as xmlNodePtr;
            *indx = 0 as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        /*
	 * if there is no move needed return the current value.
	 */
        if pos == 0 as std::os::raw::c_int { pos = 1 as std::os::raw::c_int }
        if bytes == 0 as std::os::raw::c_int {
            *node = cur;
            *indx = pos;
            return 0 as std::os::raw::c_int
        }
        /*
	 * We should have a text (or cdata) node ...
	 */
        len = 0 as std::os::raw::c_int;
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).content.is_null() {
            len = xmlStrlen((*cur).content)
        }
        if pos > len {
            /* Strange, the indx in the text node is greater than it's len */
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Internal error at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"xpointer.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       2349 as
                                                                           std::os::raw::c_int);
            pos = len
        }
        if pos + bytes >= len {
            bytes -= len - pos;
            cur = xmlXPtrAdvanceNode(cur, 0 as *mut std::os::raw::c_int);
            pos = 0 as std::os::raw::c_int
        } else if pos + bytes < len {
            pos += bytes;
            *node = cur;
            *indx = pos;
            return 0 as std::os::raw::c_int
        }
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlXPtrMatchString:
 * @string:  the string to search
 * @start:  the start textnode
 * @startindex:  the start index
 * @end:  the end textnode IN/OUT
 * @endindex:  the end index IN/OUT
 *
 * Check whether the document contains @string at the position
 * (@start, @startindex) and limited by the (@end, @endindex) point
 *
 * Returns -1 in case of failure, 0 if not found, 1 if found in which case
 *            (@start, @startindex) will indicate the position of the beginning
 *            of the range and (@end, @endindex) will indicate the end
 *            of the range
 */
unsafe extern "C" fn xmlXPtrMatchString(mut string: *const xmlChar,
                                        mut start: xmlNodePtr,
                                        mut startindex: std::os::raw::c_int,
                                        mut end: *mut xmlNodePtr,
                                        mut endindex: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode; /* 0 based */
    let mut pos: std::os::raw::c_int = 0; /* in bytes */
    let mut len: std::os::raw::c_int = 0; /* in bytes */
    let mut stringlen: std::os::raw::c_int = 0;
    let mut match_0: std::os::raw::c_int = 0;
    if string.is_null() { return -(1 as std::os::raw::c_int) }
    if start.is_null() ||
           (*start).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    if end.is_null() || (*end).is_null() ||
           (**end).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           endindex.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    cur = start;
    pos = startindex - 1 as std::os::raw::c_int;
    stringlen = xmlStrlen(string);
    while stringlen > 0 as std::os::raw::c_int {
        if cur == *end && pos + stringlen > *endindex {
            return 0 as std::os::raw::c_int
        }
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).content.is_null() {
            len = xmlStrlen((*cur).content);
            if len >= pos + stringlen {
                match_0 =
                    (xmlStrncmp(&mut *(*cur).content.offset(pos as isize),
                                string, stringlen) == 0) as std::os::raw::c_int;
                if match_0 != 0 {
                    *end = cur;
                    *endindex = pos + stringlen;
                    return 1 as std::os::raw::c_int
                } else { return 0 as std::os::raw::c_int }
            } else {
                let mut sub: std::os::raw::c_int = len - pos;
                match_0 =
                    (xmlStrncmp(&mut *(*cur).content.offset(pos as isize),
                                string, sub) == 0) as std::os::raw::c_int;
                if match_0 != 0 {
                    string = &*string.offset(sub as isize) as *const xmlChar;
                    stringlen -= sub
                } else { return 0 as std::os::raw::c_int }
            }
        }
        cur = xmlXPtrAdvanceNode(cur, 0 as *mut std::os::raw::c_int);
        if cur.is_null() { return 0 as std::os::raw::c_int }
        pos = 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlXPtrSearchString:
 * @string:  the string to search
 * @start:  the start textnode IN/OUT
 * @startindex:  the start index IN/OUT
 * @end:  the end textnode
 * @endindex:  the end index
 *
 * Search the next occurrence of @string within the document content
 * until the (@end, @endindex) point is reached
 *
 * Returns -1 in case of failure, 0 if not found, 1 if found in which case
 *            (@start, @startindex) will indicate the position of the beginning
 *            of the range and (@end, @endindex) will indicate the end
 *            of the range
 */
unsafe extern "C" fn xmlXPtrSearchString(mut string: *const xmlChar,
                                         mut start: *mut xmlNodePtr,
                                         mut startindex: *mut std::os::raw::c_int,
                                         mut end: *mut xmlNodePtr,
                                         mut endindex: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode; /* 0 based */
    let mut str: *const xmlChar = 0 as *const xmlChar; /* in bytes */
    let mut pos: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut first: xmlChar = 0;
    if string.is_null() { return -(1 as std::os::raw::c_int) }
    if start.is_null() || (*start).is_null() ||
           (**start).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           startindex.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    if end.is_null() || endindex.is_null() { return -(1 as std::os::raw::c_int) }
    cur = *start;
    pos = *startindex - 1 as std::os::raw::c_int;
    first = *string.offset(0 as std::os::raw::c_int as isize);
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).content.is_null() {
            len = xmlStrlen((*cur).content);
            while pos <= len {
                if first as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    str =
                        xmlStrchr(&mut *(*cur).content.offset(pos as isize),
                                  first);
                    if !str.is_null() {
                        pos =
                            str.offset_from((*cur).content) as
                                std::os::raw::c_long as std::os::raw::c_int;
                        if xmlXPtrMatchString(string, cur,
                                              pos + 1 as std::os::raw::c_int, end,
                                              endindex) != 0 {
                            *start = cur;
                            *startindex = pos + 1 as std::os::raw::c_int;
                            return 1 as std::os::raw::c_int
                        }
                        pos += 1
                    } else { pos = len + 1 as std::os::raw::c_int }
                } else {
                    /*
		     * An empty string is considered to match before each
		     * character of the string-value and after the final
		     * character.
		     */
                    *start = cur;
                    *startindex = pos + 1 as std::os::raw::c_int;
                    *end = cur;
                    *endindex = pos + 1 as std::os::raw::c_int;
                    return 1 as std::os::raw::c_int
                }
            }
        }
        if cur == *end && pos >= *endindex { return 0 as std::os::raw::c_int }
        cur = xmlXPtrAdvanceNode(cur, 0 as *mut std::os::raw::c_int);
        if cur.is_null() { return 0 as std::os::raw::c_int }
        pos = 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXPtrGetLastChar:
 * @node:  the node
 * @index:  the index
 *
 * Computes the point coordinates of the last char of this point
 *
 * Returns -1 in case of failure, 0 otherwise
 */
unsafe extern "C" fn xmlXPtrGetLastChar(mut node: *mut xmlNodePtr,
                                        mut indx: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut pos: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if node.is_null() || (*node).is_null() ||
           (**node).type_0 as std::os::raw::c_uint ==
               XML_NAMESPACE_DECL as std::os::raw::c_int as std::os::raw::c_uint ||
           indx.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    cur = *node;
    pos = *indx;
    if (*cur).type_0 as std::os::raw::c_uint ==
           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*cur).type_0 as std::os::raw::c_uint ==
               XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
        if pos > 0 as std::os::raw::c_int { cur = xmlXPtrGetNthChild(cur, pos) }
    }
    while !cur.is_null() {
        if !(*cur).last.is_null() {
            cur = (*cur).last
        } else if (*cur).type_0 as std::os::raw::c_uint !=
                      XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      !(*cur).content.is_null() {
            len = xmlStrlen((*cur).content);
            break ;
        } else { return -(1 as std::os::raw::c_int) }
    }
    if cur.is_null() { return -(1 as std::os::raw::c_int) }
    *node = cur;
    *indx = len;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlXPtrGetStartPoint:
 * @obj:  an range
 * @node:  the resulting node
 * @indx:  the resulting index
 *
 * read the object and return the start point coordinates.
 *
 * Returns -1 in case of failure, 0 otherwise
 */
unsafe extern "C" fn xmlXPtrGetStartPoint(mut obj: xmlXPathObjectPtr,
                                          mut node: *mut xmlNodePtr,
                                          mut indx: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    if obj.is_null() || node.is_null() || indx.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    match (*obj).type_0 as std::os::raw::c_uint {
        5 => {
            *node = (*obj).user as xmlNodePtr;
            if (*obj).index <= 0 as std::os::raw::c_int {
                *indx = 0 as std::os::raw::c_int
            } else { *indx = (*obj).index }
            return 0 as std::os::raw::c_int
        }
        6 => {
            *node = (*obj).user as xmlNodePtr;
            if (*obj).index <= 0 as std::os::raw::c_int {
                *indx = 0 as std::os::raw::c_int
            } else { *indx = (*obj).index }
            return 0 as std::os::raw::c_int
        }
        _ => { }
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlXPtrGetEndPoint:
 * @obj:  an range
 * @node:  the resulting node
 * @indx:  the resulting indx
 *
 * read the object and return the end point coordinates.
 *
 * Returns -1 in case of failure, 0 otherwise
 */
unsafe extern "C" fn xmlXPtrGetEndPoint(mut obj: xmlXPathObjectPtr,
                                        mut node: *mut xmlNodePtr,
                                        mut indx: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    if obj.is_null() || node.is_null() || indx.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    match (*obj).type_0 as std::os::raw::c_uint {
        5 => {
            *node = (*obj).user as xmlNodePtr;
            if (*obj).index <= 0 as std::os::raw::c_int {
                *indx = 0 as std::os::raw::c_int
            } else { *indx = (*obj).index }
            return 0 as std::os::raw::c_int
        }
        6 => {
            *node = (*obj).user as xmlNodePtr;
            if (*obj).index <= 0 as std::os::raw::c_int {
                *indx = 0 as std::os::raw::c_int
            } else { *indx = (*obj).index }
            return 0 as std::os::raw::c_int
        }
        _ => { }
    }
    return -(1 as std::os::raw::c_int);
}
/* ***********************************************************************
 *									*
 *			General routines				*
 *									*
 ************************************************************************/
/* *
 * xmlXPtrStringRangeFunction:
 * @ctxt:  the XPointer Parser context
 * @nargs:  the number of args
 *
 * Function implementing the string-range() function
 * range as described in 5.4.2
 *
 * ------------------------------
 * [Definition: For each location in the location-set argument,
 * string-range returns a set of string ranges, a set of substrings in a
 * string. Specifically, the string-value of the location is searched for
 * substrings that match the string argument, and the resulting location-set
 * will contain a range location for each non-overlapping match.]
 * An empty string is considered to match before each character of the
 * string-value and after the final character. Whitespace in a string
 * is matched literally, with no normalization except that provided by
 * XML for line ends. The third argument gives the position of the first
 * character to be in the resulting range, relative to the start of the
 * match. The default value is 1, which makes the range start immediately
 * before the first character of the matched string. The fourth argument
 * gives the number of characters in the range; the default is that the
 * range extends to the end of the matched string.
 *
 * Element boundaries, as well as entire embedded nodes such as processing
 * instructions and comments, are ignored as defined in [XPath].
 *
 * If the string in the second argument is not found in the string-value
 * of the location, or if a value in the third or fourth argument indicates
 * a string that is beyond the beginning or end of the document, the
 * expression fails.
 *
 * The points of the range-locations in the returned location-set will
 * all be character points.
 * ------------------------------
 */
unsafe extern "C" fn xmlXPtrStringRangeFunction(mut ctxt:
                                                    xmlXPathParserContextPtr,
                                                mut nargs: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut startindex: std::os::raw::c_int = 0;
    let mut endindex: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut fendindex: std::os::raw::c_int = 0;
    let mut start: xmlNodePtr = 0 as *mut xmlNode;
    let mut end: xmlNodePtr = 0 as xmlNodePtr;
    let mut fend: xmlNodePtr = 0 as *mut xmlNode;
    let mut set: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut oldset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    let mut newset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    let mut string: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut position: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut number: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut found: std::os::raw::c_int = 0;
    let mut pos: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut num: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /*
     * Grab the arguments
     */
    if nargs < 2 as std::os::raw::c_int || nargs > 4 as std::os::raw::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as std::os::raw::c_int);
        return
    }
    if nargs >= 4 as std::os::raw::c_int {
        if (*ctxt).value.is_null() ||
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NUMBER as std::os::raw::c_int as std::os::raw::c_uint {
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
            return
        }
        number = valuePop(ctxt);
        if !number.is_null() { num = (*number).floatval as std::os::raw::c_int }
    }
    if nargs >= 3 as std::os::raw::c_int {
        if (*ctxt).value.is_null() ||
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NUMBER as std::os::raw::c_int as std::os::raw::c_uint {
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
            return
        }
        position = valuePop(ctxt);
        if !position.is_null() { pos = (*position).floatval as std::os::raw::c_int }
    }
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_STRING as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    string = valuePop(ctxt);
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
                   XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    set = valuePop(ctxt);
    newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
    if newset.is_null() {
        xmlXPathFreeObject(set);
        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
        return
    }
    if !(*set).nodesetval.is_null() {
        if (*set).type_0 as std::os::raw::c_uint ==
               XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint {
            let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            /*
	 * First convert to a location set
	 */
            tmp = xmlXPtrNewLocationSetNodeSet((*set).nodesetval);
            xmlXPathFreeObject(set);
            if tmp.is_null() {
                xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as std::os::raw::c_int);
                return
            }
            set = tmp
        }
        oldset = (*set).user as xmlLocationSetPtr;
        /*
     * The loop is to search for each element in the location set
     * the list of location set corresponding to that search
     */
        i = 0 as std::os::raw::c_int;
        while i < (*oldset).locNr {
            xmlXPtrGetStartPoint(*(*oldset).locTab.offset(i as isize),
                                 &mut start, &mut startindex);
            xmlXPtrGetEndPoint(*(*oldset).locTab.offset(i as isize), &mut end,
                               &mut endindex);
            xmlXPtrAdvanceChar(&mut start, &mut startindex, 0 as std::os::raw::c_int);
            xmlXPtrGetLastChar(&mut end, &mut endindex);
            loop  {
                fend = end;
                fendindex = endindex;
                found =
                    xmlXPtrSearchString((*string).stringval, &mut start,
                                        &mut startindex, &mut fend,
                                        &mut fendindex);
                if found == 1 as std::os::raw::c_int {
                    if position.is_null() {
                        xmlXPtrLocationSetAdd(newset,
                                              xmlXPtrNewRange(start,
                                                              startindex,
                                                              fend,
                                                              fendindex));
                    } else if xmlXPtrAdvanceChar(&mut start, &mut startindex,
                                                 pos - 1 as std::os::raw::c_int) ==
                                  0 as std::os::raw::c_int {
                        if !number.is_null() && num > 0 as std::os::raw::c_int {
                            let mut rindx: std::os::raw::c_int = 0;
                            let mut rend: xmlNodePtr = 0 as *mut xmlNode;
                            rend = start;
                            rindx = startindex - 1 as std::os::raw::c_int;
                            if xmlXPtrAdvanceChar(&mut rend, &mut rindx, num)
                                   == 0 as std::os::raw::c_int {
                                xmlXPtrLocationSetAdd(newset,
                                                      xmlXPtrNewRange(start,
                                                                      startindex,
                                                                      rend,
                                                                      rindx));
                            }
                        } else if !number.is_null() && num <= 0 as std::os::raw::c_int
                         {
                            xmlXPtrLocationSetAdd(newset,
                                                  xmlXPtrNewRange(start,
                                                                  startindex,
                                                                  start,
                                                                  startindex));
                        } else {
                            xmlXPtrLocationSetAdd(newset,
                                                  xmlXPtrNewRange(start,
                                                                  startindex,
                                                                  fend,
                                                                  fendindex));
                        }
                    }
                    start = fend;
                    startindex = fendindex;
                    if *(*string).stringval.offset(0 as std::os::raw::c_int as isize)
                           as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        startindex += 1
                    }
                }
                if !(found == 1 as std::os::raw::c_int) { break ; }
            }
            i += 1
        }
    }
    /*
     * Save the new value and cleanup
     */
    valuePush(ctxt, xmlXPtrWrapLocationSet(newset));
    xmlXPathFreeObject(set);
    xmlXPathFreeObject(string);
    if !position.is_null() { xmlXPathFreeObject(position); }
    if !number.is_null() { xmlXPathFreeObject(number); };
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
/* number of locations in the set */
/* size of the array as allocated */
/* array of locations */
/*
 * Handling of location sets.
 */
/*
 * Functions.
 */
/* *
 * xmlXPtrEvalRangePredicate:
 * @ctxt:  the XPointer Parser context
 *
 *  [8]   Predicate ::=   '[' PredicateExpr ']'
 *  [9]   PredicateExpr ::=   Expr
 *
 * Evaluate a predicate as in xmlXPathEvalPredicate() but for
 * a Location Set instead of a node set
 */
#[no_mangle]
pub unsafe extern "C" fn xmlXPtrEvalRangePredicate(mut ctxt:
                                                       xmlXPathParserContextPtr) {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut newset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
    let mut oldset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
    let mut i: std::os::raw::c_int = 0;
    if ctxt.is_null() { return }
    while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                  *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
    }
    if *(*ctxt).cur as std::os::raw::c_int != '[' as i32 {
        xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as std::os::raw::c_int);
        return
    }
    if *(*ctxt).cur as std::os::raw::c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1)
    } else { };
    while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                  *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
    }
    /*
     * Extract the old set, and then evaluate the result of the
     * expression for all the element in the set. use it to grow
     * up a new set.
     */
    if (*ctxt).value.is_null() ||
           (*(*ctxt).value).type_0 as std::os::raw::c_uint !=
               XPATH_LOCATIONSET as std::os::raw::c_int as std::os::raw::c_uint {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as std::os::raw::c_int);
        return
    }
    obj = valuePop(ctxt);
    oldset = (*obj).user as xmlLocationSetPtr;
    (*(*ctxt).context).node = 0 as xmlNodePtr;
    if oldset.is_null() || (*oldset).locNr == 0 as std::os::raw::c_int {
        (*(*ctxt).context).contextSize = 0 as std::os::raw::c_int;
        (*(*ctxt).context).proximityPosition = 0 as std::os::raw::c_int;
        xmlXPathEvalExpr(ctxt);
        res = valuePop(ctxt);
        if !res.is_null() { xmlXPathFreeObject(res); }
        valuePush(ctxt, obj);
        if (*ctxt).error != XPATH_EXPRESSION_OK as std::os::raw::c_int { return }
    } else {
        /*
	 * Save the expression pointer since we will have to evaluate
	 * it multiple times. Initialize the new set.
	 */
        cur = (*ctxt).cur;
        newset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
        i = 0 as std::os::raw::c_int;
        while i < (*oldset).locNr {
            (*ctxt).cur = cur;
            /*
	     * Run the evaluation with a node list made of a single item
	     * in the nodeset.
	     */
            (*(*ctxt).context).node =
                (**(*oldset).locTab.offset(i as isize)).user as xmlNodePtr;
            tmp = xmlXPathNewNodeSet((*(*ctxt).context).node);
            valuePush(ctxt, tmp);
            (*(*ctxt).context).contextSize = (*oldset).locNr;
            (*(*ctxt).context).proximityPosition = i + 1 as std::os::raw::c_int;
            xmlXPathEvalExpr(ctxt);
            if (*ctxt).error != XPATH_EXPRESSION_OK as std::os::raw::c_int { return }
            /*
	     * The result of the evaluation need to be tested to
	     * decided whether the filter succeeded or not
	     */
            res = valuePop(ctxt);
            if xmlXPathEvaluatePredicateResult(ctxt, res) != 0 {
                xmlXPtrLocationSetAdd(newset,
                                      xmlXPathObjectCopy(*(*oldset).locTab.offset(i
                                                                                      as
                                                                                      isize)));
            }
            /*
	     * Cleanup
	     */
            if !res.is_null() { xmlXPathFreeObject(res); }
            if (*ctxt).value == tmp {
                res = valuePop(ctxt);
                xmlXPathFreeObject(res);
            }
            (*(*ctxt).context).node = 0 as xmlNodePtr;
            i += 1
        }
        /*
	 * The result is used as the new evaluation set.
	 */
        xmlXPathFreeObject(obj);
        (*(*ctxt).context).node = 0 as xmlNodePtr;
        (*(*ctxt).context).contextSize = -(1 as std::os::raw::c_int);
        (*(*ctxt).context).proximityPosition = -(1 as std::os::raw::c_int);
        valuePush(ctxt, xmlXPtrWrapLocationSet(newset));
    }
    if *(*ctxt).cur as std::os::raw::c_int != ']' as i32 {
        xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as std::os::raw::c_int);
        return
    }
    if *(*ctxt).cur as std::os::raw::c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1)
    } else { };
    while *(*ctxt).cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *(*ctxt).cur as std::os::raw::c_int &&
                  *(*ctxt).cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *(*ctxt).cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        if *(*ctxt).cur as std::os::raw::c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1)
        } else { };
    };
}
/* __INCLUDE_ELFGCCHACK */
