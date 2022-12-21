
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlDict;
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
    pub type _xmlHashTable;
    pub type _xmlXPathCompExpr;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(__s: *const std::os::raw::c_char);
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
    fn xmlStrlen(str: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn xmlMemoryDump();
    #[no_mangle]
    fn xmlFreeDoc(cur: xmlDocPtr);
    /* defined(LIBXML_TREE_ENABLED) || defined(LIBXML_DEBUG_ENABLED) */
    #[no_mangle]
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    #[no_mangle]
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    /*
 * Init/Cleanup
 */
    #[no_mangle]
    fn xmlInitParser();
    #[no_mangle]
    fn xmlCleanupParser();
    /* LIBXML_SAX1_ENABLED */
    #[no_mangle]
    fn xmlSubstituteEntitiesDefault(val: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlReadFile(URL: *const std::os::raw::c_char, encoding: *const std::os::raw::c_char,
                   options: std::os::raw::c_int) -> xmlDocPtr;
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
    fn __xmlLoadExtDtdDefaultValue() -> *mut std::os::raw::c_int;
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
    fn xmlXPathEvalExpression(str: *const xmlChar, ctxt: xmlXPathContextPtr)
     -> xmlXPathObjectPtr;
    /* *
 * Extending a context.
 */
    #[no_mangle]
    fn xmlXPathRegisterNs(ctxt: xmlXPathContextPtr, prefix: *const xmlChar,
                          ns_uri: *const xmlChar) -> std::os::raw::c_int;
    #[no_mangle]
    fn xmlC14NDocDumpMemory(doc: xmlDocPtr, nodes: xmlNodeSetPtr,
                            mode: std::os::raw::c_int,
                            inclusive_ns_prefixes: *mut *mut xmlChar,
                            with_comments: std::os::raw::c_int,
                            doc_txt_ptr: *mut *mut xmlChar) -> std::os::raw::c_int;
}
pub type xmlChar = std::os::raw::c_uchar;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
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
pub type ssize_t = __ssize_t;
pub type xmlFreeFunc
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>;
pub type xmlMallocFunc
    =
    Option<unsafe extern "C" fn(_: size_t) -> *mut std::os::raw::c_void>;
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
/* *
 * xmlMallocFunc:
 * @size:  the size requested in bytes
 *
 * Signature for a malloc() implementation.
 *
 * Returns a pointer to the newly allocated block or NULL in case of error.
 */
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
/* the validating regexp */
/* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
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
pub type xmlNodePtr = *mut xmlNode;
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlDictPtr = *mut xmlDict;
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
pub type xmlDict = _xmlDict;
pub type xmlDocPtr = *mut xmlDoc;
/* *
 * xmlDoc:
 *
 * An XML document.
 */
pub type xmlDoc = _xmlDoc;
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
/* What part of the library raised this error */
/* The error code, e.g. an xmlParserError */
/* human-readable informative error message */
/* how consequent is the error */
/* the filename */
/* the line number if available */
/* extra string information */
/* extra string information */
/* extra string information */
/* extra number information */
/* error column # or 0 if N/A (todo: rename field when we would brk ABI) */
/* the parser context if available */
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
 * New set of simpler/more flexible APIs
 */
/* *
 * xmlParserOption:
 *
 * This is the set of XML parser options that can be passed down
 * to the xmlReadDoc() and similar calls.
 */
pub type C2RustUnnamed = std::os::raw::c_uint;
/* Store big lines numbers in text PSVI field */
/* ignore internal document encoding hint */
pub const XML_PARSE_BIG_LINES: C2RustUnnamed = 4194304;
/* parse using SAX2 interface before 2.7.0 */
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed = 2097152;
/* relax any hardcoded limit from the parser */
pub const XML_PARSE_OLDSAX: C2RustUnnamed = 1048576;
/* do not fixup XINCLUDE xml:base uris */
pub const XML_PARSE_HUGE: C2RustUnnamed = 524288;
/* parse using XML-1.0 before update 5 */
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed = 262144;
/* compact small text nodes; no modification of
                                   the tree allowed afterwards (will possibly
				   crash if you try to modify the tree) */
pub const XML_PARSE_OLD10: C2RustUnnamed = 131072;
/* do not generate XINCLUDE START/END nodes */
pub const XML_PARSE_COMPACT: C2RustUnnamed = 65536;
/* merge CDATA as text nodes */
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed = 32768;
/* remove redundant namespaces declarations */
pub const XML_PARSE_NOCDATA: C2RustUnnamed = 16384;
/* Do not reuse the context dictionary */
pub const XML_PARSE_NSCLEAN: C2RustUnnamed = 8192;
/* Forbid network access */
pub const XML_PARSE_NODICT: C2RustUnnamed = 4096;
/* Implement XInclude substitition  */
pub const XML_PARSE_NONET: C2RustUnnamed = 2048;
/* use the SAX1 interface internally */
pub const XML_PARSE_XINCLUDE: C2RustUnnamed = 1024;
/* remove blank nodes */
pub const XML_PARSE_SAX1: C2RustUnnamed = 512;
/* pedantic error reporting */
pub const XML_PARSE_NOBLANKS: C2RustUnnamed = 256;
/* suppress warning reports */
pub const XML_PARSE_PEDANTIC: C2RustUnnamed = 128;
/* suppress error reports */
pub const XML_PARSE_NOWARNING: C2RustUnnamed = 64;
/* validate with the DTD */
pub const XML_PARSE_NOERROR: C2RustUnnamed = 32;
/* default DTD attributes */
pub const XML_PARSE_DTDVALID: C2RustUnnamed = 16;
/* load the external subset */
pub const XML_PARSE_DTDATTR: C2RustUnnamed = 8;
/* substitute entities */
pub const XML_PARSE_DTDLOAD: C2RustUnnamed = 4;
/* recover on errors */
pub const XML_PARSE_NOENT: C2RustUnnamed = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed = 1;
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
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const XML_C14N_1_1: C2RustUnnamed_0 = 2;
pub const XML_C14N_EXCLUSIVE_1_0: C2RustUnnamed_0 = 1;
pub const XML_C14N_1_0: C2RustUnnamed_0 = 0;
/*
 * Canonical XML implementation test program
 * (http://www.w3.org/TR/2001/REC-xml-c14n-20010315)
 *
 * See Copyright for the status of this software.
 *
 * Author: Aleksey Sanin <aleksey@aleksey.com>
 */
/* HAVE_UNISTD_H */
unsafe extern "C" fn usage(mut name: *const std::os::raw::c_char) {
    fprintf(stderr,
            b"Usage: %s <mode> <xml-file> [<xpath-expr>] [<inclusive-ns-list>]\n\x00"
                as *const u8 as *const std::os::raw::c_char, name);
    fprintf(stderr,
            b"where <mode> is one of following:\n\x00" as *const u8 as
                *const std::os::raw::c_char);
    fprintf(stderr,
            b"--with-comments       \t XML file canonicalization v1.0 w comments \n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"--without-comments    \t XML file canonicalization v1.0 w/o comments\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"--1-1-with-comments       \t XML file canonicalization v1.1 w comments\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"--1-1-without-comments    \t XML file canonicalization v1.1 w/o comments\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"--exc-with-comments   \t Exclusive XML file canonicalization v1.0 w comments\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    fprintf(stderr,
            b"--exc-without-comments\t Exclusive XML file canonicalization v1.0 w/o comments\n\x00"
                as *const u8 as *const std::os::raw::c_char);
}
/* static void print_xpath_nodes(xmlNodeSetPtr nodes); */
unsafe extern "C" fn test_c14n(mut xml_filename: *const std::os::raw::c_char,
                               mut with_comments: std::os::raw::c_int,
                               mut mode: std::os::raw::c_int,
                               mut xpath_filename: *const std::os::raw::c_char,
                               mut inclusive_namespaces: *mut *mut xmlChar)
 -> std::os::raw::c_int {
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut xpath: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut result: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: std::os::raw::c_int = 0;
    /*
     * build an XML tree from a the file; we need to add default
     * attributes and resolve all character and entities references
     */
    *__xmlLoadExtDtdDefaultValue() = 2 as std::os::raw::c_int | 4 as std::os::raw::c_int;
    xmlSubstituteEntitiesDefault(1 as std::os::raw::c_int);
    doc =
        xmlReadFile(xml_filename, 0 as *const std::os::raw::c_char,
                    XML_PARSE_DTDATTR as std::os::raw::c_int |
                        XML_PARSE_NOENT as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Error: unable to parse file \"%s\"\n\x00" as *const u8 as
                    *const std::os::raw::c_char, xml_filename);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Check the document is of the right kind
     */
    if xmlDocGetRootElement(doc as *const xmlDoc).is_null() {
        fprintf(stderr,
                b"Error: empty document for file \"%s\"\n\x00" as *const u8 as
                    *const std::os::raw::c_char, xml_filename);
        xmlFreeDoc(doc);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * load xpath file if specified
     */
    if !xpath_filename.is_null() {
        xpath = load_xpath_expr(doc, xpath_filename);
        if xpath.is_null() {
            fprintf(stderr,
                    b"Error: unable to evaluate xpath expression\n\x00" as
                        *const u8 as *const std::os::raw::c_char);
            xmlFreeDoc(doc);
            return -(1 as std::os::raw::c_int)
        }
    }
    /*
     * Canonical form
     */
    /* fprintf(stderr,"File \"%s\" loaded: start canonization\n", xml_filename); */
    ret =
        xmlC14NDocDumpMemory(doc,
                             if !xpath.is_null() {
                                 (*xpath).nodesetval
                             } else { 0 as xmlNodeSetPtr }, mode,
                             inclusive_namespaces, with_comments,
                             &mut result);
    if ret >= 0 as std::os::raw::c_int {
        if !result.is_null() {
            if write(1 as std::os::raw::c_int, result as *const std::os::raw::c_void,
                     ret as size_t) == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
                fprintf(stderr,
                        b"Can\'t write data\n\x00" as *const u8 as
                            *const std::os::raw::c_char);
            }
            xmlFree.expect("non-null function pointer")(result as
                                                            *mut std::os::raw::c_void);
        }
    } else {
        fprintf(stderr,
                b"Error: failed to canonicalize XML file \"%s\" (ret=%d)\n\x00"
                    as *const u8 as *const std::os::raw::c_char, xml_filename, ret);
        if !result.is_null() {
            xmlFree.expect("non-null function pointer")(result as
                                                            *mut std::os::raw::c_void);
        }
        xmlFreeDoc(doc);
        return -(1 as std::os::raw::c_int)
    }
    /*
     * Cleanup
     */
    if !xpath.is_null() { xmlXPathFreeObject(xpath); }
    xmlFreeDoc(doc);
    return ret;
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    /*
     * Init libxml
     */
    xmlInitParser();
    xmlCheckVersion(20908 as std::os::raw::c_int);
    /*
     * Parse command line and process file
     */
    if argc < 3 as std::os::raw::c_int {
        fprintf(stderr,
                b"Error: wrong number of arguments.\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        usage(*argv.offset(0 as std::os::raw::c_int as isize));
    } else if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                     b"--with-comments\x00" as *const u8 as
                         *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
        ret =
            test_c14n(*argv.offset(2 as std::os::raw::c_int as isize),
                      1 as std::os::raw::c_int, XML_C14N_1_0 as std::os::raw::c_int,
                      if argc > 3 as std::os::raw::c_int {
                          *argv.offset(3 as std::os::raw::c_int as isize)
                      } else { 0 as *mut std::os::raw::c_char },
                      0 as *mut *mut xmlChar)
    } else if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                     b"--without-comments\x00" as *const u8 as
                         *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
        ret =
            test_c14n(*argv.offset(2 as std::os::raw::c_int as isize),
                      0 as std::os::raw::c_int, XML_C14N_1_0 as std::os::raw::c_int,
                      if argc > 3 as std::os::raw::c_int {
                          *argv.offset(3 as std::os::raw::c_int as isize)
                      } else { 0 as *mut std::os::raw::c_char },
                      0 as *mut *mut xmlChar)
    } else if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                     b"--1-1-with-comments\x00" as *const u8 as
                         *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
        ret =
            test_c14n(*argv.offset(2 as std::os::raw::c_int as isize),
                      1 as std::os::raw::c_int, XML_C14N_1_1 as std::os::raw::c_int,
                      if argc > 3 as std::os::raw::c_int {
                          *argv.offset(3 as std::os::raw::c_int as isize)
                      } else { 0 as *mut std::os::raw::c_char },
                      0 as *mut *mut xmlChar)
    } else if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                     b"--1-1-without-comments\x00" as *const u8 as
                         *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
        ret =
            test_c14n(*argv.offset(2 as std::os::raw::c_int as isize),
                      0 as std::os::raw::c_int, XML_C14N_1_1 as std::os::raw::c_int,
                      if argc > 3 as std::os::raw::c_int {
                          *argv.offset(3 as std::os::raw::c_int as isize)
                      } else { 0 as *mut std::os::raw::c_char },
                      0 as *mut *mut xmlChar)
    } else if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                     b"--exc-with-comments\x00" as *const u8 as
                         *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
        let mut list: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
        /* load exclusive namespace from command line */
        list =
            if argc > 4 as std::os::raw::c_int {
                parse_list(*argv.offset(4 as std::os::raw::c_int as isize) as
                               *mut xmlChar)
            } else { 0 as *mut *mut xmlChar };
        ret =
            test_c14n(*argv.offset(2 as std::os::raw::c_int as isize),
                      1 as std::os::raw::c_int, XML_C14N_EXCLUSIVE_1_0 as std::os::raw::c_int,
                      if argc > 3 as std::os::raw::c_int {
                          *argv.offset(3 as std::os::raw::c_int as isize)
                      } else { 0 as *mut std::os::raw::c_char }, list);
        if !list.is_null() {
            xmlFree.expect("non-null function pointer")(list as
                                                            *mut std::os::raw::c_void);
        }
    } else if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
                     b"--exc-without-comments\x00" as *const u8 as
                         *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
        let mut list_0: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
        /* load exclusive namespace from command line */
        list_0 =
            if argc > 4 as std::os::raw::c_int {
                parse_list(*argv.offset(4 as std::os::raw::c_int as isize) as
                               *mut xmlChar)
            } else { 0 as *mut *mut xmlChar };
        ret =
            test_c14n(*argv.offset(2 as std::os::raw::c_int as isize),
                      0 as std::os::raw::c_int, XML_C14N_EXCLUSIVE_1_0 as std::os::raw::c_int,
                      if argc > 3 as std::os::raw::c_int {
                          *argv.offset(3 as std::os::raw::c_int as isize)
                      } else { 0 as *mut std::os::raw::c_char }, list_0);
        if !list_0.is_null() {
            xmlFree.expect("non-null function pointer")(list_0 as
                                                            *mut std::os::raw::c_void);
        }
    } else {
        fprintf(stderr,
                b"Error: bad option.\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        usage(*argv.offset(0 as std::os::raw::c_int as isize));
    }
    /*
     * Shutdown libxml
     */
    xmlCleanupParser();
    xmlMemoryDump();
    return if ret >= 0 as std::os::raw::c_int {
               0 as std::os::raw::c_int
           } else { 1 as std::os::raw::c_int };
}
/*
 * Macro used to grow the current buffer.
 */
unsafe extern "C" fn parse_list(mut str: *mut xmlChar) -> *mut *mut xmlChar {
    let mut buffer: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut out: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut buffer_size: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut len: std::os::raw::c_int = 0;
    if str.is_null() { return 0 as *mut *mut xmlChar }
    len = xmlStrlen(str);
    if *str.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == '\'' as i32 &&
           *str.offset((len - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int ==
               '\'' as i32 {
        *str.offset((len - 1 as std::os::raw::c_int) as isize) =
            '\u{0}' as i32 as xmlChar;
        str = str.offset(1)
    }
    /*
     * allocate an translation buffer.
     */
    buffer_size = 1000 as std::os::raw::c_int;
    buffer =
        xmlMalloc.expect("non-null function pointer")((buffer_size as
                                                           std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
                                                                                           as
                                                                                           std::os::raw::c_ulong))
            as *mut *mut xmlChar;
    if buffer.is_null() {
        perror(b"malloc failed\x00" as *const u8 as *const std::os::raw::c_char);
        return 0 as *mut *mut xmlChar
    }
    out = buffer;
    while *str as std::os::raw::c_int != '\u{0}' as i32 {
        if out.offset_from(buffer) as std::os::raw::c_long >
               (buffer_size - 10 as std::os::raw::c_int) as std::os::raw::c_long {
            let mut indx: std::os::raw::c_int =
                out.offset_from(buffer) as std::os::raw::c_long as
                    std::os::raw::c_int;
            buffer_size *= 2 as std::os::raw::c_int;
            buffer =
                xmlRealloc.expect("non-null function pointer")(buffer as
                                                                   *mut std::os::raw::c_void,
                                                               (buffer_size as
                                                                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut xmlChar>()
                                                                                                    as
                                                                                                    std::os::raw::c_ulong))
                    as *mut *mut xmlChar;
            if buffer.is_null() {
                perror(b"realloc failed\x00" as *const u8 as
                           *const std::os::raw::c_char);
                return 0 as *mut *mut xmlChar
            }
            out = &mut *buffer.offset(indx as isize) as *mut *mut xmlChar
        }
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = str;
        while *str as std::os::raw::c_int != ',' as i32 &&
                  *str as std::os::raw::c_int != '\u{0}' as i32 {
            str = str.offset(1)
        }
        if *str as std::os::raw::c_int == ',' as i32 {
            let fresh1 = str;
            str = str.offset(1);
            *fresh1 = '\u{0}' as i32 as xmlChar
        }
    }
    *out = 0 as *mut xmlChar;
    return buffer;
}
unsafe extern "C" fn load_xpath_expr(mut parent_doc: xmlDocPtr,
                                     mut filename: *const std::os::raw::c_char)
 -> xmlXPathObjectPtr {
    let mut xpath: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut expr: *mut xmlChar = 0 as *mut xmlChar;
    let mut ctx: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    /*
     * load XPath expr as a file
     */
    *__xmlLoadExtDtdDefaultValue() = 2 as std::os::raw::c_int | 4 as std::os::raw::c_int;
    xmlSubstituteEntitiesDefault(1 as std::os::raw::c_int);
    doc =
        xmlReadFile(filename, 0 as *const std::os::raw::c_char,
                    XML_PARSE_DTDATTR as std::os::raw::c_int |
                        XML_PARSE_NOENT as std::os::raw::c_int);
    if doc.is_null() {
        fprintf(stderr,
                b"Error: unable to parse file \"%s\"\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        return 0 as xmlXPathObjectPtr
    }
    /*
     * Check the document is of the right kind
     */
    if xmlDocGetRootElement(doc as *const xmlDoc).is_null() {
        fprintf(stderr,
                b"Error: empty document for file \"%s\"\n\x00" as *const u8 as
                    *const std::os::raw::c_char, filename);
        xmlFreeDoc(doc);
        return 0 as xmlXPathObjectPtr
    }
    node = (*doc).children;
    while !node.is_null() &&
              xmlStrEqual((*node).name,
                          b"XPath\x00" as *const u8 as *const std::os::raw::c_char as
                              *const xmlChar) == 0 {
        node = (*node).next
    }
    if node.is_null() {
        fprintf(stderr,
                b"Error: XPath element expected in the file  \"%s\"\n\x00" as
                    *const u8 as *const std::os::raw::c_char, filename);
        xmlFreeDoc(doc);
        return 0 as xmlXPathObjectPtr
    }
    expr = xmlNodeGetContent(node as *const xmlNode);
    if expr.is_null() {
        fprintf(stderr,
                b"Error: XPath content element is NULL \"%s\"\n\x00" as
                    *const u8 as *const std::os::raw::c_char, filename);
        xmlFreeDoc(doc);
        return 0 as xmlXPathObjectPtr
    }
    ctx = xmlXPathNewContext(parent_doc);
    if ctx.is_null() {
        fprintf(stderr,
                b"Error: unable to create new context\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(expr as
                                                        *mut std::os::raw::c_void);
        xmlFreeDoc(doc);
        return 0 as xmlXPathObjectPtr
    }
    /*
     * Register namespaces
     */
    ns = (*node).nsDef;
    while !ns.is_null() {
        if xmlXPathRegisterNs(ctx, (*ns).prefix, (*ns).href) !=
               0 as std::os::raw::c_int {
            fprintf(stderr,
                    b"Error: unable to register NS with prefix=\"%s\" and href=\"%s\"\n\x00"
                        as *const u8 as *const std::os::raw::c_char, (*ns).prefix,
                    (*ns).href);
            xmlFree.expect("non-null function pointer")(expr as
                                                            *mut std::os::raw::c_void);
            xmlXPathFreeContext(ctx);
            xmlFreeDoc(doc);
            return 0 as xmlXPathObjectPtr
        }
        ns = (*ns).next
    }
    /*
     * Evaluate xpath
     */
    xpath = xmlXPathEvalExpression(expr, ctx);
    if xpath.is_null() {
        fprintf(stderr,
                b"Error: unable to evaluate xpath expression\n\x00" as
                    *const u8 as *const std::os::raw::c_char);
        xmlFree.expect("non-null function pointer")(expr as
                                                        *mut std::os::raw::c_void);
        xmlXPathFreeContext(ctx);
        xmlFreeDoc(doc);
        return 0 as xmlXPathObjectPtr
    }
    /* print_xpath_nodes(xpath->nodesetval); */
    xmlFree.expect("non-null function pointer")(expr as *mut std::os::raw::c_void);
    xmlXPathFreeContext(ctx);
    xmlFreeDoc(doc);
    return xpath;
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
/* LIBXML_C14N_ENABLED */
/*
static void
print_xpath_nodes(xmlNodeSetPtr nodes) {
    xmlNodePtr cur;
    int i;

    if(nodes == NULL ){
	fprintf(stderr, "Error: no nodes set defined\n");
	return;
    }

    fprintf(stderr, "Nodes Set:\n-----\n");
    for(i = 0; i < nodes->nodeNr; ++i) {
	if(nodes->nodeTab[i]->type == XML_NAMESPACE_DECL) {
	    xmlNsPtr ns;

	    ns = (xmlNsPtr)nodes->nodeTab[i];
	    cur = (xmlNodePtr)ns->next;
	    fprintf(stderr, "namespace \"%s\"=\"%s\" for node %s:%s\n",
		    ns->prefix, ns->href,
		    (cur->ns) ? cur->ns->prefix : BAD_CAST "", cur->name);
	} else if(nodes->nodeTab[i]->type == XML_ELEMENT_NODE) {
	    cur = nodes->nodeTab[i];
	    fprintf(stderr, "element node \"%s:%s\"\n",
		    (cur->ns) ? cur->ns->prefix : BAD_CAST "", cur->name);
	} else {
	    cur = nodes->nodeTab[i];
	    fprintf(stderr, "node \"%s\": type %d\n", cur->name, cur->type);
	}
    }
}
*/
