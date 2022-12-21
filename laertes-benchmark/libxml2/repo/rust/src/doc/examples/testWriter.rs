
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
    pub type _xmlTextWriter;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(__filename: *const std::os::raw::c_char, __modes: *const std::os::raw::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn xmlBufferCreate() -> xmlBufferPtr;
    #[no_mangle]
    fn xmlBufferFree(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_SCHEMAS_ENABLED) */
    /*
 * Creating new nodes.
 */
    #[no_mangle]
    fn xmlNewDocNode(doc: xmlDocPtr, ns: xmlNsPtr, name: *const xmlChar,
                     content: *const xmlChar) -> xmlNodePtr;
    /*
 * Changing the structure.
 */
    #[no_mangle]
    fn xmlDocSetRootElement(doc: xmlDocPtr, root: xmlNodePtr) -> xmlNodePtr;
    #[no_mangle]
    fn xmlSaveFileEnc(filename: *const std::os::raw::c_char, cur: xmlDocPtr,
                      encoding: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlCleanupParser();
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
    static mut xmlRealloc: xmlReallocFunc;
    #[no_mangle]
    static mut xmlFree: xmlFreeFunc;
    #[no_mangle]
    fn xmlFindCharEncodingHandler(name: *const std::os::raw::c_char)
     -> xmlCharEncodingHandlerPtr;
    #[no_mangle]
    fn xmlNewTextWriterFilename(uri: *const std::os::raw::c_char,
                                compression: std::os::raw::c_int) -> xmlTextWriterPtr;
    #[no_mangle]
    fn xmlNewTextWriterMemory(buf: xmlBufferPtr, compression: std::os::raw::c_int)
     -> xmlTextWriterPtr;
    #[no_mangle]
    fn xmlNewTextWriterDoc(doc: *mut xmlDocPtr, compression: std::os::raw::c_int)
     -> xmlTextWriterPtr;
    #[no_mangle]
    fn xmlNewTextWriterTree(doc: xmlDocPtr, node: xmlNodePtr,
                            compression: std::os::raw::c_int) -> xmlTextWriterPtr;
    #[no_mangle]
    fn xmlFreeTextWriter(writer: xmlTextWriterPtr);
    /*
 * Functions
 */
    /*
 * Document
 */
    #[no_mangle]
    fn xmlTextWriterStartDocument(writer: xmlTextWriterPtr,
                                  version: *const std::os::raw::c_char,
                                  encoding: *const std::os::raw::c_char,
                                  standalone: *const std::os::raw::c_char)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextWriterEndDocument(writer: xmlTextWriterPtr) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteFormatComment(writer: xmlTextWriterPtr,
                                       format: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteComment(writer: xmlTextWriterPtr,
                                 content: *const xmlChar) -> std::os::raw::c_int;
    /*
 * Elements
 */
    #[no_mangle]
    fn xmlTextWriterStartElement(writer: xmlTextWriterPtr,
                                 name: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextWriterEndElement(writer: xmlTextWriterPtr) -> std::os::raw::c_int;
    /*
 * Elements conveniency functions
 */
    #[no_mangle]
    fn xmlTextWriterWriteFormatElement(writer: xmlTextWriterPtr,
                                       name: *const xmlChar,
                                       format: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteElement(writer: xmlTextWriterPtr,
                                 name: *const xmlChar,
                                 content: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteAttribute(writer: xmlTextWriterPtr,
                                   name: *const xmlChar,
                                   content: *const xmlChar) -> std::os::raw::c_int;
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
pub type iconv_t = *mut std::os::raw::c_void;
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
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
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
 * Summary: text writing API for XML
 * Description: text writing API for XML
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Alfred Mickautsch <alfred@mickautsch.de>
 */
pub type xmlTextWriter = _xmlTextWriter;
pub type xmlTextWriterPtr = *mut xmlTextWriter;
unsafe fn main_0() -> std::os::raw::c_int {
    /*
     * this initialize the library and check potential ABI mismatches
     * between the version it was compiled for and the actual shared
     * library used.
     */
    xmlCheckVersion(20908 as std::os::raw::c_int);
    /* first, the file version */
    testXmlwriterFilename(b"writer1.tmp\x00" as *const u8 as
                              *const std::os::raw::c_char);
    /* next, the memory version */
    testXmlwriterMemory(b"writer2.tmp\x00" as *const u8 as
                            *const std::os::raw::c_char);
    /* next, the DOM version */
    testXmlwriterDoc(b"writer3.tmp\x00" as *const u8 as *const std::os::raw::c_char);
    /* next, the tree version */
    testXmlwriterTree(b"writer4.tmp\x00" as *const u8 as *const std::os::raw::c_char);
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
/* *
 * testXmlwriterFilename:
 * @uri: the output URI
 *
 * test the xmlWriter interface when writing to a new file
 */
#[no_mangle]
pub unsafe extern "C" fn testXmlwriterFilename(mut uri: *const std::os::raw::c_char) {
    let mut rc: std::os::raw::c_int = 0;
    let mut writer: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    /* Create a new XmlWriter for uri, with no compression. */
    writer = xmlNewTextWriterFilename(uri, 0 as std::os::raw::c_int);
    if writer.is_null() {
        printf(b"testXmlwriterFilename: Error creating the xml writer\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start the document with the xml default for the version,
     * encoding ISO 8859-1 and the default for the standalone
     * declaration. */
    rc =
        xmlTextWriterStartDocument(writer, 0 as *const std::os::raw::c_char,
                                   b"ISO-8859-1\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   0 as *const std::os::raw::c_char);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "EXAMPLE". Since thist is the first
     * element, this will be the root element of the document. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"EXAMPLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of EXAMPLE.
     * Please observe, that the input to the xmlTextWriter functions
     * HAS to be in UTF-8, even if the output XML is encoded
     * in iso-8859-1 */
    tmp =
        ConvertInput(b"This is a comment with special chars: <\xe4\xf6\xfc>\x00"
                         as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc = xmlTextWriterWriteComment(writer, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "ORDER" as child of EXAMPLE. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ORDER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "version" and value "1.0" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"version\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"1.0\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "xml:lang" and value "de" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"xml:lang\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"de\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of ORDER */
    tmp =
        ConvertInput(b"<\xe4\xf6\xfc>\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteFormatComment(writer,
                                        b"This is another comment with special chars: %s\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteFormatComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "HEADER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"HEADER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "X_ORDER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"X_ORDER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%010d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        53535 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "CUSTOMER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"CUSTOMER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        1010 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "NAME_1" as child of HEADER. */
    tmp =
        ConvertInput(b"M\xfcller\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_1\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Write an element named "NAME_2" as child of HEADER. */
    tmp =
        ConvertInput(b"J\xf6rg\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_2\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Close the element named HEADER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRIES" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRIES\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        10 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test 2>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        20 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRIES. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "FOOTER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"FOOTER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "TEXT" as child of FOOTER. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"TEXT\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"This is a text.\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named FOOTER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Here we could close the elements ORDER and EXAMPLE using the
     * function xmlTextWriterEndElement, but since we do not want to
     * write any other elements, we simply call xmlTextWriterEndDocument,
     * which will do all the work. */
    rc = xmlTextWriterEndDocument(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterFilename: Error at xmlTextWriterEndDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    xmlFreeTextWriter(writer);
}
/* *
 * testXmlwriterMemory:
 * @file: the output file
 *
 * test the xmlWriter interface when writing to memory
 */
#[no_mangle]
pub unsafe extern "C" fn testXmlwriterMemory(mut file: *const std::os::raw::c_char) {
    let mut rc: std::os::raw::c_int = 0;
    let mut writer: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut buf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut fp: *mut FILE = 0 as *mut FILE;
    /* Create a new XML buffer, to which the XML document will be
     * written */
    buf = xmlBufferCreate();
    if buf.is_null() {
        printf(b"testXmlwriterMemory: Error creating the xml buffer\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Create a new XmlWriter for memory, with no compression.
     * Remark: there is no compression for this kind of xmlTextWriter */
    writer = xmlNewTextWriterMemory(buf, 0 as std::os::raw::c_int);
    if writer.is_null() {
        printf(b"testXmlwriterMemory: Error creating the xml writer\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start the document with the xml default for the version,
     * encoding ISO 8859-1 and the default for the standalone
     * declaration. */
    rc =
        xmlTextWriterStartDocument(writer, 0 as *const std::os::raw::c_char,
                                   b"ISO-8859-1\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   0 as *const std::os::raw::c_char);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "EXAMPLE". Since thist is the first
     * element, this will be the root element of the document. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"EXAMPLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of EXAMPLE.
     * Please observe, that the input to the xmlTextWriter functions
     * HAS to be in UTF-8, even if the output XML is encoded
     * in iso-8859-1 */
    tmp =
        ConvertInput(b"This is a comment with special chars: <\xe4\xf6\xfc>\x00"
                         as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc = xmlTextWriterWriteComment(writer, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "ORDER" as child of EXAMPLE. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ORDER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "version" and value "1.0" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"version\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"1.0\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "xml:lang" and value "de" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"xml:lang\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"de\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of ORDER */
    tmp =
        ConvertInput(b"<\xe4\xf6\xfc>\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteFormatComment(writer,
                                        b"This is another comment with special chars: %s\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteFormatComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "HEADER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"HEADER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "X_ORDER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"X_ORDER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%010d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        53535 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "CUSTOMER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"CUSTOMER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        1010 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "NAME_1" as child of HEADER. */
    tmp =
        ConvertInput(b"M\xfcller\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_1\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Write an element named "NAME_2" as child of HEADER. */
    tmp =
        ConvertInput(b"J\xf6rg\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_2\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Close the element named HEADER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRIES" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRIES\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        10 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test 2>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        20 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRIES. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "FOOTER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"FOOTER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "TEXT" as child of FOOTER. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"TEXT\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"This is a text.\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named FOOTER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterEndElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Here we could close the elements ORDER and EXAMPLE using the
     * function xmlTextWriterEndElement, but since we do not want to
     * write any other elements, we simply call xmlTextWriterEndDocument,
     * which will do all the work. */
    rc = xmlTextWriterEndDocument(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterMemory: Error at xmlTextWriterEndDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    xmlFreeTextWriter(writer);
    fp = fopen(file, b"w\x00" as *const u8 as *const std::os::raw::c_char);
    if fp.is_null() {
        printf(b"testXmlwriterMemory: Error at fopen\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
        return
    }
    fprintf(fp, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
            (*buf).content as *const std::os::raw::c_char);
    fclose(fp);
    xmlBufferFree(buf);
}
/* *
 * testXmlwriterDoc:
 * @file: the output file
 *
 * test the xmlWriter interface when creating a new document
 */
#[no_mangle]
pub unsafe extern "C" fn testXmlwriterDoc(mut file: *const std::os::raw::c_char) {
    let mut rc: std::os::raw::c_int = 0;
    let mut writer: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    /* Create a new XmlWriter for DOM, with no compression. */
    writer = xmlNewTextWriterDoc(&mut doc, 0 as std::os::raw::c_int);
    if writer.is_null() {
        printf(b"testXmlwriterDoc: Error creating the xml writer\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start the document with the xml default for the version,
     * encoding ISO 8859-1 and the default for the standalone
     * declaration. */
    rc =
        xmlTextWriterStartDocument(writer, 0 as *const std::os::raw::c_char,
                                   b"ISO-8859-1\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   0 as *const std::os::raw::c_char);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "EXAMPLE". Since thist is the first
     * element, this will be the root element of the document. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"EXAMPLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of EXAMPLE.
     * Please observe, that the input to the xmlTextWriter functions
     * HAS to be in UTF-8, even if the output XML is encoded
     * in iso-8859-1 */
    tmp =
        ConvertInput(b"This is a comment with special chars: <\xe4\xf6\xfc>\x00"
                         as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc = xmlTextWriterWriteComment(writer, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "ORDER" as child of EXAMPLE. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ORDER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "version" and value "1.0" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"version\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"1.0\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "xml:lang" and value "de" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"xml:lang\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"de\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of ORDER */
    tmp =
        ConvertInput(b"<\xe4\xf6\xfc>\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteFormatComment(writer,
                                        b"This is another comment with special chars: %s\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteFormatComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "HEADER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"HEADER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "X_ORDER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"X_ORDER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%010d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        53535 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "CUSTOMER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"CUSTOMER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        1010 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "NAME_1" as child of HEADER. */
    tmp =
        ConvertInput(b"M\xfcller\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_1\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Write an element named "NAME_2" as child of HEADER. */
    tmp =
        ConvertInput(b"J\xf6rg\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_2\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Close the element named HEADER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRIES" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRIES\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        10 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test 2>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        20 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRIES. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "FOOTER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"FOOTER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "TEXT" as child of FOOTER. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"TEXT\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"This is a text.\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named FOOTER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Here we could close the elements ORDER and EXAMPLE using the
     * function xmlTextWriterEndElement, but since we do not want to
     * write any other elements, we simply call xmlTextWriterEndDocument,
     * which will do all the work. */
    rc = xmlTextWriterEndDocument(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterDoc: Error at xmlTextWriterEndDocument\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    xmlFreeTextWriter(writer);
    xmlSaveFileEnc(file, doc,
                   b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    xmlFreeDoc(doc);
}
/* *
 * testXmlwriterTree:
 * @file: the output file
 *
 * test the xmlWriter interface when writing to a subtree
 */
#[no_mangle]
pub unsafe extern "C" fn testXmlwriterTree(mut file: *const std::os::raw::c_char) {
    let mut rc: std::os::raw::c_int = 0;
    let mut writer: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    /* Create a new XML DOM tree, to which the XML document will be
     * written */
    doc =
        xmlNewDoc(b"1.0\x00" as *const u8 as *const std::os::raw::c_char as
                      *mut xmlChar);
    if doc.is_null() {
        printf(b"testXmlwriterTree: Error creating the xml document tree\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Create a new XML node, to which the XML document will be
     * appended */
    node =
        xmlNewDocNode(doc, 0 as xmlNsPtr,
                      b"EXAMPLE\x00" as *const u8 as *const std::os::raw::c_char as
                          *mut xmlChar, 0 as *const xmlChar);
    if node.is_null() {
        printf(b"testXmlwriterTree: Error creating the xml node\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Make ELEMENT the root node of the tree */
    xmlDocSetRootElement(doc, node);
    /* Create a new XmlWriter for DOM tree, with no compression. */
    writer = xmlNewTextWriterTree(doc, node, 0 as std::os::raw::c_int);
    if writer.is_null() {
        printf(b"testXmlwriterTree: Error creating the xml writer\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start the document with the xml default for the version,
     * encoding ISO 8859-1 and the default for the standalone
     * declaration. */
    rc =
        xmlTextWriterStartDocument(writer, 0 as *const std::os::raw::c_char,
                                   b"ISO-8859-1\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   0 as *const std::os::raw::c_char);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of EXAMPLE.
     * Please observe, that the input to the xmlTextWriter functions
     * HAS to be in UTF-8, even if the output XML is encoded
     * in iso-8859-1 */
    tmp =
        ConvertInput(b"This is a comment with special chars: <\xe4\xf6\xfc>\x00"
                         as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc = xmlTextWriterWriteComment(writer, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "ORDER" as child of EXAMPLE. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ORDER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "version" and value "1.0" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"version\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"1.0\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Add an attribute with name "xml:lang" and value "de" to ORDER. */
    rc =
        xmlTextWriterWriteAttribute(writer,
                                    b"xml:lang\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar,
                                    b"de\x00" as *const u8 as
                                        *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteAttribute\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write a comment as child of ORDER */
    tmp =
        ConvertInput(b"<\xe4\xf6\xfc>\x00" as *const u8 as
                         *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteFormatComment(writer,
                                        b"This is another comment with special chars: %s\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char, tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteFormatComment\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Start an element named "HEADER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"HEADER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "X_ORDER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"X_ORDER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%010d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        53535 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "CUSTOMER_ID" as child of HEADER. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"CUSTOMER_ID\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        1010 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "NAME_1" as child of HEADER. */
    tmp =
        ConvertInput(b"M\xfcller\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_1\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Write an element named "NAME_2" as child of HEADER. */
    tmp =
        ConvertInput(b"J\xf6rg\x00" as *const u8 as *const std::os::raw::c_char,
                     b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"NAME_2\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  tmp);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    if !tmp.is_null() {
        xmlFree.expect("non-null function pointer")(tmp as *mut std::os::raw::c_void);
    }
    /* Close the element named HEADER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRIES" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRIES\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        10 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "ENTRY" as child of ENTRIES. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"ENTRY\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ARTICLE" as child of ENTRY. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"ARTICLE\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"<Test 2>\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "ENTRY_NO" as child of ENTRY. */
    rc =
        xmlTextWriterWriteFormatElement(writer,
                                        b"ENTRY_NO\x00" as *const u8 as
                                            *const std::os::raw::c_char as
                                            *mut xmlChar,
                                        b"%d\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        20 as std::os::raw::c_int);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteFormatElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRY. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named ENTRIES. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Start an element named "FOOTER" as child of ORDER. */
    rc =
        xmlTextWriterStartElement(writer,
                                  b"FOOTER\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterStartElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Write an element named "TEXT" as child of FOOTER. */
    rc =
        xmlTextWriterWriteElement(writer,
                                  b"TEXT\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar,
                                  b"This is a text.\x00" as *const u8 as
                                      *const std::os::raw::c_char as *mut xmlChar);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterWriteElement\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Close the element named FOOTER. */
    rc = xmlTextWriterEndElement(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterEndElement\n\x00" as
                   *const u8 as *const std::os::raw::c_char);
        return
    }
    /* Here we could close the elements ORDER and EXAMPLE using the
     * function xmlTextWriterEndElement, but since we do not want to
     * write any other elements, we simply call xmlTextWriterEndDocument,
     * which will do all the work. */
    rc = xmlTextWriterEndDocument(writer);
    if rc < 0 as std::os::raw::c_int {
        printf(b"testXmlwriterTree: Error at xmlTextWriterEndDocument\n\x00"
                   as *const u8 as *const std::os::raw::c_char);
        return
    }
    xmlFreeTextWriter(writer);
    xmlSaveFileEnc(file, doc,
                   b"ISO-8859-1\x00" as *const u8 as *const std::os::raw::c_char);
    xmlFreeDoc(doc);
}
/* *
 * ConvertInput:
 * @in: string in a given encoding
 * @encoding: the encoding used
 *
 * Converts @in into UTF-8 for processing with libxml2 APIs
 *
 * Returns the converted UTF-8 string, or NULL in case of error.
 */
#[no_mangle]
pub unsafe extern "C" fn ConvertInput(mut in_0: *const std::os::raw::c_char,
                                      mut encoding: *const std::os::raw::c_char)
 -> *mut xmlChar {
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0;
    let mut size: std::os::raw::c_int = 0;
    let mut out_size: std::os::raw::c_int = 0;
    let mut temp: std::os::raw::c_int = 0;
    let mut handler: xmlCharEncodingHandlerPtr =
        0 as *mut xmlCharEncodingHandler;
    if in_0.is_null() { return 0 as *mut xmlChar }
    handler = xmlFindCharEncodingHandler(encoding);
    if handler.is_null() {
        printf(b"ConvertInput: no encoding handler found for \'%s\'\n\x00" as
                   *const u8 as *const std::os::raw::c_char,
               if !encoding.is_null() {
                   encoding
               } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        return 0 as *mut xmlChar
    }
    size = strlen(in_0) as std::os::raw::c_int + 1 as std::os::raw::c_int;
    out_size = size * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int;
    out =
        xmlMalloc.expect("non-null function pointer")(out_size as size_t) as
            *mut std::os::raw::c_uchar;
    if !out.is_null() {
        temp = size - 1 as std::os::raw::c_int;
        ret =
            (*handler).input.expect("non-null function pointer")(out,
                                                                 &mut out_size,
                                                                 in_0 as
                                                                     *const xmlChar,
                                                                 &mut temp);
        if ret < 0 as std::os::raw::c_int || temp - size + 1 as std::os::raw::c_int != 0 {
            if ret < 0 as std::os::raw::c_int {
                printf(b"ConvertInput: conversion wasn\'t successful.\n\x00"
                           as *const u8 as *const std::os::raw::c_char);
            } else {
                printf(b"ConvertInput: conversion wasn\'t successful. converted: %i octets.\n\x00"
                           as *const u8 as *const std::os::raw::c_char, temp);
            }
            xmlFree.expect("non-null function pointer")(out as
                                                            *mut std::os::raw::c_void);
            out = 0 as *mut xmlChar
        } else {
            out =
                xmlRealloc.expect("non-null function pointer")(out as
                                                                   *mut std::os::raw::c_void,
                                                               (out_size +
                                                                    1 as
                                                                        std::os::raw::c_int)
                                                                   as size_t)
                    as *mut std::os::raw::c_uchar;
            *out.offset(out_size as isize) = 0 as std::os::raw::c_int as xmlChar
            /*null terminating out */
        }
    } else {
        printf(b"ConvertInput: no mem\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
    }
    return out;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
