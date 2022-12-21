#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main,
           register_tool)]
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
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(__s: *const std::os::raw::c_char);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlInitParser();
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlParseFile(filename: *const std::os::raw::c_char) -> xmlDocPtr;
    #[no_mangle]
    fn __xmlGenericErrorContext() -> *mut *mut std::os::raw::c_void;
    #[no_mangle]
    fn __xmlDoValidityCheckingDefaultValue() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlLoadCatalog(filename: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlCatalogCleanup();
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                                     -> *mut std::os::raw::c_void>,
                      __arg: *mut std::os::raw::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut std::os::raw::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
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
pub type pthread_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [std::os::raw::c_char; 56],
    pub __align: std::os::raw::c_long,
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
static mut tid: [pthread_t; 20] = [0; 20];
static mut catalog: *const std::os::raw::c_char =
    b"test/threads/complex.xml\x00" as *const u8 as *const std::os::raw::c_char;
static mut testfiles: [*const std::os::raw::c_char; 7] =
    [b"test/threads/abc.xml\x00" as *const u8 as *const std::os::raw::c_char,
     b"test/threads/acb.xml\x00" as *const u8 as *const std::os::raw::c_char,
     b"test/threads/bac.xml\x00" as *const u8 as *const std::os::raw::c_char,
     b"test/threads/bca.xml\x00" as *const u8 as *const std::os::raw::c_char,
     b"test/threads/cab.xml\x00" as *const u8 as *const std::os::raw::c_char,
     b"test/threads/cba.xml\x00" as *const u8 as *const std::os::raw::c_char,
     b"test/threads/invalid.xml\x00" as *const u8 as *const std::os::raw::c_char];
static mut Okay: *const std::os::raw::c_char =
    b"OK\x00" as *const u8 as *const std::os::raw::c_char;
static mut Failed: *const std::os::raw::c_char =
    b"Failed\x00" as *const u8 as *const std::os::raw::c_char;
unsafe extern "C" fn thread_specific_data(mut private_data: *mut std::os::raw::c_void)
 -> *mut std::os::raw::c_void {
    let mut myDoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut filename: *const std::os::raw::c_char =
        private_data as *const std::os::raw::c_char;
    let mut okay: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if strcmp(filename,
              b"test/threads/invalid.xml\x00" as *const u8 as
                  *const std::os::raw::c_char) == 0 {
        *__xmlDoValidityCheckingDefaultValue() = 0 as std::os::raw::c_int;
        let ref mut fresh0 = *__xmlGenericErrorContext();
        *fresh0 = stdout as *mut std::os::raw::c_void
    } else {
        *__xmlDoValidityCheckingDefaultValue() = 1 as std::os::raw::c_int;
        let ref mut fresh1 = *__xmlGenericErrorContext();
        *fresh1 = stderr as *mut std::os::raw::c_void
    }
    myDoc = xmlParseFile(filename);
    if !myDoc.is_null() {
        xmlFreeDoc(myDoc);
    } else {
        printf(b"parse failed\n\x00" as *const u8 as *const std::os::raw::c_char);
        okay = 0 as std::os::raw::c_int
    }
    if strcmp(filename,
              b"test/threads/invalid.xml\x00" as *const u8 as
                  *const std::os::raw::c_char) == 0 {
        if *__xmlDoValidityCheckingDefaultValue() != 0 as std::os::raw::c_int {
            printf(b"ValidityCheckingDefaultValue override failed\n\x00" as
                       *const u8 as *const std::os::raw::c_char);
            okay = 0 as std::os::raw::c_int
        }
        if *__xmlGenericErrorContext() != stdout as *mut std::os::raw::c_void {
            printf(b"xmlGenericErrorContext override failed\n\x00" as
                       *const u8 as *const std::os::raw::c_char);
            okay = 0 as std::os::raw::c_int
        }
    } else {
        if *__xmlDoValidityCheckingDefaultValue() != 1 as std::os::raw::c_int {
            printf(b"ValidityCheckingDefaultValue override failed\n\x00" as
                       *const u8 as *const std::os::raw::c_char);
            okay = 0 as std::os::raw::c_int
        }
        if *__xmlGenericErrorContext() != stderr as *mut std::os::raw::c_void {
            printf(b"xmlGenericErrorContext override failed\n\x00" as
                       *const u8 as *const std::os::raw::c_char);
            okay = 0 as std::os::raw::c_int
        }
    }
    if okay == 0 as std::os::raw::c_int { return Failed as *mut std::os::raw::c_void }
    return Okay as *mut std::os::raw::c_void;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_uint = 0;
    let mut repeat: std::os::raw::c_uint = 0;
    let mut num_threads: std::os::raw::c_uint =
        (::std::mem::size_of::<[*const std::os::raw::c_char; 7]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*const std::os::raw::c_char>()
                                             as std::os::raw::c_ulong) as
            std::os::raw::c_uint;
    let mut results: [*mut std::os::raw::c_void; 20] = [0 as *mut std::os::raw::c_void; 20];
    let mut ret: std::os::raw::c_int = 0;
    xmlInitParser();
    repeat = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while repeat < 500 as std::os::raw::c_int as std::os::raw::c_uint {
        xmlLoadCatalog(catalog);
        memset(results.as_mut_ptr() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               (::std::mem::size_of::<*mut std::os::raw::c_void>() as
                    std::os::raw::c_ulong).wrapping_mul(num_threads as
                                                    std::os::raw::c_ulong));
        memset(tid.as_mut_ptr() as *mut std::os::raw::c_void, 0xff as std::os::raw::c_int,
               (::std::mem::size_of::<pthread_t>() as
                    std::os::raw::c_ulong).wrapping_mul(num_threads as
                                                    std::os::raw::c_ulong));
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < num_threads {
            ret =
                pthread_create(&mut *tid.as_mut_ptr().offset(i as isize),
                               0 as *const pthread_attr_t,
                               Some(thread_specific_data as
                                        unsafe extern "C" fn(_:
                                                                 *mut std::os::raw::c_void)
                                            -> *mut std::os::raw::c_void),
                               testfiles[i as usize] as *mut std::os::raw::c_void);
            if ret != 0 as std::os::raw::c_int {
                perror(b"pthread_create\x00" as *const u8 as
                           *const std::os::raw::c_char);
                exit(1 as std::os::raw::c_int);
            }
            i = i.wrapping_add(1)
        }
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < num_threads {
            ret =
                pthread_join(tid[i as usize],
                             &mut *results.as_mut_ptr().offset(i as isize));
            if ret != 0 as std::os::raw::c_int {
                perror(b"pthread_join\x00" as *const u8 as
                           *const std::os::raw::c_char);
                exit(1 as std::os::raw::c_int);
            }
            i = i.wrapping_add(1)
        }
        xmlCatalogCleanup();
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < num_threads {
            if results[i as usize] != Okay as *mut std::os::raw::c_void {
                printf(b"Thread %d handling %s failed\n\x00" as *const u8 as
                           *const std::os::raw::c_char, i, testfiles[i as usize]);
            }
            i = i.wrapping_add(1)
        }
        repeat = repeat.wrapping_add(1)
    }
    xmlCleanupParser();
    xmlMemoryDump();
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
/* !LIBXML_THREADS_ENABLED */
/* pthreads or BeOS threads */
