
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
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlIsBlankNode(node: *const xmlNode) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr,
                         href: *const xmlChar) -> xmlNsPtr;
    #[no_mangle]
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    #[no_mangle]
    fn xmlNodeListGetString(doc: xmlDocPtr, list: *const xmlNode,
                            inLine: std::os::raw::c_int) -> *mut xmlChar;
    #[no_mangle]
    fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> std::os::raw::c_int;
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
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlParseFile(filename: *const std::os::raw::c_char) -> xmlDocPtr;
    #[no_mangle]
    fn xmlKeepBlanksDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct person {
    pub name: *mut xmlChar,
    pub email: *mut xmlChar,
    pub company: *mut xmlChar,
    pub organisation: *mut xmlChar,
    pub smail: *mut xmlChar,
    pub webPage: *mut xmlChar,
    pub phone: *mut xmlChar,
}
pub type personPtr = *mut person;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub projectID: *mut xmlChar,
    pub application: *mut xmlChar,
    pub category: *mut xmlChar,
    pub contact: personPtr,
    pub nbDevelopers: std::os::raw::c_int,
    pub developers: [personPtr; 100],
}
pub type jobPtr = *mut job;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gjob {
    pub nbJobs: std::os::raw::c_int,
    pub jobs: [jobPtr; 500],
}
/*
 * A pool of Gnome Jobs
 */
pub type gJob = gjob;
pub type gJobPtr = *mut gjob;
/* using dynamic alloc is left as an exercise */
/*
 * And the code needed to parse it
 */
unsafe extern "C" fn parsePerson(mut doc: xmlDocPtr, mut ns: xmlNsPtr,
                                 mut cur: xmlNodePtr) -> personPtr {
    let mut ret: personPtr = 0 as personPtr;
    printf(b"parsePerson\n\x00" as *const u8 as *const std::os::raw::c_char);
    /*
     * allocate the struct
     */
    ret =
        malloc(::std::mem::size_of::<person>() as std::os::raw::c_ulong) as personPtr;
    if ret.is_null() {
        fprintf(stderr,
                b"out of memory\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as personPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<person>() as std::os::raw::c_ulong);
    /* We don't care what the top level element name is */
    /* COMPAT xmlChildrenNode is a macro unifying libxml1 and libxml2 names */
    cur = (*cur).children;
    while !cur.is_null() {
        if xmlStrcmp((*cur).name,
                     b"Person\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            (*ret).name =
                xmlNodeListGetString(doc, (*cur).children, 1 as std::os::raw::c_int)
        }
        if xmlStrcmp((*cur).name,
                     b"Email\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            (*ret).email =
                xmlNodeListGetString(doc, (*cur).children, 1 as std::os::raw::c_int)
        }
        cur = (*cur).next
    }
    return ret;
}
/*
 * and to print it
 */
unsafe extern "C" fn printPerson(mut cur: personPtr) {
    if cur.is_null() { return }
    printf(b"------ Person\n\x00" as *const u8 as *const std::os::raw::c_char);
    if !(*cur).name.is_null() {
        printf(b"\tname: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).name);
    }
    if !(*cur).email.is_null() {
        printf(b"\temail: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).email);
    }
    if !(*cur).company.is_null() {
        printf(b"\tcompany: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).company);
    }
    if !(*cur).organisation.is_null() {
        printf(b"\torganisation: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, (*cur).organisation);
    }
    if !(*cur).smail.is_null() {
        printf(b"\tsmail: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).smail);
    }
    if !(*cur).webPage.is_null() {
        printf(b"\tWeb: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).webPage);
    }
    if !(*cur).phone.is_null() {
        printf(b"\tphone: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).phone);
    }
    printf(b"------\n\x00" as *const u8 as *const std::os::raw::c_char);
}
/*
 * And the code needed to parse it
 */
unsafe extern "C" fn parseJob(mut doc: xmlDocPtr, mut ns: xmlNsPtr,
                              mut cur: xmlNodePtr) -> jobPtr {
    let mut ret: jobPtr = 0 as jobPtr;
    printf(b"parseJob\n\x00" as *const u8 as *const std::os::raw::c_char);
    /*
     * allocate the struct
     */
    ret = malloc(::std::mem::size_of::<job>() as std::os::raw::c_ulong) as jobPtr;
    if ret.is_null() {
        fprintf(stderr,
                b"out of memory\n\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as jobPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<job>() as std::os::raw::c_ulong);
    /* We don't care what the top level element name is */
    cur = (*cur).children;
    while !cur.is_null() {
        if xmlStrcmp((*cur).name,
                     b"Project\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            (*ret).projectID =
                xmlGetProp(cur as *const xmlNode,
                           b"ID\x00" as *const u8 as *const std::os::raw::c_char as
                               *const xmlChar);
            if (*ret).projectID.is_null() {
                fprintf(stderr,
                        b"Project has no ID\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
        }
        if xmlStrcmp((*cur).name,
                     b"Application\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            (*ret).application =
                xmlNodeListGetString(doc, (*cur).children, 1 as std::os::raw::c_int)
        }
        if xmlStrcmp((*cur).name,
                     b"Category\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            (*ret).category =
                xmlNodeListGetString(doc, (*cur).children, 1 as std::os::raw::c_int)
        }
        if xmlStrcmp((*cur).name,
                     b"Contact\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            (*ret).contact = parsePerson(doc, ns, cur)
        }
        cur = (*cur).next
    }
    return ret;
}
/*
 * and to print it
 */
unsafe extern "C" fn printJob(mut cur: jobPtr) {
    let mut i: std::os::raw::c_int = 0;
    if cur.is_null() { return }
    printf(b"=======  Job\n\x00" as *const u8 as *const std::os::raw::c_char);
    if !(*cur).projectID.is_null() {
        printf(b"projectID: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).projectID);
    }
    if !(*cur).application.is_null() {
        printf(b"application: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).application);
    }
    if !(*cur).category.is_null() {
        printf(b"category: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
               (*cur).category);
    }
    if !(*cur).contact.is_null() { printPerson((*cur).contact); }
    printf(b"%d developers\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*cur).nbDevelopers);
    i = 0 as std::os::raw::c_int;
    while i < (*cur).nbDevelopers {
        printPerson((*cur).developers[i as usize]);
        i += 1
    }
    printf(b"======= \n\x00" as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn parseGjobFile(mut filename: *mut std::os::raw::c_char)
 -> gJobPtr {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ret: gJobPtr = 0 as *mut gjob;
    let mut curjob: jobPtr = 0 as *mut job;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    /*
     * build an XML tree from a the file;
     */
    doc = xmlParseFile(filename);
    if doc.is_null() { return 0 as gJobPtr }
    /* LIBXML_SAX1_ENABLED */
    /*
     * Check the document is of the right kind
     */
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if cur.is_null() {
        fprintf(stderr,
                b"empty document\n\x00" as *const u8 as *const std::os::raw::c_char);
        xmlFreeDoc(doc);
        return 0 as gJobPtr
    }
    ns =
        xmlSearchNsByHref(doc, cur,
                          b"http://www.gnome.org/some-location\x00" as
                              *const u8 as *const std::os::raw::c_char as
                              *const xmlChar);
    if ns.is_null() {
        fprintf(stderr,
                b"document of the wrong type, GJob Namespace not found\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
        xmlFreeDoc(doc);
        return 0 as gJobPtr
    }
    if xmlStrcmp((*cur).name,
                 b"Helping\x00" as *const u8 as *const std::os::raw::c_char as
                     *const xmlChar) != 0 {
        fprintf(stderr,
                b"document of the wrong type, root node != Helping\x00" as
                    *const u8 as *const std::os::raw::c_char);
        xmlFreeDoc(doc);
        return 0 as gJobPtr
    }
    /*
     * Allocate the structure to be returned.
     */
    ret = malloc(::std::mem::size_of::<gJob>() as std::os::raw::c_ulong) as gJobPtr;
    if ret.is_null() {
        fprintf(stderr,
                b"out of memory\n\x00" as *const u8 as *const std::os::raw::c_char);
        xmlFreeDoc(doc);
        return 0 as gJobPtr
    }
    memset(ret as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<gJob>() as std::os::raw::c_ulong);
    /*
     * Now, walk the tree.
     */
    /* First level we expect just Jobs */
    cur = (*cur).children;
    while !cur.is_null() && xmlIsBlankNode(cur as *const xmlNode) != 0 {
        cur = (*cur).next
    }
    if cur.is_null() {
        xmlFreeDoc(doc);
        free(ret as *mut std::os::raw::c_void);
        return 0 as gJobPtr
    }
    if xmlStrcmp((*cur).name,
                 b"Jobs\x00" as *const u8 as *const std::os::raw::c_char as
                     *const xmlChar) != 0 || (*cur).ns != ns {
        fprintf(stderr,
                b"document of the wrong type, was \'%s\', Jobs expected\x00"
                    as *const u8 as *const std::os::raw::c_char, (*cur).name);
        fprintf(stderr,
                b"xmlDocDump follows\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        xmlDocDump(stderr, doc);
        fprintf(stderr,
                b"xmlDocDump finished\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        /* LIBXML_OUTPUT_ENABLED */
        xmlFreeDoc(doc);
        free(ret as *mut std::os::raw::c_void);
        return 0 as gJobPtr
    }
    /* Second level is a list of Job, but be laxist */
    cur = (*cur).children;
    while !cur.is_null() {
        if xmlStrcmp((*cur).name,
                     b"Job\x00" as *const u8 as *const std::os::raw::c_char as
                         *const xmlChar) == 0 && (*cur).ns == ns {
            curjob = parseJob(doc, ns, cur);
            if !curjob.is_null() {
                let fresh0 = (*ret).nbJobs;
                (*ret).nbJobs = (*ret).nbJobs + 1;
                (*ret).jobs[fresh0 as usize] = curjob
            }
            if (*ret).nbJobs >= 500 as std::os::raw::c_int { break ; }
        }
        cur = (*cur).next
    }
    return ret;
}
unsafe extern "C" fn handleGjob(mut cur: gJobPtr) {
    let mut i: std::os::raw::c_int = 0;
    /*
     * Do whatever you want and free the structure.
     */
    printf(b"%d Jobs registered\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*cur).nbJobs);
    i = 0 as std::os::raw::c_int;
    while i < (*cur).nbJobs { printJob((*cur).jobs[i as usize]); i += 1 };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut cur: gJobPtr = 0 as *mut gjob;
    /* COMPAT: Do not genrate nodes for formatting spaces */
    xmlCheckVersion(20908 as std::os::raw::c_int);
    xmlKeepBlanksDefault(0 as std::os::raw::c_int);
    i = 1 as std::os::raw::c_int;
    while i < argc {
        cur = parseGjobFile(*argv.offset(i as isize));
        if !cur.is_null() {
            handleGjob(cur);
        } else {
            fprintf(stderr,
                    b"Error parsing file \'%s\'\n\x00" as *const u8 as
                        *const std::os::raw::c_char, *argv.offset(i as isize));
        }
        i += 1
    }
    /* Clean up everything else before quitting. */
    xmlCleanupParser();
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
