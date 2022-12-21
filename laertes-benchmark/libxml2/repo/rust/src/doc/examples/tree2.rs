
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
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    /*
 * Creating/freeing new structures.
 */
    #[no_mangle]
    fn xmlCreateIntSubset(doc: xmlDocPtr, name: *const xmlChar,
                          ExternalID: *const xmlChar,
                          SystemID: *const xmlChar) -> xmlDtdPtr;
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlNewProp(node: xmlNodePtr, name: *const xmlChar,
                  value: *const xmlChar) -> xmlAttrPtr;
    #[no_mangle]
    fn xmlNewNode(ns: xmlNsPtr, name: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewChild(parent: xmlNodePtr, ns: xmlNsPtr, name: *const xmlChar,
                   content: *const xmlChar) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
    /*
 * Changing the structure.
 */
    #[no_mangle]
    fn xmlDocSetRootElement(doc: xmlDocPtr, root: xmlNodePtr) -> xmlNodePtr;
    /* LIBXML_TREE_ENABLED */
    #[no_mangle]
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlSaveFormatFileEnc(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                            encoding: *const std::os::raw::c_char,
                            format: std::os::raw::c_int) -> std::os::raw::c_int;
    /*
 * Summary: compile-time version informations
 * Description: compile-time version informations for the XML library
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /*
 * use those to be sure nothing nasty will happen if
 * your library and includes mismatch
 */
    #[no_mangle]
    fn xmlCheckVersion(version: std::os::raw::c_int);
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlCleanupParser();
}
pub type xmlChar = std::os::raw::c_uchar;
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
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
/* 
 * section:  Tree
 * synopsis: Creates a tree
 * purpose:  Shows how to create document, nodes and dump it to stdout or file.
 * usage:    tree2 <filename>  -Default output: stdout
 * test:     tree2 > tree2.tmp && diff tree2.tmp $(srcdir)/tree2.res
 * author:   Lucas Brasilino <brasilino@recife.pe.gov.br>
 * copy:     see Copyright for the status of this software
 */
/*
 *To compile this file using gcc you can type
 *gcc `xml2-config --cflags --libs` -o tree2 tree2.c
 */
/* A simple example how to create DOM. Libxml2 automagically 
 * allocates the necessary amount of memory to it.
*/
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr; /* document pointer */
    let mut root_node: xmlNodePtr = 0 as xmlNodePtr; /* node pointers */
    let mut node: xmlNodePtr = 0 as xmlNodePtr;
    let mut node1: xmlNodePtr = 0 as xmlNodePtr;
    let mut buff: [std::os::raw::c_char; 256] = [0; 256];
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    xmlCheckVersion(20908 as std::os::raw::c_int);
    /* 
     * Creates a new document, a node and set it as a root node
     */
    doc =
        xmlNewDoc(b"1.0\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar);
    root_node =
        xmlNewNode(0 as xmlNsPtr,
                   b"root\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    xmlDocSetRootElement(doc, root_node);
    /*
     * Creates a DTD declaration. Isn't mandatory. 
     */
    xmlCreateIntSubset(doc,
                       b"root\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar, 0 as *const xmlChar,
                       b"tree2.dtd\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar);
    /* 
     * xmlNewChild() creates a new node, which is "attached" as child node
     * of root_node node. 
     */
    xmlNewChild(root_node, 0 as xmlNsPtr,
                b"node1\x00" as *const u8 as *const std::os::raw::c_char as
                    *mut xmlChar,
                b"content of node 1\x00" as *const u8 as *const std::os::raw::c_char
                    as *mut xmlChar);
    /* 
     * The same as above, but the new child node doesn't have a content 
     */
    xmlNewChild(root_node, 0 as xmlNsPtr,
                b"node2\x00" as *const u8 as *const std::os::raw::c_char as
                    *mut xmlChar, 0 as *const xmlChar);
    /* 
     * xmlNewProp() creates attributes, which is "attached" to an node.
     * It returns xmlAttrPtr, which isn't used here.
     */
    node =
        xmlNewChild(root_node, 0 as xmlNsPtr,
                    b"node3\x00" as *const u8 as *const std::os::raw::c_char as
                        *mut xmlChar,
                    b"this node has attributes\x00" as *const u8 as
                        *const std::os::raw::c_char as *mut xmlChar);
    xmlNewProp(node,
               b"attribute\x00" as *const u8 as *const std::os::raw::c_char as
                   *mut xmlChar,
               b"yes\x00" as *const u8 as *const std::os::raw::c_char as
                   *mut xmlChar);
    xmlNewProp(node,
               b"foo\x00" as *const u8 as *const std::os::raw::c_char as *mut xmlChar,
               b"bar\x00" as *const u8 as *const std::os::raw::c_char as
                   *mut xmlChar);
    /*
     * Here goes another way to create nodes. xmlNewNode() and xmlNewText
     * creates a node and a text node separately. They are "attached"
     * by xmlAddChild() 
     */
    node =
        xmlNewNode(0 as xmlNsPtr,
                   b"node4\x00" as *const u8 as *const std::os::raw::c_char as
                       *mut xmlChar);
    node1 =
        xmlNewText(b"other way to create content (which is also a node)\x00"
                       as *const u8 as *const std::os::raw::c_char as *mut xmlChar);
    xmlAddChild(node, node1);
    xmlAddChild(root_node, node);
    /* 
     * A simple loop that "automates" nodes creation 
     */
    i = 5 as std::os::raw::c_int;
    while i < 7 as std::os::raw::c_int {
        sprintf(buff.as_mut_ptr(),
                b"node%d\x00" as *const u8 as *const std::os::raw::c_char, i);
        node =
            xmlNewChild(root_node, 0 as xmlNsPtr,
                        buff.as_mut_ptr() as *mut xmlChar,
                        0 as *const xmlChar);
        j = 1 as std::os::raw::c_int;
        while j < 4 as std::os::raw::c_int {
            sprintf(buff.as_mut_ptr(),
                    b"node%d%d\x00" as *const u8 as *const std::os::raw::c_char, i,
                    j);
            node1 =
                xmlNewChild(node, 0 as xmlNsPtr,
                            buff.as_mut_ptr() as *mut xmlChar,
                            0 as *const xmlChar);
            xmlNewProp(node1,
                       b"odd\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut xmlChar,
                       if j % 2 as std::os::raw::c_int != 0 {
                           b"no\x00" as *const u8 as *const std::os::raw::c_char
                       } else {
                           b"yes\x00" as *const u8 as *const std::os::raw::c_char
                       } as *mut xmlChar);
            j += 1
        }
        i += 1
    }
    /* 
     * Dumping document to stdio or file
     */
    xmlSaveFormatFileEnc(if argc > 1 as std::os::raw::c_int {
                             *argv.offset(1 as std::os::raw::c_int as isize)
                         } else {
                             b"-\x00" as *const u8 as *const std::os::raw::c_char
                         }, doc,
                         b"UTF-8\x00" as *const u8 as *const std::os::raw::c_char,
                         1 as std::os::raw::c_int);
    /*free the document */
    xmlFreeDoc(doc);
    /*
     *Free the global variables that may
     *have been allocated by the parser.
     */
    xmlCleanupParser();
    /*
     * this is to debug memory for regression tests
     */
    xmlMemoryDump();
    return 0 as std::os::raw::c_int;
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
