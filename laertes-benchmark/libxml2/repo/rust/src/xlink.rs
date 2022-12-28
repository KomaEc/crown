
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
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    /*
 * Namespaces.
 */
    #[no_mangle]
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr,
                   nameSpace: *const xmlChar) -> xmlNsPtr;
    #[no_mangle]
    fn xmlGetNsProp(node: *const xmlNode, name: *const xmlChar,
                    nameSpace: *const xmlChar) -> *mut xmlChar;
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
}
pub type xmlChar = std::os::raw::c_uchar;
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
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
pub type xmlNsPtr = *mut xmlNs;
pub type xlinkHRef = *mut xmlChar;
pub type xlinkRole = *mut xmlChar;
pub type xlinkTitle = *mut xmlChar;
pub type xlinkType = std::os::raw::c_uint;
pub const XLINK_TYPE_EXTENDED_SET: xlinkType = 3;
pub const XLINK_TYPE_EXTENDED: xlinkType = 2;
pub const XLINK_TYPE_SIMPLE: xlinkType = 1;
pub const XLINK_TYPE_NONE: xlinkType = 0;
pub type xlinkShow = std::os::raw::c_uint;
pub const XLINK_SHOW_REPLACE: xlinkShow = 3;
pub const XLINK_SHOW_EMBED: xlinkShow = 2;
pub const XLINK_SHOW_NEW: xlinkShow = 1;
pub const XLINK_SHOW_NONE: xlinkShow = 0;
pub type xlinkActuate = std::os::raw::c_uint;
pub const XLINK_ACTUATE_ONREQUEST: xlinkActuate = 2;
pub const XLINK_ACTUATE_AUTO: xlinkActuate = 1;
pub const XLINK_ACTUATE_NONE: xlinkActuate = 0;
pub type xlinkNodeDetectFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlNodePtr) -> ()>;
pub type xlinkSimpleLinkFunk
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlNodePtr,
                                _: xlinkHRef, _: xlinkRole, _: xlinkTitle)
               -> ()>;
pub type xlinkExtendedLinkFunk
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlNodePtr,
                                _: std::os::raw::c_int, _: *const xlinkHRef,
                                _: *const xlinkRole, _: std::os::raw::c_int,
                                _: *const xlinkRole, _: *const xlinkRole,
                                _: *mut xlinkShow, _: *mut xlinkActuate,
                                _: std::os::raw::c_int, _: *const xlinkTitle,
                                _: *mut *const xmlChar) -> ()>;
pub type xlinkExtendedLinkSetFunk
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: xmlNodePtr,
                                _: std::os::raw::c_int, _: *const xlinkHRef,
                                _: *const xlinkRole, _: std::os::raw::c_int,
                                _: *const xlinkTitle, _: *mut *const xmlChar)
               -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xlinkHandler {
    pub simple: xlinkSimpleLinkFunk,
    pub extended: xlinkExtendedLinkFunk,
    pub set: xlinkExtendedLinkSetFunk,
}
pub type xlinkHandler = _xlinkHandler;
pub type xlinkHandlerPtr = *mut xlinkHandler;
/* ***************************************************************
 *								*
 *           Default setting and related functions		*
 *								*
 ****************************************************************/
static mut xlinkDefaultHandler: xlinkHandlerPtr =
    0 as *const xlinkHandler as xlinkHandlerPtr;
static mut xlinkDefaultDetect: xlinkNodeDetectFunc = None;
/* *
 * xlinkGetDefaultHandler:
 *
 * Get the default xlink handler.
 *
 * Returns the current xlinkHandlerPtr value.
 */
#[no_mangle]
pub unsafe extern "C" fn xlinkGetDefaultHandler() -> xlinkHandlerPtr {
    return xlinkDefaultHandler;
}
/* *
 * xlinkSetDefaultHandler:
 * @handler:  the new value for the xlink handler block
 *
 * Set the default xlink handlers
 */
#[no_mangle]
pub unsafe extern "C" fn xlinkSetDefaultHandler(mut handler:
                                                    xlinkHandlerPtr) {
    xlinkDefaultHandler = handler;
}
/* *
 * xlinkGetDefaultDetect:
 *
 * Get the default xlink detection routine
 *
 * Returns the current function or NULL;
 */
#[no_mangle]
pub unsafe extern "C" fn xlinkGetDefaultDetect() -> xlinkNodeDetectFunc {
    return xlinkDefaultDetect;
}
/* *
 * xlinkSetDefaultDetect:
 * @func: pointer to the new detection routine.
 *
 * Set the default xlink detection routine
 */
#[no_mangle]
pub unsafe extern "C" fn xlinkSetDefaultDetect(mut func:
                                                   xlinkNodeDetectFunc) {
    xlinkDefaultDetect = func;
}
/*
 * Summary: unfinished XLink detection module
 * Description: unfinished XLink detection module
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * Various defines for the various Link properties.
 *
 * NOTE: the link detection layer will try to resolve QName expansion
 *       of namespaces. If "foo" is the prefix for "http://foo.com/"
 *       then the link detection layer will expand role="foo:myrole"
 *       to "http://foo.com/:myrole".
 * NOTE: the link detection layer will expand URI-Refences found on
 *       href attributes by using the base mechanism if found.
 */
/* *
 * xlinkNodeDetectFunc:
 * @ctx:  user data pointer
 * @node:  the node to check
 *
 * This is the prototype for the link detection routine.
 * It calls the default link detection callbacks upon link detection.
 */
/*
 * The link detection module interact with the upper layers using
 * a set of callback registered at parsing time.
 */
/* *
 * xlinkSimpleLinkFunk:
 * @ctx:  user data pointer
 * @node:  the node carrying the link
 * @href:  the target of the link
 * @role:  the role string
 * @title:  the link title
 *
 * This is the prototype for a simple link detection callback.
 */
/* *
 * xlinkExtendedLinkFunk:
 * @ctx:  user data pointer
 * @node:  the node carrying the link
 * @nbLocators: the number of locators detected on the link
 * @hrefs:  pointer to the array of locator hrefs
 * @roles:  pointer to the array of locator roles
 * @nbArcs: the number of arcs detected on the link
 * @from:  pointer to the array of source roles found on the arcs
 * @to:  pointer to the array of target roles found on the arcs
 * @show:  array of values for the show attributes found on the arcs
 * @actuate:  array of values for the actuate attributes found on the arcs
 * @nbTitles: the number of titles detected on the link
 * @title:  array of titles detected on the link
 * @langs:  array of xml:lang values for the titles
 *
 * This is the prototype for a extended link detection callback.
 */
/* *
 * xlinkExtendedLinkSetFunk:
 * @ctx:  user data pointer
 * @node:  the node carrying the link
 * @nbLocators: the number of locators detected on the link
 * @hrefs:  pointer to the array of locator hrefs
 * @roles:  pointer to the array of locator roles
 * @nbTitles: the number of titles detected on the link
 * @title:  array of titles detected on the link
 * @langs:  array of xml:lang values for the titles
 *
 * This is the prototype for a extended link set detection callback.
 */
/* *
 * This is the structure containing a set of Links detection callbacks.
 *
 * There is no default xlink callbacks, if one want to get link
 * recognition activated, those call backs must be provided before parsing.
 */
/*
 * The default detection routine, can be overridden, they call the default
 * detection callbacks.
 */
/*
 * Routines to set/get the default handlers.
 */
/*
 * Link detection module itself.
 */
/* ***************************************************************
 *								*
 *                  The detection routines			*
 *								*
 ****************************************************************/
/* *
 * xlinkIsLink:
 * @doc:  the document containing the node
 * @node:  the node pointer itself
 *
 * Check whether the given node carries the attributes needed
 * to be a link element (or is one of the linking elements issued
 * from the (X)HTML DtDs).
 * This routine don't try to do full checking of the link validity
 * but tries to detect and return the appropriate link type.
 *
 * Returns the xlinkType of the node (XLINK_TYPE_NONE if there is no
 *         link detected.
 */
#[no_mangle]
pub unsafe extern "C" fn xlinkIsLink(mut doc: xmlDocPtr, mut node: xmlNodePtr)
 -> xlinkType {
    let mut type_0: *mut xmlChar = 0 as *mut xmlChar;
    let mut role: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: xlinkType = XLINK_TYPE_NONE;
    if node.is_null() { return XLINK_TYPE_NONE }
    if doc.is_null() { doc = (*node).doc }
    if !(!doc.is_null() &&
             (*doc).type_0 as std::os::raw::c_uint ==
                 XML_HTML_DOCUMENT_NODE as std::os::raw::c_int as std::os::raw::c_uint) {
        (!(*node).ns.is_null()) &&
            xmlStrEqual((*(*node).ns).href,
                        b"http://www.w3.org/1999/xhtml/\x00" as *const u8 as
                            *const std::os::raw::c_char as *mut xmlChar) != 0;
    }
    /*
     * We don't prevent a-priori having XML Linking constructs on
     * XHTML elements
     */
    type_0 =
        xmlGetNsProp(node as *const xmlNode,
                     b"type\x00" as *const u8 as *const std::os::raw::c_char as
                         *mut xmlChar,
                     b"http://www.w3.org/1999/xlink/namespace/\x00" as
                         *const u8 as *const std::os::raw::c_char as *mut xmlChar);
    if !type_0.is_null() {
        if xmlStrEqual(type_0,
                       b"simple\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar) != 0 {
            ret = XLINK_TYPE_SIMPLE
        } else if xmlStrEqual(type_0,
                              b"extended\x00" as *const u8 as
                                  *const std::os::raw::c_char as *mut xmlChar) != 0 {
            role =
                xmlGetNsProp(node as *const xmlNode,
                             b"role\x00" as *const u8 as *const std::os::raw::c_char
                                 as *mut xmlChar,
                             b"http://www.w3.org/1999/xlink/namespace/\x00" as
                                 *const u8 as *const std::os::raw::c_char as
                                 *mut xmlChar);
            if !role.is_null() {
                let mut xlink: xmlNsPtr = 0 as *mut xmlNs;
                xlink =
                    xmlSearchNs(doc, node,
                                b"http://www.w3.org/1999/xlink/namespace/\x00"
                                    as *const u8 as *const std::os::raw::c_char as
                                    *mut xmlChar);
                if xlink.is_null() {
                    /* Humm, fallback method */
                    if xmlStrEqual(role,
                                   b"xlink:external-linkset\x00" as *const u8
                                       as *const std::os::raw::c_char as *mut xmlChar)
                           != 0 {
                        ret = XLINK_TYPE_EXTENDED_SET
                    }
                } else {
                    let mut buf: [xmlChar; 200] = [0; 200];
                    snprintf(buf.as_mut_ptr() as *mut std::os::raw::c_char,
                             ::std::mem::size_of::<[xmlChar; 200]>() as
                                 std::os::raw::c_ulong,
                             b"%s:external-linkset\x00" as *const u8 as
                                 *const std::os::raw::c_char,
                             (*xlink).prefix as *mut std::os::raw::c_char);
                    buf[(::std::mem::size_of::<[xmlChar; 200]>() as
                             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                            usize] = 0 as std::os::raw::c_int as xmlChar;
                    if xmlStrEqual(role, buf.as_mut_ptr()) != 0 {
                        ret = XLINK_TYPE_EXTENDED_SET
                    }
                }
            }
            ret = XLINK_TYPE_EXTENDED
        }
    }
    if !type_0.is_null() {
        xmlFree.expect("non-null function pointer")(type_0 as
                                                        *mut std::os::raw::c_void);
    }
    if !role.is_null() {
        xmlFree.expect("non-null function pointer")(role as
                                                        *mut std::os::raw::c_void);
    }
    return ret;
}
/* LIBXML_XPTR_ENABLED */