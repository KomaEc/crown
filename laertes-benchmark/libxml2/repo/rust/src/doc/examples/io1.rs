
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
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlFreeDoc(cur_0: xmlDocPtr);
    #[no_mangle]
    fn xmlDocDump(f: *mut FILE, cur_0: xmlDocPtr) -> std::os::raw::c_int;
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
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlRegisterInputCallbacks(matchFunc: xmlInputMatchCallback,
                                 openFunc: xmlInputOpenCallback,
                                 readFunc: xmlInputReadCallback,
                                 closeFunc: xmlInputCloseCallback)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlReadMemory(buffer: *const std::os::raw::c_char, size: std::os::raw::c_int,
                     URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                     options: std::os::raw::c_int) -> xmlDocPtr;
    /*
 * standalone processing
 */
    #[no_mangle]
    fn xmlXIncludeProcess(doc: xmlDocPtr) -> std::os::raw::c_int;
}
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
pub type xmlChar = std::os::raw::c_uchar;
pub type xmlInputCloseCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>;
pub type xmlInputReadCallback
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_char,
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
pub type xmlInputMatchCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> std::os::raw::c_int>;
pub type xmlInputOpenCallback
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_void>;
/* *
 * section: InputOutput
 * synopsis: Example of custom Input/Output
 * purpose: Demonstrate the use of xmlRegisterInputCallbacks
 *          to build a custom I/O layer, this is used in an
 *          XInclude method context to show how dynamic document can
 *          be built in a clean way.
 * usage: io1
 * test: io1 > io1.tmp && diff io1.tmp $(srcdir)/io1.res
 * author: Daniel Veillard
 * copy: see Copyright for the status of this software.
 */
static mut result: *const std::os::raw::c_char =
    b"<list><people>a</people><people>b</people></list>\x00" as *const u8 as
        *const std::os::raw::c_char;
static mut cur: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
static mut rlen: std::os::raw::c_int = 0;
/* *
 * sqlMatch:
 * @URI: an URI to test
 *
 * Check for an sql: query
 *
 * Returns 1 if yes and 0 if another Input module should be used
 */
unsafe extern "C" fn sqlMatch(mut URI: *const std::os::raw::c_char) -> std::os::raw::c_int {
    if !URI.is_null() &&
           strncmp(URI, b"sql:\x00" as *const u8 as *const std::os::raw::c_char,
                   4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/* *
 * sqlOpen:
 * @URI: an URI to test
 *
 * Return a pointer to the sql: query handler, in this example simply
 * the current pointer...
 *
 * Returns an Input context or NULL in case or error
 */
unsafe extern "C" fn sqlOpen(mut URI: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    if URI.is_null() ||
           strncmp(URI, b"sql:\x00" as *const u8 as *const std::os::raw::c_char,
                   4 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 {
        return 0 as *mut std::os::raw::c_void
    }
    cur = result;
    rlen = strlen(result) as std::os::raw::c_int;
    return cur as *mut std::os::raw::c_void;
}
/* *
 * sqlClose:
 * @context: the read context
 *
 * Close the sql: query handler
 *
 * Returns 0 or -1 in case of error
 */
unsafe extern "C" fn sqlClose(mut context: *mut std::os::raw::c_void) -> std::os::raw::c_int {
    if context.is_null() { return -(1 as std::os::raw::c_int) }
    cur = 0 as *const std::os::raw::c_char;
    rlen = 0 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/* *
 * sqlRead:
 * @context: the read context
 * @buffer: where to store data
 * @len: number of bytes to read
 *
 * Implement an sql: query read.
 *
 * Returns the number of bytes read or -1 in case of error
 */
unsafe extern "C" fn sqlRead(mut context: *mut std::os::raw::c_void,
                             mut buffer: *mut std::os::raw::c_char,
                             mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut ptr: *const std::os::raw::c_char = context as *const std::os::raw::c_char;
    if context.is_null() || buffer.is_null() || len < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int)
    }
    if len > rlen { len = rlen }
    memcpy(buffer as *mut std::os::raw::c_void, ptr as *const std::os::raw::c_void,
           len as std::os::raw::c_ulong);
    rlen -= len;
    return len;
}
#[no_mangle]
pub static mut include: *const std::os::raw::c_char =
    b"<?xml version=\'1.0\'?>\n<document xmlns:xi=\"http://www.w3.org/2003/XInclude\">\n  <p>List of people:</p>\n  <xi:include href=\"sql:select_name_from_people\"/>\n</document>\n\x00"
        as *const u8 as *const std::os::raw::c_char;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    /*
     * this initialize the library and check potential ABI mismatches
     * between the version it was compiled for and the actual shared
     * library used.
     */
    xmlCheckVersion(20908 as std::os::raw::c_int);
    /*
     * register the new I/O handlers
     */
    if xmlRegisterInputCallbacks(Some(sqlMatch as
                                          unsafe extern "C" fn(_:
                                                                   *const std::os::raw::c_char)
                                              -> std::os::raw::c_int),
                                 Some(sqlOpen as
                                          unsafe extern "C" fn(_:
                                                                   *const std::os::raw::c_char)
                                              -> *mut std::os::raw::c_void),
                                 Some(sqlRead as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void,
                                                               _:
                                                                   *mut std::os::raw::c_char,
                                                               _: std::os::raw::c_int)
                                              -> std::os::raw::c_int),
                                 Some(sqlClose as
                                          unsafe extern "C" fn(_:
                                                                   *mut std::os::raw::c_void)
                                              -> std::os::raw::c_int)) <
           0 as std::os::raw::c_int {
        fprintf(stderr,
                b"failed to register SQL handler\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    /*
     * parse include into a document
     */
    doc =
        xmlReadMemory(include, strlen(include) as std::os::raw::c_int,
                      b"include.xml\x00" as *const u8 as *const std::os::raw::c_char,
                      0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"failed to parse the including file\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    /*
     * apply the XInclude process, this should trigger the I/O just
     * registered.
     */
    if xmlXIncludeProcess(doc) <= 0 as std::os::raw::c_int {
        fprintf(stderr,
                b"XInclude processing failed\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    /*
     * save the output for checking to stdout
     */
    xmlDocDump(stdout, doc);
    /*
     * Free the document
     */
    xmlFreeDoc(doc);
    /*
     * Cleanup function for the XML library.
     */
    xmlCleanupParser();
    /*
     * this is to debug memory for regression tests
     */
    xmlMemoryDump();
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
