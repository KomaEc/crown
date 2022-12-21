
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
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlUTF8Strlen(utf: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrndup(cur: *const xmlChar, len: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memmove(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlRegexpExec(comp: xmlRegexpPtr, value: *const xmlChar)
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
    fn xmlValidateQName(value: *const xmlChar, space: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateName(value: *const xmlChar, space: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlValidateNMToken(value: *const xmlChar, space: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar)
     -> *mut xmlChar;
    /*
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr,
                   nameSpace: *const xmlChar) -> xmlNsPtr;
    /*
 * Constructor and destructor.
 */
    #[no_mangle]
    fn xmlHashCreate(size: std::os::raw::c_int) -> xmlHashTablePtr;
    #[no_mangle]
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    #[no_mangle]
    fn xmlHashAddEntry2(table: xmlHashTablePtr, name: *const xmlChar,
                        name2: *const xmlChar, userdata: *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlHashLookup2(table: xmlHashTablePtr, name: *const xmlChar,
                      name2: *const xmlChar) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn __xmlSimpleError(domain: std::os::raw::c_int, code: std::os::raw::c_int,
                        node: xmlNodePtr, msg: *const std::os::raw::c_char,
                        extra: *const std::os::raw::c_char);
    /* LIBXML_OUTPUT_ENABLED */
    /* IDs */
    #[no_mangle]
    fn xmlAddID(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, value: *const xmlChar,
                attr: xmlAttrPtr) -> xmlIDPtr;
    /* IDREFs */
    #[no_mangle]
    fn xmlAddRef(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, value: *const xmlChar,
                 attr: xmlAttrPtr) -> xmlRefPtr;
    /* LIBXML_VALID_ENABLED */
    #[no_mangle]
    fn xmlValidateNotationUse(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr,
                              notationName: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn labs(_: std::os::raw::c_long) -> std::os::raw::c_long;
    #[no_mangle]
    static mut xmlMalloc: xmlMallocFunc;
    #[no_mangle]
    static mut xmlMallocAtomic: xmlMallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar)
     -> xmlEntityPtr;
    #[no_mangle]
    fn xmlCheckLanguageID(lang: *const xmlChar) -> std::os::raw::c_int;
    /* used to limit Pop on the stack */
    /* ***********************************************************************
 *									*
 *			Public API					*
 *									*
 ************************************************************************/
    /* *
 * Objects and Nodesets handling
 */
    #[no_mangle]
    static mut xmlXPathNAN: std::os::raw::c_double;
    #[no_mangle]
    static mut xmlXPathPINF: std::os::raw::c_double;
    #[no_mangle]
    static mut xmlXPathNINF: std::os::raw::c_double;
    #[no_mangle]
    fn xmlXPathIsNaN(val: std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlParseURI(str: *const std::os::raw::c_char) -> xmlURIPtr;
    #[no_mangle]
    fn xmlFreeURI(uri: xmlURIPtr);
    /* Obsolete */
    #[no_mangle]
    fn xmlSchemaFreeType(type_0: xmlSchemaTypePtr);
    #[no_mangle]
    fn xmlSchemaFreeWildcard(wildcard: xmlSchemaWildcardPtr);
    #[no_mangle]
    fn xmlSchemaNewFacet() -> xmlSchemaFacetPtr;
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn floor(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type xmlMallocFunc
    =
    Option<unsafe extern "C" fn(_: size_t) -> *mut std::os::raw::c_void>;
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
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
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
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
pub type xmlNsPtr = *mut xmlNs;
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
pub type xmlSchemaValType = std::os::raw::c_uint;
pub const XML_SCHEMAS_ANYSIMPLETYPE: xmlSchemaValType = 46;
pub const XML_SCHEMAS_ANYTYPE: xmlSchemaValType = 45;
pub const XML_SCHEMAS_BASE64BINARY: xmlSchemaValType = 44;
pub const XML_SCHEMAS_HEXBINARY: xmlSchemaValType = 43;
pub const XML_SCHEMAS_UBYTE: xmlSchemaValType = 42;
pub const XML_SCHEMAS_BYTE: xmlSchemaValType = 41;
pub const XML_SCHEMAS_USHORT: xmlSchemaValType = 40;
pub const XML_SCHEMAS_SHORT: xmlSchemaValType = 39;
pub const XML_SCHEMAS_ULONG: xmlSchemaValType = 38;
pub const XML_SCHEMAS_LONG: xmlSchemaValType = 37;
pub const XML_SCHEMAS_UINT: xmlSchemaValType = 36;
pub const XML_SCHEMAS_INT: xmlSchemaValType = 35;
pub const XML_SCHEMAS_PINTEGER: xmlSchemaValType = 34;
pub const XML_SCHEMAS_NNINTEGER: xmlSchemaValType = 33;
pub const XML_SCHEMAS_NINTEGER: xmlSchemaValType = 32;
pub const XML_SCHEMAS_NPINTEGER: xmlSchemaValType = 31;
pub const XML_SCHEMAS_INTEGER: xmlSchemaValType = 30;
pub const XML_SCHEMAS_ANYURI: xmlSchemaValType = 29;
pub const XML_SCHEMAS_NOTATION: xmlSchemaValType = 28;
pub const XML_SCHEMAS_ENTITIES: xmlSchemaValType = 27;
pub const XML_SCHEMAS_ENTITY: xmlSchemaValType = 26;
pub const XML_SCHEMAS_IDREFS: xmlSchemaValType = 25;
pub const XML_SCHEMAS_IDREF: xmlSchemaValType = 24;
pub const XML_SCHEMAS_ID: xmlSchemaValType = 23;
pub const XML_SCHEMAS_NCNAME: xmlSchemaValType = 22;
pub const XML_SCHEMAS_QNAME: xmlSchemaValType = 21;
pub const XML_SCHEMAS_NAME: xmlSchemaValType = 20;
pub const XML_SCHEMAS_NMTOKENS: xmlSchemaValType = 19;
pub const XML_SCHEMAS_NMTOKEN: xmlSchemaValType = 18;
pub const XML_SCHEMAS_LANGUAGE: xmlSchemaValType = 17;
pub const XML_SCHEMAS_TOKEN: xmlSchemaValType = 16;
pub const XML_SCHEMAS_BOOLEAN: xmlSchemaValType = 15;
pub const XML_SCHEMAS_DOUBLE: xmlSchemaValType = 14;
pub const XML_SCHEMAS_FLOAT: xmlSchemaValType = 13;
pub const XML_SCHEMAS_DURATION: xmlSchemaValType = 12;
pub const XML_SCHEMAS_DATETIME: xmlSchemaValType = 11;
pub const XML_SCHEMAS_DATE: xmlSchemaValType = 10;
pub const XML_SCHEMAS_GYEARMONTH: xmlSchemaValType = 9;
pub const XML_SCHEMAS_GYEAR: xmlSchemaValType = 8;
pub const XML_SCHEMAS_GMONTHDAY: xmlSchemaValType = 7;
pub const XML_SCHEMAS_GMONTH: xmlSchemaValType = 6;
pub const XML_SCHEMAS_GDAY: xmlSchemaValType = 5;
pub const XML_SCHEMAS_TIME: xmlSchemaValType = 4;
pub const XML_SCHEMAS_DECIMAL: xmlSchemaValType = 3;
pub const XML_SCHEMAS_NORMSTRING: xmlSchemaValType = 2;
pub const XML_SCHEMAS_STRING: xmlSchemaValType = 1;
pub const XML_SCHEMAS_UNKNOWN: xmlSchemaValType = 0;
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
pub type xmlSchemaContentType = std::os::raw::c_uint;
pub const XML_SCHEMA_CONTENT_ANY: xmlSchemaContentType = 7;
pub const XML_SCHEMA_CONTENT_BASIC: xmlSchemaContentType = 6;
pub const XML_SCHEMA_CONTENT_MIXED_OR_ELEMENTS: xmlSchemaContentType = 5;
pub const XML_SCHEMA_CONTENT_SIMPLE: xmlSchemaContentType = 4;
pub const XML_SCHEMA_CONTENT_MIXED: xmlSchemaContentType = 3;
pub const XML_SCHEMA_CONTENT_ELEMENTS: xmlSchemaContentType = 2;
pub const XML_SCHEMA_CONTENT_EMPTY: xmlSchemaContentType = 1;
pub const XML_SCHEMA_CONTENT_UNKNOWN: xmlSchemaContentType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaVal {
    pub type_0: xmlSchemaValType,
    pub next: *mut _xmlSchemaVal,
    pub value: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub decimal: xmlSchemaValDecimal,
    pub date: xmlSchemaValDate,
    pub dur: xmlSchemaValDuration,
    pub qname: xmlSchemaValQName,
    pub hex: xmlSchemaValHex,
    pub base64: xmlSchemaValBase64,
    pub f: std::os::raw::c_float,
    pub d: std::os::raw::c_double,
    pub b: std::os::raw::c_int,
    pub str_0: *mut xmlChar,
}
pub type xmlSchemaValBase64 = _xmlSchemaValBase64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValBase64 {
    pub str_0: *mut xmlChar,
    pub total: std::os::raw::c_uint,
}
pub type xmlSchemaValHex = _xmlSchemaValHex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValHex {
    pub str_0: *mut xmlChar,
    pub total: std::os::raw::c_uint,
}
pub type xmlSchemaValQName = _xmlSchemaValQName;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValQName {
    pub name: *mut xmlChar,
    pub uri: *mut xmlChar,
}
/* -1440 <= tzo <= 1440;
					   currently only -840 to +840 are needed */
/* Duration value */
pub type xmlSchemaValDuration = _xmlSchemaValDuration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValDuration {
    pub mon: std::os::raw::c_long,
    pub day: std::os::raw::c_long,
    pub sec: std::os::raw::c_double,
}
/* Date value */
pub type xmlSchemaValDate = _xmlSchemaValDate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValDate {
    pub year: std::os::raw::c_long,
    pub mon_day_hour_min: [u8; 3],
    pub c2rust_padding: [u8; 5],
    pub sec: std::os::raw::c_double,
    pub tz_flag_tzo: [u8; 2],
    pub c2rust_padding_0: [u8; 6],
}

#[inline(always)]
fn get_bits(bits: &[u8], range: std::ops::RangeInclusive<usize>) -> std::os::raw::c_uint {
    let mut result = 0;
    for i in range {
        result = result | (((1 << i) * ((bits[i / 8] >> (i % 8))) & 1)) as std::os::raw::c_uint;
    }
    result
}

#[inline(always)]
fn set_bits(bits: &mut [u8], value: std::os::raw::c_uint, range: std::ops::RangeInclusive<usize>) {
    let mut j = 0;
    for i in range {
        let bit = ((value >> j) & 1);
        if bit == 1 {
            bits[i / 8] = bits[i / 8] | bit as u8;
        } else {
            bits[i / 8] = bits[i / 8] & (255 - (1 << (i % 8)));
        }
        j = j + 1;
    }
}

impl _xmlSchemaValDate {
    pub fn mon(&self) -> std::os::raw::c_uint { get_bits(&self.mon_day_hour_min, 0..=3) as std::os::raw::c_uint }
    pub fn day(&self) -> std::os::raw::c_uint { get_bits(&self.mon_day_hour_min, 4..=8) as std::os::raw::c_uint }
    pub fn hour(&self) -> std::os::raw::c_uint { get_bits(&self.mon_day_hour_min, 9..=13) as std::os::raw::c_uint }
    pub fn min(&self) -> std::os::raw::c_uint { get_bits(&self.mon_day_hour_min, 14..=19) as std::os::raw::c_uint }

    pub fn set_mon(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.mon_day_hour_min, x, 0..=3) }
    pub fn set_day(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.mon_day_hour_min, x, 4..=8) }
    pub fn set_hour(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.mon_day_hour_min, x, 9..=13) }
    pub fn set_min(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.mon_day_hour_min, x, 14..=19) }


    pub fn tz_flag(&self) -> std::os::raw::c_uint { get_bits(&self.tz_flag_tzo, 0..=0) as std::os::raw::c_uint }
    pub fn tzo(&self) -> std::os::raw::c_int { get_bits(&self.tz_flag_tzo, 1..=12) as std::os::raw::c_int }

    pub fn set_tz_flag(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.tz_flag_tzo, x, 0..=0) }
    pub fn set_tzo(&mut self,     x: std::os::raw::c_int ) { set_bits(&mut self.tz_flag_tzo, x as std::os::raw::c_uint, 1..=12) }
}

/* sec stores min and hour also */
pub type xmlSchemaValDecimal = _xmlSchemaValDecimal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValDecimal {
    pub lo: std::os::raw::c_ulong,
    pub mi: std::os::raw::c_ulong,
    pub hi: std::os::raw::c_ulong,
    pub extra: std::os::raw::c_uint,
    pub sign_frac_total: [u8; 2],
    pub c2rust_padding: [u8; 2],
}

impl _xmlSchemaValDecimal {
    pub fn sign(&self) -> std::os::raw::c_uint { get_bits(&self.sign_frac_total, 0..=0) }
    pub fn frac(&self) -> std::os::raw::c_uint { get_bits(&self.sign_frac_total, 1..=7) }
    pub fn total(&self) -> std::os::raw::c_uint { get_bits(&self.sign_frac_total, 8..=15) }

    pub fn set_sign(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.sign_frac_total, x, 0..=0) }
    pub fn set_frac(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.sign_frac_total, x, 1..=7) }
    pub fn set_total(&mut self, x: std::os::raw::c_uint) { set_bits(&mut self.sign_frac_total, x, 8..=15) }
}

pub type xmlSchemaVal = _xmlSchemaVal;
pub type xmlSchemaValPtr = *mut xmlSchemaVal;
/* *
 * XML_SCHEMAS_TYPE_NORMVALUENEEDED:
 *
 * indicates if the facets (pattern) need a normalized value
 */
/* *
 * XML_SCHEMAS_TYPE_FIXUP_1:
 *
 * First stage of fixup was done.
 */
/* *
 * XML_SCHEMAS_TYPE_REDEFINED:
 *
 * The type was redefined.
 */
/* *
 * XML_SCHEMAS_TYPE_REDEFINING:
 *
 * The type redefines an other type.
 */
/* #define XML_SCHEMAS_TYPE_REDEFINING    1 << 31 */
/* *
 * _xmlSchemaType:
 *
 * Schemas type definition.
 */
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
pub type xmlSchemaTypePtr = *mut xmlSchemaType;
pub type xmlSchemaType = _xmlSchemaType;
pub type xmlSchemaFacetLinkPtr = *mut xmlSchemaFacetLink;
/* the linked type */
/* *
 * xmlSchemaFacetLink:
 * Used to build a list of facets.
 */
pub type xmlSchemaFacetLink = _xmlSchemaFacetLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacetLink {
    pub next: *mut _xmlSchemaFacetLink,
    pub facet: xmlSchemaFacetPtr,
}
pub type xmlSchemaFacetPtr = *mut xmlSchemaFacet;
pub type xmlSchemaFacet = _xmlSchemaFacet;
/* The identity-constraint defs */
/*
 * XML_SCHEMAS_FACET_UNKNOWN:
 *
 * unknown facet handling
 */
/*
 * XML_SCHEMAS_FACET_PRESERVE:
 *
 * preserve the type of the facet
 */
/*
 * XML_SCHEMAS_FACET_REPLACE:
 *
 * replace the type of the facet
 */
/*
 * XML_SCHEMAS_FACET_COLLAPSE:
 *
 * collapse the types of the facet
 */
/* *
 * A facet definition.
 */
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
pub type xmlSchemaTypeLinkPtr = *mut xmlSchemaTypeLink;
/* *
 * xmlSchemaTypeLink:
 * Used to build a list of types (e.g. member types of
 * simpleType with variety "union").
 */
pub type xmlSchemaTypeLink = _xmlSchemaTypeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaTypeLink {
    pub next: *mut _xmlSchemaTypeLink,
    pub type_0: xmlSchemaTypePtr,
}
pub type xmlSchemaWildcardPtr = *mut xmlSchemaWildcard;
/* the value */
/* *
 * xmlSchemaWildcard.
 * A wildcard.
 */
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
/* the linked attribute */
/* *
 * XML_SCHEMAS_WILDCARD_COMPLETE:
 *
 * If the wildcard is complete.
 */
/* *
 * xmlSchemaCharValueLink:
 * Used to build a list of namespaces on wildcards.
 */
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
pub type xmlSchemaWhitespaceValueType = std::os::raw::c_uint;
pub const XML_SCHEMA_WHITESPACE_COLLAPSE: xmlSchemaWhitespaceValueType = 3;
pub const XML_SCHEMA_WHITESPACE_REPLACE: xmlSchemaWhitespaceValueType = 2;
pub const XML_SCHEMA_WHITESPACE_PRESERVE: xmlSchemaWhitespaceValueType = 1;
pub const XML_SCHEMA_WHITESPACE_UNKNOWN: xmlSchemaWhitespaceValueType = 0;
pub type xmlSchemaTreeItemPtr = *mut xmlSchemaTreeItem;
pub type xmlSchemaTreeItem = _xmlSchemaTreeItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaTreeItem {
    pub type_0: xmlSchemaTypeType,
    pub annot: xmlSchemaAnnotPtr,
    pub next: xmlSchemaTreeItemPtr,
    pub children: xmlSchemaTreeItemPtr,
}
pub type xmlSchemaParticlePtr = *mut xmlSchemaParticle;
pub type xmlSchemaParticle = _xmlSchemaParticle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaParticle {
    pub type_0: xmlSchemaTypeType,
    pub annot: xmlSchemaAnnotPtr,
    pub next: xmlSchemaTreeItemPtr,
    pub children: xmlSchemaTreeItemPtr,
    pub minOccurs: std::os::raw::c_int,
    pub maxOccurs: std::os::raw::c_int,
    pub node: xmlNodePtr,
}
pub type xmlSchemaModelGroupPtr = *mut xmlSchemaModelGroup;
pub type xmlSchemaModelGroup = _xmlSchemaModelGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaModelGroup {
    pub type_0: xmlSchemaTypeType,
    pub annot: xmlSchemaAnnotPtr,
    pub next: xmlSchemaTreeItemPtr,
    pub children: xmlSchemaTreeItemPtr,
    pub node: xmlNodePtr,
}
pub type xmlSchemaValDatePtr = *mut xmlSchemaValDate;
pub type xmlSchemaValDurationPtr = *mut xmlSchemaValDuration;
static mut xmlSchemaTypesInitialized: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut xmlSchemaTypesBank: xmlHashTablePtr =
    0 as *const xmlHashTable as xmlHashTablePtr;
/*
 * Basic types
 */
static mut xmlSchemaTypeStringDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeAnyTypeDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeAnySimpleTypeDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeDecimalDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeDatetimeDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeDateDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeTimeDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeGYearDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeGYearMonthDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeGDayDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeGMonthDayDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeGMonthDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeDurationDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeFloatDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeBooleanDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeDoubleDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeHexBinaryDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeBase64BinaryDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeAnyURIDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
/*
 * Derived types
 */
static mut xmlSchemaTypePositiveIntegerDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNonPositiveIntegerDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNegativeIntegerDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNonNegativeIntegerDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeIntegerDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeLongDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeIntDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeShortDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeByteDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeUnsignedLongDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeUnsignedIntDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeUnsignedShortDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeUnsignedByteDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNormStringDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeTokenDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeLanguageDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNameDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeQNameDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNCNameDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeIdDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeIdrefDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeIdrefsDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeEntityDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeEntitiesDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNotationDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNmtokenDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
static mut xmlSchemaTypeNmtokensDef: xmlSchemaTypePtr =
    0 as *const xmlSchemaType as xmlSchemaTypePtr;
/* ***********************************************************************
 *									*
 *			Datatype error handlers				*
 *									*
 ************************************************************************/
/* *
 * xmlSchemaTypeErrMemory:
 * @extra:  extra informations
 *
 * Handle an out of memory condition
 */
unsafe extern "C" fn xmlSchemaTypeErrMemory(mut node: xmlNodePtr,
                                            mut extra: *const std::os::raw::c_char) {
    __xmlSimpleError(XML_FROM_DATATYPE as std::os::raw::c_int,
                     XML_ERR_NO_MEMORY as std::os::raw::c_int, node,
                     0 as *const std::os::raw::c_char, extra);
}
/* ***********************************************************************
 *									*
 *			Base types support				*
 *									*
 ************************************************************************/
/* *
 * xmlSchemaNewValue:
 * @type:  the value type
 *
 * Allocate a new simple type value
 *
 * Returns a pointer to the new value or NULL in case of error
 */
unsafe extern "C" fn xmlSchemaNewValue(mut type_0: xmlSchemaValType)
 -> xmlSchemaValPtr {
    let mut value: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    value =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaVal>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaValPtr;
    if value.is_null() { return 0 as xmlSchemaValPtr }
    memset(value as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaVal>() as std::os::raw::c_ulong);
    (*value).type_0 = type_0;
    return value;
}
unsafe extern "C" fn xmlSchemaNewMinLengthFacet(mut value: std::os::raw::c_int)
 -> xmlSchemaFacetPtr {
    let mut ret: xmlSchemaFacetPtr = 0 as *mut xmlSchemaFacet;
    ret = xmlSchemaNewFacet();
    if ret.is_null() { return 0 as xmlSchemaFacetPtr }
    (*ret).type_0 = XML_SCHEMA_FACET_MINLENGTH;
    (*ret).val = xmlSchemaNewValue(XML_SCHEMAS_NNINTEGER);
    if (*ret).val.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut std::os::raw::c_void);
        return 0 as xmlSchemaFacetPtr
    }
    (*(*ret).val).value.decimal.lo = value as std::os::raw::c_ulong;
    return ret;
}
/*
 * xmlSchemaInitBasicType:
 * @name:  the type name
 * @type:  the value type associated
 *
 * Initialize one primitive built-in type
 */
unsafe extern "C" fn xmlSchemaInitBasicType(mut name: *const std::os::raw::c_char,
                                            mut type_0: xmlSchemaValType,
                                            mut baseType: xmlSchemaTypePtr)
 -> xmlSchemaTypePtr {
    let mut ret: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaType>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaTypePtr;
    if ret.is_null() {
        xmlSchemaTypeErrMemory(0 as xmlNodePtr,
                               b"could not initialize basic types\x00" as
                                   *const u8 as *const std::os::raw::c_char);
        return 0 as xmlSchemaTypePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaType>() as std::os::raw::c_ulong);
    (*ret).name = name as *const xmlChar;
    (*ret).targetNamespace =
        b"http://www.w3.org/2001/XMLSchema\x00" as *const u8 as
            *const std::os::raw::c_char as *const xmlChar;
    (*ret).type_0 = XML_SCHEMA_TYPE_BASIC;
    (*ret).baseType = baseType;
    (*ret).contentType = XML_SCHEMA_CONTENT_BASIC;
    /*
    * Primitive types.
    */
    match type_0 as std::os::raw::c_uint {
        1 | 3 | 10 | 11 | 4 | 8 | 9 | 6 | 7 | 5 | 12 | 13 | 14 | 15 | 29 | 43
        | 44 | 21 | 28 => {
            (*ret).flags |= (1 as std::os::raw::c_int) << 14 as std::os::raw::c_int
        }
        _ => { }
    }
    /*
    * Set variety.
    */
    match type_0 as std::os::raw::c_uint {
        45 | 46 => { }
        25 | 19 | 27 => {
            (*ret).flags |= (1 as std::os::raw::c_int) << 6 as std::os::raw::c_int;
            (*ret).facets = xmlSchemaNewMinLengthFacet(1 as std::os::raw::c_int);
            (*ret).flags |= (1 as std::os::raw::c_int) << 27 as std::os::raw::c_int
        }
        _ => { (*ret).flags |= (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int }
    }
    xmlHashAddEntry2(xmlSchemaTypesBank, (*ret).name,
                     b"http://www.w3.org/2001/XMLSchema\x00" as *const u8 as
                         *const std::os::raw::c_char as *const xmlChar,
                     ret as *mut std::os::raw::c_void);
    (*ret).builtInType = type_0 as std::os::raw::c_int;
    return ret;
}
unsafe extern "C" fn xmlSchemaAddParticle() -> xmlSchemaParticlePtr {
    let mut ret: xmlSchemaParticlePtr = 0 as xmlSchemaParticlePtr;
    ret =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaParticle>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaParticlePtr;
    if ret.is_null() {
        xmlSchemaTypeErrMemory(0 as xmlNodePtr,
                               b"allocating particle component\x00" as
                                   *const u8 as *const std::os::raw::c_char);
        return 0 as xmlSchemaParticlePtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaParticle>() as std::os::raw::c_ulong);
    (*ret).type_0 = XML_SCHEMA_TYPE_PARTICLE;
    (*ret).minOccurs = 1 as std::os::raw::c_int;
    (*ret).maxOccurs = 1 as std::os::raw::c_int;
    return ret;
}
/*
 * xmlSchemaInitTypes:
 *
 * Initialize the default XML Schemas type library
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaInitTypes() {
    if xmlSchemaTypesInitialized != 0 as std::os::raw::c_int { return }
    xmlSchemaTypesBank = xmlHashCreate(40 as std::os::raw::c_int);
    /*
    * 3.4.7 Built-in Complex Type Definition
    */
    xmlSchemaTypeAnyTypeDef =
        xmlSchemaInitBasicType(b"anyType\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_ANYTYPE,
                               0 as xmlSchemaTypePtr);
    (*xmlSchemaTypeAnyTypeDef).baseType = xmlSchemaTypeAnyTypeDef;
    (*xmlSchemaTypeAnyTypeDef).contentType = XML_SCHEMA_CONTENT_MIXED;
    /*
    * Init the content type.
    */
    (*xmlSchemaTypeAnyTypeDef).contentType = XML_SCHEMA_CONTENT_MIXED;
    let mut particle: xmlSchemaParticlePtr = 0 as *mut xmlSchemaParticle;
    let mut sequence: xmlSchemaModelGroupPtr = 0 as *mut xmlSchemaModelGroup;
    let mut wild: xmlSchemaWildcardPtr = 0 as *mut xmlSchemaWildcard;
    /* First particle. */
    particle = xmlSchemaAddParticle();
    if particle.is_null() { return }
    (*xmlSchemaTypeAnyTypeDef).subtypes = particle as xmlSchemaTypePtr;
    /* Sequence model group. */
    sequence =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaModelGroup>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaModelGroupPtr;
    if sequence.is_null() {
        xmlSchemaTypeErrMemory(0 as xmlNodePtr,
                               b"allocating model group component\x00" as
                                   *const u8 as *const std::os::raw::c_char);
        return
    }
    memset(sequence as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaModelGroup>() as std::os::raw::c_ulong);
    (*sequence).type_0 = XML_SCHEMA_TYPE_SEQUENCE;
    (*particle).children = sequence as xmlSchemaTreeItemPtr;
    /* Second particle. */
    particle = xmlSchemaAddParticle();
    if particle.is_null() { return }
    (*particle).minOccurs = 0 as std::os::raw::c_int;
    (*particle).maxOccurs = (1 as std::os::raw::c_int) << 30 as std::os::raw::c_int;
    (*sequence).children = particle as xmlSchemaTreeItemPtr;
    /* The wildcard */
    wild =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaWildcard>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaWildcardPtr;
    if wild.is_null() {
        xmlSchemaTypeErrMemory(0 as xmlNodePtr,
                               b"allocating wildcard component\x00" as
                                   *const u8 as *const std::os::raw::c_char);
        return
    }
    memset(wild as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaWildcard>() as std::os::raw::c_ulong);
    (*wild).type_0 = XML_SCHEMA_TYPE_ANY;
    (*wild).any = 1 as std::os::raw::c_int;
    (*wild).processContents = 2 as std::os::raw::c_int;
    (*particle).children = wild as xmlSchemaTreeItemPtr;
    /*
	* Create the attribute wildcard.
	*/
    wild =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaWildcard>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaWildcardPtr;
    if wild.is_null() {
        xmlSchemaTypeErrMemory(0 as xmlNodePtr,
                               b"could not create an attribute wildcard on anyType\x00"
                                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    memset(wild as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaWildcard>() as std::os::raw::c_ulong);
    (*wild).any = 1 as std::os::raw::c_int;
    (*wild).processContents = 2 as std::os::raw::c_int;
    (*xmlSchemaTypeAnyTypeDef).attributeWildcard = wild;
    xmlSchemaTypeAnySimpleTypeDef =
        xmlSchemaInitBasicType(b"anySimpleType\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               XML_SCHEMAS_ANYSIMPLETYPE,
                               xmlSchemaTypeAnyTypeDef);
    /*
    * primitive datatypes
    */
    xmlSchemaTypeStringDef =
        xmlSchemaInitBasicType(b"string\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_STRING,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeDecimalDef =
        xmlSchemaInitBasicType(b"decimal\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_DECIMAL,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeDateDef =
        xmlSchemaInitBasicType(b"date\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_DATE,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeDatetimeDef =
        xmlSchemaInitBasicType(b"dateTime\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_DATETIME,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeTimeDef =
        xmlSchemaInitBasicType(b"time\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_TIME,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeGYearDef =
        xmlSchemaInitBasicType(b"gYear\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_GYEAR,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeGYearMonthDef =
        xmlSchemaInitBasicType(b"gYearMonth\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               XML_SCHEMAS_GYEARMONTH,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeGMonthDef =
        xmlSchemaInitBasicType(b"gMonth\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_GMONTH,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeGMonthDayDef =
        xmlSchemaInitBasicType(b"gMonthDay\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_GMONTHDAY,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeGDayDef =
        xmlSchemaInitBasicType(b"gDay\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_GDAY,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeDurationDef =
        xmlSchemaInitBasicType(b"duration\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_DURATION,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeFloatDef =
        xmlSchemaInitBasicType(b"float\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_FLOAT,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeDoubleDef =
        xmlSchemaInitBasicType(b"double\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_DOUBLE,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeBooleanDef =
        xmlSchemaInitBasicType(b"boolean\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_BOOLEAN,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeAnyURIDef =
        xmlSchemaInitBasicType(b"anyURI\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_ANYURI,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeHexBinaryDef =
        xmlSchemaInitBasicType(b"hexBinary\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_HEXBINARY,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeBase64BinaryDef =
        xmlSchemaInitBasicType(b"base64Binary\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               XML_SCHEMAS_BASE64BINARY,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeNotationDef =
        xmlSchemaInitBasicType(b"NOTATION\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NOTATION,
                               xmlSchemaTypeAnySimpleTypeDef);
    xmlSchemaTypeQNameDef =
        xmlSchemaInitBasicType(b"QName\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_QNAME,
                               xmlSchemaTypeAnySimpleTypeDef);
    /*
     * derived datatypes
     */
    xmlSchemaTypeIntegerDef =
        xmlSchemaInitBasicType(b"integer\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_INTEGER,
                               xmlSchemaTypeDecimalDef);
    xmlSchemaTypeNonPositiveIntegerDef =
        xmlSchemaInitBasicType(b"nonPositiveInteger\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NPINTEGER,
                               xmlSchemaTypeIntegerDef);
    xmlSchemaTypeNegativeIntegerDef =
        xmlSchemaInitBasicType(b"negativeInteger\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NINTEGER,
                               xmlSchemaTypeNonPositiveIntegerDef);
    xmlSchemaTypeLongDef =
        xmlSchemaInitBasicType(b"long\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_LONG,
                               xmlSchemaTypeIntegerDef);
    xmlSchemaTypeIntDef =
        xmlSchemaInitBasicType(b"int\x00" as *const u8 as *const std::os::raw::c_char,
                               XML_SCHEMAS_INT, xmlSchemaTypeLongDef);
    xmlSchemaTypeShortDef =
        xmlSchemaInitBasicType(b"short\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_SHORT,
                               xmlSchemaTypeIntDef);
    xmlSchemaTypeByteDef =
        xmlSchemaInitBasicType(b"byte\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_BYTE,
                               xmlSchemaTypeShortDef);
    xmlSchemaTypeNonNegativeIntegerDef =
        xmlSchemaInitBasicType(b"nonNegativeInteger\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NNINTEGER,
                               xmlSchemaTypeIntegerDef);
    xmlSchemaTypeUnsignedLongDef =
        xmlSchemaInitBasicType(b"unsignedLong\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_ULONG,
                               xmlSchemaTypeNonNegativeIntegerDef);
    xmlSchemaTypeUnsignedIntDef =
        xmlSchemaInitBasicType(b"unsignedInt\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_UINT,
                               xmlSchemaTypeUnsignedLongDef);
    xmlSchemaTypeUnsignedShortDef =
        xmlSchemaInitBasicType(b"unsignedShort\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_USHORT,
                               xmlSchemaTypeUnsignedIntDef);
    xmlSchemaTypeUnsignedByteDef =
        xmlSchemaInitBasicType(b"unsignedByte\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_UBYTE,
                               xmlSchemaTypeUnsignedShortDef);
    xmlSchemaTypePositiveIntegerDef =
        xmlSchemaInitBasicType(b"positiveInteger\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_PINTEGER,
                               xmlSchemaTypeNonNegativeIntegerDef);
    xmlSchemaTypeNormStringDef =
        xmlSchemaInitBasicType(b"normalizedString\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               XML_SCHEMAS_NORMSTRING,
                               xmlSchemaTypeStringDef);
    xmlSchemaTypeTokenDef =
        xmlSchemaInitBasicType(b"token\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_TOKEN,
                               xmlSchemaTypeNormStringDef);
    xmlSchemaTypeLanguageDef =
        xmlSchemaInitBasicType(b"language\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_LANGUAGE,
                               xmlSchemaTypeTokenDef);
    xmlSchemaTypeNameDef =
        xmlSchemaInitBasicType(b"Name\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NAME,
                               xmlSchemaTypeTokenDef);
    xmlSchemaTypeNmtokenDef =
        xmlSchemaInitBasicType(b"NMTOKEN\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NMTOKEN,
                               xmlSchemaTypeTokenDef);
    xmlSchemaTypeNCNameDef =
        xmlSchemaInitBasicType(b"NCName\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NCNAME,
                               xmlSchemaTypeNameDef);
    xmlSchemaTypeIdDef =
        xmlSchemaInitBasicType(b"ID\x00" as *const u8 as *const std::os::raw::c_char,
                               XML_SCHEMAS_ID, xmlSchemaTypeNCNameDef);
    xmlSchemaTypeIdrefDef =
        xmlSchemaInitBasicType(b"IDREF\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_IDREF,
                               xmlSchemaTypeNCNameDef);
    xmlSchemaTypeEntityDef =
        xmlSchemaInitBasicType(b"ENTITY\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_ENTITY,
                               xmlSchemaTypeNCNameDef);
    /*
    * Derived list types.
    */
    /* ENTITIES */
    xmlSchemaTypeEntitiesDef =
        xmlSchemaInitBasicType(b"ENTITIES\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_ENTITIES,
                               xmlSchemaTypeAnySimpleTypeDef);
    (*xmlSchemaTypeEntitiesDef).subtypes = xmlSchemaTypeEntityDef;
    /* IDREFS */
    xmlSchemaTypeIdrefsDef =
        xmlSchemaInitBasicType(b"IDREFS\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_IDREFS,
                               xmlSchemaTypeAnySimpleTypeDef);
    (*xmlSchemaTypeIdrefsDef).subtypes = xmlSchemaTypeIdrefDef;
    /* NMTOKENS */
    xmlSchemaTypeNmtokensDef =
        xmlSchemaInitBasicType(b"NMTOKENS\x00" as *const u8 as
                                   *const std::os::raw::c_char, XML_SCHEMAS_NMTOKENS,
                               xmlSchemaTypeAnySimpleTypeDef);
    (*xmlSchemaTypeNmtokensDef).subtypes = xmlSchemaTypeNmtokenDef;
    xmlSchemaTypesInitialized = 1 as std::os::raw::c_int;
}
unsafe extern "C" fn xmlSchemaFreeTypeEntry(mut type_0: *mut std::os::raw::c_void,
                                            mut name: *const xmlChar) {
    xmlSchemaFreeType(type_0 as xmlSchemaTypePtr);
}
/* *
 * xmlSchemaCleanupTypes:
 *
 * Cleanup the default XML Schemas type library
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaCleanupTypes() {
    if xmlSchemaTypesInitialized == 0 as std::os::raw::c_int { return }
    /*
    * Free xs:anyType.
    */
    let mut particle: xmlSchemaParticlePtr = 0 as *mut xmlSchemaParticle;
    /* Attribute wildcard. */
    xmlSchemaFreeWildcard((*xmlSchemaTypeAnyTypeDef).attributeWildcard);
    /* Content type. */
    particle = (*xmlSchemaTypeAnyTypeDef).subtypes as xmlSchemaParticlePtr;
    /* Wildcard. */
    xmlSchemaFreeWildcard((*(*(*particle).children).children).children as
                              xmlSchemaWildcardPtr);
    xmlFree.expect("non-null function pointer")((*(*particle).children).children
                                                    as xmlSchemaParticlePtr as
                                                    *mut std::os::raw::c_void);
    /* Sequence model group. */
    xmlFree.expect("non-null function pointer")((*particle).children as
                                                    xmlSchemaModelGroupPtr as
                                                    *mut std::os::raw::c_void);
    xmlFree.expect("non-null function pointer")(particle as
                                                    *mut std::os::raw::c_void);
    (*xmlSchemaTypeAnyTypeDef).subtypes = 0 as xmlSchemaTypePtr;
    xmlHashFree(xmlSchemaTypesBank,
                Some(xmlSchemaFreeTypeEntry as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                              _: *const xmlChar) -> ()));
    xmlSchemaTypesInitialized = 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaIsBuiltInTypeFacet:
 * @type: the built-in type
 * @facetType:  the facet type
 *
 * Evaluates if a specific facet can be
 * used in conjunction with a type.
 *
 * Returns 1 if the facet can be used with the given built-in type,
 * 0 otherwise and -1 in case the type is not a built-in type.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaIsBuiltInTypeFacet(mut type_0:
                                                         xmlSchemaTypePtr,
                                                     mut facetType:
                                                         std::os::raw::c_int)
 -> std::os::raw::c_int {
    if type_0.is_null() { return -(1 as std::os::raw::c_int) }
    if (*type_0).type_0 as std::os::raw::c_uint !=
           XML_SCHEMA_TYPE_BASIC as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    match (*type_0).builtInType {
        15 => {
            if facetType == XML_SCHEMA_FACET_PATTERN as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_WHITESPACE as std::os::raw::c_int {
                return 1 as std::os::raw::c_int
            } else { return 0 as std::os::raw::c_int }
        }
        1 | 28 | 21 | 29 | 44 | 43 => {
            if facetType == XML_SCHEMA_FACET_LENGTH as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_MINLENGTH as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_MAXLENGTH as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_PATTERN as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_ENUMERATION as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_WHITESPACE as std::os::raw::c_int {
                return 1 as std::os::raw::c_int
            } else { return 0 as std::os::raw::c_int }
        }
        3 => {
            if facetType == XML_SCHEMA_FACET_TOTALDIGITS as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_FRACTIONDIGITS as std::os::raw::c_int
                   || facetType == XML_SCHEMA_FACET_PATTERN as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_WHITESPACE as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_ENUMERATION as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_MAXINCLUSIVE as std::os::raw::c_int
                   ||
                   facetType == XML_SCHEMA_FACET_MAXEXCLUSIVE as std::os::raw::c_int
                   ||
                   facetType == XML_SCHEMA_FACET_MININCLUSIVE as std::os::raw::c_int
                   ||
                   facetType == XML_SCHEMA_FACET_MINEXCLUSIVE as std::os::raw::c_int {
                return 1 as std::os::raw::c_int
            } else { return 0 as std::os::raw::c_int }
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 => {
            if facetType == XML_SCHEMA_FACET_PATTERN as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_ENUMERATION as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_WHITESPACE as std::os::raw::c_int ||
                   facetType == XML_SCHEMA_FACET_MAXINCLUSIVE as std::os::raw::c_int
                   ||
                   facetType == XML_SCHEMA_FACET_MAXEXCLUSIVE as std::os::raw::c_int
                   ||
                   facetType == XML_SCHEMA_FACET_MININCLUSIVE as std::os::raw::c_int
                   ||
                   facetType == XML_SCHEMA_FACET_MINEXCLUSIVE as std::os::raw::c_int {
                return 1 as std::os::raw::c_int
            } else { return 0 as std::os::raw::c_int }
        }
        _ => { }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaGetBuiltInType:
 * @type:  the type of the built in type
 *
 * Gives you the type struct for a built-in
 * type by its type id.
 *
 * Returns the type if found, NULL otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetBuiltInType(mut type_0: xmlSchemaValType)
 -> xmlSchemaTypePtr {
    if xmlSchemaTypesInitialized == 0 as std::os::raw::c_int { xmlSchemaInitTypes(); }
    match type_0 as std::os::raw::c_uint {
        46 => { return xmlSchemaTypeAnySimpleTypeDef }
        1 => { return xmlSchemaTypeStringDef }
        2 => { return xmlSchemaTypeNormStringDef }
        3 => { return xmlSchemaTypeDecimalDef }
        4 => { return xmlSchemaTypeTimeDef }
        5 => { return xmlSchemaTypeGDayDef }
        6 => { return xmlSchemaTypeGMonthDef }
        7 => { return xmlSchemaTypeGMonthDayDef }
        8 => { return xmlSchemaTypeGYearDef }
        9 => { return xmlSchemaTypeGYearMonthDef }
        10 => { return xmlSchemaTypeDateDef }
        11 => { return xmlSchemaTypeDatetimeDef }
        12 => { return xmlSchemaTypeDurationDef }
        13 => { return xmlSchemaTypeFloatDef }
        14 => { return xmlSchemaTypeDoubleDef }
        15 => { return xmlSchemaTypeBooleanDef }
        16 => { return xmlSchemaTypeTokenDef }
        17 => { return xmlSchemaTypeLanguageDef }
        18 => { return xmlSchemaTypeNmtokenDef }
        19 => { return xmlSchemaTypeNmtokensDef }
        20 => { return xmlSchemaTypeNameDef }
        21 => { return xmlSchemaTypeQNameDef }
        22 => { return xmlSchemaTypeNCNameDef }
        23 => { return xmlSchemaTypeIdDef }
        24 => { return xmlSchemaTypeIdrefDef }
        25 => { return xmlSchemaTypeIdrefsDef }
        26 => { return xmlSchemaTypeEntityDef }
        27 => { return xmlSchemaTypeEntitiesDef }
        28 => { return xmlSchemaTypeNotationDef }
        29 => { return xmlSchemaTypeAnyURIDef }
        30 => { return xmlSchemaTypeIntegerDef }
        31 => { return xmlSchemaTypeNonPositiveIntegerDef }
        32 => { return xmlSchemaTypeNegativeIntegerDef }
        33 => { return xmlSchemaTypeNonNegativeIntegerDef }
        34 => { return xmlSchemaTypePositiveIntegerDef }
        35 => { return xmlSchemaTypeIntDef }
        36 => { return xmlSchemaTypeUnsignedIntDef }
        37 => { return xmlSchemaTypeLongDef }
        38 => { return xmlSchemaTypeUnsignedLongDef }
        39 => { return xmlSchemaTypeShortDef }
        40 => { return xmlSchemaTypeUnsignedShortDef }
        41 => { return xmlSchemaTypeByteDef }
        42 => { return xmlSchemaTypeUnsignedByteDef }
        43 => { return xmlSchemaTypeHexBinaryDef }
        44 => { return xmlSchemaTypeBase64BinaryDef }
        45 => { return xmlSchemaTypeAnyTypeDef }
        _ => { return 0 as xmlSchemaTypePtr }
    };
}
/* *
 * xmlSchemaValueAppend:
 * @prev: the value
 * @cur: the value to be appended
 *
 * Appends a next sibling to a list of computed values.
 *
 * Returns 0 if succeeded and -1 on API errors.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValueAppend(mut prev: xmlSchemaValPtr,
                                              mut cur: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    if prev.is_null() || cur.is_null() { return -(1 as std::os::raw::c_int) }
    (*prev).next = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaValueGetNext:
 * @cur: the value
 *
 * Accessor for the next sibling of a list of computed values.
 *
 * Returns the next value or NULL if there was none, or on
 *         API errors.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValueGetNext(mut cur: xmlSchemaValPtr)
 -> xmlSchemaValPtr {
    if cur.is_null() { return 0 as xmlSchemaValPtr }
    return (*cur).next;
}
/* *
 * xmlSchemaValueGetAsString:
 * @val: the value
 *
 * Accessor for the string value of a computed value.
 *
 * Returns the string value or NULL if there was none, or on
 *         API errors.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValueGetAsString(mut val: xmlSchemaValPtr)
 -> *const xmlChar {
    if val.is_null() { return 0 as *const xmlChar }
    match (*val).type_0 as std::os::raw::c_uint {
        1 | 2 | 46 | 16 | 17 | 18 | 20 | 22 | 23 | 24 | 26 | 29 => {
            return (*val).value.str_0
        }
        _ => { }
    }
    return 0 as *const xmlChar;
}
/* *
 * xmlSchemaValueGetAsBoolean:
 * @val: the value
 *
 * Accessor for the boolean value of a computed value.
 *
 * Returns 1 if true and 0 if false, or in case of an error. Hmm.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValueGetAsBoolean(mut val: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    if val.is_null() ||
           (*val).type_0 as std::os::raw::c_uint !=
               XML_SCHEMAS_BOOLEAN as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    return (*val).value.b;
}
/* *
 * xmlSchemaNewStringValue:
 * @type:  the value type
 * @value:  the value
 *
 * Allocate a new simple type value. The type can be
 * of XML_SCHEMAS_STRING.
 * WARNING: This one is intended to be expanded for other
 * string based types. We need this for anySimpleType as well.
 * The given value is consumed and freed with the struct.
 *
 * Returns a pointer to the new value or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewStringValue(mut type_0: xmlSchemaValType,
                                                 mut value: *const xmlChar)
 -> xmlSchemaValPtr {
    let mut val: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    if type_0 as std::os::raw::c_uint !=
           XML_SCHEMAS_STRING as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlSchemaValPtr
    }
    val =
        xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlSchemaVal>()
                                                          as std::os::raw::c_ulong) as
            xmlSchemaValPtr;
    if val.is_null() { return 0 as xmlSchemaValPtr }
    memset(val as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<xmlSchemaVal>() as std::os::raw::c_ulong);
    (*val).type_0 = type_0;
    (*val).value.str_0 = value as *mut xmlChar;
    return val;
}
/* *
 * xmlSchemaNewNOTATIONValue:
 * @name:  the notation name
 * @ns: the notation namespace name or NULL
 *
 * Allocate a new NOTATION value.
 * The given values are consumed and freed with the struct.
 *
 * Returns a pointer to the new value or NULL in case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewNOTATIONValue(mut name: *const xmlChar,
                                                   mut ns: *const xmlChar)
 -> xmlSchemaValPtr {
    let mut val: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    val = xmlSchemaNewValue(XML_SCHEMAS_NOTATION);
    if val.is_null() { return 0 as xmlSchemaValPtr }
    (*val).value.qname.name = name as *mut xmlChar;
    if !ns.is_null() { (*val).value.qname.uri = ns as *mut xmlChar }
    return val;
}
/* *
 * xmlSchemaNewQNameValue:
 * @namespaceName: the namespace name
 * @localName: the local name
 *
 * Allocate a new QName value.
 * The given values are consumed and freed with the struct.
 *
 * Returns a pointer to the new value or NULL in case of an error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaNewQNameValue(mut namespaceName:
                                                    *const xmlChar,
                                                mut localName: *const xmlChar)
 -> xmlSchemaValPtr {
    let mut val: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    val = xmlSchemaNewValue(XML_SCHEMAS_QNAME);
    if val.is_null() { return 0 as xmlSchemaValPtr }
    (*val).value.qname.name = localName as *mut xmlChar;
    (*val).value.qname.uri = namespaceName as *mut xmlChar;
    return val;
}
/* *
 * xmlSchemaFreeValue:
 * @value:  the value to free
 *
 * Cleanup the default XML Schemas type library
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaFreeValue(mut value: xmlSchemaValPtr) {
    let mut prev: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    while !value.is_null() {
        match (*value).type_0 as std::os::raw::c_uint {
            1 | 2 | 16 | 17 | 18 | 19 | 20 | 22 | 23 | 24 | 25 | 26 | 27 | 29
            | 46 => {
                if !(*value).value.str_0.is_null() {
                    xmlFree.expect("non-null function pointer")((*value).value.str_0
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            28 | 21 => {
                if !(*value).value.qname.uri.is_null() {
                    xmlFree.expect("non-null function pointer")((*value).value.qname.uri
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
                if !(*value).value.qname.name.is_null() {
                    xmlFree.expect("non-null function pointer")((*value).value.qname.name
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            43 => {
                if !(*value).value.hex.str_0.is_null() {
                    xmlFree.expect("non-null function pointer")((*value).value.hex.str_0
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            44 => {
                if !(*value).value.base64.str_0.is_null() {
                    xmlFree.expect("non-null function pointer")((*value).value.base64.str_0
                                                                    as
                                                                    *mut std::os::raw::c_void);
                }
            }
            _ => { }
        }
        prev = value;
        value = (*value).next;
        xmlFree.expect("non-null function pointer")(prev as
                                                        *mut std::os::raw::c_void);
    };
}
/* *
 * xmlSchemaGetPredefinedType:
 * @name: the type name
 * @ns:  the URI of the namespace usually "http://www.w3.org/2001/XMLSchema"
 *
 * Lookup a type in the default XML Schemas type library
 *
 * Returns the type if found, NULL otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetPredefinedType(mut name: *const xmlChar,
                                                    mut ns: *const xmlChar)
 -> xmlSchemaTypePtr {
    if xmlSchemaTypesInitialized == 0 as std::os::raw::c_int { xmlSchemaInitTypes(); }
    if name.is_null() { return 0 as xmlSchemaTypePtr }
    return xmlHashLookup2(xmlSchemaTypesBank, name, ns) as xmlSchemaTypePtr;
}
/* *
 * xmlSchemaGetBuiltInListSimpleTypeItemType:
 * @type: the built-in simple type.
 *
 * Lookup function
 *
 * Returns the item type of @type as defined by the built-in datatype
 * hierarchy of XML Schema Part 2: Datatypes, or NULL in case of an error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetBuiltInListSimpleTypeItemType(mut type_0:
                                                                       xmlSchemaTypePtr)
 -> xmlSchemaTypePtr {
    if type_0.is_null() ||
           (*type_0).type_0 as std::os::raw::c_uint !=
               XML_SCHEMA_TYPE_BASIC as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as xmlSchemaTypePtr
    }
    match (*type_0).builtInType {
        19 => { return xmlSchemaTypeNmtokenDef }
        25 => { return xmlSchemaTypeIdrefDef }
        27 => { return xmlSchemaTypeEntityDef }
        _ => { return 0 as xmlSchemaTypePtr }
    };
}
static mut daysInMonth: [std::os::raw::c_uint; 12] =
    [31 as std::os::raw::c_int as std::os::raw::c_uint, 28 as std::os::raw::c_int as std::os::raw::c_uint,
     31 as std::os::raw::c_int as std::os::raw::c_uint, 30 as std::os::raw::c_int as std::os::raw::c_uint,
     31 as std::os::raw::c_int as std::os::raw::c_uint, 30 as std::os::raw::c_int as std::os::raw::c_uint,
     31 as std::os::raw::c_int as std::os::raw::c_uint, 31 as std::os::raw::c_int as std::os::raw::c_uint,
     30 as std::os::raw::c_int as std::os::raw::c_uint, 31 as std::os::raw::c_int as std::os::raw::c_uint,
     30 as std::os::raw::c_int as std::os::raw::c_uint, 31 as std::os::raw::c_int as std::os::raw::c_uint];
static mut daysInMonthLeap: [std::os::raw::c_uint; 12] =
    [31 as std::os::raw::c_int as std::os::raw::c_uint, 29 as std::os::raw::c_int as std::os::raw::c_uint,
     31 as std::os::raw::c_int as std::os::raw::c_uint, 30 as std::os::raw::c_int as std::os::raw::c_uint,
     31 as std::os::raw::c_int as std::os::raw::c_uint, 30 as std::os::raw::c_int as std::os::raw::c_uint,
     31 as std::os::raw::c_int as std::os::raw::c_uint, 31 as std::os::raw::c_int as std::os::raw::c_uint,
     30 as std::os::raw::c_int as std::os::raw::c_uint, 31 as std::os::raw::c_int as std::os::raw::c_uint,
     30 as std::os::raw::c_int as std::os::raw::c_uint, 31 as std::os::raw::c_int as std::os::raw::c_uint];
static mut dayInYearByMonth: [std::os::raw::c_long; 12] =
    [0 as std::os::raw::c_int as std::os::raw::c_long, 31 as std::os::raw::c_int as std::os::raw::c_long,
     59 as std::os::raw::c_int as std::os::raw::c_long, 90 as std::os::raw::c_int as std::os::raw::c_long,
     120 as std::os::raw::c_int as std::os::raw::c_long, 151 as std::os::raw::c_int as std::os::raw::c_long,
     181 as std::os::raw::c_int as std::os::raw::c_long, 212 as std::os::raw::c_int as std::os::raw::c_long,
     243 as std::os::raw::c_int as std::os::raw::c_long, 273 as std::os::raw::c_int as std::os::raw::c_long,
     304 as std::os::raw::c_int as std::os::raw::c_long, 334 as std::os::raw::c_int as std::os::raw::c_long];
static mut dayInLeapYearByMonth: [std::os::raw::c_long; 12] =
    [0 as std::os::raw::c_int as std::os::raw::c_long, 31 as std::os::raw::c_int as std::os::raw::c_long,
     60 as std::os::raw::c_int as std::os::raw::c_long, 91 as std::os::raw::c_int as std::os::raw::c_long,
     121 as std::os::raw::c_int as std::os::raw::c_long, 152 as std::os::raw::c_int as std::os::raw::c_long,
     182 as std::os::raw::c_int as std::os::raw::c_long, 213 as std::os::raw::c_int as std::os::raw::c_long,
     244 as std::os::raw::c_int as std::os::raw::c_long, 274 as std::os::raw::c_int as std::os::raw::c_long,
     305 as std::os::raw::c_int as std::os::raw::c_long, 335 as std::os::raw::c_int as std::os::raw::c_long];
/* *
 * _xmlSchemaParseGYear:
 * @dt:  pointer to a date structure
 * @str: pointer to the string to analyze
 *
 * Parses a xs:gYear without time zone and fills in the appropriate
 * field of the @dt structure. @str is updated to point just after the
 * xs:gYear. It is supposed that @dt->year is big enough to contain
 * the year.
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn _xmlSchemaParseGYear(mut dt: xmlSchemaValDatePtr,
                                          mut str: *mut *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = *str;
    let mut firstChar: *const xmlChar = 0 as *const xmlChar;
    let mut isneg: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut digcnt: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if ((*cur as std::os::raw::c_int) < '0' as i32 ||
            *cur as std::os::raw::c_int > '9' as i32) &&
           *cur as std::os::raw::c_int != '-' as i32 &&
           *cur as std::os::raw::c_int != '+' as i32 {
        return -(1 as std::os::raw::c_int)
    }
    if *cur as std::os::raw::c_int == '-' as i32 {
        isneg = 1 as std::os::raw::c_int;
        cur = cur.offset(1)
    }
    firstChar = cur;
    while *cur as std::os::raw::c_int >= '0' as i32 &&
              *cur as std::os::raw::c_int <= '9' as i32 {
        (*dt).year =
            (*dt).year * 10 as std::os::raw::c_int as std::os::raw::c_long +
                (*cur as std::os::raw::c_int - '0' as i32) as std::os::raw::c_long;
        cur = cur.offset(1);
        digcnt += 1
    }
    /* year must be at least 4 digits (CCYY); over 4
     * digits cannot have a leading zero. */
    if digcnt < 4 as std::os::raw::c_int ||
           digcnt > 4 as std::os::raw::c_int &&
               *firstChar as std::os::raw::c_int == '0' as i32 {
        return 1 as std::os::raw::c_int
    }
    if isneg != 0 { (*dt).year = -(*dt).year }
    if !((*dt).year != 0 as std::os::raw::c_int as std::os::raw::c_long) {
        return 2 as std::os::raw::c_int
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * PARSE_2_DIGITS:
 * @num:  the integer to fill in
 * @cur:  an #xmlChar *
 * @invalid: an integer
 *
 * Parses a 2-digits integer and updates @num with the value. @cur is
 * updated to point just after the integer.
 * In case of error, @invalid is set to %TRUE, values of @num and
 * @cur are undefined.
 */
/* *
 * PARSE_FLOAT:
 * @num:  the double to fill in
 * @cur:  an #xmlChar *
 * @invalid: an integer
 *
 * Parses a float and updates @num with the value. @cur is
 * updated to point just after the float. The float must have a
 * 2-digits integer part and may or may not have a decimal part.
 * In case of error, @invalid is set to %TRUE, values of @num and
 * @cur are undefined.
 */
/* *
 * _xmlSchemaParseGMonth:
 * @dt:  pointer to a date structure
 * @str: pointer to the string to analyze
 *
 * Parses a xs:gMonth without time zone and fills in the appropriate
 * field of the @dt structure. @str is updated to point just after the
 * xs:gMonth.
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn _xmlSchemaParseGMonth(mut dt: xmlSchemaValDatePtr,
                                           mut str: *mut *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = *str;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut value: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) < '0' as i32 ||
           *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
           ||
           (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '0' as i32 ||
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
       {
        ret = 1 as std::os::raw::c_int
    } else {
        value =
            ((*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                  '0' as i32) * 10 as std::os::raw::c_int +
                 (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                      '0' as i32)) as std::os::raw::c_uint
    }
    cur = cur.offset(2 as std::os::raw::c_int as isize);
    if ret != 0 as std::os::raw::c_int { return ret }
    if !(value >= 1 as std::os::raw::c_int as std::os::raw::c_uint &&
             value <= 12 as std::os::raw::c_int as std::os::raw::c_uint) {
        return 2 as std::os::raw::c_int
    }
    (*dt).set_mon(value);
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * _xmlSchemaParseGDay:
 * @dt:  pointer to a date structure
 * @str: pointer to the string to analyze
 *
 * Parses a xs:gDay without time zone and fills in the appropriate
 * field of the @dt structure. @str is updated to point just after the
 * xs:gDay.
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn _xmlSchemaParseGDay(mut dt: xmlSchemaValDatePtr,
                                         mut str: *mut *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = *str;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut value: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) < '0' as i32 ||
           *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
           ||
           (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '0' as i32 ||
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
       {
        ret = 1 as std::os::raw::c_int
    } else {
        value =
            ((*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                  '0' as i32) * 10 as std::os::raw::c_int +
                 (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                      '0' as i32)) as std::os::raw::c_uint
    }
    cur = cur.offset(2 as std::os::raw::c_int as isize);
    if ret != 0 as std::os::raw::c_int { return ret }
    if !(value >= 1 as std::os::raw::c_int as std::os::raw::c_uint &&
             value <= 31 as std::os::raw::c_int as std::os::raw::c_uint) {
        return 2 as std::os::raw::c_int
    }
    (*dt).set_day(value);
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * _xmlSchemaParseTime:
 * @dt:  pointer to a date structure
 * @str: pointer to the string to analyze
 *
 * Parses a xs:time without time zone and fills in the appropriate
 * fields of the @dt structure. @str is updated to point just after the
 * xs:time.
 * In case of error, values of @dt fields are undefined.
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn _xmlSchemaParseTime(mut dt: xmlSchemaValDatePtr,
                                         mut str: *mut *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = *str;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut value: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) < '0' as i32 ||
           *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
           ||
           (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '0' as i32 ||
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
       {
        ret = 1 as std::os::raw::c_int
    } else {
        value =
            (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                 '0' as i32) * 10 as std::os::raw::c_int +
                (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                     '0' as i32)
    }
    cur = cur.offset(2 as std::os::raw::c_int as isize);
    if ret != 0 as std::os::raw::c_int { return ret }
    if *cur as std::os::raw::c_int != ':' as i32 { return 1 as std::os::raw::c_int }
    if !(value >= 0 as std::os::raw::c_int && value <= 23 as std::os::raw::c_int) &&
           value != 24 as std::os::raw::c_int {
        /* Allow end-of-day hour */
        return 2 as std::os::raw::c_int
    }
    cur = cur.offset(1);
    /* the ':' insures this string is xs:time */
    (*dt).set_hour(value as std::os::raw::c_uint);
    if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) < '0' as i32 ||
           *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
           ||
           (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '0' as i32 ||
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
       {
        ret = 1 as std::os::raw::c_int
    } else {
        value =
            (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                 '0' as i32) * 10 as std::os::raw::c_int +
                (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                     '0' as i32)
    }
    cur = cur.offset(2 as std::os::raw::c_int as isize);
    if ret != 0 as std::os::raw::c_int { return ret }
    if !(value >= 0 as std::os::raw::c_int && value <= 59 as std::os::raw::c_int) {
        return 2 as std::os::raw::c_int
    }
    (*dt).set_min(value as std::os::raw::c_uint);
    if *cur as std::os::raw::c_int != ':' as i32 { return 1 as std::os::raw::c_int }
    cur = cur.offset(1);
    if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) < '0' as i32 ||
           *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
           ||
           (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               '0' as i32 ||
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int > '9' as i32
       {
        ret = 1 as std::os::raw::c_int
    } else {
        (*dt).sec =
            ((*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                  '0' as i32) * 10 as std::os::raw::c_int +
                 (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                      '0' as i32)) as std::os::raw::c_double
    }
    cur = cur.offset(2 as std::os::raw::c_int as isize);
    if ret == 0 && *cur as std::os::raw::c_int == '.' as i32 {
        let mut mult: std::os::raw::c_double = 1 as std::os::raw::c_int as std::os::raw::c_double;
        cur = cur.offset(1);
        if (*cur as std::os::raw::c_int) < '0' as i32 ||
               *cur as std::os::raw::c_int > '9' as i32 {
            ret = 1 as std::os::raw::c_int
        }
        while *cur as std::os::raw::c_int >= '0' as i32 &&
                  *cur as std::os::raw::c_int <= '9' as i32 {
            mult /= 10 as std::os::raw::c_int as std::os::raw::c_double;
            (*dt).sec +=
                (*cur as std::os::raw::c_int - '0' as i32) as std::os::raw::c_double * mult;
            cur = cur.offset(1)
        }
    }
    if ret != 0 as std::os::raw::c_int { return ret }
    if !(((*dt).hour() as std::os::raw::c_int >= 0 as std::os::raw::c_int &&
              (*dt).hour() as std::os::raw::c_int <= 23 as std::os::raw::c_int &&
              ((*dt).min() as std::os::raw::c_int >= 0 as std::os::raw::c_int &&
                   (*dt).min() as std::os::raw::c_int <= 59 as std::os::raw::c_int) &&
              ((*dt).sec >= 0 as std::os::raw::c_int as std::os::raw::c_double &&
                   (*dt).sec < 60 as std::os::raw::c_int as std::os::raw::c_double) ||
              (*dt).hour() as std::os::raw::c_int == 24 as std::os::raw::c_int &&
                  (*dt).min() as std::os::raw::c_int == 0 as std::os::raw::c_int &&
                  (*dt).sec == 0 as std::os::raw::c_int as std::os::raw::c_double) &&
             ((*dt).tzo() > -(840 as std::os::raw::c_int) &&
                  (*dt).tzo() < 840 as std::os::raw::c_int)) {
        return 2 as std::os::raw::c_int
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * _xmlSchemaParseTimeZone:
 * @dt:  pointer to a date structure
 * @str: pointer to the string to analyze
 *
 * Parses a time zone without time zone and fills in the appropriate
 * field of the @dt structure. @str is updated to point just after the
 * time zone.
 *
 * Returns 0 or the error code
 */
unsafe extern "C" fn _xmlSchemaParseTimeZone(mut dt: xmlSchemaValDatePtr,
                                             mut str: *mut *const xmlChar)
 -> std::os::raw::c_int {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if str.is_null() { return -(1 as std::os::raw::c_int) }
    cur = *str;
    match *cur as std::os::raw::c_int {
        0 => {
            (*dt).set_tz_flag(0 as std::os::raw::c_int as std::os::raw::c_uint);
            (*dt).set_tzo(0 as std::os::raw::c_int)
        }
        90 => {
            (*dt).set_tz_flag(1 as std::os::raw::c_int as std::os::raw::c_uint);
            (*dt).set_tzo(0 as std::os::raw::c_int);
            cur = cur.offset(1)
        }
        43 | 45 => {
            let mut isneg: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut tmp: std::os::raw::c_int = 0 as std::os::raw::c_int;
            isneg = (*cur as std::os::raw::c_int == '-' as i32) as std::os::raw::c_int;
            cur = cur.offset(1);
            if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
                   '0' as i32 ||
                   *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >
                       '9' as i32 ||
                   (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
                       '0' as i32 ||
                   *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >
                       '9' as i32 {
                ret = 1 as std::os::raw::c_int
            } else {
                tmp =
                    (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                         '0' as i32) * 10 as std::os::raw::c_int +
                        (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                             - '0' as i32)
            }
            cur = cur.offset(2 as std::os::raw::c_int as isize);
            if ret != 0 as std::os::raw::c_int { return ret }
            if !(tmp >= 0 as std::os::raw::c_int && tmp <= 23 as std::os::raw::c_int) {
                return 2 as std::os::raw::c_int
            }
            if *cur as std::os::raw::c_int != ':' as i32 { return 1 as std::os::raw::c_int }
            cur = cur.offset(1);
            (*dt).set_tzo(tmp * 60 as std::os::raw::c_int);
            if (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
                   '0' as i32 ||
                   *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >
                       '9' as i32 ||
                   (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
                       '0' as i32 ||
                   *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >
                       '9' as i32 {
                ret = 1 as std::os::raw::c_int
            } else {
                tmp =
                    (*cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                         '0' as i32) * 10 as std::os::raw::c_int +
                        (*cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                             - '0' as i32)
            }
            cur = cur.offset(2 as std::os::raw::c_int as isize);
            if ret != 0 as std::os::raw::c_int { return ret }
            if !(tmp >= 0 as std::os::raw::c_int && tmp <= 59 as std::os::raw::c_int) {
                return 2 as std::os::raw::c_int
            }
            (*dt).set_tzo((*dt).tzo() + tmp);
            if isneg != 0 { (*dt).set_tzo(-(*dt).tzo()) }
            if !((*dt).tzo() > -(840 as std::os::raw::c_int) &&
                     (*dt).tzo() < 840 as std::os::raw::c_int) {
                return 2 as std::os::raw::c_int
            }
            (*dt).set_tz_flag(1 as std::os::raw::c_int as std::os::raw::c_uint)
        }
        _ => { return 1 as std::os::raw::c_int }
    }
    *str = cur;
    return 0 as std::os::raw::c_int;
}
/* *
 * _xmlSchemaBase64Decode:
 * @ch: a character
 *
 * Converts a base64 encoded character to its base 64 value.
 *
 * Returns 0-63 (value), 64 (pad), or -1 (not recognized)
 */
unsafe extern "C" fn _xmlSchemaBase64Decode(ch: xmlChar) -> std::os::raw::c_int {
    if 'A' as i32 <= ch as std::os::raw::c_int && ch as std::os::raw::c_int <= 'Z' as i32 {
        return ch as std::os::raw::c_int - 'A' as i32
    }
    if 'a' as i32 <= ch as std::os::raw::c_int && ch as std::os::raw::c_int <= 'z' as i32 {
        return ch as std::os::raw::c_int - 'a' as i32 + 26 as std::os::raw::c_int
    }
    if '0' as i32 <= ch as std::os::raw::c_int && ch as std::os::raw::c_int <= '9' as i32 {
        return ch as std::os::raw::c_int - '0' as i32 + 52 as std::os::raw::c_int
    }
    if '+' as i32 == ch as std::os::raw::c_int { return 62 as std::os::raw::c_int }
    if '/' as i32 == ch as std::os::raw::c_int { return 63 as std::os::raw::c_int }
    if '=' as i32 == ch as std::os::raw::c_int { return 64 as std::os::raw::c_int }
    return -(1 as std::os::raw::c_int);
}
/* ***************************************************************
 *								*
 *	XML Schema Dates/Times Datatypes Handling		*
 *								*
 ****************************************************************/
/* *
 * PARSE_DIGITS:
 * @num:  the integer to fill in
 * @cur:  an #xmlChar *
 * @num_type: an integer flag
 *
 * Parses a digits integer and updates @num with the value. @cur is
 * updated to point just after the integer.
 * In case of error, @num_type is set to -1, values of @num and
 * @cur are undefined.
 */
/* *
 * PARSE_NUM:
 * @num:  the double to fill in
 * @cur:  an #xmlChar *
 * @num_type: an integer flag
 *
 * Parses a float or integer and updates @num with the value. @cur is
 * updated to point just after the number. If the number is a float,
 * then it must have an integer part and a decimal part; @num_type will
 * be set to 1. If there is no decimal part, @num_type is set to zero.
 * In case of error, @num_type is set to -1, values of @num and
 * @cur are undefined.
 */
/* *
 * xmlSchemaValidateDates:
 * @type: the expected type or XML_SCHEMAS_UNKNOWN
 * @dateTime:  string to analyze
 * @val:  the return computed value
 *
 * Check that @dateTime conforms to the lexical space of one of the date types.
 * if true a value is computed and returned in @val.
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
unsafe extern "C" fn xmlSchemaValidateDates(mut type_0: xmlSchemaValType,
                                            mut dateTime: *const xmlChar,
                                            mut val: *mut xmlSchemaValPtr,
                                            mut collapse: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut dt: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut ret: std::os::raw::c_int = 0;
    let mut cur: *const xmlChar = dateTime;
    if dateTime.is_null() { return -(1 as std::os::raw::c_int) }
    if collapse != 0 {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
    }
    if *cur as std::os::raw::c_int != '-' as i32 && (*cur as std::os::raw::c_int) < '0' as i32
           && *cur as std::os::raw::c_int > '9' as i32 {
        return 1 as std::os::raw::c_int
    }
    dt = xmlSchemaNewValue(XML_SCHEMAS_UNKNOWN);
    if dt.is_null() { return -(1 as std::os::raw::c_int) }
    if *cur.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '-' as i32 &&
           *cur.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '-' as i32
       {
        /*
	 * It's an incomplete date (xs:gMonthDay, xs:gMonth or
	 * xs:gDay)
	 */
        cur = cur.offset(2 as std::os::raw::c_int as isize);
        /* is it an xs:gDay? */
        if *cur as std::os::raw::c_int == '-' as i32 {
            if type_0 as std::os::raw::c_uint ==
                   XML_SCHEMAS_GMONTH as std::os::raw::c_int as std::os::raw::c_uint {
                current_block = 7269807481137428954;
            } else {
                cur = cur.offset(1);
                ret = _xmlSchemaParseGDay(&mut (*dt).value.date, &mut cur);
                if ret != 0 as std::os::raw::c_int {
                    current_block = 7269807481137428954;
                } else if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                              *cur as std::os::raw::c_int == 'Z' as i32 ||
                              *cur as std::os::raw::c_int == '+' as i32 ||
                              *cur as std::os::raw::c_int == '-' as i32 {
                    ret =
                        _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                &mut cur);
                    if ret == 0 as std::os::raw::c_int {
                        if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                            current_block = 7269807481137428954;
                        } else {
                            (*dt).type_0 = XML_SCHEMAS_GDAY;
                            current_block = 7723759518367501574;
                        }
                    } else { current_block = 7269807481137428954; }
                } else { current_block = 7269807481137428954; }
            }
        } else {
            /*
	 * it should be an xs:gMonthDay or xs:gMonth
	 */
            ret = _xmlSchemaParseGMonth(&mut (*dt).value.date, &mut cur);
            if ret != 0 as std::os::raw::c_int {
                current_block = 7269807481137428954;
            } else {
                /*
         * a '-' char could indicate this type is xs:gMonthDay or
         * a negative time zone offset. Check for xs:gMonthDay first.
         * Also the first three char's of a negative tzo (-MM:SS) can
         * appear to be a valid day; so even if the day portion
         * of the xs:gMonthDay verifies, we must insure it was not
         * a tzo.
         */
                if *cur as std::os::raw::c_int == '-' as i32 {
                    let mut rewnd: *const xmlChar = cur;
                    cur = cur.offset(1);
                    ret =
                        _xmlSchemaParseGDay(&mut (*dt).value.date, &mut cur);
                    if ret == 0 as std::os::raw::c_int &&
                           (*cur as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                                *cur as std::os::raw::c_int != ':' as i32) {
                        /*
                 * we can use the VALID_MDAY macro to validate the month
                 * and day because the leap year test will flag year zero
                 * as a leap year (even though zero is an invalid year).
		 * FUTURE TODO: Zero will become valid in XML Schema 1.1
		 * probably.
                 */
                        if if (*dt).value.date.year %
                                  4 as std::os::raw::c_int as std::os::raw::c_long ==
                                  0 as std::os::raw::c_int as std::os::raw::c_long &&
                                  (*dt).value.date.year %
                                      100 as std::os::raw::c_int as std::os::raw::c_long !=
                                      0 as std::os::raw::c_int as std::os::raw::c_long ||
                                  (*dt).value.date.year %
                                      400 as std::os::raw::c_int as std::os::raw::c_long ==
                                      0 as std::os::raw::c_int as std::os::raw::c_long {
                               ((*dt).value.date.day() <=
                                    daysInMonthLeap[((*dt).value.date.mon() as
                                                         std::os::raw::c_int -
                                                         1 as std::os::raw::c_int) as
                                                        usize]) as std::os::raw::c_int
                           } else {
                               ((*dt).value.date.day() <=
                                    daysInMonth[((*dt).value.date.mon() as
                                                     std::os::raw::c_int -
                                                     1 as std::os::raw::c_int) as
                                                    usize]) as std::os::raw::c_int
                           } != 0 {
                            if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                                   *cur as std::os::raw::c_int == 'Z' as i32 ||
                                   *cur as std::os::raw::c_int == '+' as i32 ||
                                   *cur as std::os::raw::c_int == '-' as i32 {
                                ret =
                                    _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                            &mut cur);
                                if ret == 0 as std::os::raw::c_int {
                                    if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int
                                       {
                                        current_block = 7269807481137428954;
                                    } else {
                                        (*dt).type_0 = XML_SCHEMAS_GMONTHDAY;
                                        current_block = 7723759518367501574;
                                    }
                                } else {
                                    current_block = 7269807481137428954;
                                }
                            } else { current_block = 7269807481137428954; }
                        } else { current_block = 15090052786889560393; }
                    } else { current_block = 15090052786889560393; }
                    match current_block {
                        7723759518367501574 => { }
                        7269807481137428954 => { }
                        _ => {
                            /*
             * not xs:gMonthDay so rewind and check if just xs:gMonth
             * with an optional time zone.
             */
                            cur = rewnd;
                            current_block = 18435049525520518667;
                        }
                    }
                } else { current_block = 18435049525520518667; }
                match current_block {
                    7269807481137428954 => { }
                    7723759518367501574 => { }
                    _ => {
                        if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                               *cur as std::os::raw::c_int == 'Z' as i32 ||
                               *cur as std::os::raw::c_int == '+' as i32 ||
                               *cur as std::os::raw::c_int == '-' as i32 {
                            ret =
                                _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                        &mut cur);
                            if ret == 0 as std::os::raw::c_int {
                                if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                    current_block = 7269807481137428954;
                                } else {
                                    (*dt).type_0 = XML_SCHEMAS_GMONTH;
                                    current_block = 7723759518367501574;
                                }
                            } else { current_block = 7269807481137428954; }
                        } else { current_block = 7269807481137428954; }
                    }
                }
            }
        }
    } else {
        /*
     * It's a right-truncated date or an xs:time.
     * Try to parse an xs:time then fallback on right-truncated dates.
     */
        if *cur as std::os::raw::c_int >= '0' as i32 &&
               *cur as std::os::raw::c_int <= '9' as i32 {
            ret = _xmlSchemaParseTime(&mut (*dt).value.date, &mut cur);
            if ret == 0 as std::os::raw::c_int {
                /* it's an xs:time */
                if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                       *cur as std::os::raw::c_int == 'Z' as i32 ||
                       *cur as std::os::raw::c_int == '+' as i32 ||
                       *cur as std::os::raw::c_int == '-' as i32 {
                    ret =
                        _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                &mut cur);
                    if ret == 0 as std::os::raw::c_int {
                        if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                            current_block = 7269807481137428954;
                        } else {
                            (*dt).type_0 = XML_SCHEMAS_TIME;
                            current_block = 7723759518367501574;
                        }
                    } else { current_block = 17836213544692497527; }
                } else { current_block = 17836213544692497527; }
            } else { current_block = 17836213544692497527; }
        } else { current_block = 17836213544692497527; }
        match current_block {
            7269807481137428954 => { }
            7723759518367501574 => { }
            _ => {
                /* fallback on date parsing */
                cur = dateTime;
                ret = _xmlSchemaParseGYear(&mut (*dt).value.date, &mut cur);
                if ret != 0 as std::os::raw::c_int {
                    current_block = 7269807481137428954;
                } else {
                    /* is it an xs:gYear? */
                    if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                           *cur as std::os::raw::c_int == 'Z' as i32 ||
                           *cur as std::os::raw::c_int == '+' as i32 ||
                           *cur as std::os::raw::c_int == '-' as i32 {
                        ret =
                            _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                    &mut cur);
                        if ret == 0 as std::os::raw::c_int {
                            if *cur as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                current_block = 7269807481137428954;
                            } else {
                                (*dt).type_0 = XML_SCHEMAS_GYEAR;
                                current_block = 7723759518367501574;
                            }
                        } else { current_block = 7858101417678297991; }
                    } else { current_block = 7858101417678297991; }
                    match current_block {
                        7269807481137428954 => { }
                        7723759518367501574 => { }
                        _ => {
                            if *cur as std::os::raw::c_int != '-' as i32 {
                                current_block = 7269807481137428954;
                            } else {
                                cur = cur.offset(1);
                                ret =
                                    _xmlSchemaParseGMonth(&mut (*dt).value.date,
                                                          &mut cur);
                                if ret != 0 as std::os::raw::c_int {
                                    current_block = 7269807481137428954;
                                } else {
                                    /* is it an xs:gYearMonth? */
                                    if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int
                                           ||
                                           *cur as std::os::raw::c_int == 'Z' as i32
                                           ||
                                           *cur as std::os::raw::c_int == '+' as i32
                                           ||
                                           *cur as std::os::raw::c_int == '-' as i32 {
                                        ret =
                                            _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                                    &mut cur);
                                        if ret == 0 as std::os::raw::c_int {
                                            if *cur as std::os::raw::c_int !=
                                                   0 as std::os::raw::c_int {
                                                current_block =
                                                    7269807481137428954;
                                            } else {
                                                (*dt).type_0 =
                                                    XML_SCHEMAS_GYEARMONTH;
                                                current_block =
                                                    7723759518367501574;
                                            }
                                        } else {
                                            current_block =
                                                2472048668343472511;
                                        }
                                    } else {
                                        current_block = 2472048668343472511;
                                    }
                                    match current_block {
                                        7269807481137428954 => { }
                                        7723759518367501574 => { }
                                        _ => {
                                            if *cur as std::os::raw::c_int !=
                                                   '-' as i32 {
                                                current_block =
                                                    7269807481137428954;
                                            } else {
                                                cur = cur.offset(1);
                                                ret =
                                                    _xmlSchemaParseGDay(&mut (*dt).value.date,
                                                                        &mut cur);
                                                if ret != 0 as std::os::raw::c_int ||
                                                       !((*dt).value.date.year
                                                             !=
                                                             0 as std::os::raw::c_int
                                                                 as
                                                                 std::os::raw::c_long
                                                             &&
                                                             ((*dt).value.date.mon()
                                                                  as
                                                                  std::os::raw::c_int
                                                                  >=
                                                                  1 as
                                                                      std::os::raw::c_int
                                                                  &&
                                                                  (*dt).value.date.mon()
                                                                      as
                                                                      std::os::raw::c_int
                                                                      <=
                                                                      12 as
                                                                          std::os::raw::c_int)
                                                             &&
                                                             (if (*dt).value.date.year
                                                                     %
                                                                     4 as
                                                                         std::os::raw::c_int
                                                                         as
                                                                         std::os::raw::c_long
                                                                     ==
                                                                     0 as
                                                                         std::os::raw::c_int
                                                                         as
                                                                         std::os::raw::c_long
                                                                     &&
                                                                     (*dt).value.date.year
                                                                         %
                                                                         100
                                                                             as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_long
                                                                         !=
                                                                         0 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_long
                                                                     ||
                                                                     (*dt).value.date.year
                                                                         %
                                                                         400
                                                                             as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_long
                                                                         ==
                                                                         0 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_long
                                                                 {
                                                                  ((*dt).value.date.day()
                                                                       <=
                                                                       daysInMonthLeap[((*dt).value.date.mon()
                                                                                            as
                                                                                            std::os::raw::c_int
                                                                                            -
                                                                                            1
                                                                                                as
                                                                                                std::os::raw::c_int)
                                                                                           as
                                                                                           usize])
                                                                      as
                                                                      std::os::raw::c_int
                                                              } else {
                                                                  ((*dt).value.date.day()
                                                                       <=
                                                                       daysInMonth[((*dt).value.date.mon()
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        -
                                                                                        1
                                                                                            as
                                                                                            std::os::raw::c_int)
                                                                                       as
                                                                                       usize])
                                                                      as
                                                                      std::os::raw::c_int
                                                              }) != 0) {
                                                    current_block =
                                                        7269807481137428954;
                                                } else {
                                                    /* is it an xs:date? */
                                                    if *cur as std::os::raw::c_int ==
                                                           0 as std::os::raw::c_int ||
                                                           *cur as std::os::raw::c_int
                                                               == 'Z' as i32
                                                           ||
                                                           *cur as std::os::raw::c_int
                                                               == '+' as i32
                                                           ||
                                                           *cur as std::os::raw::c_int
                                                               == '-' as i32 {
                                                        ret =
                                                            _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                                                    &mut cur);
                                                        if ret ==
                                                               0 as
                                                                   std::os::raw::c_int
                                                           {
                                                            if *cur as
                                                                   std::os::raw::c_int
                                                                   !=
                                                                   0 as
                                                                       std::os::raw::c_int
                                                               {
                                                                current_block
                                                                    =
                                                                    7269807481137428954;
                                                            } else {
                                                                (*dt).type_0 =
                                                                    XML_SCHEMAS_DATE;
                                                                current_block
                                                                    =
                                                                    7723759518367501574;
                                                            }
                                                        } else {
                                                            current_block =
                                                                178030534879405462;
                                                        }
                                                    } else {
                                                        current_block =
                                                            178030534879405462;
                                                    }
                                                    match current_block {
                                                        7723759518367501574 =>
                                                        {
                                                        }
                                                        7269807481137428954 =>
                                                        {
                                                        }
                                                        _ => {
                                                            if *cur as
                                                                   std::os::raw::c_int
                                                                   !=
                                                                   'T' as i32
                                                               {
                                                                current_block
                                                                    =
                                                                    7269807481137428954;
                                                            } else {
                                                                cur =
                                                                    cur.offset(1);
                                                                /* it should be an xs:dateTime */
                                                                ret =
                                                                    _xmlSchemaParseTime(&mut (*dt).value.date,
                                                                                        &mut cur);
                                                                if ret !=
                                                                       0 as
                                                                           std::os::raw::c_int
                                                                   {
                                                                    current_block
                                                                        =
                                                                        7269807481137428954;
                                                                } else {
                                                                    ret =
                                                                        _xmlSchemaParseTimeZone(&mut (*dt).value.date,
                                                                                                &mut cur);
                                                                    if collapse
                                                                           !=
                                                                           0 {
                                                                        while *cur
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  ==
                                                                                  0x20
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                  ||
                                                                                  0x9
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      <=
                                                                                      *cur
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                      &&
                                                                                      *cur
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          <=
                                                                                          0xa
                                                                                              as
                                                                                              std::os::raw::c_int
                                                                                  ||
                                                                                  *cur
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      ==
                                                                                      0xd
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                              {
                                                                            cur
                                                                                =
                                                                                cur.offset(1)
                                                                        }
                                                                    }
                                                                    if ret !=
                                                                           0
                                                                               as
                                                                               std::os::raw::c_int
                                                                           ||
                                                                           *cur
                                                                               as
                                                                               std::os::raw::c_int
                                                                               !=
                                                                               0
                                                                                   as
                                                                                   std::os::raw::c_int
                                                                           ||
                                                                           !((*dt).value.date.year
                                                                                 !=
                                                                                 0
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_long
                                                                                 &&
                                                                                 ((*dt).value.date.mon()
                                                                                      as
                                                                                      std::os::raw::c_int
                                                                                      >=
                                                                                      1
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                      &&
                                                                                      (*dt).value.date.mon()
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          <=
                                                                                          12
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                 &&
                                                                                 (if (*dt).value.date.year
                                                                                         %
                                                                                         4
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             std::os::raw::c_long
                                                                                         ==
                                                                                         0
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             std::os::raw::c_long
                                                                                         &&
                                                                                         (*dt).value.date.year
                                                                                             %
                                                                                             100
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_long
                                                                                             !=
                                                                                             0
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_long
                                                                                         ||
                                                                                         (*dt).value.date.year
                                                                                             %
                                                                                             400
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_long
                                                                                             ==
                                                                                             0
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_long
                                                                                     {
                                                                                      ((*dt).value.date.day()
                                                                                           <=
                                                                                           daysInMonthLeap[((*dt).value.date.mon()
                                                                                                                as
                                                                                                                std::os::raw::c_int
                                                                                                                -
                                                                                                                1
                                                                                                                    as
                                                                                                                    std::os::raw::c_int)
                                                                                                               as
                                                                                                               usize])
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                  } else {
                                                                                      ((*dt).value.date.day()
                                                                                           <=
                                                                                           daysInMonth[((*dt).value.date.mon()
                                                                                                            as
                                                                                                            std::os::raw::c_int
                                                                                                            -
                                                                                                            1
                                                                                                                as
                                                                                                                std::os::raw::c_int)
                                                                                                           as
                                                                                                           usize])
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                  })
                                                                                     !=
                                                                                     0
                                                                                 &&
                                                                                 (((*dt).value.date.hour()
                                                                                       as
                                                                                       std::os::raw::c_int
                                                                                       >=
                                                                                       0
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                       &&
                                                                                       (*dt).value.date.hour()
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                           <=
                                                                                           23
                                                                                               as
                                                                                               std::os::raw::c_int
                                                                                       &&
                                                                                       ((*dt).value.date.min()
                                                                                            as
                                                                                            std::os::raw::c_int
                                                                                            >=
                                                                                            0
                                                                                                as
                                                                                                std::os::raw::c_int
                                                                                            &&
                                                                                            (*dt).value.date.min()
                                                                                                as
                                                                                                std::os::raw::c_int
                                                                                                <=
                                                                                                59
                                                                                                    as
                                                                                                    std::os::raw::c_int)
                                                                                       &&
                                                                                       ((*dt).value.date.sec
                                                                                            >=
                                                                                            0
                                                                                                as
                                                                                                std::os::raw::c_int
                                                                                                as
                                                                                                std::os::raw::c_double
                                                                                            &&
                                                                                            (*dt).value.date.sec
                                                                                                <
                                                                                                60
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_double)
                                                                                       ||
                                                                                       (*dt).value.date.hour()
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                           ==
                                                                                           24
                                                                                               as
                                                                                               std::os::raw::c_int
                                                                                           &&
                                                                                           (*dt).value.date.min()
                                                                                               as
                                                                                               std::os::raw::c_int
                                                                                               ==
                                                                                               0
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                           &&
                                                                                           (*dt).value.date.sec
                                                                                               ==
                                                                                               0
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   as
                                                                                                   std::os::raw::c_double)
                                                                                      &&
                                                                                      ((*dt).value.date.tzo()
                                                                                           >
                                                                                           -(840
                                                                                                 as
                                                                                                 std::os::raw::c_int)
                                                                                           &&
                                                                                           (*dt).value.date.tzo()
                                                                                               <
                                                                                               840
                                                                                                   as
                                                                                                   std::os::raw::c_int)))
                                                                       {
                                                                        current_block
                                                                            =
                                                                            7269807481137428954;
                                                                    } else {
                                                                        (*dt).type_0
                                                                            =
                                                                            XML_SCHEMAS_DATETIME;
                                                                        current_block
                                                                            =
                                                                            7723759518367501574;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        7723759518367501574 => {
            if !(type_0 as std::os::raw::c_uint !=
                     XML_SCHEMAS_UNKNOWN as std::os::raw::c_int as std::os::raw::c_uint &&
                     type_0 as std::os::raw::c_uint != (*dt).type_0 as std::os::raw::c_uint) {
                if !val.is_null() {
                    *val = dt
                } else { xmlSchemaFreeValue(dt); }
                return 0 as std::os::raw::c_int
            }
        }
        _ => { }
    }
    if !dt.is_null() { xmlSchemaFreeValue(dt); }
    return 1 as std::os::raw::c_int;
}
/* *
 * xmlSchemaValidateDuration:
 * @type: the predefined type
 * @duration:  string to analyze
 * @val:  the return computed value
 *
 * Check that @duration conforms to the lexical space of the duration type.
 * if true a value is computed and returned in @val.
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
unsafe extern "C" fn xmlSchemaValidateDuration(mut type_0: xmlSchemaTypePtr,
                                               mut duration: *const xmlChar,
                                               mut val: *mut xmlSchemaValPtr,
                                               mut collapse: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut current_block: u64; /* -1 = invalid, 0 = int, 1 = floating */
    let mut cur: *const xmlChar = duration;
    let mut dur: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut isneg: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut seq: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut num: std::os::raw::c_double = 0.;
    let mut num_type: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let desig: [xmlChar; 6] =
        ['Y' as i32 as xmlChar, 'M' as i32 as xmlChar, 'D' as i32 as xmlChar,
         'H' as i32 as xmlChar, 'M' as i32 as xmlChar, 'S' as i32 as xmlChar];
    let multi: [std::os::raw::c_double; 7] =
        [0.0f64, 0.0f64, 86400.0f64, 3600.0f64, 60.0f64, 1.0f64, 0.0f64];
    if duration.is_null() { return -(1 as std::os::raw::c_int) }
    if collapse != 0 {
        while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                      *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            cur = cur.offset(1)
        }
    }
    if *cur as std::os::raw::c_int == '-' as i32 {
        isneg = 1 as std::os::raw::c_int;
        cur = cur.offset(1)
    }
    /* duration must start with 'P' (after sign) */
    let fresh0 = cur;
    cur = cur.offset(1);
    if *fresh0 as std::os::raw::c_int != 'P' as i32 { return 1 as std::os::raw::c_int }
    if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    dur = xmlSchemaNewValue(XML_SCHEMAS_DURATION);
    if dur.is_null() { return -(1 as std::os::raw::c_int) }
    's_95:
        loop  {
            if !(*cur as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                current_block = 317151059986244064;
                break ;
            }
            /* input string should be empty or invalid date/time item */
            if seq as std::os::raw::c_ulong >=
                   ::std::mem::size_of::<[xmlChar; 6]>() as std::os::raw::c_ulong {
                current_block = 3421249662423776657;
                break ;
            }
            /* T designator must be present for time items */
            if *cur as std::os::raw::c_int == 'T' as i32 {
                if seq <= 3 as std::os::raw::c_int as std::os::raw::c_uint {
                    seq = 3 as std::os::raw::c_int as std::os::raw::c_uint;
                    cur = cur.offset(1)
                } else { return 1 as std::os::raw::c_int }
            } else if seq == 3 as std::os::raw::c_int as std::os::raw::c_uint {
                current_block = 3421249662423776657;
                break ;
            }
            /* parse the number portion of the item */
            num = 0 as std::os::raw::c_int as std::os::raw::c_double;
            if (*cur as std::os::raw::c_int) < '0' as i32 ||
                   *cur as std::os::raw::c_int > '9' as i32 {
                num_type = -(1 as std::os::raw::c_int)
            } else {
                while *cur as std::os::raw::c_int >= '0' as i32 &&
                          *cur as std::os::raw::c_int <= '9' as i32 {
                    num =
                        num * 10 as std::os::raw::c_int as std::os::raw::c_double +
                            (*cur as std::os::raw::c_int - '0' as i32) as
                                std::os::raw::c_double;
                    cur = cur.offset(1)
                }
            }
            if num_type == 0 && *cur as std::os::raw::c_int == '.' as i32 {
                let mut mult: std::os::raw::c_double =
                    1 as std::os::raw::c_int as std::os::raw::c_double;
                cur = cur.offset(1);
                if (*cur as std::os::raw::c_int) < '0' as i32 ||
                       *cur as std::os::raw::c_int > '9' as i32 {
                    num_type = -(1 as std::os::raw::c_int)
                } else { num_type = 1 as std::os::raw::c_int }
                while *cur as std::os::raw::c_int >= '0' as i32 &&
                          *cur as std::os::raw::c_int <= '9' as i32 {
                    mult /= 10 as std::os::raw::c_int as std::os::raw::c_double;
                    num +=
                        (*cur as std::os::raw::c_int - '0' as i32) as std::os::raw::c_double *
                            mult;
                    cur = cur.offset(1)
                }
            }
            if num_type == -(1 as std::os::raw::c_int) ||
                   *cur as std::os::raw::c_int == 0 as std::os::raw::c_int {
                current_block = 3421249662423776657;
                break ;
            }
            /* update duration based on item type */
            while (seq as std::os::raw::c_ulong) <
                      ::std::mem::size_of::<[xmlChar; 6]>() as std::os::raw::c_ulong {
                if *cur as std::os::raw::c_int == desig[seq as usize] as std::os::raw::c_int {
                    /* verify numeric type; only seconds can be float */
                    if num_type != 0 as std::os::raw::c_int &&
                           (seq as std::os::raw::c_ulong) <
                               (::std::mem::size_of::<[xmlChar; 6]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                       {
                        current_block = 3421249662423776657;
                        break 's_95 ;
                    }
                    match seq {
                        0 => {
                            (*dur).value.dur.mon =
                                num as std::os::raw::c_long *
                                    12 as std::os::raw::c_int as std::os::raw::c_long
                        }
                        1 => { (*dur).value.dur.mon += num as std::os::raw::c_long }
                        _ => {
                            /* convert to seconds using multiplier */
                            (*dur).value.dur.sec += num * multi[seq as usize];
                            seq = seq.wrapping_add(1)
                        }
                    }
                    break ;
                    /* exit loop */
                } else {
                    /* no date designators found? */
                    seq = seq.wrapping_add(1);
                    if seq == 3 as std::os::raw::c_int as std::os::raw::c_uint ||
                           seq == 6 as std::os::raw::c_int as std::os::raw::c_uint {
                        current_block = 3421249662423776657;
                        break 's_95 ;
                    }
                }
            }
            cur = cur.offset(1);
            if collapse != 0 {
                while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                              *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                          *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    cur = cur.offset(1)
                }
            }
        }
    match current_block {
        3421249662423776657 => {
            if !dur.is_null() { xmlSchemaFreeValue(dur); }
            return 1 as std::os::raw::c_int
        }
        _ => {
            if isneg != 0 {
                (*dur).value.dur.mon = -(*dur).value.dur.mon;
                (*dur).value.dur.day = -(*dur).value.dur.day;
                (*dur).value.dur.sec = -(*dur).value.dur.sec
            }
            if !val.is_null() { *val = dur } else { xmlSchemaFreeValue(dur); }
            return 0 as std::os::raw::c_int
        }
    };
}
/* *
 * xmlSchemaStrip:
 * @value: a value
 *
 * Removes the leading and ending spaces of a string
 *
 * Returns the new string or NULL if no change was required.
 */
unsafe extern "C" fn xmlSchemaStrip(mut value: *const xmlChar)
 -> *mut xmlChar {
    let mut start: *const xmlChar = value;
    let mut end: *const xmlChar = 0 as *const xmlChar;
    let mut f: *const xmlChar = 0 as *const xmlChar;
    if value.is_null() { return 0 as *mut xmlChar }
    while *start as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              (*start as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                   0x9 as std::os::raw::c_int <= *start as std::os::raw::c_int &&
                       *start as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                   *start as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
        start = start.offset(1)
    }
    end = start;
    while *end as std::os::raw::c_int != 0 as std::os::raw::c_int { end = end.offset(1) }
    f = end;
    end = end.offset(-1);
    while end > start &&
              (*end as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                   0x9 as std::os::raw::c_int <= *end as std::os::raw::c_int &&
                       *end as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                   *end as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
        end = end.offset(-1)
    }
    end = end.offset(1);
    if start == value && f == end { return 0 as *mut xmlChar }
    return xmlStrndup(start,
                      end.offset_from(start) as std::os::raw::c_long as
                          std::os::raw::c_int);
}
/* *
 * xmlSchemaWhiteSpaceReplace:
 * @value: a value
 *
 * Replaces 0xd, 0x9 and 0xa with a space.
 *
 * Returns the new string or NULL if no change was required.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaWhiteSpaceReplace(mut value: *const xmlChar)
 -> *mut xmlChar {
    let mut cur: *const xmlChar = value;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut mcur: *mut xmlChar = 0 as *mut xmlChar;
    if value.is_null() { return 0 as *mut xmlChar }
    while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              (*cur as std::os::raw::c_int != 0xd as std::os::raw::c_int &&
                   *cur as std::os::raw::c_int != 0x9 as std::os::raw::c_int &&
                   *cur as std::os::raw::c_int != 0xa as std::os::raw::c_int) {
        cur = cur.offset(1)
    }
    if *cur as std::os::raw::c_int == 0 as std::os::raw::c_int { return 0 as *mut xmlChar }
    ret = xmlStrdup(value);
    /* TODO FIXME: I guess gcc will bark at this. */
    mcur =
        ret.offset(cur.offset_from(value) as std::os::raw::c_long as isize);
    loop  {
        if *mcur as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
               *mcur as std::os::raw::c_int == 0x9 as std::os::raw::c_int ||
               *mcur as std::os::raw::c_int == 0xa as std::os::raw::c_int {
            *mcur = ' ' as i32 as xmlChar
        }
        mcur = mcur.offset(1);
        if !(*mcur as std::os::raw::c_int != 0 as std::os::raw::c_int) { break ; }
    }
    return ret;
}
/* *
 * xmlSchemaCollapseString:
 * @value: a value
 *
 * Removes and normalize white spaces in the string
 *
 * Returns the new string or NULL if no change was required.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaCollapseString(mut value: *const xmlChar)
 -> *mut xmlChar {
    let mut start: *const xmlChar = value;
    let mut end: *const xmlChar = 0 as *const xmlChar;
    let mut f: *const xmlChar = 0 as *const xmlChar;
    let mut g: *mut xmlChar = 0 as *mut xmlChar;
    let mut col: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if value.is_null() { return 0 as *mut xmlChar }
    while *start as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              (*start as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                   0x9 as std::os::raw::c_int <= *start as std::os::raw::c_int &&
                       *start as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                   *start as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
        start = start.offset(1)
    }
    end = start;
    while *end as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *end as std::os::raw::c_int == ' ' as i32 &&
               (*end.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                    0x20 as std::os::raw::c_int ||
                    0x9 as std::os::raw::c_int <=
                        *end.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                        &&
                        *end.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                            <= 0xa as std::os::raw::c_int ||
                    *end.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                        0xd as std::os::raw::c_int) {
            col =
                end.offset_from(start) as std::os::raw::c_long as
                    std::os::raw::c_int;
            break ;
        } else if *end as std::os::raw::c_int == 0xa as std::os::raw::c_int ||
                      *end as std::os::raw::c_int == 0x9 as std::os::raw::c_int ||
                      *end as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            col =
                end.offset_from(start) as std::os::raw::c_long as
                    std::os::raw::c_int;
            break ;
        } else { end = end.offset(1) }
    }
    if col == 0 as std::os::raw::c_int {
        f = end;
        end = end.offset(-1);
        while end > start &&
                  (*end as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                       0x9 as std::os::raw::c_int <= *end as std::os::raw::c_int &&
                           *end as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                       *end as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
            end = end.offset(-1)
        }
        end = end.offset(1);
        if start == value && f == end { return 0 as *mut xmlChar }
        return xmlStrndup(start,
                          end.offset_from(start) as std::os::raw::c_long as
                              std::os::raw::c_int)
    }
    start = xmlStrdup(start);
    if start.is_null() { return 0 as *mut xmlChar }
    g = start.offset(col as isize) as *mut xmlChar;
    end = g;
    while *end as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *end as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *end as std::os::raw::c_int &&
                   *end as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *end as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            end = end.offset(1);
            while *end as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *end as std::os::raw::c_int &&
                          *end as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *end as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                end = end.offset(1)
            }
            if *end as std::os::raw::c_int != 0 as std::os::raw::c_int {
                let fresh1 = g;
                g = g.offset(1);
                *fresh1 = ' ' as i32 as xmlChar
            }
        } else {
            let fresh2 = end;
            end = end.offset(1);
            let fresh3 = g;
            g = g.offset(1);
            *fresh3 = *fresh2
        }
    }
    *g = 0 as std::os::raw::c_int as xmlChar;
    return start as *mut xmlChar;
}
/* *
 * xmlSchemaValAtomicListNode:
 * @type: the predefined atomic type for a token in the list
 * @value: the list value to check
 * @ret:  the return computed value
 * @node:  the node containing the value
 *
 * Check that a value conforms to the lexical space of the predefined
 * list type. if true a value is computed and returned in @ret.
 *
 * Returns the number of items if this validates, a negative error code
 *         number otherwise
 */
unsafe extern "C" fn xmlSchemaValAtomicListNode(mut type_0: xmlSchemaTypePtr,
                                                mut value: *const xmlChar,
                                                mut ret: *mut xmlSchemaValPtr,
                                                mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: *mut xmlChar = 0 as *mut xmlChar;
    let mut endval: *mut xmlChar = 0 as *mut xmlChar;
    let mut nb_values: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut tmp: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    val = xmlStrdup(value);
    if val.is_null() { return -(1 as std::os::raw::c_int) }
    if !ret.is_null() { *ret = 0 as xmlSchemaValPtr }
    cur = val;
    /*
     * Split the list
     */
    while *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                  *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        let fresh4 = cur;
        cur = cur.offset(1);
        *fresh4 = 0 as std::os::raw::c_int as xmlChar
    }
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
                let fresh5 = cur;
                cur = cur.offset(1);
                *fresh5 = 0 as std::os::raw::c_int as xmlChar
            }
        } else {
            nb_values += 1;
            cur = cur.offset(1);
            while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      !(*cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                            0x9 as std::os::raw::c_int <= *cur as std::os::raw::c_int &&
                                *cur as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                            *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                cur = cur.offset(1)
            }
        }
    }
    if nb_values == 0 as std::os::raw::c_int {
        xmlFree.expect("non-null function pointer")(val as *mut std::os::raw::c_void);
        return nb_values
    }
    endval = cur;
    cur = val;
    while *cur as std::os::raw::c_int == 0 as std::os::raw::c_int && cur != endval {
        cur = cur.offset(1)
    }
    while cur != endval {
        tmp =
            xmlSchemaValPredefTypeNode(type_0, cur, 0 as *mut xmlSchemaValPtr,
                                       node);
        if tmp != 0 as std::os::raw::c_int { break ; }
        while *cur as std::os::raw::c_int != 0 as std::os::raw::c_int { cur = cur.offset(1) }
        while *cur as std::os::raw::c_int == 0 as std::os::raw::c_int && cur != endval {
            cur = cur.offset(1)
        }
    }
    /* TODO what return value ? c.f. bug #158628
    if (ret != NULL) {
	TODO
    } */
    xmlFree.expect("non-null function pointer")(val as *mut std::os::raw::c_void);
    if tmp == 0 as std::os::raw::c_int { return nb_values }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlSchemaParseUInt:
 * @str: pointer to the string R/W
 * @llo: pointer to the low result
 * @lmi: pointer to the mid result
 * @lhi: pointer to the high result
 *
 * Parse an unsigned long into 3 fields.
 *
 * Returns the number of significant digits in the number or
 * -1 if overflow of the capacity and -2 if it's not a number.
 */
unsafe extern "C" fn xmlSchemaParseUInt(mut str: *mut *const xmlChar,
                                        mut llo: *mut std::os::raw::c_ulong,
                                        mut lmi: *mut std::os::raw::c_ulong,
                                        mut lhi: *mut std::os::raw::c_ulong)
 -> std::os::raw::c_int {
    let mut lo: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut mi: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut hi: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = *str;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !(*cur as std::os::raw::c_int >= '0' as i32 &&
             *cur as std::os::raw::c_int <= '9' as i32) {
        return -(2 as std::os::raw::c_int)
    }
    while *cur as std::os::raw::c_int == '0' as i32 {
        /* ignore leading zeroes */
        cur = cur.offset(1)
    }
    tmp = cur;
    while *tmp as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *tmp as std::os::raw::c_int >= '0' as i32 &&
              *tmp as std::os::raw::c_int <= '9' as i32 {
        i += 1;
        tmp = tmp.offset(1);
        ret += 1
    }
    if i > 24 as std::os::raw::c_int { *str = tmp; return -(1 as std::os::raw::c_int) }
    while i > 16 as std::os::raw::c_int {
        let fresh6 = cur;
        cur = cur.offset(1);
        hi =
            hi.wrapping_mul(10 as std::os::raw::c_int as
                                std::os::raw::c_ulong).wrapping_add((*fresh6 as
                                                                 std::os::raw::c_int -
                                                                 '0' as i32)
                                                                as
                                                                std::os::raw::c_ulong);
        i -= 1
    }
    while i > 8 as std::os::raw::c_int {
        let fresh7 = cur;
        cur = cur.offset(1);
        mi =
            mi.wrapping_mul(10 as std::os::raw::c_int as
                                std::os::raw::c_ulong).wrapping_add((*fresh7 as
                                                                 std::os::raw::c_int -
                                                                 '0' as i32)
                                                                as
                                                                std::os::raw::c_ulong);
        i -= 1
    }
    while i > 0 as std::os::raw::c_int {
        let fresh8 = cur;
        cur = cur.offset(1);
        lo =
            lo.wrapping_mul(10 as std::os::raw::c_int as
                                std::os::raw::c_ulong).wrapping_add((*fresh8 as
                                                                 std::os::raw::c_int -
                                                                 '0' as i32)
                                                                as
                                                                std::os::raw::c_ulong);
        i -= 1
    }
    *str = cur;
    *llo = lo;
    *lmi = mi;
    *lhi = hi;
    return ret;
}
/* *
 * xmlSchemaValAtomicType:
 * @type: the predefined type
 * @value: the value to check
 * @val:  the return computed value
 * @node:  the node containing the value
 * flags:  flags to control the vlidation
 *
 * Check that a value conforms to the lexical space of the atomic type.
 * if true a value is computed and returned in @val.
 * This checks the value space for list types as well (IDREFS, NMTOKENS).
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
unsafe extern "C" fn xmlSchemaValAtomicType(mut type_0: xmlSchemaTypePtr,
                                            mut value: *const xmlChar,
                                            mut val: *mut xmlSchemaValPtr,
                                            mut node: xmlNodePtr,
                                            mut flags: std::os::raw::c_int,
                                            mut ws:
                                                xmlSchemaWhitespaceValueType,
                                            mut normOnTheFly: std::os::raw::c_int,
                                            mut applyNorm: std::os::raw::c_int,
                                            mut createStringValue:
                                                std::os::raw::c_int) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut v: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut norm: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if xmlSchemaTypesInitialized == 0 as std::os::raw::c_int { xmlSchemaInitTypes(); }
    if type_0.is_null() { return -(1 as std::os::raw::c_int) }
    /*
     * validating a non existant text node is similar to validating
     * an empty one.
     */
    if value.is_null() {
        value = b"\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar
    }
    if !val.is_null() { *val = 0 as xmlSchemaValPtr }
    if flags == 0 as std::os::raw::c_int && !value.is_null() {
        if (*type_0).builtInType != XML_SCHEMAS_STRING as std::os::raw::c_int &&
               (*type_0).builtInType != XML_SCHEMAS_ANYTYPE as std::os::raw::c_int &&
               (*type_0).builtInType !=
                   XML_SCHEMAS_ANYSIMPLETYPE as std::os::raw::c_int {
            if (*type_0).builtInType == XML_SCHEMAS_NORMSTRING as std::os::raw::c_int
               {
                norm = xmlSchemaWhiteSpaceReplace(value)
            } else { norm = xmlSchemaCollapseString(value) }
            if !norm.is_null() { value = norm }
        }
    }
    match (*type_0).builtInType {
        0 => { current_block = 11903844685369979367; }
        45 | 46 => {
            if createStringValue != 0 && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_ANYSIMPLETYPE);
                if !v.is_null() {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 7696918058364574881;
                } else { current_block = 11903844685369979367; }
            } else { current_block = 7696918058364574881; }
        }
        1 => {
            if normOnTheFly == 0 {
                let mut cur: *const xmlChar = value;
                if ws as std::os::raw::c_uint ==
                       XML_SCHEMA_WHITESPACE_REPLACE as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    loop  {
                        if !(*cur as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                            current_block = 1345366029464561491;
                            break ;
                        }
                        if *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                               *cur as std::os::raw::c_int == 0xa as std::os::raw::c_int ||
                               *cur as std::os::raw::c_int == 0x9 as std::os::raw::c_int {
                            current_block = 15441416072758698702;
                            break ;
                        }
                        cur = cur.offset(1)
                    }
                } else if ws as std::os::raw::c_uint ==
                              XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                    loop  {
                        if !(*cur as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                            current_block = 1345366029464561491;
                            break ;
                        }
                        if *cur as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                               *cur as std::os::raw::c_int == 0xa as std::os::raw::c_int ||
                               *cur as std::os::raw::c_int == 0x9 as std::os::raw::c_int {
                            current_block = 15441416072758698702;
                            break ;
                        }
                        if *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int {
                            cur = cur.offset(1);
                            if *cur as std::os::raw::c_int == 0x20 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                                break ;
                            }
                        } else { cur = cur.offset(1) }
                    }
                } else { current_block = 1345366029464561491; }
            } else { current_block = 1345366029464561491; }
            match current_block {
                15441416072758698702 => { }
                _ => {
                    if createStringValue != 0 && !val.is_null() {
                        if applyNorm != 0 {
                            if ws as std::os::raw::c_uint ==
                                   XML_SCHEMA_WHITESPACE_COLLAPSE as
                                       std::os::raw::c_int as std::os::raw::c_uint {
                                norm = xmlSchemaCollapseString(value)
                            } else if ws as std::os::raw::c_uint ==
                                          XML_SCHEMA_WHITESPACE_REPLACE as
                                              std::os::raw::c_int as std::os::raw::c_uint {
                                norm = xmlSchemaWhiteSpaceReplace(value)
                            }
                            if !norm.is_null() { value = norm }
                        }
                        v = xmlSchemaNewValue(XML_SCHEMAS_STRING);
                        if !v.is_null() {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7696918058364574881;
                        } else { current_block = 11903844685369979367; }
                    } else { current_block = 7696918058364574881; }
                }
            }
        }
        2 => {
            if normOnTheFly != 0 {
                if applyNorm != 0 {
                    if ws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        norm = xmlSchemaCollapseString(value)
                    } else { norm = xmlSchemaWhiteSpaceReplace(value) }
                    if !norm.is_null() { value = norm }
                }
                current_block = 10435735846551762309;
            } else {
                let mut cur_0: *const xmlChar = value;
                loop  {
                    if !(*cur_0 as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                        current_block = 10435735846551762309;
                        break ;
                    }
                    if *cur_0 as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                           *cur_0 as std::os::raw::c_int == 0xa as std::os::raw::c_int ||
                           *cur_0 as std::os::raw::c_int == 0x9 as std::os::raw::c_int {
                        current_block = 15441416072758698702;
                        break ;
                    }
                    cur_0 = cur_0.offset(1)
                }
            }
            match current_block {
                15441416072758698702 => { }
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_NORMSTRING);
                        if !v.is_null() {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7696918058364574881;
                        } else { current_block = 11903844685369979367; }
                    } else { current_block = 7696918058364574881; }
                }
            }
        }
        3 => {
            let mut cur_1: *const xmlChar = value;
            let mut len: std::os::raw::c_uint = 0;
            let mut neg: std::os::raw::c_uint = 0;
            let mut integ: std::os::raw::c_uint = 0;
            let mut hasLeadingZeroes: std::os::raw::c_uint = 0;
            let mut cval: [xmlChar; 25] = [0; 25];
            let mut cptr: *mut xmlChar = cval.as_mut_ptr();
            if cur_1.is_null() || *cur_1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                current_block = 15441416072758698702;
            } else {
                /*
		* xs:decimal has a whitespace-facet value of 'collapse'.
		*/
                if normOnTheFly != 0 {
                    while *cur_1 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *cur_1 as std::os::raw::c_int &&
                                  *cur_1 as std::os::raw::c_int <= 0xa as std::os::raw::c_int
                              || *cur_1 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        cur_1 = cur_1.offset(1)
                    }
                }
                /*
		* First we handle an optional sign.
		*/
                neg = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                if *cur_1 as std::os::raw::c_int == '-' as i32 {
                    neg = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                    cur_1 = cur_1.offset(1)
                } else if *cur_1 as std::os::raw::c_int == '+' as i32 {
                    cur_1 = cur_1.offset(1)
                }
                /*
		* Disallow: "", "-", "- "
		*/
                if *cur_1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else {
                    /*
		 * Next we "pre-parse" the number, in preparation for calling
		 * the common routine xmlSchemaParseUInt.  We get rid of any
		 * leading zeroes (because we have reserved only 25 chars),
		 * and note the position of a decimal point.
		 */
                    len = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    integ = !(0 as std::os::raw::c_uint);
                    hasLeadingZeroes = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    /*
		* Skip leading zeroes.
		*/
                    while *cur_1 as std::os::raw::c_int == '0' as i32 {
                        cur_1 = cur_1.offset(1);
                        hasLeadingZeroes = 1 as std::os::raw::c_int as std::os::raw::c_uint
                    }
                    if *cur_1 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                        loop  {
                            if *cur_1 as std::os::raw::c_int >= '0' as i32 &&
                                   *cur_1 as std::os::raw::c_int <= '9' as i32 {
                                let fresh9 = cur_1;
                                cur_1 = cur_1.offset(1);
                                let fresh10 = cptr;
                                cptr = cptr.offset(1);
                                *fresh10 = *fresh9;
                                len = len.wrapping_add(1);
                                if !(len < 24 as std::os::raw::c_int as std::os::raw::c_uint)
                                   {
                                    current_block = 1069630499025798221;
                                    break ;
                                }
                            } else {
                                if !(*cur_1 as std::os::raw::c_int == '.' as i32) {
                                    current_block = 1069630499025798221;
                                    break ;
                                }
                                cur_1 = cur_1.offset(1);
                                integ = len;
                                while *cur_1 as std::os::raw::c_int >= '0' as i32 &&
                                          *cur_1 as std::os::raw::c_int <= '9' as i32
                                      {
                                    let fresh11 = cur_1;
                                    cur_1 = cur_1.offset(1);
                                    let fresh12 = cptr;
                                    cptr = cptr.offset(1);
                                    *fresh12 = *fresh11;
                                    len = len.wrapping_add(1);
                                    if !(len <
                                             24 as std::os::raw::c_int as
                                                 std::os::raw::c_uint) {
                                        break ;
                                    }
                                }
                                /*
			    * Disallow "." but allow "00."
			    */
                                if len == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                                       hasLeadingZeroes == 0 {
                                    current_block =
                                        15441416072758698702; /* error if any extraneous chars */
                                    break ;
                                } else {
                                    current_block = 1069630499025798221;
                                    break ;
                                }
                            }
                        }
                    } else { current_block = 1069630499025798221; }
                    match current_block {
                        15441416072758698702 => { }
                        _ => {
                            if normOnTheFly != 0 {
                                while *cur_1 as std::os::raw::c_int ==
                                          0x20 as std::os::raw::c_int ||
                                          0x9 as std::os::raw::c_int <=
                                              *cur_1 as std::os::raw::c_int &&
                                              *cur_1 as std::os::raw::c_int <=
                                                  0xa as std::os::raw::c_int ||
                                          *cur_1 as std::os::raw::c_int ==
                                              0xd as std::os::raw::c_int {
                                    cur_1 = cur_1.offset(1)
                                }
                            }
                            if *cur_1 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else {
                                if !val.is_null() {
                                    v =
                                        xmlSchemaNewValue(XML_SCHEMAS_DECIMAL);
                                    if !v.is_null() {
                                        /*
			* Now evaluate the significant digits of the number
			*/
                                        if len !=
                                               0 as std::os::raw::c_int as
                                                   std::os::raw::c_uint {
                                            if integ != !(0 as std::os::raw::c_uint) {
                                                /*
				* Get rid of trailing zeroes in the
				* fractional part.
				*/
                                                while len != integ &&
                                                          *cptr.offset(-(1 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             isize))
                                                              as std::os::raw::c_int
                                                              == '0' as i32 {
                                                    cptr = cptr.offset(-1);
                                                    len = len.wrapping_sub(1)
                                                }
                                            }
                                            /*
			    * Terminate the (preparsed) string.
			    */
                                            if len !=
                                                   0 as std::os::raw::c_int as
                                                       std::os::raw::c_uint {
                                                *cptr =
                                                    0 as std::os::raw::c_int as
                                                        xmlChar;
                                                cptr = cval.as_mut_ptr();
                                                xmlSchemaParseUInt(&mut cptr
                                                                       as
                                                                       *mut *mut xmlChar
                                                                       as
                                                                       *mut *const xmlChar,
                                                                   &mut (*v).value.decimal.lo,
                                                                   &mut (*v).value.decimal.mi,
                                                                   &mut (*v).value.decimal.hi);
                                            }
                                        }
                                        /*
			* Set the total digits to 1 if a zero value.
			*/
                                        (*v).value.decimal.set_sign(neg);
                                        if len ==
                                               0 as std::os::raw::c_int as
                                                   std::os::raw::c_uint {
                                            /* Speedup for zero values. */
                                            (*v).value.decimal.set_total(1 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_uint)
                                        } else {
                                            (*v).value.decimal.set_total(len);
                                            if integ == !(0 as std::os::raw::c_uint) {
                                                (*v).value.decimal.set_frac(0
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint)
                                            } else {
                                                (*v).value.decimal.set_frac(len.wrapping_sub(integ))
                                            }
                                        }
                                        *val = v
                                    }
                                }
                                current_block = 7696918058364574881;
                            }
                        }
                    }
                }
            }
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
            ret =
                xmlSchemaValidateDates((*type_0).builtInType as
                                           xmlSchemaValType, value, val,
                                       normOnTheFly);
            current_block = 8487402319155624291;
        }
        12 => {
            ret = xmlSchemaValidateDuration(type_0, value, val, normOnTheFly);
            current_block = 8487402319155624291;
        }
        13 | 14 => {
            let mut cur_2: *const xmlChar = value;
            let mut neg_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut digits_before: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut digits_after: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if normOnTheFly != 0 {
                while *cur_2 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <= *cur_2 as std::os::raw::c_int &&
                              *cur_2 as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                          *cur_2 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    cur_2 = cur_2.offset(1)
                }
            }
            if *cur_2.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   'N' as i32 &&
                   *cur_2.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'a' as i32 &&
                   *cur_2.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'N' as i32 {
                cur_2 = cur_2.offset(3 as std::os::raw::c_int as isize);
                if *cur_2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else if !val.is_null() {
                    if type_0 == xmlSchemaTypeFloatDef {
                        v = xmlSchemaNewValue(XML_SCHEMAS_FLOAT);
                        if !v.is_null() {
                            (*v).value.f = xmlXPathNAN as std::os::raw::c_float;
                            current_block = 3414715837273685534;
                        } else {
                            xmlSchemaFreeValue(v);
                            current_block = 11903844685369979367;
                        }
                    } else {
                        v = xmlSchemaNewValue(XML_SCHEMAS_DOUBLE);
                        if !v.is_null() {
                            (*v).value.d = xmlXPathNAN;
                            current_block = 3414715837273685534;
                        } else {
                            xmlSchemaFreeValue(v);
                            current_block = 11903844685369979367;
                        }
                    }
                    match current_block {
                        11903844685369979367 => { }
                        _ => {
                            *val = v;
                            current_block = 7696918058364574881;
                        }
                    }
                } else { current_block = 7696918058364574881; }
            } else {
                if *cur_2 as std::os::raw::c_int == '-' as i32 {
                    neg_0 = 1 as std::os::raw::c_int;
                    cur_2 = cur_2.offset(1)
                }
                if *cur_2.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       'I' as i32 &&
                       *cur_2.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 'N' as i32 &&
                       *cur_2.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 'F' as i32 {
                    cur_2 = cur_2.offset(3 as std::os::raw::c_int as isize);
                    if *cur_2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                        current_block = 15441416072758698702;
                    } else if !val.is_null() {
                        if type_0 == xmlSchemaTypeFloatDef {
                            v = xmlSchemaNewValue(XML_SCHEMAS_FLOAT);
                            if !v.is_null() {
                                if neg_0 != 0 {
                                    (*v).value.f =
                                        xmlXPathNINF as std::os::raw::c_float
                                } else {
                                    (*v).value.f =
                                        xmlXPathPINF as std::os::raw::c_float
                                }
                                current_block = 16718638665978159145;
                            } else {
                                xmlSchemaFreeValue(v);
                                current_block = 11903844685369979367;
                            }
                        } else {
                            v = xmlSchemaNewValue(XML_SCHEMAS_DOUBLE);
                            if !v.is_null() {
                                if neg_0 != 0 {
                                    (*v).value.d = xmlXPathNINF
                                } else { (*v).value.d = xmlXPathPINF }
                                current_block = 16718638665978159145;
                            } else {
                                xmlSchemaFreeValue(v);
                                current_block = 11903844685369979367;
                            }
                        }
                        match current_block {
                            11903844685369979367 => { }
                            _ => {
                                *val = v;
                                current_block = 7696918058364574881;
                            }
                        }
                    } else { current_block = 7696918058364574881; }
                } else {
                    if neg_0 == 0 as std::os::raw::c_int &&
                           *cur_2 as std::os::raw::c_int == '+' as i32 {
                        cur_2 = cur_2.offset(1)
                    }
                    if *cur_2.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                           == 0 as std::os::raw::c_int ||
                           *cur_2.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == '+' as i32 ||
                           *cur_2.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == '-' as i32 {
                        current_block = 15441416072758698702;
                    } else {
                        while *cur_2 as std::os::raw::c_int >= '0' as i32 &&
                                  *cur_2 as std::os::raw::c_int <= '9' as i32 {
                            cur_2 = cur_2.offset(1);
                            digits_before += 1
                        }
                        if *cur_2 as std::os::raw::c_int == '.' as i32 {
                            cur_2 = cur_2.offset(1);
                            while *cur_2 as std::os::raw::c_int >= '0' as i32 &&
                                      *cur_2 as std::os::raw::c_int <= '9' as i32 {
                                cur_2 = cur_2.offset(1);
                                digits_after += 1
                            }
                        }
                        if digits_before == 0 as std::os::raw::c_int &&
                               digits_after == 0 as std::os::raw::c_int {
                            current_block = 15441416072758698702;
                        } else {
                            if *cur_2 as std::os::raw::c_int == 'e' as i32 ||
                                   *cur_2 as std::os::raw::c_int == 'E' as i32 {
                                cur_2 = cur_2.offset(1);
                                if *cur_2 as std::os::raw::c_int == '-' as i32 ||
                                       *cur_2 as std::os::raw::c_int == '+' as i32 {
                                    cur_2 = cur_2.offset(1)
                                }
                                while *cur_2 as std::os::raw::c_int >= '0' as i32 &&
                                          *cur_2 as std::os::raw::c_int <= '9' as i32
                                      {
                                    cur_2 = cur_2.offset(1)
                                }
                            }
                            if normOnTheFly != 0 {
                                while *cur_2 as std::os::raw::c_int ==
                                          0x20 as std::os::raw::c_int ||
                                          0x9 as std::os::raw::c_int <=
                                              *cur_2 as std::os::raw::c_int &&
                                              *cur_2 as std::os::raw::c_int <=
                                                  0xa as std::os::raw::c_int ||
                                          *cur_2 as std::os::raw::c_int ==
                                              0xd as std::os::raw::c_int {
                                    cur_2 = cur_2.offset(1)
                                }
                            }
                            if *cur_2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else if !val.is_null() {
                                if type_0 == xmlSchemaTypeFloatDef {
                                    v = xmlSchemaNewValue(XML_SCHEMAS_FLOAT);
                                    if !v.is_null() {
                                        /*
			    * TODO: sscanf seems not to give the correct
			    * value for extremely high/low values.
			    * E.g. "1E-149" results in zero.
			    */
                                        if sscanf(value as
                                                      *const std::os::raw::c_char,
                                                  b"%f\x00" as *const u8 as
                                                      *const std::os::raw::c_char,
                                                  &mut (*v).value.f as
                                                      *mut std::os::raw::c_float) ==
                                               1 as std::os::raw::c_int {
                                            *val = v;
                                            current_block =
                                                7696918058364574881;
                                        } else {
                                            xmlSchemaFreeValue(v);
                                            current_block =
                                                15441416072758698702;
                                        }
                                    } else {
                                        current_block = 11903844685369979367;
                                    }
                                } else {
                                    v = xmlSchemaNewValue(XML_SCHEMAS_DOUBLE);
                                    if !v.is_null() {
                                        /*
			    * TODO: sscanf seems not to give the correct
			    * value for extremely high/low values.
			    */
                                        if sscanf(value as
                                                      *const std::os::raw::c_char,
                                                  b"%lf\x00" as *const u8 as
                                                      *const std::os::raw::c_char,
                                                  &mut (*v).value.d as
                                                      *mut std::os::raw::c_double) ==
                                               1 as std::os::raw::c_int {
                                            *val = v;
                                            current_block =
                                                7696918058364574881;
                                        } else {
                                            xmlSchemaFreeValue(v);
                                            current_block =
                                                15441416072758698702;
                                        }
                                    } else {
                                        current_block = 11903844685369979367;
                                    }
                                }
                            } else { current_block = 7696918058364574881; }
                        }
                    }
                }
            }
        }
        15 => {
            let mut cur_3: *const xmlChar = value;
            if normOnTheFly != 0 {
                while *cur_3 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                          0x9 as std::os::raw::c_int <= *cur_3 as std::os::raw::c_int &&
                              *cur_3 as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                          *cur_3 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                    cur_3 = cur_3.offset(1)
                }
                if *cur_3 as std::os::raw::c_int == '0' as i32 {
                    ret = 0 as std::os::raw::c_int;
                    cur_3 = cur_3.offset(1);
                    current_block = 717878598772063298;
                } else if *cur_3 as std::os::raw::c_int == '1' as i32 {
                    ret = 1 as std::os::raw::c_int;
                    cur_3 = cur_3.offset(1);
                    current_block = 717878598772063298;
                } else if *cur_3 as std::os::raw::c_int == 't' as i32 {
                    cur_3 = cur_3.offset(1);
                    let fresh13 = cur_3;
                    cur_3 = cur_3.offset(1);
                    if *fresh13 as std::os::raw::c_int == 'r' as i32 &&
                           {
                               let fresh14 = cur_3;
                               cur_3 = cur_3.offset(1);
                               (*fresh14 as std::os::raw::c_int) == 'u' as i32
                           } &&
                           {
                               let fresh15 = cur_3;
                               cur_3 = cur_3.offset(1);
                               (*fresh15 as std::os::raw::c_int) == 'e' as i32
                           } {
                        ret = 1 as std::os::raw::c_int;
                        current_block = 717878598772063298;
                    } else { current_block = 15441416072758698702; }
                } else if *cur_3 as std::os::raw::c_int == 'f' as i32 {
                    cur_3 = cur_3.offset(1);
                    let fresh16 = cur_3;
                    cur_3 = cur_3.offset(1);
                    if *fresh16 as std::os::raw::c_int == 'a' as i32 &&
                           {
                               let fresh17 = cur_3;
                               cur_3 = cur_3.offset(1);
                               (*fresh17 as std::os::raw::c_int) == 'l' as i32
                           } &&
                           {
                               let fresh18 = cur_3;
                               cur_3 = cur_3.offset(1);
                               (*fresh18 as std::os::raw::c_int) == 's' as i32
                           } &&
                           {
                               let fresh19 = cur_3;
                               cur_3 = cur_3.offset(1);
                               (*fresh19 as std::os::raw::c_int) == 'e' as i32
                           } {
                        ret = 0 as std::os::raw::c_int;
                        current_block = 717878598772063298;
                    } else { current_block = 15441416072758698702; }
                } else { current_block = 15441416072758698702; }
                match current_block {
                    15441416072758698702 => { }
                    _ => {
                        if *cur_3 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                            while *cur_3 as std::os::raw::c_int == 0x20 as std::os::raw::c_int
                                      ||
                                      0x9 as std::os::raw::c_int <=
                                          *cur_3 as std::os::raw::c_int &&
                                          *cur_3 as std::os::raw::c_int <=
                                              0xa as std::os::raw::c_int ||
                                      *cur_3 as std::os::raw::c_int ==
                                          0xd as std::os::raw::c_int {
                                cur_3 = cur_3.offset(1)
                            }
                            if *cur_3 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else { current_block = 17689994068582603570; }
                        } else { current_block = 17689994068582603570; }
                    }
                }
            } else if *cur_3.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                          == '0' as i32 &&
                          *cur_3.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 0 as std::os::raw::c_int {
                ret = 0 as std::os::raw::c_int;
                current_block = 17689994068582603570;
            } else if *cur_3.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                          == '1' as i32 &&
                          *cur_3.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 0 as std::os::raw::c_int {
                ret = 1 as std::os::raw::c_int;
                current_block = 17689994068582603570;
            } else if *cur_3.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                          == 't' as i32 &&
                          *cur_3.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'r' as i32 &&
                          *cur_3.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'u' as i32 &&
                          *cur_3.offset(3 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'e' as i32 &&
                          *cur_3.offset(4 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 0 as std::os::raw::c_int {
                ret = 1 as std::os::raw::c_int;
                current_block = 17689994068582603570;
            } else if *cur_3.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                          == 'f' as i32 &&
                          *cur_3.offset(1 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'a' as i32 &&
                          *cur_3.offset(2 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'l' as i32 &&
                          *cur_3.offset(3 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 's' as i32 &&
                          *cur_3.offset(4 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 'e' as i32 &&
                          *cur_3.offset(5 as std::os::raw::c_int as isize) as
                              std::os::raw::c_int == 0 as std::os::raw::c_int {
                ret = 0 as std::os::raw::c_int;
                current_block = 17689994068582603570;
            } else { current_block = 15441416072758698702; }
            match current_block {
                15441416072758698702 => { }
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_BOOLEAN);
                        if !v.is_null() {
                            (*v).value.b = ret;
                            *val = v;
                            current_block = 7696918058364574881;
                        } else { current_block = 11903844685369979367; }
                    } else { current_block = 7696918058364574881; }
                }
            }
        }
        16 => {
            let mut cur_4: *const xmlChar = value;
            if normOnTheFly == 0 {
                loop  {
                    if !(*cur_4 as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                        current_block = 10878888195156817878;
                        break ;
                    }
                    if *cur_4 as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                           *cur_4 as std::os::raw::c_int == 0xa as std::os::raw::c_int ||
                           *cur_4 as std::os::raw::c_int == 0x9 as std::os::raw::c_int {
                        current_block = 15441416072758698702;
                        break ;
                    }
                    if *cur_4 as std::os::raw::c_int == ' ' as i32 {
                        cur_4 = cur_4.offset(1);
                        if *cur_4 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                            current_block = 15441416072758698702;
                            break ;
                        }
                        if *cur_4 as std::os::raw::c_int == ' ' as i32 {
                            current_block = 15441416072758698702;
                            break ;
                        }
                    } else { cur_4 = cur_4.offset(1) }
                }
            } else { current_block = 10878888195156817878; }
            match current_block {
                15441416072758698702 => { }
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_TOKEN);
                        if !v.is_null() {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7696918058364574881;
                        } else { current_block = 11903844685369979367; }
                    } else { current_block = 7696918058364574881; }
                }
            }
        }
        17 => {
            if normOnTheFly != 0 {
                norm = xmlSchemaCollapseString(value);
                if !norm.is_null() { value = norm }
            }
            if xmlCheckLanguageID(value) == 1 as std::os::raw::c_int {
                if !val.is_null() {
                    v = xmlSchemaNewValue(XML_SCHEMAS_LANGUAGE);
                    if !v.is_null() {
                        (*v).value.str_0 = xmlStrdup(value);
                        *val = v;
                        current_block = 7696918058364574881;
                    } else { current_block = 11903844685369979367; }
                } else { current_block = 7696918058364574881; }
            } else { current_block = 15441416072758698702; }
        }
        18 => {
            if xmlValidateNMToken(value, 1 as std::os::raw::c_int) == 0 as std::os::raw::c_int
               {
                if !val.is_null() {
                    v = xmlSchemaNewValue(XML_SCHEMAS_NMTOKEN);
                    if !v.is_null() {
                        (*v).value.str_0 = xmlStrdup(value);
                        *val = v;
                        current_block = 7696918058364574881;
                    } else { current_block = 11903844685369979367; }
                } else { current_block = 7696918058364574881; }
            } else { current_block = 15441416072758698702; }
        }
        19 => {
            ret =
                xmlSchemaValAtomicListNode(xmlSchemaTypeNmtokenDef, value,
                                           val, node);
            if ret > 0 as std::os::raw::c_int {
                ret = 0 as std::os::raw::c_int
            } else { ret = 1 as std::os::raw::c_int }
            current_block = 8487402319155624291;
        }
        20 => {
            ret = xmlValidateName(value, 1 as std::os::raw::c_int);
            if ret == 0 as std::os::raw::c_int && !val.is_null() && !value.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_NAME);
                if !v.is_null() {
                    let mut start: *const xmlChar = value;
                    let mut end: *const xmlChar = 0 as *const xmlChar;
                    while *start as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *start as std::os::raw::c_int &&
                                  *start as std::os::raw::c_int <= 0xa as std::os::raw::c_int
                              || *start as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        start = start.offset(1)
                    }
                    end = start;
                    while *end as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                              !(*end as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                    0x9 as std::os::raw::c_int <= *end as std::os::raw::c_int
                                        &&
                                        *end as std::os::raw::c_int <=
                                            0xa as std::os::raw::c_int ||
                                    *end as std::os::raw::c_int == 0xd as std::os::raw::c_int)
                          {
                        end = end.offset(1)
                    }
                    (*v).value.str_0 =
                        xmlStrndup(start,
                                   end.offset_from(start) as
                                       std::os::raw::c_long as std::os::raw::c_int);
                    *val = v;
                    current_block = 8487402319155624291;
                } else { current_block = 11903844685369979367; }
            } else { current_block = 8487402319155624291; }
        }
        21 => {
            let mut uri: *const xmlChar = 0 as *const xmlChar;
            let mut local: *mut xmlChar = 0 as *mut xmlChar;
            ret = xmlValidateQName(value, 1 as std::os::raw::c_int);
            if ret != 0 as std::os::raw::c_int {
                current_block = 8487402319155624291;
            } else {
                if !node.is_null() {
                    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
                    local = xmlSplitQName2(value, &mut prefix);
                    ns = xmlSearchNs((*node).doc, node, prefix);
                    if ns.is_null() && !prefix.is_null() {
                        xmlFree.expect("non-null function pointer")(prefix as
                                                                        *mut std::os::raw::c_void);
                        if !local.is_null() {
                            xmlFree.expect("non-null function pointer")(local
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                        current_block = 15441416072758698702;
                    } else {
                        if !ns.is_null() { uri = (*ns).href }
                        if !prefix.is_null() {
                            xmlFree.expect("non-null function pointer")(prefix
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        }
                        current_block = 9032728402242067558;
                    }
                } else { current_block = 9032728402242067558; }
                match current_block {
                    15441416072758698702 => { }
                    _ => {
                        if !val.is_null() {
                            v = xmlSchemaNewValue(XML_SCHEMAS_QNAME);
                            if v.is_null() {
                                if !local.is_null() {
                                    xmlFree.expect("non-null function pointer")(local
                                                                                    as
                                                                                    *mut std::os::raw::c_void);
                                }
                                current_block = 11903844685369979367;
                            } else {
                                if !local.is_null() {
                                    (*v).value.qname.name = local
                                } else {
                                    (*v).value.qname.name = xmlStrdup(value)
                                }
                                if !uri.is_null() {
                                    (*v).value.qname.uri = xmlStrdup(uri)
                                }
                                *val = v;
                                current_block = 8487402319155624291;
                            }
                        } else {
                            if !local.is_null() {
                                xmlFree.expect("non-null function pointer")(local
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            }
                            current_block = 8487402319155624291;
                        }
                    }
                }
            }
        }
        22 => {
            ret = xmlValidateNCName(value, 1 as std::os::raw::c_int);
            if ret == 0 as std::os::raw::c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_NCNAME);
                if !v.is_null() {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 8487402319155624291;
                } else { current_block = 11903844685369979367; }
            } else { current_block = 8487402319155624291; }
        }
        23 => {
            ret = xmlValidateNCName(value, 1 as std::os::raw::c_int);
            if ret == 0 as std::os::raw::c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_ID);
                if !v.is_null() {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 4076512631241092329;
                } else { current_block = 11903844685369979367; }
            } else { current_block = 4076512631241092329; }
            match current_block {
                11903844685369979367 => { }
                _ => {
                    if ret == 0 as std::os::raw::c_int && !node.is_null() &&
                           (*node).type_0 as std::os::raw::c_uint ==
                               XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        let mut attr: xmlAttrPtr = node as xmlAttrPtr;
                        /*
                 * NOTE: the IDness might have already be declared in the DTD
                 */
                        if (*attr).atype as std::os::raw::c_uint !=
                               XML_ATTRIBUTE_ID as std::os::raw::c_int as std::os::raw::c_uint
                           {
                            let mut res: xmlIDPtr = 0 as *mut xmlID;
                            let mut strip: *mut xmlChar = 0 as *mut xmlChar;
                            strip = xmlSchemaStrip(value);
                            if !strip.is_null() {
                                res =
                                    xmlAddID(0 as xmlValidCtxtPtr,
                                             (*node).doc, strip, attr);
                                xmlFree.expect("non-null function pointer")(strip
                                                                                as
                                                                                *mut std::os::raw::c_void);
                            } else {
                                res =
                                    xmlAddID(0 as xmlValidCtxtPtr,
                                             (*node).doc, value, attr)
                            }
                            if res.is_null() {
                                ret = 2 as std::os::raw::c_int
                            } else { (*attr).atype = XML_ATTRIBUTE_ID }
                        }
                    }
                    current_block = 8487402319155624291;
                }
            }
        }
        24 => {
            ret = xmlValidateNCName(value, 1 as std::os::raw::c_int);
            if ret == 0 as std::os::raw::c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_IDREF);
                if v.is_null() {
                    current_block = 11903844685369979367;
                } else {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 11900320504231061096;
                }
            } else { current_block = 11900320504231061096; }
            match current_block {
                11903844685369979367 => { }
                _ => {
                    if ret == 0 as std::os::raw::c_int && !node.is_null() &&
                           (*node).type_0 as std::os::raw::c_uint ==
                               XML_ATTRIBUTE_NODE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                        let mut attr_0: xmlAttrPtr = node as xmlAttrPtr;
                        let mut strip_0: *mut xmlChar = 0 as *mut xmlChar;
                        strip_0 = xmlSchemaStrip(value);
                        if !strip_0.is_null() {
                            xmlAddRef(0 as xmlValidCtxtPtr, (*node).doc,
                                      strip_0, attr_0);
                            xmlFree.expect("non-null function pointer")(strip_0
                                                                            as
                                                                            *mut std::os::raw::c_void);
                        } else {
                            xmlAddRef(0 as xmlValidCtxtPtr, (*node).doc,
                                      value, attr_0);
                        }
                        (*attr_0).atype = XML_ATTRIBUTE_IDREF
                    }
                    current_block = 8487402319155624291;
                }
            }
        }
        25 => {
            ret =
                xmlSchemaValAtomicListNode(xmlSchemaTypeIdrefDef, value, val,
                                           node);
            if ret < 0 as std::os::raw::c_int {
                ret = 2 as std::os::raw::c_int
            } else { ret = 0 as std::os::raw::c_int }
            if ret == 0 as std::os::raw::c_int && !node.is_null() &&
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                let mut attr_1: xmlAttrPtr = node as xmlAttrPtr;
                (*attr_1).atype = XML_ATTRIBUTE_IDREFS
            }
            current_block = 8487402319155624291;
        }
        26 => {
            let mut strip_1: *mut xmlChar = 0 as *mut xmlChar;
            ret = xmlValidateNCName(value, 1 as std::os::raw::c_int);
            if node.is_null() || (*node).doc.is_null() {
                ret = 3 as std::os::raw::c_int
            }
            if ret == 0 as std::os::raw::c_int {
                let mut ent: xmlEntityPtr = 0 as *mut xmlEntity;
                strip_1 = xmlSchemaStrip(value);
                if !strip_1.is_null() {
                    ent = xmlGetDocEntity((*node).doc, strip_1);
                    xmlFree.expect("non-null function pointer")(strip_1 as
                                                                    *mut std::os::raw::c_void);
                } else { ent = xmlGetDocEntity((*node).doc, value) }
                if ent.is_null() ||
                       (*ent).etype as std::os::raw::c_uint !=
                           XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as std::os::raw::c_int
                               as std::os::raw::c_uint {
                    ret = 4 as std::os::raw::c_int
                }
            }
            if ret == 0 as std::os::raw::c_int && !val.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Unimplemented block at %s:%d\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           b"xmlschemastypes.c\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           2845
                                                                               as
                                                                               std::os::raw::c_int);
            }
            if ret == 0 as std::os::raw::c_int && !node.is_null() &&
                   (*node).type_0 as std::os::raw::c_uint ==
                       XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                let mut attr_2: xmlAttrPtr = node as xmlAttrPtr;
                (*attr_2).atype = XML_ATTRIBUTE_ENTITY
            }
            current_block = 8487402319155624291;
        }
        27 => {
            if node.is_null() || (*node).doc.is_null() {
                if !norm.is_null() {
                    xmlFree.expect("non-null function pointer")(norm as
                                                                    *mut std::os::raw::c_void);
                }
                return 3 as std::os::raw::c_int
            } else {
                ret =
                    xmlSchemaValAtomicListNode(xmlSchemaTypeEntityDef, value,
                                               val, node);
                if ret <= 0 as std::os::raw::c_int {
                    ret = 1 as std::os::raw::c_int
                } else { ret = 0 as std::os::raw::c_int }
                if ret == 0 as std::os::raw::c_int && !node.is_null() &&
                       (*node).type_0 as std::os::raw::c_uint ==
                           XML_ATTRIBUTE_NODE as std::os::raw::c_int as std::os::raw::c_uint {
                    let mut attr_3: xmlAttrPtr = node as xmlAttrPtr;
                    (*attr_3).atype = XML_ATTRIBUTE_ENTITIES
                }
            }
            current_block = 8487402319155624291;
        }
        28 => {
            let mut uri_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut local_0: *mut xmlChar = 0 as *mut xmlChar;
            ret = xmlValidateQName(value, 1 as std::os::raw::c_int);
            if ret == 0 as std::os::raw::c_int && !node.is_null() {
                let mut prefix_0: *mut xmlChar = 0 as *mut xmlChar;
                local_0 = xmlSplitQName2(value, &mut prefix_0);
                if !prefix_0.is_null() {
                    let mut ns_0: xmlNsPtr = 0 as *mut xmlNs;
                    ns_0 = xmlSearchNs((*node).doc, node, prefix_0);
                    if ns_0.is_null() {
                        ret = 1 as std::os::raw::c_int
                    } else if !val.is_null() {
                        uri_0 = xmlStrdup((*ns_0).href)
                    }
                }
                if !local_0.is_null() &&
                       (val.is_null() || ret != 0 as std::os::raw::c_int) {
                    xmlFree.expect("non-null function pointer")(local_0 as
                                                                    *mut std::os::raw::c_void);
                }
                if !prefix_0.is_null() {
                    xmlFree.expect("non-null function pointer")(prefix_0 as
                                                                    *mut std::os::raw::c_void);
                }
            }
            if node.is_null() || (*node).doc.is_null() {
                ret = 3 as std::os::raw::c_int
            }
            if ret == 0 as std::os::raw::c_int {
                ret =
                    xmlValidateNotationUse(0 as xmlValidCtxtPtr, (*node).doc,
                                           value);
                if ret == 1 as std::os::raw::c_int {
                    ret = 0 as std::os::raw::c_int
                } else { ret = 1 as std::os::raw::c_int }
            }
            if ret == 0 as std::os::raw::c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_NOTATION);
                if !v.is_null() {
                    if !local_0.is_null() {
                        (*v).value.qname.name = local_0
                    } else { (*v).value.qname.name = xmlStrdup(value) }
                    if !uri_0.is_null() { (*v).value.qname.uri = uri_0 }
                    *val = v;
                    current_block = 8487402319155624291;
                } else {
                    if !local_0.is_null() {
                        xmlFree.expect("non-null function pointer")(local_0 as
                                                                        *mut std::os::raw::c_void);
                    }
                    if !uri_0.is_null() {
                        xmlFree.expect("non-null function pointer")(uri_0 as
                                                                        *mut std::os::raw::c_void);
                    }
                    current_block = 11903844685369979367;
                }
            } else { current_block = 8487402319155624291; }
        }
        29 => {
            if *value as std::os::raw::c_int != 0 as std::os::raw::c_int {
                let mut uri_1: xmlURIPtr = 0 as *mut xmlURI;
                let mut tmpval: *mut xmlChar = 0 as *mut xmlChar;
                let mut cur_5: *mut xmlChar = 0 as *mut xmlChar;
                if normOnTheFly != 0 {
                    norm = xmlSchemaCollapseString(value);
                    if !norm.is_null() { value = norm }
                }
                tmpval = xmlStrdup(value);
                cur_5 = tmpval;
                while *cur_5 != 0 {
                    if (*cur_5 as std::os::raw::c_int) < 32 as std::os::raw::c_int ||
                           *cur_5 as std::os::raw::c_int >= 127 as std::os::raw::c_int ||
                           *cur_5 as std::os::raw::c_int == ' ' as i32 ||
                           *cur_5 as std::os::raw::c_int == '<' as i32 ||
                           *cur_5 as std::os::raw::c_int == '>' as i32 ||
                           *cur_5 as std::os::raw::c_int == '\"' as i32 ||
                           *cur_5 as std::os::raw::c_int == '{' as i32 ||
                           *cur_5 as std::os::raw::c_int == '}' as i32 ||
                           *cur_5 as std::os::raw::c_int == '|' as i32 ||
                           *cur_5 as std::os::raw::c_int == '\\' as i32 ||
                           *cur_5 as std::os::raw::c_int == '^' as i32 ||
                           *cur_5 as std::os::raw::c_int == '`' as i32 ||
                           *cur_5 as std::os::raw::c_int == '\'' as i32 {
                        *cur_5 = '_' as i32 as xmlChar
                    }
                    cur_5 = cur_5.offset(1)
                }
                uri_1 = xmlParseURI(tmpval as *const std::os::raw::c_char);
                xmlFree.expect("non-null function pointer")(tmpval as
                                                                *mut std::os::raw::c_void);
                if uri_1.is_null() {
                    current_block = 15441416072758698702;
                } else {
                    xmlFreeURI(uri_1);
                    current_block = 2980441442721982033;
                }
            } else { current_block = 2980441442721982033; }
            match current_block {
                15441416072758698702 => { }
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_ANYURI);
                        if v.is_null() {
                            current_block = 11903844685369979367;
                        } else {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7696918058364574881;
                        }
                    } else { current_block = 7696918058364574881; }
                }
            }
        }
        43 => {
            let mut cur_6: *const xmlChar = value;
            let mut start_0: *const xmlChar = 0 as *const xmlChar;
            let mut base: *mut xmlChar = 0 as *mut xmlChar;
            let mut total: std::os::raw::c_int = 0;
            let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if cur_6.is_null() {
                current_block = 15441416072758698702;
            } else {
                if normOnTheFly != 0 {
                    while *cur_6 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *cur_6 as std::os::raw::c_int &&
                                  *cur_6 as std::os::raw::c_int <= 0xa as std::os::raw::c_int
                              || *cur_6 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        cur_6 = cur_6.offset(1)
                    }
                }
                start_0 = cur_6;
                while *cur_6 as std::os::raw::c_int >= '0' as i32 &&
                          *cur_6 as std::os::raw::c_int <= '9' as i32 ||
                          *cur_6 as std::os::raw::c_int >= 'A' as i32 &&
                              *cur_6 as std::os::raw::c_int <= 'F' as i32 ||
                          *cur_6 as std::os::raw::c_int >= 'a' as i32 &&
                              *cur_6 as std::os::raw::c_int <= 'f' as i32 {
                    i += 1;
                    cur_6 = cur_6.offset(1)
                }
                if normOnTheFly != 0 {
                    while *cur_6 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *cur_6 as std::os::raw::c_int &&
                                  *cur_6 as std::os::raw::c_int <= 0xa as std::os::raw::c_int
                              || *cur_6 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        cur_6 = cur_6.offset(1)
                    }
                }
                if *cur_6 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else if i % 2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else if !val.is_null() {
                    v = xmlSchemaNewValue(XML_SCHEMAS_HEXBINARY);
                    if v.is_null() {
                        current_block = 11903844685369979367;
                    } else {
                        /*
		    * Copy only the normalized piece.
		    * CRITICAL TODO: Check this.
		    */
                        cur_6 = xmlStrndup(start_0, i); /* number of octets */
                        if cur_6.is_null() {
                            xmlSchemaTypeErrMemory(node,
                                                   b"allocating hexbin data\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char);
                            xmlFree.expect("non-null function pointer")(v as
                                                                            *mut std::os::raw::c_void);
                            current_block = 15441416072758698702;
                        } else {
                            total = i / 2 as std::os::raw::c_int;
                            base = cur_6 as *mut xmlChar;
                            loop  {
                                let fresh20 = i;
                                i = i - 1;
                                if !(fresh20 > 0 as std::os::raw::c_int) { break ; }
                                if *base as std::os::raw::c_int >= 'a' as i32 {
                                    *base =
                                        (*base as std::os::raw::c_int -
                                             ('a' as i32 - 'A' as i32)) as
                                            xmlChar
                                }
                                base = base.offset(1)
                            }
                            (*v).value.hex.str_0 = cur_6 as *mut xmlChar;
                            (*v).value.hex.total = total as std::os::raw::c_uint;
                            *val = v;
                            current_block = 7696918058364574881;
                        }
                    }
                } else { current_block = 7696918058364574881; }
            }
        }
        44 => {
            /* ISSUE:
                 *
                 * Ignore all stray characters? (yes, currently)
                 * Worry about long lines? (no, currently)
                 *
                 * rfc2045.txt:
                 *
                 * "The encoded output stream must be represented in lines of
                 * no more than 76 characters each.  All line breaks or other
                 * characters not found in Table 1 must be ignored by decoding
                 * software.  In base64 data, characters other than those in
                 * Table 1, line breaks, and other white space probably
                 * indicate a transmission error, about which a warning
                 * message or even a message rejection might be appropriate
                 * under some circumstances." */
            let mut cur_7: *const xmlChar = value;
            let mut base_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut total_0: std::os::raw::c_int = 0;
            let mut i_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
            let mut pad: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if cur_7.is_null() {
                current_block = 15441416072758698702;
            } else {
                while *cur_7 != 0 {
                    let mut decc: std::os::raw::c_int = 0;
                    decc = _xmlSchemaBase64Decode(*cur_7);
                    if !(decc < 0 as std::os::raw::c_int) {
                        if !(decc < 64 as std::os::raw::c_int) { break ; }
                        i_0 += 1
                    }
                    cur_7 = cur_7.offset(1)
                }
                loop  {
                    if !(*cur_7 != 0) {
                        current_block = 14484578172259868768;
                        break ;
                    }
                    let mut decc_0: std::os::raw::c_int = 0;
                    decc_0 = _xmlSchemaBase64Decode(*cur_7);
                    if !(decc_0 < 0 as std::os::raw::c_int) {
                        if decc_0 < 64 as std::os::raw::c_int {
                            current_block = 15441416072758698702;
                            break ;
                        }
                    }
                    if decc_0 == 64 as std::os::raw::c_int { pad += 1 }
                    cur_7 = cur_7.offset(1)
                }
                match current_block {
                    15441416072758698702 => { }
                    _ => {
                        /* rfc2045.txt: "Special processing is performed if fewer than
                 * 24 bits are available at the end of the data being encoded.
                 * A full encoding quantum is always completed at the end of a
                 * body.  When fewer than 24 input bits are available in an
                 * input group, zero bits are added (on the right) to form an
                 * integral number of 6-bit groups.  Padding at the end of the
                 * data is performed using the "=" character.  Since all
                 * base64 input is an integral number of octets, only the
                 * following cases can arise: (1) the final quantum of
                 * encoding input is an integral multiple of 24 bits; here,
                 * the final unit of encoded output will be an integral
                 * multiple ofindent: Standard input:701: Warning:old style
		 * assignment ambiguity in "=*".  Assuming "= *" 4 characters
		 * with no "=" padding, (2) the final
                 * quantum of encoding input is exactly 8 bits; here, the
                 * final unit of encoded output will be two characters
                 * followed by two "=" padding characters, or (3) the final
                 * quantum of encoding input is exactly 16 bits; here, the
                 * final unit of encoded output will be three characters
                 * followed by one "=" padding character." */
                        total_0 = 3 as std::os::raw::c_int * (i_0 / 4 as std::os::raw::c_int);
                        if pad == 0 as std::os::raw::c_int {
                            if i_0 % 4 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else { current_block = 5682333073795846871; }
                        } else if pad == 1 as std::os::raw::c_int {
                            let mut decc_1: std::os::raw::c_int = 0;
                            if i_0 % 4 as std::os::raw::c_int != 3 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else {
                                decc_1 = _xmlSchemaBase64Decode(*cur_7);
                                while decc_1 < 0 as std::os::raw::c_int ||
                                          decc_1 > 63 as std::os::raw::c_int {
                                    cur_7 = cur_7.offset(-1);
                                    decc_1 = _xmlSchemaBase64Decode(*cur_7)
                                }
                                /* 16bits in 24bits means 2 pad bits: nnnnnn nnmmmm mmmm00*/
                    /* 00111100 -> 0x3c */
                                if decc_1 & !(0x3c as std::os::raw::c_int) != 0 {
                                    current_block = 15441416072758698702;
                                } else {
                                    total_0 += 2 as std::os::raw::c_int;
                                    current_block = 5682333073795846871;
                                }
                            }
                        } else if pad == 2 as std::os::raw::c_int {
                            let mut decc_2: std::os::raw::c_int = 0;
                            if i_0 % 4 as std::os::raw::c_int != 2 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else {
                                decc_2 = _xmlSchemaBase64Decode(*cur_7);
                                while decc_2 < 0 as std::os::raw::c_int ||
                                          decc_2 > 63 as std::os::raw::c_int {
                                    cur_7 = cur_7.offset(-1);
                                    decc_2 = _xmlSchemaBase64Decode(*cur_7)
                                }
                                /* 8bits in 12bits means 4 pad bits: nnnnnn nn0000 */
                    /* 00110000 -> 0x30 */
                                if decc_2 & !(0x30 as std::os::raw::c_int) != 0 {
                                    current_block = 15441416072758698702;
                                } else {
                                    total_0 += 1 as std::os::raw::c_int;
                                    current_block = 5682333073795846871;
                                }
                            }
                        } else { current_block = 15441416072758698702; }
                        match current_block {
                            15441416072758698702 => { }
                            _ => {
                                if !val.is_null() {
                                    v =
                                        xmlSchemaNewValue(XML_SCHEMAS_BASE64BINARY);
                                    if v.is_null() {
                                        current_block = 11903844685369979367;
                                    } else {
                                        base_0 =
                                            xmlMallocAtomic.expect("non-null function pointer")(((i_0
                                                                                                      +
                                                                                                      pad
                                                                                                      +
                                                                                                      1
                                                                                                          as
                                                                                                          std::os::raw::c_int)
                                                                                                     as
                                                                                                     std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<xmlChar>()
                                                                                                                                     as
                                                                                                                                     std::os::raw::c_ulong))
                                                as *mut xmlChar;
                                        if base_0.is_null() {
                                            xmlSchemaTypeErrMemory(node,
                                                                   b"allocating base64 data\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const std::os::raw::c_char);
                                            xmlFree.expect("non-null function pointer")(v
                                                                                            as
                                                                                            *mut std::os::raw::c_void);
                                            current_block =
                                                15441416072758698702;
                                        } else {
                                            (*v).value.base64.str_0 = base_0;
                                            cur_7 = value;
                                            while *cur_7 != 0 {
                                                if _xmlSchemaBase64Decode(*cur_7)
                                                       >= 0 as std::os::raw::c_int {
                                                    *base_0 = *cur_7;
                                                    base_0 = base_0.offset(1)
                                                }
                                                cur_7 = cur_7.offset(1)
                                            }
                                            *base_0 =
                                                0 as std::os::raw::c_int as xmlChar;
                                            (*v).value.base64.total =
                                                total_0 as std::os::raw::c_uint;
                                            *val = v;
                                            current_block =
                                                7696918058364574881;
                                        }
                                    }
                                } else {
                                    current_block = 7696918058364574881;
                                }
                            }
                        }
                    }
                }
            }
        }
        30 | 34 | 31 | 32 | 33 => {
            let mut cur_8: *const xmlChar = value;
            let mut lo: std::os::raw::c_ulong = 0;
            let mut mi: std::os::raw::c_ulong = 0;
            let mut hi: std::os::raw::c_ulong = 0;
            let mut sign: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if cur_8.is_null() {
                current_block = 15441416072758698702;
            } else {
                if normOnTheFly != 0 {
                    while *cur_8 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                              0x9 as std::os::raw::c_int <= *cur_8 as std::os::raw::c_int &&
                                  *cur_8 as std::os::raw::c_int <= 0xa as std::os::raw::c_int
                              || *cur_8 as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                        cur_8 = cur_8.offset(1)
                    }
                }
                if *cur_8 as std::os::raw::c_int == '-' as i32 {
                    sign = 1 as std::os::raw::c_int;
                    cur_8 = cur_8.offset(1)
                } else if *cur_8 as std::os::raw::c_int == '+' as i32 {
                    cur_8 = cur_8.offset(1)
                }
                ret =
                    xmlSchemaParseUInt(&mut cur_8, &mut lo, &mut mi, &mut hi);
                if ret < 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else {
                    if normOnTheFly != 0 {
                        while *cur_8 as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                  0x9 as std::os::raw::c_int <= *cur_8 as std::os::raw::c_int
                                      &&
                                      *cur_8 as std::os::raw::c_int <=
                                          0xa as std::os::raw::c_int ||
                                  *cur_8 as std::os::raw::c_int == 0xd as std::os::raw::c_int
                              {
                            cur_8 = cur_8.offset(1)
                        }
                    }
                    if *cur_8 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                        current_block = 15441416072758698702;
                    } else {
                        if (*type_0).builtInType ==
                               XML_SCHEMAS_NPINTEGER as std::os::raw::c_int {
                            if sign == 0 as std::os::raw::c_int &&
                                   (hi != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                                        mi !=
                                            0 as std::os::raw::c_int as std::os::raw::c_ulong
                                        ||
                                        lo !=
                                            0 as std::os::raw::c_int as std::os::raw::c_ulong)
                               {
                                current_block = 15441416072758698702;
                            } else { current_block = 10529234500244145779; }
                        } else if (*type_0).builtInType ==
                                      XML_SCHEMAS_PINTEGER as std::os::raw::c_int {
                            if sign == 1 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else if hi == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                                          &&
                                          mi ==
                                              0 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong &&
                                          lo ==
                                              0 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else { current_block = 10529234500244145779; }
                        } else if (*type_0).builtInType ==
                                      XML_SCHEMAS_NINTEGER as std::os::raw::c_int {
                            if sign == 0 as std::os::raw::c_int {
                                current_block = 15441416072758698702;
                            } else if hi == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                                          &&
                                          mi ==
                                              0 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong &&
                                          lo ==
                                              0 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else { current_block = 10529234500244145779; }
                        } else if (*type_0).builtInType ==
                                      XML_SCHEMAS_NNINTEGER as std::os::raw::c_int {
                            if sign == 1 as std::os::raw::c_int &&
                                   (hi != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                                        mi !=
                                            0 as std::os::raw::c_int as std::os::raw::c_ulong
                                        ||
                                        lo !=
                                            0 as std::os::raw::c_int as std::os::raw::c_ulong)
                               {
                                current_block = 15441416072758698702;
                            } else { current_block = 10529234500244145779; }
                        } else { current_block = 10529234500244145779; }
                        match current_block {
                            15441416072758698702 => { }
                            _ => {
                                if !val.is_null() {
                                    v =
                                        xmlSchemaNewValue((*type_0).builtInType
                                                              as
                                                              xmlSchemaValType);
                                    if !v.is_null() {
                                        if ret == 0 as std::os::raw::c_int {
                                            ret += 1
                                        }
                                        (*v).value.decimal.lo = lo;
                                        (*v).value.decimal.mi = mi;
                                        (*v).value.decimal.hi = hi;
                                        (*v).value.decimal.set_sign(sign as
                                                                        std::os::raw::c_uint);
                                        (*v).value.decimal.set_frac(0 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_uint);
                                        (*v).value.decimal.set_total(ret as
                                                                         std::os::raw::c_uint);
                                        *val = v
                                    }
                                }
                                current_block = 7696918058364574881;
                            }
                        }
                    }
                }
            }
        }
        37 | 41 | 39 | 35 => {
            let mut cur_9: *const xmlChar = value;
            let mut lo_0: std::os::raw::c_ulong = 0;
            let mut mi_0: std::os::raw::c_ulong = 0;
            let mut hi_0: std::os::raw::c_ulong = 0;
            let mut sign_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if cur_9.is_null() {
                current_block = 15441416072758698702;
            } else {
                if *cur_9 as std::os::raw::c_int == '-' as i32 {
                    sign_0 = 1 as std::os::raw::c_int;
                    cur_9 = cur_9.offset(1)
                } else if *cur_9 as std::os::raw::c_int == '+' as i32 {
                    cur_9 = cur_9.offset(1)
                }
                ret =
                    xmlSchemaParseUInt(&mut cur_9, &mut lo_0, &mut mi_0,
                                       &mut hi_0);
                if ret < 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else if *cur_9 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else {
                    if (*type_0).builtInType ==
                           XML_SCHEMAS_LONG as std::os::raw::c_int {
                        if hi_0 >= 922 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if hi_0 > 922 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else if mi_0 >=
                                          33720368 as std::os::raw::c_int as
                                              std::os::raw::c_ulong {
                                if mi_0 >
                                       33720368 as std::os::raw::c_int as
                                           std::os::raw::c_ulong {
                                    current_block = 15441416072758698702;
                                } else if sign_0 == 0 as std::os::raw::c_int &&
                                              lo_0 >
                                                  54775807 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong {
                                    current_block = 15441416072758698702;
                                } else if sign_0 == 1 as std::os::raw::c_int &&
                                              lo_0 >
                                                  54775808 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong {
                                    current_block = 15441416072758698702;
                                } else {
                                    current_block = 3636633399307766178;
                                }
                            } else { current_block = 3636633399307766178; }
                        } else { current_block = 3636633399307766178; }
                    } else if (*type_0).builtInType ==
                                  XML_SCHEMAS_INT as std::os::raw::c_int {
                        if hi_0 != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if mi_0 >= 21 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if mi_0 > 21 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else if sign_0 == 0 as std::os::raw::c_int &&
                                          lo_0 >
                                              47483647 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else if sign_0 == 1 as std::os::raw::c_int &&
                                          lo_0 >
                                              47483648 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else { current_block = 3636633399307766178; }
                        } else { current_block = 3636633399307766178; }
                    } else if (*type_0).builtInType ==
                                  XML_SCHEMAS_SHORT as std::os::raw::c_int {
                        if mi_0 != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                               hi_0 != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if sign_0 == 1 as std::os::raw::c_int &&
                                      lo_0 >
                                          32768 as std::os::raw::c_int as
                                              std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if sign_0 == 0 as std::os::raw::c_int &&
                                      lo_0 >
                                          32767 as std::os::raw::c_int as
                                              std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else { current_block = 3636633399307766178; }
                    } else if (*type_0).builtInType ==
                                  XML_SCHEMAS_BYTE as std::os::raw::c_int {
                        if mi_0 != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                               hi_0 != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if sign_0 == 1 as std::os::raw::c_int &&
                                      lo_0 >
                                          128 as std::os::raw::c_int as std::os::raw::c_ulong
                         {
                            current_block = 15441416072758698702;
                        } else if sign_0 == 0 as std::os::raw::c_int &&
                                      lo_0 >
                                          127 as std::os::raw::c_int as std::os::raw::c_ulong
                         {
                            current_block = 15441416072758698702;
                        } else { current_block = 3636633399307766178; }
                    } else { current_block = 3636633399307766178; }
                    match current_block {
                        15441416072758698702 => { }
                        _ => {
                            if !val.is_null() {
                                v =
                                    xmlSchemaNewValue((*type_0).builtInType as
                                                          xmlSchemaValType);
                                if !v.is_null() {
                                    (*v).value.decimal.lo = lo_0;
                                    (*v).value.decimal.mi = mi_0;
                                    (*v).value.decimal.hi = hi_0;
                                    (*v).value.decimal.set_sign(sign_0 as
                                                                    std::os::raw::c_uint);
                                    (*v).value.decimal.set_frac(0 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint);
                                    (*v).value.decimal.set_total(ret as
                                                                     std::os::raw::c_uint);
                                    *val = v
                                }
                            }
                            current_block = 7696918058364574881;
                        }
                    }
                }
            }
        }
        36 | 38 | 40 | 42 => {
            let mut cur_10: *const xmlChar = value;
            let mut lo_1: std::os::raw::c_ulong = 0;
            let mut mi_1: std::os::raw::c_ulong = 0;
            let mut hi_1: std::os::raw::c_ulong = 0;
            if cur_10.is_null() {
                current_block = 15441416072758698702;
            } else {
                ret =
                    xmlSchemaParseUInt(&mut cur_10, &mut lo_1, &mut mi_1,
                                       &mut hi_1);
                if ret < 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else if *cur_10 as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    current_block = 15441416072758698702;
                } else {
                    if (*type_0).builtInType ==
                           XML_SCHEMAS_ULONG as std::os::raw::c_int {
                        if hi_1 >= 1844 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if hi_1 > 1844 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else if mi_1 >=
                                          67440737 as std::os::raw::c_int as
                                              std::os::raw::c_ulong {
                                if mi_1 >
                                       67440737 as std::os::raw::c_int as
                                           std::os::raw::c_ulong {
                                    current_block = 15441416072758698702;
                                } else if lo_1 >
                                              9551615 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong {
                                    current_block = 15441416072758698702;
                                } else {
                                    current_block = 5816501739065458839;
                                }
                            } else { current_block = 5816501739065458839; }
                        } else { current_block = 5816501739065458839; }
                    } else if (*type_0).builtInType ==
                                  XML_SCHEMAS_UINT as std::os::raw::c_int {
                        if hi_1 != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if mi_1 >= 42 as std::os::raw::c_int as std::os::raw::c_ulong {
                            if mi_1 > 42 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else if lo_1 >
                                          94967295 as std::os::raw::c_int as
                                              std::os::raw::c_ulong {
                                current_block = 15441416072758698702;
                            } else { current_block = 5816501739065458839; }
                        } else { current_block = 5816501739065458839; }
                    } else if (*type_0).builtInType ==
                                  XML_SCHEMAS_USHORT as std::os::raw::c_int {
                        if mi_1 != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                               hi_1 != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if lo_1 > 65535 as std::os::raw::c_int as std::os::raw::c_ulong
                         {
                            current_block = 15441416072758698702;
                        } else { current_block = 5816501739065458839; }
                    } else if (*type_0).builtInType ==
                                  XML_SCHEMAS_UBYTE as std::os::raw::c_int {
                        if mi_1 != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                               hi_1 != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else if lo_1 > 255 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 15441416072758698702;
                        } else { current_block = 5816501739065458839; }
                    } else { current_block = 5816501739065458839; }
                    match current_block {
                        15441416072758698702 => { }
                        _ => {
                            if !val.is_null() {
                                v =
                                    xmlSchemaNewValue((*type_0).builtInType as
                                                          xmlSchemaValType);
                                if !v.is_null() {
                                    (*v).value.decimal.lo = lo_1;
                                    (*v).value.decimal.mi = mi_1;
                                    (*v).value.decimal.hi = hi_1;
                                    (*v).value.decimal.set_sign(0 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint);
                                    (*v).value.decimal.set_frac(0 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint);
                                    (*v).value.decimal.set_total(ret as
                                                                     std::os::raw::c_uint);
                                    *val = v
                                }
                            }
                            current_block = 7696918058364574881;
                        }
                    }
                }
            }
        }
        _ => { current_block = 8487402319155624291; }
    }
    match current_block {
        11903844685369979367 => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as
                                                                *mut std::os::raw::c_void);
            }
            return -(1 as std::os::raw::c_int)
        }
        15441416072758698702 => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as
                                                                *mut std::os::raw::c_void);
            }
            return 1 as std::os::raw::c_int
        }
        7696918058364574881 => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as
                                                                *mut std::os::raw::c_void);
            }
            return 0 as std::os::raw::c_int
        }
        _ => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as
                                                                *mut std::os::raw::c_void);
            }
            return ret
        }
    };
}
/* *
 * xmlSchemaValPredefTypeNode:
 * @type: the predefined type
 * @value: the value to check
 * @val:  the return computed value
 * @node:  the node containing the value
 *
 * Check that a value conforms to the lexical space of the predefined type.
 * if true a value is computed and returned in @val.
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValPredefTypeNode(mut type_0:
                                                        xmlSchemaTypePtr,
                                                    mut value: *const xmlChar,
                                                    mut val:
                                                        *mut xmlSchemaValPtr,
                                                    mut node: xmlNodePtr)
 -> std::os::raw::c_int {
    return xmlSchemaValAtomicType(type_0, value, val, node, 0 as std::os::raw::c_int,
                                  XML_SCHEMA_WHITESPACE_UNKNOWN,
                                  1 as std::os::raw::c_int, 1 as std::os::raw::c_int,
                                  0 as std::os::raw::c_int);
}
/* *
 * xmlSchemaValPredefTypeNodeNoNorm:
 * @type: the predefined type
 * @value: the value to check
 * @val:  the return computed value
 * @node:  the node containing the value
 *
 * Check that a value conforms to the lexical space of the predefined type.
 * if true a value is computed and returned in @val.
 * This one does apply any normalization to the value.
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValPredefTypeNodeNoNorm(mut type_0:
                                                              xmlSchemaTypePtr,
                                                          mut value:
                                                              *const xmlChar,
                                                          mut val:
                                                              *mut xmlSchemaValPtr,
                                                          mut node:
                                                              xmlNodePtr)
 -> std::os::raw::c_int {
    return xmlSchemaValAtomicType(type_0, value, val, node, 1 as std::os::raw::c_int,
                                  XML_SCHEMA_WHITESPACE_UNKNOWN,
                                  1 as std::os::raw::c_int, 0 as std::os::raw::c_int,
                                  1 as std::os::raw::c_int);
}
/* *
 * xmlSchemaValidatePredefinedType:
 * @type: the predefined type
 * @value: the value to check
 * @val:  the return computed value
 *
 * Check that a value conforms to the lexical space of the predefined type.
 * if true a value is computed and returned in @val.
 *
 * Returns 0 if this validates, a positive error code number otherwise
 *         and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidatePredefinedType(mut type_0:
                                                             xmlSchemaTypePtr,
                                                         mut value:
                                                             *const xmlChar,
                                                         mut val:
                                                             *mut xmlSchemaValPtr)
 -> std::os::raw::c_int {
    return xmlSchemaValPredefTypeNode(type_0, value, val, 0 as xmlNodePtr);
}
/* *
 * xmlSchemaCompareDecimals:
 * @x:  a first decimal value
 * @y:  a second decimal value
 *
 * Compare 2 decimals
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y and -2 in case of error
 */
unsafe extern "C" fn xmlSchemaCompareDecimals(mut x: xmlSchemaValPtr,
                                              mut y: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    let mut swp: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut order: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut integx: std::os::raw::c_int = 0;
    let mut integy: std::os::raw::c_int = 0;
    let mut dlen: std::os::raw::c_int = 0;
    let mut hi: std::os::raw::c_ulong = 0;
    let mut mi: std::os::raw::c_ulong = 0;
    let mut lo: std::os::raw::c_ulong = 0;
    /*
     * First test: If x is -ve and not zero
     */
    if (*x).value.decimal.sign() as std::os::raw::c_int != 0 &&
           ((*x).value.decimal.lo != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                (*x).value.decimal.mi != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                (*x).value.decimal.hi != 0 as std::os::raw::c_int as std::os::raw::c_ulong) {
        /*
	 * Then if y is -ve and not zero reverse the compare
	 */
        if (*y).value.decimal.sign() as std::os::raw::c_int != 0 &&
               ((*y).value.decimal.lo != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                    (*y).value.decimal.mi != 0 as std::os::raw::c_int as std::os::raw::c_ulong
                    ||
                    (*y).value.decimal.hi !=
                        0 as std::os::raw::c_int as std::os::raw::c_ulong) {
            order = -(1 as std::os::raw::c_int)
        } else {
            /*
	 * Otherwise (y >= 0) we have the answer
	 */
            return -(1 as std::os::raw::c_int)
        }
        /*
     * If x is not -ve and y is -ve we have the answer
     */
    } else if (*y).value.decimal.sign() as std::os::raw::c_int != 0 &&
                  ((*y).value.decimal.lo != 0 as std::os::raw::c_int as std::os::raw::c_ulong
                       ||
                       (*y).value.decimal.mi !=
                           0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                       (*y).value.decimal.hi !=
                           0 as std::os::raw::c_int as std::os::raw::c_ulong) {
        return 1 as std::os::raw::c_int
    }
    /*
     * If it's not simply determined by a difference in sign,
     * then we need to compare the actual values of the two nums.
     * To do this, we start by looking at the integral parts.
     * If the number of integral digits differ, then we have our
     * answer.
     */
    integx =
        (*x).value.decimal.total() as std::os::raw::c_int -
            (*x).value.decimal.frac() as std::os::raw::c_int;
    integy =
        (*y).value.decimal.total() as std::os::raw::c_int -
            (*y).value.decimal.frac() as std::os::raw::c_int;
    /*
    * NOTE: We changed the "total" for values like "0.1"
    *   (or "-0.1" or ".1") to be 1, which was 2 previously.
    *   Therefore the special case, when such values are
    *   compared with 0, needs to be handled separately;
    *   otherwise a zero would be recognized incorrectly as
    *   greater than those values. This has the nice side effect
    *   that we gain an overall optimized comparison with zeroes.
    * Note that a "0" has a "total" of 1 already.
    */
    if integx == 1 as std::os::raw::c_int {
        if (*x).value.decimal.lo == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            if integy != 1 as std::os::raw::c_int {
                return -order
            } else if (*y).value.decimal.lo !=
                          0 as std::os::raw::c_int as std::os::raw::c_ulong {
                return -order
            } else { return 0 as std::os::raw::c_int }
        }
    }
    if integy == 1 as std::os::raw::c_int {
        if (*y).value.decimal.lo == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            if integx != 1 as std::os::raw::c_int {
                return order
            } else if (*x).value.decimal.lo !=
                          0 as std::os::raw::c_int as std::os::raw::c_ulong {
                return order
            } else { return 0 as std::os::raw::c_int }
        }
    }
    if integx > integy {
        return order
    } else { if integy > integx { return -order } }
    /*
     * If the number of integral digits is the same for both numbers,
     * then things get a little more complicated.  We need to "normalize"
     * the numbers in order to properly compare them.  To do this, we
     * look at the total length of each number (length => number of
     * significant digits), and divide the "shorter" by 10 (decreasing
     * the length) until they are of equal length.
     */
    dlen =
        (*x).value.decimal.total() as std::os::raw::c_int -
            (*y).value.decimal.total() as
                std::os::raw::c_int; /* x has more digits than y */
    if dlen < 0 as std::os::raw::c_int {
        /* y has more digits than x */
        swp = x;
        hi = (*y).value.decimal.hi;
        mi = (*y).value.decimal.mi;
        lo = (*y).value.decimal.lo;
        dlen = -dlen;
        order = -order
    } else {
        swp = y;
        hi = (*x).value.decimal.hi;
        mi = (*x).value.decimal.mi;
        lo = (*x).value.decimal.lo
    }
    while dlen > 8 as std::os::raw::c_int {
        /* in effect, right shift by 10**8 */
        lo = mi;
        mi = hi;
        hi = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
        dlen -= 8 as std::os::raw::c_int
    }
    while dlen > 0 as std::os::raw::c_int {
        let mut rem1: std::os::raw::c_ulong = 0;
        let mut rem2: std::os::raw::c_ulong = 0;
        rem1 =
            hi.wrapping_rem(10 as std::os::raw::c_int as
                                std::os::raw::c_ulong).wrapping_mul(100000000 as
                                                                std::os::raw::c_long
                                                                as
                                                                std::os::raw::c_ulong);
        hi = hi.wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_ulong);
        rem2 =
            mi.wrapping_rem(10 as std::os::raw::c_int as
                                std::os::raw::c_ulong).wrapping_mul(100000000 as
                                                                std::os::raw::c_long
                                                                as
                                                                std::os::raw::c_ulong);
        mi =
            mi.wrapping_add(rem1).wrapping_div(10 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
        lo =
            lo.wrapping_add(rem2).wrapping_div(10 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
        dlen -= 1
    }
    if hi > (*swp).value.decimal.hi {
        return order
    } else {
        if hi == (*swp).value.decimal.hi {
            if mi > (*swp).value.decimal.mi {
                return order
            } else {
                if mi == (*swp).value.decimal.mi {
                    if lo > (*swp).value.decimal.lo {
                        return order
                    } else {
                        if lo == (*swp).value.decimal.lo {
                            if (*x).value.decimal.total() as std::os::raw::c_int ==
                                   (*y).value.decimal.total() as std::os::raw::c_int {
                                return 0 as std::os::raw::c_int
                            } else { return order }
                        }
                    }
                }
            }
        }
    }
    return -order;
}
/* *
 * xmlSchemaCompareDurations:
 * @x:  a first duration value
 * @y:  a second duration value
 *
 * Compare 2 durations
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareDurations(mut x: xmlSchemaValPtr,
                                               mut y: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    let mut carry: std::os::raw::c_long = 0;
    let mut mon: std::os::raw::c_long = 0;
    let mut day: std::os::raw::c_long = 0;
    let mut sec: std::os::raw::c_double = 0.;
    let mut invert: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut xmon: std::os::raw::c_long = 0;
    let mut xday: std::os::raw::c_long = 0;
    let mut myear: std::os::raw::c_long = 0;
    let mut minday: std::os::raw::c_long = 0;
    let mut maxday: std::os::raw::c_long = 0;
    static mut dayRange: [[std::os::raw::c_long; 12]; 2] =
        [[0 as std::os::raw::c_int as std::os::raw::c_long, 28 as std::os::raw::c_int as std::os::raw::c_long,
          59 as std::os::raw::c_int as std::os::raw::c_long,
          89 as std::os::raw::c_int as std::os::raw::c_long,
          120 as std::os::raw::c_int as std::os::raw::c_long,
          150 as std::os::raw::c_int as std::os::raw::c_long,
          181 as std::os::raw::c_int as std::os::raw::c_long,
          212 as std::os::raw::c_int as std::os::raw::c_long,
          242 as std::os::raw::c_int as std::os::raw::c_long,
          273 as std::os::raw::c_int as std::os::raw::c_long,
          303 as std::os::raw::c_int as std::os::raw::c_long,
          334 as std::os::raw::c_int as std::os::raw::c_long],
         [0 as std::os::raw::c_int as std::os::raw::c_long, 31 as std::os::raw::c_int as std::os::raw::c_long,
          62 as std::os::raw::c_int as std::os::raw::c_long,
          92 as std::os::raw::c_int as std::os::raw::c_long,
          123 as std::os::raw::c_int as std::os::raw::c_long,
          153 as std::os::raw::c_int as std::os::raw::c_long,
          184 as std::os::raw::c_int as std::os::raw::c_long,
          215 as std::os::raw::c_int as std::os::raw::c_long,
          245 as std::os::raw::c_int as std::os::raw::c_long,
          276 as std::os::raw::c_int as std::os::raw::c_long,
          306 as std::os::raw::c_int as std::os::raw::c_long,
          337 as std::os::raw::c_int as std::os::raw::c_long]];
    if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
    /* months */
    mon = (*x).value.dur.mon - (*y).value.dur.mon;
    /* seconds */
    sec = (*x).value.dur.sec - (*y).value.dur.sec;
    carry =
        (sec /
             (24 as std::os::raw::c_int * (60 as std::os::raw::c_int * 60 as std::os::raw::c_int)) as
                 std::os::raw::c_double) as std::os::raw::c_long;
    sec -=
        carry as std::os::raw::c_double *
            (24 as std::os::raw::c_int * (60 as std::os::raw::c_int * 60 as std::os::raw::c_int)) as
                std::os::raw::c_double;
    /* days */
    day = (*x).value.dur.day - (*y).value.dur.day + carry;
    /* easy test */
    if mon == 0 as std::os::raw::c_int as std::os::raw::c_long {
        if day == 0 as std::os::raw::c_int as std::os::raw::c_long {
            if sec == 0.0f64 {
                return 0 as std::os::raw::c_int
            } else if sec < 0.0f64 {
                return -(1 as std::os::raw::c_int)
            } else { return 1 as std::os::raw::c_int }
        } else if day < 0 as std::os::raw::c_int as std::os::raw::c_long {
            return -(1 as std::os::raw::c_int)
        } else { return 1 as std::os::raw::c_int }
    } /* can this really happen ? */
    if mon > 0 as std::os::raw::c_int as std::os::raw::c_long {
        if day >= 0 as std::os::raw::c_int as std::os::raw::c_long && sec >= 0.0f64 {
            return 1 as std::os::raw::c_int
        } else { xmon = mon; xday = -day }
    } else if day <= 0 as std::os::raw::c_int as std::os::raw::c_long && sec <= 0.0f64 {
        return -(1 as std::os::raw::c_int)
    } else { invert = -(1 as std::os::raw::c_int); xmon = -mon; xday = day }
    myear = xmon / 12 as std::os::raw::c_int as std::os::raw::c_long;
    if myear == 0 as std::os::raw::c_int as std::os::raw::c_long {
        minday = 0 as std::os::raw::c_int as std::os::raw::c_long;
        maxday = 0 as std::os::raw::c_int as std::os::raw::c_long
    } else {
        maxday =
            366 as std::os::raw::c_int as std::os::raw::c_long *
                ((myear + 3 as std::os::raw::c_int as std::os::raw::c_long) /
                     4 as std::os::raw::c_int as std::os::raw::c_long) +
                365 as std::os::raw::c_int as std::os::raw::c_long *
                    ((myear - 1 as std::os::raw::c_int as std::os::raw::c_long) %
                         4 as std::os::raw::c_int as std::os::raw::c_long);
        minday = maxday - 1 as std::os::raw::c_int as std::os::raw::c_long
    }
    xmon = xmon % 12 as std::os::raw::c_int as std::os::raw::c_long;
    minday += dayRange[0 as std::os::raw::c_int as usize][xmon as usize];
    maxday += dayRange[1 as std::os::raw::c_int as usize][xmon as usize];
    if maxday == minday && maxday == xday { return 0 as std::os::raw::c_int }
    if maxday < xday { return -invert }
    if minday > xday { return invert }
    /* indeterminate */
    return 2 as std::os::raw::c_int;
}
/* *
 * xmlSchemaDupVal:
 * @v: the #xmlSchemaValPtr value to duplicate
 *
 * Makes a copy of @v. The calling program is responsible for freeing
 * the returned value.
 *
 * returns a pointer to a duplicated #xmlSchemaValPtr or NULL if error.
 */
unsafe extern "C" fn xmlSchemaDupVal(mut v: xmlSchemaValPtr)
 -> xmlSchemaValPtr {
    let mut ret: xmlSchemaValPtr = xmlSchemaNewValue((*v).type_0);
    if ret.is_null() { return 0 as xmlSchemaValPtr }
    memcpy(ret as *mut std::os::raw::c_void, v as *const std::os::raw::c_void,
           ::std::mem::size_of::<xmlSchemaVal>() as std::os::raw::c_ulong);
    (*ret).next = 0 as *mut _xmlSchemaVal;
    return ret;
}
/* *
 * xmlSchemaCopyValue:
 * @val:  the precomputed value to be copied
 *
 * Copies the precomputed value. This duplicates any string within.
 *
 * Returns the copy or NULL if a copy for a data-type is not implemented.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaCopyValue(mut val: xmlSchemaValPtr)
 -> xmlSchemaValPtr {
    let mut ret: xmlSchemaValPtr = 0 as xmlSchemaValPtr;
    let mut prev: xmlSchemaValPtr = 0 as xmlSchemaValPtr;
    let mut cur: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    /*
    * Copy the string values.
    */
    while !val.is_null() {
        match (*val).type_0 as std::os::raw::c_uint {
            45 | 25 | 27 | 19 => {
                xmlSchemaFreeValue(ret);
                return 0 as xmlSchemaValPtr
            }
            46 | 1 | 2 | 16 | 17 | 20 | 22 | 23 | 24 | 26 | 18 | 29 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.str_0.is_null() {
                    (*cur).value.str_0 = xmlStrdup((*val).value.str_0)
                }
            }
            21 | 28 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.qname.name.is_null() {
                    (*cur).value.qname.name =
                        xmlStrdup((*val).value.qname.name)
                }
                if !(*val).value.qname.uri.is_null() {
                    (*cur).value.qname.uri = xmlStrdup((*val).value.qname.uri)
                }
            }
            43 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.hex.str_0.is_null() {
                    (*cur).value.hex.str_0 = xmlStrdup((*val).value.hex.str_0)
                }
            }
            44 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.base64.str_0.is_null() {
                    (*cur).value.base64.str_0 =
                        xmlStrdup((*val).value.base64.str_0)
                }
            }
            _ => { cur = xmlSchemaDupVal(val) }
        }
        if ret.is_null() { ret = cur } else { (*prev).next = cur }
        prev = cur;
        val = (*val).next
    }
    return ret;
}
/* *
 * _xmlSchemaDateAdd:
 * @dt: an #xmlSchemaValPtr
 * @dur: an #xmlSchemaValPtr of type #XS_DURATION
 *
 * Compute a new date/time from @dt and @dur. This function assumes @dt
 * is either #XML_SCHEMAS_DATETIME, #XML_SCHEMAS_DATE, #XML_SCHEMAS_GYEARMONTH,
 * or #XML_SCHEMAS_GYEAR. The returned #xmlSchemaVal is the same type as
 * @dt. The calling program is responsible for freeing the returned value.
 *
 * Returns a pointer to a new #xmlSchemaVal or NULL if error.
 */
unsafe extern "C" fn _xmlSchemaDateAdd(mut dt: xmlSchemaValPtr,
                                       mut dur: xmlSchemaValPtr)
 -> xmlSchemaValPtr {
    let mut ret: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut tmp: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut carry: std::os::raw::c_long = 0;
    let mut tempdays: std::os::raw::c_long = 0;
    let mut temp: std::os::raw::c_long = 0;
    let mut r: xmlSchemaValDatePtr = 0 as *mut xmlSchemaValDate;
    let mut d: xmlSchemaValDatePtr = 0 as *mut xmlSchemaValDate;
    let mut u: xmlSchemaValDurationPtr = 0 as *mut xmlSchemaValDuration;
    if dt.is_null() || dur.is_null() { return 0 as xmlSchemaValPtr }
    ret = xmlSchemaNewValue((*dt).type_0);
    if ret.is_null() { return 0 as xmlSchemaValPtr }
    /* make a copy so we don't alter the original value */
    tmp = xmlSchemaDupVal(dt);
    if tmp.is_null() { xmlSchemaFreeValue(ret); return 0 as xmlSchemaValPtr }
    r = &mut (*ret).value.date;
    d = &mut (*tmp).value.date;
    u = &mut (*dur).value.dur;
    /* normalization */
    if (*d).mon() as std::os::raw::c_int == 0 as std::os::raw::c_int {
        (*d).set_mon(1 as std::os::raw::c_int as std::os::raw::c_uint)
    }
    /* normalize for time zone offset */
    (*u).sec -= ((*d).tzo() * 60 as std::os::raw::c_int) as std::os::raw::c_double;
    (*d).set_tzo(0 as std::os::raw::c_int);
    /* normalization */
    if (*d).day() as std::os::raw::c_int == 0 as std::os::raw::c_int {
        (*d).set_day(1 as std::os::raw::c_int as std::os::raw::c_uint)
    }
    /* month */
    carry = (*d).mon() as std::os::raw::c_long + (*u).mon;
    (*r).set_mon(((carry - 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_double
                      -
                      floor((carry - 1 as std::os::raw::c_int as std::os::raw::c_long) as
                                std::os::raw::c_double /
                                (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                                    std::os::raw::c_double) *
                          (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                              std::os::raw::c_double +
                      1 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_uint);
    carry =
        floor((carry - 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_double /
                  (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_double) as
            std::os::raw::c_long;
    /* year (may be modified later) */
    (*r).year = (*d).year + carry;
    if (*r).year == 0 as std::os::raw::c_int as std::os::raw::c_long {
        if (*d).year > 0 as std::os::raw::c_int as std::os::raw::c_long {
            (*r).year -= 1
        } else { (*r).year += 1 }
    }
    /* time zone */
    (*r).set_tzo((*d).tzo());
    (*r).set_tz_flag((*d).tz_flag());
    /* seconds */
    (*r).sec = (*d).sec + (*u).sec;
    carry =
        floor((*r).sec as std::os::raw::c_long as std::os::raw::c_double /
                  60 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_long;
    if (*r).sec != 0.0f64 {
        (*r).sec = (*r).sec - floor((*r).sec / 60.0f64) * 60.0f64
    }
    /* minute */
    carry += (*d).min() as std::os::raw::c_long;
    (*r).set_min((carry as std::os::raw::c_double -
                      floor(carry as std::os::raw::c_double /
                                60 as std::os::raw::c_int as std::os::raw::c_double) *
                          60 as std::os::raw::c_int as std::os::raw::c_double) as
                     std::os::raw::c_uint);
    carry =
        floor(carry as std::os::raw::c_double / 60 as std::os::raw::c_int as std::os::raw::c_double)
            as std::os::raw::c_long;
    /* hours */
    carry += (*d).hour() as std::os::raw::c_long;
    (*r).set_hour((carry as std::os::raw::c_double -
                       floor(carry as std::os::raw::c_double /
                                 24 as std::os::raw::c_int as std::os::raw::c_double) *
                           24 as std::os::raw::c_int as std::os::raw::c_double) as
                      std::os::raw::c_uint);
    carry =
        floor(carry as std::os::raw::c_double / 24 as std::os::raw::c_int as std::os::raw::c_double)
            as std::os::raw::c_long;
    /*
     * days
     * Note we use tempdays because the temporary values may need more
     * than 5 bits
     */
    if (*r).year != 0 as std::os::raw::c_int as std::os::raw::c_long &&
           ((*r).mon() as std::os::raw::c_int >= 1 as std::os::raw::c_int &&
                (*r).mon() as std::os::raw::c_int <= 12 as std::os::raw::c_int) &&
           (*d).day() >
               (if (*r).year % 4 as std::os::raw::c_int as std::os::raw::c_long ==
                       0 as std::os::raw::c_int as std::os::raw::c_long &&
                       (*r).year % 100 as std::os::raw::c_int as std::os::raw::c_long !=
                           0 as std::os::raw::c_int as std::os::raw::c_long ||
                       (*r).year % 400 as std::os::raw::c_int as std::os::raw::c_long ==
                           0 as std::os::raw::c_int as std::os::raw::c_long {
                    daysInMonthLeap[((*r).mon() as std::os::raw::c_int -
                                         1 as std::os::raw::c_int) as usize]
                } else {
                    daysInMonth[((*r).mon() as std::os::raw::c_int - 1 as std::os::raw::c_int)
                                    as usize]
                }) {
        tempdays =
            if (*r).year % 4 as std::os::raw::c_int as std::os::raw::c_long ==
                   0 as std::os::raw::c_int as std::os::raw::c_long &&
                   (*r).year % 100 as std::os::raw::c_int as std::os::raw::c_long !=
                       0 as std::os::raw::c_int as std::os::raw::c_long ||
                   (*r).year % 400 as std::os::raw::c_int as std::os::raw::c_long ==
                       0 as std::os::raw::c_int as std::os::raw::c_long {
                daysInMonthLeap[((*r).mon() as std::os::raw::c_int - 1 as std::os::raw::c_int)
                                    as usize]
            } else {
                daysInMonth[((*r).mon() as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                                usize]
            } as std::os::raw::c_long
    } else if ((*d).day() as std::os::raw::c_int) < 1 as std::os::raw::c_int {
        tempdays = 1 as std::os::raw::c_int as std::os::raw::c_long
    } else { tempdays = (*d).day() as std::os::raw::c_long }
    tempdays += (*u).day + carry;
    loop  {
        if tempdays < 1 as std::os::raw::c_int as std::os::raw::c_long {
            let mut tmon: std::os::raw::c_long =
                (((*r).mon() as std::os::raw::c_int - 1 as std::os::raw::c_int -
                      1 as std::os::raw::c_int) as std::os::raw::c_double -
                     floor(((*r).mon() as std::os::raw::c_int - 1 as std::os::raw::c_int -
                                1 as std::os::raw::c_int) as std::os::raw::c_double /
                               (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                                   std::os::raw::c_double) *
                         (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                             std::os::raw::c_double +
                     1 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_long;
            let mut tyr: std::os::raw::c_long =
                (*r).year +
                    floor(((*r).mon() as std::os::raw::c_int - 1 as std::os::raw::c_int -
                               1 as std::os::raw::c_int) as std::os::raw::c_double /
                              (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                                  std::os::raw::c_double) as std::os::raw::c_long;
            if tyr == 0 as std::os::raw::c_int as std::os::raw::c_long { tyr -= 1 }
            /*
	     * Coverity detected an overrun in daysInMonth
	     * of size 12 at position 12 with index variable "((r)->mon - 1)"
	     */
            if tmon < 1 as std::os::raw::c_int as std::os::raw::c_long {
                tmon = 1 as std::os::raw::c_int as std::os::raw::c_long
            }
            if tmon > 12 as std::os::raw::c_int as std::os::raw::c_long {
                tmon = 12 as std::os::raw::c_int as std::os::raw::c_long
            }
            tempdays +=
                if tyr % 4 as std::os::raw::c_int as std::os::raw::c_long ==
                       0 as std::os::raw::c_int as std::os::raw::c_long &&
                       tyr % 100 as std::os::raw::c_int as std::os::raw::c_long !=
                           0 as std::os::raw::c_int as std::os::raw::c_long ||
                       tyr % 400 as std::os::raw::c_int as std::os::raw::c_long ==
                           0 as std::os::raw::c_int as std::os::raw::c_long {
                    daysInMonthLeap[(tmon - 1 as std::os::raw::c_int as std::os::raw::c_long)
                                        as usize]
                } else {
                    daysInMonth[(tmon - 1 as std::os::raw::c_int as std::os::raw::c_long) as
                                    usize]
                } as std::os::raw::c_long;
            carry = -(1 as std::os::raw::c_int) as std::os::raw::c_long
        } else {
            if !((*r).year != 0 as std::os::raw::c_int as std::os::raw::c_long &&
                     ((*r).mon() as std::os::raw::c_int >= 1 as std::os::raw::c_int &&
                          (*r).mon() as std::os::raw::c_int <= 12 as std::os::raw::c_int) &&
                     tempdays >
                         (if (*r).year % 4 as std::os::raw::c_int as std::os::raw::c_long ==
                                 0 as std::os::raw::c_int as std::os::raw::c_long &&
                                 (*r).year %
                                     100 as std::os::raw::c_int as std::os::raw::c_long !=
                                     0 as std::os::raw::c_int as std::os::raw::c_long ||
                                 (*r).year %
                                     400 as std::os::raw::c_int as std::os::raw::c_long ==
                                     0 as std::os::raw::c_int as std::os::raw::c_long {
                              daysInMonthLeap[((*r).mon() as std::os::raw::c_int -
                                                   1 as std::os::raw::c_int) as usize]
                          } else {
                              daysInMonth[((*r).mon() as std::os::raw::c_int -
                                               1 as std::os::raw::c_int) as usize]
                          }) as std::os::raw::c_long) {
                break ;
            }
            tempdays =
                tempdays -
                    (if (*r).year % 4 as std::os::raw::c_int as std::os::raw::c_long ==
                            0 as std::os::raw::c_int as std::os::raw::c_long &&
                            (*r).year % 100 as std::os::raw::c_int as std::os::raw::c_long !=
                                0 as std::os::raw::c_int as std::os::raw::c_long ||
                            (*r).year % 400 as std::os::raw::c_int as std::os::raw::c_long ==
                                0 as std::os::raw::c_int as std::os::raw::c_long {
                         daysInMonthLeap[((*r).mon() as std::os::raw::c_int -
                                              1 as std::os::raw::c_int) as usize]
                     } else {
                         daysInMonth[((*r).mon() as std::os::raw::c_int -
                                          1 as std::os::raw::c_int) as usize]
                     }) as std::os::raw::c_long;
            carry = 1 as std::os::raw::c_int as std::os::raw::c_long
        }
        temp = (*r).mon() as std::os::raw::c_long + carry;
        (*r).set_mon(((temp - 1 as std::os::raw::c_int as std::os::raw::c_long) as
                          std::os::raw::c_double -
                          floor((temp - 1 as std::os::raw::c_int as std::os::raw::c_long) as
                                    std::os::raw::c_double /
                                    (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                                        std::os::raw::c_double) *
                              (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                                  std::os::raw::c_double +
                          1 as std::os::raw::c_int as std::os::raw::c_double) as
                         std::os::raw::c_uint);
        (*r).year =
            (*r).year +
                floor((temp - 1 as std::os::raw::c_int as std::os::raw::c_long) as
                          std::os::raw::c_double /
                          (13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                              std::os::raw::c_double) as std::os::raw::c_uint as std::os::raw::c_long;
        if (*r).year == 0 as std::os::raw::c_int as std::os::raw::c_long {
            if temp < 1 as std::os::raw::c_int as std::os::raw::c_long {
                (*r).year -= 1
            } else { (*r).year += 1 }
        }
    }
    (*r).set_day(tempdays as std::os::raw::c_uint);
    /*
     * adjust the date/time type to the date values
     */
    if (*ret).type_0 as std::os::raw::c_uint !=
           XML_SCHEMAS_DATETIME as std::os::raw::c_int as std::os::raw::c_uint {
        if (*r).hour() as std::os::raw::c_int != 0 || (*r).min() as std::os::raw::c_int != 0
               || (*r).sec != 0. {
            (*ret).type_0 = XML_SCHEMAS_DATETIME
        } else if (*ret).type_0 as std::os::raw::c_uint !=
                      XML_SCHEMAS_DATE as std::os::raw::c_int as std::os::raw::c_uint {
            if (*r).mon() as std::os::raw::c_int != 1 as std::os::raw::c_int &&
                   (*r).day() as std::os::raw::c_int != 1 as std::os::raw::c_int {
                (*ret).type_0 = XML_SCHEMAS_DATE
            } else if (*ret).type_0 as std::os::raw::c_uint !=
                          XML_SCHEMAS_GYEARMONTH as std::os::raw::c_int as
                              std::os::raw::c_uint &&
                          (*r).mon() as std::os::raw::c_int != 1 as std::os::raw::c_int {
                (*ret).type_0 = XML_SCHEMAS_GYEARMONTH
            }
        }
    }
    xmlSchemaFreeValue(tmp);
    return ret;
}
/* *
 * xmlSchemaDateNormalize:
 * @dt: an #xmlSchemaValPtr of a date/time type value.
 * @offset: number of seconds to adjust @dt by.
 *
 * Normalize @dt to GMT time. The @offset parameter is subtracted from
 * the return value is a time-zone offset is present on @dt.
 *
 * Returns a normalized copy of @dt or NULL if error.
 */
unsafe extern "C" fn xmlSchemaDateNormalize(mut dt: xmlSchemaValPtr,
                                            mut offset: std::os::raw::c_double)
 -> xmlSchemaValPtr {
    let mut dur: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut ret: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    if dt.is_null() { return 0 as xmlSchemaValPtr }
    if (*dt).type_0 as std::os::raw::c_uint !=
           XML_SCHEMAS_TIME as std::os::raw::c_int as std::os::raw::c_uint &&
           (*dt).type_0 as std::os::raw::c_uint !=
               XML_SCHEMAS_DATETIME as std::os::raw::c_int as std::os::raw::c_uint &&
           (*dt).type_0 as std::os::raw::c_uint !=
               XML_SCHEMAS_DATE as std::os::raw::c_int as std::os::raw::c_uint ||
           (*dt).value.date.tzo() == 0 as std::os::raw::c_int {
        return xmlSchemaDupVal(dt)
    }
    dur = xmlSchemaNewValue(XML_SCHEMAS_DURATION);
    if dur.is_null() { return 0 as xmlSchemaValPtr }
    (*dur).value.date.sec -= offset;
    ret = _xmlSchemaDateAdd(dt, dur);
    if ret.is_null() { return 0 as xmlSchemaValPtr }
    xmlSchemaFreeValue(dur);
    /* ret->value.date.tzo = 0; */
    return ret;
}
/* *
 * _xmlSchemaDateCastYMToDays:
 * @dt: an #xmlSchemaValPtr
 *
 * Convert mon and year of @dt to total number of days. Take the
 * number of years since (or before) 1 AD and add the number of leap
 * years. This is a function  because negative
 * years must be handled a little differently and there is no zero year.
 *
 * Returns number of days.
 */
unsafe extern "C" fn _xmlSchemaDateCastYMToDays(dt: xmlSchemaValPtr)
 -> std::os::raw::c_long {
    let mut ret: std::os::raw::c_long = 0; /* normalization */
    let mut mon: std::os::raw::c_int = 0;
    mon = (*dt).value.date.mon() as std::os::raw::c_int;
    if mon <= 0 as std::os::raw::c_int { mon = 1 as std::os::raw::c_int }
    if (*dt).value.date.year <= 0 as std::os::raw::c_int as std::os::raw::c_long {
        ret =
            (*dt).value.date.year * 365 as std::os::raw::c_int as std::os::raw::c_long +
                (((*dt).value.date.year + 1 as std::os::raw::c_int as std::os::raw::c_long) /
                     4 as std::os::raw::c_int as std::os::raw::c_long -
                     ((*dt).value.date.year +
                          1 as std::os::raw::c_int as std::os::raw::c_long) /
                         100 as std::os::raw::c_int as std::os::raw::c_long +
                     ((*dt).value.date.year +
                          1 as std::os::raw::c_int as std::os::raw::c_long) /
                         400 as std::os::raw::c_int as std::os::raw::c_long) +
                ((if (*dt).value.date.year % 4 as std::os::raw::c_int as std::os::raw::c_long
                         == 0 as std::os::raw::c_int as std::os::raw::c_long &&
                         (*dt).value.date.year %
                             100 as std::os::raw::c_int as std::os::raw::c_long !=
                             0 as std::os::raw::c_int as std::os::raw::c_long ||
                         (*dt).value.date.year %
                             400 as std::os::raw::c_int as std::os::raw::c_long ==
                             0 as std::os::raw::c_int as std::os::raw::c_long {
                      dayInLeapYearByMonth[(mon - 1 as std::os::raw::c_int) as usize]
                  } else {
                      dayInYearByMonth[(mon - 1 as std::os::raw::c_int) as usize]
                  }) + 0 as std::os::raw::c_int as std::os::raw::c_long)
    } else {
        ret =
            ((*dt).value.date.year - 1 as std::os::raw::c_int as std::os::raw::c_long) *
                365 as std::os::raw::c_int as std::os::raw::c_long +
                (((*dt).value.date.year - 1 as std::os::raw::c_int as std::os::raw::c_long) /
                     4 as std::os::raw::c_int as std::os::raw::c_long -
                     ((*dt).value.date.year -
                          1 as std::os::raw::c_int as std::os::raw::c_long) /
                         100 as std::os::raw::c_int as std::os::raw::c_long +
                     ((*dt).value.date.year -
                          1 as std::os::raw::c_int as std::os::raw::c_long) /
                         400 as std::os::raw::c_int as std::os::raw::c_long) +
                ((if (*dt).value.date.year % 4 as std::os::raw::c_int as std::os::raw::c_long
                         == 0 as std::os::raw::c_int as std::os::raw::c_long &&
                         (*dt).value.date.year %
                             100 as std::os::raw::c_int as std::os::raw::c_long !=
                             0 as std::os::raw::c_int as std::os::raw::c_long ||
                         (*dt).value.date.year %
                             400 as std::os::raw::c_int as std::os::raw::c_long ==
                             0 as std::os::raw::c_int as std::os::raw::c_long {
                      dayInLeapYearByMonth[(mon - 1 as std::os::raw::c_int) as usize]
                  } else {
                      dayInYearByMonth[(mon - 1 as std::os::raw::c_int) as usize]
                  }) + 0 as std::os::raw::c_int as std::os::raw::c_long)
    }
    return ret;
}
/* *
 * xmlSchemaCompareDates:
 * @x:  a first date/time value
 * @y:  a second date/time value
 *
 * Compare 2 date/times
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareDates(mut x: xmlSchemaValPtr,
                                           mut y: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    let mut xmask: std::os::raw::c_uchar = 0;
    let mut ymask: std::os::raw::c_uchar = 0;
    let mut xor_mask: std::os::raw::c_uchar = 0;
    let mut and_mask: std::os::raw::c_uchar = 0;
    let mut p1: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut p2: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut q1: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut q2: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
    let mut p1d: std::os::raw::c_long = 0;
    let mut p2d: std::os::raw::c_long = 0;
    let mut q1d: std::os::raw::c_long = 0;
    let mut q2d: std::os::raw::c_long = 0;
    if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
    if (*x).value.date.tz_flag() != 0 {
        if (*y).value.date.tz_flag() == 0 {
            p1 =
                xmlSchemaDateNormalize(x, 0 as std::os::raw::c_int as std::os::raw::c_double);
            p1d =
                _xmlSchemaDateCastYMToDays(p1) +
                    (*p1).value.date.day() as std::os::raw::c_long;
            /* normalize y + 14:00 */
            q1 =
                xmlSchemaDateNormalize(y,
                                       (14 as std::os::raw::c_int *
                                            (60 as std::os::raw::c_int *
                                                 60 as std::os::raw::c_int)) as
                                           std::os::raw::c_double);
            q1d =
                _xmlSchemaDateCastYMToDays(q1) +
                    (*q1).value.date.day() as std::os::raw::c_long;
            if p1d < q1d {
                xmlSchemaFreeValue(p1);
                xmlSchemaFreeValue(q1);
                return -(1 as std::os::raw::c_int)
            } else {
                if p1d == q1d {
                    let mut sec: std::os::raw::c_double = 0.;
                    sec =
                        ((*p1).value.date.hour() as std::os::raw::c_int *
                             (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                             (*p1).value.date.min() as std::os::raw::c_int *
                                 60 as std::os::raw::c_int +
                             (*p1).value.date.tzo() * 60 as std::os::raw::c_int) as
                            std::os::raw::c_double + (*p1).value.date.sec -
                            (((*q1).value.date.hour() as std::os::raw::c_int *
                                  (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                                  (*q1).value.date.min() as std::os::raw::c_int *
                                      60 as std::os::raw::c_int +
                                  (*q1).value.date.tzo() * 60 as std::os::raw::c_int)
                                 as std::os::raw::c_double + (*q1).value.date.sec);
                    if sec < 0.0f64 {
                        xmlSchemaFreeValue(p1);
                        xmlSchemaFreeValue(q1);
                        return -(1 as std::os::raw::c_int)
                    } else {
                        let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
                        /* normalize y - 14:00 */
                        q2 =
                            xmlSchemaDateNormalize(y,
                                                   -(14 as std::os::raw::c_int *
                                                         (60 as std::os::raw::c_int *
                                                              60 as
                                                                  std::os::raw::c_int))
                                                       as std::os::raw::c_double);
                        q2d =
                            _xmlSchemaDateCastYMToDays(q2) +
                                (*q2).value.date.day() as std::os::raw::c_long;
                        if p1d > q2d {
                            ret = 1 as std::os::raw::c_int
                        } else if p1d == q2d {
                            sec =
                                ((*p1).value.date.hour() as std::os::raw::c_int *
                                     (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                                     (*p1).value.date.min() as std::os::raw::c_int *
                                         60 as std::os::raw::c_int +
                                     (*p1).value.date.tzo() *
                                         60 as std::os::raw::c_int) as std::os::raw::c_double
                                    + (*p1).value.date.sec -
                                    (((*q2).value.date.hour() as std::os::raw::c_int *
                                          (60 as std::os::raw::c_int *
                                               60 as std::os::raw::c_int) +
                                          (*q2).value.date.min() as
                                              std::os::raw::c_int * 60 as std::os::raw::c_int
                                          +
                                          (*q2).value.date.tzo() *
                                              60 as std::os::raw::c_int) as
                                         std::os::raw::c_double +
                                         (*q2).value.date.sec);
                            if sec > 0.0f64 {
                                ret = 1 as std::os::raw::c_int
                            } else { ret = 2 as std::os::raw::c_int }
                            /* indeterminate */
                        }
                        xmlSchemaFreeValue(p1);
                        xmlSchemaFreeValue(q1);
                        xmlSchemaFreeValue(q2);
                        if ret != 0 as std::os::raw::c_int { return ret }
                    }
                } else { xmlSchemaFreeValue(p1); xmlSchemaFreeValue(q1); }
            }
        }
    } else if (*y).value.date.tz_flag() != 0 {
        q1 = xmlSchemaDateNormalize(y, 0 as std::os::raw::c_int as std::os::raw::c_double);
        q1d =
            _xmlSchemaDateCastYMToDays(q1) +
                (*q1).value.date.day() as std::os::raw::c_long;
        /* normalize x - 14:00 */
        p1 =
            xmlSchemaDateNormalize(x,
                                   -(14 as std::os::raw::c_int *
                                         (60 as std::os::raw::c_int *
                                              60 as std::os::raw::c_int)) as
                                       std::os::raw::c_double);
        p1d =
            _xmlSchemaDateCastYMToDays(p1) +
                (*p1).value.date.day() as std::os::raw::c_long;
        if p1d < q1d {
            xmlSchemaFreeValue(p1);
            xmlSchemaFreeValue(q1);
            return -(1 as std::os::raw::c_int)
        } else {
            if p1d == q1d {
                let mut sec_0: std::os::raw::c_double = 0.;
                sec_0 =
                    ((*p1).value.date.hour() as std::os::raw::c_int *
                         (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                         (*p1).value.date.min() as std::os::raw::c_int *
                             60 as std::os::raw::c_int +
                         (*p1).value.date.tzo() * 60 as std::os::raw::c_int) as
                        std::os::raw::c_double + (*p1).value.date.sec -
                        (((*q1).value.date.hour() as std::os::raw::c_int *
                              (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                              (*q1).value.date.min() as std::os::raw::c_int *
                                  60 as std::os::raw::c_int +
                              (*q1).value.date.tzo() * 60 as std::os::raw::c_int) as
                             std::os::raw::c_double + (*q1).value.date.sec);
                if sec_0 < 0.0f64 {
                    xmlSchemaFreeValue(p1);
                    xmlSchemaFreeValue(q1);
                    return -(1 as std::os::raw::c_int)
                } else {
                    let mut ret_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
                    /* normalize x + 14:00 */
                    p2 =
                        xmlSchemaDateNormalize(x,
                                               (14 as std::os::raw::c_int *
                                                    (60 as std::os::raw::c_int *
                                                         60 as std::os::raw::c_int))
                                                   as std::os::raw::c_double);
                    p2d =
                        _xmlSchemaDateCastYMToDays(p2) +
                            (*p2).value.date.day() as std::os::raw::c_long;
                    if p2d > q1d {
                        ret_0 = 1 as std::os::raw::c_int
                    } else if p2d == q1d {
                        sec_0 =
                            ((*p2).value.date.hour() as std::os::raw::c_int *
                                 (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                                 (*p2).value.date.min() as std::os::raw::c_int *
                                     60 as std::os::raw::c_int +
                                 (*p2).value.date.tzo() * 60 as std::os::raw::c_int)
                                as std::os::raw::c_double + (*p2).value.date.sec -
                                (((*q1).value.date.hour() as std::os::raw::c_int *
                                      (60 as std::os::raw::c_int * 60 as std::os::raw::c_int)
                                      +
                                      (*q1).value.date.min() as std::os::raw::c_int *
                                          60 as std::os::raw::c_int +
                                      (*q1).value.date.tzo() *
                                          60 as std::os::raw::c_int) as std::os::raw::c_double
                                     + (*q1).value.date.sec);
                        if sec_0 > 0.0f64 {
                            ret_0 = 1 as std::os::raw::c_int
                        } else { ret_0 = 2 as std::os::raw::c_int }
                        /* indeterminate */
                    }
                    xmlSchemaFreeValue(p1);
                    xmlSchemaFreeValue(q1);
                    xmlSchemaFreeValue(p2);
                    if ret_0 != 0 as std::os::raw::c_int { return ret_0 }
                }
            } else { xmlSchemaFreeValue(p1); xmlSchemaFreeValue(q1); }
        }
    }
    /*
     * if the same type then calculate the difference
     */
    if (*x).type_0 as std::os::raw::c_uint == (*y).type_0 as std::os::raw::c_uint {
        let mut ret_1: std::os::raw::c_int =
            0 as std::os::raw::c_int; /* mark type differences */
        q1 =
            xmlSchemaDateNormalize(y,
                                   0 as std::os::raw::c_int as
                                       std::os::raw::c_double); /* mark field specification */
        q1d =
            _xmlSchemaDateCastYMToDays(q1) +
                (*q1).value.date.day() as std::os::raw::c_long;
        p1 = xmlSchemaDateNormalize(x, 0 as std::os::raw::c_int as std::os::raw::c_double);
        p1d =
            _xmlSchemaDateCastYMToDays(p1) +
                (*p1).value.date.day() as std::os::raw::c_long;
        if p1d < q1d {
            ret_1 = -(1 as std::os::raw::c_int)
        } else if p1d > q1d {
            ret_1 = 1 as std::os::raw::c_int
        } else {
            let mut sec_1: std::os::raw::c_double = 0.;
            sec_1 =
                ((*p1).value.date.hour() as std::os::raw::c_int *
                     (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                     (*p1).value.date.min() as std::os::raw::c_int * 60 as std::os::raw::c_int
                     + (*p1).value.date.tzo() * 60 as std::os::raw::c_int) as
                    std::os::raw::c_double + (*p1).value.date.sec -
                    (((*q1).value.date.hour() as std::os::raw::c_int *
                          (60 as std::os::raw::c_int * 60 as std::os::raw::c_int) +
                          (*q1).value.date.min() as std::os::raw::c_int *
                              60 as std::os::raw::c_int +
                          (*q1).value.date.tzo() * 60 as std::os::raw::c_int) as
                         std::os::raw::c_double + (*q1).value.date.sec);
            if sec_1 < 0.0f64 {
                ret_1 = -(1 as std::os::raw::c_int)
            } else if sec_1 > 0.0f64 { ret_1 = 1 as std::os::raw::c_int }
        }
        xmlSchemaFreeValue(p1);
        xmlSchemaFreeValue(q1);
        return ret_1
    }
    match (*x).type_0 as std::os::raw::c_uint {
        11 => { xmask = 0xf as std::os::raw::c_int as std::os::raw::c_uchar }
        10 => { xmask = 0x7 as std::os::raw::c_int as std::os::raw::c_uchar }
        8 => { xmask = 0x1 as std::os::raw::c_int as std::os::raw::c_uchar }
        6 => { xmask = 0x2 as std::os::raw::c_int as std::os::raw::c_uchar }
        5 => { xmask = 0x3 as std::os::raw::c_int as std::os::raw::c_uchar }
        9 => { xmask = 0x3 as std::os::raw::c_int as std::os::raw::c_uchar }
        7 => { xmask = 0x6 as std::os::raw::c_int as std::os::raw::c_uchar }
        4 => { xmask = 0x8 as std::os::raw::c_int as std::os::raw::c_uchar }
        _ => { xmask = 0 as std::os::raw::c_int as std::os::raw::c_uchar }
    }
    match (*y).type_0 as std::os::raw::c_uint {
        11 => { ymask = 0xf as std::os::raw::c_int as std::os::raw::c_uchar }
        10 => { ymask = 0x7 as std::os::raw::c_int as std::os::raw::c_uchar }
        8 => { ymask = 0x1 as std::os::raw::c_int as std::os::raw::c_uchar }
        6 => { ymask = 0x2 as std::os::raw::c_int as std::os::raw::c_uchar }
        5 => { ymask = 0x3 as std::os::raw::c_int as std::os::raw::c_uchar }
        9 => { ymask = 0x3 as std::os::raw::c_int as std::os::raw::c_uchar }
        7 => { ymask = 0x6 as std::os::raw::c_int as std::os::raw::c_uchar }
        4 => { ymask = 0x8 as std::os::raw::c_int as std::os::raw::c_uchar }
        _ => { ymask = 0 as std::os::raw::c_int as std::os::raw::c_uchar }
    }
    xor_mask = (xmask as std::os::raw::c_int ^ ymask as std::os::raw::c_int) as std::os::raw::c_uchar;
    and_mask = (xmask as std::os::raw::c_int & ymask as std::os::raw::c_int) as std::os::raw::c_uchar;
    /* year */
    if xor_mask as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 {
        return 2 as std::os::raw::c_int
    } else {
        if and_mask as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 {
            if (*x).value.date.year < (*y).value.date.year {
                return -(1 as std::os::raw::c_int)
            } else {
                if (*x).value.date.year > (*y).value.date.year {
                    return 1 as std::os::raw::c_int
                }
            }
        }
    } /* indeterminate */
    /* month */
    if xor_mask as std::os::raw::c_int & 2 as std::os::raw::c_int != 0 {
        return 2 as std::os::raw::c_int
    } else {
        if and_mask as std::os::raw::c_int & 2 as std::os::raw::c_int != 0 {
            if ((*x).value.date.mon() as std::os::raw::c_int) <
                   (*y).value.date.mon() as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            } else {
                if (*x).value.date.mon() as std::os::raw::c_int >
                       (*y).value.date.mon() as std::os::raw::c_int {
                    return 1 as std::os::raw::c_int
                }
            }
        }
    } /* indeterminate */
    /* day */
    if xor_mask as std::os::raw::c_int & 4 as std::os::raw::c_int != 0 {
        return 2 as std::os::raw::c_int
    } else {
        if and_mask as std::os::raw::c_int & 4 as std::os::raw::c_int != 0 {
            if ((*x).value.date.day() as std::os::raw::c_int) <
                   (*y).value.date.day() as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            } else {
                if (*x).value.date.day() as std::os::raw::c_int >
                       (*y).value.date.day() as std::os::raw::c_int {
                    return 1 as std::os::raw::c_int
                }
            }
        }
    } /* indeterminate */
    /* time */
    if xor_mask as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
        return 2 as std::os::raw::c_int
    } else {
        if and_mask as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
            if ((*x).value.date.hour() as std::os::raw::c_int) <
                   (*y).value.date.hour() as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            } else {
                if (*x).value.date.hour() as std::os::raw::c_int >
                       (*y).value.date.hour() as std::os::raw::c_int {
                    return 1 as std::os::raw::c_int
                } else {
                    if ((*x).value.date.min() as std::os::raw::c_int) <
                           (*y).value.date.min() as std::os::raw::c_int {
                        return -(1 as std::os::raw::c_int)
                    } else {
                        if (*x).value.date.min() as std::os::raw::c_int >
                               (*y).value.date.min() as std::os::raw::c_int {
                            return 1 as std::os::raw::c_int
                        } else {
                            if (*x).value.date.sec < (*y).value.date.sec {
                                return -(1 as std::os::raw::c_int)
                            } else {
                                if (*x).value.date.sec > (*y).value.date.sec {
                                    return 1 as std::os::raw::c_int
                                }
                            }
                        }
                    }
                }
            }
        }
    } /* indeterminate */
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaComparePreserveReplaceStrings:
 * @x:  a first string value
 * @y:  a second string value
 * @invert: inverts the result if x < y or x > y.
 *
 * Compare 2 string for their normalized values.
 * @x is a string with whitespace of "preserve", @y is
 * a string with a whitespace of "replace". I.e. @x could
 * be an "xsd:string" and @y an "xsd:normalizedString".
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaComparePreserveReplaceStrings(mut x:
                                                                *const xmlChar,
                                                            mut y:
                                                                *const xmlChar,
                                                            mut invert:
                                                                std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut tmp: std::os::raw::c_int = 0;
    while *x as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *y as std::os::raw::c_int == 0x9 as std::os::raw::c_int ||
               *y as std::os::raw::c_int == 0xa as std::os::raw::c_int ||
               *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if !(*x as std::os::raw::c_int == 0x20 as std::os::raw::c_int) {
                if (*x as std::os::raw::c_int - 0x20 as std::os::raw::c_int) <
                       0 as std::os::raw::c_int {
                    if invert != 0 {
                        return 1 as std::os::raw::c_int
                    } else { return -(1 as std::os::raw::c_int) }
                } else if invert != 0 {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
        } else {
            tmp = *x as std::os::raw::c_int - *y as std::os::raw::c_int;
            if tmp < 0 as std::os::raw::c_int {
                if invert != 0 {
                    return 1 as std::os::raw::c_int
                } else { return -(1 as std::os::raw::c_int) }
            }
            if tmp > 0 as std::os::raw::c_int {
                if invert != 0 {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
        }
        x = x.offset(1);
        y = y.offset(1)
    }
    if *x as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if invert != 0 {
            return -(1 as std::os::raw::c_int)
        } else { return 1 as std::os::raw::c_int }
    }
    if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if invert != 0 {
            return 1 as std::os::raw::c_int
        } else { return -(1 as std::os::raw::c_int) }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaComparePreserveCollapseStrings:
 * @x:  a first string value
 * @y:  a second string value
 *
 * Compare 2 string for their normalized values.
 * @x is a string with whitespace of "preserve", @y is
 * a string with a whitespace of "collapse". I.e. @x could
 * be an "xsd:string" and @y an "xsd:normalizedString".
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaComparePreserveCollapseStrings(mut x:
                                                                 *const xmlChar,
                                                             mut y:
                                                                 *const xmlChar,
                                                             mut invert:
                                                                 std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut tmp: std::os::raw::c_int = 0;
    /*
    * Skip leading blank chars of the collapsed string.
    */
    while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                  *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        y = y.offset(1)
    }
    while *x as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                   *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if !(*x as std::os::raw::c_int == 0x20 as std::os::raw::c_int) {
                /*
		* The yv character would have been replaced to 0x20.
		*/
                if (*x as std::os::raw::c_int - 0x20 as std::os::raw::c_int) <
                       0 as std::os::raw::c_int {
                    if invert != 0 {
                        return 1 as std::os::raw::c_int
                    } else { return -(1 as std::os::raw::c_int) }
                } else if invert != 0 {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
            x = x.offset(1);
            y = y.offset(1);
            /*
	    * Skip contiguous blank chars of the collapsed string.
	    */
            while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                          *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                y = y.offset(1)
            }
        } else {
            let fresh21 = x;
            x = x.offset(1);
            let fresh22 = y;
            y = y.offset(1);
            tmp = *fresh21 as std::os::raw::c_int - *fresh22 as std::os::raw::c_int;
            if tmp < 0 as std::os::raw::c_int {
                if invert != 0 {
                    return 1 as std::os::raw::c_int
                } else { return -(1 as std::os::raw::c_int) }
            }
            if tmp > 0 as std::os::raw::c_int {
                if invert != 0 {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
        }
    }
    if *x as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if invert != 0 {
            return -(1 as std::os::raw::c_int)
        } else { return 1 as std::os::raw::c_int }
    }
    if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        /*
	* Skip trailing blank chars of the collapsed string.
	*/
        while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                      *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            y = y.offset(1)
        }
        if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
            if invert != 0 {
                return 1 as std::os::raw::c_int
            } else { return -(1 as std::os::raw::c_int) }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaComparePreserveCollapseStrings:
 * @x:  a first string value
 * @y:  a second string value
 *
 * Compare 2 string for their normalized values.
 * @x is a string with whitespace of "preserve", @y is
 * a string with a whitespace of "collapse". I.e. @x could
 * be an "xsd:string" and @y an "xsd:normalizedString".
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareReplaceCollapseStrings(mut x:
                                                                *const xmlChar,
                                                            mut y:
                                                                *const xmlChar,
                                                            mut invert:
                                                                std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut tmp: std::os::raw::c_int = 0;
    /*
    * Skip leading blank chars of the collapsed string.
    */
    while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                  *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        y = y.offset(1)
    }
    while *x as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                   *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if !(*x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                     0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                         *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                     *x as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                /*
		* The yv character would have been replaced to 0x20.
		*/
                if (*x as std::os::raw::c_int - 0x20 as std::os::raw::c_int) <
                       0 as std::os::raw::c_int {
                    if invert != 0 {
                        return 1 as std::os::raw::c_int
                    } else { return -(1 as std::os::raw::c_int) }
                } else if invert != 0 {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
            x = x.offset(1);
            y = y.offset(1);
            /*
	    * Skip contiguous blank chars of the collapsed string.
	    */
            while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                          *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                y = y.offset(1)
            }
        } else {
            if *x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                   0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                       *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                   *x as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                /*
		* The xv character would have been replaced to 0x20.
		*/
                if (0x20 as std::os::raw::c_int - *y as std::os::raw::c_int) <
                       0 as std::os::raw::c_int {
                    if invert != 0 {
                        return 1 as std::os::raw::c_int
                    } else { return -(1 as std::os::raw::c_int) }
                } else if invert != 0 {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
            let fresh23 = x;
            x = x.offset(1);
            let fresh24 = y;
            y = y.offset(1);
            tmp = *fresh23 as std::os::raw::c_int - *fresh24 as std::os::raw::c_int;
            if tmp < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            if tmp > 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
        }
    }
    if *x as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if invert != 0 {
            return -(1 as std::os::raw::c_int)
        } else { return 1 as std::os::raw::c_int }
    }
    if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        /*
	* Skip trailing blank chars of the collapsed string.
	*/
        while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                      *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            y = y.offset(1)
        }
        if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
            if invert != 0 {
                return 1 as std::os::raw::c_int
            } else { return -(1 as std::os::raw::c_int) }
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaCompareReplacedStrings:
 * @x:  a first string value
 * @y:  a second string value
 *
 * Compare 2 string for their normalized values.
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareReplacedStrings(mut x: *const xmlChar,
                                                     mut y: *const xmlChar)
 -> std::os::raw::c_int {
    let mut tmp: std::os::raw::c_int = 0;
    while *x as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                   *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if !(*x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                     0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                         *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                     *x as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                if (*x as std::os::raw::c_int - 0x20 as std::os::raw::c_int) <
                       0 as std::os::raw::c_int {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
        } else {
            if *x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                   0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                       *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                   *x as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                if (0x20 as std::os::raw::c_int - *y as std::os::raw::c_int) <
                       0 as std::os::raw::c_int {
                    return -(1 as std::os::raw::c_int)
                } else { return 1 as std::os::raw::c_int }
            }
            tmp = *x as std::os::raw::c_int - *y as std::os::raw::c_int;
            if tmp < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            if tmp > 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
        }
        x = x.offset(1);
        y = y.offset(1)
    }
    if *x as std::os::raw::c_int != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if *y as std::os::raw::c_int != 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaCompareNormStrings:
 * @x:  a first string value
 * @y:  a second string value
 *
 * Compare 2 string for their normalized values.
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareNormStrings(mut x: *const xmlChar,
                                                 mut y: *const xmlChar)
 -> std::os::raw::c_int {
    let mut tmp: std::os::raw::c_int = 0;
    while *x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                  *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *x as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        x = x.offset(1)
    }
    while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                  *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        y = y.offset(1)
    }
    while *x as std::os::raw::c_int != 0 as std::os::raw::c_int &&
              *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
               0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                   *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
               *x as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            if !(*y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                     0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                         *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                     *y as std::os::raw::c_int == 0xd as std::os::raw::c_int) {
                tmp = *x as std::os::raw::c_int - *y as std::os::raw::c_int;
                return tmp
            }
            while *x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                          *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *x as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                x = x.offset(1)
            }
            while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                          *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                y = y.offset(1)
            }
        } else {
            let fresh25 = x;
            x = x.offset(1);
            let fresh26 = y;
            y = y.offset(1);
            tmp = *fresh25 as std::os::raw::c_int - *fresh26 as std::os::raw::c_int;
            if tmp < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
            if tmp > 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
        }
    }
    if *x as std::os::raw::c_int != 0 as std::os::raw::c_int {
        while *x as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *x as std::os::raw::c_int &&
                      *x as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *x as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            x = x.offset(1)
        }
        if *x as std::os::raw::c_int != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    }
    if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
        while *y as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                  0x9 as std::os::raw::c_int <= *y as std::os::raw::c_int &&
                      *y as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                  *y as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            y = y.offset(1)
        }
        if *y as std::os::raw::c_int != 0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaCompareFloats:
 * @x:  a first float or double value
 * @y:  a second float or double value
 *
 * Compare 2 values
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareFloats(mut x: xmlSchemaValPtr,
                                            mut y: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    let mut d1: std::os::raw::c_double = 0.;
    let mut d2: std::os::raw::c_double = 0.;
    if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
    /*
     * Cast everything to doubles.
     */
    if (*x).type_0 as std::os::raw::c_uint ==
           XML_SCHEMAS_DOUBLE as std::os::raw::c_int as std::os::raw::c_uint {
        d1 = (*x).value.d
    } else if (*x).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMAS_FLOAT as std::os::raw::c_int as std::os::raw::c_uint {
        d1 = (*x).value.f as std::os::raw::c_double
    } else { return -(2 as std::os::raw::c_int) }
    if (*y).type_0 as std::os::raw::c_uint ==
           XML_SCHEMAS_DOUBLE as std::os::raw::c_int as std::os::raw::c_uint {
        d2 = (*y).value.d
    } else if (*y).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMAS_FLOAT as std::os::raw::c_int as std::os::raw::c_uint {
        d2 = (*y).value.f as std::os::raw::c_double
    } else { return -(2 as std::os::raw::c_int) }
    /*
     * Check for special cases.
     */
    if xmlXPathIsNaN(d1) != 0 {
        if xmlXPathIsNaN(d2) != 0 { return 0 as std::os::raw::c_int }
        return 1 as std::os::raw::c_int
    }
    if xmlXPathIsNaN(d2) != 0 { return -(1 as std::os::raw::c_int) }
    if d1 == xmlXPathPINF {
        if d2 == xmlXPathPINF { return 0 as std::os::raw::c_int }
        return 1 as std::os::raw::c_int
    }
    if d2 == xmlXPathPINF { return -(1 as std::os::raw::c_int) }
    if d1 == xmlXPathNINF {
        if d2 == xmlXPathNINF { return 0 as std::os::raw::c_int }
        return -(1 as std::os::raw::c_int)
    }
    if d2 == xmlXPathNINF { return 1 as std::os::raw::c_int }
    /*
     * basic tests, the last one we should have equality, but
     * portability is more important than speed and handling
     * NaN or Inf in a portable way is always a challenge, so ...
     */
    if d1 < d2 { return -(1 as std::os::raw::c_int) }
    if d1 > d2 { return 1 as std::os::raw::c_int }
    if d1 == d2 { return 0 as std::os::raw::c_int }
    return 2 as std::os::raw::c_int;
}
/* *
 * xmlSchemaCompareValues:
 * @x:  a first value
 * @xvalue: the first value as a string (optional)
 * @xwtsp: the whitespace type
 * @y:  a second value
 * @xvalue: the second value as a string (optional)
 * @ywtsp: the whitespace type
 *
 * Compare 2 values
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, 3 if not
 * comparable and -2 in case of error
 */
unsafe extern "C" fn xmlSchemaCompareValuesInternal(mut xtype:
                                                        xmlSchemaValType,
                                                    mut x: xmlSchemaValPtr,
                                                    mut xvalue:
                                                        *const xmlChar,
                                                    mut xws:
                                                        xmlSchemaWhitespaceValueType,
                                                    mut ytype:
                                                        xmlSchemaValType,
                                                    mut y: xmlSchemaValPtr,
                                                    mut yvalue:
                                                        *const xmlChar,
                                                    mut yws:
                                                        xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    match xtype as std::os::raw::c_uint {
        0 | 45 => { return -(2 as std::os::raw::c_int) }
        30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 3 =>
        {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint == xtype as std::os::raw::c_uint {
                return xmlSchemaCompareDecimals(x, y)
            }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_DECIMAL as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_INTEGER as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NPINTEGER as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NINTEGER as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NNINTEGER as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_PINTEGER as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_INT as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_UINT as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_LONG as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_ULONG as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_SHORT as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_USHORT as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_BYTE as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_UBYTE as std::os::raw::c_int as std::os::raw::c_uint {
                return xmlSchemaCompareDecimals(x, y)
            }
            return -(2 as std::os::raw::c_int)
        }
        12 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_DURATION as std::os::raw::c_int as std::os::raw::c_uint {
                return xmlSchemaCompareDurations(x, y)
            }
            return -(2 as std::os::raw::c_int)
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_DATETIME as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_TIME as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_GDAY as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_GMONTH as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_GMONTHDAY as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_GYEAR as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_DATE as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_GYEARMONTH as std::os::raw::c_int as std::os::raw::c_uint {
                return xmlSchemaCompareDates(x, y)
            }
            return -(2 as std::os::raw::c_int)
        }
        46 | 1 | 2 | 16 | 17 | 18 | 20 | 22 | 23 | 24 | 26 | 29 => {
            /*
	* Note that we will support comparison of string types against
	* anySimpleType as well.
	*/
            let mut xv: *const xmlChar = 0 as *const xmlChar;
            let mut yv: *const xmlChar = 0 as *const xmlChar;
            if x.is_null() { xv = xvalue } else { xv = (*x).value.str_0 }
            if y.is_null() { yv = yvalue } else { yv = (*y).value.str_0 }
            /*
	    * TODO: Compare those against QName.
	    */
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_QNAME as std::os::raw::c_int as std::os::raw::c_uint {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Unimplemented block at %s:%d\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           b"xmlschemastypes.c\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           4795
                                                                               as
                                                                               std::os::raw::c_int);
                if y.is_null() { return -(2 as std::os::raw::c_int) }
                return -(2 as std::os::raw::c_int)
            }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_ANYSIMPLETYPE as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_STRING as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NORMSTRING as std::os::raw::c_int as std::os::raw::c_uint
                   ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_TOKEN as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_LANGUAGE as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NMTOKEN as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NAME as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NCNAME as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_ID as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_IDREF as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_ENTITY as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_ANYURI as std::os::raw::c_int as std::os::raw::c_uint {
                if xws as std::os::raw::c_uint ==
                       XML_SCHEMA_WHITESPACE_PRESERVE as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_PRESERVE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        /* TODO: What about x < y or x > y. */
                        if xmlStrEqual(xv, yv) != 0 {
                            return 0 as std::os::raw::c_int
                        } else { return 2 as std::os::raw::c_int }
                    } else {
                        if yws as std::os::raw::c_uint ==
                               XML_SCHEMA_WHITESPACE_REPLACE as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                            return xmlSchemaComparePreserveReplaceStrings(xv,
                                                                          yv,
                                                                          0 as
                                                                              std::os::raw::c_int)
                        } else {
                            if yws as std::os::raw::c_uint ==
                                   XML_SCHEMA_WHITESPACE_COLLAPSE as
                                       std::os::raw::c_int as std::os::raw::c_uint {
                                return xmlSchemaComparePreserveCollapseStrings(xv,
                                                                               yv,
                                                                               0
                                                                                   as
                                                                                   std::os::raw::c_int)
                            }
                        }
                    }
                } else if xws as std::os::raw::c_uint ==
                              XML_SCHEMA_WHITESPACE_REPLACE as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_PRESERVE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        return xmlSchemaComparePreserveReplaceStrings(yv, xv,
                                                                      1 as
                                                                          std::os::raw::c_int)
                    }
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_REPLACE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        return xmlSchemaCompareReplacedStrings(xv, yv)
                    }
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        return xmlSchemaCompareReplaceCollapseStrings(xv, yv,
                                                                      0 as
                                                                          std::os::raw::c_int)
                    }
                } else if xws as std::os::raw::c_uint ==
                              XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_PRESERVE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        return xmlSchemaComparePreserveCollapseStrings(yv, xv,
                                                                       1 as
                                                                           std::os::raw::c_int)
                    }
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_REPLACE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        return xmlSchemaCompareReplaceCollapseStrings(yv, xv,
                                                                      1 as
                                                                          std::os::raw::c_int)
                    }
                    if yws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        return xmlSchemaCompareNormStrings(xv, yv)
                    }
                } else { return -(2 as std::os::raw::c_int) }
            }
            return -(2 as std::os::raw::c_int)
        }
        21 | 28 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_QNAME as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_NOTATION as std::os::raw::c_int as std::os::raw::c_uint {
                if xmlStrEqual((*x).value.qname.name, (*y).value.qname.name)
                       != 0 &&
                       xmlStrEqual((*x).value.qname.uri, (*y).value.qname.uri)
                           != 0 {
                    return 0 as std::os::raw::c_int
                }
                return 2 as std::os::raw::c_int
            }
            return -(2 as std::os::raw::c_int)
        }
        13 | 14 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_FLOAT as std::os::raw::c_int as std::os::raw::c_uint ||
                   ytype as std::os::raw::c_uint ==
                       XML_SCHEMAS_DOUBLE as std::os::raw::c_int as std::os::raw::c_uint {
                return xmlSchemaCompareFloats(x, y)
            }
            return -(2 as std::os::raw::c_int)
        }
        15 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_BOOLEAN as std::os::raw::c_int as std::os::raw::c_uint {
                if (*x).value.b == (*y).value.b { return 0 as std::os::raw::c_int }
                if (*x).value.b == 0 as std::os::raw::c_int {
                    return -(1 as std::os::raw::c_int)
                }
                return 1 as std::os::raw::c_int
            }
            return -(2 as std::os::raw::c_int)
        }
        43 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_HEXBINARY as std::os::raw::c_int as std::os::raw::c_uint {
                if (*x).value.hex.total == (*y).value.hex.total {
                    let mut ret: std::os::raw::c_int =
                        xmlStrcmp((*x).value.hex.str_0, (*y).value.hex.str_0);
                    if ret > 0 as std::os::raw::c_int {
                        return 1 as std::os::raw::c_int
                    } else {
                        if ret == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
                    }
                } else if (*x).value.hex.total > (*y).value.hex.total {
                    return 1 as std::os::raw::c_int
                }
                return -(1 as std::os::raw::c_int)
            }
            return -(2 as std::os::raw::c_int)
        }
        44 => {
            if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
            if ytype as std::os::raw::c_uint ==
                   XML_SCHEMAS_BASE64BINARY as std::os::raw::c_int as std::os::raw::c_uint {
                if (*x).value.base64.total == (*y).value.base64.total {
                    let mut ret_0: std::os::raw::c_int =
                        xmlStrcmp((*x).value.base64.str_0,
                                  (*y).value.base64.str_0);
                    if ret_0 > 0 as std::os::raw::c_int {
                        return 1 as std::os::raw::c_int
                    } else if ret_0 == 0 as std::os::raw::c_int {
                        return 0 as std::os::raw::c_int
                    } else { return -(1 as std::os::raw::c_int) }
                } else if (*x).value.base64.total > (*y).value.base64.total {
                    return 1 as std::os::raw::c_int
                } else { return -(1 as std::os::raw::c_int) }
            }
            return -(2 as std::os::raw::c_int)
        }
        25 | 27 | 19 => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"xmlschemastypes.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       4920 as
                                                                           std::os::raw::c_int);
        }
        _ => { }
    }
    return -(2 as std::os::raw::c_int);
}
/* *
 * xmlSchemaCompareValues:
 * @x:  a first value
 * @y:  a second value
 *
 * Compare 2 values
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, and -2 in
 * case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaCompareValues(mut x: xmlSchemaValPtr,
                                                mut y: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    let mut xws: xmlSchemaWhitespaceValueType = XML_SCHEMA_WHITESPACE_UNKNOWN;
    let mut yws: xmlSchemaWhitespaceValueType = XML_SCHEMA_WHITESPACE_UNKNOWN;
    if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
    if (*x).type_0 as std::os::raw::c_uint ==
           XML_SCHEMAS_STRING as std::os::raw::c_int as std::os::raw::c_uint {
        xws = XML_SCHEMA_WHITESPACE_PRESERVE
    } else if (*x).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMAS_NORMSTRING as std::os::raw::c_int as std::os::raw::c_uint {
        xws = XML_SCHEMA_WHITESPACE_REPLACE
    } else { xws = XML_SCHEMA_WHITESPACE_COLLAPSE }
    if (*y).type_0 as std::os::raw::c_uint ==
           XML_SCHEMAS_STRING as std::os::raw::c_int as std::os::raw::c_uint {
        yws = XML_SCHEMA_WHITESPACE_PRESERVE
    } else if (*y).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMAS_NORMSTRING as std::os::raw::c_int as std::os::raw::c_uint {
        yws = XML_SCHEMA_WHITESPACE_REPLACE
    } else { yws = XML_SCHEMA_WHITESPACE_COLLAPSE }
    return xmlSchemaCompareValuesInternal((*x).type_0, x, 0 as *const xmlChar,
                                          xws, (*y).type_0, y,
                                          0 as *const xmlChar, yws);
}
/* *
 * xmlSchemaCompareValuesWhtsp:
 * @x:  a first value
 * @xws: the whitespace value of x
 * @y:  a second value
 * @yws: the whitespace value of y
 *
 * Compare 2 values
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, and -2 in
 * case of error
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaCompareValuesWhtsp(mut x: xmlSchemaValPtr,
                                                     mut xws:
                                                         xmlSchemaWhitespaceValueType,
                                                     mut y: xmlSchemaValPtr,
                                                     mut yws:
                                                         xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    if x.is_null() || y.is_null() { return -(2 as std::os::raw::c_int) }
    return xmlSchemaCompareValuesInternal((*x).type_0, x, 0 as *const xmlChar,
                                          xws, (*y).type_0, y,
                                          0 as *const xmlChar, yws);
}
/* *
 * xmlSchemaCompareValuesWhtspExt:
 * @x:  a first value
 * @xws: the whitespace value of x
 * @y:  a second value
 * @yws: the whitespace value of y
 *
 * Compare 2 values
 *
 * Returns -1 if x < y, 0 if x == y, 1 if x > y, 2 if x <> y, and -2 in
 * case of error
 */
unsafe extern "C" fn xmlSchemaCompareValuesWhtspExt(mut xtype:
                                                        xmlSchemaValType,
                                                    mut x: xmlSchemaValPtr,
                                                    mut xvalue:
                                                        *const xmlChar,
                                                    mut xws:
                                                        xmlSchemaWhitespaceValueType,
                                                    mut ytype:
                                                        xmlSchemaValType,
                                                    mut y: xmlSchemaValPtr,
                                                    mut yvalue:
                                                        *const xmlChar,
                                                    mut yws:
                                                        xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    return xmlSchemaCompareValuesInternal(xtype, x, xvalue, xws, ytype, y,
                                          yvalue, yws);
}
/* *
 * xmlSchemaNormLen:
 * @value:  a string
 *
 * Computes the UTF8 length of the normalized value of the string
 *
 * Returns the length or -1 in case of error.
 */
unsafe extern "C" fn xmlSchemaNormLen(mut value: *const xmlChar)
 -> std::os::raw::c_int {
    let mut utf: *const xmlChar = 0 as *const xmlChar;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if value.is_null() { return -(1 as std::os::raw::c_int) }
    utf = value;
    while *utf as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
              0x9 as std::os::raw::c_int <= *utf as std::os::raw::c_int &&
                  *utf as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
              *utf as std::os::raw::c_int == 0xd as std::os::raw::c_int {
        utf = utf.offset(1)
    }
    while *utf as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *utf.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int &
               0x80 as std::os::raw::c_int != 0 {
            if *utf.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                   0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            }
            if *utf.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                   0xe0 as std::os::raw::c_int == 0xe0 as std::os::raw::c_int {
                if *utf.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                       0xc0 as std::os::raw::c_int != 0x80 as std::os::raw::c_int {
                    return -(1 as std::os::raw::c_int)
                }
                if *utf.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                       0xf0 as std::os::raw::c_int == 0xf0 as std::os::raw::c_int {
                    if *utf.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int &
                           0xf8 as std::os::raw::c_int != 0xf0 as std::os::raw::c_int ||
                           *utf.offset(3 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int & 0xc0 as std::os::raw::c_int !=
                               0x80 as std::os::raw::c_int {
                        return -(1 as std::os::raw::c_int)
                    }
                    utf = utf.offset(4 as std::os::raw::c_int as isize)
                } else { utf = utf.offset(3 as std::os::raw::c_int as isize) }
            } else { utf = utf.offset(2 as std::os::raw::c_int as isize) }
        } else if *utf as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *utf as std::os::raw::c_int &&
                          *utf as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *utf as std::os::raw::c_int == 0xd as std::os::raw::c_int {
            while *utf as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                      0x9 as std::os::raw::c_int <= *utf as std::os::raw::c_int &&
                          *utf as std::os::raw::c_int <= 0xa as std::os::raw::c_int ||
                      *utf as std::os::raw::c_int == 0xd as std::os::raw::c_int {
                utf = utf.offset(1)
            }
            if *utf as std::os::raw::c_int == 0 as std::os::raw::c_int { break ; }
        } else { utf = utf.offset(1) }
        ret += 1
    }
    return ret;
}
/* *
 * xmlSchemaGetFacetValueAsULong:
 * @facet: an schemas type facet
 *
 * Extract the value of a facet
 *
 * Returns the value as a long
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetFacetValueAsULong(mut facet:
                                                           xmlSchemaFacetPtr)
 -> std::os::raw::c_ulong {
    /*
    * TODO: Check if this is a decimal.
    */
    if facet.is_null() { return 0 as std::os::raw::c_int as std::os::raw::c_ulong }
    return (*(*facet).val).value.decimal.lo;
}
/* *
 * xmlSchemaValidateListSimpleTypeFacet:
 * @facet:  the facet to check
 * @value:  the lexical repr of the value to validate
 * @actualLen:  the number of list items
 * @expectedLen: the resulting expected number of list items
 *
 * Checks the value of a list simple type against a facet.
 *
 * Returns 0 if the value is valid, a positive error code
 * number otherwise and -1 in case of an internal error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateListSimpleTypeFacet(mut facet:
                                                                  xmlSchemaFacetPtr,
                                                              mut value:
                                                                  *const xmlChar,
                                                              mut actualLen:
                                                                  std::os::raw::c_ulong,
                                                              mut expectedLen:
                                                                  *mut std::os::raw::c_ulong)
 -> std::os::raw::c_int {
    if facet.is_null() { return -(1 as std::os::raw::c_int) }
    /*
    * TODO: Check if this will work with large numbers.
    * (compare value.decimal.mi and value.decimal.hi as well?).
    */
    if (*facet).type_0 as std::os::raw::c_uint ==
           XML_SCHEMA_FACET_LENGTH as std::os::raw::c_int as std::os::raw::c_uint {
        if actualLen != (*(*facet).val).value.decimal.lo {
            if !expectedLen.is_null() {
                *expectedLen = (*(*facet).val).value.decimal.lo
            }
            return XML_SCHEMAV_CVC_LENGTH_VALID as std::os::raw::c_int
        }
    } else if (*facet).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMA_FACET_MINLENGTH as std::os::raw::c_int as std::os::raw::c_uint {
        if actualLen < (*(*facet).val).value.decimal.lo {
            if !expectedLen.is_null() {
                *expectedLen = (*(*facet).val).value.decimal.lo
            }
            return XML_SCHEMAV_CVC_MINLENGTH_VALID as std::os::raw::c_int
        }
    } else if (*facet).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMA_FACET_MAXLENGTH as std::os::raw::c_int as std::os::raw::c_uint {
        if actualLen > (*(*facet).val).value.decimal.lo {
            if !expectedLen.is_null() {
                *expectedLen = (*(*facet).val).value.decimal.lo
            }
            return XML_SCHEMAV_CVC_MAXLENGTH_VALID as std::os::raw::c_int
        }
    } else {
        /*
	* NOTE: That we can pass NULL as xmlSchemaValPtr to
	* xmlSchemaValidateFacet, since the remaining facet types
	* are: XML_SCHEMA_FACET_PATTERN, XML_SCHEMA_FACET_ENUMERATION.
	*/
        return xmlSchemaValidateFacet(0 as xmlSchemaTypePtr, facet, value,
                                      0 as xmlSchemaValPtr)
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaValidateLengthFacet:
 * @type:  the built-in type
 * @facet:  the facet to check
 * @value:  the lexical repr. of the value to be validated
 * @val:  the precomputed value
 * @ws: the whitespace type of the value
 * @length: the actual length of the value
 *
 * Checka a value against a "length", "minLength" and "maxLength"
 * facet; sets @length to the computed length of @value.
 *
 * Returns 0 if the value is valid, a positive error code
 * otherwise and -1 in case of an internal or API error.
 */
unsafe extern "C" fn xmlSchemaValidateLengthFacetInternal(mut facet:
                                                              xmlSchemaFacetPtr,
                                                          mut valType:
                                                              xmlSchemaValType,
                                                          mut value:
                                                              *const xmlChar,
                                                          mut val:
                                                              xmlSchemaValPtr,
                                                          mut length:
                                                              *mut std::os::raw::c_ulong,
                                                          mut ws:
                                                              xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    let mut len: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if length.is_null() || facet.is_null() { return -(1 as std::os::raw::c_int) }
    *length = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    if (*facet).type_0 as std::os::raw::c_uint !=
           XML_SCHEMA_FACET_LENGTH as std::os::raw::c_int as std::os::raw::c_uint &&
           (*facet).type_0 as std::os::raw::c_uint !=
               XML_SCHEMA_FACET_MAXLENGTH as std::os::raw::c_int as std::os::raw::c_uint &&
           (*facet).type_0 as std::os::raw::c_uint !=
               XML_SCHEMA_FACET_MINLENGTH as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    /*
    * TODO: length, maxLength and minLength must be of type
    * nonNegativeInteger only. Check if decimal is used somehow.
    */
    if (*facet).val.is_null() ||
           (*(*facet).val).type_0 as std::os::raw::c_uint !=
               XML_SCHEMAS_DECIMAL as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*facet).val).type_0 as std::os::raw::c_uint !=
                   XML_SCHEMAS_NNINTEGER as std::os::raw::c_int as std::os::raw::c_uint ||
           (*(*facet).val).value.decimal.frac() as std::os::raw::c_int !=
               0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if !val.is_null() &&
           (*val).type_0 as std::os::raw::c_uint ==
               XML_SCHEMAS_HEXBINARY as std::os::raw::c_int as std::os::raw::c_uint {
        len = (*val).value.hex.total
    } else if !val.is_null() &&
                  (*val).type_0 as std::os::raw::c_uint ==
                      XML_SCHEMAS_BASE64BINARY as std::os::raw::c_int as std::os::raw::c_uint
     {
        len = (*val).value.base64.total
    } else {
        match valType as std::os::raw::c_uint {
            1 | 2 => {
                if ws as std::os::raw::c_uint ==
                       XML_SCHEMA_WHITESPACE_UNKNOWN as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    /*
		    * This is to ensure API compatibility with the old
		    * xmlSchemaValidateLengthFacet(). Anyway, this was and
		    * is not the correct handling.
		    * TODO: Get rid of this case somehow.
		    */
                    if valType as std::os::raw::c_uint ==
                           XML_SCHEMAS_STRING as std::os::raw::c_int as std::os::raw::c_uint {
                        len = xmlUTF8Strlen(value) as std::os::raw::c_uint
                    } else { len = xmlSchemaNormLen(value) as std::os::raw::c_uint }
                } else if !value.is_null() {
                    if ws as std::os::raw::c_uint ==
                           XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                               std::os::raw::c_uint {
                        len = xmlSchemaNormLen(value) as std::os::raw::c_uint
                    } else {
                        /*
		    * Should be OK for "preserve" as well.
		    */
                        len = xmlUTF8Strlen(value) as std::os::raw::c_uint
                    }
                }
            }
            24 | 16 | 17 | 18 | 20 | 22 | 23 | 29 => {
                /*
		* FIXME: What exactly to do with anyURI?
		*/
                if !value.is_null() {
                    len = xmlSchemaNormLen(value) as std::os::raw::c_uint
                }
            }
            21 | 28 => {
                /*
		* For QName and NOTATION, those facets are
		* deprecated and should be ignored.
		*/
                return 0 as std::os::raw::c_int
            }
            _ => {
                (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                           b"Unimplemented block at %s:%d\n\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           b"xmlschemastypes.c\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const std::os::raw::c_char,
                                                                           5221
                                                                               as
                                                                               std::os::raw::c_int);
            }
        }
    }
    *length = len as std::os::raw::c_ulong;
    /*
    * TODO: Return the whole expected value, i.e. "lo", "mi" and "hi".
    */
    if (*facet).type_0 as std::os::raw::c_uint ==
           XML_SCHEMA_FACET_LENGTH as std::os::raw::c_int as std::os::raw::c_uint {
        if len as std::os::raw::c_ulong != (*(*facet).val).value.decimal.lo {
            return XML_SCHEMAV_CVC_LENGTH_VALID as std::os::raw::c_int
        }
    } else if (*facet).type_0 as std::os::raw::c_uint ==
                  XML_SCHEMA_FACET_MINLENGTH as std::os::raw::c_int as std::os::raw::c_uint {
        if (len as std::os::raw::c_ulong) < (*(*facet).val).value.decimal.lo {
            return XML_SCHEMAV_CVC_MINLENGTH_VALID as std::os::raw::c_int
        }
    } else if len as std::os::raw::c_ulong > (*(*facet).val).value.decimal.lo {
        return XML_SCHEMAV_CVC_MAXLENGTH_VALID as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaValidateLengthFacet:
 * @type:  the built-in type
 * @facet:  the facet to check
 * @value:  the lexical repr. of the value to be validated
 * @val:  the precomputed value
 * @length: the actual length of the value
 *
 * Checka a value against a "length", "minLength" and "maxLength"
 * facet; sets @length to the computed length of @value.
 *
 * Returns 0 if the value is valid, a positive error code
 * otherwise and -1 in case of an internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateLengthFacet(mut type_0:
                                                          xmlSchemaTypePtr,
                                                      mut facet:
                                                          xmlSchemaFacetPtr,
                                                      mut value:
                                                          *const xmlChar,
                                                      mut val:
                                                          xmlSchemaValPtr,
                                                      mut length:
                                                          *mut std::os::raw::c_ulong)
 -> std::os::raw::c_int {
    if type_0.is_null() { return -(1 as std::os::raw::c_int) }
    return xmlSchemaValidateLengthFacetInternal(facet,
                                                (*type_0).builtInType as
                                                    xmlSchemaValType, value,
                                                val, length,
                                                XML_SCHEMA_WHITESPACE_UNKNOWN);
}
/* *
 * xmlSchemaValidateLengthFacetWhtsp:
 * @facet:  the facet to check
 * @valType:  the built-in type
 * @value:  the lexical repr. of the value to be validated
 * @val:  the precomputed value
 * @ws: the whitespace type of the value
 * @length: the actual length of the value
 *
 * Checka a value against a "length", "minLength" and "maxLength"
 * facet; sets @length to the computed length of @value.
 *
 * Returns 0 if the value is valid, a positive error code
 * otherwise and -1 in case of an internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateLengthFacetWhtsp(mut facet:
                                                               xmlSchemaFacetPtr,
                                                           mut valType:
                                                               xmlSchemaValType,
                                                           mut value:
                                                               *const xmlChar,
                                                           mut val:
                                                               xmlSchemaValPtr,
                                                           mut length:
                                                               *mut std::os::raw::c_ulong,
                                                           mut ws:
                                                               xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    return xmlSchemaValidateLengthFacetInternal(facet, valType, value, val,
                                                length, ws);
}
/* *
 * xmlSchemaValidateFacetInternal:
 * @facet:  the facet to check
 * @fws: the whitespace type of the facet's value
 * @valType: the built-in type of the value
 * @value:  the lexical repr of the value to validate
 * @val:  the precomputed value
 * @ws: the whitespace type of the value
 *
 * Check a value against a facet condition
 *
 * Returns 0 if the element is schemas valid, a positive error code
 *     number otherwise and -1 in case of internal or API error.
 */
unsafe extern "C" fn xmlSchemaValidateFacetInternal(mut facet:
                                                        xmlSchemaFacetPtr,
                                                    mut fws:
                                                        xmlSchemaWhitespaceValueType,
                                                    mut valType:
                                                        xmlSchemaValType,
                                                    mut value: *const xmlChar,
                                                    mut val: xmlSchemaValPtr,
                                                    mut ws:
                                                        xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0;
    let mut stringType: std::os::raw::c_int = 0;
    if facet.is_null() { return -(1 as std::os::raw::c_int) }
    let mut current_block_98: u64;
    match (*facet).type_0 as std::os::raw::c_uint {
        1006 => {
            /*
	    * NOTE that for patterns, the @value needs to be the normalized
	    * value, *not* the lexical initial value or the canonical value.
	    */
            if value.is_null() { return -(1 as std::os::raw::c_int) }
            /*
	    * If string-derived type, regexp must be tested on the value space of
	    * the datatype.
	    * See https://www.w3.org/TR/xmlschema-2/#rf-pattern
	    */
            stringType =
                (!val.is_null() &&
                     ((*val).type_0 as std::os::raw::c_uint >=
                          XML_SCHEMAS_STRING as std::os::raw::c_int as std::os::raw::c_uint &&
                          (*val).type_0 as std::os::raw::c_uint <=
                              XML_SCHEMAS_NORMSTRING as std::os::raw::c_int as
                                  std::os::raw::c_uint ||
                          (*val).type_0 as std::os::raw::c_uint >=
                              XML_SCHEMAS_TOKEN as std::os::raw::c_int as std::os::raw::c_uint
                              &&
                              (*val).type_0 as std::os::raw::c_uint <=
                                  XML_SCHEMAS_NCNAME as std::os::raw::c_int as
                                      std::os::raw::c_uint)) as std::os::raw::c_int;
            ret =
                xmlRegexpExec((*facet).regexp,
                              if stringType != 0 &&
                                     !(*val).value.str_0.is_null() {
                                  (*val).value.str_0
                              } else { value });
            if ret == 1 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
            if ret == 0 as std::os::raw::c_int {
                return XML_SCHEMAV_CVC_PATTERN_VALID as std::os::raw::c_int
            }
            return ret
        }
        1003 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
            if ret == -(1 as std::os::raw::c_int) { return 0 as std::os::raw::c_int }
            return XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID as std::os::raw::c_int
        }
        1002 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
            if ret == -(1 as std::os::raw::c_int) || ret == 0 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int
            }
            return XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID as std::os::raw::c_int
        }
        1001 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
            if ret == 1 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
            return XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID as std::os::raw::c_int
        }
        1000 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
            if ret == 1 as std::os::raw::c_int || ret == 0 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int
            }
            return XML_SCHEMAV_CVC_MININCLUSIVE_VALID as std::os::raw::c_int
        }
        1008 => {
            /* TODO whitespaces */
	    /*
	    * NOTE: Whitespace should be handled to normalize
	    * the value to be validated against a the facets;
	    * not to normalize the value in-between.
	    */
            return 0 as std::os::raw::c_int
        }
        1007 => {
            if ws as std::os::raw::c_uint ==
                   XML_SCHEMA_WHITESPACE_UNKNOWN as std::os::raw::c_int as
                       std::os::raw::c_uint {
                /*
		* This is to ensure API compatibility with the old
		* xmlSchemaValidateFacet().
		* TODO: Get rid of this case.
		*/
                if !(*facet).value.is_null() &&
                       xmlStrEqual((*facet).value, value) != 0 {
                    return 0 as std::os::raw::c_int
                }
            } else {
                ret =
                    xmlSchemaCompareValuesWhtspExt((*(*facet).val).type_0,
                                                   (*facet).val,
                                                   (*facet).value, fws,
                                                   valType, val, value, ws);
                if ret == -(2 as std::os::raw::c_int) { return -(1 as std::os::raw::c_int) }
                if ret == 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
            }
            return XML_SCHEMAV_CVC_ENUMERATION_VALID as std::os::raw::c_int
        }
        1009 => {
            /*
	    * SPEC (1.3) "if {primitive type definition} is QName or NOTATION,
	    * then any {value} is facet-valid."
	    */
            if valType as std::os::raw::c_uint ==
                   XML_SCHEMAS_QNAME as std::os::raw::c_int as std::os::raw::c_uint ||
                   valType as std::os::raw::c_uint ==
                       XML_SCHEMAS_NOTATION as std::os::raw::c_int as std::os::raw::c_uint {
                return 0 as std::os::raw::c_int
            }
            current_block_98 = 790185930182612747;
        }
        1010 | 1011 => { current_block_98 = 790185930182612747; }
        1004 | 1005 => {
            if (*facet).val.is_null() ||
                   (*(*facet).val).type_0 as std::os::raw::c_uint !=
                       XML_SCHEMAS_PINTEGER as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*(*facet).val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_NNINTEGER as std::os::raw::c_int as
                               std::os::raw::c_uint ||
                   (*(*facet).val).value.decimal.frac() as std::os::raw::c_int !=
                       0 as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            }
            if val.is_null() ||
                   (*val).type_0 as std::os::raw::c_uint !=
                       XML_SCHEMAS_DECIMAL as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_INTEGER as std::os::raw::c_int as std::os::raw::c_uint
                       &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_NPINTEGER as std::os::raw::c_int as
                               std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_NINTEGER as std::os::raw::c_int as std::os::raw::c_uint
                       &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_NNINTEGER as std::os::raw::c_int as
                               std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_PINTEGER as std::os::raw::c_int as std::os::raw::c_uint
                       &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_INT as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_UINT as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_LONG as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_ULONG as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_SHORT as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_USHORT as std::os::raw::c_int as std::os::raw::c_uint
                       &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_BYTE as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_UBYTE as std::os::raw::c_int as std::os::raw::c_uint {
                return -(1 as std::os::raw::c_int)
            }
            if (*facet).type_0 as std::os::raw::c_uint ==
                   XML_SCHEMA_FACET_TOTALDIGITS as std::os::raw::c_int as std::os::raw::c_uint
               {
                if (*val).value.decimal.total() as std::os::raw::c_ulong >
                       (*(*facet).val).value.decimal.lo {
                    return XML_SCHEMAV_CVC_TOTALDIGITS_VALID as std::os::raw::c_int
                }
            } else if (*facet).type_0 as std::os::raw::c_uint ==
                          XML_SCHEMA_FACET_FRACTIONDIGITS as std::os::raw::c_int as
                              std::os::raw::c_uint {
                if (*val).value.decimal.frac() as std::os::raw::c_ulong >
                       (*(*facet).val).value.decimal.lo {
                    return XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID as std::os::raw::c_int
                }
            }
            current_block_98 = 9500030526577190060;
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                       b"Unimplemented block at %s:%d\n\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       b"xmlschemastypes.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char,
                                                                       5521 as
                                                                           std::os::raw::c_int);
            current_block_98 = 9500030526577190060;
        }
    }
    match current_block_98 {
        790185930182612747 =>
        /* Falls through. */
        {
            let mut len: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            if valType as std::os::raw::c_uint ==
                   XML_SCHEMAS_QNAME as std::os::raw::c_int as std::os::raw::c_uint ||
                   valType as std::os::raw::c_uint ==
                       XML_SCHEMAS_NOTATION as std::os::raw::c_int as std::os::raw::c_uint {
                return 0 as std::os::raw::c_int
            }
            /*
	    * TODO: length, maxLength and minLength must be of type
	    * nonNegativeInteger only. Check if decimal is used somehow.
	    */
            if (*facet).val.is_null() ||
                   (*(*facet).val).type_0 as std::os::raw::c_uint !=
                       XML_SCHEMAS_DECIMAL as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*(*facet).val).type_0 as std::os::raw::c_uint !=
                           XML_SCHEMAS_NNINTEGER as std::os::raw::c_int as
                               std::os::raw::c_uint ||
                   (*(*facet).val).value.decimal.frac() as std::os::raw::c_int !=
                       0 as std::os::raw::c_int {
                return -(1 as std::os::raw::c_int)
            }
            if !val.is_null() &&
                   (*val).type_0 as std::os::raw::c_uint ==
                       XML_SCHEMAS_HEXBINARY as std::os::raw::c_int as std::os::raw::c_uint {
                len = (*val).value.hex.total
            } else if !val.is_null() &&
                          (*val).type_0 as std::os::raw::c_uint ==
                              XML_SCHEMAS_BASE64BINARY as std::os::raw::c_int as
                                  std::os::raw::c_uint {
                len = (*val).value.base64.total
            } else {
                match valType as std::os::raw::c_uint {
                    1 | 2 => {
                        if ws as std::os::raw::c_uint ==
                               XML_SCHEMA_WHITESPACE_UNKNOWN as std::os::raw::c_int as
                                   std::os::raw::c_uint {
                            /*
			    * This is to ensure API compatibility with the old
			    * xmlSchemaValidateFacet(). Anyway, this was and
			    * is not the correct handling.
			    * TODO: Get rid of this case somehow.
			    */
                            if valType as std::os::raw::c_uint ==
                                   XML_SCHEMAS_STRING as std::os::raw::c_int as
                                       std::os::raw::c_uint {
                                len = xmlUTF8Strlen(value) as std::os::raw::c_uint
                            } else {
                                len = xmlSchemaNormLen(value) as std::os::raw::c_uint
                            }
                        } else if !value.is_null() {
                            if ws as std::os::raw::c_uint ==
                                   XML_SCHEMA_WHITESPACE_COLLAPSE as
                                       std::os::raw::c_int as std::os::raw::c_uint {
                                len = xmlSchemaNormLen(value) as std::os::raw::c_uint
                            } else {
                                /*
				* Should be OK for "preserve" as well.
				*/
                                len = xmlUTF8Strlen(value) as std::os::raw::c_uint
                            }
                        }
                    }
                    24 | 16 | 17 | 18 | 20 | 22 | 23 | 29 => {
                        if !value.is_null() {
                            len = xmlSchemaNormLen(value) as std::os::raw::c_uint
                        }
                    }
                    _ => {
                        (*__xmlGenericError()).expect("non-null function pointer")(*__xmlGenericErrorContext(),
                                                                                   b"Unimplemented block at %s:%d\n\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   b"xmlschemastypes.c\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   5470
                                                                                       as
                                                                                       std::os::raw::c_int);
                    }
                }
            }
            if (*facet).type_0 as std::os::raw::c_uint ==
                   XML_SCHEMA_FACET_LENGTH as std::os::raw::c_int as std::os::raw::c_uint {
                if len as std::os::raw::c_ulong != (*(*facet).val).value.decimal.lo {
                    return XML_SCHEMAV_CVC_LENGTH_VALID as std::os::raw::c_int
                }
            } else if (*facet).type_0 as std::os::raw::c_uint ==
                          XML_SCHEMA_FACET_MINLENGTH as std::os::raw::c_int as
                              std::os::raw::c_uint {
                if (len as std::os::raw::c_ulong) < (*(*facet).val).value.decimal.lo {
                    return XML_SCHEMAV_CVC_MINLENGTH_VALID as std::os::raw::c_int
                }
            } else if len as std::os::raw::c_ulong > (*(*facet).val).value.decimal.lo
             {
                return XML_SCHEMAV_CVC_MAXLENGTH_VALID as std::os::raw::c_int
            }
        }
        _ => { }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaValidateFacet:
 * @base:  the base type
 * @facet:  the facet to check
 * @value:  the lexical repr of the value to validate
 * @val:  the precomputed value
 *
 * Check a value against a facet condition
 *
 * Returns 0 if the element is schemas valid, a positive error code
 *     number otherwise and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateFacet(mut base: xmlSchemaTypePtr,
                                                mut facet: xmlSchemaFacetPtr,
                                                mut value: *const xmlChar,
                                                mut val: xmlSchemaValPtr)
 -> std::os::raw::c_int {
    /*
    * This tries to ensure API compatibility regarding the old
    * xmlSchemaValidateFacet() and the new xmlSchemaValidateFacetInternal() and
    * xmlSchemaValidateFacetWhtsp().
    */
    if !val.is_null() {
        return xmlSchemaValidateFacetInternal(facet,
                                              XML_SCHEMA_WHITESPACE_UNKNOWN,
                                              (*val).type_0, value, val,
                                              XML_SCHEMA_WHITESPACE_UNKNOWN)
    } else {
        if !base.is_null() {
            return xmlSchemaValidateFacetInternal(facet,
                                                  XML_SCHEMA_WHITESPACE_UNKNOWN,
                                                  (*base).builtInType as
                                                      xmlSchemaValType, value,
                                                  val,
                                                  XML_SCHEMA_WHITESPACE_UNKNOWN)
        }
    }
    return -(1 as std::os::raw::c_int);
}
/* *
 * xmlSchemaValidateFacetWhtsp:
 * @facet:  the facet to check
 * @fws: the whitespace type of the facet's value
 * @valType: the built-in type of the value
 * @value:  the lexical (or normalized for pattern) repr of the value to validate
 * @val:  the precomputed value
 * @ws: the whitespace type of the value
 *
 * Check a value against a facet condition. This takes value normalization
 * according to the specified whitespace types into account.
 * Note that @value needs to be the *normalized* value if the facet
 * is of type "pattern".
 *
 * Returns 0 if the element is schemas valid, a positive error code
 *     number otherwise and -1 in case of internal or API error.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateFacetWhtsp(mut facet:
                                                         xmlSchemaFacetPtr,
                                                     mut fws:
                                                         xmlSchemaWhitespaceValueType,
                                                     mut valType:
                                                         xmlSchemaValType,
                                                     mut value:
                                                         *const xmlChar,
                                                     mut val: xmlSchemaValPtr,
                                                     mut ws:
                                                         xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    return xmlSchemaValidateFacetInternal(facet, fws, valType, value, val,
                                          ws);
}
/* *
 * xmlSchemaGetCanonValue:
 * @val: the precomputed value
 * @retValue: the returned value
 *
 * Get the canonical lexical representation of the value.
 * The caller has to FREE the returned retValue.
 *
 * WARNING: Some value types are not supported yet, resulting
 * in a @retValue of "???".
 *
 * TODO: XML Schema 1.0 does not define canonical representations
 * for: duration, gYearMonth, gYear, gMonthDay, gMonth, gDay,
 * anyURI, QName, NOTATION. This will be fixed in XML Schema 1.1.
 *
 *
 * Returns 0 if the value could be built, 1 if the value type is
 * not supported yet and -1 in case of API errors.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetCanonValue(mut val: xmlSchemaValPtr,
                                                mut retValue:
                                                    *mut *const xmlChar)
 -> std::os::raw::c_int {
    if retValue.is_null() || val.is_null() { return -(1 as std::os::raw::c_int) }
    *retValue = 0 as *const xmlChar;
    match (*val).type_0 as std::os::raw::c_uint {
        1 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar)
            } else {
                *retValue = xmlStrdup((*val).value.str_0 as *const xmlChar)
            }
        }
        2 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar)
            } else {
                *retValue =
                    xmlSchemaWhiteSpaceReplace((*val).value.str_0 as
                                                   *const xmlChar);
                if (*retValue).is_null() {
                    *retValue =
                        xmlStrdup((*val).value.str_0 as *const xmlChar)
                }
            }
        }
        16 | 17 | 18 | 20 | 22 | 23 | 24 | 26 | 28 | 29 => {
            /* Unclear */
            /* Unclear */
            if (*val).value.str_0.is_null() { return -(1 as std::os::raw::c_int) }
            *retValue = xmlSchemaCollapseString((*val).value.str_0);
            if (*retValue).is_null() {
                *retValue = xmlStrdup((*val).value.str_0 as *const xmlChar)
            }
        }
        21 => {
            /* TODO: Unclear in XML Schema 1.0. */
            if (*val).value.qname.uri.is_null() {
                *retValue = xmlStrdup((*val).value.qname.name);
                return 0 as std::os::raw::c_int
            } else {
                *retValue =
                    xmlStrdup(b"{\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar);
                *retValue =
                    xmlStrcat(*retValue as *mut xmlChar,
                              (*val).value.qname.uri);
                *retValue =
                    xmlStrcat(*retValue as *mut xmlChar,
                              b"}\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar);
                *retValue =
                    xmlStrcat(*retValue as *mut xmlChar,
                              (*val).value.qname.uri)
            }
        }
        3 => {
            /*
	    * TODO: Lookout for a more simple implementation.
	    */
            if (*val).value.decimal.total() as std::os::raw::c_int == 1 as std::os::raw::c_int
                   &&
                   (*val).value.decimal.lo ==
                       0 as std::os::raw::c_int as std::os::raw::c_ulong {
                *retValue =
                    xmlStrdup(b"0.0\x00" as *const u8 as *const std::os::raw::c_char
                                  as *mut xmlChar)
            } else {
                let mut dec: xmlSchemaValDecimal = (*val).value.decimal;
                let mut bufsize: std::os::raw::c_int = 0;
                let mut buf: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
                let mut offs: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
                /* Add room for the decimal point as well. */
                bufsize = dec.total() as std::os::raw::c_int + 2 as std::os::raw::c_int;
                if dec.sign() != 0 { bufsize += 1 }
                /* Add room for leading/trailing zero. */
                if dec.frac() as std::os::raw::c_int == 0 as std::os::raw::c_int ||
                       dec.frac() as std::os::raw::c_int == dec.total() as std::os::raw::c_int
                   {
                    bufsize += 1
                }
                buf =
                    xmlMalloc.expect("non-null function pointer")(bufsize as
                                                                      size_t)
                        as *mut std::os::raw::c_char;
                if buf.is_null() { return -(1 as std::os::raw::c_int) }
                offs = buf;
                if dec.sign() != 0 {
                    let fresh27 = offs;
                    offs = offs.offset(1);
                    *fresh27 = '-' as i32 as std::os::raw::c_char
                }
                if dec.frac() as std::os::raw::c_int == dec.total() as std::os::raw::c_int {
                    let fresh28 = offs;
                    offs = offs.offset(1);
                    *fresh28 = '0' as i32 as std::os::raw::c_char;
                    let fresh29 = offs;
                    offs = offs.offset(1);
                    *fresh29 = '.' as i32 as std::os::raw::c_char
                }
                if dec.hi != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    snprintf(offs,
                             (bufsize as std::os::raw::c_long -
                                  offs.offset_from(buf) as
                                      std::os::raw::c_long) as std::os::raw::c_ulong,
                             b"%lu%lu%lu\x00" as *const u8 as
                                 *const std::os::raw::c_char, dec.hi, dec.mi, dec.lo);
                } else if dec.mi != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    snprintf(offs,
                             (bufsize as std::os::raw::c_long -
                                  offs.offset_from(buf) as
                                      std::os::raw::c_long) as std::os::raw::c_ulong,
                             b"%lu%lu\x00" as *const u8 as
                                 *const std::os::raw::c_char, dec.mi, dec.lo);
                } else {
                    snprintf(offs,
                             (bufsize as std::os::raw::c_long -
                                  offs.offset_from(buf) as
                                      std::os::raw::c_long) as std::os::raw::c_ulong,
                             b"%lu\x00" as *const u8 as *const std::os::raw::c_char,
                             dec.lo);
                }
                if dec.frac() as std::os::raw::c_int != 0 as std::os::raw::c_int {
                    if dec.frac() as std::os::raw::c_int != dec.total() as std::os::raw::c_int
                       {
                        let mut diff: std::os::raw::c_int =
                            dec.total() as std::os::raw::c_int -
                                dec.frac() as std::os::raw::c_int;
                        /*
			* Insert the decimal point.
			*/
                        memmove(offs.offset(diff as
                                                isize).offset(1 as std::os::raw::c_int
                                                                  as isize) as
                                    *mut std::os::raw::c_void,
                                offs.offset(diff as isize) as
                                    *const std::os::raw::c_void,
                                (dec.frac() as std::os::raw::c_int + 1 as std::os::raw::c_int)
                                    as std::os::raw::c_ulong);
                        *offs.offset(diff as isize) =
                            '.' as i32 as std::os::raw::c_char
                    } else {
                        let mut i: std::os::raw::c_uint =
                            0 as std::os::raw::c_int as std::os::raw::c_uint;
                        /*
			* Insert missing zeroes behind the decimal point.
			*/
                        while *offs.offset(i as isize) as std::os::raw::c_int !=
                                  0 as std::os::raw::c_int {
                            i = i.wrapping_add(1)
                        }
                        if i < dec.total() {
                            memmove(offs.offset(dec.total().wrapping_sub(i) as
                                                    isize) as
                                        *mut std::os::raw::c_void,
                                    offs as *const std::os::raw::c_void,
                                    i.wrapping_add(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as
                                        std::os::raw::c_ulong);
                            memset(offs as *mut std::os::raw::c_void, '0' as i32,
                                   dec.total().wrapping_sub(i) as
                                       std::os::raw::c_ulong);
                        }
                    }
                } else {
                    /*
		    * Append decimal point and zero.
		    */
                    offs =
                        buf.offset(bufsize as
                                       isize).offset(-(1 as std::os::raw::c_int as
                                                           isize));
                    let fresh30 = offs;
                    offs = offs.offset(-1);
                    *fresh30 = 0 as std::os::raw::c_int as std::os::raw::c_char;
                    let fresh31 = offs;
                    offs = offs.offset(-1);
                    *fresh31 = '0' as i32 as std::os::raw::c_char;
                    let fresh32 = offs;
                    offs = offs.offset(-1);
                    *fresh32 = '.' as i32 as std::os::raw::c_char
                }
                *retValue = buf as *mut xmlChar
            }
        }
        30 | 34 | 31 | 32 | 33 | 37 | 41 | 39 | 35 | 36 | 38 | 40 | 42 => {
            if (*val).value.decimal.total() as std::os::raw::c_int == 1 as std::os::raw::c_int
                   &&
                   (*val).value.decimal.lo ==
                       0 as std::os::raw::c_int as std::os::raw::c_ulong {
                *retValue =
                    xmlStrdup(b"0\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar)
            } else {
                let mut dec_0: xmlSchemaValDecimal = (*val).value.decimal;
                let mut bufsize_0: std::os::raw::c_int =
                    dec_0.total() as std::os::raw::c_int + 1 as std::os::raw::c_int;
                /* Add room for the decimal point as well. */
                if dec_0.sign() != 0 { bufsize_0 += 1 }
                *retValue =
                    xmlMalloc.expect("non-null function pointer")(bufsize_0 as
                                                                      size_t)
                        as *const xmlChar;
                if (*retValue).is_null() { return -(1 as std::os::raw::c_int) }
                if dec_0.hi != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    if dec_0.sign() != 0 {
                        snprintf(*retValue as *mut std::os::raw::c_char,
                                 bufsize_0 as std::os::raw::c_ulong,
                                 b"-%lu%lu%lu\x00" as *const u8 as
                                     *const std::os::raw::c_char, dec_0.hi, dec_0.mi,
                                 dec_0.lo);
                    } else {
                        snprintf(*retValue as *mut std::os::raw::c_char,
                                 bufsize_0 as std::os::raw::c_ulong,
                                 b"%lu%lu%lu\x00" as *const u8 as
                                     *const std::os::raw::c_char, dec_0.hi, dec_0.mi,
                                 dec_0.lo);
                    }
                } else if dec_0.mi != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    if dec_0.sign() != 0 {
                        snprintf(*retValue as *mut std::os::raw::c_char,
                                 bufsize_0 as std::os::raw::c_ulong,
                                 b"-%lu%lu\x00" as *const u8 as
                                     *const std::os::raw::c_char, dec_0.mi, dec_0.lo);
                    } else {
                        snprintf(*retValue as *mut std::os::raw::c_char,
                                 bufsize_0 as std::os::raw::c_ulong,
                                 b"%lu%lu\x00" as *const u8 as
                                     *const std::os::raw::c_char, dec_0.mi, dec_0.lo);
                    }
                } else if dec_0.sign() != 0 {
                    snprintf(*retValue as *mut std::os::raw::c_char,
                             bufsize_0 as std::os::raw::c_ulong,
                             b"-%lu\x00" as *const u8 as *const std::os::raw::c_char,
                             dec_0.lo);
                } else {
                    snprintf(*retValue as *mut std::os::raw::c_char,
                             bufsize_0 as std::os::raw::c_ulong,
                             b"%lu\x00" as *const u8 as *const std::os::raw::c_char,
                             dec_0.lo);
                }
            }
        }
        15 => {
            if (*val).value.b != 0 {
                *retValue =
                    xmlStrdup(b"true\x00" as *const u8 as *const std::os::raw::c_char
                                  as *mut xmlChar)
            } else {
                *retValue =
                    xmlStrdup(b"false\x00" as *const u8 as *const std::os::raw::c_char
                                  as *mut xmlChar)
            }
        }
        12 => {
            let mut buf_0: [std::os::raw::c_char; 100] = [0; 100];
            let mut year: std::os::raw::c_ulong = 0;
            let mut mon: std::os::raw::c_ulong = 0;
            let mut day: std::os::raw::c_ulong = 0;
            let mut hour: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
            let mut min: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
            let mut sec: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
            let mut left: std::os::raw::c_double = 0.;
            /* TODO: Unclear in XML Schema 1.0 */
		/*
		* TODO: This results in a normalized output of the value
		* - which is NOT conformant to the spec -
		* since the exact values of each property are not
		* recoverable. Think about extending the structure to
		* provide a field for every property.
		*/
            year =
                floor(labs((*val).value.dur.mon) as std::os::raw::c_double /
                          12 as std::os::raw::c_int as std::os::raw::c_double) as
                    std::os::raw::c_ulong;
            mon =
                (labs((*val).value.dur.mon) as
                     std::os::raw::c_ulong).wrapping_sub((12 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong).wrapping_mul(year));
            day =
                floor(fabs((*val).value.dur.sec) /
                          86400 as std::os::raw::c_int as std::os::raw::c_double) as
                    std::os::raw::c_ulong;
            left =
                fabs((*val).value.dur.sec) -
                    day.wrapping_mul(86400 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        std::os::raw::c_double;
            if left > 0 as std::os::raw::c_int as std::os::raw::c_double {
                hour =
                    floor(left / 3600 as std::os::raw::c_int as std::os::raw::c_double) as
                        std::os::raw::c_ulong;
                left =
                    left -
                        hour.wrapping_mul(3600 as std::os::raw::c_int as
                                              std::os::raw::c_ulong) as
                            std::os::raw::c_double;
                if left > 0 as std::os::raw::c_int as std::os::raw::c_double {
                    min =
                        floor(left / 60 as std::os::raw::c_int as std::os::raw::c_double) as
                            std::os::raw::c_ulong;
                    sec =
                        left -
                            min.wrapping_mul(60 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as
                                std::os::raw::c_double
                }
            }
            if (*val).value.dur.mon < 0 as std::os::raw::c_int as std::os::raw::c_long ||
                   (*val).value.dur.sec < 0 as std::os::raw::c_int as std::os::raw::c_double {
                snprintf(buf_0.as_mut_ptr(),
                         100 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"P%luY%luM%luDT%luH%luM%.14gS\x00" as *const u8 as
                             *const std::os::raw::c_char, year, mon, day, hour, min,
                         sec);
            } else {
                snprintf(buf_0.as_mut_ptr(),
                         100 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"-P%luY%luM%luDT%luH%luM%.14gS\x00" as *const u8 as
                             *const std::os::raw::c_char, year, mon, day, hour, min,
                         sec);
            }
            *retValue = xmlStrdup(buf_0.as_mut_ptr() as *mut xmlChar)
        }
        8 => {
            let mut buf_1: [std::os::raw::c_char; 30] = [0; 30];
            /* TODO: Unclear in XML Schema 1.0 */
		/* TODO: What to do with the timezone? */
            snprintf(buf_1.as_mut_ptr(), 30 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"%04ld\x00" as *const u8 as *const std::os::raw::c_char,
                     (*val).value.date.year);
            *retValue = xmlStrdup(buf_1.as_mut_ptr() as *mut xmlChar)
        }
        6 => {
            /* TODO: Unclear in XML Schema 1.0 */
		/* TODO: What to do with the timezone? */
            *retValue =
                xmlMalloc.expect("non-null function pointer")(6 as std::os::raw::c_int
                                                                  as size_t)
                    as *const xmlChar;
            if (*retValue).is_null() { return -(1 as std::os::raw::c_int) }
            snprintf(*retValue as *mut std::os::raw::c_char,
                     6 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"--%02u\x00" as *const u8 as *const std::os::raw::c_char,
                     (*val).value.date.mon() as std::os::raw::c_int);
        }
        5 => {
            /* TODO: Unclear in XML Schema 1.0 */
		/* TODO: What to do with the timezone? */
            *retValue =
                xmlMalloc.expect("non-null function pointer")(6 as std::os::raw::c_int
                                                                  as size_t)
                    as *const xmlChar;
            if (*retValue).is_null() { return -(1 as std::os::raw::c_int) }
            snprintf(*retValue as *mut std::os::raw::c_char,
                     6 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"---%02u\x00" as *const u8 as *const std::os::raw::c_char,
                     (*val).value.date.day() as std::os::raw::c_int);
        }
        7 => {
            /* TODO: Unclear in XML Schema 1.0 */
		/* TODO: What to do with the timezone? */
            *retValue =
                xmlMalloc.expect("non-null function pointer")(8 as std::os::raw::c_int
                                                                  as size_t)
                    as *const xmlChar;
            if (*retValue).is_null() { return -(1 as std::os::raw::c_int) }
            snprintf(*retValue as *mut std::os::raw::c_char,
                     8 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"--%02u-%02u\x00" as *const u8 as *const std::os::raw::c_char,
                     (*val).value.date.mon() as std::os::raw::c_int,
                     (*val).value.date.day() as std::os::raw::c_int);
        }
        9 => {
            let mut buf_2: [std::os::raw::c_char; 35] = [0; 35];
            /* TODO: Unclear in XML Schema 1.0 */
		/* TODO: What to do with the timezone? */
            if (*val).value.date.year < 0 as std::os::raw::c_int as std::os::raw::c_long {
                snprintf(buf_2.as_mut_ptr(),
                         35 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"-%04ld-%02u\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         labs((*val).value.date.year),
                         (*val).value.date.mon() as std::os::raw::c_int);
            } else {
                snprintf(buf_2.as_mut_ptr(),
                         35 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%04ld-%02u\x00" as *const u8 as
                             *const std::os::raw::c_char, (*val).value.date.year,
                         (*val).value.date.mon() as std::os::raw::c_int);
            }
            *retValue = xmlStrdup(buf_2.as_mut_ptr() as *mut xmlChar)
        }
        4 => {
            let mut buf_3: [std::os::raw::c_char; 30] = [0; 30];
            if (*val).value.date.tz_flag() != 0 {
                let mut norm: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
                norm =
                    xmlSchemaDateNormalize(val,
                                           0 as std::os::raw::c_int as
                                               std::os::raw::c_double);
                if norm.is_null() { return -(1 as std::os::raw::c_int) }
                /*
		    * TODO: Check if "%.14g" is portable.
		    */
                snprintf(buf_3.as_mut_ptr(),
                         30 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%02u:%02u:%02.14gZ\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         (*norm).value.date.hour() as std::os::raw::c_int,
                         (*norm).value.date.min() as std::os::raw::c_int,
                         (*norm).value.date.sec);
                xmlSchemaFreeValue(norm);
            } else {
                snprintf(buf_3.as_mut_ptr(),
                         30 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%02u:%02u:%02.14g\x00" as *const u8 as
                             *const std::os::raw::c_char,
                         (*val).value.date.hour() as std::os::raw::c_int,
                         (*val).value.date.min() as std::os::raw::c_int,
                         (*val).value.date.sec);
            }
            *retValue = xmlStrdup(buf_3.as_mut_ptr() as *mut xmlChar)
        }
        10 => {
            let mut buf_4: [std::os::raw::c_char; 30] = [0; 30];
            if (*val).value.date.tz_flag() != 0 {
                let mut norm_0: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
                norm_0 =
                    xmlSchemaDateNormalize(val,
                                           0 as std::os::raw::c_int as
                                               std::os::raw::c_double);
                if norm_0.is_null() { return -(1 as std::os::raw::c_int) }
                /*
		    * TODO: Append the canonical value of the
		    * recoverable timezone and not "Z".
		    */
                snprintf(buf_4.as_mut_ptr(),
                         30 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%04ld:%02u:%02uZ\x00" as *const u8 as
                             *const std::os::raw::c_char, (*norm_0).value.date.year,
                         (*norm_0).value.date.mon() as std::os::raw::c_int,
                         (*norm_0).value.date.day() as std::os::raw::c_int);
                xmlSchemaFreeValue(norm_0);
            } else {
                snprintf(buf_4.as_mut_ptr(),
                         30 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%04ld:%02u:%02u\x00" as *const u8 as
                             *const std::os::raw::c_char, (*val).value.date.year,
                         (*val).value.date.mon() as std::os::raw::c_int,
                         (*val).value.date.day() as std::os::raw::c_int);
            }
            *retValue = xmlStrdup(buf_4.as_mut_ptr() as *mut xmlChar)
        }
        11 => {
            let mut buf_5: [std::os::raw::c_char; 50] = [0; 50];
            if (*val).value.date.tz_flag() != 0 {
                let mut norm_1: xmlSchemaValPtr = 0 as *mut xmlSchemaVal;
                norm_1 =
                    xmlSchemaDateNormalize(val,
                                           0 as std::os::raw::c_int as
                                               std::os::raw::c_double);
                if norm_1.is_null() { return -(1 as std::os::raw::c_int) }
                /*
		    * TODO: Check if "%.14g" is portable.
		    */
                snprintf(buf_5.as_mut_ptr(),
                         50 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%04ld:%02u:%02uT%02u:%02u:%02.14gZ\x00" as
                             *const u8 as *const std::os::raw::c_char,
                         (*norm_1).value.date.year,
                         (*norm_1).value.date.mon() as std::os::raw::c_int,
                         (*norm_1).value.date.day() as std::os::raw::c_int,
                         (*norm_1).value.date.hour() as std::os::raw::c_int,
                         (*norm_1).value.date.min() as std::os::raw::c_int,
                         (*norm_1).value.date.sec);
                xmlSchemaFreeValue(norm_1);
            } else {
                snprintf(buf_5.as_mut_ptr(),
                         50 as std::os::raw::c_int as std::os::raw::c_ulong,
                         b"%04ld:%02u:%02uT%02u:%02u:%02.14g\x00" as *const u8
                             as *const std::os::raw::c_char, (*val).value.date.year,
                         (*val).value.date.mon() as std::os::raw::c_int,
                         (*val).value.date.day() as std::os::raw::c_int,
                         (*val).value.date.hour() as std::os::raw::c_int,
                         (*val).value.date.min() as std::os::raw::c_int,
                         (*val).value.date.sec);
            }
            *retValue = xmlStrdup(buf_5.as_mut_ptr() as *mut xmlChar)
        }
        43 => { *retValue = xmlStrdup((*val).value.hex.str_0) }
        44 => {
            /*
	    * TODO: Is the following spec piece implemented?:
	    * SPEC: "Note: For some values the canonical form defined
	    * above does not conform to [RFC 2045], which requires breaking
	    * with linefeeds at appropriate intervals."
	    */
            *retValue = xmlStrdup((*val).value.base64.str_0)
        }
        13 => {
            let mut buf_6: [std::os::raw::c_char; 30] = [0; 30];
            /*
		* |m| < 16777216, -149 <= e <= 104.
		* TODO: Handle, NaN, INF, -INF. The format is not
		* yet conformant. The c type float does not cover
		* the whole range.
		*/
            snprintf(buf_6.as_mut_ptr(), 30 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"%01.14e\x00" as *const u8 as *const std::os::raw::c_char,
                     (*val).value.f as std::os::raw::c_double);
            *retValue = xmlStrdup(buf_6.as_mut_ptr() as *mut xmlChar)
        }
        14 => {
            let mut buf_7: [std::os::raw::c_char; 40] = [0; 40];
            /* |m| < 9007199254740992, -1075 <= e <= 970 */
		/*
		* TODO: Handle, NaN, INF, -INF. The format is not
		* yet conformant. The c type float does not cover
		* the whole range.
		*/
            snprintf(buf_7.as_mut_ptr(), 40 as std::os::raw::c_int as std::os::raw::c_ulong,
                     b"%01.14e\x00" as *const u8 as *const std::os::raw::c_char,
                     (*val).value.d);
            *retValue = xmlStrdup(buf_7.as_mut_ptr() as *mut xmlChar)
        }
        _ => {
            *retValue =
                xmlStrdup(b"???\x00" as *const u8 as *const std::os::raw::c_char as
                              *mut xmlChar);
            return 1 as std::os::raw::c_int
        }
    }
    if (*retValue).is_null() { return -(1 as std::os::raw::c_int) }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaGetCanonValueWhtsp:
 * @val: the precomputed value
 * @retValue: the returned value
 * @ws: the whitespace type of the value
 *
 * Get the canonical representation of the value.
 * The caller has to free the returned @retValue.
 *
 * Returns 0 if the value could be built, 1 if the value type is
 * not supported yet and -1 in case of API errors.
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetCanonValueWhtsp(mut val: xmlSchemaValPtr,
                                                     mut retValue:
                                                         *mut *const xmlChar,
                                                     mut ws:
                                                         xmlSchemaWhitespaceValueType)
 -> std::os::raw::c_int {
    if retValue.is_null() || val.is_null() { return -(1 as std::os::raw::c_int) }
    if ws as std::os::raw::c_uint ==
           XML_SCHEMA_WHITESPACE_UNKNOWN as std::os::raw::c_int as std::os::raw::c_uint ||
           ws as std::os::raw::c_uint >
               XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as std::os::raw::c_uint {
        return -(1 as std::os::raw::c_int)
    }
    *retValue = 0 as *const xmlChar;
    match (*val).type_0 as std::os::raw::c_uint {
        1 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar)
            } else if ws as std::os::raw::c_uint ==
                          XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                              std::os::raw::c_uint {
                *retValue = xmlSchemaCollapseString((*val).value.str_0)
            } else if ws as std::os::raw::c_uint ==
                          XML_SCHEMA_WHITESPACE_REPLACE as std::os::raw::c_int as
                              std::os::raw::c_uint {
                *retValue = xmlSchemaWhiteSpaceReplace((*val).value.str_0)
            }
            if (*retValue).is_null() {
                *retValue = xmlStrdup((*val).value.str_0)
            }
        }
        2 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\x00" as *const u8 as *const std::os::raw::c_char as
                                  *mut xmlChar)
            } else {
                if ws as std::os::raw::c_uint ==
                       XML_SCHEMA_WHITESPACE_COLLAPSE as std::os::raw::c_int as
                           std::os::raw::c_uint {
                    *retValue = xmlSchemaCollapseString((*val).value.str_0)
                } else {
                    *retValue = xmlSchemaWhiteSpaceReplace((*val).value.str_0)
                }
                if (*retValue).is_null() {
                    *retValue = xmlStrdup((*val).value.str_0)
                }
            }
        }
        _ => { return xmlSchemaGetCanonValue(val, retValue) }
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * xmlSchemaGetValType:
 * @val: a schemas value
 *
 * Accessor for the type of a value
 *
 * Returns the xmlSchemaValType of the value
 */
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetValType(mut val: xmlSchemaValPtr)
 -> xmlSchemaValType {
    if val.is_null() { return XML_SCHEMAS_UNKNOWN }
    return (*val).type_0;
}
/* LIBXML_SCHEMAS_ENABLED */
/* __INCLUDE_ELFGCCHACK */
