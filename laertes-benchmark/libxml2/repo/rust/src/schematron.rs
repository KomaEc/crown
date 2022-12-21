
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
    pub type _xmlXPathCompExpr;
    pub type _xmlPattern;
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
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    /*
 * Constructor and destructor.
 */
    #[no_mangle]
    fn xmlDictCreate() -> xmlDictPtr;
    #[no_mangle]
    fn xmlDictReference(dict: xmlDictPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlDictFree(dict: xmlDictPtr);
    /*
 * Lookup of entry in the dictionary.
 */
    #[no_mangle]
    fn xmlDictLookup(dict: xmlDictPtr, name: *const xmlChar, len: std::os::raw::c_int)
     -> *const xmlChar;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* LIBXML_TREE_ENABLED */
    /*
 * Navigating.
 */
    #[no_mangle]
    fn xmlGetLineNo(node: *const xmlNode) -> std::os::raw::c_long;
    #[no_mangle]
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_XINCLUDE_ENABLED) || \
	  defined(LIBXML_SCHEMAS_ENABLED) || defined(LIBXML_HTML_ENABLED) */
    #[no_mangle]
    fn xmlGetNoNsProp(node: *const xmlNode, name: *const xmlChar)
     -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
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
    fn __xmlSimpleError(domain: std::os::raw::c_int, code: std::os::raw::c_int,
                        node: xmlNodePtr, msg: *const std::os::raw::c_char,
                        extra: *const std::os::raw::c_char);
    #[no_mangle]
    fn xmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                     URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                     options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
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
    fn xmlXPathEval(str: *const xmlChar, ctx: xmlXPathContextPtr)
     -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathCtxtCompile(ctxt: xmlXPathContextPtr, str: *const xmlChar)
     -> xmlXPathCompExprPtr;
    #[no_mangle]
    fn xmlXPathCompiledEval(comp: xmlXPathCompExprPtr,
                            ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    #[no_mangle]
    fn xmlXPathFreeCompExpr(comp: xmlXPathCompExprPtr);
    #[no_mangle]
    fn xmlXPathIsNaN(val: std::os::raw::c_double) -> std::os::raw::c_int;
    /* *
 * Extending a context.
 */
    #[no_mangle]
    fn xmlXPathRegisterNs(ctxt: xmlXPathContextPtr, prefix: *const xmlChar,
                          ns_uri: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreePattern(comp: xmlPatternPtr);
    #[no_mangle]
    fn xmlPatterncompile(pattern: *const xmlChar, dict: *mut xmlDict,
                         flags: std::os::raw::c_int, namespaces: *mut *const xmlChar)
     -> xmlPatternPtr;
    #[no_mangle]
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> std::os::raw::c_int;
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
pub type xmlOutputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlOutputWriteCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: std::os::raw::c_int) -> std::os::raw::c_int>;
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
pub type C2RustUnnamed_1 = std::os::raw::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_1 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_1 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_1 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_1 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_1 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_1 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_1 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_1 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_1 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_1 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_1 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_1 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_1 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_1 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_1 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_1 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_1 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_1 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_1 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_1 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_1 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_1 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_1 = 1;
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
pub type xmlPattern = _xmlPattern;
pub type xmlPatternPtr = *mut xmlPattern;
pub type C2RustUnnamed_2 = std::os::raw::c_uint;
pub const XML_PATTERN_XSFIELD: C2RustUnnamed_2 = 4;
pub const XML_PATTERN_XSSEL: C2RustUnnamed_2 = 2;
pub const XML_PATTERN_XPATH: C2RustUnnamed_2 = 1;
pub const XML_PATTERN_DEFAULT: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = std::os::raw::c_uint;
pub const XML_SCHEMATRON_OUT_IO: C2RustUnnamed_3 = 1024;
pub const XML_SCHEMATRON_OUT_BUFFER: C2RustUnnamed_3 = 512;
pub const XML_SCHEMATRON_OUT_FILE: C2RustUnnamed_3 = 256;
pub const XML_SCHEMATRON_OUT_ERROR: C2RustUnnamed_3 = 8;
pub const XML_SCHEMATRON_OUT_XML: C2RustUnnamed_3 = 4;
pub const XML_SCHEMATRON_OUT_TEXT: C2RustUnnamed_3 = 2;
pub const XML_SCHEMATRON_OUT_QUIET: C2RustUnnamed_3 = 1;
/* the name of the pattern */
/* *
 * _xmlSchematron:
 *
 * A Schematrons definition
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchematron {
    pub name: *const xmlChar,
    pub preserve: std::os::raw::c_int,
    pub doc: xmlDocPtr,
    pub flags: std::os::raw::c_int,
    pub _private: *mut std::os::raw::c_void,
    pub dict: xmlDictPtr,
    pub title: *const xmlChar,
    pub nbNs: std::os::raw::c_int,
    pub nbPattern: std::os::raw::c_int,
    pub patterns: xmlSchematronPatternPtr,
    pub rules: xmlSchematronRulePtr,
    pub nbNamespaces: std::os::raw::c_int,
    pub maxNamespaces: std::os::raw::c_int,
    pub namespaces: *mut *const xmlChar,
}
pub type xmlSchematronRulePtr = *mut xmlSchematronRule;
/* the message to report */
/* *
 * _xmlSchematronRule:
 *
 * A Schematrons rule
 */
pub type xmlSchematronRule = _xmlSchematronRule;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchematronRule {
    pub next: xmlSchematronRulePtr,
    pub patnext: xmlSchematronRulePtr,
    pub node: xmlNodePtr,
    pub context: *mut xmlChar,
    pub tests: xmlSchematronTestPtr,
    pub pattern: xmlPatternPtr,
    pub report: *mut xmlChar,
}
pub type xmlSchematronTestPtr = *mut xmlSchematronTest;
/* *
 * _xmlSchematronTest:
 *
 * A Schematrons test, either an assert or a report
 */
pub type xmlSchematronTest = _xmlSchematronTest;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchematronTest {
    pub next: xmlSchematronTestPtr,
    pub type_0: xmlSchematronTestType,
    pub node: xmlNodePtr,
    pub test: *mut xmlChar,
    pub comp: xmlXPathCompExprPtr,
    pub report: *mut xmlChar,
}
pub type xmlSchematronTestType = std::os::raw::c_uint;
pub const XML_SCHEMATRON_REPORT: xmlSchematronTestType = 2;
pub const XML_SCHEMATRON_ASSERT: xmlSchematronTestType = 1;
pub type xmlSchematronPatternPtr = *mut xmlSchematronPattern;
/* the message to report */
/* *
 * _xmlSchematronPattern:
 *
 * A Schematrons pattern
 */
pub type xmlSchematronPattern = _xmlSchematronPattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchematronPattern {
    pub next: xmlSchematronPatternPtr,
    pub rules: xmlSchematronRulePtr,
    pub name: *mut xmlChar,
}
/* *
 * The schemas related types are kept internal
 */
pub type xmlSchematron = _xmlSchematron;
pub type xmlSchematronPtr = *mut xmlSchematron;
/* *
 * xmlSchematronValidityErrorFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of an error callback from a Schematron validation
 */
pub type xmlSchematronValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * xmlSchematronValidityWarningFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of a warning callback from a Schematron validation
 */
pub type xmlSchematronValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* the structured function */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchematronParserCtxt {
    pub type_0: std::os::raw::c_int,
    pub URL: *const xmlChar,
    pub doc: xmlDocPtr,
    pub preserve: std::os::raw::c_int,
    pub buffer: *const std::os::raw::c_char,
    pub size: std::os::raw::c_int,
    pub dict: xmlDictPtr,
    pub nberrors: std::os::raw::c_int,
    pub err: std::os::raw::c_int,
    pub xctxt: xmlXPathContextPtr,
    pub schema: xmlSchematronPtr,
    pub nbNamespaces: std::os::raw::c_int,
    pub maxNamespaces: std::os::raw::c_int,
    pub namespaces: *mut *const xmlChar,
    pub nbIncludes: std::os::raw::c_int,
    pub maxIncludes: std::os::raw::c_int,
    pub includes: *mut xmlNodePtr,
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlSchematronValidityErrorFunc,
    pub warning: xmlSchematronValidityWarningFunc,
    pub serror: xmlStructuredErrorFunc,
}
/* *
 * A schemas validation context
 */
pub type xmlSchematronParserCtxt = _xmlSchematronParserCtxt;
pub type xmlSchematronParserCtxtPtr = *mut xmlSchematronParserCtxt;
/* the array of namespaces */
/* *
 * xmlSchematronValidCtxt:
 *
 * A Schematrons validation context
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchematronValidCtxt {
    pub type_0: std::os::raw::c_int,
    pub flags: std::os::raw::c_int,
    pub dict: xmlDictPtr,
    pub nberrors: std::os::raw::c_int,
    pub err: std::os::raw::c_int,
    pub schema: xmlSchematronPtr,
    pub xctxt: xmlXPathContextPtr,
    pub outputFile: *mut FILE,
    pub outputBuffer: xmlBufferPtr,
    pub iowrite: xmlOutputWriteCallback,
    pub ioclose: xmlOutputCloseCallback,
    pub ioctx: *mut std::os::raw::c_void,
    pub userData: *mut std::os::raw::c_void,
    pub error: xmlSchematronValidityErrorFunc,
    pub warning: xmlSchematronValidityWarningFunc,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlSchematronValidCtxt = _xmlSchematronValidCtxt;
pub type xmlSchematronValidCtxtPtr = *mut xmlSchematronValidCtxt;
static mut xmlSchematronNs: *const xmlChar =
    b"http://purl.oclc.org/dsdl/schematron\x00" as *const u8 as
        *const std::os::raw::c_char as *mut xmlChar;
static mut xmlOldSchematronNs: *const xmlChar =
    b"http://www.ascc.net/xml/schematron\x00" as *const u8 as
        *const std::os::raw::c_char as *mut xmlChar;
/* ***********************************************************************
 *									*
 *			Error reporting					*
 *									*
 ************************************************************************/
/* *
 * xmlSchematronPErrMemory:
 * @node: a context node
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlSchematronPErrMemory(mut ctxt:
                                                 xmlSchematronParserCtxtPtr,
                                             mut extra: *const std::os::raw::c_char,
                                             mut node: xmlNodePtr) {
    if !ctxt.is_null() { (*ctxt).nberrors += 1 }
    __xmlSimpleError(XML_FROM_SCHEMASP as std::os::raw::c_int,
                     XML_ERR_NO_MEMORY as std::os::raw::c_int, node,
                     0 as *const std::os::raw::c_char, extra);
}
/* *
 * xmlSchematronPErr:
 * @ctxt: the parsing context
 * @node: the context node
 * @error: the error code
 * @msg: the error message
 * @str1: extra data
 * @str2: extra data
 *
 * Handle a parser error
 */
unsafe extern "C" fn xmlSchematronPErr(mut ctxt: xmlSchematronParserCtxtPtr,
                                       mut node: xmlNodePtr,
                                       mut error: std::os::raw::c_int,
                                       mut msg: *const std::os::raw::c_char,
                                       mut str1: *const xmlChar,
                                       mut str2: *const xmlChar) {
    let mut channel: xmlGenericErrorFunc = None;
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if !ctxt.is_null() {
        (*ctxt).nberrors += 1;
        channel = (*ctxt).error;
        data = (*ctxt).userData;
        schannel = (*ctxt).serror
    }
    __xmlRaiseError(schannel, channel, data, ctxt as *mut std::os::raw::c_void,
                    node as *mut std::os::raw::c_void,
                    XML_FROM_SCHEMASP as std::os::raw::c_int, error, XML_ERR_ERROR,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    str1 as *const std::os::raw::c_char, str2 as *const std::os::raw::c_char,
                    0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                    0 as std::os::raw::c_int, msg, str1, str2);
}
/* *
 * xmlSchematronVTypeErrMemory:
 * @node: a context node
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlSchematronVErrMemory(mut ctxt:
                                                 xmlSchematronValidCtxtPtr,
                                             mut extra: *const std::os::raw::c_char,
                                             mut node: xmlNodePtr) {
    if !ctxt.is_null() {
        (*ctxt).nberrors += 1;
        (*ctxt).err = XML_SCHEMAV_INTERNAL as std::os::raw::c_int
    }
    __xmlSimpleError(XML_FROM_SCHEMASV as std::os::raw::c_int,
                     XML_ERR_NO_MEMORY as std::os::raw::c_int, node,
                     0 as *const std::os::raw::c_char, extra);
}
/* ***********************************************************************
 *									*
 *		Parsing and compilation of the Schematrontrons		*
 *									*
 ************************************************************************/
/* *
 * xmlSchematronAddTest:
 * @ctxt: the schema parsing context
 * @type:  the type of test
 * @rule:  the parent rule
 * @node:  the node hosting the test
 * @test: the associated test
 * @report: the associated report string
 *
 * Add a test to a schematron
 *
 * Returns the new pointer or NULL in case of error
 */
unsafe extern "C" fn xmlSchematronAddTest(mut ctxt:
                                              xmlSchematronParserCtxtPtr,
                                          mut type_0: xmlSchematronTestType,
                                          mut rule: xmlSchematronRulePtr,
                                          mut node: xmlNodePtr,
                                          mut test: *mut xmlChar,
                                          mut report: *mut xmlChar)
 -> xmlSchematronTestPtr {
    let mut ret: xmlSchematronTestPtr = 0 as *mut xmlSchematronTest;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    if ctxt.is_null() || rule.is_null() || node.is_null() || test.is_null() {
        return 0 as xmlSchematronTestPtr
    }
    /*
     * try first to compile the test expression
     */
    comp = xmlXPathCtxtCompile((*ctxt).xctxt, test);
    if comp.is_null() {
        xmlSchematronPErr(ctxt, node, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"Failed to compile test expression %s\x00" as
                              *const u8 as *const std::os::raw::c_char, test,
                          0 as *const xmlChar);
        return 0 as xmlSchematronTestPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronTest>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronTestPtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(ctxt,
                                b"allocating schema test\x00" as *const u8 as
                                    *const std::os::raw::c_char, node);
        return 0 as xmlSchematronTestPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronTest>() as std::os::raw::c_ulong);
    (*ret).type_0 = type_0;
    (*ret).node = node;
    (*ret).test = test;
    (*ret).comp = comp;
    (*ret).report = report;
    (*ret).next = 0 as xmlSchematronTestPtr;
    if (*rule).tests.is_null() {
        (*rule).tests = ret
    } else {
        let mut prev: xmlSchematronTestPtr = (*rule).tests;
        while !(*prev).next.is_null() { prev = (*prev).next }
        (*prev).next = ret
    }
    return ret;
}
/* *
 * xmlSchematronFreeTests:
 * @tests:  a list of tests
 *
 * Free a list of tests.
 */
unsafe extern "C" fn xmlSchematronFreeTests(mut tests: xmlSchematronTestPtr) {
    let mut next: xmlSchematronTestPtr = 0 as *mut xmlSchematronTest;
    while !tests.is_null() {
        next = (*tests).next;
        if !(*tests).test.is_null() {
            xmlFree.expect("non-null function pointer")((*tests).test as
                                                            *mut std::os::raw::c_void);
        }
        if !(*tests).comp.is_null() { xmlXPathFreeCompExpr((*tests).comp); }
        if !(*tests).report.is_null() {
            xmlFree.expect("non-null function pointer")((*tests).report as
                                                            *mut std::os::raw::c_void);
        }
        xmlFree.expect("non-null function pointer")(tests as
                                                        *mut std::os::raw::c_void);
        tests = next
    };
}
/* *
 * xmlSchematronAddRule:
 * @ctxt: the schema parsing context
 * @schema:  a schema structure
 * @node:  the node hosting the rule
 * @context: the associated context string
 * @report: the associated report string
 *
 * Add a rule to a schematron
 *
 * Returns the new pointer or NULL in case of error
 */
unsafe extern "C" fn xmlSchematronAddRule(mut ctxt:
                                              xmlSchematronParserCtxtPtr,
                                          mut schema: xmlSchematronPtr,
                                          mut pat: xmlSchematronPatternPtr,
                                          mut node: xmlNodePtr,
                                          mut context: *mut xmlChar,
                                          mut report: *mut xmlChar)
 -> xmlSchematronRulePtr {
    let mut ret: xmlSchematronRulePtr = 0 as *mut xmlSchematronRule;
    let mut pattern: xmlPatternPtr = 0 as *mut xmlPattern;
    if ctxt.is_null() || schema.is_null() || node.is_null() ||
           context.is_null() {
        return 0 as xmlSchematronRulePtr
    }
    /*
     * Try first to compile the pattern
     */
    pattern =
        xmlPatterncompile(context, (*ctxt).dict,
                          XML_PATTERN_XPATH as std::os::raw::c_int,
                          (*ctxt).namespaces);
    if pattern.is_null() {
        xmlSchematronPErr(ctxt, node, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"Failed to compile context expression %s\x00" as
                              *const u8 as *const std::os::raw::c_char, context,
                          0 as *const xmlChar);
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronRule>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronRulePtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(ctxt,
                                b"allocating schema rule\x00" as *const u8 as
                                    *const std::os::raw::c_char, node);
        return 0 as xmlSchematronRulePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronRule>() as std::os::raw::c_ulong);
    (*ret).node = node;
    (*ret).context = context;
    (*ret).pattern = pattern;
    (*ret).report = report;
    (*ret).next = 0 as xmlSchematronRulePtr;
    if (*schema).rules.is_null() {
        (*schema).rules = ret
    } else {
        let mut prev: xmlSchematronRulePtr = (*schema).rules;
        while !(*prev).next.is_null() { prev = (*prev).next }
        (*prev).next = ret
    }
    (*ret).patnext = 0 as xmlSchematronRulePtr;
    if (*pat).rules.is_null() {
        (*pat).rules = ret
    } else {
        let mut prev_0: xmlSchematronRulePtr = (*pat).rules;
        while !(*prev_0).patnext.is_null() { prev_0 = (*prev_0).patnext }
        (*prev_0).patnext = ret
    }
    return ret;
}
/* *
 * xmlSchematronFreeRules:
 * @rules:  a list of rules
 *
 * Free a list of rules.
 */
unsafe extern "C" fn xmlSchematronFreeRules(mut rules: xmlSchematronRulePtr) {
    let mut next: xmlSchematronRulePtr = 0 as *mut xmlSchematronRule;
    while !rules.is_null() {
        next = (*rules).next;
        if !(*rules).tests.is_null() {
            xmlSchematronFreeTests((*rules).tests);
        }
        if !(*rules).context.is_null() {
            xmlFree.expect("non-null function pointer")((*rules).context as
                                                            *mut std::os::raw::c_void);
        }
        if !(*rules).pattern.is_null() { xmlFreePattern((*rules).pattern); }
        if !(*rules).report.is_null() {
            xmlFree.expect("non-null function pointer")((*rules).report as
                                                            *mut std::os::raw::c_void);
        }
        xmlFree.expect("non-null function pointer")(rules as
                                                        *mut std::os::raw::c_void);
        rules = next
    };
}
/* *
 * xmlSchematronAddPattern:
 * @ctxt: the schema parsing context
 * @schema:  a schema structure
 * @node:  the node hosting the pattern
 * @id: the id or name of the pattern
 *
 * Add a pattern to a schematron
 *
 * Returns the new pointer or NULL in case of error
 */
unsafe extern "C" fn xmlSchematronAddPattern(mut ctxt:
                                                 xmlSchematronParserCtxtPtr,
                                             mut schema: xmlSchematronPtr,
                                             mut node: xmlNodePtr,
                                             mut name: *mut xmlChar)
 -> xmlSchematronPatternPtr {
    let mut ret: xmlSchematronPatternPtr = 0 as *mut xmlSchematronPattern;
    if ctxt.is_null() || schema.is_null() || node.is_null() || name.is_null()
       {
        return 0 as xmlSchematronPatternPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronPattern>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronPatternPtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(ctxt,
                                b"allocating schema pattern\x00" as *const u8
                                    as *const std::os::raw::c_char, node);
        return 0 as xmlSchematronPatternPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronPattern>() as std::os::raw::c_ulong);
    (*ret).name = name;
    (*ret).next = 0 as xmlSchematronPatternPtr;
    if (*schema).patterns.is_null() {
        (*schema).patterns = ret
    } else {
        let mut prev: xmlSchematronPatternPtr = (*schema).patterns;
        while !(*prev).next.is_null() { prev = (*prev).next }
        (*prev).next = ret
    }
    return ret;
}
/* *
 * xmlSchematronFreePatterns:
 * @patterns:  a list of patterns
 *
 * Free a list of patterns.
 */
unsafe extern "C" fn xmlSchematronFreePatterns(mut patterns:
                                                   xmlSchematronPatternPtr) {
    let mut next: xmlSchematronPatternPtr = 0 as *mut xmlSchematronPattern;
    while !patterns.is_null() {
        next = (*patterns).next;
        if !(*patterns).name.is_null() {
            xmlFree.expect("non-null function pointer")((*patterns).name as
                                                            *mut std::os::raw::c_void);
        }
        xmlFree.expect("non-null function pointer")(patterns as
                                                        *mut std::os::raw::c_void);
        patterns = next
    };
}
/* *
 * xmlSchematronNewSchematron:
 * @ctxt:  a schema validation context
 *
 * Allocate a new Schematron structure.
 *
 * Returns the newly allocated structure or NULL in case or error
 */
unsafe extern "C" fn xmlSchematronNewSchematron(mut ctxt:
                                                    xmlSchematronParserCtxtPtr)
 -> xmlSchematronPtr {
    let mut ret: xmlSchematronPtr = 0 as *mut xmlSchematron;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematron>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronPtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(ctxt,
                                b"allocating schema\x00" as *const u8 as
                                    *const std::os::raw::c_char, 0 as xmlNodePtr);
        return 0 as xmlSchematronPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematron>() as std::os::raw::c_ulong);
    (*ret).dict = (*ctxt).dict;
    xmlDictReference((*ret).dict);
    return ret;
}
/* *
 * xmlSchematronFree:
 * @schema:  a schema structure
 *
 * Deallocate a Schematron structure.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronFree(mut schema: xmlSchematronPtr) {
    if schema.is_null() { return }
    if !(*schema).doc.is_null() && (*schema).preserve == 0 {
        xmlFreeDoc((*schema).doc);
    }
    if !(*schema).namespaces.is_null() {
        xmlFree.expect("non-null function pointer")((*schema).namespaces as
                                                        *mut *mut std::os::raw::c_char
                                                        as *mut std::os::raw::c_void);
    }
    xmlSchematronFreeRules((*schema).rules);
    xmlSchematronFreePatterns((*schema).patterns);
    xmlDictFree((*schema).dict);
    xmlFree.expect("non-null function pointer")(schema as *mut std::os::raw::c_void);
}
/*
 * Interfaces for parsing.
 */
/* *
 * xmlSchematronNewParserCtxt:
 * @URL:  the location of the schema
 *
 * Create an XML Schematrons parse context for that file/resource expected
 * to contain an XML Schematrons file.
 *
 * Returns the parser context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronNewParserCtxt(mut URL:
                                                        *const std::os::raw::c_char)
 -> xmlSchematronParserCtxtPtr {
    let mut ret: xmlSchematronParserCtxtPtr =
        0 as *mut xmlSchematronParserCtxt;
    if URL.is_null() { return 0 as xmlSchematronParserCtxtPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronParserCtxtPtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser context\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        return 0 as xmlSchematronParserCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronParserCtxt>() as std::os::raw::c_ulong);
    (*ret).type_0 = 1 as std::os::raw::c_int;
    (*ret).dict = xmlDictCreate();
    (*ret).URL =
        xmlDictLookup((*ret).dict, URL as *const xmlChar,
                      -(1 as std::os::raw::c_int));
    (*ret).includes = 0 as *mut xmlNodePtr;
    (*ret).xctxt = xmlXPathNewContext(0 as xmlDocPtr);
    if (*ret).xctxt.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser XPath context\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        xmlSchematronFreeParserCtxt(ret);
        return 0 as xmlSchematronParserCtxtPtr
    }
    (*(*ret).xctxt).flags = (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int;
    return ret;
}
/* *
 * xmlSchematronNewMemParserCtxt:
 * @buffer:  a pointer to a char array containing the schemas
 * @size:  the size of the array
 *
 * Create an XML Schematrons parse context for that memory buffer expected
 * to contain an XML Schematrons file.
 *
 * Returns the parser context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronNewMemParserCtxt(mut buffer:
                                                           *const std::os::raw::c_char,
                                                       mut size: std::os::raw::c_int)
 -> xmlSchematronParserCtxtPtr {
    let mut ret: xmlSchematronParserCtxtPtr =
        0 as *mut xmlSchematronParserCtxt;
    if buffer.is_null() || size <= 0 as std::os::raw::c_int {
        return 0 as xmlSchematronParserCtxtPtr
    }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronParserCtxtPtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser context\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        return 0 as xmlSchematronParserCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronParserCtxt>() as std::os::raw::c_ulong);
    (*ret).buffer = buffer;
    (*ret).size = size;
    (*ret).dict = xmlDictCreate();
    (*ret).xctxt = xmlXPathNewContext(0 as xmlDocPtr);
    if (*ret).xctxt.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser XPath context\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        xmlSchematronFreeParserCtxt(ret);
        return 0 as xmlSchematronParserCtxtPtr
    }
    return ret;
}
/* *
 * xmlSchematronNewDocParserCtxt:
 * @doc:  a preparsed document tree
 *
 * Create an XML Schematrons parse context for that document.
 * NB. The document may be modified during the parsing process.
 *
 * Returns the parser context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronNewDocParserCtxt(mut doc: xmlDocPtr)
 -> xmlSchematronParserCtxtPtr {
    let mut ret: xmlSchematronParserCtxtPtr =
        0 as *mut xmlSchematronParserCtxt;
    if doc.is_null() { return 0 as xmlSchematronParserCtxtPtr }
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronParserCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronParserCtxtPtr;
    if ret.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser context\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        return 0 as xmlSchematronParserCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronParserCtxt>() as std::os::raw::c_ulong);
    (*ret).doc = doc;
    (*ret).dict = xmlDictCreate();
    /* The application has responsibility for the document */
    (*ret).preserve = 1 as std::os::raw::c_int;
    (*ret).xctxt = xmlXPathNewContext(doc);
    if (*ret).xctxt.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser XPath context\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        xmlSchematronFreeParserCtxt(ret);
        return 0 as xmlSchematronParserCtxtPtr
    }
    return ret;
}
/* *
 * xmlSchematronFreeParserCtxt:
 * @ctxt:  the schema parser context
 *
 * Free the resources associated to the schema parser context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronFreeParserCtxt(mut ctxt:
                                                         xmlSchematronParserCtxtPtr) {
    if ctxt.is_null() { return }
    if !(*ctxt).doc.is_null() && (*ctxt).preserve == 0 {
        xmlFreeDoc((*ctxt).doc);
    }
    if !(*ctxt).xctxt.is_null() { xmlXPathFreeContext((*ctxt).xctxt); }
    if !(*ctxt).namespaces.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).namespaces as
                                                        *mut *mut std::os::raw::c_char
                                                        as *mut std::os::raw::c_void);
    }
    xmlDictFree((*ctxt).dict);
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
/* *
 * xmlSchematronAddNamespace:
 * @ctxt:  the schema parser context
 * @prefix:  the namespace prefix
 * @ns:  the namespace name
 *
 * Add a namespace definition in the context
 */
unsafe extern "C" fn xmlSchematronAddNamespace(mut ctxt:
                                                   xmlSchematronParserCtxtPtr,
                                               mut prefix: *const xmlChar,
                                               mut ns: *const xmlChar) {
    if (*ctxt).namespaces.is_null() {
        (*ctxt).maxNamespaces = 10 as std::os::raw::c_int;
        (*ctxt).namespaces =
            xmlMalloc.expect("non-null function pointer")((((*ctxt).maxNamespaces
                                                                *
                                                                2 as
                                                                    std::os::raw::c_int)
                                                               as
                                                               std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                                                                               as
                                                                                               std::os::raw::c_ulong))
                as *mut *const xmlChar;
        if (*ctxt).namespaces.is_null() {
            xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                    b"allocating parser namespaces\x00" as
                                        *const u8 as *const std::os::raw::c_char,
                                    0 as xmlNodePtr);
            return
        }
        (*ctxt).nbNamespaces = 0 as std::os::raw::c_int
    } else if (*ctxt).nbNamespaces + 2 as std::os::raw::c_int >= (*ctxt).maxNamespaces
     {
        let mut tmp: *mut *const xmlChar = 0 as *mut *const xmlChar;
        tmp =
            xmlRealloc.expect("non-null function pointer")((*ctxt).namespaces
                                                               as
                                                               *mut *mut xmlChar
                                                               as
                                                               *mut std::os::raw::c_void,
                                                           (((*ctxt).maxNamespaces
                                                                 *
                                                                 4 as
                                                                     std::os::raw::c_int)
                                                                as
                                                                std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*const xmlChar>()
                                                                                                as
                                                                                                std::os::raw::c_ulong))
                as *mut *const xmlChar;
        if tmp.is_null() {
            xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                    b"allocating parser namespaces\x00" as
                                        *const u8 as *const std::os::raw::c_char,
                                    0 as xmlNodePtr);
            return
        }
        (*ctxt).namespaces = tmp;
        (*ctxt).maxNamespaces *= 2 as std::os::raw::c_int
    }
    let ref mut fresh0 =
        *(*ctxt).namespaces.offset((2 as std::os::raw::c_int * (*ctxt).nbNamespaces)
                                       as isize);
    *fresh0 = xmlDictLookup((*ctxt).dict, ns, -(1 as std::os::raw::c_int));
    let ref mut fresh1 =
        *(*ctxt).namespaces.offset((2 as std::os::raw::c_int * (*ctxt).nbNamespaces +
                                        1 as std::os::raw::c_int) as isize);
    *fresh1 = xmlDictLookup((*ctxt).dict, prefix, -(1 as std::os::raw::c_int));
    (*ctxt).nbNamespaces += 1;
    let ref mut fresh2 =
        *(*ctxt).namespaces.offset((2 as std::os::raw::c_int * (*ctxt).nbNamespaces)
                                       as isize);
    *fresh2 = 0 as *const xmlChar;
    let ref mut fresh3 =
        *(*ctxt).namespaces.offset((2 as std::os::raw::c_int * (*ctxt).nbNamespaces +
                                        1 as std::os::raw::c_int) as isize);
    *fresh3 = 0 as *const xmlChar;
}
/* *
 * xmlSchematronParseRule:
 * @ctxt:  a schema validation context
 * @rule:  the rule node
 *
 * parse a rule element
 */
unsafe extern "C" fn xmlSchematronParseRule(mut ctxt:
                                                xmlSchematronParserCtxtPtr,
                                            mut pattern:
                                                xmlSchematronPatternPtr,
                                            mut rule: xmlNodePtr) {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut nbChecks: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut test: *mut xmlChar = 0 as *mut xmlChar;
    let mut context: *mut xmlChar = 0 as *mut xmlChar;
    let mut report: *mut xmlChar = 0 as *mut xmlChar;
    let mut ruleptr: xmlSchematronRulePtr = 0 as *mut xmlSchematronRule;
    let mut testptr: xmlSchematronTestPtr = 0 as *mut xmlSchematronTest;
    if ctxt.is_null() || rule.is_null() { return }
    context =
        xmlGetNoNsProp(rule as *const xmlNode,
                       b"context\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
    if context.is_null() {
        xmlSchematronPErr(ctxt, rule, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"rule has no context attribute\x00" as *const u8 as
                              *const std::os::raw::c_char, 0 as *const xmlChar,
                          0 as *const xmlChar);
        return
    } else {
        if *context.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0 as std::os::raw::c_int {
            xmlSchematronPErr(ctxt, rule, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                              b"rule has an empty context attribute\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                              0 as *const xmlChar, 0 as *const xmlChar);
            xmlFree.expect("non-null function pointer")(context as
                                                            *mut std::os::raw::c_void);
            return
        } else {
            ruleptr =
                xmlSchematronAddRule(ctxt, (*ctxt).schema, pattern, rule,
                                     context, 0 as *mut xmlChar);
            if ruleptr.is_null() {
                xmlFree.expect("non-null function pointer")(context as
                                                                *mut std::os::raw::c_void);
                return
            }
        }
    }
    cur = (*rule).children;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).ns.is_null() &&
               (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                    xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) != 0) {
            break ;
        }
        cur = (*cur).next
    }
    while !cur.is_null() {
        if !cur.is_null() &&
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).ns.is_null() &&
               xmlStrEqual((*cur).name,
                           b"assert\x00" as *const u8 as *const std::os::raw::c_char
                               as *const xmlChar) != 0 &&
               (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                    xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) != 0) {
            nbChecks += 1;
            test =
                xmlGetNoNsProp(cur as *const xmlNode,
                               b"test\x00" as *const u8 as *const std::os::raw::c_char
                                   as *mut xmlChar);
            if test.is_null() {
                xmlSchematronPErr(ctxt, cur,
                                  XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                  b"assert has no test attribute\x00" as
                                      *const u8 as *const std::os::raw::c_char,
                                  0 as *const xmlChar, 0 as *const xmlChar);
            } else if *test.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                          == 0 as std::os::raw::c_int {
                xmlSchematronPErr(ctxt, cur,
                                  XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                  b"assert has an empty test attribute\x00" as
                                      *const u8 as *const std::os::raw::c_char,
                                  0 as *const xmlChar, 0 as *const xmlChar);
                xmlFree.expect("non-null function pointer")(test as
                                                                *mut std::os::raw::c_void);
            } else {
                /* TODO will need dynamic processing instead */
                report = xmlNodeGetContent(cur as *const xmlNode);
                testptr =
                    xmlSchematronAddTest(ctxt, XML_SCHEMATRON_ASSERT, ruleptr,
                                         cur, test, report);
                if testptr.is_null() {
                    xmlFree.expect("non-null function pointer")(test as
                                                                    *mut std::os::raw::c_void);
                }
            }
        } else if !cur.is_null() &&
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      !(*cur).ns.is_null() &&
                      xmlStrEqual((*cur).name,
                                  b"report\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                           xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs)
                               != 0) {
            nbChecks += 1;
            test =
                xmlGetNoNsProp(cur as *const xmlNode,
                               b"test\x00" as *const u8 as *const std::os::raw::c_char
                                   as *mut xmlChar);
            if test.is_null() {
                xmlSchematronPErr(ctxt, cur,
                                  XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                  b"assert has no test attribute\x00" as
                                      *const u8 as *const std::os::raw::c_char,
                                  0 as *const xmlChar, 0 as *const xmlChar);
            } else if *test.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                          == 0 as std::os::raw::c_int {
                xmlSchematronPErr(ctxt, cur,
                                  XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                  b"assert has an empty test attribute\x00" as
                                      *const u8 as *const std::os::raw::c_char,
                                  0 as *const xmlChar, 0 as *const xmlChar);
                xmlFree.expect("non-null function pointer")(test as
                                                                *mut std::os::raw::c_void);
            } else {
                /* TODO will need dynamic processing instead */
                report = xmlNodeGetContent(cur as *const xmlNode);
                testptr =
                    xmlSchematronAddTest(ctxt, XML_SCHEMATRON_REPORT, ruleptr,
                                         cur, test, report);
                if testptr.is_null() {
                    xmlFree.expect("non-null function pointer")(test as
                                                                    *mut std::os::raw::c_void);
                }
            }
        } else {
            xmlSchematronPErr(ctxt, cur, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                              b"Expecting an assert or a report element instead of %s\x00"
                                  as *const u8 as *const std::os::raw::c_char,
                              (*cur).name, 0 as *const xmlChar);
        }
        cur = (*cur).next;
        while !cur.is_null() {
            if (*cur).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   !(*cur).ns.is_null() &&
                   (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                        xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) !=
                            0) {
                break ;
            }
            cur = (*cur).next
        }
    }
    if nbChecks == 0 as std::os::raw::c_int {
        xmlSchematronPErr(ctxt, rule, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"rule has no assert nor report element\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          0 as *const xmlChar, 0 as *const xmlChar);
    };
}
/* *
 * xmlSchematronParsePattern:
 * @ctxt:  a schema validation context
 * @pat:  the pattern node
 *
 * parse a pattern element
 */
unsafe extern "C" fn xmlSchematronParsePattern(mut ctxt:
                                                   xmlSchematronParserCtxtPtr,
                                               mut pat: xmlNodePtr) {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut pattern: xmlSchematronPatternPtr = 0 as *mut xmlSchematronPattern;
    let mut nbRules: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut id: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() || pat.is_null() { return }
    id =
        xmlGetNoNsProp(pat as *const xmlNode,
                       b"id\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
    if id.is_null() {
        id =
            xmlGetNoNsProp(pat as *const xmlNode,
                           b"name\x00" as *const u8 as *const std::os::raw::c_char as
                               *mut xmlChar)
    }
    pattern = xmlSchematronAddPattern(ctxt, (*ctxt).schema, pat, id);
    if pattern.is_null() {
        if !id.is_null() {
            xmlFree.expect("non-null function pointer")(id as
                                                            *mut std::os::raw::c_void);
        }
        return
    }
    cur = (*pat).children;
    while !cur.is_null() {
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).ns.is_null() &&
               (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                    xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) != 0) {
            break ;
        }
        cur = (*cur).next
    }
    while !cur.is_null() {
        if !cur.is_null() &&
               (*cur).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
               !(*cur).ns.is_null() &&
               xmlStrEqual((*cur).name,
                           b"rule\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar) != 0 &&
               (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                    xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) != 0) {
            xmlSchematronParseRule(ctxt, pattern, cur);
            nbRules += 1
        } else {
            xmlSchematronPErr(ctxt, cur, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                              b"Expecting a rule element instead of %s\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                              (*cur).name, 0 as *const xmlChar);
        }
        cur = (*cur).next;
        while !cur.is_null() {
            if (*cur).type_0 as std::os::raw::c_uint ==
                   XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   !(*cur).ns.is_null() &&
                   (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                        xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) !=
                            0) {
                break ;
            }
            cur = (*cur).next
        }
    }
    if nbRules == 0 as std::os::raw::c_int {
        xmlSchematronPErr(ctxt, pat, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"Pattern has no rule element\x00" as *const u8 as
                              *const std::os::raw::c_char, 0 as *const xmlChar,
                          0 as *const xmlChar);
    };
}
/* ****
XMLPUBFUN void XMLCALL
	    xmlSchematronSetParserErrors(xmlSchematronParserCtxtPtr ctxt,
					 xmlSchematronValidityErrorFunc err,
					 xmlSchematronValidityWarningFunc warn,
					 void *ctx);
XMLPUBFUN int XMLCALL
		xmlSchematronGetParserErrors(xmlSchematronParserCtxtPtr ctxt,
					xmlSchematronValidityErrorFunc * err,
					xmlSchematronValidityWarningFunc * warn,
					void **ctx);
XMLPUBFUN int XMLCALL
		xmlSchematronIsValid	(xmlSchematronValidCtxtPtr ctxt);
 *****/
/* *
 * xmlSchematronParse:
 * @ctxt:  a schema validation context
 *
 * parse a schema definition resource and build an internal
 * XML Shema struture which can be used to validate instances.
 *
 * Returns the internal XML Schematron structure built from the resource or
 *         NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronParse(mut ctxt:
                                                xmlSchematronParserCtxtPtr)
 -> xmlSchematronPtr {
    let mut ret: xmlSchematronPtr = 0 as xmlSchematronPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut preserve: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ctxt.is_null() { return 0 as xmlSchematronPtr }
    (*ctxt).nberrors = 0 as std::os::raw::c_int;
    /*
     * First step is to parse the input document into an DOM/Infoset
     */
    if !(*ctxt).URL.is_null() {
        doc =
            xmlReadFile((*ctxt).URL as *const std::os::raw::c_char,
                        0 as *const std::os::raw::c_char,
                        XML_PARSE_NOENT as std::os::raw::c_int);
        if doc.is_null() {
            xmlSchematronPErr(ctxt, 0 as xmlNodePtr,
                              XML_SCHEMAP_FAILED_LOAD as std::os::raw::c_int,
                              b"xmlSchematronParse: could not load \'%s\'.\n\x00"
                                  as *const u8 as *const std::os::raw::c_char,
                              (*ctxt).URL, 0 as *const xmlChar);
            return 0 as xmlSchematronPtr
        }
        (*ctxt).preserve = 0 as std::os::raw::c_int
    } else if !(*ctxt).buffer.is_null() {
        doc =
            xmlReadMemory((*ctxt).buffer, (*ctxt).size,
                          0 as *const std::os::raw::c_char, 0 as *const std::os::raw::c_char,
                          XML_PARSE_NOENT as std::os::raw::c_int);
        if doc.is_null() {
            xmlSchematronPErr(ctxt, 0 as xmlNodePtr,
                              XML_SCHEMAP_FAILED_PARSE as std::os::raw::c_int,
                              b"xmlSchematronParse: could not parse.\n\x00" as
                                  *const u8 as *const std::os::raw::c_char,
                              0 as *const xmlChar, 0 as *const xmlChar);
            return 0 as xmlSchematronPtr
        }
        (*doc).URL =
            xmlStrdup(b"in_memory_buffer\x00" as *const u8 as
                          *const std::os::raw::c_char as *mut xmlChar);
        (*ctxt).URL =
            xmlDictLookup((*ctxt).dict,
                          b"in_memory_buffer\x00" as *const u8 as
                              *const std::os::raw::c_char as *mut xmlChar,
                          -(1 as std::os::raw::c_int));
        (*ctxt).preserve = 0 as std::os::raw::c_int
    } else if !(*ctxt).doc.is_null() {
        doc = (*ctxt).doc;
        preserve = 1 as std::os::raw::c_int;
        (*ctxt).preserve = 1 as std::os::raw::c_int
    } else {
        xmlSchematronPErr(ctxt, 0 as xmlNodePtr,
                          XML_SCHEMAP_NOTHING_TO_PARSE as std::os::raw::c_int,
                          b"xmlSchematronParse: could not parse.\n\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          0 as *const xmlChar, 0 as *const xmlChar);
        return 0 as xmlSchematronPtr
    }
    /*
     * Then extract the root and Schematron parse it
     */
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlSchematronPErr(ctxt, doc as xmlNodePtr,
                          XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"The schema has no document element.\n\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          0 as *const xmlChar, 0 as *const xmlChar);
        if preserve == 0 { xmlFreeDoc(doc); }
        return 0 as xmlSchematronPtr
    }
    if !(!root.is_null() &&
             (*root).type_0 as std::os::raw::c_uint ==
                 XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
             !(*root).ns.is_null() &&
             xmlStrEqual((*root).name,
                         b"schema\x00" as *const u8 as *const std::os::raw::c_char as
                             *const xmlChar) != 0 &&
             (xmlStrEqual((*(*root).ns).href, xmlSchematronNs) != 0 ||
                  xmlStrEqual((*(*root).ns).href, xmlOldSchematronNs) != 0)) {
        xmlSchematronPErr(ctxt, root, XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                          b"The XML document \'%s\' is not a XML schematron document\x00"
                              as *const u8 as *const std::os::raw::c_char,
                          (*ctxt).URL, 0 as *const xmlChar);
    } else {
        ret = xmlSchematronNewSchematron(ctxt);
        if !ret.is_null() {
            (*ctxt).schema = ret;
            /*
     * scan the schema elements
     */
            cur = (*root).children;
            while !cur.is_null() {
                if (*cur).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                       !(*cur).ns.is_null() &&
                       (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0
                            ||
                            xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs)
                                != 0) {
                    break ;
                }
                cur = (*cur).next
            }
            if !cur.is_null() &&
                   (*cur).type_0 as std::os::raw::c_uint ==
                       XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                   !(*cur).ns.is_null() &&
                   xmlStrEqual((*cur).name,
                               b"title\x00" as *const u8 as
                                   *const std::os::raw::c_char as *const xmlChar) != 0
                   &&
                   (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                        xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs) !=
                            0) {
                let mut title: *mut xmlChar =
                    xmlNodeGetContent(cur as *const xmlNode);
                if !title.is_null() {
                    (*ret).title =
                        xmlDictLookup((*ret).dict, title,
                                      -(1 as std::os::raw::c_int));
                    xmlFree.expect("non-null function pointer")(title as
                                                                    *mut std::os::raw::c_void);
                }
                cur = (*cur).next;
                while !cur.is_null() {
                    if (*cur).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                           !(*cur).ns.is_null() &&
                           (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) !=
                                0 ||
                                xmlStrEqual((*(*cur).ns).href,
                                            xmlOldSchematronNs) != 0) {
                        break ;
                    }
                    cur = (*cur).next
                }
            }
            while !cur.is_null() &&
                      (*cur).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      !(*cur).ns.is_null() &&
                      xmlStrEqual((*cur).name,
                                  b"ns\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0 ||
                           xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs)
                               != 0) {
                let mut prefix: *mut xmlChar =
                    xmlGetNoNsProp(cur as *const xmlNode,
                                   b"prefix\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                let mut uri: *mut xmlChar =
                    xmlGetNoNsProp(cur as *const xmlNode,
                                   b"uri\x00" as *const u8 as
                                       *const std::os::raw::c_char as *mut xmlChar);
                if uri.is_null() ||
                       *uri.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 0 as std::os::raw::c_int {
                    xmlSchematronPErr(ctxt, cur,
                                      XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                      b"ns element has no uri\x00" as
                                          *const u8 as *const std::os::raw::c_char,
                                      0 as *const xmlChar,
                                      0 as *const xmlChar);
                }
                if prefix.is_null() ||
                       *prefix.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int == 0 as std::os::raw::c_int {
                    xmlSchematronPErr(ctxt, cur,
                                      XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                      b"ns element has no prefix\x00" as
                                          *const u8 as *const std::os::raw::c_char,
                                      0 as *const xmlChar,
                                      0 as *const xmlChar);
                }
                if !prefix.is_null() && !uri.is_null() {
                    xmlXPathRegisterNs((*ctxt).xctxt, prefix, uri);
                    xmlSchematronAddNamespace(ctxt, prefix, uri);
                    (*ret).nbNs += 1
                }
                if !uri.is_null() {
                    xmlFree.expect("non-null function pointer")(uri as
                                                                    *mut std::os::raw::c_void);
                }
                if !prefix.is_null() {
                    xmlFree.expect("non-null function pointer")(prefix as
                                                                    *mut std::os::raw::c_void);
                }
                cur = (*cur).next;
                while !cur.is_null() {
                    if (*cur).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                           !(*cur).ns.is_null() &&
                           (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) !=
                                0 ||
                                xmlStrEqual((*(*cur).ns).href,
                                            xmlOldSchematronNs) != 0) {
                        break ;
                    }
                    cur = (*cur).next
                }
            }
            while !cur.is_null() {
                if !cur.is_null() &&
                       (*cur).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                       !(*cur).ns.is_null() &&
                       xmlStrEqual((*cur).name,
                                   b"pattern\x00" as *const u8 as
                                       *const std::os::raw::c_char as *const xmlChar)
                           != 0 &&
                       (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) != 0
                            ||
                            xmlStrEqual((*(*cur).ns).href, xmlOldSchematronNs)
                                != 0) {
                    xmlSchematronParsePattern(ctxt, cur);
                    (*ret).nbPattern += 1
                } else {
                    xmlSchematronPErr(ctxt, cur,
                                      XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                      b"Expecting a pattern element instead of %s\x00"
                                          as *const u8 as *const std::os::raw::c_char,
                                      (*cur).name, 0 as *const xmlChar);
                }
                cur = (*cur).next;
                while !cur.is_null() {
                    if (*cur).type_0 as std::os::raw::c_uint ==
                           XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                           !(*cur).ns.is_null() &&
                           (xmlStrEqual((*(*cur).ns).href, xmlSchematronNs) !=
                                0 ||
                                xmlStrEqual((*(*cur).ns).href,
                                            xmlOldSchematronNs) != 0) {
                        break ;
                    }
                    cur = (*cur).next
                }
            }
            if (*ret).nbPattern == 0 as std::os::raw::c_int {
                xmlSchematronPErr(ctxt, root,
                                  XML_SCHEMAP_NOROOT as std::os::raw::c_int,
                                  b"The schematron document \'%s\' has no pattern\x00"
                                      as *const u8 as *const std::os::raw::c_char,
                                  (*ctxt).URL, 0 as *const xmlChar);
            } else {
                /* the original document must be kept for reporting */
                (*ret).doc = doc;
                if preserve != 0 { (*ret).preserve = 1 as std::os::raw::c_int }
                preserve = 1 as std::os::raw::c_int
            }
        }
    }
    if preserve == 0 { xmlFreeDoc(doc); }
    if !ret.is_null() {
        if (*ctxt).nberrors != 0 as std::os::raw::c_int {
            xmlSchematronFree(ret);
            ret = 0 as xmlSchematronPtr
        } else {
            (*ret).namespaces = (*ctxt).namespaces;
            (*ret).nbNamespaces = (*ctxt).nbNamespaces;
            (*ctxt).namespaces = 0 as *mut *const xmlChar
        }
    }
    return ret;
}
/* ***********************************************************************
 *									*
 *		Schematrontron Reports handler				*
 *									*
 ************************************************************************/
unsafe extern "C" fn xmlSchematronGetNode(mut ctxt: xmlSchematronValidCtxtPtr,
                                          mut cur: xmlNodePtr,
                                          mut xpath: *const xmlChar)
 -> xmlNodePtr {
    let mut node: xmlNodePtr = 0 as xmlNodePtr;
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() || cur.is_null() || xpath.is_null() {
        return 0 as xmlNodePtr
    }
    (*(*ctxt).xctxt).doc = (*cur).doc;
    (*(*ctxt).xctxt).node = cur;
    ret = xmlXPathEval(xpath, (*ctxt).xctxt);
    if ret.is_null() { return 0 as xmlNodePtr }
    if (*ret).type_0 as std::os::raw::c_uint ==
           XPATH_NODESET as std::os::raw::c_int as std::os::raw::c_uint &&
           !(*ret).nodesetval.is_null() &&
           (*(*ret).nodesetval).nodeNr > 0 as std::os::raw::c_int {
        node = *(*(*ret).nodesetval).nodeTab.offset(0 as std::os::raw::c_int as isize)
    }
    xmlXPathFreeObject(ret);
    return node;
}
/* *
 * xmlSchematronReportOutput:
 * @ctxt: the validation context
 * @cur: the current node tested
 * @msg: the message output
 *
 * Output part of the report to whatever channel the user selected
 */
unsafe extern "C" fn xmlSchematronReportOutput(mut ctxt:
                                                   xmlSchematronValidCtxtPtr,
                                               mut cur: xmlNodePtr,
                                               mut msg: *const std::os::raw::c_char) {
    /* TODO */
    fprintf(stderr, b"%s\x00" as *const u8 as *const std::os::raw::c_char, msg);
}
/* *
 * xmlSchematronFormatReport:
 * @ctxt:  the validation context
 * @test: the test node
 * @cur: the current node tested
 *
 * Build the string being reported to the user.
 *
 * Returns a report string or NULL in case of error. The string needs
 *         to be deallocated by teh caller
 */
unsafe extern "C" fn xmlSchematronFormatReport(mut ctxt:
                                                   xmlSchematronValidCtxtPtr,
                                               mut test: xmlNodePtr,
                                               mut cur: xmlNodePtr)
 -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if test.is_null() || cur.is_null() { return ret }
    child = (*test).children;
    while !child.is_null() {
        if (*child).type_0 as std::os::raw::c_uint ==
               XML_TEXT_NODE as std::os::raw::c_int as std::os::raw::c_uint ||
               (*child).type_0 as std::os::raw::c_uint ==
                   XML_CDATA_SECTION_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            ret = xmlStrcat(ret, (*child).content)
        } else if !child.is_null() &&
                      (*child).type_0 as std::os::raw::c_uint ==
                          XML_ELEMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint &&
                      !(*child).ns.is_null() &&
                      xmlStrEqual((*child).name,
                                  b"name\x00" as *const u8 as
                                      *const std::os::raw::c_char as *const xmlChar)
                          != 0 &&
                      (xmlStrEqual((*(*child).ns).href, xmlSchematronNs) != 0
                           ||
                           xmlStrEqual((*(*child).ns).href,
                                       xmlOldSchematronNs) != 0) {
            let mut path: *mut xmlChar = 0 as *mut xmlChar;
            path =
                xmlGetNoNsProp(child as *const xmlNode,
                               b"path\x00" as *const u8 as *const std::os::raw::c_char
                                   as *mut xmlChar);
            node = cur;
            if !path.is_null() {
                node = xmlSchematronGetNode(ctxt, cur, path);
                if node.is_null() { node = cur }
                xmlFree.expect("non-null function pointer")(path as
                                                                *mut std::os::raw::c_void);
            }
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                ret = xmlStrcat(ret, (*node).name)
            } else {
                ret = xmlStrcat(ret, (*(*node).ns).prefix);
                ret =
                    xmlStrcat(ret,
                              b":\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar);
                ret = xmlStrcat(ret, (*node).name)
            }
        } else { child = (*child).next; continue ; }
        /*
	 * remove superfluous \n
	 */
        if !ret.is_null() {
            let mut len: std::os::raw::c_int = xmlStrlen(ret);
            let mut c: xmlChar = 0;
            if len > 0 as std::os::raw::c_int {
                c = *ret.offset((len - 1 as std::os::raw::c_int) as isize);
                if c as std::os::raw::c_int == ' ' as i32 ||
                       c as std::os::raw::c_int == '\n' as i32 ||
                       c as std::os::raw::c_int == '\r' as i32 ||
                       c as std::os::raw::c_int == '\t' as i32 {
                    while c as std::os::raw::c_int == ' ' as i32 ||
                              c as std::os::raw::c_int == '\n' as i32 ||
                              c as std::os::raw::c_int == '\r' as i32 ||
                              c as std::os::raw::c_int == '\t' as i32 {
                        len -= 1;
                        if len == 0 as std::os::raw::c_int { break ; }
                        c = *ret.offset((len - 1 as std::os::raw::c_int) as isize)
                    }
                    *ret.offset(len as isize) = ' ' as i32 as xmlChar;
                    *ret.offset((len + 1 as std::os::raw::c_int) as isize) =
                        0 as std::os::raw::c_int as xmlChar
                }
            }
        }
        child = (*child).next
    }
    return ret;
}
/* *
 * xmlSchematronReportSuccess:
 * @ctxt:  the validation context
 * @test: the compiled test
 * @cur: the current node tested
 * @success: boolean value for the result
 *
 * called from the validation engine when an assert or report test have
 * been done.
 */
unsafe extern "C" fn xmlSchematronReportSuccess(mut ctxt:
                                                    xmlSchematronValidCtxtPtr,
                                                mut test:
                                                    xmlSchematronTestPtr,
                                                mut cur: xmlNodePtr,
                                                mut pattern:
                                                    xmlSchematronPatternPtr,
                                                mut success: std::os::raw::c_int) {
    if ctxt.is_null() || cur.is_null() || test.is_null() { return }
    /* if quiet and not SVRL report only failures */
    if (*ctxt).flags & XML_SCHEMATRON_OUT_QUIET as std::os::raw::c_int != 0 &&
           (*ctxt).flags & XML_SCHEMATRON_OUT_XML as std::os::raw::c_int ==
               0 as std::os::raw::c_int &&
           (*test).type_0 as std::os::raw::c_uint ==
               XML_SCHEMATRON_REPORT as std::os::raw::c_int as std::os::raw::c_uint {
        return
    }
    if (*ctxt).flags & XML_SCHEMATRON_OUT_XML as std::os::raw::c_int != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"schematron.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   1380 as
                                                                       std::os::raw::c_int);
    } else {
        let mut path: *mut xmlChar = 0 as *mut xmlChar;
        let mut msg: [std::os::raw::c_char; 1000] = [0; 1000];
        let mut line: std::os::raw::c_long = 0;
        let mut report: *const xmlChar = 0 as *const xmlChar;
        if ((*test).type_0 as std::os::raw::c_uint ==
                XML_SCHEMATRON_REPORT as std::os::raw::c_int as std::os::raw::c_uint) as
               std::os::raw::c_int & (success == 0) as std::os::raw::c_int != 0 ||
               ((*test).type_0 as std::os::raw::c_uint ==
                    XML_SCHEMATRON_ASSERT as std::os::raw::c_int as std::os::raw::c_uint) as
                   std::os::raw::c_int & success != 0 {
            return
        }
        line = xmlGetLineNo(cur as *const xmlNode);
        path = xmlGetNodePath(cur as *const xmlNode);
        if path.is_null() { path = (*cur).name as *mut xmlChar }
        if !(*test).node.is_null() {
            report = xmlSchematronFormatReport(ctxt, (*test).node, cur)
        }
        if report.is_null() {
            if (*test).type_0 as std::os::raw::c_uint ==
                   XML_SCHEMATRON_ASSERT as std::os::raw::c_int as std::os::raw::c_uint {
                report =
                    xmlStrdup(b"node failed assert\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar)
            } else {
                report =
                    xmlStrdup(b"node failed report\x00" as *const u8 as
                                  *const std::os::raw::c_char as *const xmlChar)
            }
        }
        snprintf(msg.as_mut_ptr(), 999 as std::os::raw::c_int as std::os::raw::c_ulong,
                 b"%s line %ld: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                 path as *const std::os::raw::c_char, line,
                 report as *const std::os::raw::c_char);
        if (*ctxt).flags & XML_SCHEMATRON_OUT_ERROR as std::os::raw::c_int != 0 {
            let mut schannel: xmlStructuredErrorFunc = None;
            let mut channel: xmlGenericErrorFunc = None;
            let mut data: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
            if !ctxt.is_null() {
                if (*ctxt).serror.is_some() {
                    schannel = (*ctxt).serror
                } else { channel = (*ctxt).error }
                data = (*ctxt).userData
            }
            __xmlRaiseError(schannel, channel, data, 0 as *mut std::os::raw::c_void,
                            cur as *mut std::os::raw::c_void,
                            XML_FROM_SCHEMATRONV as std::os::raw::c_int,
                            if (*test).type_0 as std::os::raw::c_uint ==
                                   XML_SCHEMATRON_ASSERT as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                                XML_SCHEMATRONV_ASSERT as std::os::raw::c_int
                            } else { XML_SCHEMATRONV_REPORT as std::os::raw::c_int },
                            XML_ERR_ERROR, 0 as *const std::os::raw::c_char,
                            line as std::os::raw::c_int,
                            if pattern.is_null() {
                                0 as *const std::os::raw::c_char
                            } else { (*pattern).name as *const std::os::raw::c_char },
                            path as *const std::os::raw::c_char,
                            report as *const std::os::raw::c_char, 0 as std::os::raw::c_int,
                            0 as std::os::raw::c_int,
                            b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                            msg.as_mut_ptr());
        } else {
            xmlSchematronReportOutput(ctxt, cur,
                                      &mut *msg.as_mut_ptr().offset(0 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        isize));
        }
        xmlFree.expect("non-null function pointer")(report as
                                                        *mut std::os::raw::c_char as
                                                        *mut std::os::raw::c_void);
        if !path.is_null() && path != (*cur).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(path as
                                                            *mut std::os::raw::c_void);
        }
    };
}
/* *
 * xmlSchematronReportPattern:
 * @ctxt:  the validation context
 * @pattern: the current pattern
 *
 * called from the validation engine when starting to check a pattern
 */
unsafe extern "C" fn xmlSchematronReportPattern(mut ctxt:
                                                    xmlSchematronValidCtxtPtr,
                                                mut pattern:
                                                    xmlSchematronPatternPtr) {
    if ctxt.is_null() || pattern.is_null() { return }
    if (*ctxt).flags & XML_SCHEMATRON_OUT_QUIET as std::os::raw::c_int != 0 ||
           (*ctxt).flags & XML_SCHEMATRON_OUT_ERROR as std::os::raw::c_int != 0 {
        /* Error gives pattern name as part of error */
        return
    }
    if (*ctxt).flags & XML_SCHEMATRON_OUT_XML as std::os::raw::c_int != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"schematron.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   1457 as
                                                                       std::os::raw::c_int);
    } else {
        let mut msg: [std::os::raw::c_char; 1000] = [0; 1000];
        if (*pattern).name.is_null() { return }
        snprintf(msg.as_mut_ptr(), 999 as std::os::raw::c_int as std::os::raw::c_ulong,
                 b"Pattern: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                 (*pattern).name as *const std::os::raw::c_char);
        xmlSchematronReportOutput(ctxt, 0 as xmlNodePtr,
                                  &mut *msg.as_mut_ptr().offset(0 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    isize));
    };
}
/*
 * Interfaces for validating
 */
/* ***********************************************************************
 *									*
 *		Validation against a Schematrontron				*
 *									*
 ************************************************************************/
/* *
 * xmlSchematronSetValidStructuredErrors:
 * @ctxt:  a Schematron validation context
 * @serror:  the structured error function
 * @ctx: the functions context
 *
 * Set the structured error callback
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronSetValidStructuredErrors(mut ctxt:
                                                                   xmlSchematronValidCtxtPtr,
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
/* *****
XMLPUBFUN void XMLCALL
	    xmlSchematronSetValidErrors	(xmlSchematronValidCtxtPtr ctxt,
					 xmlSchematronValidityErrorFunc err,
					 xmlSchematronValidityWarningFunc warn,
					 void *ctx);
XMLPUBFUN int XMLCALL
	    xmlSchematronGetValidErrors	(xmlSchematronValidCtxtPtr ctxt,
					 xmlSchematronValidityErrorFunc *err,
					 xmlSchematronValidityWarningFunc *warn,
					 void **ctx);
XMLPUBFUN int XMLCALL
	    xmlSchematronSetValidOptions(xmlSchematronValidCtxtPtr ctxt,
					 int options);
XMLPUBFUN int XMLCALL
	    xmlSchematronValidCtxtGetOptions(xmlSchematronValidCtxtPtr ctxt);
XMLPUBFUN int XMLCALL
            xmlSchematronValidateOneElement (xmlSchematronValidCtxtPtr ctxt,
			                 xmlNodePtr elem);
 *******/
/* *
 * xmlSchematronNewValidCtxt:
 * @schema:  a precompiled XML Schematrons
 * @options: a set of xmlSchematronValidOptions
 *
 * Create an XML Schematrons validation context based on the given schema.
 *
 * Returns the validation context or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronNewValidCtxt(mut schema:
                                                       xmlSchematronPtr,
                                                   mut options: std::os::raw::c_int)
 -> xmlSchematronValidCtxtPtr {
    let mut i: std::os::raw::c_int = 0;
    let mut ret: xmlSchematronValidCtxtPtr = 0 as *mut xmlSchematronValidCtxt;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchematronValidCtxt>()
                                                          as std::os::raw::c_ulong) as
            xmlSchematronValidCtxtPtr;
    if ret.is_null() {
        xmlSchematronVErrMemory(0 as xmlSchematronValidCtxtPtr,
                                b"allocating validation context\x00" as
                                    *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        return 0 as xmlSchematronValidCtxtPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchematronValidCtxt>() as std::os::raw::c_ulong);
    (*ret).type_0 = 2 as std::os::raw::c_int;
    (*ret).schema = schema;
    (*ret).xctxt = xmlXPathNewContext(0 as xmlDocPtr);
    (*ret).flags = options;
    if (*ret).xctxt.is_null() {
        xmlSchematronPErrMemory(0 as xmlSchematronParserCtxtPtr,
                                b"allocating schema parser XPath context\x00"
                                    as *const u8 as *const std::os::raw::c_char,
                                0 as xmlNodePtr);
        xmlSchematronFreeValidCtxt(ret);
        return 0 as xmlSchematronValidCtxtPtr
    }
    i = 0 as std::os::raw::c_int;
    while i < (*schema).nbNamespaces {
        if (*(*schema).namespaces.offset((2 as std::os::raw::c_int * i) as
                                             isize)).is_null() ||
               (*(*schema).namespaces.offset((2 as std::os::raw::c_int * i +
                                                  1 as std::os::raw::c_int) as
                                                 isize)).is_null() {
            break ;
        }
        xmlXPathRegisterNs((*ret).xctxt,
                           *(*schema).namespaces.offset((2 as std::os::raw::c_int * i
                                                             +
                                                             1 as std::os::raw::c_int)
                                                            as isize),
                           *(*schema).namespaces.offset((2 as std::os::raw::c_int * i)
                                                            as isize));
        i += 1
    }
    return ret;
}
/* *
 * xmlSchematronFreeValidCtxt:
 * @ctxt:  the schema validation context
 *
 * Free the resources associated to the schema validation context
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronFreeValidCtxt(mut ctxt:
                                                        xmlSchematronValidCtxtPtr) {
    if ctxt.is_null() { return }
    if !(*ctxt).xctxt.is_null() { xmlXPathFreeContext((*ctxt).xctxt); }
    if !(*ctxt).dict.is_null() { xmlDictFree((*ctxt).dict); }
    xmlFree.expect("non-null function pointer")(ctxt as *mut std::os::raw::c_void);
}
unsafe extern "C" fn xmlSchematronNextNode(mut cur: xmlNodePtr)
 -> xmlNodePtr {
    if !(*cur).children.is_null() {
        /*
	 * Do not descend on entities declarations
	 */
        if (*(*cur).children).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint {
            cur = (*cur).children;
            /*
	     * Skip DTDs
	     */
            if (*cur).type_0 as std::os::raw::c_uint !=
                   XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                return cur
            }
        }
    }
    while !(*cur).next.is_null() {
        cur = (*cur).next;
        if (*cur).type_0 as std::os::raw::c_uint !=
               XML_ENTITY_DECL as std::os::raw::c_int as std::os::raw::c_uint &&
               (*cur).type_0 as std::os::raw::c_uint !=
                   XML_DTD_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return cur
        }
    }
    loop  {
        cur = (*cur).parent;
        if cur.is_null() { break ; }
        if (*cur).type_0 as std::os::raw::c_uint ==
               XML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as xmlNodePtr
        }
        if !(*cur).next.is_null() { cur = (*cur).next; return cur }
        if cur.is_null() { break ; }
    }
    return cur;
}
/* *
 * xmlSchematronRunTest:
 * @ctxt:  the schema validation context
 * @test:  the current test
 * @instance:  the document instace tree
 * @cur:  the current node in the instance
 *
 * Validate a rule against a tree instance at a given position
 *
 * Returns 1 in case of success, 0 if error and -1 in case of internal error
 */
unsafe extern "C" fn xmlSchematronRunTest(mut ctxt: xmlSchematronValidCtxtPtr,
                                          mut test: xmlSchematronTestPtr,
                                          mut instance: xmlDocPtr,
                                          mut cur: xmlNodePtr,
                                          mut pattern:
                                              xmlSchematronPatternPtr)
 -> std::os::raw::c_int {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut failed: std::os::raw::c_int = 0;
    failed = 0 as std::os::raw::c_int;
    (*(*ctxt).xctxt).doc = instance;
    (*(*ctxt).xctxt).node = cur;
    ret = xmlXPathCompiledEval((*test).comp, (*ctxt).xctxt);
    if ret.is_null() {
        failed = 1 as std::os::raw::c_int
    } else {
        match (*ret).type_0 as std::os::raw::c_uint {
            9 | 1 => {
                if (*ret).nodesetval.is_null() ||
                       (*(*ret).nodesetval).nodeNr == 0 as std::os::raw::c_int {
                    failed = 1 as std::os::raw::c_int
                }
            }
            2 => { failed = ((*ret).boolval == 0) as std::os::raw::c_int }
            3 => {
                if xmlXPathIsNaN((*ret).floatval) != 0 ||
                       (*ret).floatval == 0.0f64 {
                    failed = 1 as std::os::raw::c_int
                }
            }
            4 => {
                if (*ret).stringval.is_null() ||
                       *(*ret).stringval.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int == 0 as std::os::raw::c_int {
                    failed = 1 as std::os::raw::c_int
                }
            }
            0 | 5 | 6 | 7 | 8 => { failed = 1 as std::os::raw::c_int }
            _ => { }
        }
        xmlXPathFreeObject(ret);
    }
    if failed != 0 &&
           (*test).type_0 as std::os::raw::c_uint ==
               XML_SCHEMATRON_ASSERT as std::os::raw::c_int as std::os::raw::c_uint {
        (*ctxt).nberrors += 1
    } else if failed == 0 &&
                  (*test).type_0 as std::os::raw::c_uint ==
                      XML_SCHEMATRON_REPORT as std::os::raw::c_int as std::os::raw::c_uint {
        (*ctxt).nberrors += 1
    }
    xmlSchematronReportSuccess(ctxt, test, cur, pattern,
                               (failed == 0) as std::os::raw::c_int);
    return (failed == 0) as std::os::raw::c_int;
}
/* *
 * xmlSchematronValidateDoc:
 * @ctxt:  the schema validation context
 * @instance:  the document instace tree
 *
 * Validate a tree instance against the schematron
 *
 * Returns 0 in case of success, -1 in case of internal error
 *         and an error count otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchematronValidateDoc(mut ctxt:
                                                      xmlSchematronValidCtxtPtr,
                                                  mut instance: xmlDocPtr)
 -> std::os::raw::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut pattern: xmlSchematronPatternPtr = 0 as *mut xmlSchematronPattern;
    let mut rule: xmlSchematronRulePtr = 0 as *mut xmlSchematronRule;
    let mut test: xmlSchematronTestPtr = 0 as *mut xmlSchematronTest;
    if ctxt.is_null() || (*ctxt).schema.is_null() ||
           (*(*ctxt).schema).rules.is_null() || instance.is_null() {
        return -(1 as std::os::raw::c_int)
    }
    (*ctxt).nberrors = 0 as std::os::raw::c_int;
    root = xmlDocGetRootElement(instance as *const xmlDoc);
    if root.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   b"schematron.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char,
                                                                   1679 as
                                                                       std::os::raw::c_int);
        (*ctxt).nberrors += 1;
        return 1 as std::os::raw::c_int
    }
    if (*ctxt).flags & XML_SCHEMATRON_OUT_QUIET as std::os::raw::c_int != 0 ||
           (*ctxt).flags == 0 as std::os::raw::c_int {
        /*
	 * we are just trying to assert the validity of the document,
	 * speed primes over the output, run in a single pass
	 */
        cur = root;
        while !cur.is_null() {
            rule = (*(*ctxt).schema).rules;
            while !rule.is_null() {
                if xmlPatternMatch((*rule).pattern, cur) == 1 as std::os::raw::c_int {
                    test = (*rule).tests;
                    while !test.is_null() {
                        xmlSchematronRunTest(ctxt, test, instance, cur,
                                             (*rule).pattern as
                                                 xmlSchematronPatternPtr);
                        test = (*test).next
                    }
                }
                rule = (*rule).next
            }
            cur = xmlSchematronNextNode(cur)
        }
    } else {
        /*
	 * Process all contexts one at a time
	 */
        pattern = (*(*ctxt).schema).patterns;
        while !pattern.is_null() {
            xmlSchematronReportPattern(ctxt, pattern);
            /*
	     * TODO convert the pattern rule to a direct XPath and
	     * compute directly instead of using the pattern matching
	     * over the full document...
	     * Check the exact semantic
	     */
            cur = root;
            while !cur.is_null() {
                rule = (*pattern).rules;
                while !rule.is_null() {
                    if xmlPatternMatch((*rule).pattern, cur) ==
                           1 as std::os::raw::c_int {
                        test = (*rule).tests;
                        while !test.is_null() {
                            xmlSchematronRunTest(ctxt, test, instance, cur,
                                                 pattern);
                            test = (*test).next
                        }
                    }
                    rule = (*rule).patnext
                }
                cur = xmlSchematronNextNode(cur)
            }
            pattern = (*pattern).next
        }
    }
    return (*ctxt).nberrors;
}
/* LIBXML_SCHEMATRON_ENABLED */
/* __INCLUDE_ELFGCCHACK */
