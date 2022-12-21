
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlLineNumbersDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn __xstat(__ver: std::os::raw::c_int, __filename: *const std::os::raw::c_char,
               __stat_buf: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn open(__file: *const std::os::raw::c_char, __oflag: std::os::raw::c_int, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn mmap(__addr: *mut std::os::raw::c_void, __len: size_t, __prot: std::os::raw::c_int,
            __flags: std::os::raw::c_int, __fd: std::os::raw::c_int, __offset: __off64_t)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut std::os::raw::c_void, __len: size_t) -> std::os::raw::c_int;
    /*
 * Interfaces for parsing.
 */
    #[no_mangle]
    fn xmlSchemaNewParserCtxt(URL: *const std::os::raw::c_char)
     -> xmlSchemaParserCtxtPtr;
    #[no_mangle]
    fn xmlSchemaNewMemParserCtxt(buffer: *const std::os::raw::c_char,
                                 size: std::os::raw::c_int) -> xmlSchemaParserCtxtPtr;
    #[no_mangle]
    fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
    #[no_mangle]
    fn xmlSchemaSetParserErrors(ctxt: xmlSchemaParserCtxtPtr,
                                err: xmlSchemaValidityErrorFunc,
                                warn: xmlSchemaValidityWarningFunc,
                                ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    #[no_mangle]
    fn xmlSchemaFree(schema: xmlSchemaPtr);
    #[no_mangle]
    fn xmlSchemaDump(output: *mut FILE, schema: xmlSchemaPtr);
    /* LIBXML_OUTPUT_ENABLED */
    /*
 * Interfaces for validating
 */
    #[no_mangle]
    fn xmlSchemaSetValidErrors(ctxt: xmlSchemaValidCtxtPtr,
                               err: xmlSchemaValidityErrorFunc,
                               warn: xmlSchemaValidityWarningFunc,
                               ctx: *mut std::os::raw::c_void);
    #[no_mangle]
    fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    #[no_mangle]
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    #[no_mangle]
    fn xmlSchemaValidateDoc(ctxt: xmlSchemaValidCtxtPtr, instance: xmlDocPtr)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlSchemaCleanupTypes();
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
pub type __dev_t = std::os::raw::c_ulong;
pub type __uid_t = std::os::raw::c_uint;
pub type __gid_t = std::os::raw::c_uint;
pub type __ino_t = std::os::raw::c_ulong;
pub type __mode_t = std::os::raw::c_uint;
pub type __nlink_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __blksize_t = std::os::raw::c_long;
pub type __blkcnt_t = std::os::raw::c_long;
pub type __syscall_slong_t = std::os::raw::c_long;
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
/* XML_ATTRIBUTE_NODE, must be second ! */
/* the name of the property */
/* the value of the property */
/* NULL */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* pointer to the associated namespace */
/* the attribute type if validating */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
/* The kind of type */
/*
* TODO: Actually all those flags used for the schema should sit
* on the schema parser context, since they are used only
* during parsing an XML schema document, and not available
* on the component level as per spec.
*/
/* *
 * XML_SCHEMAS_QUALIF_ELEM:
 *
 * Reflects elementFormDefault == qualified in
 * an XML schema document.
 */
/* *
 * XML_SCHEMAS_QUALIF_ATTR:
 *
 * Reflects attributeFormDefault == qualified in
 * an XML schema document.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_EXTENSION:
 *
 * the schema has "extension" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_RESTRICTION:
 *
 * the schema has "restriction" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_LIST:
 *
 * the cshema has "list" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_FINAL_DEFAULT_UNION:
 *
 * the schema has "union" in the set of finalDefault.
 */
/* *
 * XML_SCHEMAS_BLOCK_DEFAULT_EXTENSION:
 *
 * the schema has "extension" in the set of blockDefault.
 */
/* *
 * XML_SCHEMAS_BLOCK_DEFAULT_RESTRICTION:
 *
 * the schema has "restriction" in the set of blockDefault.
 */
/* *
 * XML_SCHEMAS_BLOCK_DEFAULT_SUBSTITUTION:
 *
 * the schema has "substitution" in the set of blockDefault.
 */
/* *
 * XML_SCHEMAS_INCLUDING_CONVERT_NS:
 *
 * the schema is currently including an other schema with
 * no target namespace.
 */
/* *
 * _xmlSchema:
 *
 * A Schemas definition
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchema {
    pub name: *const xmlChar,
    pub targetNamespace: *const xmlChar,
    pub version: *const xmlChar,
    pub id: *const xmlChar,
    pub doc: xmlDocPtr,
    pub annot: xmlSchemaAnnotPtr,
    pub flags: std::os::raw::c_int,
    pub typeDecl: xmlHashTablePtr,
    pub attrDecl: xmlHashTablePtr,
    pub attrgrpDecl: xmlHashTablePtr,
    pub elemDecl: xmlHashTablePtr,
    pub notaDecl: xmlHashTablePtr,
    pub schemasImports: xmlHashTablePtr,
    pub _private: *mut std::os::raw::c_void,
    pub groupDecl: xmlHashTablePtr,
    pub dict: xmlDictPtr,
    pub includes: *mut std::os::raw::c_void,
    pub preserve: std::os::raw::c_int,
    pub counter: std::os::raw::c_int,
    pub idcDef: xmlHashTablePtr,
    pub volatiles: *mut std::os::raw::c_void,
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
/*
    XML_SCHEMA_VAL_XSI_ASSEMBLE			= 1<<1,
	* assemble schemata using
	* xsi:schemaLocation and
	* xsi:noNamespaceSchemaLocation
*/
/* *
 * The schemas related types are kept internal
 */
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
/* *
 * xmlSchemaValidityErrorFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of an error callback from an XSD validation
 */
pub type xmlSchemaValidityErrorFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * xmlSchemaValidityWarningFunc:
 * @ctx: the validation context
 * @msg: the message
 * @...: extra arguments
 *
 * Signature of a warning callback from an XSD validation
 */
pub type xmlSchemaValidityWarningFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_char,
                                _: ...) -> ()>;
/* *
 * A schemas validation context
 */
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
#[inline]
unsafe extern "C" fn stat(mut __path: *const std::os::raw::c_char,
                          mut __statbuf: *mut stat) -> std::os::raw::c_int {
    return __xstat(1 as std::os::raw::c_int, __path, __statbuf);
}
/*
 * testSchemas.c : a small tester program for Schema validation
 *
 * See Copyright for the status of this software.
 *
 * Daniel.Veillard@w3.org
 */
/* seems needed for Solaris */
static mut debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut noout: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut memory: std::os::raw::c_int = 0 as std::os::raw::c_int;
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut i: std::os::raw::c_int = 0;
    let mut files: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut schema: xmlSchemaPtr = 0 as xmlSchemaPtr;
    i = 1 as std::os::raw::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize),
                  b"-debug\x00" as *const u8 as *const std::os::raw::c_char) == 0 ||
               strcmp(*argv.offset(i as isize),
                      b"--debug\x00" as *const u8 as *const std::os::raw::c_char) == 0
           {
            debug += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-memory\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--memory\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            memory += 1
        } else if strcmp(*argv.offset(i as isize),
                         b"-noout\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 ||
                      strcmp(*argv.offset(i as isize),
                             b"--noout\x00" as *const u8 as
                                 *const std::os::raw::c_char) == 0 {
            noout += 1
        }
        i += 1
    }
    xmlLineNumbersDefault(1 as std::os::raw::c_int);
    i = 1 as std::os::raw::c_int;
    loop  {
        if !(i < argc) { current_block = 9241535491006583629; break ; }
        if *(*argv.offset(i as isize)).offset(0 as std::os::raw::c_int as isize) as
               std::os::raw::c_int != '-' as i32 {
            if schema.is_null() {
                let mut ctxt: xmlSchemaParserCtxtPtr =
                    0 as *mut xmlSchemaParserCtxt;
                if memory != 0 {
                    let mut fd: std::os::raw::c_int = 0;
                    let mut info: stat =
                        stat{st_dev: 0,
                             st_ino: 0,
                             st_nlink: 0,
                             st_mode: 0,
                             st_uid: 0,
                             st_gid: 0,
                             __pad0: 0,
                             st_rdev: 0,
                             st_size: 0,
                             st_blksize: 0,
                             st_blocks: 0,
                             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                             __glibc_reserved: [0; 3],};
                    let mut base: *const std::os::raw::c_char =
                        0 as *const std::os::raw::c_char;
                    if stat(*argv.offset(i as isize), &mut info) <
                           0 as std::os::raw::c_int {
                        current_block = 9241535491006583629;
                        break ;
                    }
                    fd = open(*argv.offset(i as isize), 0 as std::os::raw::c_int);
                    if fd < 0 as std::os::raw::c_int {
                        current_block = 9241535491006583629;
                        break ;
                    }
                    base =
                        mmap(0 as *mut std::os::raw::c_void, info.st_size as size_t,
                             0x1 as std::os::raw::c_int, 0x1 as std::os::raw::c_int, fd,
                             0 as std::os::raw::c_int as __off64_t) as
                            *const std::os::raw::c_char;
                    if base ==
                           -(1 as std::os::raw::c_int) as *mut std::os::raw::c_void as
                               *const std::os::raw::c_char {
                        current_block = 9241535491006583629;
                        break ;
                    }
                    ctxt =
                        xmlSchemaNewMemParserCtxt(base as *mut std::os::raw::c_char,
                                                  info.st_size as
                                                      std::os::raw::c_int);
                    xmlSchemaSetParserErrors(ctxt,
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut FILE,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    std::os::raw::c_int>,
                                                                     xmlSchemaValidityErrorFunc>(Some(fprintf
                                                                                                          as
                                                                                                          unsafe extern "C" fn(_:
                                                                                                                                   *mut FILE,
                                                                                                                               _:
                                                                                                                                   *const std::os::raw::c_char,
                                                                                                                               _:
                                                                                                                                   ...)
                                                                                                              ->
                                                                                                                  std::os::raw::c_int)),
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut FILE,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    std::os::raw::c_int>,
                                                                     xmlSchemaValidityWarningFunc>(Some(fprintf
                                                                                                            as
                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                     *mut FILE,
                                                                                                                                 _:
                                                                                                                                     *const std::os::raw::c_char,
                                                                                                                                 _:
                                                                                                                                     ...)
                                                                                                                ->
                                                                                                                    std::os::raw::c_int)),
                                             stderr as *mut std::os::raw::c_void);
                    schema = xmlSchemaParse(ctxt);
                    xmlSchemaFreeParserCtxt(ctxt);
                    munmap(base as *mut std::os::raw::c_char as *mut std::os::raw::c_void,
                           info.st_size as size_t);
                } else {
                    ctxt = xmlSchemaNewParserCtxt(*argv.offset(i as isize));
                    xmlSchemaSetParserErrors(ctxt,
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut FILE,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    std::os::raw::c_int>,
                                                                     xmlSchemaValidityErrorFunc>(Some(fprintf
                                                                                                          as
                                                                                                          unsafe extern "C" fn(_:
                                                                                                                                   *mut FILE,
                                                                                                                               _:
                                                                                                                                   *const std::os::raw::c_char,
                                                                                                                               _:
                                                                                                                                   ...)
                                                                                                              ->
                                                                                                                  std::os::raw::c_int)),
                                             ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                     *mut FILE,
                                                                                                 _:
                                                                                                     *const std::os::raw::c_char,
                                                                                                 _:
                                                                                                     ...)
                                                                                ->
                                                                                    std::os::raw::c_int>,
                                                                     xmlSchemaValidityWarningFunc>(Some(fprintf
                                                                                                            as
                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                     *mut FILE,
                                                                                                                                 _:
                                                                                                                                     *const std::os::raw::c_char,
                                                                                                                                 _:
                                                                                                                                     ...)
                                                                                                                ->
                                                                                                                    std::os::raw::c_int)),
                                             stderr as *mut std::os::raw::c_void);
                    schema = xmlSchemaParse(ctxt);
                    xmlSchemaFreeParserCtxt(ctxt);
                }
                if debug != 0 { xmlSchemaDump(stdout, schema); }
                /* LIBXML_OUTPUT_ENABLED */
                if schema.is_null() {
                    current_block = 12375689746858790082;
                    break ;
                }
            } else {
                let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
                doc =
                    xmlReadFile(*argv.offset(i as isize),
                                0 as *const std::os::raw::c_char, 0 as std::os::raw::c_int);
                if doc.is_null() {
                    fprintf(stderr,
                            b"Could not parse %s\n\x00" as *const u8 as
                                *const std::os::raw::c_char,
                            *argv.offset(i as isize));
                } else {
                    let mut ctxt_0: xmlSchemaValidCtxtPtr =
                        0 as *mut xmlSchemaValidCtxt;
                    let mut ret: std::os::raw::c_int = 0;
                    ctxt_0 = xmlSchemaNewValidCtxt(schema);
                    xmlSchemaSetValidErrors(ctxt_0,
                                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                    *mut FILE,
                                                                                                _:
                                                                                                    *const std::os::raw::c_char,
                                                                                                _:
                                                                                                    ...)
                                                                               ->
                                                                                   std::os::raw::c_int>,
                                                                    xmlSchemaValidityErrorFunc>(Some(fprintf
                                                                                                         as
                                                                                                         unsafe extern "C" fn(_:
                                                                                                                                  *mut FILE,
                                                                                                                              _:
                                                                                                                                  *const std::os::raw::c_char,
                                                                                                                              _:
                                                                                                                                  ...)
                                                                                                             ->
                                                                                                                 std::os::raw::c_int)),
                                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                    *mut FILE,
                                                                                                _:
                                                                                                    *const std::os::raw::c_char,
                                                                                                _:
                                                                                                    ...)
                                                                               ->
                                                                                   std::os::raw::c_int>,
                                                                    xmlSchemaValidityWarningFunc>(Some(fprintf
                                                                                                           as
                                                                                                           unsafe extern "C" fn(_:
                                                                                                                                    *mut FILE,
                                                                                                                                _:
                                                                                                                                    *const std::os::raw::c_char,
                                                                                                                                _:
                                                                                                                                    ...)
                                                                                                               ->
                                                                                                                   std::os::raw::c_int)),
                                            stderr as *mut std::os::raw::c_void);
                    ret = xmlSchemaValidateDoc(ctxt_0, doc);
                    if ret == 0 as std::os::raw::c_int {
                        printf(b"%s validates\n\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               *argv.offset(i as isize));
                    } else if ret > 0 as std::os::raw::c_int {
                        printf(b"%s fails to validate\n\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               *argv.offset(i as isize));
                    } else {
                        printf(b"%s validation generated an internal error\n\x00"
                                   as *const u8 as *const std::os::raw::c_char,
                               *argv.offset(i as isize));
                    }
                    xmlSchemaFreeValidCtxt(ctxt_0);
                    xmlFreeDoc(doc);
                }
            }
            files += 1
        }
        i += 1
    }
    match current_block {
        9241535491006583629 => {
            if !schema.is_null() { xmlSchemaFree(schema); }
            if files == 0 as std::os::raw::c_int {
                printf(b"Usage : %s [--debug] [--noout] schemas XMLfiles ...\n\x00"
                           as *const u8 as *const std::os::raw::c_char,
                       *argv.offset(0 as std::os::raw::c_int as isize));
                printf(b"\tParse the HTML files and output the result of the parsing\n\x00"
                           as *const u8 as *const std::os::raw::c_char);
                printf(b"\t--debug : dump a debug tree of the in-memory document\n\x00"
                           as *const u8 as *const std::os::raw::c_char);
                printf(b"\t--noout : do not print the result\n\x00" as
                           *const u8 as *const std::os::raw::c_char);
                printf(b"\t--memory : test the schemas in memory parsing\n\x00"
                           as *const u8 as *const std::os::raw::c_char);
            }
        }
        _ => { }
    }
    xmlSchemaCleanupTypes();
    xmlCleanupParser();
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
/* LIBXML_SCHEMAS_ENABLED */
